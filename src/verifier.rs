use anyhow::Result;
use std::fs;

use crate::{claude, prompt, state};

pub fn run_step(
    _project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
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
    let result = claude::run(opts)?;

    // Write verification result
    let verify_path = state::track_dir(phase_id, track_id)
        .join("steps")
        .join(format!("{}-VERIFY.md", step_id));
    fs::write(&verify_path, &result)?;

    // Check for PASS/FAIL in output
    if result.to_lowercase().contains("fail") && !result.to_lowercase().contains("pass") {
        anyhow::bail!("Verification failed for {}", step_id);
    }

    println!("  {} verified.", step_id);
    Ok(())
}

pub fn run_track(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
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
    let result = claude::run(opts)?;

    // Write track verification
    let verify_path = state::track_dir(phase_id, track_id).join("VERIFICATION.md");
    fs::write(&verify_path, &result)?;

    println!("Track verification saved to {}", verify_path.display());
    Ok(())
}
