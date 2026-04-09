use std::io::{self, IsTerminal, Stdout};
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
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

use crate::events::{EventReceiver, EventSender, ProgressEvent};
use crate::state::{ProjectState, StepStatus};

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
    pub phase_id: String,
    pub track_id: String,
    pub step_id: String,
    pub title: String,
    pub step_num: usize,
    pub total_steps: usize,
    pub started_at: Instant,
}

const MAX_OUTPUT_LINES: usize = 1000;

pub struct DashboardState {
    pub tracks: Vec<TrackInfo>,
    pub current_step: Option<StepInfo>,
    pub output_lines: Vec<String>,
    pub finished: Option<(usize, usize)>,
    pub scroll_offset: u16,
    pub auto_scroll: bool,
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
            auto_scroll: true,
        }
    }

    pub fn update(&mut self, event: ProgressEvent) {
        match event {
            ProgressEvent::PhaseStarted { .. } => {}

            ProgressEvent::TrackStarted { track_id, .. } => {
                if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
                    t.status = TrackStatus::Active;
                }
            }

            ProgressEvent::StepStarted {
                phase_id,
                track_id,
                step_id,
                step_title,
                step_num,
                total_steps,
            } => {
                if let Some(t) = self.tracks.iter_mut().find(|t| t.id == track_id) {
                    t.status = TrackStatus::Active;
                }
                self.current_step = Some(StepInfo {
                    phase_id,
                    track_id,
                    step_id,
                    title: step_title,
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
                self.output_lines.push(line);
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
    receiver: Option<EventReceiver>,
}

pub fn init(dashboard: DashboardState, receiver: EventReceiver) -> Result<App> {
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
        receiver: Some(receiver),
    })
}

pub fn is_interactive() -> bool {
    io::stdout().is_terminal()
}

pub fn run_with_tui<F>(project_state: ProjectState, task: F) -> Result<()>
where
    F: FnOnce(Option<EventSender>) -> Result<()> + Send + 'static,
{
    let (tx, rx) = crate::events::channel();
    let dashboard = DashboardState::from_project_state(&project_state);
    let mut app = init(dashboard, rx)?;

    let handle = std::thread::spawn(move || task(Some(tx)));

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
                    KeyCode::Char('q') => app.running = false,
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.dashboard.auto_scroll = false;
                        app.dashboard.scroll_offset =
                            app.dashboard.scroll_offset.saturating_add(1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.dashboard.auto_scroll = false;
                        app.dashboard.scroll_offset =
                            app.dashboard.scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Char('G') => {
                        app.dashboard.auto_scroll = true;
                    }
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
        // Pre-compute auto-scroll offset using terminal size so we can update it
        // before borrowing dashboard immutably for the render closure.
        {
            let term_size = self.terminal.size()?;
            let track_height = (self.dashboard.tracks.len() as u16 + 2).max(4);
            let middle_height = 4u16;
            // Output inner height = total - track panel - middle panel - 2 (output borders)
            let output_inner_height = term_size
                .height
                .saturating_sub(track_height)
                .saturating_sub(middle_height)
                .saturating_sub(2);
            if self.dashboard.auto_scroll {
                let total = self.dashboard.output_lines.len() as u16;
                self.dashboard.scroll_offset = total.saturating_sub(output_inner_height);
            }
        }

        let dashboard = &self.dashboard;
        let tracks: Vec<_> = dashboard.tracks.iter().collect();
        let finished = dashboard.finished;

        self.terminal.draw(|frame| {
            let area = frame.area();

            // Three vertical chunks: top=track overview, middle=step progress, bottom=output
            let track_height = (tracks.len() as u16 + 2).max(4); // borders + content
            let chunks = Layout::vertical([
                Constraint::Length(track_height),
                Constraint::Length(4),
                Constraint::Min(5),
            ])
            .split(area);

            // ── Top panel: track overview ─────────────────────────────────────
            let track_lines: Vec<Line> = tracks
                .iter()
                .map(|t| {
                    let (icon, color) = match t.status {
                        TrackStatus::Done => ("✓", Color::Green),
                        TrackStatus::Active => ("▶", Color::Yellow),
                        TrackStatus::Pending => ("○", Color::DarkGray),
                        TrackStatus::Blocked => ("✗", Color::Red),
                    };
                    let progress = match t.status {
                        TrackStatus::Blocked => format!(" ({}/{})", t.steps_done, t.steps_total),
                        _ => format!(" ({}/{})", t.steps_done, t.steps_total),
                    };
                    Line::from(vec![
                        Span::styled(
                            format!(" {} {} — {}", icon, t.id, t.title),
                            Style::default().fg(color),
                        ),
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

            let overview = Paragraph::new(all_lines).block(
                Block::default().title(" Tracks ").borders(Borders::ALL),
            );
            frame.render_widget(overview, chunks[0]);

            // ── Middle panel: step progress ───────────────────────────────────
            {
                let all_steps_done: usize = tracks.iter().map(|t| t.steps_done).sum();
                let all_steps_total: usize = tracks.iter().map(|t| t.steps_total).sum();
                let tracks_done =
                    tracks.iter().filter(|t| t.status == TrackStatus::Done).count();
                let tracks_total = tracks.len();
                let current_step = &dashboard.current_step;

                let progress_block = Block::bordered().title(" Progress ");

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
                let output_block = Block::bordered().title(" Output ");
                let output_lines: Vec<Line> = dashboard
                    .output_lines
                    .iter()
                    .map(|l| Line::from(l.as_str()))
                    .collect();
                let output_para = Paragraph::new(output_lines)
                    .block(output_block)
                    .scroll((dashboard.scroll_offset, 0));
                frame.render_widget(output_para, chunks[2]);
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
