use anyhow::{bail, Context, Result};
use std::process::Command;

use crate::state;

#[allow(dead_code)]
pub fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to run git")?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Detect the default branch name (main, master, etc.)
fn default_branch() -> Result<String> {
    // Try symbolic-ref for the remote HEAD
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
        .output();
    if let Ok(out) = output {
        if out.status.success() {
            let branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
            // Returns "origin/main" or "origin/master" — strip the prefix
            if let Some(name) = branch.strip_prefix("origin/") {
                return Ok(name.to_string());
            }
        }
    }

    // Fallback: check if "main" exists
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "refs/heads/main"])
        .output();
    if let Ok(out) = output {
        if out.status.success() {
            return Ok("main".to_string());
        }
    }

    // Fallback: check if "master" exists
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "refs/heads/master"])
        .output();
    if let Ok(out) = output {
        if out.status.success() {
            return Ok("master".to_string());
        }
    }

    Ok("main".to_string())
}

pub fn create_track_branch(phase_id: &str, track_id: &str) -> Result<()> {
    let branch = format!("mz/{}/{}", phase_id, track_id);
    let output = Command::new("git")
        .args(["checkout", "-b", &branch])
        .output()
        .context("Failed to create branch")?;
    if !output.status.success() {
        let output = Command::new("git")
            .args(["checkout", &branch])
            .output()
            .context("Failed to checkout branch")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to checkout branch {}: {}", branch, stderr);
        }
    }
    Ok(())
}

pub fn commit_step(phase_id: &str, track_id: &str, step_id: &str, title: &str) -> Result<()> {
    Command::new("git")
        .args(["add", "-A"])
        .output()
        .context("Failed to git add")?;

    let status = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .output()?;

    if status.status.success() {
        return Ok(());
    }

    let message = format!("{}/{}/{}: {}", phase_id, track_id, step_id, title);
    let output = Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .context("Failed to git commit")?;

    // BUG-5 fix: check git commit exit status
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git commit failed: {}", stderr);
    }

    Ok(())
}

pub fn merge_track(phase_id: &str, track_id: &str) -> Result<()> {
    let branch = format!("mz/{}/{}", phase_id, track_id);
    let default = default_branch()?; // BUG-7 fix: detect default branch

    let project_state = state::load()?;
    let (track_title, step_bullets) = build_track_summary(&project_state, phase_id, track_id);

    // BUG-6 fix: check checkout exit status
    let output = Command::new("git")
        .args(["checkout", &default])
        .output()
        .context("Failed to checkout default branch")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to checkout {}: {}", default, stderr);
    }

    let output = Command::new("git")
        .args(["merge", "--squash", &branch])
        .output()
        .context("Failed to squash merge")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Merge failed: {}", stderr);
    }

    let subject = format!("{}/{}: {}", phase_id, track_id, track_title);
    let body = if step_bullets.is_empty() {
        String::new()
    } else {
        format!("\n\n{}", step_bullets)
    };
    let message = format!("{}{}", subject, body);

    let output = Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .context("Failed to commit merge")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Merge commit failed: {}", stderr);
    }

    Ok(())
}

fn build_track_summary(
    project_state: &state::ProjectState,
    phase_id: &str,
    track_id: &str,
) -> (String, String) {
    let mut track_title = String::from("track complete");
    let mut bullets = Vec::new();

    for phase in &project_state.phases {
        if phase.id != phase_id {
            continue;
        }
        for track in &phase.tracks {
            if track.id != track_id {
                continue;
            }
            track_title = track.title.clone();
            for step in &track.steps {
                let one_liner = state::read_step_summary(phase_id, track_id, &step.id)
                    .ok()
                    .and_then(|s| extract_what_was_done(&s))
                    .unwrap_or_else(|| step.title.clone());
                bullets.push(format!("- {}/{}: {}", track_id, step.id, one_liner));
            }
        }
    }

    (track_title, bullets.join("\n"))
}

fn extract_what_was_done(summary: &str) -> Option<String> {
    let mut in_section = false;
    for line in summary.lines() {
        if line.starts_with("## What was done") || line.starts_with("## What Was Done") {
            in_section = true;
            continue;
        }
        if in_section {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with('#') {
                break;
            }
            let truncated: String = trimmed.chars().take(120).collect();
            return Some(truncated);
        }
    }
    None
}
