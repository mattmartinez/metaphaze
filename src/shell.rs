//! Interactive shell command parser, dispatcher, history, and tab completion.
//!
//! This module backs `mz`'s persistent TUI shell (TR02–TR06 of phase P013).
//! `parse` turns a command-line string into a `ShellCommand` enum, `dispatch`
//! routes a parsed command to the appropriate `cmd_*_inner` function or to a
//! self-contained implementation that emits lines via the event sender.
//!
//! Commands are parsed with simple whitespace tokenization, not clap, because
//! the shell input model is line-by-line and doesn't need clap's full grammar.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;

use crate::events::{self, EventSender, ProgressEvent};
use crate::{budget, run_record, state};

// ── ShellCommand enum ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum ShellCommand {
    Auto {
        max_steps: Option<usize>,
        max_budget_usd: Option<f64>,
    },
    Next {
        max_budget_usd: Option<f64>,
    },
    Plan {
        phase: Option<String>,
    },
    Status {
        detail: bool,
    },
    Doctor,
    Log {
        phase: Option<String>,
        track: Option<String>,
        failed: bool,
        last: usize,
        detail: bool,
        summary: bool,
    },
    Steer {
        message: String,
    },
    Reset {
        step_id: String,
        phase: Option<String>,
    },
    Budget {
        action: Option<BudgetSubcommand>,
    },
    Discuss {
        phase: Option<String>,
    },
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BudgetSubcommand {
    Set { amount: f64 },
    Clear,
}

// ── Parser ─────────────────────────────────────────────────────────────────────

