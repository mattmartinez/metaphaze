use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::{claude, events, events::EventSender, git, prompt, run_record, state, verifier};

/// Returns (completed: bool, blocked: bool) for TUI signaling
pub fn run_next(project_state: &state::ProjectState, sender: Option<&EventSender>) -> Result<(bool, bool)> {
    match project_state.next_pending_step() {
        Some((phase_id, track_id, step_id)) => {
            events::emit(sender, &format!("Executing {}/{}/{}...", phase_id, track_id, step_id));
            run_step(project_state, &phase_id, &track_id, &step_id, sender, None)?;
            if let Some(tx) = sender { let _ = tx.send(events::ProgressEvent::PhaseLabel { label: "── verify ──".into(), track_id: None }); }
            let verify_passed = match verifier::run_step(project_state, &phase_id, &track_id, &step_id, sender) {
                Ok(()) => true,
                Err(e) => {
                    events::emit(sender, &format!("Verification failed: {}", e));
                    false
                }
            };
            if verify_passed {
                state::mark_step_complete(&phase_id, &track_id, &step_id)?;
                events::emit(sender, "Step complete.");
                Ok((true, false))
            } else {
                events::emit(sender, "Step verification failed — marking blocked.");
                state::mark_step_blocked(&phase_id, &track_id, &step_id, "Verification failed")?;
                Ok((false, true))
            }
        }
        None => {
            events::emit(sender, "No pending steps.");
            Ok((false, false))
        }
    }
}

pub fn run_step(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
    sender: Option<&EventSender>,
    worktree_dir: Option<&Path>,
) -> Result<()> {
    state::mark_step_in_progress(phase_id, track_id, step_id)?;

    // When running in a worktree, the branch is already checked out there.
    // For serial execution (worktree_dir == None), switch to the track branch as before.
    if worktree_dir.is_none() {
        git::create_track_branch(phase_id, track_id)?;
    }

    // Prepare log file (truncate/create fresh for this step execution)
    let log_path = state::step_output_log_path(phase_id, track_id, step_id);
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Truncate on first write by creating the file
    fs::write(&log_path, "")?;

    // Plan the track before executing the first step
    if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── plan ──".into(), track_id: None }); }
    let plan_output = plan_track(project_state, phase_id, track_id, step_id, sender, worktree_dir)?;
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

    // NOTE: State file reads (read_step_plan, read_context, etc.) use relative `.mz/` paths and
    // must be called from the main repo root. This works because the orchestrator's CWD never
    // changes — only the Claude subprocess CWD is set to the worktree via opts.cwd().
    let mut opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(50)
        .system_prompt(&sys_prompt);
    if let Some(dir) = worktree_dir {
        opts = opts.cwd(dir.to_path_buf());
    }

    if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── execute ──".into(), track_id: None }); }
    let step_output = claude::run(opts, sender);
    // Write step output to log even if it failed, and record the run
    match &step_output {
        Ok(run_result) => {
            append_to_log(&log_path, "--- step execution ---\n", &run_result.output);
            let finished_at = chrono::Utc::now();
            let started_at = finished_at - chrono::Duration::milliseconds(run_result.wall_clock_ms as i64);
            run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "execute".to_string(),
                model: run_result.model.clone(),
                started_at: started_at.to_rfc3339(),
                finished_at: finished_at.to_rfc3339(),
                duration_ms: run_result.wall_clock_ms,
                cost_usd: run_result.cost_usd,
                num_turns: run_result.num_turns,
                outcome: "success".to_string(),
                error: None,
                input_tokens: run_result.input_tokens,
                output_tokens: run_result.output_tokens,
            })?;
        }
        Err(e) => {
            append_to_log(&log_path, "--- step execution (failed) ---\n", &e.to_string());
            let now = chrono::Utc::now().to_rfc3339();
            let _ = run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "execute".to_string(),
                model: String::new(),
                started_at: now.clone(),
                finished_at: now,
                duration_ms: 0,
                cost_usd: None,
                num_turns: None,
                outcome: "error".to_string(),
                error: Some(e.to_string()),
                input_tokens: None,
                output_tokens: None,
            });
        }
    }
    step_output?;

    // Summarize what was done before committing
    if let Some(tx) = sender { let _ = tx.send(crate::events::ProgressEvent::PhaseLabel { label: "── summarize ──".into(), track_id: None }); }
    let summary_output = summarize_step(phase_id, track_id, step_id, sender, worktree_dir)?;
    append_to_log(&log_path, "--- summarization ---\n", &summary_output);

    // Commit the work; pass worktree_dir so git runs inside the worktree when set.
    git::commit_step(phase_id, track_id, step_id, &step_title, worktree_dir)?;

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
    worktree_dir: Option<&Path>,
) -> Result<Option<String>> {
    if !is_first_step_of_track(project_state, phase_id, track_id, step_id) {
        return Ok(None);
    }

    events::emit(sender, &format!("Planning track {}/{}...", phase_id, track_id));

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

    let mut opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(30)
        .system_prompt(&sys_prompt);
    if let Some(dir) = worktree_dir {
        opts = opts.cwd(dir.to_path_buf());
    }

    let plan_result = claude::run(opts, sender);
    match &plan_result {
        Ok(r) => {
            let finished_at = chrono::Utc::now();
            let started_at = finished_at - chrono::Duration::milliseconds(r.wall_clock_ms as i64);
            run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "plan_track".to_string(),
                model: r.model.clone(),
                started_at: started_at.to_rfc3339(),
                finished_at: finished_at.to_rfc3339(),
                duration_ms: r.wall_clock_ms,
                cost_usd: r.cost_usd,
                num_turns: r.num_turns,
                outcome: "success".to_string(),
                error: None,
                input_tokens: r.input_tokens,
                output_tokens: r.output_tokens,
            })?;
        }
        Err(e) => {
            let now = chrono::Utc::now().to_rfc3339();
            let _ = run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "plan_track".to_string(),
                model: String::new(),
                started_at: now.clone(),
                finished_at: now,
                duration_ms: 0,
                cost_usd: None,
                num_turns: None,
                outcome: "error".to_string(),
                error: Some(e.to_string()),
                input_tokens: None,
                output_tokens: None,
            });
        }
    }
    Ok(Some(plan_result?.output))
}

