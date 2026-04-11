use crate::run_record::{self, RunRecord};
use crate::state::{self, ProjectState, StepStatus};
use std::process::Command;

pub struct StepDiagnosis {
    pub phase_id: String,
    pub track_id: String,
    pub step_id: String,
    pub issue: StepIssue,
    pub detail: String,
    pub cost_burned: f64,
}

pub enum StepIssue {
    VerifyOscillation,
    RepeatedExecFailure,
    CostEscalation,
}

pub fn diagnose_step(
    records: &[RunRecord],
    phase_id: &str,
    track_id: &str,
    step_id: &str,
) -> Option<StepDiagnosis> {
    let step_records: Vec<&RunRecord> = records
        .iter()
        .filter(|r| r.phase_id == phase_id && r.track_id == track_id && r.step_id == step_id)
        .collect();

    if step_records.is_empty() {
        return None;
    }

    let cost_burned: f64 = step_records.iter().filter_map(|r| r.cost_usd).sum();

    // Check VerifyOscillation: 2+ execute_step with outcome "success" followed by verify_step with outcome "error"
    let oscillation_count = count_verify_oscillations(&step_records);
    if oscillation_count >= 2 {
        return Some(StepDiagnosis {
            phase_id: phase_id.to_string(),
            track_id: track_id.to_string(),
            step_id: step_id.to_string(),
            issue: StepIssue::VerifyOscillation,
            detail: format!(
                "{} exec-ok/verify-fail cycles detected",
                oscillation_count
            ),
            cost_burned,
        });
    }

    // Check RepeatedExecFailure: 2+ execute_step with outcome "error" in a row
    if has_repeated_exec_failure(&step_records) {
        let fail_count = count_consecutive_exec_failures(&step_records);
        return Some(StepDiagnosis {
            phase_id: phase_id.to_string(),
            track_id: track_id.to_string(),
            step_id: step_id.to_string(),
            issue: StepIssue::RepeatedExecFailure,
            detail: format!("{} consecutive execute failures", fail_count),
            cost_burned,
        });
    }

    // Check CostEscalation: last attempt's cost > 2x the first attempt's cost
    if let Some(detail) = check_cost_escalation(&step_records) {
        return Some(StepDiagnosis {
            phase_id: phase_id.to_string(),
            track_id: track_id.to_string(),
            step_id: step_id.to_string(),
            issue: StepIssue::CostEscalation,
            detail,
            cost_burned,
        });
    }

    None
}

fn count_verify_oscillations(records: &[&RunRecord]) -> usize {
    // Count occurrences of: execute_step success followed by verify_step error
    let mut count = 0;
    let mut last_exec_success = false;
    for r in records {
        if r.stage == "execute_step" && r.outcome == "success" {
            last_exec_success = true;
        } else if r.stage == "verify_step" && r.outcome == "error" && last_exec_success {
            count += 1;
            last_exec_success = false;
        } else if r.stage == "execute_step" {
            last_exec_success = r.outcome == "success";
        }
    }
    count
}

fn has_repeated_exec_failure(records: &[&RunRecord]) -> bool {
    count_consecutive_exec_failures(records) >= 2
}

fn count_consecutive_exec_failures(records: &[&RunRecord]) -> usize {
    let mut max_consecutive = 0usize;
    let mut current = 0usize;
    for r in records {
        if r.stage == "execute_step" && r.outcome == "error" {
            current += 1;
            if current > max_consecutive {
                max_consecutive = current;
            }
        } else if r.stage == "execute_step" {
            current = 0;
        }
    }
    max_consecutive
}

fn check_cost_escalation(records: &[&RunRecord]) -> Option<String> {
    let first_cost = records.first().and_then(|r| r.cost_usd)?;
    let last_cost = records.last().and_then(|r| r.cost_usd)?;
    if first_cost > 0.0 && last_cost > first_cost * 2.0 {
        Some(format!(
            "cost grew from ${:.4} to ${:.4} ({:.1}x)",
            first_cost,
            last_cost,
            last_cost / first_cost
        ))
    } else {
        None
    }
}

