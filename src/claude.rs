use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::events::EventSender;
use crate::stream::{self, ContentBlock, DeltaContent, StreamEvent};

/// Write a diagnostic line to /tmp/mz-stream-debug.log when MZ_STREAM_DEBUG is set.
fn stream_debug_log(msg: &str) {
    if std::env::var("MZ_STREAM_DEBUG").is_ok() {
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/mz-stream-debug.log")
        {
            let _ = writeln!(f, "[claude] {}", msg);
        }
    }
}

pub struct RunResult {
    pub output: String,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    pub num_turns: Option<u32>,
    pub model: String,
    pub wall_clock_ms: u64,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}

pub struct ClaudeOptions {
    pub prompt: String,
    pub model: Option<String>,
    pub max_turns: Option<u32>,
    pub allowed_tools: Vec<String>,
    pub append_system_prompt: Option<String>,
    pub cwd: Option<PathBuf>,
}

impl ClaudeOptions {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            model: None,
            max_turns: Some(50),
            allowed_tools: vec![],
            append_system_prompt: None,
            cwd: None,
        }
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn max_turns(mut self, turns: u32) -> Self {
        self.max_turns = Some(turns);
        self
    }

    pub fn system_prompt(mut self, prompt: &str) -> Self {
        self.append_system_prompt = Some(prompt.to_string());
        self
    }

    pub fn cwd(mut self, dir: PathBuf) -> Self {
        self.cwd = Some(dir);
        self
    }
}