fn summarize_step(phase_id: &str, track_id: &str, step_id: &str, sender: Option<&EventSender>, worktree_dir: Option<&Path>) -> Result<String> {
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

    let mut opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(20)
        .system_prompt(&sys_prompt);
    if let Some(dir) = worktree_dir {
        opts = opts.cwd(dir.to_path_buf());
    }

    let summarize_result = claude::run(opts, sender);
    match &summarize_result {
        Ok(r) => {
            let finished_at = chrono::Utc::now();
            let started_at = finished_at - chrono::Duration::milliseconds(r.wall_clock_ms as i64);
            run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "summarize".to_string(),
                model: r.model.clone(),
                started_at: started_at.to_rfc3339(),
                finished_at: finished_at.to_rfc3339(),
                duration_ms: r.wall_clock_ms,
                cost_usd: r.cost_usd,
                num_turns: r.num_turns,
                outcome: "success".to_string(),
                error: None,
                input_tokens: r.input_tokens,
                output_tokens: r.output_tokens,
            })?;
        }
        Err(e) => {
            let now = chrono::Utc::now().to_rfc3339();
            let _ = run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "summarize".to_string(),
                model: String::new(),
                started_at: now.clone(),
                finished_at: now,
                duration_ms: 0,
                cost_usd: None,
                num_turns: None,
                outcome: "error".to_string(),
                error: Some(e.to_string()),
                input_tokens: None,
                output_tokens: None,
            });
        }
    }
    Ok(summarize_result?.output)
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
