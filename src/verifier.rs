use anyhow::Result;
use std::fs;

use crate::{claude, prompt, state};

pub fn run_task(
    _project_state: &state::ProjectState,
    milestone_id: &str,
    slice_id: &str,
    task_id: &str,
) -> Result<()> {
    let task_plan = state::read_task_plan(milestone_id, slice_id, task_id)?;
    let task_summary = state::read_task_summary(milestone_id, slice_id, task_id)?;

    if task_summary.is_empty() {
        anyhow::bail!("No summary found for {}/{}/{} — task may not have completed", milestone_id, slice_id, task_id);
    }

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "task_plan", &task_plan);
    prompt::set(&mut vars, "task_summary", &task_summary);
    prompt::set(&mut vars, "milestone_id", milestone_id);
    prompt::set(&mut vars, "slice_id", slice_id);
    prompt::set(&mut vars, "task_id", task_id);

    let rendered = prompt::render(prompt::templates::VERIFY_TASK, &vars);

    let opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(30);

    println!("  Verifying {}/{}...", slice_id, task_id);
    let result = claude::run(opts)?;

    // Write verification result
    let verify_path = state::slice_dir(milestone_id, slice_id)
        .join("tasks")
        .join(format!("{}-VERIFY.md", task_id));
    fs::write(&verify_path, &result)?;

    // Check for PASS/FAIL in output
    if result.to_lowercase().contains("fail") && !result.to_lowercase().contains("pass") {
        anyhow::bail!("Verification failed for {}", task_id);
    }

    println!("  {} verified.", task_id);
    Ok(())
}

pub fn run_slice(
    project_state: &state::ProjectState,
    milestone_id: &str,
    slice_id: &str,
) -> Result<()> {
    // Gather all task plans and summaries for the slice
    let mut all_plans = String::new();
    let mut all_summaries = String::new();

    for ms in &project_state.milestones {
        if ms.id != milestone_id { continue; }
        for slice in &ms.slices {
            if slice.id != slice_id { continue; }
            for task in &slice.tasks {
                let plan = state::read_task_plan(milestone_id, slice_id, &task.id)?;
                let summary = state::read_task_summary(milestone_id, slice_id, &task.id)?;
                all_plans.push_str(&format!("\n### {} — {}\n\n{}\n", task.id, task.title, plan));
                all_summaries.push_str(&format!("\n### {} — {}\n\n{}\n", task.id, task.title, summary));
            }
        }
    }

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "all_plans", &all_plans);
    prompt::set(&mut vars, "all_summaries", &all_summaries);
    prompt::set(&mut vars, "milestone_id", milestone_id);
    prompt::set(&mut vars, "slice_id", slice_id);

    let rendered = prompt::render(prompt::templates::VERIFY_SLICE, &vars);

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(40);

    println!("Running end-to-end slice verification...");
    let result = claude::run(opts)?;

    // Write slice verification
    let verify_path = state::slice_dir(milestone_id, slice_id).join("VERIFICATION.md");
    fs::write(&verify_path, &result)?;

    println!("Slice verification saved to {}", verify_path.display());
    Ok(())
}
