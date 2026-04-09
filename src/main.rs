mod claude;
mod discuss;
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Discuss { phase } => cmd_discuss(phase),
        Commands::Plan { phase } => cmd_plan(phase),
        Commands::Next => cmd_next(),
        Commands::Auto { max_steps } => cmd_auto(max_steps),
        Commands::Status { detail } => cmd_status(detail),
        Commands::Steer { message } => cmd_steer(message),
        Commands::Reset { step_id } => {
            let project_state = state::load()?;
            state::reset_step(&project_state.current_phase, &step_id)
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

fn cmd_plan(phase: Option<String>) -> Result<()> {
    let project_state = state::load()?;
    let phase_id = phase.unwrap_or_else(|| project_state.current_phase().to_string());
    println!("Planning {}...\n", phase_id);
    planner::run(&project_state, &phase_id)
}

fn cmd_next() -> Result<()> {
    let project_state = state::load()?;
    executor::run_next(&project_state)
}

fn cmd_auto(max_steps: Option<usize>) -> Result<()> {
    use std::collections::HashMap;

    let limit = max_steps.unwrap_or(usize::MAX);
    let mut completed = 0;
    let mut retries: HashMap<String, u32> = HashMap::new();

    println!("Starting autonomous execution...\n");

    loop {
        if completed >= limit {
            println!("\nReached step limit ({}). Stopping.", limit);
            break;
        }

        let project_state = state::load()?;

        match project_state.next_pending_step() {
            Some((phase_id, track_id, step_id)) => {
                println!("━━━ {}/{}/{} ━━━", phase_id, track_id, step_id);

                let key = format!("{}/{}", track_id, step_id);
                // Seed from persisted attempts on first encounter (supports restart recovery)
                let count = retries
                    .entry(key.clone())
                    .or_insert_with(|| project_state.step_attempts(&phase_id, &track_id, &step_id));

                if let Err(e) = executor::run_step(&project_state, &phase_id, &track_id, &step_id) {
                    eprintln!("Step failed: {}", e);
                    *count += 1;
                    state::increment_step_attempts(&phase_id, &track_id, &step_id)?;

                    if *count >= 3 {
                        eprintln!("Step {} failed after 3 attempts. Marking blocked.", step_id);
                        state::mark_step_blocked(
                            &phase_id,
                            &track_id,
                            &step_id,
                            "Failed after 3 retries",
                        )?;
                    } else {
                        eprintln!("Retrying {} (attempt {}/3)", step_id, count);
                        // Step remains InProgress; loop will pick it up again
                    }
                    continue;
                }

                if let Err(e) = verifier::run_step(&project_state, &phase_id, &track_id, &step_id) {
                    eprintln!("Verification failed: {}", e);
                }

                state::mark_step_complete(&phase_id, &track_id, &step_id)?;
                completed += 1;

                let updated = state::load()?;
                if updated.is_track_complete(&phase_id, &track_id) {
                    println!("\nTrack {} complete. Running track verification...", track_id);
                    if let Err(e) = verifier::run_track(&updated, &phase_id, &track_id) {
                        eprintln!("Track verification issue: {}", e);
                    }
                    if let Err(e) = git::merge_track(&phase_id, &track_id) {
                        eprintln!("Git merge issue: {}", e);
                    }
                }
            }
            None => {
                println!("\nNo pending steps. Phase complete or all remaining steps blocked.");
                break;
            }
        }
    }

    println!("\nCompleted {} steps.", completed);
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
    planner::replan(&project_state, &phase_id, &message)
}