pub fn run(opts: ClaudeOptions, sender: Option<&EventSender>) -> Result<RunResult> {
    let start = std::time::Instant::now();
    let claude_bin = find_claude()?;

    let mut cmd = Command::new(&claude_bin);

    cmd.arg("-p").arg(&opts.prompt);
    cmd.arg("--permission-mode").arg("acceptEdits");
    cmd.arg("--verbose");
    cmd.arg("--output-format").arg("stream-json");

    if let Some(model) = &opts.model {
        cmd.arg("--model").arg(model);
    }

    if let Some(turns) = opts.max_turns {
        cmd.arg("--max-turns").arg(turns.to_string());
    }

    if let Some(sys) = &opts.append_system_prompt {
        cmd.arg("--append-system-prompt").arg(sys);
    }

    for tool in &opts.allowed_tools {
        cmd.arg("--allowedTools").arg(tool);
    }

    if let Some(ref dir) = opts.cwd {
        cmd.current_dir(dir);
    }

    // Log what we're about to run
    let model_str = opts.model.as_deref().unwrap_or("default");
    let turns_str = opts.max_turns.map(|t| t.to_string()).unwrap_or_else(|| "∞".to_string());
    crate::events::emit(sender, &format!("[claude] model={} turns={}", model_str, turns_str));

    // Stream output: pipe stdout line-by-line so the user sees progress
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn claude at {}", claude_bin))?;

    let child_stdout = child.stdout.take().unwrap();
    let child_stderr = child.stderr.take().unwrap();

    // Read stderr in a background thread
    let stderr_handle = std::thread::spawn(move || {
        let reader = BufReader::new(child_stderr);
        let mut stderr = String::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                stderr.push_str(&line);
                stderr.push('\n');
            }
        }
        stderr
    });

    // Parse stream-json NDJSON events from stdout
    let mut stdout = String::new();
    let mut result_received = false;
    let mut fallback_lines: Vec<String> = Vec::new();
    let mut last_tool_summary: Option<String> = None;
    let mut detected_model = String::new();
    let mut run_cost: Option<f64> = None;
    let mut run_duration: Option<u64> = None;
    let mut run_turns: Option<u32> = None;
    let mut run_input_tokens: Option<u64> = None;
    let mut run_output_tokens: Option<u64> = None;

    // Diagnostic counters — written to /tmp/mz-stream-debug.log when MZ_STREAM_DEBUG is set.
    let mut dbg_lines_total: usize = 0;
    let mut dbg_content_block_delta: usize = 0;
    let mut dbg_token_send_ok: usize = 0;
    let mut dbg_token_send_err: usize = 0;
    let mut dbg_tool_use: usize = 0;
    let mut dbg_tool_result: usize = 0;
    let mut dbg_other_parsed: usize = 0;

    // Tracks whether any content_block_delta events were seen for the current assistant turn.
    // If the CLI doesn't emit token-level deltas, we fall back to the assembled `assistant` event.
    let mut seen_token_deltas = false;

    let reader = BufReader::new(child_stdout);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                dbg_lines_total += 1;
                match stream::parse_stream_line(&line) {
                    Some(StreamEvent::Assistant { message }) => {
                        dbg_other_parsed += 1;
                        let text: String = message.content.iter()
                            .filter_map(|block| {
                                if let ContentBlock::Text { text } = block { Some(text.as_str()) } else { None }
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        if !text.is_empty() {
                            if sender.is_none() {
                                // Non-TUI: user already saw text token-by-token; just terminate the line
                                eprintln!();
                            } else if !seen_token_deltas {
                                // TUI fallback: CLI didn't emit content_block_delta events.
                                // Send the assembled text line-by-line so the output panel shows it.
                                if let Some(tx) = sender {
                                    stream_debug_log("assistant fallback: sending assembled text via AssistantText");
                                    for line in text.lines() {
                                        let _ = tx.send(crate::events::ProgressEvent::AssistantText {
                                            text: line.to_string(),
                                        });
                                    }
                                }
                            }
                            // else: content already streamed token-by-token via TokenDelta
                        }
                        // Reset per-turn flag so the next assistant turn is evaluated independently
                        seen_token_deltas = false;
                    }
                    Some(StreamEvent::ContentBlockDelta { ref delta }) => {
                        dbg_content_block_delta += 1;
                        seen_token_deltas = true;
                        if let DeltaContent::TextDelta { text } = delta {
                            if let Some(tx) = sender {
                                let send_result = tx.send(crate::events::ProgressEvent::TokenDelta {
                                    text: text.clone(),
                                });
                                match send_result {
                                    Ok(_) => dbg_token_send_ok += 1,
                                    Err(_) => {
                                        dbg_token_send_err += 1;
                                        stream_debug_log(&format!(
                                            "TokenDelta send FAILED (receiver dropped?), text={:?}",
                                            &text[..text.len().min(40)]
                                        ));
                                    }
                                }
                            } else {
                                // Non-TUI: stream to stderr in real-time
                                eprint!("{}", text.dimmed());
                                let _ = std::io::stderr().flush();
                            }
                        }
                        // Non-text deltas (input_json_delta) are ignored
                    }
                    Some(ref parsed @ StreamEvent::ToolUse { ref tool, .. }) => {
                        dbg_tool_use += 1;
                        let summary = parsed.tool_use_summary().unwrap_or_else(|| tool.clone());
                        last_tool_summary = Some(summary.clone());
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ToolUseStarted { tool: summary.clone() });
                        } else {
                            eprintln!("  {} {}", "🔧".cyan(), summary.cyan());
                        }
                    }
                    Some(StreamEvent::ToolResult { tool, .. }) => {
                        dbg_tool_result += 1;
                        let result_tool = last_tool_summary.take().unwrap_or_else(|| tool.clone());
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ToolResultReceived { tool: result_tool.clone() });
                        } else {
                            eprintln!("  {} {}", "✓".dimmed(), result_tool.dimmed());
                        }
                    }
                    Some(StreamEvent::Result { result, cost_usd, duration_ms, num_turns, input_tokens, output_tokens }) => {
                        dbg_other_parsed += 1;
                        run_cost = cost_usd;
                        run_duration = duration_ms;
                        run_turns = num_turns;
                        // Prefer Result-level token counts; they'll override MessageStart values
                        if input_tokens.is_some() {
                            run_input_tokens = input_tokens;
                        }
                        if output_tokens.is_some() {
                            run_output_tokens = output_tokens;
                        }
                        stdout = result;
                        result_received = true;
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ClaudeOutput {
                                line: "── done ──".to_string(),
                            });
                            // Emit live cost update: historical + current run
                            if let Ok(records) = crate::run_record::load_all() {
                                let historical = crate::run_record::total_project_cost(&records);
                                let current = cost_usd.unwrap_or(0.0);
                                let budget_cfg = crate::budget::load().unwrap_or_default();
                                let _ = tx.send(crate::events::ProgressEvent::CostUpdate {
                                    spent: historical + current,
                                    limit: budget_cfg.max_usd,
                                });
                            }
                        }
                    }
                    Some(StreamEvent::Error { error }) => {
                        dbg_other_parsed += 1;
                        let msg = format!("⚠ {}", error);
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ClaudeOutput { line: msg.clone() });
                        } else {
                            eprintln!("  {} {}", "⚠".red(), error.red());
                        }
                    }
                    Some(StreamEvent::MessageStart { message }) => {
                        dbg_other_parsed += 1;
                        detected_model = message.model.clone();
                        // Capture initial token usage from MessageStart as fallback
                        if let Some(usage) = &message.usage {
                            if run_input_tokens.is_none() {
                                run_input_tokens = usage.input_tokens;
                            }
                            if run_output_tokens.is_none() {
                                run_output_tokens = usage.output_tokens;
                            }
                        }
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ModelDetected {
                                model: message.model.clone(),
                            });
                        }
                    }
                    Some(StreamEvent::User { .. }) | Some(StreamEvent::System { .. }) => {
                        dbg_other_parsed += 1;
                        // Ignore user/system-level events
                    }
                    None => {
                        stream_debug_log(&format!("fallback (unparsed): {}", &line[..line.len().min(120)]));
                        fallback_lines.push(line);
                    }
                }
            }
            Err(e) => eprintln!("{} read error: {}", "  [claude]".dimmed(), e),
        }
    }

    stream_debug_log(&format!(
        "stream summary: lines={} content_block_delta={} token_send_ok={} token_send_err={} \
         tool_use={} tool_result={} other_parsed={} fallback={}",
        dbg_lines_total, dbg_content_block_delta, dbg_token_send_ok, dbg_token_send_err,
        dbg_tool_use, dbg_tool_result, dbg_other_parsed, fallback_lines.len()
    ));

    let status = child.wait().context("Failed to wait for claude")?;
    let stderr = stderr_handle.join().unwrap_or_default();

    if !status.success() {
        let combined = if stderr.is_empty() {
            // BUG-24 fix: truncate at char boundary, not byte boundary
            let preview: String = stdout.chars().take(500).collect();
            format!("claude exited with {} (no stderr)\nstdout: {}", status, preview)
        } else {
            format!("claude exited with {}: {}", status, stderr.trim())
        };
        bail!("{}", combined);
    }

    if !result_received {
        bail!("No result event in stream-json output");
    }

    crate::events::emit(sender, &format!("[claude] done, {} bytes", stdout.len()));

    Ok(RunResult {
        output: stdout,
        cost_usd: run_cost,
        duration_ms: run_duration,
        num_turns: run_turns,
        model: detected_model,
        wall_clock_ms: start.elapsed().as_millis() as u64,
        input_tokens: run_input_tokens,
        output_tokens: run_output_tokens,
    })
}

