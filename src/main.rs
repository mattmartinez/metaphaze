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
        /// Milestone to discuss (e.g., M001). Defaults to current.
        milestone: Option<String>,
    },

    /// Decompose current milestone into slices and tasks
    Plan {
        /// Milestone to plan (e.g., M001). Defaults to current.
        milestone: Option<String>,
    },

    /// Execute the next pending task
    Next,

    /// Autonomous loop — execute all tasks until milestone complete or blocked
    Auto {
        /// Maximum tasks to execute before stopping
        #[arg(long)]
        max_tasks: Option<usize>,
    },

    /// Show current progress
    Status {
        /// Show task-level detail
        #[arg(long)]
        detail: bool,
    },

    /// Steer the project — record a decision and re-plan remaining work
    Steer {
        /// The decision or direction change
        message: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Discuss { milestone } => cmd_discuss(milestone),
        Commands::Plan { milestone } => cmd_plan(milestone),
        Commands::Next => cmd_next(),
        Commands::Auto { max_tasks } => cmd_auto(max_tasks),
        Commands::Status { detail } => cmd_status(detail),
        Commands::Steer { message } => cmd_steer(message),
    }
}

fn cmd_init() -> Result<()> {
    println!("Initializing new Metaphaze project...\n");
    state::init_project()?;
    println!("\nProject initialized at .mz/");
    println!("  PROJECT.md  — your project definition");
    println!("  STATE.md    — current state dashboard");
    println!("\nNext: run `mz discuss` to capture decisions, then `mz plan` to decompose into tasks.");
    Ok(())
}

fn cmd_discuss(milestone: Option<String>) -> Result<()> {
    let project_state = state::load()?;
    let milestone_id = milestone.unwrap_or_else(|| project_state.current_milestone().to_string());
    println!("Starting discussion for {}...\n", milestone_id);
    discuss::run(&project_state, &milestone_id)
}

fn cmd_plan(milestone: Option<String>) -> Result<()> {
    let project_state = state::load()?;
    let milestone_id = milestone.unwrap_or_else(|| project_state.current_milestone().to_string());
    println!("Planning {}...\n", milestone_id);
    planner::run(&project_state, &milestone_id)
}

fn cmd_next() -> Result<()> {
    let project_state = state::load()?;
    executor::run_next(&project_state)
}

fn cmd_auto(max_tasks: Option<usize>) -> Result<()> {
    let limit = max_tasks.unwrap_or(usize::MAX);
    let mut completed = 0;

    println!("Starting autonomous execution...\n");

    loop {
        if completed >= limit {
            println!("\nReached task limit ({}). Stopping.", limit);
            break;
        }

        let project_state = state::load()?;

        match project_state.next_pending_task() {
            Some((milestone_id, slice_id, task_id)) => {
                println!("━━━ {}/{}/{} ━━━", milestone_id, slice_id, task_id);

                if let Err(e) = executor::run_task(&project_state, &milestone_id, &slice_id, &task_id) {
                    eprintln!("Task failed: {}", e);
                    state::mark_task_blocked(&milestone_id, &slice_id, &task_id, &format!("{e}"))?;
                    continue;
                }

                if let Err(e) = verifier::run_task(&project_state, &milestone_id, &slice_id, &task_id) {
                    eprintln!("Verification failed: {}", e);
                }

                state::mark_task_complete(&milestone_id, &slice_id, &task_id)?;
                completed += 1;

                let updated = state::load()?;
                if updated.is_slice_complete(&milestone_id, &slice_id) {
                    println!("\nSlice {} complete. Running slice verification...", slice_id);
                    if let Err(e) = verifier::run_slice(&updated, &milestone_id, &slice_id) {
                        eprintln!("Slice verification issue: {}", e);
                    }
                    if let Err(e) = git::merge_slice(&milestone_id, &slice_id) {
                        eprintln!("Git merge issue: {}", e);
                    }
                }
            }
            None => {
                println!("\nNo pending tasks. Milestone complete or all remaining tasks blocked.");
                break;
            }
        }
    }

    println!("\nCompleted {} tasks.", completed);
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

    let milestone_id = project_state.current_milestone().to_string();
    println!("Re-planning remaining tasks in {}...", milestone_id);
    planner::replan(&project_state, &milestone_id, &message)
}
