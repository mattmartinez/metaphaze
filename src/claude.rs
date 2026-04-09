use anyhow::{bail, Context, Result};
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
    cmd.arg("--yes");
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

    let output = cmd
        .output()
        .with_context(|| format!("Failed to spawn claude at {}", claude_bin))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("claude exited with {}: {}", output.status, stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

fn find_claude() -> Result<String> {
    // Check common locations
    let candidates = [
        "claude",
        "/usr/local/bin/claude",
        "/opt/homebrew/bin/claude",
    ];

    for candidate in candidates {
        if Command::new("which")
            .arg(candidate)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Ok(candidate.to_string());
        }
    }

    // Try which directly
    let output = Command::new("which").arg("claude").output()?;
    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
            return Ok(path);
        }
    }

    bail!(
        "Claude Code CLI not found. Install it: https://docs.anthropic.com/en/docs/claude-code\n\
         Or ensure `claude` is in your PATH."
    )
}
