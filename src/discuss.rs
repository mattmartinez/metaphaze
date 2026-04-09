use anyhow::Result;
use std::fs;

use crate::{claude, prompt, state};

pub fn run(project_state: &state::ProjectState, milestone_id: &str) -> Result<()> {
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "milestone_id", milestone_id);

    let rendered = prompt::render(prompt::templates::DISCUSS, &vars);

    let sys_prompt = format!(
        "You are helping plan a software project called '{}'. \
         Your job is to ask probing questions to uncover ambiguity, \
         then record the answers as clear decisions. \
         Write the discussion results to {}/CONTEXT.md when done.",
        project_state.name,
        state::context_path(milestone_id).display(),
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(80)
        .system_prompt(&sys_prompt);

    println!("Launching interactive discussion with Claude...\n");
    let result = claude::run(opts)?;

    // Write context if Claude didn't already
    let ctx_path = state::context_path(milestone_id);
    if !ctx_path.exists() {
        fs::create_dir_all(ctx_path.parent().unwrap())?;
        fs::write(&ctx_path, &result)?;
    }

    println!("\nDiscussion complete. Context saved to {}", ctx_path.display());
    Ok(())
}
