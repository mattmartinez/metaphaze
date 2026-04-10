use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[cfg(test)]
thread_local! {
    static TEST_MZ_DIR: std::cell::RefCell<Option<PathBuf>> = std::cell::RefCell::new(None);
}

#[cfg(test)]
pub(crate) fn set_test_mz_dir(path: Option<PathBuf>) {
    TEST_MZ_DIR.with(|d| *d.borrow_mut() = path);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunRecord {
    pub id: String,
    pub phase_id: String,
    pub track_id: String,
    pub step_id: String,
    pub stage: String,
    pub model: String,
    pub started_at: String,
    pub finished_at: String,
    pub duration_ms: u64,
    pub cost_usd: Option<f64>,
    pub num_turns: Option<u32>,
    pub outcome: String,
    pub error: Option<String>,
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens: Option<u64>,
}

pub fn ledger_path() -> PathBuf {
    #[cfg(test)]
    {
        let override_dir = TEST_MZ_DIR.with(|d| d.borrow().clone());
        if let Some(dir) = override_dir {
            return dir.join("runs.jsonl");
        }
    }
    PathBuf::from(".mz/runs.jsonl")
}

pub fn append(record: &RunRecord) -> Result<()> {
    let path = ledger_path();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)?;
    let line = serde_json::to_string(record)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

pub struct PhaseSummary {
    pub phase_id: String,
    pub runs: usize,
    pub ok: usize,
    pub err: usize,
    pub cost_usd: f64,
    pub duration_ms: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

pub struct TrackSummary {
    pub phase_id: String,
    pub track_id: String,
    pub steps: usize,
    pub runs: usize,
    pub cost_usd: f64,
    pub duration_ms: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

pub fn phase_summaries(records: &[RunRecord]) -> Vec<PhaseSummary> {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, PhaseSummary> = BTreeMap::new();
    for r in records {
        let entry = map.entry(r.phase_id.clone()).or_insert(PhaseSummary {
            phase_id: r.phase_id.clone(),
            runs: 0,
            ok: 0,
            err: 0,
            cost_usd: 0.0,
            duration_ms: 0,
            input_tokens: 0,
            output_tokens: 0,
        });
        entry.runs += 1;
        if r.outcome == "error" {
            entry.err += 1;
        } else {
            entry.ok += 1;
        }
        entry.cost_usd += r.cost_usd.unwrap_or(0.0);
        entry.duration_ms += r.duration_ms;
        entry.input_tokens += r.input_tokens.unwrap_or(0);
        entry.output_tokens += r.output_tokens.unwrap_or(0);
    }
    map.into_values().collect()
}

pub fn track_summaries(records: &[RunRecord]) -> Vec<TrackSummary> {
    use std::collections::{BTreeMap, BTreeSet};
    let mut runs_map: BTreeMap<(String, String), TrackSummary> = BTreeMap::new();
    let mut steps_map: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
    for r in records {
        let key = (r.phase_id.clone(), r.track_id.clone());
        let entry = runs_map.entry(key.clone()).or_insert(TrackSummary {
            phase_id: r.phase_id.clone(),
            track_id: r.track_id.clone(),
            steps: 0,
            runs: 0,
            cost_usd: 0.0,
            duration_ms: 0,
            input_tokens: 0,
            output_tokens: 0,
        });
        entry.runs += 1;
        entry.cost_usd += r.cost_usd.unwrap_or(0.0);
        entry.duration_ms += r.duration_ms;
        entry.input_tokens += r.input_tokens.unwrap_or(0);
        entry.output_tokens += r.output_tokens.unwrap_or(0);
        steps_map.entry(key).or_default().insert(r.step_id.clone());
    }
    for (key, summary) in runs_map.iter_mut() {
        if let Some(steps) = steps_map.get(key) {
            summary.steps = steps.len();
        }
    }
    runs_map.into_values().collect()
}

pub fn total_project_cost(records: &[RunRecord]) -> f64 {
    records.iter().filter_map(|r| r.cost_usd).sum()
}

pub fn load_all() -> Result<Vec<RunRecord>> {
    let path = ledger_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let file = std::fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let record: RunRecord = serde_json::from_str(&line)?;
        records.push(record);
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    /// RAII guard: sets the thread-local mz dir override for the duration of the test.
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

    fn make_record() -> RunRecord {
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
            cost_usd: Some(0.01),
            num_turns: Some(3),
            outcome: "success".to_string(),
            error: None,
            input_tokens: None,
            output_tokens: None,
        }
    }

    #[test]
    fn test_roundtrip_serialization() {
        let record = make_record();
        let json = serde_json::to_string(&record).unwrap();
        let parsed: RunRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record.id, parsed.id);
        assert_eq!(record.phase_id, parsed.phase_id);
        assert_eq!(record.track_id, parsed.track_id);
        assert_eq!(record.step_id, parsed.step_id);
        assert_eq!(record.stage, parsed.stage);
        assert_eq!(record.model, parsed.model);
        assert_eq!(record.started_at, parsed.started_at);
        assert_eq!(record.finished_at, parsed.finished_at);
        assert_eq!(record.duration_ms, parsed.duration_ms);
        assert_eq!(record.cost_usd, parsed.cost_usd);
        assert_eq!(record.num_turns, parsed.num_turns);
        assert_eq!(record.outcome, parsed.outcome);
        assert_eq!(record.error, parsed.error);
    }

    #[test]
    fn test_append_and_load() {
        let _mz = TempMz::new();

        let r1 = RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P001".to_string(), track_id: "TR01".to_string(), step_id: "ST01".to_string(), stage: "execute".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:00Z".to_string(), finished_at: "2026-01-01T00:00:01Z".to_string(), duration_ms: 100, cost_usd: Some(0.01), num_turns: Some(1), outcome: "success".to_string(), error: None, input_tokens: None, output_tokens: None };
        let r2 = RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P001".to_string(), track_id: "TR01".to_string(), step_id: "ST02".to_string(), stage: "verify".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:02Z".to_string(), finished_at: "2026-01-01T00:00:03Z".to_string(), duration_ms: 200, cost_usd: Some(0.02), num_turns: Some(2), outcome: "success".to_string(), error: None, input_tokens: None, output_tokens: None };
        let r3 = RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P002".to_string(), track_id: "TR02".to_string(), step_id: "ST01".to_string(), stage: "execute".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:04Z".to_string(), finished_at: "2026-01-01T00:00:05Z".to_string(), duration_ms: 300, cost_usd: None, num_turns: None, outcome: "error".to_string(), error: Some("boom".to_string()), input_tokens: None, output_tokens: None };

        append(&r1).unwrap();
        append(&r2).unwrap();
        append(&r3).unwrap();

        let records = load_all().unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].step_id, "ST01");
        assert_eq!(records[0].phase_id, "P001");
        assert_eq!(records[1].step_id, "ST02");
        assert_eq!(records[2].phase_id, "P002");
        assert_eq!(records[2].outcome, "error");
        assert_eq!(records[2].error, Some("boom".to_string()));
        assert_eq!(records[2].cost_usd, None);
        assert_eq!(records[2].num_turns, None);
    }

    #[test]
    fn test_load_empty_file() {
        let _mz = TempMz::new();
        // ledger_path() returns inside the temp dir, which doesn't have runs.jsonl yet
        let records = load_all().unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn test_load_with_blank_lines() {
        let _mz = TempMz::new();
        let path = ledger_path();

        let record = make_record();
        let json = serde_json::to_string(&record).unwrap();
        // Write blank lines interspersed
        std::fs::write(&path, format!("\n{}\n\n{}\n\n", json, json)).unwrap();

        let records = load_all().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].id, record.id);
    }

    #[test]
    fn test_optional_fields() {
        let record = RunRecord {
            id: Uuid::new_v4().to_string(),
            phase_id: "P001".to_string(),
            track_id: "TR01".to_string(),
            step_id: "ST01".to_string(),
            stage: "execute".to_string(),
            model: "m".to_string(),
            started_at: "2026-01-01T00:00:00Z".to_string(),
            finished_at: "2026-01-01T00:00:01Z".to_string(),
            duration_ms: 500,
            cost_usd: None,
            num_turns: None,
            outcome: "success".to_string(),
            error: None,
            input_tokens: None,
            output_tokens: None,
        };
        let json = serde_json::to_string(&record).unwrap();
        let parsed: RunRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.cost_usd, None);
        assert_eq!(parsed.num_turns, None);
        assert_eq!(parsed.error, None);
        assert_eq!(parsed.duration_ms, 500);
    }

    #[test]
    fn test_token_fields_roundtrip() {
        let record = RunRecord {
            id: Uuid::new_v4().to_string(),
            phase_id: "P001".to_string(),
            track_id: "TR01".to_string(),
            step_id: "ST01".to_string(),
            stage: "execute".to_string(),
            model: "m".to_string(),
            started_at: "2026-01-01T00:00:00Z".to_string(),
            finished_at: "2026-01-01T00:00:01Z".to_string(),
            duration_ms: 100,
            cost_usd: Some(0.05),
            num_turns: Some(2),
            outcome: "success".to_string(),
            error: None,
            input_tokens: Some(1000),
            output_tokens: Some(500),
        };
        let json = serde_json::to_string(&record).unwrap();
        let parsed: RunRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.input_tokens, Some(1000));
        assert_eq!(parsed.output_tokens, Some(500));
    }

    #[test]
    fn test_token_fields_backward_compat() {
        // JSON without token fields should deserialize to None via #[serde(default)]
        let json = r#"{"id":"abc","phase_id":"P001","track_id":"TR01","step_id":"ST01","stage":"execute","model":"m","started_at":"2026-01-01T00:00:00Z","finished_at":"2026-01-01T00:00:01Z","duration_ms":100,"cost_usd":0.01,"num_turns":1,"outcome":"success","error":null}"#;
        let parsed: RunRecord = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.input_tokens, None);
        assert_eq!(parsed.output_tokens, None);
    }

    #[test]
    fn test_phase_summary_aggregates_tokens() {
        let records = vec![
            RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P001".to_string(), track_id: "TR01".to_string(), step_id: "ST01".to_string(), stage: "execute".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:00Z".to_string(), finished_at: "2026-01-01T00:00:01Z".to_string(), duration_ms: 100, cost_usd: Some(0.01), num_turns: Some(1), outcome: "success".to_string(), error: None, input_tokens: Some(1000), output_tokens: Some(200) },
            RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P001".to_string(), track_id: "TR01".to_string(), step_id: "ST02".to_string(), stage: "verify".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:02Z".to_string(), finished_at: "2026-01-01T00:00:03Z".to_string(), duration_ms: 200, cost_usd: Some(0.02), num_turns: Some(1), outcome: "success".to_string(), error: None, input_tokens: Some(500), output_tokens: Some(100) },
            RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P002".to_string(), track_id: "TR01".to_string(), step_id: "ST01".to_string(), stage: "execute".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:04Z".to_string(), finished_at: "2026-01-01T00:00:05Z".to_string(), duration_ms: 300, cost_usd: Some(0.03), num_turns: Some(1), outcome: "success".to_string(), error: None, input_tokens: None, output_tokens: None },
        ];
        let summaries = phase_summaries(&records);
        assert_eq!(summaries.len(), 2);
        let p1 = summaries.iter().find(|s| s.phase_id == "P001").unwrap();
        assert_eq!(p1.input_tokens, 1500);
        assert_eq!(p1.output_tokens, 300);
        let p2 = summaries.iter().find(|s| s.phase_id == "P002").unwrap();
        assert_eq!(p2.input_tokens, 0);
        assert_eq!(p2.output_tokens, 0);
    }

    #[test]
    fn test_total_project_cost() {
        let records = vec![
            RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P001".to_string(), track_id: "TR01".to_string(), step_id: "ST01".to_string(), stage: "execute".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:00Z".to_string(), finished_at: "2026-01-01T00:00:01Z".to_string(), duration_ms: 100, cost_usd: Some(1.50), num_turns: Some(1), outcome: "success".to_string(), error: None, input_tokens: None, output_tokens: None },
            RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P001".to_string(), track_id: "TR01".to_string(), step_id: "ST02".to_string(), stage: "verify".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:02Z".to_string(), finished_at: "2026-01-01T00:00:03Z".to_string(), duration_ms: 200, cost_usd: Some(2.25), num_turns: Some(1), outcome: "success".to_string(), error: None, input_tokens: None, output_tokens: None },
            RunRecord { id: Uuid::new_v4().to_string(), phase_id: "P002".to_string(), track_id: "TR01".to_string(), step_id: "ST01".to_string(), stage: "execute".to_string(), model: "m".to_string(), started_at: "2026-01-01T00:00:04Z".to_string(), finished_at: "2026-01-01T00:00:05Z".to_string(), duration_ms: 300, cost_usd: None, num_turns: None, outcome: "error".to_string(), error: Some("boom".to_string()), input_tokens: None, output_tokens: None },
        ];
        let total = total_project_cost(&records);
        assert!((total - 3.75).abs() < 1e-9);
    }
}
