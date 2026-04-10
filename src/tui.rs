use std::collections::VecDeque;
use std::io::{self, IsTerminal, Stdout};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use anyhow::Result;
use unicode_width::UnicodeWidthStr;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Terminal,
};

use crate::events::{EventReceiver, EventSender, ProgressEvent};
use crate::state::{ProjectState, StepStatus};

/// Write a diagnostic line to /tmp/mz-stream-debug.log when MZ_STREAM_DEBUG is set.
fn stream_debug_log(msg: &str) {
    if std::env::var("MZ_STREAM_DEBUG").is_ok() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/mz-stream-debug.log")
        {
            let _ = writeln!(f, "[tui] {}", msg);
        }
    }
}

// ── Panel ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Panel {
    Tracks,
    Progress,
    Output,
}

impl Panel {
    fn next(&self) -> Panel {
        match self {
            Panel::Tracks => Panel::Progress,
            Panel::Progress => Panel::Output,
            Panel::Output => Panel::Tracks,
        }
    }
}

// ── DashboardState ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TrackStatus {
    Done,
    Active,
    Pending,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub id: String,
    pub title: String,
    pub status: TrackStatus,
    pub steps_done: usize,
    pub steps_total: usize,
}

#[derive(Debug, Clone)]
pub struct StepInfo {
    pub track_id: String,
    pub step_num: usize,
    pub total_steps: usize,
    pub started_at: Instant,
}

const MAX_OUTPUT_LINES: usize = 5000;
const MIN_TUI_WIDTH: u16 = 60;
const MIN_TUI_HEIGHT: u16 = 20;

#[derive(Debug, Clone)]
pub enum OutputLine {
    Plain(String),
    Assistant(String),
    ToolUse(String),
    ToolResult(String),
    Label(String),
}

pub struct DashboardState {
    pub phase_id: String,
    pub tracks: Vec<TrackInfo>,
    pub current_step: Option<StepInfo>,
    pub output_lines: VecDeque<OutputLine>,
    pub finished: Option<(usize, usize)>,
    pub scroll_offset: u16,
    pub user_scrolled: bool,
    pub track_scroll_offset: u16,
    pub partial_line: String,
    pub has_partial: bool,
    pub model: Option<String>,
    pub start_time: std::time::Instant,
    pub cost: Option<(f64, Option<f64>)>,
}

