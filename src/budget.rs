use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::run_record::{self, RunRecord};

#[cfg(test)]
thread_local! {
    static TEST_MZ_DIR: std::cell::RefCell<Option<PathBuf>> = std::cell::RefCell::new(None);
}

#[cfg(test)]
fn set_test_mz_dir(path: Option<PathBuf>) {
    TEST_MZ_DIR.with(|d| *d.borrow_mut() = path);
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BudgetConfig {
    /// Maximum USD spend allowed. None = unlimited.
    pub max_usd: Option<f64>,
}

pub fn config_path() -> PathBuf {
    #[cfg(test)]
    {
        let override_dir = TEST_MZ_DIR.with(|d| d.borrow().clone());
        if let Some(dir) = override_dir {
            return dir.join("budget.yaml");
        }
    }
    PathBuf::from(".mz/budget.yaml")
}

pub fn load() -> Result<BudgetConfig> {
    let path = config_path();
    if !path.exists() {
        return Ok(BudgetConfig::default());
    }
    let contents = std::fs::read_to_string(&path)?;
    let config: BudgetConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub fn save(config: &BudgetConfig) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let contents = serde_yaml::to_string(config)?;
    std::fs::write(&path, contents)?;
    Ok(())
}

pub struct BudgetStatus {
    pub spent: f64,
    pub limit: Option<f64>,
    pub remaining: Option<f64>,
    pub exhausted: bool,
}

pub fn check(config: &BudgetConfig, records: &[RunRecord]) -> BudgetStatus {
    let spent = run_record::total_project_cost(records);
    match config.max_usd {
        None => BudgetStatus {
            spent,
            limit: None,
            remaining: None,
            exhausted: false,
        },
        Some(limit) => {
            let remaining = (limit - spent).max(0.0);
            BudgetStatus {
                spent,
                limit: Some(limit),
                remaining: Some(remaining),
                exhausted: spent >= limit,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    struct TempMz {
        _dir: TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            set_test_mz_dir(Some(dir.path().to_path_buf()));
            TempMz { _dir: dir }
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            set_test_mz_dir(None);
        }
    }

    fn make_record(cost_usd: Option<f64>) -> RunRecord {
        RunRecord {
            id: Uuid::new_v4().to_string(),
            phase_id: "P001".to_string(),
            track_id: "TR01".to_string(),
            step_id: "ST01".to_string(),
            stage: "execute".to_string(),
            model: "claude-sonnet-4-6".to_string(),
            started_at: "2026-04-09T00:00:00Z".to_string(),
            finished_at: "2026-04-09T00:00:01Z".to_string(),
            duration_ms: 1000,
            cost_usd,
            num_turns: Some(1),
            outcome: "success".to_string(),
            error: None,
            input_tokens: None,
            output_tokens: None,
        }
    }

    #[test]
    fn test_load_missing_file_returns_default() {
        let _mz = TempMz::new();
        let config = load().unwrap();
        assert!(config.max_usd.is_none());
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let _mz = TempMz::new();
        let config = BudgetConfig { max_usd: Some(42.5) };
        save(&config).unwrap();
        let loaded = load().unwrap();
        assert_eq!(loaded.max_usd, Some(42.5));
    }

    #[test]
    fn test_check_no_budget() {
        let config = BudgetConfig { max_usd: None };
        let records = vec![make_record(Some(3.0))];
        let status = check(&config, &records);
        assert!(!status.exhausted);
        assert!(status.remaining.is_none());
    }

    #[test]
    fn test_check_under_budget() {
        let config = BudgetConfig { max_usd: Some(10.0) };
        let records = vec![make_record(Some(3.0))];
        let status = check(&config, &records);
        assert!(!status.exhausted);
        assert!((status.remaining.unwrap() - 7.0).abs() < 1e-9);
    }

    #[test]
    fn test_check_over_budget() {
        let config = BudgetConfig { max_usd: Some(5.0) };
        let records = vec![make_record(Some(6.0))];
        let status = check(&config, &records);
        assert!(status.exhausted);
        // remaining clamps to 0.0 (max(0, limit - spent))
        assert_eq!(status.remaining, Some(0.0));
    }

    #[test]
    fn test_check_exactly_at_budget() {
        let config = BudgetConfig { max_usd: Some(5.0) };
        let records = vec![make_record(Some(5.0))];
        let status = check(&config, &records);
        assert!(status.exhausted);
        assert_eq!(status.remaining, Some(0.0));
    }
}
