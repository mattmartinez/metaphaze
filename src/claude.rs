use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::process::Command;

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

pub fn run(opts: ClaudeOptions) -> Result<String> {
    let claude_bin = find_claude()?;

    let mut cmd = Command::new(&claude_bin);

    cmd.arg("-p").arg(&opts.prompt);
    cmd.arg("--permission-mode").arg("acceptEdits");
    cmd.arg("--output-format").arg("text");

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
    eprintln!("{} binary={}", "  [claude]".dimmed(), claude_bin.dimmed());
    let model_str = opts.model.as_deref().unwrap_or("default");
    let turns_str = opts.max_turns.map(|t| t.to_string()).unwrap_or_else(|| "∞".to_string());
    let prompt_preview: String = opts.prompt.chars().take(80).collect();
    let prompt_preview = if opts.prompt.len() > 80 {
        format!("{}...", prompt_preview)
    } else {
        prompt_preview
    };

    eprintln!(
        "{} model={} turns={} prompt={}",
        "  [claude]".dimmed(),
        model_str.cyan(),
        turns_str,
        prompt_preview.dimmed(),
    );

    let output = cmd
        .output()
        .with_context(|| format!("Failed to spawn claude at {}", claude_bin))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stderr.is_empty() {
        for line in stderr.lines() {
            eprintln!("{} {}", "  [claude]".dimmed(), line.dimmed());
        }
    }

    if !output.status.success() {
        let combined = if stderr.is_empty() {
            format!("claude exited with {} (no stderr output)\nstdout: {}", output.status, &stdout[..stdout.len().min(500)])
        } else {
            format!("claude exited with {}: {}", output.status, stderr.trim())
        };
        bail!("{}", combined);
    }

    let stdout_len = stdout.len();
    eprintln!(
        "{} done, {} bytes of output",
        "  [claude]".dimmed(),
        stdout_len,
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
    cmd.arg("--permission-mode").arg("acceptEdits");
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
