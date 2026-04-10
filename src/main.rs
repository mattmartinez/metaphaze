mod claude;
mod discuss;
mod events;
mod stream;
mod tui;
mod executor;
mod git;
mod planner;
mod prompt;
mod state;
mod verifier;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mz", version, about = "A spec-driven context engine for Claude Code")]
struct Cli {
    /// Force raw terminal output — skip TUI even in interactive terminals
    #[arg(long, global = true)]
    no_tui: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project — interactive setup
    Init,

    /// Deep interactive discussion to capture decisions and resolve ambiguity
    Discuss {
        /// Phase to discuss (e.g., P001). Defaults to current.
        phase: Option<String>,
    },

    /// Decompose current phase into tracks and steps
    Plan {
        /// Phase to plan (e.g., P001). Defaults to current.
        phase: Option<String>,
    },

    /// Execute the next pending step
    Next,

    /// Autonomous loop — execute all steps until phase complete or blocked
    Auto {
        /// Maximum steps to execute before stopping
        #[arg(long)]
        max_steps: Option<usize>,
    },

    /// Show current progress
    Status {
        /// Show step-level detail
        #[arg(long)]
        detail: bool,
    },

    /// Steer the project — record a decision and re-plan remaining work
    Steer {
        /// The decision or direction change
        message: String,
    },

    /// Reset a step back to pending (escape hatch for blocked or stuck steps)
    Reset {
        /// Step ID to reset — e.g. ST03 or TR02/ST01
        step_id: String,
        /// Phase to target (defaults to current phase)
        #[arg(long)]
        phase: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let no_tui = cli.no_tui;

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Discuss { phase } => cmd_discuss(phase),
        Commands::Plan { phase } => cmd_plan(phase, no_tui),
        Commands::Next => cmd_next(no_tui),
        Commands::Auto { max_steps } => cmd_auto(max_steps, no_tui),
        Commands::Status { detail } => cmd_status(detail),
        Commands::Steer { message } => cmd_steer(message),
        Commands::Reset { step_id, phase } => {
            let project_state = state::load()?;
            let phase_id = phase.unwrap_or_else(|| project_state.current_phase().to_string());
            state::reset_step(&phase_id, &step_id)
        }
    }
}

fn cmd_init() -> Result<()> {
    println!("Initializing new Metaphaze project...\n");
    state::init_project()?;
    println!("\nProject initialized at .mz/");
    println!("  PROJECT.md  — your project definition");
    println!("  STATE.md    — current state dashboard");
    println!("\nNext: run `mz discuss` to capture decisions, then `mz plan` to decompose into steps.");
    Ok(())
}

fn cmd_discuss(phase: Option<String>) -> Result<()> {
    let project_state = state::load()?;
    let phase_id = phase.unwrap_or_else(|| project_state.current_phase().to_string());
    println!("Starting discussion for {}...\n", phase_id);
    discuss::run(&project_state, &phase_id)
}

fn cmd_plan(phase: Option<String>, no_tui: bool) -> Result<()> {
    let project_state = state::load()?;

    if phase.is_none() {
        // No phase arg — generate the multi-phase roadmap
        if !no_tui && tui::is_interactive() {
            tui::run_with_tui_phase(project_state, None, move |sender, stop, paused| {
                cmd_plan_inner(None, sender, stop, paused)
            })
        } else {
            println!("Generating roadmap...\n");
            planner::generate_roadmap(&project_state, None)
        }
    } else {
        let phase_id = phase.unwrap();
        if !no_tui && tui::is_interactive() {
            let pid = phase_id.clone();
            let phase_id = std::sync::Arc::new(phase_id);
            tui::run_with_tui_phase(project_state, Some(&pid), move |sender, stop, paused| {
                cmd_plan_inner(Some((*phase_id).clone()), sender, stop, paused)
            })
        } else {
            println!("Planning {}...\n", phase_id);
            planner::run(&project_state, &phase_id, None)
        }
    }
}

