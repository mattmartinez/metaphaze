use anyhow::Result;
use std::fs;

use crate::{claude, events::EventSender, prompt, run_record, state};

pub fn run_step(
    _project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
    sender: Option<&EventSender>,
) -> Result<()> {
    let step_plan = state::read_step_plan(phase_id, track_id, step_id)?;
    let step_summary = state::read_step_summary(phase_id, track_id, step_id)?;

    if step_summary.is_empty() {
        anyhow::bail!("No summary found for {}/{}/{} — step may not have completed", phase_id, track_id, step_id);
    }

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "step_plan", &step_plan);
    prompt::set(&mut vars, "step_summary", &step_summary);
    prompt::set(&mut vars, "phase_id", phase_id);
    prompt::set(&mut vars, "track_id", track_id);
    prompt::set(&mut vars, "step_id", step_id);

    let rendered = prompt::render(prompt::templates::VERIFY_STEP, &vars);

    let opts = claude::ClaudeOptions::new(rendered)
        .model("sonnet")
        .max_turns(30);

    println!("  Verifying {}/{}...", track_id, step_id);
    let verify_run = claude::run(opts, sender);
    match &verify_run {
        Ok(r) => {
            let finished_at = chrono::Utc::now();
            let started_at = finished_at - chrono::Duration::milliseconds(r.wall_clock_ms as i64);
            run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: step_id.to_string(),
                stage: "verify_step".to_string(),
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
                stage: "verify_step".to_string(),
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
    let result = verify_run?.output;

    // Write verification result
    let verify_path = state::track_dir(phase_id, track_id)
        .join("steps")
        .join(format!("{}-VERIFY.md", step_id));
    fs::write(&verify_path, &result)?;

    // Check for structured VERDICT line
    let lower = result.to_lowercase();
    let has_verdict_fail = lower.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.contains("verdict") && trimmed.contains("fail") && !trimmed.contains("pass")
    });
    let has_status_fail = lower.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("**status:**") && trimmed.contains("fail")
    });
    if has_verdict_fail || has_status_fail {
        anyhow::bail!("Verification failed for {}", step_id);
    }

    println!("  {} verified.", step_id);
    Ok(())
}

pub fn run_track(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    sender: Option<&EventSender>,
) -> Result<()> {
    // Gather all step plans and summaries for the track
    let mut all_plans = String::new();
    let mut all_summaries = String::new();

    for ph in &project_state.phases {
        if ph.id != phase_id { continue; }
        for track in &ph.tracks {
            if track.id != track_id { continue; }
            for step in &track.steps {
                let plan = state::read_step_plan(phase_id, track_id, &step.id)?;
                let summary = state::read_step_summary(phase_id, track_id, &step.id)?;
                all_plans.push_str(&format!("\n### {} — {}\n\n{}\n", step.id, step.title, plan));
                all_summaries.push_str(&format!("\n### {} — {}\n\n{}\n", step.id, step.title, summary));
            }
        }
    }

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "all_plans", &all_plans);
    prompt::set(&mut vars, "all_summaries", &all_summaries);
    prompt::set(&mut vars, "phase_id", phase_id);
    prompt::set(&mut vars, "track_id", track_id);

    let rendered = prompt::render(prompt::templates::VERIFY_TRACK, &vars);

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(40);

    println!("Running end-to-end track verification...");
    let track_verify_run = claude::run(opts, sender);
    match &track_verify_run {
        Ok(r) => {
            let finished_at = chrono::Utc::now();
            let started_at = finished_at - chrono::Duration::milliseconds(r.wall_clock_ms as i64);
            run_record::append(&run_record::RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: phase_id.to_string(),
                track_id: track_id.to_string(),
                step_id: String::new(),
                stage: "verify_track".to_string(),
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
                step_id: String::new(),
                stage: "verify_track".to_string(),
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
    let result = track_verify_run?.output;

    // Write track verification
    let verify_path = state::track_dir(phase_id, track_id).join("VERIFICATION.md");
    fs::write(&verify_path, &result)?;

    println!("Track verification saved to {}", verify_path.display());
    Ok(())
}