/// Parse a single line of shell input into a `ShellCommand`. Returns a
/// human-readable error string on parse failure (the caller renders it as
/// `OutputLine::Blocked` in the output panel).
pub fn parse(input: &str) -> std::result::Result<ShellCommand, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input".to_string());
    }

    let mut tokens = trimmed.split_whitespace();
    let cmd = tokens.next().unwrap().to_lowercase();
    let rest: Vec<&str> = tokens.collect();

    match cmd.as_str() {
        "auto" => {
            let mut max_steps = None;
            let mut max_budget_usd = None;
            let mut i = 0;
            while i < rest.len() {
                match rest[i] {
                    "--max-steps" => {
                        let v = rest
                            .get(i + 1)
                            .ok_or_else(|| "--max-steps requires a value".to_string())?;
                        max_steps = Some(
                            v.parse::<usize>()
                                .map_err(|_| format!("invalid --max-steps value: {}", v))?,
                        );
                        i += 2;
                    }
                    "--max-budget-usd" => {
                        let v = rest
                            .get(i + 1)
                            .ok_or_else(|| "--max-budget-usd requires a value".to_string())?;
                        max_budget_usd = Some(
                            v.parse::<f64>()
                                .map_err(|_| format!("invalid --max-budget-usd value: {}", v))?,
                        );
                        i += 2;
                    }
                    other => return Err(format!("auto: unknown flag {}", other)),
                }
            }
            Ok(ShellCommand::Auto {
                max_steps,
                max_budget_usd,
            })
        }
        "next" => {
            let mut max_budget_usd = None;
            let mut i = 0;
            while i < rest.len() {
                match rest[i] {
                    "--max-budget-usd" => {
                        let v = rest
                            .get(i + 1)
                            .ok_or_else(|| "--max-budget-usd requires a value".to_string())?;
                        max_budget_usd = Some(
                            v.parse::<f64>()
                                .map_err(|_| format!("invalid --max-budget-usd value: {}", v))?,
                        );
                        i += 2;
                    }
                    other => return Err(format!("next: unknown flag {}", other)),
                }
            }
            Ok(ShellCommand::Next { max_budget_usd })
        }
        "plan" => {
            let phase = rest.first().map(|s| s.to_string());
            Ok(ShellCommand::Plan { phase })
        }
        "status" => {
            let detail = rest.iter().any(|s| *s == "--detail");
            Ok(ShellCommand::Status { detail })
        }
        "doctor" => Ok(ShellCommand::Doctor),
        "log" => {
            let mut phase = None;
            let mut track = None;
            let mut failed = false;
            let mut last: usize = 20;
            let mut detail = false;
            let mut summary = false;
            let mut i = 0;
            while i < rest.len() {
                match rest[i] {
                    "--phase" => {
                        phase = Some(
                            rest.get(i + 1)
                                .ok_or_else(|| "--phase requires a value".to_string())?
                                .to_string(),
                        );
                        i += 2;
                    }
                    "--track" => {
                        track = Some(
                            rest.get(i + 1)
                                .ok_or_else(|| "--track requires a value".to_string())?
                                .to_string(),
                        );
                        i += 2;
                    }
                    "--failed" => {
                        failed = true;
                        i += 1;
                    }
                    "--last" => {
                        let v = rest
                            .get(i + 1)
                            .ok_or_else(|| "--last requires a value".to_string())?;
                        last = v
                            .parse::<usize>()
                            .map_err(|_| format!("invalid --last value: {}", v))?;
                        i += 2;
                    }
                    "--detail" => {
                        detail = true;
                        i += 1;
                    }
                    "--summary" => {
                        summary = true;
                        i += 1;
                    }
                    other => return Err(format!("log: unknown flag {}", other)),
                }
            }
            Ok(ShellCommand::Log {
                phase,
                track,
                failed,
                last,
                detail,
                summary,
            })
        }
        "steer" => {
            if rest.is_empty() {
                return Err("steer requires a message".to_string());
            }
            let message = rest.join(" ");
            Ok(ShellCommand::Steer { message })
        }
        "reset" => {
            if rest.is_empty() {
                return Err("reset requires a step_id (e.g. ST03 or TR02/ST01)".to_string());
            }
            let step_id = rest[0].to_string();
            let mut phase = None;
            let mut i = 1;
            while i < rest.len() {
                match rest[i] {
                    "--phase" => {
                        phase = Some(
                            rest.get(i + 1)
                                .ok_or_else(|| "--phase requires a value".to_string())?
                                .to_string(),
                        );
                        i += 2;
                    }
                    other => return Err(format!("reset: unknown flag {}", other)),
                }
            }
            Ok(ShellCommand::Reset { step_id, phase })
        }
        "budget" => {
            let action = if rest.is_empty() {
                None
            } else {
                match rest[0] {
                    "set" => {
                        let v = rest
                            .get(1)
                            .ok_or_else(|| "budget set requires an amount".to_string())?;
                        let amount = v
                            .parse::<f64>()
                            .map_err(|_| format!("invalid budget amount: {}", v))?;
                        Some(BudgetSubcommand::Set { amount })
                    }
                    "clear" => Some(BudgetSubcommand::Clear),
                    other => return Err(format!("budget: unknown subcommand {}", other)),
                }
            };
            Ok(ShellCommand::Budget { action })
        }
        "discuss" => {
            let phase = rest.first().map(|s| s.to_string());
            Ok(ShellCommand::Discuss { phase })
        }
        "help" | "?" => Ok(ShellCommand::Help),
        other => Err(format!(
            "unknown command: {} — type 'help' for the list",
            other
        )),
    }
}

// ── Command names + tab completion ────────────────────────────────────────────

/// Sorted list of valid shell command names. Used by tab completion and the
/// help text.
pub fn command_names() -> &'static [&'static str] {
    &[
        "auto", "budget", "discuss", "doctor", "help", "log", "next", "plan", "reset", "status",
        "steer",
    ]
}

/// Pure tab-completion helper: returns all `names` that start with `prefix`,
/// case-insensitive. An empty prefix returns the full list. No matches returns
/// an empty vec.
pub fn complete_command(prefix: &str, names: &[&str]) -> Vec<String> {
    let prefix_lower = prefix.to_lowercase();
    names
        .iter()
        .filter(|n| n.to_lowercase().starts_with(&prefix_lower))
        .map(|n| n.to_string())
        .collect()
}

// ── History persistence ───────────────────────────────────────────────────────

const HISTORY_MAX_LINES: usize = 500;

fn history_path() -> PathBuf {
    // state::mz_root() already returns the `.mz/` directory itself.
    state::mz_root().join("history")
}

