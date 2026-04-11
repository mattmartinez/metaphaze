//! Project configuration loaded from `.mz/config.yaml`.
//!
//! All keys are optional. Missing keys fall back to built-in defaults provided
//! by the `Default` impls below. Callers should always be able to call
//! `config::load()` and get a usable `Config` even when the file does not
//! exist — the orchestrator must work in projects that predate this file.
//!
//! Lookup precedence for any tunable: explicit CLI flag → `.mz/config.yaml`
//! → built-in default. The CLI plumbing for that precedence lives in the
//! call sites (e.g. `cmd_auto` in `main.rs` or `scheduler::run`); this module
//! only provides the middle layer.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::state;

/// Top-level config struct backing `.mz/config.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub models: ModelConfig,
    #[serde(default)]
    pub budget: BudgetSection,
    #[serde(default)]
    pub retry: RetryConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub theme: ThemeConfig,
}

/// Per-stage Claude model selection. Values are model aliases that
/// `claude::ClaudeOptions::model()` accepts: `"opus" | "sonnet" | "haiku"`
/// (or any model id Claude Code recognizes).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ModelConfig {
    pub execute: String,
    pub summarize: String,
    pub plan_track: String,
    pub verify_step: String,
    pub verify_track: String,
    pub plan_phase: String,
    pub plan_roadmap: String,
    pub replan: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            execute: "sonnet".into(),
            summarize: "sonnet".into(),
            plan_track: "opus".into(),
            verify_step: "sonnet".into(),
            verify_track: "opus".into(),
            plan_phase: "opus".into(),
            plan_roadmap: "opus".into(),
            replan: "opus".into(),
        }
    }
}

/// Budget defaults. The legacy `.mz/budget.yaml` (managed by `mz budget set`)
/// always wins; this section only kicks in when budget.yaml is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BudgetSection {
    /// Default project-wide cap in USD. `null` (None) = unlimited.
    pub default_max_usd: Option<f64>,
}

/// Retry caps for the autonomous loop. Once a step's per-stage retry counter
/// hits the cap it gets marked blocked instead of being retried again.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RetryConfig {
    pub max_executor_attempts: u32,
    pub max_verifier_attempts: u32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        // Mirrors the historical hard-coded values in `scheduler::run`.
        Self {
            max_executor_attempts: 3,
            max_verifier_attempts: 2,
        }
    }
}

/// Git workflow knobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitConfig {
    /// `"squash"` (the historical default) or `"no-ff"` for a true merge commit.
    pub merge_strategy: String,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            merge_strategy: "squash".into(),
        }
    }
}

/// Distinct merge strategies the git layer understands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    Squash,
    NoFf,
}

impl GitConfig {
    /// Parse the configured strategy. Unknown values silently fall back to
    /// `Squash` so a typo in config.yaml never bricks the orchestrator.
    pub fn strategy(&self) -> MergeStrategy {
        match self.merge_strategy.as_str() {
            "no-ff" | "noff" | "merge" => MergeStrategy::NoFf,
            _ => MergeStrategy::Squash,
        }
    }
}

/// TUI theme. Currently a single accent-color knob; expand as the TUI grows
/// more chrome.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    /// Theme preset name. Known values: `"default"`, `"high-contrast"`.
    pub name: String,
    /// Override accent color (used for the interactive prompt cursor and
    /// other highlights). Recognized: `cyan | magenta | yellow | green | blue | red | white`.
    pub accent: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".into(),
            accent: "cyan".into(),
        }
    }
}

/// Path to `.mz/config.yaml`. Honors the test override on `state::mz_root()`
/// so unit tests can point this at a tempdir.
pub fn config_path() -> PathBuf {
    state::mz_root().join("config.yaml")
}

/// Load `.mz/config.yaml`. Returns `Config::default()` if the file is missing
/// so callers can unconditionally call `load()` without `if path.exists()`
/// pre-checks.
pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = std::fs::read_to_string(&path)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