pub fn diagnose_all_steps(records: &[RunRecord], state: &ProjectState) -> Vec<StepDiagnosis> {
    let current_phase_id = state.current_phase();
    let mut diagnoses = Vec::new();

    for phase in &state.phases {
        if phase.id != current_phase_id {
            continue;
        }
        for track in &phase.tracks {
            for step in &track.steps {
                if step.status != StepStatus::Blocked && step.status != StepStatus::InProgress {
                    continue;
                }
                if let Some(diagnosis) =
                    diagnose_step(records, &phase.id, &track.id, &step.id)
                {
                    diagnoses.push(diagnosis);
                }
            }
        }
    }

    diagnoses
}

pub fn format_blocked_reason(diagnosis: &StepDiagnosis) -> String {
    match &diagnosis.issue {
        StepIssue::VerifyOscillation => format!(
            "Verify oscillation: {}, ${:.2} burned",
            diagnosis.detail, diagnosis.cost_burned
        ),
        StepIssue::RepeatedExecFailure => format!(
            "Repeated exec failure: {}, ${:.2} burned",
            diagnosis.detail, diagnosis.cost_burned
        ),
        StepIssue::CostEscalation => format!(
            "Cost escalation: {}, ${:.2} burned",
            diagnosis.detail, diagnosis.cost_burned
        ),
    }
}

pub enum Severity {
    Error,
    Warning,
    Info,
}

pub struct HealthIssue {
    pub severity: Severity,
    pub description: String,
    pub suggestion: String,
}

pub fn check_state_integrity(state: &ProjectState) -> Vec<HealthIssue> {
    let mut issues = Vec::new();

    // Load run records for stale-in-progress check; ignore errors (empty ledger is fine)
    let records = run_record::load_all().unwrap_or_default();

    // Check if all tracks in current_phase are complete but current_phase still points here
    let current = state.current_phase();
    if state.is_phase_complete(current) {
        // There is no `mz advance` command — auto-advance happens inside
        // `mz auto` when it detects a complete phase mid-run, or when
        // `mz plan` plans the next phase. Suggest the right one based on
        // whether a next phase already exists in state.
        let suggestion = match state.next_phase_id() {
            Some(next) => format!(
                "Run `mz auto` to execute and auto-advance into {}",
                next
            ),
            None => "All planned phases complete. Run `mz plan` to generate a new roadmap, \
                     or `mz discuss <phase>` to start a new one."
                .to_string(),
        };
        issues.push(HealthIssue {
            severity: Severity::Warning,
            description: format!(
                "Phase {} has all tracks complete but current_phase still points to it",
                current
            ),
            suggestion,
        });
    }

    for phase in &state.phases {
        for track in &phase.tracks {
            for step in &track.steps {
                match step.status {
                    StepStatus::Complete => {
                        // Check SUMMARY.md exists
                        let summary_path =
                            state::step_summary_path(&phase.id, &track.id, &step.id);
                        if !summary_path.exists() {
                            issues.push(HealthIssue {
                                severity: Severity::Error,
                                                    description: format!(
                                    "{}/{}/{} is Complete but SUMMARY.md is missing",
                                    phase.id, track.id, step.id
                                ),
                                suggestion: format!(
                                    "Step may have been interrupted. Use mz reset {} --phase {} to re-run.",
                                    step.id, phase.id
                                ),
                            });
                        }
                        // Check PLAN.md exists
                        let plan_path =
                            state::step_plan_path(&phase.id, &track.id, &step.id);
                        if !plan_path.exists() {
                            issues.push(HealthIssue {
                                severity: Severity::Warning,
                                                    description: format!(
                                    "{}/{}/{} is Complete but PLAN.md is missing",
                                    phase.id, track.id, step.id
                                ),
                                suggestion: format!(
                                    "Create {}",
                                    plan_path.display()
                                ),
                            });
                        }
                    }
                    StepStatus::InProgress => {
                        // Check for stale in-progress: no run records in last 30 minutes
                        let step_records: Vec<&RunRecord> = records
                            .iter()
                            .filter(|r| {
                                r.phase_id == phase.id
                                    && r.track_id == track.id
                                    && r.step_id == step.id
                            })
                            .collect();

                        let is_stale = if step_records.is_empty() {
                            true
                        } else {
                            let most_recent = step_records
                                .iter()
                                .filter_map(|r| {
                                    chrono::DateTime::parse_from_rfc3339(&r.started_at).ok()
                                })
                                .max();
                            match most_recent {
                                None => true,
                                Some(ts) => {
                                    let now = chrono::Utc::now();
                                    let age = now.signed_duration_since(ts.with_timezone(&chrono::Utc));
                                    age.num_minutes() >= 30
                                }
                            }
                        };

                        if is_stale {
                            issues.push(HealthIssue {
                                severity: Severity::Warning,
                                                    description: format!(
                                    "{}/{}/{} is InProgress with no recent activity (>30 min)",
                                    phase.id, track.id, step.id
                                ),
                                suggestion: "Check if an agent is still running; if not, reset the step status".to_string(),
                            });
                        }
                    }
                    StepStatus::Pending => {
                        // Pending but attempts > 0 is an impossible state
                        if step.attempts > 0 {
                            issues.push(HealthIssue {
                                severity: Severity::Error,
                                                    description: format!(
                                    "{}/{}/{} is Pending but has {} attempts recorded",
                                    phase.id, track.id, step.id, step.attempts
                                ),
                                suggestion: "Manually inspect state.yaml and correct the step status".to_string(),
                            });
                        }
                    }
                    StepStatus::Blocked => {}
                }
            }
        }
    }

    issues
}

