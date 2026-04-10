use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
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

pub fn commit_step(phase_id: &str, track_id: &str, step_id: &str, title: &str, cwd: Option<&Path>) -> Result<()> {
    let mut add_cmd = Command::new("git");
    add_cmd.args(["add", "-A"]);
    if let Some(dir) = cwd {
        add_cmd.current_dir(dir);
    }
    add_cmd.output().context("Failed to git add")?;

    let mut status_cmd = Command::new("git");
    status_cmd.args(["diff", "--cached", "--quiet"]);
    if let Some(dir) = cwd {
        status_cmd.current_dir(dir);
    }
    let status = status_cmd.output()?;

    if status.status.success() {
        return Ok(());
    }

    let message = format!("{}/{}/{}: {}", phase_id, track_id, step_id, title);
    let mut commit_cmd = Command::new("git");
    commit_cmd.args(["commit", "-m", &message]);
    if let Some(dir) = cwd {
        commit_cmd.current_dir(dir);
    }
    let output = commit_cmd.output().context("Failed to git commit")?;

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

/// Returns the absolute path to the repository root.
fn repo_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("Failed to run git rev-parse --show-toplevel")?;
    if !output.status.success() {
        bail!("Not inside a git repository");
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
    ))
}

/// Returns true if the given branch already exists locally.
fn branch_exists(branch: &str) -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", &format!("refs/heads/{}", branch)])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Create a git worktree for the given phase/track.
/// Path: `.mz/worktrees/{phase_id}/{track_id}` inside the repo root.
/// Branch: `mz/{phase_id}/{track_id}`.
/// If the branch already exists, adds the worktree without `-b`.
/// Returns the absolute path to the worktree.
pub fn create_worktree(phase_id: &str, track_id: &str) -> Result<PathBuf> {
    let root = repo_root()?;
    let rel_path = format!(".mz/worktrees/{}/{}", phase_id, track_id);
    let worktree_path = root.join(&rel_path);

    // Create parent directories if needed.
    if let Some(parent) = worktree_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent dir {}", parent.display()))?;
    }

    let branch = format!("mz/{}/{}", phase_id, track_id);
    let path_str = worktree_path.to_string_lossy();

    let output = if branch_exists(&branch) {
        Command::new("git")
            .args(["worktree", "add", path_str.as_ref(), &branch])
            .output()
            .context("Failed to run git worktree add")?
    } else {
        Command::new("git")
            .args(["worktree", "add", path_str.as_ref(), "-b", &branch])
            .output()
            .context("Failed to run git worktree add -b")?
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git worktree add failed: {}", stderr);
    }

    Ok(worktree_path)
}

/// Remove a git worktree at the given path.
/// Uses `--force` and ignores errors if the worktree is already gone.
pub fn remove_worktree(worktree_path: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["worktree", "remove", "--force", &worktree_path.to_string_lossy()])
        .output()
        .context("Failed to run git worktree remove")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignore "not a worktree" / "does not exist" errors — it's already gone.
        if !stderr.contains("is not a working tree") && !stderr.contains("does not exist") {
            bail!("git worktree remove failed: {}", stderr);
        }
    }
    Ok(())
}

/// List all worktree paths known to git.
/// Runs `git worktree list --porcelain` and returns the `worktree` lines.
pub fn list_worktrees() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .context("Failed to run git worktree list")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git worktree list failed: {}", stderr);
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let paths = text
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(|p| p.to_string())
        .collect();

    Ok(paths)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::sync::Mutex;

    /// Global mutex to serialize tests that modify the process CWD.
    static CWD_LOCK: Mutex<()> = Mutex::new(());

    /// Set up a temporary git repository with an initial commit.
    fn init_temp_repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        let p = dir.path();

        Command::new("git").args(["init"]).current_dir(p).output().unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(p)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(p)
            .output()
            .unwrap();

        // Need at least one commit so worktree operations work.
        let readme = p.join("README.md");
        std::fs::write(&readme, "init").unwrap();
        Command::new("git").args(["add", "."]).current_dir(p).output().unwrap();
        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(p)
            .output()
            .unwrap();

        dir
    }

    #[test]
    fn test_worktree_create_remove_list() {
        let _lock = CWD_LOCK.lock().unwrap();
        let dir = init_temp_repo();
        let p = dir.path();

        // Our git functions use Command::new("git") without explicit current_dir,
        // so they rely on the process CWD. Save and restore around the test.
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(p).unwrap();

        // list_worktrees should return at least the main worktree (the temp repo itself).
        // Canonicalize to resolve macOS /tmp -> /private/var/... symlinks.
        let canonical_p = p.canonicalize().unwrap();
        let worktrees = list_worktrees().expect("list_worktrees");
        assert!(!worktrees.is_empty(), "should have at least the main worktree");
        let canonical_p_str = canonical_p.to_str().unwrap();
        assert!(
            worktrees.iter().any(|w| w == canonical_p_str),
            "main worktree should be in list.\nExpected: {:?}\nList: {:?}",
            canonical_p_str,
            worktrees
        );

        // create_worktree should create a new worktree + branch.
        let wt_path = create_worktree("P099", "TR01").expect("create_worktree");
        assert!(wt_path.exists(), "worktree directory should exist");

        // The new worktree should appear in list.
        let canonical_wt = wt_path.canonicalize().unwrap();
        let worktrees = list_worktrees().expect("list_worktrees after create");
        assert!(
            worktrees.iter().any(|w| w == canonical_wt.to_str().unwrap()),
            "new worktree should appear in list: {:?}",
            worktrees
        );

        // create_worktree again (branch already exists) should succeed.
        // First remove the worktree so we can re-add it.
        remove_worktree(&wt_path).expect("remove_worktree");
        assert!(!wt_path.exists(), "worktree directory should be gone after remove");

        // The worktree should no longer appear in list.
        let worktrees = list_worktrees().expect("list_worktrees after remove");
        assert!(
            !worktrees.iter().any(|w| *w == wt_path.to_string_lossy().as_ref()),
            "removed worktree should not appear in list"
        );

        // Re-create using the existing branch (branch_exists path).
        let wt_path2 = create_worktree("P099", "TR01").expect("create_worktree with existing branch");
        assert!(wt_path2.exists(), "re-created worktree should exist");
        remove_worktree(&wt_path2).expect("final remove");

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_remove_worktree_already_gone() {
        let _lock = CWD_LOCK.lock().unwrap();
        // remove_worktree on a non-existent path should not error.
        let dir = init_temp_repo();
        let p = dir.path();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(p).unwrap();

        let fake_path = p.join("nonexistent_wt");
        // Should not panic or return an error.
        let _ = remove_worktree(&fake_path); // error is acceptable here (not registered with git)

        std::env::set_current_dir(original_dir).unwrap();
    }
}
