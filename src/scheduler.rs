use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;

use crate::{budget, diagnostics, events, events::EventSender, executor, git, run_record, state, verifier};
use crate::state::{ProjectState, StepStatus};

/// Ensures all worktrees in the list are removed when dropped.
/// Used as a panic-safe cleanup guard — call `defuse()` after explicit cleanup.
struct WorktreeGuard {
    paths: Vec<PathBuf>,
}

impl WorktreeGuard {
    fn new(paths: Vec<PathBuf>) -> Self {
        WorktreeGuard { paths }
    }

    /// Clear the path list so Drop does nothing (call after explicit cleanup is done).
    fn defuse(&mut self) {
        self.paths.clear();
    }
}

impl Drop for WorktreeGuard {
    fn drop(&mut self) {
        for path in &self.paths {
            let _ = git::remove_worktree(path);
        }
    }
}

pub struct ParallelScheduler {
    pub sender: Option<EventSender>,
    pub stop: Arc<AtomicBool>,
    pub paused: Arc<AtomicBool>,
    pub max_budget_usd: Option<f64>,
}

pub struct BatchResult {
    pub completed: usize,
    pub blocked: usize,
    pub tracks_finished: Vec<String>,
}

/// Returns `(phase_id, track_id, [(step_id, step_title)])` for each track in the current phase
/// that has at least one pending step and whose `depends_on` entries are all complete.
pub fn runnable_tracks(
    project_state: &ProjectState,
) -> Vec<(String, String, Vec<(String, String)>)> {
    let current_phase_id = &project_state.current_phase;

    let phase = match project_state
        .phases
        .iter()
        .find(|ph| ph.id == *current_phase_id)
    {
        Some(p) => p,
        None => return Vec::new(),
    };

    let mut result = Vec::new();

    for track in &phase.tracks {
        // Replicate the track_deps_satisfied logic from ProjectState.
        let deps_ok = track.depends_on.iter().all(|dep_id| {
            phase.tracks.iter().any(|t| {
                t.id == *dep_id && t.steps.iter().all(|s| s.status == StepStatus::Complete)
            })
        });
        if !deps_ok {
            continue;
        }

        let pending: Vec<(String, String)> = track
            .steps
            .iter()
            .filter(|s| s.status == StepStatus::Pending)
            .map(|s| (s.id.clone(), s.title.clone()))
            .collect();

        if pending.is_empty() {
            continue;
        }

        result.push((current_phase_id.clone(), track.id.clone(), pending));
    }

    result
}

struct TrackTask {
    phase_id: String,
    track_id: String,
    pending_steps: Vec<(String, String)>,
    worktree_path: PathBuf,
}

struct TrackOutcome {
    track_id: String,
    completed: usize,
    blocked: usize,
}