fn cmd_plan_inner(
    phase_id: Option<String>,
    sender: Option<events::EventSender>,
    _stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    _paused: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    let project_state = state::load()?;

    if let Some(tx) = &sender {
        let _ = tx.send(events::ProgressEvent::PhaseStarted);
        let _ = tx.send(events::ProgressEvent::StepStarted {
            track_id: String::new(),
            step_num: 1,
            total_steps: 1,
        });
    }

    let result = match phase_id {
        None => planner::generate_roadmap(&project_state, sender.as_ref()),
        Some(ref pid) => planner::run(&project_state, pid, sender.as_ref()),
    };

    match result {
        Ok(()) => {
            if let Some(tx) = &sender {
                let _ = tx.send(events::ProgressEvent::StepCompleted {
                    track_id: String::new(),
                });
                let _ = tx.send(events::ProgressEvent::ExecutionFinished {
                    completed: 1,
                    blocked: 0,
                });
            }
            Ok(())
        }
        Err(e) => {
            if let Some(tx) = &sender {
                let _ = tx.send(events::ProgressEvent::StepFailed {
                    track_id: String::new(),
                });
                let _ = tx.send(events::ProgressEvent::ExecutionFinished {
                    completed: 0,
                    blocked: 1,
                });
            }
            Err(e)
        }
    }
}

fn cmd_next(no_tui: bool) -> Result<()> {
    if !no_tui && tui::is_interactive() {
        let project_state = state::load()?;
        tui::run_with_tui(project_state, move |sender, stop, paused| {
            cmd_next_inner(sender, stop, paused)
        })
    } else {
        let project_state = state::load()?;
        executor::run_next(&project_state, None)?;
        Ok(())
    }
}

fn cmd_next_inner(
    sender: Option<events::EventSender>,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    _paused: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    use std::sync::atomic::Ordering;

    let project_state = state::load()?;

    if let Some(tx) = &sender {
        let _ = tx.send(events::ProgressEvent::PhaseStarted);
    }

    match project_state.next_pending_step() {
        Some((phase_id, track_id, step_id)) => {
            if stop.load(Ordering::Relaxed) {
                if let Some(tx) = &sender {
                    let _ = tx.send(events::ProgressEvent::ExecutionFinished {
                        completed: 0,
                        blocked: 0,
                    });
                }
                return Ok(());
            }

            println!("Executing {}/{}/{}...\n", phase_id, track_id, step_id);

            if let Some(tx) = &sender {
                let step_title = project_state
                    .phases
                    .iter()
                    .find(|p| p.id == phase_id)
                    .and_then(|p| p.tracks.iter().find(|t| t.id == track_id))
                    .and_then(|t| {
                        t.steps.iter().enumerate().find(|(_, s)| s.id == step_id)
                    })
                    .map(|(i, s)| {
                        let total = project_state
                            .phases
                            .iter()
                            .find(|p| p.id == phase_id)
                            .and_then(|p| p.tracks.iter().find(|t| t.id == track_id))
                            .map(|t| t.steps.len())
                            .unwrap_or(0);
                        (i + 1, s.title.clone(), total)
                    });
                if let Some((step_num, _title, total)) = step_title {
                    let _ = tx.send(events::ProgressEvent::StepStarted {
                        track_id: track_id.clone(),
                        step_num,
                        total_steps: total,
                    });
                }
            }

            let run_result =
                executor::run_step(&project_state, &phase_id, &track_id, &step_id, sender.as_ref());

            let (completed, blocked) = match run_result {
                Ok(()) => {
                    // BUG-1 fix: only verify when execution succeeded
                    if let Some(tx) = &sender { let _ = tx.send(events::ProgressEvent::PhaseLabel { label: "── verify ──".into() }); }
                    let verify_ok = match verifier::run_step(&project_state, &phase_id, &track_id, &step_id, sender.as_ref()) {
                        Ok(()) => true,
                        Err(e) => {
                            emit_output(&sender, &format!("Verification failed: {}", e));
                            false
                        }
                    };
                    // BUG-17 fix: verification failure blocks step completion
                    if verify_ok {
                        state::mark_step_complete(&phase_id, &track_id, &step_id)?;
                        emit_output(&sender, "Step complete.");
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::StepCompleted {
                                track_id: track_id.clone(),
                            });
                        }
                        (1, 0)
                    } else {
                        state::mark_step_blocked(&phase_id, &track_id, &step_id, "Verification failed")?;
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::StepFailed {
                                track_id: track_id.clone(),
                            });
                        }
                        (0, 1)
                    }
                }
                Err(e) => {
                    emit_output(&sender, &format!("Step failed: {}", e));
                    if let Some(tx) = &sender {
                        let _ = tx.send(events::ProgressEvent::StepFailed {
                            track_id: track_id.clone(),
                        });
                    }
                    (0, 1)
                }
            };

            if let Some(tx) = &sender {
                let _ = tx.send(events::ProgressEvent::ExecutionFinished { completed, blocked });
            }
        }
        None => {
            println!("No pending steps.");
            if let Some(tx) = &sender {
                let _ = tx.send(events::ProgressEvent::ExecutionFinished {
                    completed: 0,
                    blocked: 0,
                });
            }
        }
    }

    Ok(())
}

