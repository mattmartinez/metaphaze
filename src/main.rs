mod budget;
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
mod run_record;
mod diagnostics;

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
    Next {
        /// Override budget limit for this run only (does not persist)
        #[arg(long)]
        max_budget_usd: Option<f64>,
    },

    /// Autonomous loop — execute all steps until phase complete or blocked
    Auto {
        /// Maximum steps to execute before stopping
        #[arg(long)]
        max_steps: Option<usize>,
        /// Override budget limit for this run only (does not persist)
        #[arg(long)]
        max_budget_usd: Option<f64>,
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

    /// Manage project spend budget
    Budget {
        #[command(subcommand)]
        action: Option<BudgetAction>,
    },

    /// Show execution history
    Log {
        /// Filter by phase (e.g. P008)
        #[arg(long)]
        phase: Option<String>,
        /// Filter by track (e.g. TR01)
        #[arg(long)]
        track: Option<String>,
        /// Show only failed runs
        #[arg(long)]
        failed: bool,
        /// Show last N runs (default: 20)
        #[arg(long, default_value = "20")]
        last: usize,
        /// Show detailed output for each run
        #[arg(long)]
        detail: bool,
        /// Show per-phase/track summary instead of individual runs
        #[arg(long)]
        summary: bool,
    },
}

#[derive(Subcommand)]
enum BudgetAction {
    /// Set the maximum USD spend limit
    Set {
        /// Budget limit in USD (e.g. 10.00)
        amount: f64,
    },
    /// Remove the budget limit
    Clear,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let no_tui = cli.no_tui;

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Discuss { phase } => cmd_discuss(phase),
        Commands::Plan { phase } => cmd_plan(phase, no_tui),
        Commands::Next { max_budget_usd } => cmd_next(max_budget_usd, no_tui),
        Commands::Auto { max_steps, max_budget_usd } => cmd_auto(max_steps, max_budget_usd, no_tui),
        Commands::Status { detail } => cmd_status(detail),
        Commands::Steer { message } => cmd_steer(message),
        Commands::Reset { step_id, phase } => {
            let project_state = state::load()?;
            let phase_id = state::normalize_phase_id(&phase.unwrap_or_else(|| project_state.current_phase().to_string()));
            state::reset_step(&phase_id, &step_id)
        }
        Commands::Budget { action } => cmd_budget(action),
        Commands::Log { phase, track, failed, last, detail, summary } => cmd_log(phase, track, failed, last, detail, summary),
    }
}

