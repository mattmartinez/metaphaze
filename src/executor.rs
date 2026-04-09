use anyhow::Result;

use crate::{claude, git, prompt, state, verifier};

pub fn run_next(project_state: &state::ProjectState) -> Result<()> {
    match project_state.next_pending_step() {
        Some((phase_id, track_id, step_id)) => {
            println!("Executing {}/{}/{}...\n", phase_id, track_id, step_id);
            run_step(project_state, &phase_id, &track_id, &step_id)?;
            if let Err(e) = verifier::run_step(project_state, &phase_id, &track_id, &step_id) {
                eprintln!("Verification failed: {}", e);
            }
            state::mark_step_complete(&phase_id, &track_id, &step_id)?;
            println!("\nStep complete.");
            Ok(())
        }
        None => {
            println!("No pending steps.");
            Ok(())
        }
    }
}

pub fn run_step(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
) -> Result<()> {
    state::mark_step_in_progress(phase_id, track_id, step_id)?;

    // Ensure we're on the right branch
    git::create_track_branch(phase_id, track_id)?;

    // Plan the track before executing the first step
    plan_track(project_state, phase_id, track_id, step_id)?;

    // Gather context
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let step_plan = state::read_step_plan(phase_id, track_id, step_id)?;
    let dep_summaries = state::collect_dependency_summaries(project_state, phase_id, track_id, step_id)?;
    let context = state::read_context(phase_id)?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "context", &context);
    prompt::set(&mut vars, "step_plan", &step_plan);
    prompt::set(&mut vars, "dependency_summaries", &dep_summaries);
    prompt::set(&mut vars, "phase_id", phase_id);
    prompt::set(&mut vars, "track_id", track_id);
    prompt::set(&mut vars, "step_id", step_id);

    let rendered = prompt::render(prompt::templates::EXECUTE_STEP, &vars);

    // Find the step title for the commit message
    let step_title = project_state
        .phases
        .iter()
        .find(|p| p.id == phase_id)
        .and_then(|p| p.tracks.iter().find(|t| t.id == track_id))
        .and_then(|t| t.steps.iter().find(|s| s.id == step_id))
        .map(|s| s.title.clone())
        .unwrap_or_else(|| step_id.to_string());

    let summary_path = state::step_summary_path(phase_id, track_id, step_id);

    let sys_prompt = format!(
        "You are an expert software engineer executing a specific step. \
         Focus ONLY on what the step plan asks for. Do not add extra features. \
         When done, write a summary of what you did to {}",
        summary_path.display(),
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(50)
        .system_prompt(&sys_prompt);

    let _result = claude::run(opts)?;

    // Summarize what was done before committing
    summarize_step(phase_id, track_id, step_id)?;

    // Commit the work
    git::commit_step(phase_id, track_id, step_id, &step_title)?;

    Ok(())
}

fn is_first_step_of_track(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
) -> bool {
    for ph in &project_state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &ph.tracks {
            if track.id != track_id {
                continue;
            }
            for step in &track.steps {
                if step.id == step_id {
                    return true;
                }
                if step.status != state::StepStatus::Pending {
                    return false;
                }
            }
        }
    }
    true
}

fn plan_track(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
) -> Result<()> {
    if !is_first_step_of_track(project_state, phase_id, track_id, step_id) {
        return Ok(());
    }

    println!("Planning track {}/{}...", phase_id, track_id);

    let project_md = state::read_project_md()?;
    let context = state::read_context(phase_id)?;
    let track_plan = state::read_track_plan(phase_id, track_id)?;
    let track_plan_path = state::track_dir(phase_id, track_id).join("PLAN.md");

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "track_plan", &track_plan);
    prompt::set(&mut vars, "context", &context);
    prompt::set(&mut vars, "phase_id", phase_id);
    prompt::set(&mut vars, "track_id", track_id);

    let rendered = prompt::render(prompt::templates::PLAN_TRACK, &vars);

    let sys_prompt = format!(
        "You are refining the implementation plan for track {}/{}. \
         Enrich the existing plan with concrete implementation details and step-level guidance. \
         Write the enriched plan to {}",
        phase_id, track_id, track_plan_path.display()
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(30)
        .system_prompt(&sys_prompt);

    claude::run(opts)?;
    Ok(())
}

fn summarize_step(phase_id: &str, track_id: &str, step_id: &str) -> Result<()> {
    let step_plan = state::read_step_plan(phase_id, track_id, step_id)?;
    let summary_path = state::step_summary_path(phase_id, track_id, step_id);

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "step_plan", &step_plan);
    prompt::set(&mut vars, "phase_id", phase_id);
    prompt::set(&mut vars, "track_id", track_id);
    prompt::set(&mut vars, "step_id", step_id);

    let rendered = prompt::render(prompt::templates::SUMMARIZE, &vars);

    let sys_prompt = format!(
        "You are summarizing the work done in a step. \
         Write the summary to {}",
        summary_path.display()
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(20)
        .system_prompt(&sys_prompt);

    claude::run(opts)?;
    Ok(())
}
