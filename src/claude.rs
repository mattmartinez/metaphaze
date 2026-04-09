use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::process::Command;

use crate::events::EventSender;
use crate::stream::{self, ContentBlock, StreamEvent};

pub struct ClaudeOptions {
    pub prompt: String,
    pub model: Option<String>,
    pub max_turns: Option<u32>,
    pub allowed_tools: Vec<String>,
    pub append_system_prompt: Option<String>,
}

impl ClaudeOptions {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            model: None,
            max_turns: Some(50),
            allowed_tools: vec![],
            append_system_prompt: None,
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
}

pub fn run(opts: ClaudeOptions, sender: Option<&EventSender>) -> Result<String> {
    let claude_bin = find_claude()?;

    let mut cmd = Command::new(&claude_bin);

    cmd.arg("-p").arg(&opts.prompt);
    cmd.arg("--permission-mode").arg("acceptEdits");
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

    // Log what we're about to run
    let model_str = opts.model.as_deref().unwrap_or("default");
    let turns_str = opts.max_turns.map(|t| t.to_string()).unwrap_or_else(|| "∞".to_string());
    eprintln!(
        "{} model={} turns={}",
        "  [claude]".dimmed(),
        model_str.cyan(),
        turns_str,
    );

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
    let reader = BufReader::new(child_stdout);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                match stream::parse_stream_line(&line) {
                    Some(StreamEvent::Assistant { message }) => {
                        let text: String = message.content.iter()
                            .filter_map(|block| {
                                if let ContentBlock::Text { text } = block { Some(text.as_str()) } else { None }
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        if !text.is_empty() {
                            if let Some(tx) = sender {
                                let _ = tx.send(crate::events::ProgressEvent::AssistantText { text: text.clone() });
                            } else {
                                eprintln!("  {}", text.dimmed());
                            }
                        }
                    }
                    Some(StreamEvent::ToolUse { tool, .. }) => {
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ToolUseStarted { tool: tool.clone() });
                        } else {
                            eprintln!("  {} {}", "🔧".cyan(), tool.cyan());
                        }
                    }
                    Some(StreamEvent::ToolResult { tool, .. }) => {
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ToolResultReceived { tool: tool.clone() });
                        } else {
                            eprintln!("  {} {}", "✓".dimmed(), tool.dimmed());
                        }
                    }
                    Some(StreamEvent::Result { result, .. }) => {
                        stdout = result;
                        result_received = true;
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ClaudeOutput {
                                line: "── done ──".to_string(),
                            });
                        }
                    }
                    Some(StreamEvent::Error { error }) => {
                        let msg = format!("⚠ {}", error);
                        if let Some(tx) = sender {
                            let _ = tx.send(crate::events::ProgressEvent::ClaudeOutput { line: msg.clone() });
                        } else {
                            eprintln!("  {} {}", "⚠".red(), error.red());
                        }
                    }
                    Some(StreamEvent::System { .. }) => {
                        // Ignore system-level events
                    }
                    None => {
                        fallback_lines.push(line);
                    }
                }
            }
            Err(e) => eprintln!("{} read error: {}", "  [claude]".dimmed(), e),
        }
    }

    let status = child.wait().context("Failed to wait for claude")?;
    let stderr = stderr_handle.join().unwrap_or_default();

    if !status.success() {
        let combined = if stderr.is_empty() {
            format!("claude exited with {} (no stderr)\nstdout: {}", status, &stdout[..stdout.len().min(500)])
        } else {
            format!("claude exited with {}: {}", status, stderr.trim())
        };
        bail!("{}", combined);
    }

    if !result_received {
        bail!("No result event in stream-json output");
    }

    eprintln!(
        "{} done, {} bytes",
        "  [claude]".dimmed(),
        stdout.len(),
    );

    Ok(stdout)
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