fn cmd_budget(action: Option<BudgetAction>) -> Result<()> {
    match action {
        Some(BudgetAction::Set { amount }) => {
            let mut config = budget::load()?;
            config.max_usd = Some(amount);
            budget::save(&config)?;
            println!("Budget set to ${:.2}", amount);
        }
        Some(BudgetAction::Clear) => {
            let mut config = budget::load()?;
            config.max_usd = None;
            budget::save(&config)?;
            println!("Budget limit cleared.");
        }
        None => {
            let config = budget::load()?;
            let records = run_record::load_all()?;
            let status = budget::check(&config, &records);
            match (status.limit, status.remaining) {
                (Some(limit), Some(remaining)) => {
                    let pct_remaining = if limit > 0.0 { remaining / limit * 100.0 } else { 0.0 };
                    println!(
                        "Budget: ${:.2} / ${:.2} ({:.1}% remaining)",
                        status.spent, limit, pct_remaining
                    );
                }
                _ => {
                    println!("Budget: ${:.2} spent (no limit set)", status.spent);
                }
            }
        }
    }
    Ok(())
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
    let phase_id = state::normalize_phase_id(&phase.unwrap_or_else(|| project_state.current_phase().to_string()));
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
        let phase_id = state::normalize_phase_id(&phase.unwrap());
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

fn cmd_next(max_budget_usd: Option<f64>, no_tui: bool) -> Result<()> {
    if !no_tui && tui::is_interactive() {
        let project_state = state::load()?;
        tui::run_with_tui(project_state, move |sender, stop, paused| {
            cmd_next_inner(max_budget_usd, sender, stop, paused)
        })
    } else {
        cmd_next_inner(
            max_budget_usd,
            None,
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        )
    }
}

fn cmd_next_inner(
    max_budget_usd: Option<f64>,
    sender: Option<events::EventSender>,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    _paused: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    use std::sync::atomic::Ordering;

    // Budget check before executing
    let budget_config = budget::load()?;
    let effective_config = match max_budget_usd {
        Some(limit) => budget::BudgetConfig { max_usd: Some(limit) },
        None => budget_config,
    };
    let records = run_record::load_all()?;
    let status = budget::check(&effective_config, &records);
    if status.exhausted {
        let limit = status.limit.unwrap_or(0.0);
        emit_output(&sender, &format!(
            "Budget exhausted: ${:.4} spent of ${:.4} limit. Stopping.",
            status.spent, limit
        ));
        if let Some(tx) = &sender {
            let _ = tx.send(events::ProgressEvent::BudgetExhausted { spent: status.spent, limit });
        }
        return Ok(());
    }

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

            // Post-step: print total spend so far
            let records = run_record::load_all()?;
            let status = budget::check(&effective_config, &records);
            emit_output(&sender, &format!("Total spent so far: ${:.4}", status.spent));

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

fn cmd_auto(max_steps: Option<usize>, max_budget_usd: Option<f64>, no_tui: bool) -> Result<()> {
    if !no_tui && tui::is_interactive() {
        let project_state = state::load()?;
        tui::run_with_tui(project_state, move |sender, stop, paused| {
            cmd_auto_inner(max_steps, max_budget_usd, sender, stop, paused)
        })
    } else {
        cmd_auto_inner(
            max_steps,
            max_budget_usd,
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
    max_budget_usd: Option<f64>,
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

        // Budget check before each step
        let budget_config = budget::load()?;
        let effective_config = match max_budget_usd {
            Some(l) => budget::BudgetConfig { max_usd: Some(l) },
            None => budget_config,
        };
        let records = run_record::load_all()?;
        let bstatus = budget::check(&effective_config, &records);
        if bstatus.exhausted {
            let blimit = bstatus.limit.unwrap_or(0.0);
            emit_output(&sender, &format!(
                "Budget exhausted: ${:.4} spent of ${:.4} limit. Stopping.",
                bstatus.spent, blimit
            ));
            if let Some(tx) = &sender {
                let _ = tx.send(events::ProgressEvent::BudgetExhausted { spent: bstatus.spent, limit: blimit });
            }
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

                    if *count >= 3 {
                        let records = run_record::load_all().unwrap_or_default();
                        let diagnosis = diagnostics::diagnose_step(&records, &phase_id, &track_id, &step_id);
                        let blocked_reason = diagnosis
                            .as_ref()
                            .map(|d| diagnostics::format_blocked_reason(d))
                            .unwrap_or_else(|| "Failed after 3 retries".to_string());
                        emit_output(&sender, &format!("Step {} blocked: {}", step_id, blocked_reason));
                        state::mark_step_blocked(&phase_id, &track_id, &step_id, &blocked_reason)?;
                        blocked += 1;
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::StepBlocked {
                                track_id: track_id.clone(),
                                step_id: step_id.clone(),
                                reason: blocked_reason,
                            });
                        }
                    } else {
                        emit_output(&sender, &format!("Retrying {} (attempt {}/3)", step_id, count));
                        // Step remains InProgress; loop will pick it up again
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::StepFailed {
                                track_id: track_id.clone(),
                            });
                        }
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
                    // Post-step budget warning (80% spend threshold)
                    let records = run_record::load_all()?;
                    let status = budget::check(&effective_config, &records);
                    if let Some(remaining) = status.remaining {
                        if remaining > 0.0 && remaining < status.limit.unwrap_or(f64::MAX) * 0.2 {
                            emit_output(&sender, &format!(
                                "⚠ Budget warning: ${:.4} remaining of ${:.4}",
                                remaining, status.limit.unwrap()
                            ));
                        }
                    }
                } else {
                    *count += 1;
                    state::increment_step_attempts(&phase_id, &track_id, &step_id)?;
                    let records = run_record::load_all().unwrap_or_default();
                    let diagnosis = diagnostics::diagnose_step(&records, &phase_id, &track_id, &step_id);
                    let is_oscillating = matches!(
                        diagnosis.as_ref().map(|d| &d.issue),
                        Some(diagnostics::StepIssue::VerifyOscillation)
                    );
                    if *count >= 2 || is_oscillating {
                        let blocked_reason = diagnosis
                            .as_ref()
                            .map(|d| diagnostics::format_blocked_reason(d))
                            .unwrap_or_else(|| "Verification failed".to_string());
                        emit_output(&sender, &format!("Step {} blocked: {}", step_id, blocked_reason));
                        state::mark_step_blocked(&phase_id, &track_id, &step_id, &blocked_reason)?;
                        blocked += 1;
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::StepBlocked {
                                track_id: track_id.clone(),
                                step_id: step_id.clone(),
                                reason: blocked_reason,
                            });
                        }
                    } else {
                        emit_output(&sender, &format!("Verify failed, retrying {} (attempt {}/3)", step_id, count));
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::StepFailed {
                                track_id: track_id.clone(),
                            });
                        }
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
                let current_phase = project_state.current_phase().to_string();
                if !project_state.is_phase_complete(&current_phase) {
                    // Some steps are blocked — cannot advance.
                    let records = run_record::load_all().unwrap_or_default();
                    let diagnoses = diagnostics::diagnose_all_steps(&records, &project_state);
                    if diagnoses.is_empty() {
                        emit_output(&sender, "No pending steps. All remaining steps are blocked.");
                    } else {
                        let mut msg = format!("No pending steps. {} step{} blocked:", diagnoses.len(), if diagnoses.len() == 1 { " is" } else { "s are" });
                        for d in &diagnoses {
                            msg.push_str(&format!("\n  {}/{}: {}", d.track_id, d.step_id, diagnostics::format_blocked_reason(d)));
                        }
                        msg.push_str("\nHint: use `mz doctor` for full diagnostics or `mz reset <step>` to retry");
                        emit_output(&sender, &msg);
                    }
                    break;
                }
                // Phase is fully complete — try to advance to the next phase.
                match project_state.next_phase_id() {
                    Some(next_id) => {
                        emit_output(
                            &sender,
                            &format!("Phase {} complete. Planning {}...", current_phase, next_id),
                        );
                        planner::run(&project_state, &next_id, sender.as_ref())?;
                        state::advance_phase(&next_id)?;
                        phase_started = false;
                        if let Some(tx) = &sender {
                            let _ = tx.send(events::ProgressEvent::PhaseTransition {
                                from: current_phase.clone(),
                                to: next_id.clone(),
                            });
                        }
                        // project_state will be reloaded at the top of the next iteration
                        continue;
                    }
                    None => {
                        emit_output(&sender, "All phases complete!");
                        break;
                    }
                }
            }
        }
    }

    if let Some(tx) = &sender {
        let _ = tx.send(events::ProgressEvent::ExecutionFinished { completed, blocked });
    }

    emit_output(&sender, &format!("Completed {} steps.", completed));
    Ok(())
}

fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60_000 {
        format!("{}s", ms / 1000)
    } else {
        format!("{}m {}s", ms / 60_000, (ms % 60_000) / 1000)
    }
}

fn format_tokens(input: Option<u64>, output: Option<u64>) -> String {
    match (input, output) {
        (None, None) => "-".to_string(),
        (i, o) => {
            let fmt = |n: Option<u64>| -> String {
                match n {
                    None => "-".to_string(),
                    Some(v) if v >= 1000 => format!("{:.1}k", v as f64 / 1000.0),
                    Some(v) => format!("{}", v),
                }
            };
            format!("{}→{}", fmt(i), fmt(o))
        }
    }
}

fn cmd_log(
    phase: Option<String>,
    track: Option<String>,
    failed: bool,
    last: usize,
    detail: bool,
    summary: bool,
) -> Result<()> {
    use chrono::{DateTime, Local, Utc};
    use colored::Colorize;

    let all_records = run_record::load_all()?;
    let total = all_records.len();

    if total == 0 {
        println!("No execution history.");
        return Ok(());
    }

    // Apply filters
    let phase_filter = phase.map(|p| state::normalize_phase_id(&p));
    let track_filter = track.map(|t| t.to_uppercase());

    let mut records: Vec<_> = all_records
        .into_iter()
        .filter(|r| {
            if let Some(ref p) = phase_filter {
                if r.phase_id != *p {
                    return false;
                }
            }
            if let Some(ref t) = track_filter {
                if r.track_id != *t {
                    return false;
                }
            }
            if failed && r.outcome != "error" {
                return false;
            }
            true
        })
        .collect();

    let filtered_total = records.len();

    if records.is_empty() {
        println!("No execution history.");
        return Ok(());
    }

    if summary {
        return cmd_log_summary(&records);
    }

    // Take last N
    let skip = filtered_total.saturating_sub(last);
    records = records.into_iter().skip(skip).collect();

    if records.is_empty() {
        println!("No execution history.");
        return Ok(());
    }

    // Compute totals for footer
    let total_cost: f64 = records.iter().filter_map(|r| r.cost_usd).sum();
    let total_ms: u64 = records.iter().map(|r| r.duration_ms).sum();

    // Column widths
    let time_w = 12;
    let phase_w = 6;
    let track_w = 6;
    let step_w = 5;
    let stage_w = 8;
    let model_w = 10;
    let dur_w = 8;
    let cost_w = 8;
    let tokens_w = 12;
    let status_w = 6;

    // Header
    println!(
        "{:<time_w$} {:<phase_w$} {:<track_w$} {:<step_w$} {:<stage_w$} {:<model_w$} {:>dur_w$} {:>cost_w$} {:>tokens_w$} {:<status_w$}",
        "TIME", "PHASE", "TRACK", "STEP", "STAGE", "MODEL", "DURATION", "COST", "TOKENS", "STATUS",
        time_w = time_w,
        phase_w = phase_w,
        track_w = track_w,
        step_w = step_w,
        stage_w = stage_w,
        model_w = model_w,
        dur_w = dur_w,
        cost_w = cost_w,
        tokens_w = tokens_w,
        status_w = status_w,
    );

    let sep_len = time_w + 1 + phase_w + 1 + track_w + 1 + step_w + 1 + stage_w + 1 + model_w + 1 + dur_w + 1 + cost_w + 1 + tokens_w + 1 + status_w;
    println!("{}", "─".repeat(sep_len));

    for record in &records {
        // Parse time
        let time_str = record
            .finished_at
            .parse::<DateTime<Utc>>()
            .map(|dt| {
                let local: DateTime<Local> = dt.into();
                local.format("%b %d %H:%M").to_string()
            })
            .unwrap_or_else(|_| record.finished_at.chars().take(time_w).collect());

        let dur_str = format_duration(record.duration_ms);

        let cost_str = match record.cost_usd {
            Some(c) => format!("${:.4}", c),
            None => "-".to_string(),
        };

        let tokens_str = format_tokens(record.input_tokens, record.output_tokens);

        let status_str = if record.outcome == "error" {
            "ERR".red().to_string()
        } else {
            "ok".green().to_string()
        };

        // Truncate model to fit
        let model_display: String = record.model.chars().take(model_w).collect();

        println!(
            "{:<time_w$} {:<phase_w$} {:<track_w$} {:<step_w$} {:<stage_w$} {:<model_w$} {:>dur_w$} {:>cost_w$} {:>tokens_w$} {}",
            time_str,
            record.phase_id,
            record.track_id,
            record.step_id,
            record.stage,
            model_display,
            dur_str,
            cost_str,
            tokens_str,
            status_str,
            time_w = time_w,
            phase_w = phase_w,
            track_w = track_w,
            step_w = step_w,
            stage_w = stage_w,
            model_w = model_w,
            dur_w = dur_w,
            cost_w = cost_w,
            tokens_w = tokens_w,
        );

        if detail && record.outcome == "error" {
            if let Some(ref err) = record.error {
                for line in err.lines() {
                    println!("    {}", line);
                }
            }
        }
    }

    println!("{}", "─".repeat(sep_len));

    let shown = records.len();
    let cost_footer = if total_cost > 0.0 {
        format!("${:.4}", total_cost)
    } else {
        "-".to_string()
    };
    println!(
        "{} runs shown ({} total). Total cost: {}. Total time: {}.",
        shown,
        filtered_total,
        cost_footer,
        format_duration(total_ms),
    );

    Ok(())
}