impl ParallelScheduler {
    pub fn run_parallel_batch(&self) -> Result<BatchResult> {
        let project_state = state::load()?;
        let runnable = runnable_tracks(&project_state);

        // If 0 or 1 tracks are runnable, return early — caller falls back to serial.
        if runnable.len() <= 1 {
            return Ok(BatchResult {
                completed: 0,
                blocked: 0,
                tracks_finished: Vec::new(),
            });
        }

        // Create a worktree for every runnable track.
        let mut tasks: Vec<TrackTask> = Vec::with_capacity(runnable.len());
        for (phase_id, track_id, pending_steps) in runnable {
            let worktree_path = git::create_worktree(&phase_id, &track_id)?;
            tasks.push(TrackTask {
                phase_id,
                track_id,
                pending_steps,
                worktree_path,
            });
        }

        // Collect worktree paths keyed by track_id for merge lookup and cleanup.
        let track_to_worktree: HashMap<String, PathBuf> = tasks
            .iter()
            .map(|t| (t.track_id.clone(), t.worktree_path.clone()))
            .collect();

        // Guard ensures all worktrees are cleaned up even if the merge loop panics.
        let mut wt_guard = WorktreeGuard::new(track_to_worktree.values().cloned().collect());

        // Spawn one thread per track.
        let stop = Arc::clone(&self.stop);
        let paused = Arc::clone(&self.paused);

        let max_budget_usd = self.max_budget_usd;

        let handles: Vec<std::thread::JoinHandle<TrackOutcome>> = tasks
            .into_iter()
            .map(|task| {
                let phase_id = task.phase_id;
                let track_id = task.track_id;
                let pending_steps = task.pending_steps;
                let worktree_path = task.worktree_path;
                let project_state = project_state.clone();
                let sender_clone = self.sender.clone();
                let stop = Arc::clone(&stop);
                let paused = Arc::clone(&paused);

                std::thread::spawn(move || {
                    let mut completed = 0usize;
                    let mut blocked = 0usize;

                    'steps: for (step_id, _step_title) in &pending_steps {
                        // Respect stop flag between steps.
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        // Spin while paused.
                        while paused.load(Ordering::Relaxed) {
                            if stop.load(Ordering::Relaxed) {
                                break 'steps;
                            }
                            std::thread::sleep(std::time::Duration::from_millis(200));
                        }

                        // Budget check before each step.
                        if let Ok(budget_config) = budget::load() {
                            let effective_config = match max_budget_usd {
                                Some(l) => budget::BudgetConfig { max_usd: Some(l) },
                                None => budget_config,
                            };
                            if let Ok(records) = run_record::load_all() {
                                let bstatus = budget::check(&effective_config, &records);
                                if bstatus.exhausted {
                                    if let Some(tx) = &sender_clone {
                                        let _ = tx.send(events::ProgressEvent::BudgetExhausted {
                                            spent: bstatus.spent,
                                            limit: bstatus.limit.unwrap_or(0.0),
                                        });
                                    }
                                    break 'steps;
                                }
                            }
                        }

                        if let Some(tx) = &sender_clone {
                            let _ = tx.send(events::ProgressEvent::StepStarted {
                                track_id: track_id.clone(),
                                step_num: completed + blocked,
                                total_steps: pending_steps.len(),
                            });
                        }

                        let sender_ref = sender_clone.as_ref();
                        let mut step_retries = 0u32;

                        'retry: loop {
                            if stop.load(Ordering::Relaxed) {
                                break 'steps;
                            }

                            // executor::run_step internally calls mark_step_in_progress.
                            let exec_result = executor::run_step(
                                &project_state,
                                &phase_id,
                                &track_id,
                                step_id,
                                sender_ref,
                                Some(&worktree_path),
                            );

                            match exec_result {
                                Ok(()) => {
                                    let verify_result = verifier::run_step(
                                        &project_state,
                                        &phase_id,
                                        &track_id,
                                        step_id,
                                        sender_ref,
                                    );

                                    match verify_result {
                                        Ok(()) => {
                                            let _ = state::mark_step_complete(
                                                &phase_id, &track_id, step_id,
                                            );
                                            completed += 1;
                                            if let Some(tx) = &sender_clone {
                                                let _ = tx.send(events::ProgressEvent::StepCompleted {
                                                    track_id: track_id.clone(),
                                                });
                                            }
                                            break 'retry;
                                        }
                                        Err(_) => {
                                            step_retries += 1;
                                            let _ = state::increment_step_attempts(&phase_id, &track_id, step_id);
                                            let records = run_record::load_all().unwrap_or_default();
                                            let diagnosis = diagnostics::diagnose_step(&records, &phase_id, &track_id, step_id);
                                            let is_oscillating = matches!(
                                                diagnosis.as_ref().map(|d| &d.issue),
                                                Some(diagnostics::StepIssue::VerifyOscillation)
                                            );
                                            if step_retries >= 2 || is_oscillating {
                                                let blocked_reason = diagnosis
                                                    .as_ref()
                                                    .map(|d| diagnostics::format_blocked_reason(d))
                                                    .unwrap_or_else(|| "Verification failed".to_string());
                                                let _ = state::mark_step_blocked(&phase_id, &track_id, step_id, &blocked_reason);
                                                blocked += 1;
                                                if let Some(tx) = &sender_clone {
                                                    let _ = tx.send(events::ProgressEvent::StepBlocked {
                                                        track_id: track_id.clone(),
                                                        step_id: step_id.clone(),
                                                        reason: blocked_reason,
                                                    });
                                                }
                                                break 'steps;
                                            } else {
                                                if let Some(tx) = &sender_clone {
                                                    let _ = tx.send(events::ProgressEvent::StepFailed {
                                                        track_id: track_id.clone(),
                                                    });
                                                }
                                                continue 'retry;
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    step_retries += 1;
                                    let _ = state::increment_step_attempts(&phase_id, &track_id, step_id);
                                    if step_retries >= 3 {
                                        let records = run_record::load_all().unwrap_or_default();
                                        let diagnosis = diagnostics::diagnose_step(&records, &phase_id, &track_id, step_id);
                                        let blocked_reason = diagnosis
                                            .as_ref()
                                            .map(|d| diagnostics::format_blocked_reason(d))
                                            .unwrap_or_else(|| "Failed after 3 retries".to_string());
                                        let _ = state::mark_step_blocked(&phase_id, &track_id, step_id, &blocked_reason);
                                        blocked += 1;
                                        if let Some(tx) = &sender_clone {
                                            let _ = tx.send(events::ProgressEvent::StepBlocked {
                                                track_id: track_id.clone(),
                                                step_id: step_id.clone(),
                                                reason: blocked_reason,
                                            });
                                        }
                                        break 'steps;
                                    } else {
                                        if let Some(tx) = &sender_clone {
                                            let _ = tx.send(events::ProgressEvent::StepFailed {
                                                track_id: track_id.clone(),
                                            });
                                        }
                                        continue 'retry;
                                    }
                                }
                            }
                        } // 'retry
                    } // 'steps

                    TrackOutcome {
                        track_id,
                        completed,
                        blocked,
                    }
                })
            })
            .collect();

        // Join all threads and aggregate results.
        let mut total_completed = 0usize;
        let mut total_blocked = 0usize;
        let mut tracks_finished: Vec<String> = Vec::new();

        for handle in handles {
            if let Ok(outcome) = handle.join() {
                total_completed += outcome.completed;
                total_blocked += outcome.blocked;
                if outcome.blocked == 0 && outcome.completed > 0 {
                    tracks_finished.push(outcome.track_id);
                }
            }
        }

        // Merge completed tracks in TR-order (sorted by track ID).
        tracks_finished.sort();

        let updated_state = state::load()?;
        let phase_id = &updated_state.current_phase;

        for track_id in &tracks_finished {
            let wt_path = track_to_worktree.get(track_id).map(PathBuf::as_path);
            if let Err(e) = git::merge_track(phase_id, track_id, wt_path) {
                events::emit(
                    self.sender.as_ref(),
                    &format!("Warning: merge failed for {}: {}", track_id, e),
                );
                // Mark all remaining pending steps blocked so user knows to reset and retry.
                if let Ok(cur_state) = state::load() {
                    if let Some(phase) = cur_state.phases.iter().find(|p| p.id == *phase_id) {
                        if let Some(track) = phase.tracks.iter().find(|t| t.id == *track_id) {
                            for step in &track.steps {
                                if step.status == StepStatus::Pending {
                                    let _ = state::mark_step_blocked(
                                        phase_id,
                                        track_id,
                                        &step.id,
                                        "Merge conflict after parallel execution",
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        // Explicit cleanup with event logging; defuse the guard to prevent double-cleanup.
        for (track_id, wt_path) in &track_to_worktree {
            if let Err(e) = git::remove_worktree(wt_path) {
                events::emit(
                    self.sender.as_ref(),
                    &format!(
                        "Warning: failed to remove worktree for {}: {}",
                        track_id,
                        e
                    ),
                );
            }
        }
        wt_guard.defuse();

        Ok(BatchResult {
            completed: total_completed,
            blocked: total_blocked,
            tracks_finished,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{PhaseEntry, StepEntry, StepStatus, TrackEntry};

    fn make_step(id: &str, status: StepStatus) -> StepEntry {
        StepEntry {
            id: id.to_string(),
            title: format!("Step {}", id),
            status,
            blocked_reason: None,
            attempts: 0,
        }
    }

    fn make_track(id: &str, depends_on: Vec<&str>, steps: Vec<StepEntry>) -> TrackEntry {
        TrackEntry {
            id: id.to_string(),
            title: format!("Track {}", id),
            steps,
            depends_on: depends_on.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn make_state(tracks: Vec<TrackEntry>) -> ProjectState {
        ProjectState {
            name: "test".to_string(),
            description: "test".to_string(),
            current_phase: "P001".to_string(),
            phases: vec![PhaseEntry {
                id: "P001".to_string(),
                title: "Phase 1".to_string(),
                tracks,
            }],
        }
    }

    /// TR01 (no deps, pending) and TR02 (no deps, pending) should be runnable.
    /// TR03 (depends on TR01, which is not complete) should NOT be runnable.
    #[test]
    fn test_runnable_tracks_respects_depends_on() {
        let tr01 = make_track("TR01", vec![], vec![make_step("ST01", StepStatus::Pending)]);
        let tr02 = make_track("TR02", vec![], vec![make_step("ST01", StepStatus::Pending)]);
        let tr03 = make_track(
            "TR03",
            vec!["TR01"],
            vec![make_step("ST01", StepStatus::Pending)],
        );

        let state = make_state(vec![tr01, tr02, tr03]);
        let runnable = runnable_tracks(&state);

        let runnable_ids: Vec<&str> = runnable.iter().map(|(_, t, _)| t.as_str()).collect();

        assert_eq!(runnable.len(), 2, "Expected TR01 and TR02 to be runnable, got: {:?}", runnable_ids);
        assert!(
            runnable_ids.contains(&"TR01"),
            "TR01 should be runnable"
        );
        assert!(
            runnable_ids.contains(&"TR02"),
            "TR02 should be runnable"
        );
        assert!(
            !runnable_ids.contains(&"TR03"),
            "TR03 should NOT be runnable because TR01 is not complete"
        );
    }

    /// TR03 should become runnable once TR01 is complete.
    #[test]
    fn test_runnable_tracks_dep_complete_unlocks() {
        let tr01 = make_track(
            "TR01",
            vec![],
            vec![make_step("ST01", StepStatus::Complete)],
        );
        let tr03 = make_track(
            "TR03",
            vec!["TR01"],
            vec![make_step("ST01", StepStatus::Pending)],
        );

        let state = make_state(vec![tr01, tr03]);
        let runnable = runnable_tracks(&state);

        let runnable_ids: Vec<&str> = runnable.iter().map(|(_, t, _)| t.as_str()).collect();

        assert!(
            runnable_ids.contains(&"TR03"),
            "TR03 should be runnable when TR01 is complete"
        );
    }
}