/// Launch claude as an interactive session with a system prompt.
/// The user talks to claude directly. Claude writes artifacts to disk.
pub fn run_interactive(system_prompt: &str, initial_prompt: &str) -> Result<()> {
    let claude_bin = find_claude()?;

    eprintln!("{} launching interactive session...", "  [claude]".dimmed());

    let mut cmd = Command::new(&claude_bin);
    cmd.arg("--append-system-prompt").arg(system_prompt);
    cmd.arg("--dangerously-skip-permissions");
    cmd.arg(initial_prompt);

    // Inherit stdin/stdout/stderr so the user interacts directly
    use std::process::Stdio;
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd.status().context("Failed to launch claude")?;

    if !status.success() {
        bail!("claude session exited with {}", status);
    }

    Ok(())
}

fn find_claude() -> Result<String> {
    // Find all `claude` binaries in PATH via `which -a`
    let output = Command::new("which").arg("-a").arg("claude").output()?;
    if output.status.success() {
        let paths = String::from_utf8_lossy(&output.stdout);
        // Prefer the real binary (Mach-O/ELF) over shell wrappers
        for path in paths.lines() {
            let path = path.trim();
            if path.is_empty() {
                continue;
            }
            if !is_shell_script(std::path::Path::new(path)) {
                return Ok(path.to_string());
            }
        }
        // Fall back to first match if all are scripts
        if let Some(first) = paths.lines().next() {
            let first = first.trim();
            if !first.is_empty() {
                return Ok(first.to_string());
            }
        }
    }

    bail!(
        "Claude Code CLI not found. Install it: https://docs.anthropic.com/en/docs/claude-code\n\
         Or ensure `claude` is in your PATH."
    )
}

fn is_shell_script(path: &std::path::Path) -> bool {
    std::fs::read(path)
        .map(|bytes| bytes.starts_with(b"#!"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_options_cwd_builder() {
        let dir = PathBuf::from("/tmp/test-worktree");
        let opts = ClaudeOptions::new("hello".to_string()).cwd(dir.clone());
        assert_eq!(opts.cwd, Some(dir));
    }

    #[test]
    fn test_claude_options_cwd_default_none() {
        let opts = ClaudeOptions::new("hello".to_string());
        assert!(opts.cwd.is_none());
    }
}
