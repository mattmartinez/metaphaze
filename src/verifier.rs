use anyhow::Result;
use std::fs;

use crate::{claude, config, events::EventSender, prompt, run_record, state};

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

    let cfg = config::current();
    let opts = claude::ClaudeOptions::new(rendered)
        .model(&cfg.models.verify_step)
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

    let cfg = config::current();
    let opts = claude::ClaudeOptions::new(rendered)
        .model(&cfg.models.verify_track)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{PhaseEntry, ProjectState, StepEntry, StepStatus, TrackEntry};
    use std::path::PathBuf;
    fn mock_claude_binary() -> PathBuf {
        let mut p = std::env::current_exe().expect("current_exe");
        p.pop(); // deps/
        p.pop(); // <profile>/
        p.push("mock_claude");
        if !p.exists() {
            panic!("mock_claude binary not found at {}", p.display());
        }
        p
    }

    struct TempMz {
        _dir: tempfile::TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let mz_path = dir.path().join(".mz");
            std::fs::create_dir_all(&mz_path).unwrap();
            crate::state::set_test_mz_dir(Some(mz_path.clone()));
            crate::run_record::set_test_mz_dir(Some(mz_path));
            TempMz { _dir: dir }
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            crate::state::set_test_mz_dir(None);
            crate::run_record::set_test_mz_dir(None);
        }
    }

    fn make_state() -> ProjectState {
        ProjectState {
            name: "test".to_string(),
            description: "".to_string(),
            current_phase: "P001".to_string(),
            phases: vec![PhaseEntry {
                id: "P001".to_string(),
                title: "Phase 1".to_string(),
                tracks: vec![TrackEntry {
                    id: "TR01".to_string(),
                    title: "Track 1".to_string(),
                    depends_on: vec![],
                    steps: vec![StepEntry {
                        id: "ST01".to_string(),
                        title: "Step 1".to_string(),
                        status: StepStatus::Complete,
                        blocked_reason: None,
                        attempts: 1,
                    }],
                }],
            }],
        }
    }

    fn write_plan_and_summary() {
        let plan_path = crate::state::step_plan_path("P001", "TR01", "ST01");
        std::fs::create_dir_all(plan_path.parent().unwrap()).unwrap();
        std::fs::write(&plan_path, "## Plan\nDo the thing").unwrap();
        let summary_path = crate::state::step_summary_path("P001", "TR01", "ST01");
        std::fs::write(&summary_path, "## What was done\nThe thing was done.").unwrap();
    }

    #[test]
    fn test_run_step_passes_when_mock_returns_pass_verdict() {
        let _env = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _tmp = TempMz::new();
        let state = make_state();
        write_plan_and_summary();

        std::env::set_var("MZ_CLAUDE_BINARY", mock_claude_binary());
        std::env::set_var("MOCK_CLAUDE_RESULT", "VERDICT: pass\nAll truths satisfied.");
        std::env::set_var("MOCK_CLAUDE_TEXT", "verifying");
        // Make sure no leftover failure mode env carries over
        std::env::remove_var("MOCK_CLAUDE_MODE");

        let result = run_step(&state, "P001", "TR01", "ST01", None);

        std::env::remove_var("MZ_CLAUDE_BINARY");
        std::env::remove_var("MOCK_CLAUDE_RESULT");
        std::env::remove_var("MOCK_CLAUDE_TEXT");

        assert!(result.is_ok(), "verifier should pass on PASS verdict: {:?}", result.err());
        // VERIFY.md should have been written
        let verify_path = crate::state::track_dir("P001", "TR01")
            .join("steps")
            .join("ST01-VERIFY.md");
        assert!(verify_path.exists(), "VERIFY.md should exist at {}", verify_path.display());
    }

    #[test]
    fn test_run_step_fails_when_mock_returns_fail_verdict() {
        let _env = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _tmp = TempMz::new();
        let state = make_state();
        write_plan_and_summary();

        std::env::set_var("MZ_CLAUDE_BINARY", mock_claude_binary());
        std::env::set_var("MOCK_CLAUDE_RESULT", "VERDICT: fail\nMissing artifact.");
        std::env::set_var("MOCK_CLAUDE_TEXT", "verifying");
        std::env::remove_var("MOCK_CLAUDE_MODE");

        let result = run_step(&state, "P001", "TR01", "ST01", None);

        std::env::remove_var("MZ_CLAUDE_BINARY");
        std::env::remove_var("MOCK_CLAUDE_RESULT");
        std::env::remove_var("MOCK_CLAUDE_TEXT");

        assert!(result.is_err(), "verifier should bail on FAIL verdict");
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Verification failed"), "got: {}", msg);
    }

    #[test]
    fn test_run_step_fails_when_summary_missing() {
        let _env = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _tmp = TempMz::new();
        let state = make_state();
        // Note: write only the plan, NOT the summary
        let plan_path = crate::state::step_plan_path("P001", "TR01", "ST01");
        std::fs::create_dir_all(plan_path.parent().unwrap()).unwrap();
        std::fs::write(&plan_path, "## Plan").unwrap();

        // Even with the env override, run_step should bail before invoking claude.
        let result = run_step(&state, "P001", "TR01", "ST01", None);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("No summary"), "got: {}", msg);
    }
}