impl DashboardState {
    pub fn from_project_state(project_state: &ProjectState) -> Self {
        let phase_id = project_state.current_phase();
        let tracks = project_state
            .phases
            .iter()
            .find(|p| p.id == phase_id)
            .map(|phase| {
                phase
                    .tracks
                    .iter()
                    .map(|t| {
                        let steps_done = t
                            .steps
                            .iter()
                            .filter(|s| s.status == StepStatus::Complete)
                            .count();
                        let steps_total = t.steps.len();
                        let has_blocked = t.steps.iter().any(|s| s.status == StepStatus::Blocked);
                        let has_in_progress =
                            t.steps.iter().any(|s| s.status == StepStatus::InProgress);
                        let status = if steps_done == steps_total && steps_total > 0 {
                            TrackStatus::Done
                        } else if has_blocked {
                            TrackStatus::Blocked
                        } else if has_in_progress || steps_done > 0 {
                            TrackStatus::Active
                        } else {
                            TrackStatus::Pending
                        };
                        TrackInfo {
                            id: t.id.clone(),
                            title: t.title.clone(),
                            status,
                            steps_done,
                            steps_total,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        DashboardState {
            phase_id: phase_id.to_string(),
            tracks,
            current_step: None,
            output_lines: VecDeque::new(),
            finished: None,
            scroll_offset: 0,
            user_scrolled: false,
            track_scroll_offset: 0,
            partial_line: String::new(),
            has_partial: false,
            model: None,
            start_time: std::time::Instant::now(),
            cost: None,
        }
    }

    fn reload_tracks(&mut self) {
        if let Ok(state) = crate::state::load() {
            let phase = state.phases.iter().find(|p| p.id == self.phase_id);
            if let Some(phase) = phase {
                self.tracks = phase.tracks.iter().map(|t| {
                    let steps_done = t.steps.iter().filter(|s| s.status == StepStatus::Complete).count();
                    let steps_total = t.steps.len();
                    let has_blocked = t.steps.iter().any(|s| s.status == StepStatus::Blocked);
                    let has_in_progress = t.steps.iter().any(|s| s.status == StepStatus::InProgress);
                    let status = if steps_done == steps_total && steps_total > 0 {
                        TrackStatus::Done
                    } else if has_blocked {
                        TrackStatus::Blocked
                    } else if has_in_progress || steps_done > 0 {
                        TrackStatus::Active
                    } else {
                        TrackStatus::Pending
                    };
                    TrackInfo {
                        id: t.id.clone(),
                        title: t.title.clone(),
                        status,
                        steps_done,
                        steps_total,
                    }
                }).collect();
            }
        }
    }

    fn flush_partial(&mut self) {
        if self.has_partial {
            self.output_lines.pop_back();
            self.has_partial = false;
        }
        if !self.partial_line.is_empty() {
            self.output_lines.push_back(OutputLine::Assistant(
                std::mem::take(&mut self.partial_line),
            ));
        }
    }

    pub fn update(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::PhaseStarted => {}

            ProgressEvent::StepStarted {
                track_id,
                step_num,
                total_steps,
            } => {
                if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
                    t.status = TrackStatus::Active;
                }
                self.current_step = Some(StepInfo {
                    track_id,
                    step_num,
                    total_steps,
                    started_at: Instant::now(),
                });
            }

            ProgressEvent::StepCompleted { .. }
            | ProgressEvent::StepFailed { .. }
            | ProgressEvent::TrackCompleted { .. } => {
                self.reload_tracks();
                self.current_step = None;
            }

            ProgressEvent::ClaudeOutput { line } => {
                self.flush_partial();
                self.output_lines.push_back(OutputLine::Plain(line));
            }

            ProgressEvent::AssistantText { text } => {
                self.flush_partial();
                self.output_lines.push_back(OutputLine::Assistant(text));
            }

            ProgressEvent::ToolUseStarted { tool } => {
                self.flush_partial();
                self.output_lines.push_back(OutputLine::ToolUse(tool));
            }

            ProgressEvent::ToolResultReceived { tool } => {
                self.flush_partial();
                self.output_lines.push_back(OutputLine::ToolResult(tool));
            }

            ProgressEvent::PhaseLabel { label } => {
                self.flush_partial();
                self.output_lines.push_back(OutputLine::Label(label));
            }

            ProgressEvent::TokenDelta { text } => {
                let lines_before = self.output_lines.len();
                if self.has_partial {
                    self.output_lines.pop_back();
                    self.has_partial = false;
                }
                self.partial_line.push_str(&text);
                while let Some(pos) = self.partial_line.find('\n') {
                    let completed: String = self.partial_line.drain(..=pos).collect();
                    let trimmed = completed.trim_end_matches('\n').to_string();
                    self.output_lines.push_back(OutputLine::Assistant(trimmed));
                }
                if !self.partial_line.is_empty() {
                    self.output_lines.push_back(OutputLine::Assistant(self.partial_line.clone()));
                    self.has_partial = true;
                }
                stream_debug_log(&format!(
                    "TokenDelta: text_len={} output_lines {} -> {} has_partial={}",
                    text.len(), lines_before, self.output_lines.len(), self.has_partial
                ));
            }

            ProgressEvent::ExecutionFinished { completed, blocked } => {
                self.finished = Some((completed, blocked));
                self.current_step = None;
            }

            ProgressEvent::ModelDetected { model } => {
                self.model = Some(model);
            }

            ProgressEvent::PhaseTransition { from, to } => {
                self.flush_partial();
                self.phase_id = to.clone();
                self.reload_tracks();
                self.output_lines.push_back(OutputLine::Label(
                    format!("━━━ Phase {} → {} ━━━", from, to),
                ));
            }

            ProgressEvent::BudgetExhausted { spent, limit } => {
                self.flush_partial();
                self.output_lines.push_back(OutputLine::Label(
                    format!("Budget exhausted: ${:.4} of ${:.4}", spent, limit),
                ));
            }

            ProgressEvent::CostUpdate { spent, limit } => {
                self.cost = Some((spent, limit));
            }
        }

        while self.output_lines.len() > MAX_OUTPUT_LINES {
            self.output_lines.pop_front();
        }
    }
}

// ── App ───────────────────────────────────────────────────────────────────────

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pub running: bool,
    restored: bool,
    pub dashboard: DashboardState,
    pub focused_panel: Panel,
    receiver: Option<EventReceiver>,
    pub stop_signal: Arc<AtomicBool>,
    pub paused: Arc<AtomicBool>,
}

pub fn init(
    dashboard: DashboardState,
    receiver: EventReceiver,
    stop_signal: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
) -> Result<App> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(App {
        terminal,
        running: true,
        restored: false,
        dashboard,
        focused_panel: Panel::Output,
        receiver: Some(receiver),
        stop_signal,
        paused,
    })
}

pub fn is_interactive() -> bool {
    if std::env::var("MZ_NO_TUI").is_ok() {
        return false;
    }
    if std::env::var("CI").is_ok() {
        return false;
    }
    if std::env::var("TERM").map(|v| v == "dumb").unwrap_or(false) {
        return false;
    }
    io::stdout().is_terminal()
}

pub fn run_with_tui<F>(project_state: ProjectState, task: F) -> Result<()>
where
    F: Fn(Option<EventSender>, Arc<AtomicBool>, Arc<AtomicBool>) -> Result<()> + Send + Sync + 'static,
{
    run_with_tui_phase(project_state, None, task)
}

pub fn run_with_tui_phase<F>(project_state: ProjectState, override_phase: Option<&str>, task: F) -> Result<()>
where
    F: Fn(Option<EventSender>, Arc<AtomicBool>, Arc<AtomicBool>) -> Result<()> + Send + Sync + 'static,
{
    let task = Arc::new(task);
    let phase_id = override_phase
        .map(|s| s.to_string())
        .unwrap_or_else(|| project_state.current_phase().to_string());

    let (tx, rx) = crate::events::channel();
    let mut dashboard = DashboardState::from_project_state(&project_state);
    // Override displayed phase if specified
    if let Some(pid) = override_phase {
        dashboard.phase_id = pid.to_string();
        // If the phase doesn't have tracks yet (e.g. planning), clear stale tracks
        let has_phase = project_state.phases.iter().any(|p| p.id == pid);
        if !has_phase {
            dashboard.tracks.clear();
        }
    }
    let stop_signal = Arc::new(AtomicBool::new(false));
    let paused = Arc::new(AtomicBool::new(false));
    let mut app = init(dashboard, rx, Arc::clone(&stop_signal), Arc::clone(&paused))?;

    // Fall back to non-TUI if terminal is too small
    let size = app.terminal.size()?;
    if size.width < MIN_TUI_WIDTH || size.height < MIN_TUI_HEIGHT {
        app.restore()?;
        eprintln!("Terminal too small for TUI, falling back to text output");
        return task(None, stop_signal, paused);
    }

    let task_clone = Arc::clone(&task);
    let stop_for_thread = Arc::clone(&stop_signal);
    let paused_for_thread = Arc::clone(&paused);
    let mut handle = std::thread::spawn(move || task_clone(Some(tx), stop_for_thread, paused_for_thread));

    loop {
        // Drain pending events
        if let Some(ref rx) = app.receiver {
            let mut dbg_drained: usize = 0;
            while let Ok(event) = rx.try_recv() {
                dbg_drained += 1;
                app.dashboard.update(event);
            }
            if dbg_drained > 0 {
                stream_debug_log(&format!(
                    "drain: {} events, output_lines={}",
                    dbg_drained, app.dashboard.output_lines.len()
                ));
            }
        }

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        app.running = false;
                        app.stop_signal.store(true, Ordering::Relaxed);
                    }
                    KeyCode::Char('p') => {
                        // Only pause while execution is running (not finished)
                        if app.dashboard.finished.is_none() {
                            app.paused.store(true, Ordering::Relaxed);
                        }
                    }
                    KeyCode::Char('r') => {
                        match app.dashboard.finished {
                            Some((_, blocked)) if blocked > 0 => {
                                // Reset blocked steps and restart execution
                                let _ = crate::state::reset_blocked_steps(&phase_id);

                                let (new_tx, new_rx) = crate::events::channel();
                                app.receiver = Some(new_rx);
                                app.dashboard.finished = None;

                                let new_stop = Arc::new(AtomicBool::new(false));
                                app.stop_signal = Arc::clone(&new_stop);

                                let new_paused = Arc::new(AtomicBool::new(false));
                                app.paused = Arc::clone(&new_paused);

                                let task_retry = Arc::clone(&task);
                                handle = std::thread::spawn(move || {
                                    task_retry(Some(new_tx), new_stop, new_paused)
                                });
                            }
                            _ => {
                                // Resume from pause (works when running or finished-complete)
                                app.paused.store(false, Ordering::Relaxed);
                            }
                        }
                    }
                    KeyCode::Tab => {
                        app.focused_panel = app.focused_panel.next();
                    }
                    KeyCode::Char('j') | KeyCode::Down => match app.focused_panel {
                        Panel::Output => {
                            app.dashboard.user_scrolled = true;
                            app.dashboard.scroll_offset =
                                app.dashboard.scroll_offset.saturating_add(1);
                        }
                        Panel::Tracks => {
                            app.dashboard.track_scroll_offset =
                                app.dashboard.track_scroll_offset.saturating_add(1);
                        }
                        Panel::Progress => {}
                    },
                    KeyCode::Char('k') | KeyCode::Up => match app.focused_panel {
                        Panel::Output => {
                            app.dashboard.user_scrolled = true;
                            app.dashboard.scroll_offset =
                                app.dashboard.scroll_offset.saturating_sub(1);
                        }
                        Panel::Tracks => {
                            app.dashboard.track_scroll_offset =
                                app.dashboard.track_scroll_offset.saturating_sub(1);
                        }
                        Panel::Progress => {}
                    },
                    _ => {}
                }
            }
        }

        app.draw()?;

        if !app.running {
            break;
        }
    }

    app.draw()?;
    app.restore()?;

    handle
        .join()
        .map_err(|_| anyhow::anyhow!("background thread panicked"))?
}

