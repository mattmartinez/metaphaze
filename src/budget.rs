use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::run_record::{self, RunRecord};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BudgetConfig {
    /// Maximum USD spend allowed. None = unlimited.
    pub max_usd: Option<f64>,
}

pub fn config_path() -> PathBuf {
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