/// Cached process-wide config. Most call sites read config once per step and
/// don't need a global cache, so this is currently just a thin wrapper around
/// `load()`. Kept as a named function so future caching can be added in one
/// place if it ever becomes a hotspot.
pub fn current() -> Config {
    load().unwrap_or_default()
}

/// Write a freshly initialized `.mz/config.yaml` containing every field with
/// its built-in default value. The file is heavily commented so users can
/// discover what they can tune without reading the source. Called by
/// `mz init` immediately after the rest of the `.mz/` skeleton is created.
pub fn write_default_commented() -> Result<()> {
    let path = config_path();
    if path.exists() {
        // Don't clobber a user-edited file.
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let body = r#"# .mz/config.yaml — project defaults for the metaphaze orchestrator.
#
# Every key is optional. Delete any line to fall back to the built-in default.
# CLI flags (e.g. --max-budget-usd) always override values in this file.

# ---------------------------------------------------------------------------
# Per-stage Claude model selection.
# Valid values: opus | sonnet | haiku (or any model id `claude` recognizes).
# Heavy reasoning stages default to opus; routine code edits default to sonnet.
models:
  execute:       sonnet   # per-step implementation
  summarize:    sonnet   # post-execution write-up of what was done
  plan_track:   opus     # track-level plan enrichment (first step of a track)
  verify_step:  sonnet   # per-step verification pass
  verify_track: opus     # end-to-end track verification
  plan_phase:   opus     # `mz plan` — phase decomposition
  plan_roadmap: opus     # multi-phase roadmap generation
  replan:       opus     # `mz steer` — re-plan after a decision change

# ---------------------------------------------------------------------------
# Budget defaults. `mz budget set` writes to .mz/budget.yaml which always
# overrides this section; this is just the bootstrapped fallback for new
# projects. `null` means unlimited.
budget:
  default_max_usd: null

# ---------------------------------------------------------------------------
# Retry caps for the autonomous loop. When a step has been retried this many
# times for a given stage, the scheduler stops retrying and marks it blocked
# so a human (or `mz reset`) can intervene.
retry:
  max_executor_attempts: 3
  max_verifier_attempts: 2

# ---------------------------------------------------------------------------
# Git workflow.
# merge_strategy: how `merge_track` integrates a finished track branch back
#   into the default branch.
#     squash → single commit on the default branch (clean linear history)
#     no-ff  → real merge commit preserving each step commit
git:
  merge_strategy: squash

# ---------------------------------------------------------------------------
# TUI theme. Minimal for now — accent drives highlights like the interactive
# prompt cursor.
theme:
  name:   default
  accent: cyan
"#;
    std::fs::write(&path, body)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    struct TempMz {
        _dir: TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let mz = dir.path().join(".mz");
            std::fs::create_dir_all(&mz).unwrap();
            crate::state::set_test_mz_dir(Some(mz));
            TempMz { _dir: dir }
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            crate::state::set_test_mz_dir(None);
        }
    }

    #[test]
    fn test_load_missing_returns_default() {
        let _lock = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _mz = TempMz::new();
        let cfg = load().unwrap();
        // Default model assignments
        assert_eq!(cfg.models.execute, "sonnet");
        assert_eq!(cfg.models.plan_phase, "opus");
        // Default retry caps mirror historical hardcoded values
        assert_eq!(cfg.retry.max_executor_attempts, 3);
        assert_eq!(cfg.retry.max_verifier_attempts, 2);
        // Default git strategy
        assert_eq!(cfg.git.strategy(), MergeStrategy::Squash);
        // Default theme
        assert_eq!(cfg.theme.name, "default");
        assert!(cfg.budget.default_max_usd.is_none());
    }

    #[test]
    fn test_load_full_yaml_overrides_all_fields() {
        let _lock = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _mz = TempMz::new();
        let yaml = r#"
models:
  execute: opus
  summarize: haiku
  plan_track: sonnet
  verify_step: opus
  verify_track: sonnet
  plan_phase: sonnet
  plan_roadmap: sonnet
  replan: sonnet
budget:
  default_max_usd: 25.0
retry:
  max_executor_attempts: 5
  max_verifier_attempts: 4
git:
  merge_strategy: no-ff
theme:
  name: high-contrast
  accent: magenta
"#;
        std::fs::write(config_path(), yaml).unwrap();
        let cfg = load().unwrap();
        assert_eq!(cfg.models.execute, "opus");
        assert_eq!(cfg.models.summarize, "haiku");
        assert_eq!(cfg.budget.default_max_usd, Some(25.0));
        assert_eq!(cfg.retry.max_executor_attempts, 5);
        assert_eq!(cfg.retry.max_verifier_attempts, 4);
        assert_eq!(cfg.git.strategy(), MergeStrategy::NoFf);
        assert_eq!(cfg.theme.name, "high-contrast");
        assert_eq!(cfg.theme.accent, "magenta");
    }

    #[test]
    fn test_load_partial_yaml_keeps_other_defaults() {
        let _lock = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _mz = TempMz::new();
        // Only override one field; everything else should remain default.
        let yaml = "retry:\n  max_executor_attempts: 7\n";
        std::fs::write(config_path(), yaml).unwrap();
        let cfg = load().unwrap();
        assert_eq!(cfg.retry.max_executor_attempts, 7);
        // Defaults preserved
        assert_eq!(cfg.retry.max_verifier_attempts, 2);
        assert_eq!(cfg.models.execute, "sonnet");
        assert_eq!(cfg.git.strategy(), MergeStrategy::Squash);
    }

    #[test]
    fn test_unknown_merge_strategy_falls_back_to_squash() {
        let cfg = GitConfig { merge_strategy: "absolutely-not-a-strategy".into() };
        assert_eq!(cfg.strategy(), MergeStrategy::Squash);
    }

    #[test]
    fn test_merge_strategy_aliases() {
        assert_eq!(GitConfig { merge_strategy: "squash".into() }.strategy(), MergeStrategy::Squash);
        assert_eq!(GitConfig { merge_strategy: "no-ff".into() }.strategy(), MergeStrategy::NoFf);
        assert_eq!(GitConfig { merge_strategy: "noff".into() }.strategy(), MergeStrategy::NoFf);
        assert_eq!(GitConfig { merge_strategy: "merge".into() }.strategy(), MergeStrategy::NoFf);
    }

    #[test]
    fn test_write_default_commented_creates_loadable_file() {
        let _lock = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _mz = TempMz::new();
        write_default_commented().unwrap();
        assert!(config_path().exists());
        // The commented file must round-trip through the loader successfully
        // (i.e. the comments don't break YAML parsing and every uncommented
        // value matches the typed Default).
        let cfg = load().unwrap();
        assert_eq!(cfg.models.execute, "sonnet");
        assert_eq!(cfg.models.plan_phase, "opus");
        assert_eq!(cfg.retry.max_executor_attempts, 3);
        assert_eq!(cfg.git.strategy(), MergeStrategy::Squash);
    }

    #[test]
    fn test_write_default_does_not_clobber_existing() {
        let _lock = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _mz = TempMz::new();
        std::fs::write(config_path(), "models:\n  execute: opus\n").unwrap();
        write_default_commented().unwrap();
        // Existing file should be preserved verbatim
        let body = std::fs::read_to_string(config_path()).unwrap();
        assert!(body.contains("execute: opus"));
        assert!(!body.contains("# .mz/config.yaml — project defaults"));
    }

    #[test]
    fn test_current_returns_default_on_missing_file() {
        let _lock = crate::state::TEST_PROCESS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _mz = TempMz::new();
        let cfg = current();
        assert_eq!(cfg.models.execute, "sonnet");
    }
}