/// Load command history from `.mz/history`. Missing file or read errors return
/// an empty vec — history is best-effort and never blocks the shell.
pub fn load_history() -> Vec<String> {
    let path = history_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => content
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Save the last `HISTORY_MAX_LINES` entries to `.mz/history`. Missing parent
/// directory or write errors are silently ignored — history is best-effort.
pub fn save_history(history: &[String]) {
    let path = history_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let start = history.len().saturating_sub(HISTORY_MAX_LINES);
    let lines: Vec<&str> = history[start..].iter().map(|s| s.as_str()).collect();
    let body = lines.join("\n");
    let _ = std::fs::write(&path, body);
}

// ── Dispatcher ─────────────────────────────────────────────────────────────────

/// Run a parsed `ShellCommand`. The sender, stop, and paused handles come
/// from the run_interactive event loop and are passed through to the
/// `cmd_*_inner` functions in main.rs for the heavy commands. Lighter
/// commands (status, doctor, log, etc.) emit their output directly via the
/// sender so they appear in the TUI's Output panel.
///
/// `dispatch` always emits an `ExecutionFinished` event before returning so
/// the shell can transition back to `Idle`.
pub fn dispatch(
    cmd: ShellCommand,
    sender: EventSender,
    stop: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
) -> Result<()> {
    let result: Result<()> = match cmd {
        ShellCommand::Auto {
            max_steps,
            max_budget_usd,
        } => crate::cmd_auto_inner(max_steps, max_budget_usd, Some(sender.clone()), stop, paused),
        ShellCommand::Next { max_budget_usd } => {
            crate::cmd_next_inner(max_budget_usd, Some(sender.clone()), stop, paused)
        }
        ShellCommand::Plan { phase } => {
            crate::cmd_plan_inner(phase, Some(sender.clone()), stop, paused)
        }
        ShellCommand::Status { detail } => run_status(&sender, detail),
        ShellCommand::Doctor => run_doctor(&sender),
        ShellCommand::Log {
            phase,
            track,
            failed,
            last,
            detail,
            summary,
        } => run_log(&sender, phase, track, failed, last, detail, summary),
        ShellCommand::Steer { message } => run_steer(&sender, message),
        ShellCommand::Reset { step_id, phase } => run_reset(&sender, step_id, phase),
        ShellCommand::Budget { action } => run_budget(&sender, action),
        ShellCommand::Discuss { .. } => {
            // `discuss` shells out to a real interactive `claude` session
            // and needs cooked stdio. The shell event loop intercepts this
            // variant before reaching the worker-thread dispatcher and runs
            // it on the main thread with the TUI suspended. If we get here,
            // someone called dispatch() outside that path — emit a clear
            // internal-error message rather than silently doing nothing.
            emit(
                &sender,
                "discuss must run on the main thread with the TUI suspended — \
                 not via the worker-thread dispatcher.",
            );
            Ok(())
        }
        ShellCommand::Help => {
            run_help(&sender);
            Ok(())
        }
    };

    // Always tell the shell loop the command is done so it can leave Running.
    let _ = sender.send(ProgressEvent::ExecutionFinished {
        completed: 0,
        blocked: 0,
    });

    result
}

fn emit(sender: &EventSender, msg: &str) {
    events::emit(Some(sender), msg);
}

// ── Helpers for the lightweight commands ──────────────────────────────────────

fn run_status(sender: &EventSender, detail: bool) -> Result<()> {
    let project_state = state::load()?;
    let (total, done, in_progress, blocked) = project_state.stats();
    let pending = total.saturating_sub(done + in_progress + blocked);

    emit(sender, &format!("Project: {}", project_state.name));
    emit(
        sender,
        &format!("Current phase: {}", project_state.current_phase),
    );

    if total == 0 {
        emit(sender, "No steps yet. Run `plan` to decompose into steps.");
        return Ok(());
    }

    let pct = if total > 0 { done * 100 / total } else { 0 };
    emit(
        sender,
        &format!(
            "Progress: {}/{} ({}%) — {} done, {} in-progress, {} pending, {} blocked",
            done, total, pct, done, in_progress, pending, blocked
        ),
    );

    for ph in &project_state.phases {
        emit(sender, &format!("{} — {}", ph.id, ph.title));
        for track in &ph.tracks {
            let track_done = track
                .steps
                .iter()
                .filter(|s| s.status == state::StepStatus::Complete)
                .count();
            let track_total = track.steps.len();
            let marker = if track_done == track_total && track_total > 0 {
                "✓"
            } else {
                "○"
            };
            emit(
                sender,
                &format!(
                    "  {} {} — {} ({}/{})",
                    marker, track.id, track.title, track_done, track_total
                ),
            );

            if !detail {
                // Show only blocked step lines outside detail mode.
                for step in &track.steps {
                    if step.status == state::StepStatus::Blocked {
                        let reason = step.blocked_reason.as_deref().unwrap_or("");
                        emit(
                            sender,
                            &format!("    ✗ {} — {} [{}]", step.id, step.title, reason),
                        );
                    }
                }
                continue;
            }

            for step in &track.steps {
                let icon = match step.status {
                    state::StepStatus::Complete => "✓",
                    state::StepStatus::InProgress => "▶",
                    state::StepStatus::Blocked => "✗",
                    state::StepStatus::Pending => "○",
                };
                emit(
                    sender,
                    &format!("    {} {} — {}", icon, step.id, step.title),
                );
            }
        }
    }
    Ok(())
}

fn run_doctor(sender: &EventSender) -> Result<()> {
    use crate::diagnostics;

    let project_state = state::load()?;
    let records = run_record::load_all().unwrap_or_default();
    let current_phase = project_state.current_phase().to_string();

    let state_issues = diagnostics::check_state_integrity(&project_state);
    let git_issues = diagnostics::check_git_integrity(&project_state);
    let artifact_issues = diagnostics::check_artifacts(&project_state, &current_phase);
    let step_diagnoses = diagnostics::diagnose_all_steps(&records, &project_state);

    emit(sender, "── State ──");
    let all_state: Vec<_> = state_issues.iter().chain(artifact_issues.iter()).collect();
    if all_state.is_empty() {
        emit(sender, "  ✓ All step statuses consistent");
    } else {
        for issue in &all_state {
            let icon = match issue.severity {
                diagnostics::Severity::Error => "✗",
                diagnostics::Severity::Warning => "!",
                diagnostics::Severity::Info => "i",
            };
            emit(sender, &format!("  {} {}", icon, issue.description));
        }
    }

    emit(sender, "── Git ──");
    if git_issues.is_empty() {
        emit(sender, "  ✓ Git state looks clean");
    } else {
        for issue in &git_issues {
            let icon = match issue.severity {
                diagnostics::Severity::Error => "✗",
                diagnostics::Severity::Warning => "!",
                diagnostics::Severity::Info => "i",
            };
            emit(sender, &format!("  {} {}", icon, issue.description));
        }
    }

    emit(sender, &format!("── Steps ({}) ──", current_phase));
    if step_diagnoses.is_empty() {
        emit(sender, "  ✓ No troubled steps detected");
    } else {
        for diag in &step_diagnoses {
            let label = match diag.issue {
                diagnostics::StepIssue::VerifyOscillation => "Verify oscillation",
                diagnostics::StepIssue::RepeatedExecFailure => "Repeated exec failure",
                diagnostics::StepIssue::CostEscalation => "Cost escalation",
            };
            emit(
                sender,
                &format!(
                    "  ✗ {}/{}: {} — {}",
                    diag.track_id, diag.step_id, label, diag.detail
                ),
            );
        }
    }
    Ok(())
}

fn run_log(
    sender: &EventSender,
    phase: Option<String>,
    track: Option<String>,
    failed: bool,
    last: usize,
    _detail: bool,
    summary: bool,
) -> Result<()> {
    let all = run_record::load_all()?;
    if all.is_empty() {
        emit(sender, "No execution history.");
        return Ok(());
    }

    let phase_filter = phase.map(|p| state::normalize_phase_id(&p));
    let track_filter = track.map(|t| t.to_uppercase());

    let filtered: Vec<_> = all
        .into_iter()
        .filter(|r| {
            if let Some(ref p) = phase_filter {
                if r.phase_id != *p {
                    return false;
                }
            }
            if let Some(ref t) = track_filter {
                if r.track_id != *t {
                    return false;
                }
            }
            if failed && r.outcome != "error" {
                return false;
            }
            true
        })
        .collect();

    if filtered.is_empty() {
        emit(sender, "No matching runs.");
        return Ok(());
    }

    if summary {
        let phases = run_record::phase_summaries(&filtered);
        for p in &phases {
            emit(
                sender,
                &format!(
                    "{}: {} runs ({} ok, {} err) — ${:.4}",
                    p.phase_id, p.runs, p.ok, p.err, p.cost_usd
                ),
            );
        }
        return Ok(());
    }

    let total = filtered.len();
    let skip = total.saturating_sub(last);
    for r in filtered.into_iter().skip(skip) {
        let cost = r
            .cost_usd
            .map(|c| format!("${:.4}", c))
            .unwrap_or_else(|| "-".to_string());
        emit(
            sender,
            &format!(
                "{} {}/{}/{} {} {} {} {}",
                r.finished_at.chars().take(19).collect::<String>(),
                r.phase_id,
                r.track_id,
                r.step_id,
                r.stage,
                r.outcome,
                cost,
                r.model
            ),
        );
    }
    Ok(())
}

fn run_steer(sender: &EventSender, message: String) -> Result<()> {
    let project_state = state::load()?;
    state::append_decision(&message)?;
    let phase_id = state::normalize_phase_id(project_state.current_phase());
    emit(
        sender,
        &format!("Recorded decision. Re-planning {}...", phase_id),
    );
    crate::planner::replan(&project_state, &phase_id, &message, Some(sender))?;
    emit(sender, "Re-plan complete.");
    Ok(())
}

fn run_reset(
    sender: &EventSender,
    step_id: String,
    phase: Option<String>,
) -> Result<()> {
    let project_state = state::load()?;
    let phase_id = state::normalize_phase_id(
        &phase.unwrap_or_else(|| project_state.current_phase().to_string()),
    );
    state::reset_step(&phase_id, &step_id)?;
    emit(sender, &format!("Reset {} in {} to pending.", step_id, phase_id));
    Ok(())
}

fn run_budget(sender: &EventSender, action: Option<BudgetSubcommand>) -> Result<()> {
    match action {
        Some(BudgetSubcommand::Set { amount }) => {
            let mut config = budget::load()?;
            config.max_usd = Some(amount);
            budget::save(&config)?;
            emit(sender, &format!("Budget set to ${:.2}", amount));
        }
        Some(BudgetSubcommand::Clear) => {
            let mut config = budget::load()?;
            config.max_usd = None;
            budget::save(&config)?;
            emit(sender, "Budget limit cleared.");
        }
        None => {
            let config = budget::load()?;
            let records = run_record::load_all().unwrap_or_default();
            let status = budget::check(&config, &records);
            match status.limit {
                Some(limit) => emit(
                    sender,
                    &format!("Budget: ${:.4} / ${:.2}", status.spent, limit),
                ),
                None => emit(sender, &format!("Budget: ${:.4} (no limit)", status.spent)),
            }
        }
    }
    Ok(())
}

fn run_help(sender: &EventSender) {
    emit(sender, "Available commands:");
    emit(sender, "  auto [--max-steps N] [--max-budget-usd X]  Run autonomous loop");
    emit(sender, "  next [--max-budget-usd X]                  Execute one step");
    emit(sender, "  plan [PHASE]                               Plan a phase or generate roadmap");
    emit(sender, "  status [--detail]                          Show project progress");
    emit(sender, "  doctor                                     Audit project health");
    emit(sender, "  log [--phase X] [--track X] [--failed] [--last N] [--detail] [--summary]");
    emit(sender, "  steer <message>                            Record decision and re-plan");
    emit(sender, "  reset <step_id> [--phase X]                Reset a step to pending");
    emit(sender, "  budget [set <amount>|clear]                Manage spend budget");
    emit(sender, "  discuss [PHASE]                            Interactive Q&A to capture phase context");
    emit(sender, "  help                                       Show this help");
    emit(sender, "");
    emit(sender, "Type a command and press Enter. Up/Down for history, Tab for completion.");
    emit(sender, "Press q to quit when idle, or q to stop a running command.");
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_commands() {
        assert_eq!(
            parse("auto").unwrap(),
            ShellCommand::Auto {
                max_steps: None,
                max_budget_usd: None,
            }
        );
        assert_eq!(
            parse("next").unwrap(),
            ShellCommand::Next { max_budget_usd: None }
        );
        assert_eq!(
            parse("status").unwrap(),
            ShellCommand::Status { detail: false }
        );
        assert_eq!(
            parse("status --detail").unwrap(),
            ShellCommand::Status { detail: true }
        );
        assert_eq!(parse("doctor").unwrap(), ShellCommand::Doctor);
        assert_eq!(parse("help").unwrap(), ShellCommand::Help);
        assert_eq!(parse("?").unwrap(), ShellCommand::Help);
    }

    #[test]
    fn parse_commands_with_args() {
        assert_eq!(
            parse("plan P009").unwrap(),
            ShellCommand::Plan {
                phase: Some("P009".to_string()),
            }
        );
        assert_eq!(
            parse("auto --max-steps 5").unwrap(),
            ShellCommand::Auto {
                max_steps: Some(5),
                max_budget_usd: None,
            }
        );
        assert_eq!(
            parse("auto --max-budget-usd 10.50").unwrap(),
            ShellCommand::Auto {
                max_steps: None,
                max_budget_usd: Some(10.50),
            }
        );
        assert_eq!(
            parse("steer fix the auth bug").unwrap(),
            ShellCommand::Steer {
                message: "fix the auth bug".to_string(),
            }
        );
        assert_eq!(
            parse("reset ST03 --phase P010").unwrap(),
            ShellCommand::Reset {
                step_id: "ST03".to_string(),
                phase: Some("P010".to_string()),
            }
        );
        assert_eq!(
            parse("reset TR02/ST01").unwrap(),
            ShellCommand::Reset {
                step_id: "TR02/ST01".to_string(),
                phase: None,
            }
        );
    }

    #[test]
    fn parse_budget_subcommands() {
        assert_eq!(parse("budget").unwrap(), ShellCommand::Budget { action: None });
        assert_eq!(
            parse("budget set 10.00").unwrap(),
            ShellCommand::Budget {
                action: Some(BudgetSubcommand::Set { amount: 10.00 }),
            }
        );
        assert_eq!(
            parse("budget clear").unwrap(),
            ShellCommand::Budget {
                action: Some(BudgetSubcommand::Clear),
            }
        );
    }

    #[test]
    fn parse_log_with_flags() {
        let cmd = parse("log --phase P008 --failed --last 5 --summary").unwrap();
        match cmd {
            ShellCommand::Log {
                phase,
                track,
                failed,
                last,
                detail,
                summary,
            } => {
                assert_eq!(phase, Some("P008".to_string()));
                assert_eq!(track, None);
                assert!(failed);
                assert_eq!(last, 5);
                assert!(!detail);
                assert!(summary);
            }
            other => panic!("expected Log, got {:?}", other),
        }
    }

    #[test]
    fn parse_errors() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
        assert!(parse("foobar").is_err());
        assert!(parse("steer").is_err());
        assert!(parse("reset").is_err());
        assert!(parse("auto --max-steps").is_err());
        assert!(parse("auto --max-steps notanumber").is_err());
        assert!(parse("budget set").is_err());
        assert!(parse("budget set notanumber").is_err());
        assert!(parse("budget weird").is_err());
    }

    #[test]
    fn parse_is_case_insensitive_for_command() {
        assert!(matches!(parse("AUTO").unwrap(), ShellCommand::Auto { .. }));
        assert!(matches!(parse("Status").unwrap(), ShellCommand::Status { .. }));
    }

    #[test]
    fn command_names_is_sorted_and_nonempty() {
        let names = command_names();
        assert!(!names.is_empty());
        let mut sorted = names.to_vec();
        sorted.sort();
        assert_eq!(sorted, names.to_vec());
    }

    #[test]
    fn complete_command_basic() {
        let names = command_names();
        assert_eq!(complete_command("au", names), vec!["auto"]);
        assert_eq!(complete_command("s", names), vec!["status", "steer"]);
        assert_eq!(complete_command("xyz", names), Vec::<String>::new());
        assert_eq!(complete_command("", names).len(), names.len());
    }

    #[test]
    fn complete_command_case_insensitive() {
        let names = command_names();
        assert_eq!(complete_command("Au", names), vec!["auto"]);
        assert_eq!(complete_command("DOC", names), vec!["doctor"]);
    }

    /// RAII guard: sets the thread-local mz dir override to a temp dir for the
    /// duration of the test, then clears it on drop.
    struct TempMz {
        _dir: tempfile::TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let mz_path = dir.path().join(".mz");
            std::fs::create_dir_all(&mz_path).unwrap();
            state::set_test_mz_dir(Some(mz_path));
            TempMz { _dir: dir }
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            state::set_test_mz_dir(None);
        }
    }

    #[test]
    fn history_save_and_load_round_trip() {
        let _tmp = TempMz::new();

        let entries: Vec<String> = vec!["status".into(), "auto".into(), "doctor".into()];
        save_history(&entries);
        let loaded = load_history();
        assert_eq!(loaded, entries);
    }

    #[test]
    fn history_caps_at_500_lines() {
        let _tmp = TempMz::new();

        let entries: Vec<String> = (0..600).map(|i| format!("cmd{}", i)).collect();
        save_history(&entries);
        let loaded = load_history();
        assert_eq!(loaded.len(), HISTORY_MAX_LINES);
        // Should keep the most recent 500.
        assert_eq!(loaded[0], "cmd100");
        assert_eq!(loaded[HISTORY_MAX_LINES - 1], "cmd599");
    }

    #[test]
    fn history_missing_file_is_empty() {
        let _tmp = TempMz::new();
        let loaded = load_history();
        assert!(loaded.is_empty());
    }

    // ── Dispatch state-machine smoke tests ────────────────────────────────────
    //
    // These don't require a terminal — they exercise the parser → dispatch
    // path with a real mpsc channel and assert the lightweight commands emit
    // an ExecutionFinished event so the shell loop transitions back to Idle.

    fn drain(rx: &events::EventReceiver) -> Vec<ProgressEvent> {
        let mut out = Vec::new();
        while let Ok(ev) = rx.try_recv() {
            out.push(ev);
        }
        out
    }

    fn has_execution_finished(events: &[ProgressEvent]) -> bool {
        events
            .iter()
            .any(|e| matches!(e, ProgressEvent::ExecutionFinished { .. }))
    }

    #[test]
    fn dispatch_help_emits_finish_and_help_lines() {
        let (tx, rx) = events::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let paused = Arc::new(AtomicBool::new(false));

        let cmd = parse("help").unwrap();
        dispatch(cmd, tx, stop, paused).unwrap();

        let events = drain(&rx);
        assert!(has_execution_finished(&events));
        // Help should emit at least the "Available commands:" header.
        let saw_header = events.iter().any(|e| match e {
            ProgressEvent::ClaudeOutput { line, .. } => line.contains("Available commands"),
            _ => false,
        });
        assert!(saw_header, "expected Help to emit the header line");
    }

    #[test]
    fn dispatch_discuss_via_worker_emits_internal_error_and_finish() {
        // The shell loop now intercepts `discuss` and runs it on the main
        // thread with the TUI suspended. The dispatcher branch is a safety
        // net that should never be hit in normal flow — if it is, it must
        // emit a diagnostic instead of silently spawning claude on a worker
        // thread (which would deadlock the terminal).
        let (tx, rx) = events::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let paused = Arc::new(AtomicBool::new(false));

        let cmd = parse("discuss").unwrap();
        dispatch(cmd, tx, stop, paused).unwrap();

        let events = drain(&rx);
        assert!(has_execution_finished(&events));
        let saw_msg = events.iter().any(|e| match e {
            ProgressEvent::ClaudeOutput { line, .. } => line.contains("main thread"),
            _ => false,
        });
        assert!(saw_msg, "expected discuss to emit the internal-error message");
    }
}
