use anyhow::Result;
use regex::Regex;
use std::fs;

use crate::{claude, prompt, state};

pub fn run(project_state: &state::ProjectState, milestone_id: &str) -> Result<()> {
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let context = state::read_context(milestone_id)?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "context", &context);
    prompt::set(&mut vars, "milestone_id", milestone_id);

    let rendered = prompt::render(prompt::templates::PLAN_MILESTONE, &vars);

    let sys_prompt = format!(
        "You are a senior software architect planning milestone {} for '{}'. \
         Decompose the work into slices (demoable vertical features) and tasks \
         (single-context-window units of work). Output the plan in the exact format specified.",
        milestone_id, project_state.name,
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(60)
        .system_prompt(&sys_prompt);

    println!("Generating milestone plan...\n");
    let result = claude::run(opts)?;

    // Write ROADMAP.md
    let ms_dir = state::milestone_dir(milestone_id);
    fs::create_dir_all(&ms_dir)?;
    fs::write(state::roadmap_path(milestone_id), &result)?;

    // Parse the plan output and create task files + update state
    parse_and_create_tasks(project_state, milestone_id, &result)?;

    println!("Plan written to {}/", ms_dir.display());
    println!("\nReview the plan, then run `mz auto` to start execution.");
    Ok(())
}

pub fn replan(project_state: &state::ProjectState, milestone_id: &str, decision: &str) -> Result<()> {
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let context = state::read_context(milestone_id)?;

    // Gather current state for context
    let current_state = serde_yaml::to_string(project_state)?;

    let prompt_text = format!(
        "# Re-planning Required\n\n\
         A new decision has been made:\n\n> {}\n\n\
         ## Current Project\n\n{}\n\n\
         ## Current State\n\n```yaml\n{}\n```\n\n\
         ## Decisions\n\n{}\n\n\
         ## Context\n\n{}\n\n\
         Re-plan the REMAINING (pending) tasks for milestone {}. \
         Keep completed tasks as-is. Output updated task plans for any \
         tasks that need to change. Use the same format as the original plan.",
        decision, project_md, current_state, decisions, context, milestone_id,
    );

    let opts = claude::ClaudeOptions::new(prompt_text)
        .model("opus")
        .max_turns(40);

    println!("Re-planning...\n");
    let result = claude::run(opts)?;
    println!("{}", result);
    Ok(())
}

fn parse_and_create_tasks(
    project_state: &state::ProjectState,
    milestone_id: &str,
    plan_output: &str,
) -> Result<()> {
    // Parse slices and tasks from the plan output
    // Expected format:
    //   ## S01 — Slice Title
    //   ### T01 — Task Title
    //   (task plan content...)

    let slice_re = Regex::new(r"(?m)^## (S\d+)\s*[—-]\s*(.+)$")?;
    let task_re = Regex::new(r"(?m)^### (T\d+)\s*[—-]\s*(.+)$")?;

    let mut milestones = project_state.milestones.clone();
    let mut milestone_entry = state::MilestoneEntry {
        id: milestone_id.to_string(),
        title: format!("Milestone {}", milestone_id),
        slices: vec![],
    };

    let slice_matches: Vec<_> = slice_re.find_iter(plan_output).collect();

    for (i, slice_match) in slice_matches.iter().enumerate() {
        let caps = slice_re.captures(slice_match.as_str()).unwrap();
        let slice_id = caps[1].to_string();
        let slice_title = caps[2].trim().to_string();

        let slice_start = slice_match.start();
        let slice_end = if i + 1 < slice_matches.len() {
            slice_matches[i + 1].start()
        } else {
            plan_output.len()
        };
        let slice_content = &plan_output[slice_start..slice_end];

        // Create slice directory
        let slice_dir = state::slice_dir(milestone_id, &slice_id);
        let tasks_dir = slice_dir.join("tasks");
        fs::create_dir_all(&tasks_dir)?;

        let mut slice_entry = state::SliceEntry {
            id: slice_id.clone(),
            title: slice_title.clone(),
            tasks: vec![],
        };

        // Parse tasks within this slice
        let task_matches: Vec<_> = task_re.find_iter(slice_content).collect();

        for (j, task_match) in task_matches.iter().enumerate() {
            let tcaps = task_re.captures(task_match.as_str()).unwrap();
            let task_id = tcaps[1].to_string();
            let task_title = tcaps[2].trim().to_string();

            let task_start = task_match.start();
            let task_end = if j + 1 < task_matches.len() {
                task_matches[j + 1].start()
            } else {
                slice_content.len()
            };
            let task_content = &slice_content[task_start..task_end];

            // Write PLAN.md for this task
            let plan_path = state::task_plan_path(milestone_id, &slice_id, &task_id);
            fs::write(&plan_path, task_content.trim())?;

            slice_entry.tasks.push(state::TaskEntry {
                id: task_id,
                title: task_title,
                status: state::TaskStatus::Pending,
                blocked_reason: None,
            });
        }

        // Write slice PLAN.md
        fs::write(slice_dir.join("PLAN.md"), slice_content.trim())?;

        milestone_entry.slices.push(slice_entry);
    }

    // Update state with the new milestone
    milestones.retain(|m| m.id != milestone_id);
    milestones.push(milestone_entry);
    milestones.sort_by(|a, b| a.id.cmp(&b.id));

    let mut new_state = project_state.clone();
    new_state.milestones = milestones;
    state::save(&new_state)?;

    Ok(())
}
