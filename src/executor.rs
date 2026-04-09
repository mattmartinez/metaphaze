use anyhow::Result;

use crate::{claude, git, prompt, state};

pub fn run_next(project_state: &state::ProjectState) -> Result<()> {
    match project_state.next_pending_task() {
        Some((milestone_id, slice_id, task_id)) => {
            println!("Executing {}/{}/{}...\n", milestone_id, slice_id, task_id);
            run_task(project_state, &milestone_id, &slice_id, &task_id)?;
            state::mark_task_complete(&milestone_id, &slice_id, &task_id)?;
            println!("\nTask complete.");
            Ok(())
        }
        None => {
            println!("No pending tasks.");
            Ok(())
        }
    }
}

pub fn run_task(
    project_state: &state::ProjectState,
    milestone_id: &str,
    slice_id: &str,
    task_id: &str,
) -> Result<()> {
    state::mark_task_in_progress(milestone_id, slice_id, task_id)?;

    // Ensure we're on the right branch
    git::create_slice_branch(milestone_id, slice_id)?;

    // Gather context
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let task_plan = state::read_task_plan(milestone_id, slice_id, task_id)?;
    let dep_summaries = state::collect_dependency_summaries(project_state, milestone_id, slice_id, task_id)?;
    let context = state::read_context(milestone_id)?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "context", &context);
    prompt::set(&mut vars, "task_plan", &task_plan);
    prompt::set(&mut vars, "dependency_summaries", &dep_summaries);
    prompt::set(&mut vars, "milestone_id", milestone_id);
    prompt::set(&mut vars, "slice_id", slice_id);
    prompt::set(&mut vars, "task_id", task_id);

    let rendered = prompt::render(prompt::templates::EXECUTE_TASK, &vars);

    // Find the task title for the commit message
    let task_title = project_state
        .milestones
        .iter()
        .find(|m| m.id == milestone_id)
        .and_then(|m| m.slices.iter().find(|s| s.id == slice_id))
        .and_then(|s| s.tasks.iter().find(|t| t.id == task_id))
        .map(|t| t.title.clone())
        .unwrap_or_else(|| task_id.to_string());

    let summary_path = state::task_summary_path(milestone_id, slice_id, task_id);

    let sys_prompt = format!(
        "You are an expert software engineer executing a specific task. \
         Focus ONLY on what the task plan asks for. Do not add extra features. \
         When done, write a summary of what you did to {}",
        summary_path.display(),
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(50)
        .system_prompt(&sys_prompt);

    let _result = claude::run(opts)?;

    // Commit the work
    git::commit_task(milestone_id, slice_id, task_id, &task_title)?;

    Ok(())
}
