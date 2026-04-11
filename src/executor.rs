use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::{claude, config, events, events::EventSender, git, prompt, run_record, state, step_context};

pub fn run_step(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
    sender: Option<&EventSender>,
    worktree_dir: Option<&Path>,
) -> Result<()> {
    state::mark_step_in_progress(phase_id, track_id, step_id)?;

    // run_step requires a worktree. The legacy serial path that mutated the
    // main repo's HEAD via `git checkout -b mz/...` was removed because it
    // corrupted the main repo and broke subsequent parallel runs. All callers
    // must now create a worktree via `git::ensure_worktree` and pass it here.
    if worktree_dir.is_none() {
        anyhow::bail!(
            "run_step requires a worktree_dir; callers must use git::ensure_worktree"
        );
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

    // Gather context. The step plan may carry an optional `context:` YAML
    // frontmatter block (`step_context::parse`) telling us which prior
    // summaries and extra files to include — we honor it when present and
    // fall back to "include everything" defaults otherwise.
    //
    // The render order below is intentional and load-bearing for prompt
    // caching: project_md → decisions → context → dep_summaries →
    // extra_files → step_plan. The first three are byte-stable across
    // every step in a phase, which lets Claude Code's CLI cache the prefix
    // (system prompt + cacheable user-prompt header) once per phase
    // instead of re-tokenizing it for each step. Don't reorder these
    // without thinking about cache hit rates.
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let raw_step_plan = state::read_step_plan(phase_id, track_id, step_id)?;
    let plan_spec = step_context::parse(&raw_step_plan);
    let step_plan = step_context::strip_frontmatter(&raw_step_plan).to_string();
    let dep_summaries = state::collect_dependency_summaries_with_spec(
        project_state,
        phase_id,
        track_id,
        step_id,
        &plan_spec,
    )?;
    let extra_files = state::read_extra_files(&plan_spec);
    let context = state::read_context(phase_id)?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "context", &context);
    prompt::set(&mut vars, "step_plan", &step_plan);
    prompt::set(&mut vars, "dependency_summaries", &dep_summaries);
    prompt::set(&mut vars, "extra_files", &extra_files);
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
    let cfg = config::current();
    let mut opts = claude::ClaudeOptions::new(rendered)
        .model(&cfg.models.execute)
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

    let cfg = config::current();
    let mut opts = claude::ClaudeOptions::new(rendered)
        .model(&cfg.models.plan_track)
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

    let cfg = config::current();
    let mut opts = claude::ClaudeOptions::new(rendered)
        .model(&cfg.models.summarize)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{PhaseEntry, ProjectState, StepEntry, StepStatus, TrackEntry};
    use std::process::Command as StdCommand;

    fn mock_claude_binary() -> std::path::PathBuf {
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
        dir: tempfile::TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let mz_path = dir.path().join(".mz");
            std::fs::create_dir_all(&mz_path).unwrap();
            crate::state::set_test_mz_dir(Some(mz_path.clone()));
            crate::run_record::set_test_mz_dir(Some(mz_path));
            TempMz { dir }
        }

        fn path(&self) -> &std::path::Path {
            self.dir.path()
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            crate::state::set_test_mz_dir(None);
            crate::run_record::set_test_mz_dir(None);
        }
    }

    fn git_init(dir: &std::path::Path) {
        let run = |args: &[&str]| {
            let out = StdCommand::new("git")
                .args(args)
                .current_dir(dir)
                .output()
                .expect("spawn git");
            assert!(
                out.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&out.stderr)
            );
        };
        run(&["init", "-q", "-b", "main"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "Test"]);
        run(&["config", "commit.gpgsign", "false"]);
        // Initial commit so HEAD exists
        std::fs::write(dir.join("seed.txt"), "seed\n").unwrap();
        run(&["add", "seed.txt"]);
        run(&["commit", "-q", "-m", "seed"]);
    }

    fn make_two_step_state() -> ProjectState {
        // ST01 already Complete, ST02 Pending — so is_first_step_of_track returns
        // false for ST02 and run_step skips the plan_track call.
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
                    steps: vec![
                        StepEntry {
                            id: "ST01".to_string(),
                            title: "Step 1".to_string(),
                            status: StepStatus::Complete,
                            blocked_reason: None,
                            attempts: 1,
                        },
                        StepEntry {
                            id: "ST02".to_string(),
                            title: "Step 2".to_string(),
                            status: StepStatus::Pending,
                            blocked_reason: None,
                            attempts: 0,
                        },
                    ],
                }],
            }],
        }
    }

    fn write_minimal_inputs(state: &ProjectState) {
        // PROJECT.md is required by run_step
        let project_md_path = crate::state::mz_root().join("PROJECT.md");
        std::fs::write(&project_md_path, "# Test project\n").unwrap();
        // state.yaml so mark_step_in_progress can mutate it
        crate::state::save(state).unwrap();
        // ST02 PLAN.md (required)
        let plan_path = crate::state::step_plan_path("P001", "TR01", "ST02");
        std::fs::create_dir_all(plan_path.parent().unwrap()).unwrap();
        std::fs::write(&plan_path, "## Plan\nDo step 2.").unwrap();
    }

    #[test]
    fn test_run_step_requires_worktree_dir() {
        let _env = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _tmp = TempMz::new();
        let state = make_two_step_state();
        write_minimal_inputs(&state);
        // No worktree_dir → should bail immediately
        let result = run_step(&state, "P001", "TR01", "ST02", None, None);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("worktree_dir"), "got: {}", msg);
    }

    #[test]
    fn test_run_step_end_to_end_with_mock() {
        let _env = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = TempMz::new();
        let state = make_two_step_state();
        write_minimal_inputs(&state);
        git_init(tmp.path());

        // Make the mock binary write a summary file as a side effect, so the
        // commit step has something real to add.
        let summary_path = crate::state::step_summary_path("P001", "TR01", "ST02");
        std::env::set_var("MZ_CLAUDE_BINARY", mock_claude_binary());
        std::env::set_var("MOCK_CLAUDE_TEXT", "executing step 2");
        std::env::set_var("MOCK_CLAUDE_RESULT", "step 2 done");
        std::env::set_var("MOCK_CLAUDE_WRITE_PATH", summary_path.display().to_string());
        std::env::set_var("MOCK_CLAUDE_WRITE_BODY", "## What was done\nStep 2 implemented.\n");
        std::env::remove_var("MOCK_CLAUDE_MODE");

        let result = run_step(&state, "P001", "TR01", "ST02", None, Some(tmp.path()));

        std::env::remove_var("MZ_CLAUDE_BINARY");
        std::env::remove_var("MOCK_CLAUDE_TEXT");
        std::env::remove_var("MOCK_CLAUDE_RESULT");
        std::env::remove_var("MOCK_CLAUDE_WRITE_PATH");
        std::env::remove_var("MOCK_CLAUDE_WRITE_BODY");

        assert!(
            result.is_ok(),
            "run_step should succeed with mock: {:?}",
            result.err()
        );

        // Side-effect assertions:
        // 1. mark_step_in_progress flipped ST02 → InProgress on disk
        let on_disk = crate::state::load().unwrap();
        let st02_status = on_disk
            .phases
            .iter()
            .find(|p| p.id == "P001")
            .unwrap()
            .tracks
            .iter()
            .find(|t| t.id == "TR01")
            .unwrap()
            .steps
            .iter()
            .find(|s| s.id == "ST02")
            .unwrap()
            .status
            .clone();
        assert_eq!(st02_status, StepStatus::InProgress);

        // 2. The mock-written summary survived (commit_step did NOT clobber it)
        assert!(summary_path.exists(), "summary file should exist");

        // 3. The output log was created and contains both header sections
        let log_path = crate::state::step_output_log_path("P001", "TR01", "ST02");
        assert!(log_path.exists(), "output log should exist");
        let log = std::fs::read_to_string(&log_path).unwrap();
        assert!(log.contains("step execution"), "log should contain execution header");
        assert!(log.contains("summarization"), "log should contain summarization header");

        // 4. run records were appended (execute + summarize stages)
        let records = crate::run_record::load_all().unwrap_or_default();
        let stages: Vec<_> = records
            .iter()
            .filter(|r| r.step_id == "ST02")
            .map(|r| r.stage.as_str())
            .collect();
        assert!(stages.contains(&"execute"), "expected execute record, got {:?}", stages);
        assert!(stages.contains(&"summarize"), "expected summarize record, got {:?}", stages);
    }

    #[test]
    fn test_run_step_propagates_mock_failure() {
        let _env = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = TempMz::new();
        let state = make_two_step_state();
        write_minimal_inputs(&state);
        git_init(tmp.path());

        std::env::set_var("MZ_CLAUDE_BINARY", mock_claude_binary());
        std::env::set_var("MOCK_CLAUDE_MODE", "exit_nonzero");
        std::env::set_var("MOCK_CLAUDE_STDERR", "mock failure for test");

        let result = run_step(&state, "P001", "TR01", "ST02", None, Some(tmp.path()));

        std::env::remove_var("MZ_CLAUDE_BINARY");
        std::env::remove_var("MOCK_CLAUDE_MODE");
        std::env::remove_var("MOCK_CLAUDE_STDERR");

        assert!(result.is_err(), "run_step should fail when mock exits nonzero");
        // Even on failure, an error run record should have been logged
        let records = crate::run_record::load_all().unwrap_or_default();
        let has_error = records
            .iter()
            .any(|r| r.step_id == "ST02" && r.outcome == "error");
        assert!(has_error, "an error run record should have been appended");
    }
}
