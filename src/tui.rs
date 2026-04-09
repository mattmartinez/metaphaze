use std::io::{self, IsTerminal, Stdout};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use anyhow::Result;
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

const MAX_OUTPUT_LINES: usize = 1000;

#[derive(Debug, Clone)]
pub enum OutputLine {
    Plain(String),
    Assistant(String),
    ToolUse(String),
    ToolResult(String),
    Label(String),
}

pub struct DashboardState {
    pub tracks: Vec<TrackInfo>,
    pub current_step: Option<StepInfo>,
    pub output_lines: Vec<OutputLine>,
    pub finished: Option<(usize, usize)>,
    pub scroll_offset: u16,
    pub user_scrolled: bool,
    pub track_scroll_offset: u16,
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
            tracks,
            current_step: None,
            output_lines: Vec::new(),
            finished: None,
            scroll_offset: 0,
            user_scrolled: false,
            track_scroll_offset: 0,
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

            ProgressEvent::StepCompleted { track_id, .. } => {
                if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
                    t.steps_done = t.steps_done.saturating_add(1);
                }
                self.current_step = None;
            }

            ProgressEvent::StepFailed { track_id, .. } => {
                if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
                    if t.steps_done == t.steps_total.saturating_sub(1) {
                        t.status = TrackStatus::Blocked;
                    }
                }
            }

            ProgressEvent::TrackCompleted { track_id, .. } => {
                if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
                    t.status = TrackStatus::Done;
                    t.steps_done = t.steps_total;
                }
            }

            ProgressEvent::ClaudeOutput { line } => {
                self.output_lines.push(OutputLine::Plain(line));
                if self.output_lines.len() > MAX_OUTPUT_LINES {
                    self.output_lines.remove(0);
                }
            }

            ProgressEvent::AssistantText { text } => {
                self.output_lines.push(OutputLine::Assistant(text));
                if self.output_lines.len() > MAX_OUTPUT_LINES {
                    self.output_lines.remove(0);
                }
            }

            ProgressEvent::ToolUseStarted { tool } => {
                self.output_lines.push(OutputLine::ToolUse(tool));
                if self.output_lines.len() > MAX_OUTPUT_LINES {
                    self.output_lines.remove(0);
                }
            }

            ProgressEvent::ToolResultReceived { tool } => {
                self.output_lines.push(OutputLine::ToolResult(tool));
                if self.output_lines.len() > MAX_OUTPUT_LINES {
                    self.output_lines.remove(0);
                }
            }

            ProgressEvent::PhaseLabel { label } => {
                self.output_lines.push(OutputLine::Label(label));
                if self.output_lines.len() > MAX_OUTPUT_LINES {
                    self.output_lines.remove(0);
                }
            }

            ProgressEvent::ExecutionFinished { completed, blocked } => {
                self.finished = Some((completed, blocked));
                self.current_step = None;
            }
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
    let task = Arc::new(task);
    let phase_id = project_state.current_phase().to_string();

    let (tx, rx) = crate::events::channel();
    let dashboard = DashboardState::from_project_state(&project_state);
    let stop_signal = Arc::new(AtomicBool::new(false));
    let paused = Arc::new(AtomicBool::new(false));
    let mut app = init(dashboard, rx, Arc::clone(&stop_signal), Arc::clone(&paused))?;

    let task_clone = Arc::clone(&task);
    let stop_for_thread = Arc::clone(&stop_signal);
    let paused_for_thread = Arc::clone(&paused);
    let mut handle = std::thread::spawn(move || task_clone(Some(tx), stop_for_thread, paused_for_thread));

    loop {
        // Drain pending events
        if let Some(ref rx) = app.receiver {
            while let Ok(event) = rx.try_recv() {
                app.dashboard.update(event);
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
            // Output inner height = total - track panel - middle panel - 2 (output borders)
            let output_inner_height = term_size
                .height
                .saturating_sub(track_height)
                .saturating_sub(middle_height)
                .saturating_sub(2);

            // TODO: wrapped line count for precise manual scroll
            // With line wrapping enabled, long lines occupy multiple visual rows, so
            // output_lines.len() underestimates the true visual height. Getting the exact
            // wrapped count requires knowing the panel width at pre-compute time (complex).
            // Auto-scroll still works correctly; manual scroll may slightly overestimate max.
            let total = self.dashboard.output_lines.len() as u16;
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

        self.terminal.draw(|frame| {
            let area = frame.area();

            // Four vertical chunks: top=track overview, middle=step progress, bottom=output, footer=status
            let track_height = (tracks.len() as u16 + 2).max(4).min(8); // borders + content, capped at 8
            let footer_height = if finished.is_some() { 1u16 } else { 0u16 };
            let chunks = Layout::vertical([
                Constraint::Length(track_height),
                Constraint::Length(4),
                Constraint::Min(5),
                Constraint::Length(footer_height),
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

            let footer_line = match finished {
                Some((_, blocked)) if blocked > 0 => {
                    Some(Line::from(Span::styled(
                        format!(" {} step(s) blocked — press q to exit", blocked),
                        Style::default().fg(Color::Red),
                    )))
                }
                Some(_) => Some(Line::from(Span::styled(
                    " Phase complete — press q to exit",
                    Style::default().fg(Color::Green),
                ))),
                None => None,
            };

            let mut all_lines = track_lines;
            if let Some(fl) = footer_line {
                all_lines.push(Line::from(""));
                all_lines.push(fl);
            }

            let overview = Paragraph::new(all_lines)
                .block(
                    Block::default()
                        .title(" Tracks ")
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
                                .unwrap_or(tracks_done + 1);
                            format!(
                                " Step {}/{} · Track {}/{}",
                                step.step_num, step.total_steps, track_num, tracks_total
                            )
                        }
                        None => " Idle".to_string(),
                    };
                    frame.render_widget(Paragraph::new(step_line), inner_chunks[0]);

                    // Line 2: gauge with elapsed time label
                    let ratio = if all_steps_total > 0 {
                        (all_steps_done as f64 / all_steps_total as f64).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let elapsed_str = match current_step {
                        Some(step) => {
                            let secs = step.started_at.elapsed().as_secs();
                            format!(" [{}m {:02}s]", secs / 60, secs % 60)
                        }
                        None => String::new(),
                    };
                    let gauge_label = format!("{}/{}{}", all_steps_done, all_steps_total, elapsed_str);
                    let gauge = Gauge::default().ratio(ratio).label(gauge_label);
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
                        OutputLine::Label(s) => Line::from(Span::styled(
                            s.clone(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )),
                    })
                    .collect();
                let output_para = Paragraph::new(output_lines)
                    .block(output_block)
                    .wrap(Wrap { trim: true })
                    .scroll((dashboard.scroll_offset, 0));
                frame.render_widget(output_para, chunks[2]);
            }

            // ── Footer: end-of-run status ─────────────────────────────────────
            if let Some((_, blocked)) = finished {
                let (msg, color) = if blocked > 0 {
                    (
                        format!(" {} step(s) blocked — press q to exit, r to retry", blocked),
                        Color::Yellow,
                    )
                } else {
                    (
                        " Phase complete — press q to exit".to_string(),
                        Color::Green,
                    )
                };
                let footer = Paragraph::new(Line::from(Span::styled(
                    msg,
                    Style::default().fg(color),
                )));
                frame.render_widget(footer, chunks[3]);
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