fn cmd_log_summary(records: &[run_record::RunRecord]) -> Result<()> {
    let phases = run_record::phase_summaries(records);
    let tracks = run_record::track_summaries(records);

    // Phase summary table
    let phase_w = 8;
    let runs_w = 6;
    let ok_w = 4;
    let err_w = 5;
    let cost_w = 10;
    let time_w = 10;
    let tokens_w = 14;

    println!(
        "{:<phase_w$} {:>runs_w$} {:>ok_w$} {:>err_w$} {:>cost_w$} {:>time_w$} {:>tokens_w$}",
        "PHASE", "RUNS", "OK", "ERR", "COST", "TIME", "TOKENS",
        phase_w = phase_w, runs_w = runs_w, ok_w = ok_w,
        err_w = err_w, cost_w = cost_w, time_w = time_w, tokens_w = tokens_w,
    );
    let sep = phase_w + 1 + runs_w + 1 + ok_w + 1 + err_w + 1 + cost_w + 1 + time_w + 1 + tokens_w;
    println!("{}", "─".repeat(sep));
    for p in &phases {
        let cost_str = format!("${:.4}", p.cost_usd);
        let tokens_str = format_tokens(
            if p.input_tokens > 0 { Some(p.input_tokens) } else { None },
            if p.output_tokens > 0 { Some(p.output_tokens) } else { None },
        );
        println!(
            "{:<phase_w$} {:>runs_w$} {:>ok_w$} {:>err_w$} {:>cost_w$} {:>time_w$} {:>tokens_w$}",
            p.phase_id, p.runs, p.ok, p.err, cost_str, format_duration(p.duration_ms), tokens_str,
            phase_w = phase_w, runs_w = runs_w, ok_w = ok_w,
            err_w = err_w, cost_w = cost_w, time_w = time_w, tokens_w = tokens_w,
        );
    }
    println!();

    // Track summary table
    let pt_w = 12;
    let steps_w = 7;
    let runs_w2 = 6;

    println!(
        "{:<pt_w$} {:>steps_w$} {:>runs_w2$} {:>cost_w$} {:>time_w$} {:>tokens_w$}",
        "PHASE/TRACK", "STEPS", "RUNS", "COST", "TIME", "TOKENS",
        pt_w = pt_w, steps_w = steps_w, runs_w2 = runs_w2,
        cost_w = cost_w, time_w = time_w, tokens_w = tokens_w,
    );
    let sep2 = pt_w + 1 + steps_w + 1 + runs_w2 + 1 + cost_w + 1 + time_w + 1 + tokens_w;
    println!("{}", "─".repeat(sep2));
    for t in &tracks {
        let label = format!("{}/{}", t.phase_id, t.track_id);
        let cost_str = format!("${:.4}", t.cost_usd);
        let tokens_str = format_tokens(
            if t.input_tokens > 0 { Some(t.input_tokens) } else { None },
            if t.output_tokens > 0 { Some(t.output_tokens) } else { None },
        );
        println!(
            "{:<pt_w$} {:>steps_w$} {:>runs_w2$} {:>cost_w$} {:>time_w$} {:>tokens_w$}",
            label, t.steps, t.runs, cost_str, format_duration(t.duration_ms), tokens_str,
            pt_w = pt_w, steps_w = steps_w, runs_w2 = runs_w2,
            cost_w = cost_w, time_w = time_w, tokens_w = tokens_w,
        );
    }

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

    let phase_id = state::normalize_phase_id(project_state.current_phase());
    println!("Re-planning remaining steps in {}...", phase_id);
    planner::replan(&project_state, &phase_id, &message, None)
}