pub fn check_git_integrity(state: &ProjectState) -> Vec<HealthIssue> {
    let mut issues = Vec::new();

    // List all mz/* branches
    let output = Command::new("git")
        .args(["branch", "--list", "mz/*"])
        .output();

    let branches = match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .map(|l| {
                    // `git branch` marks the current branch with `* ` and
                    // branches checked out in another worktree with `+ `.
                    // Strip whichever marker is present, otherwise the raw
                    // line bleeds into the suggested fix command.
                    let trimmed = l.trim();
                    trimmed
                        .strip_prefix("* ")
                        .or_else(|| trimmed.strip_prefix("+ "))
                        .unwrap_or(trimmed)
                        .to_string()
                })
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
        }
        _ => vec![],
    };

    // Build set of active mz/ branches (Pending or InProgress tracks)
    let active_branches: std::collections::HashSet<String> = state
        .phases
        .iter()
        .flat_map(|ph| {
            ph.tracks.iter().filter_map(move |track| {
                let has_active = track
                    .steps
                    .iter()
                    .any(|s| s.status == StepStatus::Pending || s.status == StepStatus::InProgress);
                if has_active {
                    Some(format!("mz/{}/{}", ph.id, track.id))
                } else {
                    None
                }
            })
        })
        .collect();

    for branch in &branches {
        if !active_branches.contains(branch) {
            issues.push(HealthIssue {
                severity: Severity::Warning,
                description: format!(
                    "Branch '{}' has no matching pending/in-progress track in state",
                    branch
                ),
                suggestion: format!("Run: git branch -d {}", branch),
            });
        }
    }

    // Check current branch
    let current_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_default();

    // Detect default branch
    let default_branch = detect_default_branch();

    if !current_branch.is_empty()
        && current_branch != default_branch
        && !active_branches.contains(&current_branch)
    {
        issues.push(HealthIssue {
            severity: Severity::Info,
            description: format!(
                "Current branch '{}' is not the default branch and not an active mz/ branch",
                current_branch
            ),
            suggestion: format!(
                "Switch to an active branch or '{}' if not intentional",
                default_branch
            ),
        });
    }

    issues
}

fn detect_default_branch() -> String {
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
        .output();
    if let Ok(out) = output {
        if out.status.success() {
            let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Some(name) = branch.strip_prefix("origin/") {
                return name.to_string();
            }
        }
    }
    "main".to_string()
}

