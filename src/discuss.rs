use anyhow::Result;
use std::fs;

use crate::{claude, prompt, state};

pub fn run(project_state: &state::ProjectState, phase_id: &str) -> Result<()> {
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "phase_id", phase_id);

    let rendered = prompt::render(prompt::templates::DISCUSS, &vars);

    let sys_prompt = format!(
        "{}\n\n\
         IMPORTANT: This is an interactive discussion. Ask questions ONE AT A TIME \
         and wait for the user to respond before moving on. \
         When all ambiguities are resolved, write the results to \
         .mz/phases/{}/CONTEXT.md",
        rendered, phase_id,
    );

    let initial_msg = format!(
        "I'm starting a discussion phase for {} of project '{}'. \
         Ask me your first question about any ambiguity or missing detail.",
        phase_id, project_state.name,
    );

    // Ensure the phase directory exists
    let ctx_path = state::context_path(phase_id);
    fs::create_dir_all(ctx_path.parent().unwrap())?;

    // Launch interactive claude session — user talks directly
    claude::run_interactive(&sys_prompt, &initial_msg)?;

    if ctx_path.exists() {
        println!("\nDiscussion complete. Context saved to {}", ctx_path.display());
    } else {
        println!("\nSession ended. No CONTEXT.md was written.");
        println!("Run `mz discuss` again to continue, or create it manually.");
    }

    Ok(())
}
