use anyhow::{Context, Result};
use std::process::Command;

#[allow(dead_code)]
pub fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to run git")?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn create_slice_branch(milestone_id: &str, slice_id: &str) -> Result<()> {
    let branch = format!("mz/{}/{}", milestone_id, slice_id);
    let output = Command::new("git")
        .args(["checkout", "-b", &branch])
        .output()
        .context("Failed to create branch")?;
    if !output.status.success() {
        // Branch might already exist
        let output = Command::new("git")
            .args(["checkout", &branch])
            .output()
            .context("Failed to checkout branch")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to checkout branch {}: {}", branch, stderr);
        }
    }
    Ok(())
}

pub fn commit_task(milestone_id: &str, slice_id: &str, task_id: &str, title: &str) -> Result<()> {
    // Stage all changes
    Command::new("git")
        .args(["add", "-A"])
        .output()
        .context("Failed to git add")?;

    // Check if there are changes to commit
    let status = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .output()?;

    if status.status.success() {
        // No changes staged
        return Ok(());
    }

    let message = format!("{}/{}/{}: {}", milestone_id, slice_id, task_id, title);
    Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .context("Failed to git commit")?;

    Ok(())
}

pub fn merge_slice(milestone_id: &str, slice_id: &str) -> Result<()> {
    let branch = format!("mz/{}/{}", milestone_id, slice_id);

    // Switch to main
    Command::new("git")
        .args(["checkout", "main"])
        .output()
        .context("Failed to checkout main")?;

    // Squash merge
    let output = Command::new("git")
        .args(["merge", "--squash", &branch])
        .output()
        .context("Failed to squash merge")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Merge failed: {}", stderr);
    }

    let message = format!("feat({}/{}): slice complete", milestone_id, slice_id);
    Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .context("Failed to commit merge")?;

    Ok(())
}