fn cmd_auto(max_steps: Option<usize>, no_tui: bool) -> Result<()> {
    if !no_tui && tui::is_interactive() {
        let project_state = state::load()?;
        tui::run_with_tui(project_state, move |sender, stop, paused| {
            cmd_auto_inner(max_steps, sender, stop, paused)
        })
    } else {
        cmd_auto_inner(
            max_steps,
            None,
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        )
    }
}

fn emit_output(sender: &Option<events::EventSender>, msg: &str) {
    events::emit(sender.as_ref(), msg);
}

fn cmd_auto_inner(
    max_steps: Option<usize>,
    sender: Option<events::EventSender>,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    paused: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    use std::collections::HashMap;
    use std::sync::atomic::Ordering;

    let limit = max_steps.unwrap_or(usize::MAX);
    let mut completed = 0;
    let mut blocked = 0;
    let mut retries: HashMap<String, u32> = HashMap::new();
    let mut phase_started = false;

    emit_output(&sender, "Starting autonomous execution...");

    loop {
        if stop.load(Ordering::Relaxed) {
            emit_output(&sender, "Execution stopped by user.");
            break;
        }

        // Spin-wait while paused (check stop signal to allow clean exit while paused)
        while paused.load(Ordering::Relaxed) && !stop.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        if stop.load(Ordering::Relaxed) {
            emit_output(&sender, "Execution stopped by user.");
            break;
        }

        if completed >= limit {
            emit_output(&sender, &format!("Reached step limit ({}). Stopping.", limit));
            break;
        }

        let project_state = state::load()?;

        if !phase_started {
            phase_started = true;
            if let Some(tx) = &sender {
                let _ = tx.send(events::ProgressEvent::PhaseStarted);
            }
        }

        match project_state.next_pending_step() {
            Some((phase_id, track_id, step_id)) => {
                emit_output(&sender, &format!("━━━ {}/{}/{} ━━━", phase_id, track_id, step_id));

                if let Some(tx) = &sender {
                    let step_title = project_state
                        .phases
                        .iter()
                        .find(|p| p.id == phase_id)
                        .and_then(|p| p.tracks.iter().find(|t| t.id == track_id))
                        .and_then(|t| {
                            t.steps.iter().enumerate().find(|(_, s)| s.id == step_id)
                        })
                        .map(|(i, s)| (i + 1, s.title.clone(), {
                            project_state
                                .phases
                                .iter()
                                .find(|p| p.id == phase_id)
                                .and_then(|p| p.tracks.iter().find(|t| t.id == track_id))
                                .map(|t| t.steps.len())
                                .unwrap_or(0)
                        }));
                    if let Some((step_num, _title, total)) = step_title {
                        let _ = tx.send(events::ProgressEvent::StepStarted {
                            track_id: track_id.clone(),
                            step_num,
                            total_steps: total,
                        });
                    }
                }

                let key = format!("{}/{}", track_id, step_id);
                // BUG-21 fix: always resync from disk in case of external reset
                let disk_attempts = project_state.step_attempts(&phase_id, &track_id, &step_id);
                let count = retries.entry(key.clone()).or_insert(0);
                if disk_attempts < *count {
                    // External reset detected — honor it
                    *count = disk_attempts;
                } else {
                    *count = disk_attempts;
                }

                if let Err(e) = executor::run_step(&project_state, &phase_id, &track_id, &step_id, sender.as_ref()) {
                    emit_output(&sender, &format!("Step failed: {}", e));
                    *count += 1;
                    state::increment_step_attempts(&phase_id, &track_id, &step_id)?;

                    if let Some(tx) = &sender {
                        let _ = tx.send(events::ProgressEvent::StepFailed {
                            track_id: track_id.clone(),
                        });
                    }

                    if *count >= 3 {
                        emit_output(&sender, &format!("Step {} failed after 3 attempts. Marking blocked.", step_id));
                        state::mark_step_blocked(
                            &phase_id,
                            &track_id,
                            &step_id,
                            "Failed after 3 retries",
                        )?;
                        blocked += 1;
                    } else {
                        emit_output(&sender, &format!("Retrying {} (attempt {}/3)", step_id, count));
                        // Step remains InProgress; loop will pick it up again
                    }
                    continue;
                }

                // BUG-25 fix: verification failure blocks step completion
                if let Some(tx) = &sender { let _ = tx.send(events::ProgressEvent::PhaseLabel { label: "── verify ──".into() }); }
                let verify_ok = match verifier::run_step(&project_state, &phase_id, &track_id, &step_id, sender.as_ref()) {
                    Ok(()) => true,
                    Err(e) => {
                        emit_output(&sender, &format!("Verification failed: {}", e));
                        false
                    }
                };

                if verify_ok {
                    state::mark_step_complete(&phase_id, &track_id, &step_id)?;
                    completed += 1;
                    if let Some(tx) = &sender {
                        let _ = tx.send(events::ProgressEvent::StepCompleted {
                            track_id: track_id.clone(),
                        });
                    }
                } else {
                    state::mark_step_blocked(&phase_id, &track_id, &step_id, "Verification failed")?;
                    blocked += 1;
                    if let Some(tx) = &sender {
                        let _ = tx.send(events::ProgressEvent::StepFailed {
                            track_id: track_id.clone(),
                        });
                    }
                    continue;
                }

                let updated = state::load()?;
                if updated.is_track_complete(&phase_id, &track_id) {
                    emit_output(&sender, &format!("Track {} complete. Running track verification...", track_id));
                    if let Some(tx) = &sender { let _ = tx.send(events::ProgressEvent::PhaseLabel { label: "── verify track ──".into() }); }
                    if let Err(e) = verifier::run_track(&updated, &phase_id, &track_id, sender.as_ref()) {
                        emit_output(&sender, &format!("Track verification issue: {}", e));
                    }
                    if let Err(e) = git::merge_track(&phase_id, &track_id) {
                        emit_output(&sender, &format!("Git merge issue: {}", e));
                    }
                    if let Some(tx) = &sender {
                        let _ = tx.send(events::ProgressEvent::TrackCompleted {
                            track_id: track_id.clone(),
                        });
                    }
                }
            }
            None => {
                emit_output(&sender, "No pending steps. Phase complete or all remaining steps blocked.");
                break;
            }
        }
    }

    if let Some(tx) = &sender {
        let _ = tx.send(events::ProgressEvent::ExecutionFinished { completed, blocked });
    }

    emit_output(&sender, &format!("Completed {} steps.", completed));
    Ok(())
}

fn cmd_status(detail: bool) -> Result<()> {
    let project_state = state::load()?;
    state::print_status(&project_state, detail)
}

fn cmd_steer(message: String) -> Result<()> {
    let project_state = state::load()?;
    println!("Recording decision...\n");
    state::append_decision(&message)?;

    let phase_id = project_state.current_phase().to_string();
    println!("Re-planning remaining steps in {}...", phase_id);
    planner::replan(&project_state, &phase_id, &message, None)
}