pub fn check_artifacts(state: &ProjectState, phase_id: &str) -> Vec<HealthIssue> {
    let mut issues = Vec::new();

    let phase = match state.phases.iter().find(|ph| ph.id == phase_id) {
        Some(p) => p,
        None => return issues,
    };

    for track in &phase.tracks {
        for step in &track.steps {
            match step.status {
                StepStatus::Complete => {
                    // Check track directory exists
                    let dir = state::track_dir(phase_id, &track.id);
                    if !dir.exists() {
                        issues.push(HealthIssue {
                            severity: Severity::Error,
                            description: format!(
                                "Track directory missing for completed step {}/{}/{}",
                                phase_id, track.id, step.id
                            ),
                            suggestion: format!("Expected directory: {}", dir.display()),
                        });
                        continue;
                    }
                    // Check SUMMARY.md
                    let summary_path = state::step_summary_path(phase_id, &track.id, &step.id);
                    if !summary_path.exists() {
                        issues.push(HealthIssue {
                            severity: Severity::Error,
                            description: format!(
                                "SUMMARY.md missing for completed step {}/{}/{}",
                                phase_id, track.id, step.id
                            ),
                            suggestion: format!(
                                "Step may have been interrupted. Use mz reset {} --phase {} to re-run.",
                                step.id, phase_id
                            ),
                        });
                    }
                    // Check PLAN.md
                    let plan_path = state::step_plan_path(phase_id, &track.id, &step.id);
                    if !plan_path.exists() {
                        issues.push(HealthIssue {
                            severity: Severity::Warning,
                            description: format!(
                                "PLAN.md missing for completed step {}/{}/{}",
                                phase_id, track.id, step.id
                            ),
                            suggestion: format!("Create {}", plan_path.display()),
                        });
                    }
                }
                StepStatus::Blocked => {
                    // Check blocked_reason is set
                    let reason_missing = step
                        .blocked_reason
                        .as_ref()
                        .map(|r| r.trim().is_empty())
                        .unwrap_or(true);
                    if reason_missing {
                        issues.push(HealthIssue {
                            severity: Severity::Warning,
                            description: format!(
                                "Step {}/{}/{} is Blocked but has no blocked_reason",
                                phase_id, track.id, step.id
                            ),
                            suggestion: "Set a blocked_reason in state.yaml to document why this step is blocked".to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn make_record(stage: &str, outcome: &str, cost: f64) -> RunRecord {
        RunRecord {
            id: Uuid::new_v4().to_string(),
            phase_id: "P001".to_string(),
            track_id: "TR01".to_string(),
            step_id: "ST01".to_string(),
            stage: stage.to_string(),
            model: "claude-sonnet-4-6".to_string(),
            started_at: "2026-04-09T00:00:00Z".to_string(),
            finished_at: "2026-04-09T00:00:01Z".to_string(),
            duration_ms: 1000,
            cost_usd: Some(cost),
            num_turns: Some(1),
            outcome: outcome.to_string(),
            error: None,
            input_tokens: None,
            output_tokens: None,
        }
    }

    #[test]
    fn test_verify_oscillation_detected() {
        let records = vec![
            make_record("execute_step", "success", 0.10),
            make_record("verify_step", "error", 0.05),
            make_record("execute_step", "success", 0.10),
            make_record("verify_step", "error", 0.05),
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01").unwrap();
        assert!(matches!(diagnosis.issue, StepIssue::VerifyOscillation));
        assert!((diagnosis.cost_burned - 0.30).abs() < 1e-9);
    }

    #[test]
    fn test_verify_oscillation_single_not_detected() {
        // Only 1 cycle — should not trigger VerifyOscillation
        let records = vec![
            make_record("execute_step", "success", 0.10),
            make_record("verify_step", "error", 0.05),
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01");
        // May be None or a different issue, but not VerifyOscillation
        if let Some(d) = diagnosis {
            assert!(!matches!(d.issue, StepIssue::VerifyOscillation));
        }
    }

    #[test]
    fn test_repeated_exec_failure_detected() {
        let records = vec![
            make_record("execute_step", "error", 0.10),
            make_record("execute_step", "error", 0.10),
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01").unwrap();
        assert!(matches!(diagnosis.issue, StepIssue::RepeatedExecFailure));
    }

    #[test]
    fn test_single_exec_failure_not_detected() {
        let records = vec![make_record("execute_step", "error", 0.10)];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01");
        assert!(diagnosis.is_none());
    }

    #[test]
    fn test_cost_escalation_detected() {
        let records = vec![
            make_record("execute_step", "success", 0.10),
            make_record("execute_step", "success", 0.25), // > 2x
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01").unwrap();
        assert!(matches!(diagnosis.issue, StepIssue::CostEscalation));
    }

    #[test]
    fn test_cost_escalation_exactly_2x_not_detected() {
        // exactly 2x should NOT trigger (must be strictly greater)
        let records = vec![
            make_record("execute_step", "success", 0.10),
            make_record("execute_step", "success", 0.20), // exactly 2x
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01");
        assert!(diagnosis.is_none());
    }

    #[test]
    fn test_no_records_returns_none() {
        let records = vec![];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01");
        assert!(diagnosis.is_none());
    }

    #[test]
    fn test_cost_burned_sum() {
        let records = vec![
            make_record("execute_step", "error", 0.10),
            make_record("execute_step", "error", 0.20),
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01").unwrap();
        assert!((diagnosis.cost_burned - 0.30).abs() < 1e-9);
    }

    #[test]
    fn test_format_blocked_reason_verify_oscillation() {
        let diagnosis = StepDiagnosis {
            phase_id: "P001".to_string(),
            track_id: "TR01".to_string(),
            step_id: "ST01".to_string(),
            issue: StepIssue::VerifyOscillation,
            detail: "3 exec-ok/verify-fail cycles detected".to_string(),
            cost_burned: 0.42,
        };
        let reason = format_blocked_reason(&diagnosis);
        assert!(reason.contains("Verify oscillation"));
        assert!(reason.contains("$0.42"));
    }

    #[test]
    fn test_format_blocked_reason_repeated_exec_failure() {
        let diagnosis = StepDiagnosis {
            phase_id: "P001".to_string(),
            track_id: "TR01".to_string(),
            step_id: "ST01".to_string(),
            issue: StepIssue::RepeatedExecFailure,
            detail: "2 consecutive execute failures".to_string(),
            cost_burned: 0.20,
        };
        let reason = format_blocked_reason(&diagnosis);
        assert!(reason.contains("Repeated exec failure"));
        assert!(reason.contains("$0.20"));
    }

    // ── check_state_integrity tests ──────────────────────────────────────────

    use crate::state::{PhaseEntry, StepEntry, TrackEntry};
    use tempfile::TempDir;

    struct TempMz {
        _dir: TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let mz_path = dir.path().join(".mz");
            std::fs::create_dir_all(&mz_path).unwrap();
            crate::state::set_test_mz_dir(Some(mz_path));
            TempMz { _dir: dir }
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            crate::state::set_test_mz_dir(None);
        }
    }

    fn make_state_with_step(status: StepStatus, attempts: u32) -> ProjectState {
        ProjectState {
            name: "test".to_string(),
            description: "test".to_string(),
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
                        status,
                        blocked_reason: None,
                        attempts,
                    }],
                }],
            }],
        }
    }

    #[test]
    fn test_complete_step_missing_summary_is_error() {
        let _tmp = TempMz::new();
        let state = make_state_with_step(StepStatus::Complete, 1);
        // No SUMMARY.md or PLAN.md exist on disk
        let issues = check_state_integrity(&state);
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.severity, Severity::Error) && i.description.contains("SUMMARY"))
            .collect();
        assert!(!errors.is_empty(), "Expected Error for missing SUMMARY.md");
    }

    #[test]
    fn test_complete_step_missing_plan_is_warning() {
        let _tmp = TempMz::new();
        // Create SUMMARY.md but not PLAN.md
        let state = make_state_with_step(StepStatus::Complete, 1);
        let summary_path = crate::state::step_summary_path("P001", "TR01", "ST01");
        std::fs::create_dir_all(summary_path.parent().unwrap()).unwrap();
        std::fs::write(&summary_path, "## What was done\nDone.").unwrap();

        let issues = check_state_integrity(&state);
        let warnings: Vec<_> = issues
            .iter()
            .filter(|i| {
                matches!(i.severity, Severity::Warning) && i.description.contains("PLAN")
            })
            .collect();
        assert!(!warnings.is_empty(), "Expected Warning for missing PLAN.md");
    }

    #[test]
    fn test_pending_with_attempts_is_error() {
        let _tmp = TempMz::new();
        // attempts > 0 but Pending — impossible state
        let state = make_state_with_step(StepStatus::Pending, 3);
        let issues = check_state_integrity(&state);
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| {
                matches!(i.severity, Severity::Error) && i.description.contains("attempts")
            })
            .collect();
        assert!(!errors.is_empty(), "Expected Error for Pending step with attempts > 0");
    }

    #[test]
    fn test_pending_with_zero_attempts_no_error() {
        let _tmp = TempMz::new();
        let state = make_state_with_step(StepStatus::Pending, 0);
        let issues = check_state_integrity(&state);
        let attempt_errors: Vec<_> = issues
            .iter()
            .filter(|i| i.description.contains("attempts"))
            .collect();
        assert!(attempt_errors.is_empty(), "Pending with 0 attempts should not produce error");
    }

    #[test]
    fn test_all_tracks_complete_but_current_phase_not_advanced_is_warning() {
        let _tmp = TempMz::new();
        // All steps Complete — phase should have advanced
        let state = make_state_with_step(StepStatus::Complete, 1);
        // Create SUMMARY.md to avoid unrelated errors
        let summary_path = crate::state::step_summary_path("P001", "TR01", "ST01");
        let plan_path = crate::state::step_plan_path("P001", "TR01", "ST01");
        std::fs::create_dir_all(summary_path.parent().unwrap()).unwrap();
        std::fs::write(&summary_path, "done").unwrap();
        std::fs::write(&plan_path, "plan").unwrap();

        let issues = check_state_integrity(&state);
        let phase_warnings: Vec<_> = issues
            .iter()
            .filter(|i| {
                matches!(i.severity, Severity::Warning)
                    && i.description.contains("all tracks complete")
            })
            .collect();
        assert!(
            !phase_warnings.is_empty(),
            "Expected Warning for phase not advanced"
        );
    }

    #[test]
    fn test_in_progress_no_records_is_stale_warning() {
        let _tmp = TempMz::new();
        // InProgress with no run records → stale
        let state = make_state_with_step(StepStatus::InProgress, 0);
        let issues = check_state_integrity(&state);
        let stale_warnings: Vec<_> = issues
            .iter()
            .filter(|i| {
                matches!(i.severity, Severity::Warning) && i.description.contains("InProgress")
            })
            .collect();
        assert!(!stale_warnings.is_empty(), "Expected Warning for stale InProgress step");
    }

    #[test]
    fn test_check_artifacts_blocked_no_reason_is_warning() {
        let _tmp = TempMz::new();
        let state = ProjectState {
            name: "test".to_string(),
            description: "test".to_string(),
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
                        status: StepStatus::Blocked,
                        blocked_reason: None,
                        attempts: 2,
                    }],
                }],
            }],
        };
        let issues = check_artifacts(&state, "P001");
        let warnings: Vec<_> = issues
            .iter()
            .filter(|i| {
                matches!(i.severity, Severity::Warning)
                    && i.description.contains("no blocked_reason")
            })
            .collect();
        assert!(!warnings.is_empty(), "Expected Warning for Blocked step without reason");
    }

    #[test]
    fn test_healthy_step_returns_none() {
        let records = vec![
            make_record("execute_step", "success", 0.10),
            make_record("verify_step", "success", 0.05),
        ];
        let diagnosis = diagnose_step(&records, "P001", "TR01", "ST01");
        assert!(diagnosis.is_none(), "Healthy step should return None");
    }

    #[test]
    fn test_check_state_integrity_clean() {
        let _tmp = TempMz::new();
        // Complete step with both SUMMARY.md and PLAN.md present
        let state = make_state_with_step(StepStatus::Complete, 1);
        let summary_path = crate::state::step_summary_path("P001", "TR01", "ST01");
        let plan_path = crate::state::step_plan_path("P001", "TR01", "ST01");
        std::fs::create_dir_all(summary_path.parent().unwrap()).unwrap();
        std::fs::write(&summary_path, "## Done").unwrap();
        std::fs::write(&plan_path, "## Plan").unwrap();

        let issues = check_state_integrity(&state);
        // Only possible remaining issue is the phase-not-advanced warning (all complete)
        let non_phase_issues: Vec<_> = issues
            .iter()
            .filter(|i| !i.description.contains("all tracks complete"))
            .collect();
        assert!(
            non_phase_issues.is_empty(),
            "Expected no state integrity issues for clean state, got: {:?}",
            non_phase_issues.iter().map(|i| &i.description).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_diagnose_all_steps_filters_current_phase() {
        // State: current_phase = P001. P001/TR01/ST01 is InProgress (no bad records).
        // P002/TR01/ST01 is InProgress and has repeated exec failures in records.
        // Only P001 is the current phase, so diagnose_all_steps should return empty.
        let state = ProjectState {
            name: "test".to_string(),
            description: "test".to_string(),
            current_phase: "P001".to_string(),
            phases: vec![
                PhaseEntry {
                    id: "P001".to_string(),
                    title: "Phase 1".to_string(),
                    tracks: vec![TrackEntry {
                        id: "TR01".to_string(),
                        title: "Track 1".to_string(),
                        depends_on: vec![],
                        steps: vec![StepEntry {
                            id: "ST01".to_string(),
                            title: "Step 1".to_string(),
                            status: StepStatus::InProgress,
                            blocked_reason: None,
                            attempts: 1,
                        }],
                    }],
                },
                PhaseEntry {
                    id: "P002".to_string(),
                    title: "Phase 2".to_string(),
                    tracks: vec![TrackEntry {
                        id: "TR01".to_string(),
                        title: "Track 1".to_string(),
                        depends_on: vec![],
                        steps: vec![StepEntry {
                            id: "ST01".to_string(),
                            title: "Step 1".to_string(),
                            status: StepStatus::InProgress,
                            blocked_reason: None,
                            attempts: 2,
                        }],
                    }],
                },
            ],
        };

        // Records for P002/TR01/ST01 show repeated exec failures
        let p002_records = vec![
            RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: "P002".to_string(),
                track_id: "TR01".to_string(),
                step_id: "ST01".to_string(),
                stage: "execute_step".to_string(),
                model: "claude-sonnet-4-6".to_string(),
                started_at: "2026-04-09T00:00:00Z".to_string(),
                finished_at: "2026-04-09T00:00:01Z".to_string(),
                duration_ms: 1000,
                cost_usd: Some(0.10),
                num_turns: Some(1),
                outcome: "error".to_string(),
                error: None,
                input_tokens: None,
                output_tokens: None,
            },
            RunRecord {
                id: uuid::Uuid::new_v4().to_string(),
                phase_id: "P002".to_string(),
                track_id: "TR01".to_string(),
                step_id: "ST01".to_string(),
                stage: "execute_step".to_string(),
                model: "claude-sonnet-4-6".to_string(),
                started_at: "2026-04-09T00:00:02Z".to_string(),
                finished_at: "2026-04-09T00:00:03Z".to_string(),
                duration_ms: 1000,
                cost_usd: Some(0.10),
                num_turns: Some(1),
                outcome: "error".to_string(),
                error: None,
                input_tokens: None,
                output_tokens: None,
            },
        ];

        let diagnoses = diagnose_all_steps(&p002_records, &state);
        assert!(
            diagnoses.is_empty(),
            "diagnose_all_steps should only check current phase (P001), not P002"
        );
    }

    #[test]
    fn test_check_artifacts_blocked_with_reason_no_warning() {
        let _tmp = TempMz::new();
        let state = ProjectState {
            name: "test".to_string(),
            description: "test".to_string(),
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
                        status: StepStatus::Blocked,
                        blocked_reason: Some("Repeated exec failure: 3 consecutive".to_string()),
                        attempts: 3,
                    }],
                }],
            }],
        };
        let issues = check_artifacts(&state, "P001");
        let no_reason_warnings: Vec<_> = issues
            .iter()
            .filter(|i| i.description.contains("no blocked_reason"))
            .collect();
        assert!(no_reason_warnings.is_empty(), "Should not warn when blocked_reason is set");
    }
}
