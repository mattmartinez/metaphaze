use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::{claude, events::EventSender, git, prompt, state, verifier};

pub fn run_next(project_state: &state::ProjectState, sender: Option<&EventSender>) -> Result<()> {
    match project_state.next_pending_step() {
        Some((phase_id, track_id, step_id)) => {
            println!("Executing {}/{}/{}...\n", phase_id, track_id, step_id);
            run_step(project_state, &phase_id, &track_id, &step_id, sender)?;
            if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── verify ──".into() }); }
            if let Err(e) = verifier::run_step(project_state, &phase_id, &track_id, &step_id, sender) {
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
    sender: Option<&EventSender>,
) -> Result<()> {
    state::mark_step_in_progress(phase_id, track_id, step_id)?;

    // Ensure we're on the right branch
    git::create_track_branch(phase_id, track_id)?;

    // Prepare log file (truncate/create fresh for this step execution)
    let log_path = state::step_output_log_path(phase_id, track_id, step_id);
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Truncate on first write by creating the file
    fs::write(&log_path, "")?;

    // Plan the track before executing the first step
    if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── plan ──".into() }); }
    let plan_output = plan_track(project_state, phase_id, track_id, step_id, sender)?;
    if let Some(output) = plan_output {
        append_to_log(&log_path, "--- track planning ---\n", &output);
    }

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

    if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── execute ──".into() }); }
    let step_output = claude::run(opts, sender);
    // Write step output to log even if it failed
    match &step_output {
        Ok(output) => append_to_log(&log_path, "--- step execution ---\n", output),
        Err(e) => append_to_log(&log_path, "--- step execution (failed) ---\n", &e.to_string()),
    }
    let _result = step_output?;

    // Summarize what was done before committing
    if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── summarize ──".into() }); }
    let summary_output = summarize_step(phase_id, track_id, step_id, sender)?;
    append_to_log(&log_path, "--- summarization ---\n", &summary_output);

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
    sender: Option<&EventSender>,
) -> Result<Option<String>> {
    if !is_first_step_of_track(project_state, phase_id, track_id, step_id) {
        return Ok(None);
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

    let output = claude::run(opts, sender)?;
    Ok(Some(output))
}

fn summarize_step(phase_id: &str, track_id: &str, step_id: &str, sender: Option<&EventSender>) -> Result<String> {
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

    claude::run(opts, sender)
}

fn append_to_log(log_path: &std::path::Path, header: &str, content: &str) {
    let mut file = match OpenOptions::new().append(true).open(log_path) {
        Ok(f) => f,
        Err(_) => return,
    };
    let _ = writeln!(file, "\n{}", header);
    let _ = file.write_all(content.as_bytes());
    if !content.ends_with('\n') {
        let _ = writeln!(file);
    }
}