impl App {
    pub fn restore(&mut self) -> Result<()> {
        if self.restored {
            return Ok(());
        }
        self.restored = true;
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        // Pre-compute scroll offsets using terminal size.
        {
            let term_size = self.terminal.size()?;
            let track_height = (self.dashboard.tracks.len() as u16 + 2).max(4).min(8);
            let middle_height = 4u16;
            let layout_chunks = Layout::vertical([
                Constraint::Length(track_height),
                Constraint::Length(middle_height),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(ratatui::layout::Rect::new(0, 0, term_size.width, term_size.height));
            let output_inner_width = layout_chunks[2].width.saturating_sub(2);
            let output_inner_height = layout_chunks[2].height.saturating_sub(2);

            let visual_line_count: u16 = self.dashboard.output_lines.iter().map(|line| {
                let text = match line {
                    OutputLine::Plain(s) | OutputLine::Assistant(s) | OutputLine::Label(s) => s.clone(),
                    OutputLine::ToolUse(s) => format!("🔧 {}", s),
                    OutputLine::ToolResult(s) => format!("  ✓ {}", s),
                };
                let display_len = text.width();
                let w = output_inner_width as usize;
                if w == 0 || display_len == 0 { 1u16 }
                else { ((display_len + w - 1) / w) as u16 }
            }).sum();
            let total = visual_line_count;
            let max_scroll = total.saturating_sub(output_inner_height);

            // Clamp track scroll offset
            let max_track_scroll = self.dashboard.tracks.len().saturating_sub(6);
            if self.dashboard.track_scroll_offset as usize > max_track_scroll {
                self.dashboard.track_scroll_offset = max_track_scroll as u16;
            }

            if !self.dashboard.user_scrolled {
                // Auto-scroll to bottom
                self.dashboard.scroll_offset = max_scroll;
            } else if self.dashboard.scroll_offset >= max_scroll {
                // User scrolled back to bottom — re-enable auto-scroll
                self.dashboard.user_scrolled = false;
                self.dashboard.scroll_offset = max_scroll;
            }
        }

        let dashboard = &self.dashboard;
        let tracks: Vec<_> = dashboard.tracks.iter().collect();
        let finished = dashboard.finished;
        let focused = &self.focused_panel;
        let is_paused = self.paused.load(Ordering::Relaxed);
        let model_name = dashboard.model.as_deref().map(|m| {
            // Strip date suffix: "claude-sonnet-4-20250514" → "claude-sonnet-4"
            // Date suffixes are 8-digit numbers at the end after a dash
            let parts: Vec<&str> = m.split('-').collect();
            let strip_count = parts.iter().rev().take_while(|p| p.chars().all(|c| c.is_ascii_digit()) && p.len() == 8).count();
            if strip_count > 0 {
                parts[..parts.len() - strip_count].join("-")
            } else {
                m.to_string()
            }
        });
        let elapsed = dashboard.start_time.elapsed();

        self.terminal.draw(|frame| {
            let area = frame.area();

            // If the terminal has been resized below minimums, show a message and wait for resize
            if area.width < MIN_TUI_WIDTH || area.height < MIN_TUI_HEIGHT {
                let msg = "Terminal too small — resize to continue";
                let y = area.height / 2;
                let rect = ratatui::layout::Rect::new(area.x, area.y + y, area.width, 1);
                let para = Paragraph::new(msg)
                    .alignment(ratatui::layout::Alignment::Center)
                    .style(Style::default().fg(Color::Yellow));
                frame.render_widget(para, rect);
                return;
            }

            // Four vertical chunks: top=track overview, middle=step progress, bottom=output, footer=status bar
            let track_height = (tracks.len() as u16 + 2).max(4).min(8); // borders + content, capped at 8
            let chunks = Layout::vertical([
                Constraint::Length(track_height),
                Constraint::Length(4),
                Constraint::Min(5),
                Constraint::Length(1),
            ])
            .split(area);

            // Helper: border style based on focus
            let border_style = |panel: &Panel| -> Style {
                if panel == focused {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }
            };

            // ── Top panel: track overview ─────────────────────────────────────
            let track_lines: Vec<Line> = tracks
                .iter()
                .map(|t| {
                    let progress = format!(" ({}/{})", t.steps_done, t.steps_total);
                    let (icon_span, text_span) = match t.status {
                        TrackStatus::Done => (
                            Span::styled(" ✓ ", Style::default().fg(Color::Green)),
                            Span::styled(
                                format!("{} — {}", t.id, t.title),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ),
                        TrackStatus::Active => (
                            Span::styled(" ▶ ", Style::default().fg(Color::Yellow)),
                            Span::styled(
                                format!("{} — {}", t.id, t.title),
                                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                            ),
                        ),
                        TrackStatus::Pending => (
                            Span::styled(" ○ ", Style::default().fg(Color::DarkGray)),
                            Span::styled(
                                format!("{} — {}", t.id, t.title),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ),
                        TrackStatus::Blocked => (
                            Span::styled(" ✗ ", Style::default().fg(Color::Red)),
                            Span::styled(
                                format!("{} — {}", t.id, t.title),
                                Style::default().fg(Color::Red),
                            ),
                        ),
                    };
                    Line::from(vec![
                        icon_span,
                        text_span,
                        Span::styled(progress, Style::default().fg(Color::DarkGray)),
                    ])
                })
                .collect();

            let phase_title = format!(" Tracks — {} ", dashboard.phase_id);
            let all_lines = track_lines;
            let overview = Paragraph::new(all_lines)
                .block(
                    Block::default()
                        .title(phase_title)
                        .borders(Borders::ALL)
                        .border_style(border_style(&Panel::Tracks)),
                )
                .scroll((dashboard.track_scroll_offset, 0));
            frame.render_widget(overview, chunks[0]);

            // ── Middle panel: step progress ───────────────────────────────────
            {
                let all_steps_done: usize = tracks.iter().map(|t| t.steps_done).sum();
                let all_steps_total: usize = tracks.iter().map(|t| t.steps_total).sum();
                let tracks_done =
                    tracks.iter().filter(|t| t.status == TrackStatus::Done).count();
                let tracks_total = tracks.len();
                let current_step = &dashboard.current_step;

                let progress_title = if is_paused { " Progress [PAUSED] " } else { " Progress " };
                let progress_block = Block::bordered()
                    .title(progress_title)
                    .border_style(border_style(&Panel::Progress));

                if let Some((_, blocked)) = finished {
                    let color = if blocked > 0 { Color::Red } else { Color::Green };
                    let msg = if blocked > 0 {
                        format!(" {} step(s) blocked — press q to exit", blocked)
                    } else {
                        " Phase complete — press q to exit".to_string()
                    };
                    let para = Paragraph::new(Line::from(Span::styled(
                        msg,
                        Style::default().fg(color),
                    )))
                    .block(progress_block);
                    frame.render_widget(para, chunks[1]);
                } else {
                    let inner = progress_block.inner(chunks[1]);
                    frame.render_widget(progress_block, chunks[1]);

                    let inner_chunks = Layout::vertical([
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .split(inner);

                    // Line 1: step/track counter
                    let step_line = match current_step {
                        Some(step) => {
                            let track_num = tracks
                                .iter()
                                .position(|t| t.id == step.track_id)
                                .map(|i| i + 1)
                                .unwrap_or(1)
                                .min(tracks_total);
                            let elapsed = step.started_at.elapsed().as_secs();
                            Line::from(vec![
                                Span::styled(" Step ", Style::default().fg(Color::DarkGray)),
                                Span::styled(
                                    format!("{}/{}", step.step_num, step.total_steps),
                                    Style::default().fg(Color::White),
                                ),
                                Span::styled("  Track ", Style::default().fg(Color::DarkGray)),
                                Span::styled(
                                    format!("{}/{}", track_num, tracks_total),
                                    Style::default().fg(Color::White),
                                ),
                                Span::styled(
                                    format!("  [{}m {:02}s]", elapsed / 60, elapsed % 60),
                                    Style::default().fg(Color::DarkGray),
                                ),
                            ])
                        }
                        None => Line::from(Span::styled(
                            " Idle",
                            Style::default().fg(Color::DarkGray),
                        )),
                    };
                    frame.render_widget(Paragraph::new(step_line), inner_chunks[0]);

                    // Line 2: progress gauge
                    let ratio = if all_steps_total > 0 {
                        (all_steps_done as f64 / all_steps_total as f64).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let gauge_label = format!("{}/{}", all_steps_done, all_steps_total);
                    let gauge = Gauge::default()
                        .ratio(ratio)
                        .label(gauge_label)
                        .gauge_style(Style::default().fg(Color::Green));
                    frame.render_widget(gauge, inner_chunks[1]);
                }
            }

            // ── Bottom panel: Claude output ───────────────────────────────────
            {
                let output_block = Block::bordered()
                    .title(" Output ")
                    .border_style(border_style(&Panel::Output));
                let output_lines: Vec<Line> = dashboard
                    .output_lines
                    .iter()
                    .map(|l| match l {
                        OutputLine::Plain(s) => Line::from(Span::raw(s.clone())),
                        OutputLine::Assistant(s) => Line::from(Span::styled(
                            s.clone(),
                            Style::default().fg(Color::White),
                        )),
                        OutputLine::ToolUse(s) => Line::from(Span::styled(
                            format!("🔧 {}", s),
                            Style::default().fg(Color::Cyan),
                        )),
                        OutputLine::ToolResult(s) => Line::from(Span::styled(
                            format!("  ✓ {}", s),
                            Style::default().fg(Color::DarkGray),
                        )),
                        OutputLine::Label(s) => {
                            let panel_width = chunks[2].width.saturating_sub(2);
                            let label = format!(" {} ", s);
                            // BUG-22 fix: use display width, not byte length
                            let label_len = label.width() as u16;
                            let left = panel_width.saturating_sub(label_len) / 2;
                            let right = panel_width.saturating_sub(left + label_len);
                            let separator = format!(
                                "{}{}{}",
                                "─".repeat(left as usize),
                                label,
                                "─".repeat(right as usize)
                            );
                            Line::from(Span::styled(
                                separator,
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::BOLD),
                            ))
                        }
                    })
                    .collect();
                let output_para = Paragraph::new(output_lines)
                    .block(output_block)
                    .wrap(Wrap { trim: false })
                    .scroll((dashboard.scroll_offset, 0));
                frame.render_widget(output_para, chunks[2]);
            }

            // ── Status bar (always visible) ───────────────────────────────────
            {
                let bar_width = chunks[3].width as usize;

                // Left: elapsed time or completion state
                let (left_text, left_color) = match finished {
                    Some((_, blocked)) if blocked > 0 => {
                        (format!(" ⚠ {} step(s) blocked", blocked), Color::Yellow)
                    }
                    Some(_) => (" ✓ Phase complete".to_string(), Color::Green),
                    None => {
                        let secs = elapsed.as_secs();
                        (format!(" {:02}:{:02}", secs / 60, secs % 60), Color::White)
                    }
                };

                // Center: model name (or empty)
                let center_text = model_name.as_deref().unwrap_or("").to_string();

                // Right: keybindings
                let right_text = if is_paused {
                    "r:resume  q:stop "
                } else {
                    match finished {
                        Some((_, blocked)) if blocked > 0 => "q:quit  r:retry ",
                        Some(_) => "q:quit ",
                        None => "q:stop  p:pause  Tab:focus  j/k:scroll ",
                    }
                };

                // Cost display (after elapsed time)
                let (cost_text, cost_color) = match dashboard.cost {
                    Some((spent, Some(limit))) => {
                        let text = format!("  ${:.2}/${:.2}", spent, limit);
                        let color = if spent >= limit {
                            Color::Red
                        } else if spent / limit >= 0.8 {
                            Color::Yellow
                        } else {
                            Color::White
                        };
                        (text, color)
                    }
                    Some((spent, None)) => (format!("  ${:.2}", spent), Color::White),
                    None => (String::new(), Color::White),
                };

                // BUG-23 fix: use display width, not char count
                let left_len = left_text.width();
                let cost_len = cost_text.width();
                let center_len = center_text.width();
                let right_len = right_text.width();
                let total_left_len = left_len + cost_len;

                // Padding: center the model name, right-align the keybindings
                let left_pad = if center_len > 0 {
                    bar_width.saturating_sub(total_left_len + center_len + right_len) / 2
                } else {
                    bar_width.saturating_sub(total_left_len + right_len)
                };
                let right_pad = bar_width.saturating_sub(total_left_len + left_pad + center_len + right_len);

                let bar_style = Style::default().bg(Color::DarkGray).fg(Color::White);
                let mut spans = vec![
                    Span::styled(left_text, Style::default().bg(Color::DarkGray).fg(left_color)),
                ];
                if !cost_text.is_empty() {
                    spans.push(Span::styled(cost_text, Style::default().bg(Color::DarkGray).fg(cost_color)));
                }
                spans.push(Span::styled(" ".repeat(left_pad), bar_style));
                if !center_text.is_empty() {
                    spans.push(Span::styled(center_text, Style::default().bg(Color::DarkGray).fg(Color::Cyan)));
                }
                spans.push(Span::styled(" ".repeat(right_pad), bar_style));
                spans.push(Span::styled(right_text, Style::default().bg(Color::DarkGray).fg(Color::White)));

                let status_bar = Paragraph::new(Line::from(spans)).style(bar_style);
                frame.render_widget(status_bar, chunks[3]);
            }
        })?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dashboard() -> DashboardState {
        DashboardState {
            phase_id: "test".into(),
            tracks: vec![],
            current_step: None,
            output_lines: VecDeque::new(),
            finished: None,
            scroll_offset: 0,
            user_scrolled: false,
            track_scroll_offset: 0,
            partial_line: String::new(),
            has_partial: false,
            model: None,
            start_time: Instant::now(),
            cost: None,
        }
    }

    #[test]
    fn test_token_streaming_accumulation() {
        let mut d = make_dashboard();
        d.update(ProgressEvent::TokenDelta { text: "Hello ".into() });
        d.update(ProgressEvent::TokenDelta { text: "world\n".into() });
        d.update(ProgressEvent::TokenDelta { text: "Line 2".into() });

        assert_eq!(d.output_lines.len(), 2);
        assert!(matches!(&d.output_lines[0], OutputLine::Assistant(s) if s == "Hello world"));
        assert!(matches!(&d.output_lines[1], OutputLine::Assistant(s) if s == "Line 2"));
        assert!(d.has_partial);
    }

    #[test]
    fn test_tool_use_then_result_pairing() {
        let mut d = make_dashboard();
        d.update(ProgressEvent::ToolUseStarted { tool: "Read src/main.rs".into() });
        d.update(ProgressEvent::ToolResultReceived { tool: "Read src/main.rs".into() });

        assert_eq!(d.output_lines.len(), 2);
        assert!(matches!(&d.output_lines[0], OutputLine::ToolUse(s) if s == "Read src/main.rs"));
        assert!(matches!(&d.output_lines[1], OutputLine::ToolResult(s) if s == "Read src/main.rs"));
    }

    #[test]
    fn test_model_detected() {
        let mut d = make_dashboard();
        d.update(ProgressEvent::ModelDetected { model: "claude-sonnet-4-20250514".into() });
        assert_eq!(d.model, Some("claude-sonnet-4-20250514".into()));
    }

    #[test]
    fn test_partial_line_flush_on_non_delta_event() {
        let mut d = make_dashboard();
        d.update(ProgressEvent::TokenDelta { text: "partial".into() });
        d.update(ProgressEvent::ToolUseStarted { tool: "Bash ls".into() });

        assert!(!d.has_partial);
        assert_eq!(d.output_lines.len(), 2);
        assert!(matches!(&d.output_lines[0], OutputLine::Assistant(s) if s == "partial"));
        assert!(matches!(&d.output_lines[1], OutputLine::ToolUse(s) if s == "Bash ls"));
    }
}
