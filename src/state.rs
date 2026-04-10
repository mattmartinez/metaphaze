use anyhow::{bail, Context, Result};
use chrono::Utc;
use colored::Colorize;
use dialoguer::Input;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

static MZ_ROOT_CACHE: OnceLock<PathBuf> = OnceLock::new();

#[cfg(test)]
thread_local! {
    static TEST_MZ_DIR: std::cell::RefCell<Option<PathBuf>> = std::cell::RefCell::new(None);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepEntry {
    pub id: String,
    pub title: String,
    pub status: StepStatus,
    pub blocked_reason: Option<String>,
    #[serde(default)]
    pub attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StepStatus {
    Pending,
    InProgress,
    Complete,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackEntry {
    pub id: String,
    pub title: String,
    pub steps: Vec<StepEntry>,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseEntry {
    pub id: String,
    pub title: String,
    pub tracks: Vec<TrackEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub name: String,
    pub description: String,
    pub current_phase: String,
    pub phases: Vec<PhaseEntry>,
}

impl ProjectState {
    pub fn current_phase(&self) -> &str {
        &self.current_phase
    }

    pub fn next_pending_step(&self) -> Option<(String, String, String)> {
        // First pass: return any in-progress step (crash recovery)
        for ph in &self.phases {
            if ph.id != self.current_phase {
                continue;
            }
            for track in &ph.tracks {
                for step in &track.steps {
                    if step.status == StepStatus::InProgress {
                        return Some((ph.id.clone(), track.id.clone(), step.id.clone()));
                    }
                }
            }
        }
        // Second pass: return next pending step, respecting track dependencies (BUG-9 fix)
        for ph in &self.phases {
            if ph.id != self.current_phase {
                continue;
            }
            for track in &ph.tracks {
                // Skip tracks whose dependencies aren't all complete
                if !self.track_deps_satisfied(ph, track) {
                    continue;
                }
                for step in &track.steps {
                    if step.status == StepStatus::Pending {
                        return Some((ph.id.clone(), track.id.clone(), step.id.clone()));
                    }
                }
            }
        }
        None
    }

    pub fn step_attempts(&self, phase_id: &str, track_id: &str, step_id: &str) -> u32 {
        for ph in &self.phases {
            if ph.id != phase_id {
                continue;
            }
            for track in &ph.tracks {
                if track.id != track_id {
                    continue;
                }
                for step in &track.steps {
                    if step.id == step_id {
                        return step.attempts;
                    }
                }
            }
        }
        0
    }

    fn track_deps_satisfied(&self, phase: &PhaseEntry, track: &TrackEntry) -> bool {
        for dep_id in &track.depends_on {
            let dep_complete = phase.tracks.iter().any(|t| {
                t.id == *dep_id && t.steps.iter().all(|s| s.status == StepStatus::Complete)
            });
            if !dep_complete {
                return false;
            }
        }
        true
    }

    pub fn is_track_complete(&self, phase_id: &str, track_id: &str) -> bool {
        for ph in &self.phases {
            if ph.id != phase_id {
                continue;
            }
            for track in &ph.tracks {
                if track.id != track_id {
                    continue;
                }
                return track.steps.iter().all(|s| s.status == StepStatus::Complete);
            }
        }
        false
    }

    pub fn is_phase_complete(&self, phase_id: &str) -> bool {
        if let Some(phase) = self.phases.iter().find(|ph| ph.id == phase_id) {
            !phase.tracks.is_empty()
                && phase
                    .tracks
                    .iter()
                    .all(|t| t.steps.iter().all(|s| s.status == StepStatus::Complete))
        } else {
            false
        }
    }

    /// Returns the ID of the next phase after `current_phase`, lexicographically.
    /// First looks in `self.phases`; if current phase is last, falls back to `.mz/ROADMAP.md`
    /// to find phases not yet in state. Returns `None` if no next phase exists.
    pub fn next_phase_id(&self) -> Option<String> {
        let current = &self.current_phase;

        // Collect all phase IDs from state, sorted lexicographically.
        let mut state_ids: Vec<String> = self.phases.iter().map(|ph| ph.id.clone()).collect();
        state_ids.sort();

        // Find the next phase ID in state that is lexicographically greater than current.
        if let Some(next) = state_ids.iter().find(|id| id.as_str() > current.as_str()) {
            return Some(next.clone());
        }

        // No next phase in state — check ROADMAP.md for phases not yet in state.
        let roadmap_path = mz_dir().join("ROADMAP.md");
        if let Ok(content) = std::fs::read_to_string(&roadmap_path) {
            let state_id_set: std::collections::HashSet<&str> =
                self.phases.iter().map(|ph| ph.id.as_str()).collect();
            // Parse roadmap phases and find the first one not in state that is > current.
            let roadmap_phases = crate::planner::parse_roadmap(&content);
            let mut roadmap_ids: Vec<String> = roadmap_phases
                .into_iter()
                .map(|p| p.id)
                .filter(|id| !state_id_set.contains(id.as_str()) && id.as_str() > current.as_str())
                .collect();
            roadmap_ids.sort();
            if let Some(first) = roadmap_ids.into_iter().next() {
                return Some(first);
            }
        }

        None
    }

    pub fn stats(&self) -> (usize, usize, usize, usize) {
        let mut total = 0;
        let mut done = 0;
        let mut blocked = 0;
        let mut in_progress = 0;
        for ph in &self.phases {
            for track in &ph.tracks {
                for step in &track.steps {
                    total += 1;
                    match step.status {
                        StepStatus::Complete => done += 1,
                        StepStatus::Blocked => blocked += 1,
                        StepStatus::InProgress => in_progress += 1,
                        StepStatus::Pending => {}
                    }
                }
            }
        }
        (total, done, in_progress, blocked)
    }
}

pub fn mz_root() -> PathBuf {
    #[cfg(test)]
    {
        let override_path = TEST_MZ_DIR.with(|d| d.borrow().clone());
        if let Some(path) = override_path {
            return path;
        }
    }
    MZ_ROOT_CACHE
        .get_or_init(|| {
            std::env::current_dir()
                .expect("Failed to get current directory")
                .join(".mz")
        })
        .clone()
}

fn mz_dir() -> PathBuf {
    mz_root()
}

fn state_path() -> PathBuf {
    mz_dir().join("state.yaml")
}

fn project_md_path() -> PathBuf {
    mz_dir().join("PROJECT.md")
}

fn decisions_path() -> PathBuf {
    mz_dir().join("DECISIONS.md")
}

pub fn phases_dir() -> PathBuf {
    mz_dir().join("phases")
}

pub fn phase_dir(phase_id: &str) -> PathBuf {
    phases_dir().join(phase_id)
}

pub fn track_dir(phase_id: &str, track_id: &str) -> PathBuf {
    phase_dir(phase_id).join("tracks").join(track_id)
}

pub fn step_plan_path(phase_id: &str, track_id: &str, step_id: &str) -> PathBuf {
    track_dir(phase_id, track_id)
        .join("steps")
        .join(format!("{}-PLAN.md", step_id))
}

pub fn step_summary_path(phase_id: &str, track_id: &str, step_id: &str) -> PathBuf {
    track_dir(phase_id, track_id)
        .join("steps")
        .join(format!("{}-SUMMARY.md", step_id))
}

pub fn step_output_log_path(phase_id: &str, track_id: &str, step_id: &str) -> PathBuf {
    track_dir(phase_id, track_id)
        .join("steps")
        .join(format!("{}-OUTPUT.log", step_id))
}

pub fn context_path(phase_id: &str) -> PathBuf {
    phase_dir(phase_id).join("CONTEXT.md")
}

pub fn roadmap_path(phase_id: &str) -> PathBuf {
    phase_dir(phase_id).join("ROADMAP.md")
}

pub fn roadmap_global_path() -> PathBuf {
    mz_dir().join("ROADMAP.md")
}

pub fn init_project() -> Result<ProjectState> {
    if mz_dir().exists() {
        bail!(".mz/ directory already exists. Delete it first to re-initialize.");
    }

    // Infer project name from current directory
    let dir_name = std::env::current_dir()?
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

    // Detect tech stack from project files
    let tech_stack = detect_tech_stack();

    // Detect description from README or Cargo.toml
    let description = detect_description(&dir_name);

    println!("Detected project: {}", dir_name);
    if !tech_stack.is_empty() {
        println!("Detected stack:   {}", tech_stack);
    }
    if !description.is_empty() {
        println!("Detected desc:    {}", description);
    }
    println!();

    // Only ask for what we couldn't infer
    let name: String = Input::new()
        .with_prompt("Project name")
        .default(dir_name)
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("What are you building? (one sentence)")
        .default(if description.is_empty() { String::new() } else { description })
        .interact_text()?;

    let tech_stack: String = Input::new()
        .with_prompt("Tech stack")
        .default(if tech_stack.is_empty() { String::new() } else { tech_stack })
        .interact_text()?;

    let constraints: String = Input::new()
        .with_prompt("Key constraints or requirements (or 'none')")
        .default("none".to_string())
        .interact_text()?;

    // Create directory structure
    fs::create_dir_all(mz_dir())?;
    fs::create_dir_all(phases_dir())?;

    // Write PROJECT.md
    let project_md = format!(
        "# {name}\n\n\
         ## Description\n\n{description}\n\n\
         ## Tech Stack\n\n{tech_stack}\n\n\
         ## Constraints\n\n{constraints}\n",
    );
    fs::write(project_md_path(), &project_md)?;

    // Write DECISIONS.md
    let decisions_md = "# Decisions\n\n\
         Append-only register of project decisions.\n\n\
         ---\n"
        .to_string();
    fs::write(decisions_path(), &decisions_md)?;

    // Write initial state
    let state = ProjectState {
        name: name.clone(),
        description: description.clone(),
        current_phase: "P001".to_string(),
        phases: vec![],
    };
    save(&state)?;

    Ok(state)
}

fn detect_tech_stack() -> String {
    let mut stack = vec![];

    if std::path::Path::new("Cargo.toml").exists() {
        stack.push("Rust");
    }
    if std::path::Path::new("package.json").exists() {
        stack.push("Node.js");
        if std::path::Path::new("next.config.js").exists()
            || std::path::Path::new("next.config.ts").exists()
            || std::path::Path::new("next.config.mjs").exists()
        {
            stack.push("Next.js");
        }
        if std::path::Path::new("tsconfig.json").exists() {
            stack.push("TypeScript");
        }
    }
    if std::path::Path::new("go.mod").exists() {
        stack.push("Go");
    }
    if std::path::Path::new("pyproject.toml").exists()
        || std::path::Path::new("requirements.txt").exists()
    {
        stack.push("Python");
    }
    if std::path::Path::new("Gemfile").exists() {
        stack.push("Ruby");
    }
    if std::path::Path::new("docker-compose.yml").exists()
        || std::path::Path::new("docker-compose.yaml").exists()
        || std::path::Path::new("Dockerfile").exists()
    {
        stack.push("Docker");
    }
    if std::path::Path::new("supabase").is_dir() {
        stack.push("Supabase");
    }

    stack.join(", ")
}

fn detect_description(_dir_name: &str) -> String {
    // Try Cargo.toml description
    if let Ok(contents) = fs::read_to_string("Cargo.toml") {
        for line in contents.lines() {
            if let Some(desc) = line.strip_prefix("description") {
                let desc = desc.trim().trim_start_matches('=').trim().trim_matches('"');
                if !desc.is_empty() {
                    return desc.to_string();
                }
            }
        }
    }

    // Try package.json description
    if let Ok(contents) = fs::read_to_string("package.json") {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) {
            if let Some(desc) = json.get("description").and_then(|d| d.as_str()) {
                if !desc.is_empty() {
                    return desc.to_string();
                }
            }
        }
    }

    // Try first meaningful line of README
    for readme in &["README.md", "readme.md", "README"] {
        if let Ok(contents) = fs::read_to_string(readme) {
            for line in contents.lines().skip(1) {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with('[') {
                    return trimmed.to_string();
                }
            }
        }
    }

    String::new()
}

pub fn load() -> Result<ProjectState> {
    let path = state_path();
    if !path.exists() {
        bail!("No .mz/ project found. Run `mz init` first.");
    }
    let lock_path = mz_root().join("state.yaml.lock");
    let lock_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&lock_path)
        .context("Failed to open state lock file")?;
    lock_file.lock_shared().context("Failed to acquire shared lock on state.yaml")?;
    let contents = fs::read_to_string(&path).context("Failed to read state.yaml")?;
    let state: ProjectState = serde_yaml::from_str(&contents).context("Failed to parse state.yaml")?;
    drop(lock_file);
    Ok(state)
}

pub fn save(state: &ProjectState) -> Result<()> {
    let yaml = serde_yaml::to_string(state).context("Failed to serialize state")?;
    let path = state_path();
    let lock_path = mz_root().join("state.yaml.lock");
    let lock_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&lock_path)
        .context("Failed to open state lock file")?;
    lock_file.lock_exclusive().context("Failed to acquire exclusive lock on state.yaml")?;
    // Atomic write: write to temp file then rename
    let tmp = path.with_extension("yaml.tmp");
    fs::write(&tmp, &yaml).context("Failed to write state.yaml.tmp")?;
    fs::rename(&tmp, &path).context("Failed to rename state.yaml.tmp")?;
    drop(lock_file);
    Ok(())
}

/// Load state, apply a mutation, save atomically. Minimizes the race window.
fn mutate_state<F>(f: F) -> Result<()>
where
    F: FnOnce(&mut ProjectState) -> Result<()>,
{
    let mut state = load()?;
    f(&mut state)?;
    save(&state)
}

pub fn mark_step_complete(phase_id: &str, track_id: &str, step_id: &str) -> Result<()> {
    mutate_state(|state| {
        update_step_status(state, phase_id, track_id, step_id, StepStatus::Complete, None)
    })
}

pub fn mark_step_blocked(phase_id: &str, track_id: &str, step_id: &str, reason: &str) -> Result<()> {
    mutate_state(|state| {
        update_step_status(state, phase_id, track_id, step_id, StepStatus::Blocked, Some(reason.to_string()))
    })
}

pub fn mark_step_in_progress(phase_id: &str, track_id: &str, step_id: &str) -> Result<()> {
    mutate_state(|state| {
        update_step_status(state, phase_id, track_id, step_id, StepStatus::InProgress, None)
    })
}

pub fn normalize_phase_id(id: &str) -> String {
    id.to_uppercase()
}

pub fn advance_phase(phase_id: &str) -> Result<()> {
    mutate_state(|state| {
        if phase_id > state.current_phase.as_str() {
            state.current_phase = phase_id.to_string();
        }
        Ok(())
    })
}

pub fn create_skeleton_phase(phase_id: &str, title: &str) -> Result<()> {
    mutate_state(|state| {
        if let Some(existing) = state.phases.iter_mut().find(|ph| ph.id == phase_id) {
            if !existing.tracks.is_empty() {
                // Phase already has tracks — leave it alone
                return Ok(());
            }
            // Phase exists with no tracks — update the title
            existing.title = title.to_string();
            return Ok(());
        }
        // Brand-new skeleton phase
        state.phases.push(PhaseEntry {
            id: phase_id.to_string(),
            title: title.to_string(),
            tracks: vec![],
        });
        state.phases.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(())
    })
}

pub fn remove_skeleton_phase(phase_id: &str) -> Result<()> {
    mutate_state(|state| {
        state.phases.retain(|ph| {
            !(ph.id == phase_id && ph.tracks.is_empty())
        });
        Ok(())
    })
}

pub fn completed_phase_ids() -> Result<Vec<String>> {
    let state = load()?;
    Ok(state
        .phases
        .iter()
        .filter(|ph| state.is_phase_complete(&ph.id))
        .map(|ph| ph.id.clone())
        .collect())
}

pub fn reset_step(phase_id: &str, step_id: &str) -> Result<()> {
    let mut state = load()?;

    // Parse optional track prefix: "TR02/ST01" or just "ST01"
    let (track_prefix, bare_step_id) = if let Some(slash) = step_id.find('/') {
        let (tr, _st) = step_id.split_at(slash);
        (Some(tr.to_string()), &step_id[slash + 1..])
    } else {
        (None, step_id)
    };

    for ph in &mut state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &mut ph.tracks {
            if let Some(ref prefix) = track_prefix {
                if &track.id != prefix {
                    continue;
                }
            }
            for step in &mut track.steps {
                if step.id == bare_step_id {
                    step.status = StepStatus::Pending;
                    step.blocked_reason = None;
                    step.attempts = 0;
                    save(&state)?;
                    println!("Reset {} to pending", step_id);
                    return Ok(());
                }
            }
        }
    }

    bail!("Step {} not found in phase {}", step_id, phase_id)
}

pub fn reset_blocked_steps(phase_id: &str) -> Result<()> {
    let mut state = load()?;
    for ph in &mut state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &mut ph.tracks {
            for step in &mut track.steps {
                if step.status == StepStatus::Blocked {
                    step.status = StepStatus::Pending;
                    step.blocked_reason = None;
                    step.attempts = 0;
                }
            }
        }
    }
    save(&state)
}

pub fn increment_step_attempts(phase_id: &str, track_id: &str, step_id: &str) -> Result<()> {
    let mut state = load()?;
    for ph in &mut state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &mut ph.tracks {
            if track.id != track_id {
                continue;
            }
            for step in &mut track.steps {
                if step.id == step_id {
                    step.attempts += 1;
                    return save(&state);
                }
            }
        }
    }
    bail!("Step {}/{}/{} not found", phase_id, track_id, step_id)
}

fn update_step_status(
    state: &mut ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
    status: StepStatus,
    blocked_reason: Option<String>,
) -> Result<()> {
    for ph in &mut state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &mut ph.tracks {
            if track.id != track_id {
                continue;
            }
            for step in &mut track.steps {
                if step.id == step_id {
                    step.status = status;
                    step.blocked_reason = blocked_reason;
                    return Ok(());
                }
            }
        }
    }
    bail!("Step {}/{}/{} not found", phase_id, track_id, step_id)
}

/// Merge a re-plan into an existing phase, preserving completed/in-progress steps.
///
/// Rules:
/// - Fully-complete tracks are kept as-is.
/// - Partially-complete tracks: keep completed/in-progress steps, replace pending ones
///   with new steps from the replan (if the track appears in new_tracks).
/// - All-pending tracks: replaced entirely with the version from new_tracks, or dropped
///   if not present in new_tracks.
/// - Brand-new track IDs in new_tracks are appended.
pub fn merge_replan(phase: &mut PhaseEntry, new_tracks: Vec<TrackEntry>) {
    use std::collections::HashMap;

    let mut new_track_map: HashMap<String, TrackEntry> =
        new_tracks.into_iter().map(|t| (t.id.clone(), t)).collect();

    let mut result: Vec<TrackEntry> = Vec::new();
    let existing_ids: Vec<String> = phase.tracks.iter().map(|t| t.id.clone()).collect();

    for existing in &phase.tracks {
        let all_complete = !existing.steps.is_empty()
            && existing.steps.iter().all(|s| s.status == StepStatus::Complete);
        let has_complete_or_progress = existing.steps.iter().any(|s| {
            s.status == StepStatus::Complete || s.status == StepStatus::InProgress
        });

        if all_complete {
            // Fully done — preserve unchanged
            result.push(existing.clone());
        } else if has_complete_or_progress {
            // Partial — keep completed/in-progress, append new pending from replan
            let mut merged = existing.clone();
            merged
                .steps
                .retain(|s| s.status == StepStatus::Complete || s.status == StepStatus::InProgress);
            if let Some(new_track) = new_track_map.remove(&existing.id) {
                for new_step in new_track.steps {
                    merged.steps.push(new_step);
                }
                merged.title = new_track.title;
            }
            result.push(merged);
        } else {
            // All pending — replace with replan version, or drop if absent
            if let Some(new_track) = new_track_map.remove(&existing.id) {
                result.push(new_track);
            }
        }
    }

    // Append brand-new tracks (IDs not present in the existing phase)
    for id in &existing_ids {
        new_track_map.remove(id);
    }
    let mut new_additions: Vec<TrackEntry> = new_track_map.into_values().collect();
    new_additions.sort_by(|a, b| a.id.cmp(&b.id));
    result.extend(new_additions);

    result.sort_by(|a, b| a.id.cmp(&b.id));
    phase.tracks = result;
}

pub fn append_decision(message: &str) -> Result<()> {
    let path = decisions_path();
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let entry = format!("\n## [{}]\n\n{}\n", timestamp, message);

    let mut contents = fs::read_to_string(&path).unwrap_or_default();
    contents.push_str(&entry);
    fs::write(&path, contents)?;
    Ok(())
}

pub fn read_project_md() -> Result<String> {
    fs::read_to_string(project_md_path()).context("Failed to read PROJECT.md")
}

pub fn read_decisions() -> Result<String> {
    let path = decisions_path();
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

pub fn read_context(phase_id: &str) -> Result<String> {
    let path = context_path(phase_id);
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

pub fn read_track_plan(phase_id: &str, track_id: &str) -> Result<String> {
    let path = track_dir(phase_id, track_id).join("PLAN.md");
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

pub fn read_step_plan(phase_id: &str, track_id: &str, step_id: &str) -> Result<String> {
    let path = step_plan_path(phase_id, track_id, step_id);
    fs::read_to_string(&path).with_context(|| format!("Failed to read step plan: {}", path.display()))
}

pub fn read_step_summary(phase_id: &str, track_id: &str, step_id: &str) -> Result<String> {
    let path = step_summary_path(phase_id, track_id, step_id);
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

pub fn collect_dependency_summaries(
    state: &ProjectState,
    phase_id: &str,
    track_id: &str,
    step_id: &str,
) -> Result<String> {
    // Find the current track to get its depends_on list
    let depends_on: Vec<String> = state
        .phases
        .iter()
        .find(|ph| ph.id == phase_id)
        .and_then(|ph| ph.tracks.iter().find(|t| t.id == track_id))
        .map(|t| t.depends_on.clone())
        .unwrap_or_default();

    let mut cross_track_parts: Vec<String> = Vec::new();

    // Collect summaries from dependency tracks
    for dep_track_id in &depends_on {
        let mut dep_summaries = String::new();
        if let Some(ph) = state.phases.iter().find(|ph| ph.id == phase_id) {
            if let Some(dep_track) = ph.tracks.iter().find(|t| &t.id == dep_track_id) {
                for step in &dep_track.steps {
                    if step.status == StepStatus::Complete {
                        let summary = read_step_summary(phase_id, dep_track_id, &step.id)?;
                        if !summary.is_empty() {
                            dep_summaries.push_str(&format!(
                                "\n### {} — {}\n\n{}\n",
                                step.id, step.title, summary
                            ));
                        }
                    }
                }
            }
        }
        if !dep_summaries.is_empty() {
            cross_track_parts.push(format!("## From {}\n{}", dep_track_id, dep_summaries));
        }
    }

    // Collect within-track summaries (steps prior to current step)
    let mut within_track = String::new();
    if let Some(ph) = state.phases.iter().find(|ph| ph.id == phase_id) {
        if let Some(track) = ph.tracks.iter().find(|t| t.id == track_id) {
            for step in &track.steps {
                if step.id == step_id {
                    break;
                }
                if step.status == StepStatus::Complete {
                    let summary = read_step_summary(phase_id, track_id, &step.id)?;
                    if !summary.is_empty() {
                        within_track.push_str(&format!(
                            "\n### {} — {}\n\n{}\n",
                            step.id, step.title, summary
                        ));
                    }
                }
            }
        }
    }

    // Combine: cross-track deps first, then current track
    if cross_track_parts.is_empty() && within_track.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::new();
    for part in cross_track_parts {
        result.push_str(&part);
        result.push('\n');
    }
    if !within_track.is_empty() {
        result.push_str(&format!("## From {} (current track)\n{}", track_id, within_track));
    }

    Ok(result)
}

#[cfg(test)]
pub(crate) fn set_test_mz_dir(path: Option<std::path::PathBuf>) {
    TEST_MZ_DIR.with(|d| *d.borrow_mut() = path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// RAII guard: sets the thread-local mz dir override for the duration of the test,
    /// then clears it on drop.
    struct TempMz {
        _dir: TempDir,
    }

    impl TempMz {
        fn new() -> Self {
            let dir = tempfile::tempdir().unwrap();
            let mz_path = dir.path().join(".mz");
            fs::create_dir_all(&mz_path).unwrap();
            TEST_MZ_DIR.with(|d| *d.borrow_mut() = Some(mz_path));
            TempMz { _dir: dir }
        }
    }

    impl Drop for TempMz {
        fn drop(&mut self) {
            TEST_MZ_DIR.with(|d| *d.borrow_mut() = None);
        }
    }

    fn make_state() -> ProjectState {
        ProjectState {
            name: "test".to_string(),
            description: "test project".to_string(),
            current_phase: "P001".to_string(),
            phases: vec![PhaseEntry {
                id: "P001".to_string(),
                title: "Phase 1".to_string(),
                tracks: vec![
                    TrackEntry {
                        id: "TR01".to_string(),
                        title: "Track 1".to_string(),
                        depends_on: vec![],
                        steps: vec![
                            StepEntry {
                                id: "ST01".to_string(),
                                title: "Step 1".to_string(),
                                status: StepStatus::Complete,
                                blocked_reason: None,
                                attempts: 0,
                            },
                            StepEntry {
                                id: "ST02".to_string(),
                                title: "Step 2".to_string(),
                                status: StepStatus::Pending,
                                blocked_reason: None,
                                attempts: 0,
                            },
                        ],
                    },
                    TrackEntry {
                        id: "TR02".to_string(),
                        title: "Track 2".to_string(),
                        depends_on: vec![],
                        steps: vec![StepEntry {
                            id: "ST01".to_string(),
                            title: "Step 1".to_string(),
                            status: StepStatus::Blocked,
                            blocked_reason: Some("reason".to_string()),
                            attempts: 1,
                        }],
                    },
                ],
            }],
        }
    }

    #[test]
    fn test_path_helpers() {
        // These are pure path computations — no filesystem needed.
        let phase = phase_dir("P001");
        assert!(phase.ends_with("P001"));
        assert!(phase.to_string_lossy().contains("phases"));

        let track = track_dir("P001", "TR01");
        assert!(track.to_string_lossy().contains("P001"));
        assert!(track.to_string_lossy().contains("TR01"));
        assert!(track.to_string_lossy().contains("tracks"));

        let plan = step_plan_path("P001", "TR01", "ST01");
        assert!(plan.to_string_lossy().contains("ST01-PLAN.md"));
        assert!(plan.to_string_lossy().contains("steps"));

        let summary = step_summary_path("P001", "TR01", "ST01");
        assert!(summary.to_string_lossy().contains("ST01-SUMMARY.md"));
        assert!(summary.to_string_lossy().contains("steps"));

        // mz_dir() is the root of all paths
        let mz = mz_dir();
        assert!(phase.starts_with(&mz));
        assert!(track.starts_with(&mz));
        assert!(plan.starts_with(&mz));
        assert!(summary.starts_with(&mz));
    }

    #[test]
    fn test_step_status_serde() {
        for (status, expected) in &[
            (StepStatus::Pending, "pending"),
            (StepStatus::InProgress, "inprogress"),
            (StepStatus::Complete, "complete"),
            (StepStatus::Blocked, "blocked"),
        ] {
            let yaml = serde_yaml::to_string(status).unwrap();
            assert!(yaml.trim() == *expected, "expected '{expected}', got '{}'", yaml.trim());
            let back: StepStatus = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(&back, status);
        }
    }

    #[test]
    fn test_next_pending_step_ordering() {
        // in_progress before pending
        let mut state = make_state();
        // Set TR01/ST02 to InProgress
        state.phases[0].tracks[0].steps[1].status = StepStatus::InProgress;

        let next = state.next_pending_step();
        assert_eq!(next, Some(("P001".into(), "TR01".into(), "ST02".into())));

        // Now clear in_progress — should return first pending
        state.phases[0].tracks[0].steps[1].status = StepStatus::Pending;
        let next = state.next_pending_step();
        assert_eq!(next, Some(("P001".into(), "TR01".into(), "ST02".into())));
    }

    #[test]
    fn test_next_pending_step_empty() {
        let mut state = make_state();
        // Mark all steps complete
        for track in &mut state.phases[0].tracks {
            for step in &mut track.steps {
                step.status = StepStatus::Complete;
                step.blocked_reason = None;
            }
        }
        assert_eq!(state.next_pending_step(), None);
    }

    #[test]
    fn test_is_track_complete() {
        let mut state = make_state();

        // TR01 has one complete and one pending — not complete
        assert!(!state.is_track_complete("P001", "TR01"));

        // Mark TR01/ST02 complete
        state.phases[0].tracks[0].steps[1].status = StepStatus::Complete;
        assert!(state.is_track_complete("P001", "TR01"));

        // TR02 has one blocked step — not complete
        assert!(!state.is_track_complete("P001", "TR02"));

        // Non-existent track returns false
        assert!(!state.is_track_complete("P001", "TR99"));
    }

    #[test]
    fn test_stats() {
        let state = make_state();
        // TR01: ST01=complete, ST02=pending
        // TR02: ST01=blocked
        // total=3, done=1, in_progress=0, blocked=1
        let (total, done, in_progress, blocked) = state.stats();
        assert_eq!(total, 3);
        assert_eq!(done, 1);
        assert_eq!(in_progress, 0);
        assert_eq!(blocked, 1);
    }

    #[test]
    fn test_mark_step_transitions() {
        let _tmp = TempMz::new();

        // Save initial state
        let state = make_state();
        save(&state).unwrap();

        // pending → in_progress
        mark_step_in_progress("P001", "TR01", "ST02").unwrap();
        let s = load().unwrap();
        let step = &s.phases[0].tracks[0].steps[1];
        assert_eq!(step.status, StepStatus::InProgress);

        // in_progress → complete
        mark_step_complete("P001", "TR01", "ST02").unwrap();
        let s = load().unwrap();
        let step = &s.phases[0].tracks[0].steps[1];
        assert_eq!(step.status, StepStatus::Complete);
        assert_eq!(step.blocked_reason, None);

        // pending → blocked with reason
        mark_step_in_progress("P001", "TR01", "ST02").unwrap(); // reset to in_progress first
        // Actually mark fresh step as blocked
        let mut s = load().unwrap();
        s.phases[0].tracks[0].steps[1].status = StepStatus::Pending;
        s.phases[0].tracks[0].steps[1].blocked_reason = None;
        save(&s).unwrap();

        mark_step_blocked("P001", "TR01", "ST02", "verification failed").unwrap();
        let s = load().unwrap();
        let step = &s.phases[0].tracks[0].steps[1];
        assert_eq!(step.status, StepStatus::Blocked);
        assert_eq!(step.blocked_reason.as_deref(), Some("verification failed"));
    }

    #[test]
    fn test_reset_step() {
        let _tmp = TempMz::new();

        let mut state = make_state();
        // Give TR01/ST01 a blocked reason to ensure reset clears it
        state.phases[0].tracks[0].steps[0].status = StepStatus::Blocked;
        state.phases[0].tracks[0].steps[0].blocked_reason = Some("broken".to_string());
        state.phases[0].tracks[0].steps[0].attempts = 3;
        save(&state).unwrap();

        // Reset with track prefix
        reset_step("P001", "TR01/ST01").unwrap();
        let s = load().unwrap();
        let step = &s.phases[0].tracks[0].steps[0];
        assert_eq!(step.status, StepStatus::Pending);
        assert_eq!(step.blocked_reason, None);
        assert_eq!(step.attempts, 0);

        // Reset without track prefix (bare step id, finds first match)
        let mut s = load().unwrap();
        s.phases[0].tracks[0].steps[1].status = StepStatus::Blocked;
        s.phases[0].tracks[0].steps[1].blocked_reason = Some("other".to_string());
        save(&s).unwrap();

        reset_step("P001", "ST02").unwrap();
        let s = load().unwrap();
        let step = &s.phases[0].tracks[0].steps[1];
        assert_eq!(step.status, StepStatus::Pending);
        assert_eq!(step.blocked_reason, None);
    }

    #[test]
    fn test_advance_phase() {
        let _tmp = TempMz::new();

        let mut state = make_state();
        state.current_phase = "P001".to_string();
        save(&state).unwrap();

        // Forward advance: P001 -> P002
        advance_phase("P002").unwrap();
        let s = load().unwrap();
        assert_eq!(s.current_phase, "P002");

        // Backward: P001 < P002, no change
        advance_phase("P001").unwrap();
        let s = load().unwrap();
        assert_eq!(s.current_phase, "P002");

        // Lateral: same phase, no change
        advance_phase("P002").unwrap();
        let s = load().unwrap();
        assert_eq!(s.current_phase, "P002");

        // Forward advance: P002 -> P003
        advance_phase("P003").unwrap();
        let s = load().unwrap();
        assert_eq!(s.current_phase, "P003");
    }

    #[test]
    fn test_is_phase_complete() {
        let mut state = make_state();

        // P001 has incomplete steps — not complete
        assert!(!state.is_phase_complete("P001"));

        // Mark all steps complete
        for track in &mut state.phases[0].tracks {
            for step in &mut track.steps {
                step.status = StepStatus::Complete;
            }
        }
        assert!(state.is_phase_complete("P001"));

        // Non-existent phase returns false
        assert!(!state.is_phase_complete("P999"));

        // Phase with no tracks is not complete
        let mut state2 = make_state();
        state2.phases.push(PhaseEntry {
            id: "P002".to_string(),
            title: "Empty phase".to_string(),
            tracks: vec![],
        });
        assert!(!state2.is_phase_complete("P002"));
    }

    #[test]
    fn test_create_skeleton_phase() {
        let _tmp = TempMz::new();

        let state = make_state();
        save(&state).unwrap();

        // Create a new skeleton phase
        create_skeleton_phase("P002", "Phase Two").unwrap();
        let s = load().unwrap();
        let p2 = s.phases.iter().find(|p| p.id == "P002").unwrap();
        assert_eq!(p2.title, "Phase Two");
        assert!(p2.tracks.is_empty());

        // Idempotent: creating again preserves title
        create_skeleton_phase("P002", "Phase Two Again").unwrap();
        let s = load().unwrap();
        let p2 = s.phases.iter().find(|p| p.id == "P002").unwrap();
        // title updated since still no tracks
        assert_eq!(p2.title, "Phase Two Again");

        // Phases are sorted by id
        create_skeleton_phase("P000", "Phase Zero").unwrap();
        let s = load().unwrap();
        assert_eq!(s.phases[0].id, "P000");
        assert_eq!(s.phases[1].id, "P001");
        assert_eq!(s.phases[2].id, "P002");

        // Skip-if-has-tracks: P001 already has tracks — title must not change
        create_skeleton_phase("P001", "Should Not Change").unwrap();
        let s = load().unwrap();
        let p1 = s.phases.iter().find(|p| p.id == "P001").unwrap();
        assert_eq!(p1.title, "Phase 1");
        assert!(!p1.tracks.is_empty());
    }

    #[test]
    fn test_completed_phase_ids() {
        let _tmp = TempMz::new();

        let mut state = make_state();
        save(&state).unwrap();

        // No phases complete yet
        let ids = completed_phase_ids().unwrap();
        assert!(ids.is_empty());

        // Mark all steps in P001 complete
        for track in &mut state.phases[0].tracks {
            for step in &mut track.steps {
                step.status = StepStatus::Complete;
            }
        }
        save(&state).unwrap();

        let ids = completed_phase_ids().unwrap();
        assert_eq!(ids, vec!["P001".to_string()]);
    }

    #[test]
    fn test_normalize_phase_id() {
        assert_eq!(normalize_phase_id("p008"), "P008");
        assert_eq!(normalize_phase_id("P008"), "P008");
        assert_eq!(normalize_phase_id("p001"), "P001");
    }

    #[test]
    fn test_remove_skeleton_phase() {
        let _tmp = TempMz::new();

        // State: P001 has tracks, P002 is a skeleton (no tracks)
        let mut state = make_state();
        state.phases.push(PhaseEntry {
            id: "P002".to_string(),
            title: "Phase Two".to_string(),
            tracks: vec![],
        });
        save(&state).unwrap();

        // Removing a skeleton phase works
        remove_skeleton_phase("P002").unwrap();
        let s = load().unwrap();
        assert!(s.phases.iter().find(|p| p.id == "P002").is_none());
        assert!(s.phases.iter().find(|p| p.id == "P001").is_some());

        // Removing a phase with tracks is a no-op
        remove_skeleton_phase("P001").unwrap();
        let s = load().unwrap();
        assert!(s.phases.iter().find(|p| p.id == "P001").is_some());
        assert!(!s.phases.iter().find(|p| p.id == "P001").unwrap().tracks.is_empty());

        // Removing a non-existent phase is a no-op (no error)
        remove_skeleton_phase("P999").unwrap();
        let s = load().unwrap();
        assert_eq!(s.phases.len(), 1); // only P001 remains
    }

    #[test]
    fn test_concurrent_save_load_no_corruption() {
        use std::sync::Arc;

        // Set up a shared temp dir that both threads will use.
        let dir = tempfile::tempdir().unwrap();
        let mz_path = dir.path().join(".mz");
        fs::create_dir_all(&mz_path).unwrap();

        // Write initial state into the shared dir.
        let mz_path = Arc::new(mz_path);
        {
            let initial = make_state();
            let yaml = serde_yaml::to_string(&initial).unwrap();
            fs::write(mz_path.join("state.yaml"), &yaml).unwrap();
        }

        let mz_path_a = Arc::clone(&mz_path);
        let mz_path_b = Arc::clone(&mz_path);

        let thread_a = std::thread::spawn(move || {
            // Each thread sets its own TEST_MZ_DIR to the shared directory.
            TEST_MZ_DIR.with(|d| *d.borrow_mut() = Some((*mz_path_a).clone()));
            for i in 0..50_u32 {
                let mut state = load().unwrap();
                state.phases[0].tracks[0].steps[0].attempts = i;
                save(&state).unwrap();
            }
            TEST_MZ_DIR.with(|d| *d.borrow_mut() = None);
        });

        let thread_b = std::thread::spawn(move || {
            TEST_MZ_DIR.with(|d| *d.borrow_mut() = Some((*mz_path_b).clone()));
            for i in 0..50_u32 {
                let mut state = load().unwrap();
                state.phases[0].tracks[0].steps[1].attempts = i;
                save(&state).unwrap();
            }
            TEST_MZ_DIR.with(|d| *d.borrow_mut() = None);
        });

        thread_a.join().expect("thread A panicked");
        thread_b.join().expect("thread B panicked");

        // Verify the final YAML is parseable (no corruption).
        let final_yaml = fs::read_to_string(mz_path.join("state.yaml")).unwrap();
        let _state: ProjectState = serde_yaml::from_str(&final_yaml)
            .expect("state.yaml is not valid YAML after concurrent writes");
    }

    /// Two threads each call load() and save() 10 times concurrently; final state is valid YAML.
    #[test]
    fn test_concurrent_state_access() {
        let dir = tempfile::tempdir().unwrap();
        let mz_path = dir.path().join(".mz");
        fs::create_dir_all(&mz_path).unwrap();

        let initial = make_state();
        let yaml = serde_yaml::to_string(&initial).unwrap();
        fs::write(mz_path.join("state.yaml"), &yaml).unwrap();

        let mz_path = std::sync::Arc::new(mz_path);
        let handles: Vec<_> = (0..2)
            .map(|_| {
                let mp = std::sync::Arc::clone(&mz_path);
                std::thread::spawn(move || {
                    TEST_MZ_DIR.with(|d| *d.borrow_mut() = Some((*mp).clone()));
                    for _ in 0..10 {
                        let mut s = load().expect("load should succeed");
                        s.name = "concurrent".to_string();
                        save(&s).expect("save should succeed");
                    }
                    TEST_MZ_DIR.with(|d| *d.borrow_mut() = None);
                })
            })
            .collect();

        for h in handles {
            h.join().expect("thread should not panic");
        }

        // Final state must be valid YAML with no corruption.
        let final_yaml = fs::read_to_string(mz_path.join("state.yaml")).unwrap();
        let final_state: ProjectState = serde_yaml::from_str(&final_yaml)
            .expect("state.yaml must be valid YAML after concurrent writes");
        assert_eq!(final_state.current_phase, "P001");
    }
}

fn format_duration_ms(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60_000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let total_secs = ms / 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{}m {}s", mins, secs)
    }
}

pub fn print_status(state: &ProjectState, detail: bool) -> Result<()> {
    use crate::run_record;

    let (total, done, in_progress, blocked) = state.stats();
    let pending = total.saturating_sub(done + in_progress + blocked);

    // Load run records (always needed for cost summary); ignore errors (e.g. missing file)
    let all_records = run_record::load_all().unwrap_or_default();

    // Build index: (phase_id, track_id, step_id) -> records
    let mut record_map: HashMap<(String, String, String), Vec<&crate::run_record::RunRecord>> =
        HashMap::new();
    for r in &all_records {
        record_map
            .entry((r.phase_id.clone(), r.track_id.clone(), r.step_id.clone()))
            .or_default()
            .push(r);
    }

    println!("{}", format!("Project: {}", state.name).bold());
    println!("Current phase: {}\n", state.current_phase);

    if total == 0 {
        println!("No steps yet. Run `mz plan` to decompose into steps.");
        return Ok(());
    }

    let pct = if total > 0 { done * 100 / total } else { 0 };
    let bar_width = 30;
    let filled = done * bar_width / total;
    let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);
    println!("[{}] {}% ({}/{})", bar, pct, done, total);
    println!(
        "  {} done  {} in progress  {} pending  {} blocked\n",
        format!("{}", done).green(),
        format!("{}", in_progress).yellow(),
        format!("{}", pending).normal(),
        format!("{}", blocked).red(),
    );

    for ph in &state.phases {
        println!("{}", format!("{} — {}", ph.id, ph.title).bold());
        for track in &ph.tracks {
            let track_done = track.steps.iter().filter(|s| s.status == StepStatus::Complete).count();
            let track_total = track.steps.len();
            let marker = if track_done == track_total && track_total > 0 {
                "✓".green().to_string()
            } else {
                "○".normal().to_string()
            };
            println!("  {} {} — {} ({}/{})", marker, track.id, track.title, track_done, track_total);

            for step in &track.steps {
                    // Always show blocked steps; other steps only in detail mode
                    if step.status != StepStatus::Blocked && !detail {
                        continue;
                    }

                    let icon = match step.status {
                        StepStatus::Complete => "  ✓".green().to_string(),
                        StepStatus::InProgress => "  ▶".yellow().to_string(),
                        StepStatus::Blocked => "  ✗".red().to_string(),
                        StepStatus::Pending => "  ○".normal().to_string(),
                    };

                    let key = (ph.id.clone(), track.id.clone(), step.id.clone());

                    // Build suffix: blocked steps get [BLOCKED: ...], others nothing
                    let suffix = if step.status == StepStatus::Blocked {
                        match &step.blocked_reason {
                            Some(r) => {
                                if detail {
                                    // In detail mode: show full reason + live cost from records
                                    let first_clause =
                                        r.split(':').next().unwrap_or(r.as_str()).trim();
                                    let cost_str =
                                        if let Some(recs) = record_map.get(&key) {
                                            let costs: Vec<f64> =
                                                recs.iter().filter_map(|r| r.cost_usd).collect();
                                            if costs.is_empty() {
                                                String::new()
                                            } else {
                                                let total: f64 = costs.iter().sum();
                                                format!(", ${:.2} burned", total)
                                            }
                                        } else {
                                            String::new()
                                        };
                                    // Extract the detail clause (between ":" and the cost suffix)
                                    let detail_clause: String = r
                                        .split_once(':')
                                        .map(|(_, rest)| {
                                            // strip trailing ", $X.XX burned" if present
                                            let rest = rest.trim();
                                            if let Some(pos) = rest.rfind(", $") {
                                                rest[..pos].to_string()
                                            } else {
                                                rest.to_string()
                                            }
                                        })
                                        .unwrap_or_default();
                                    if detail_clause.is_empty() {
                                        format!("  [BLOCKED: {}{}]", first_clause, cost_str)
                                            .red()
                                            .to_string()
                                    } else {
                                        format!(
                                            "  [BLOCKED: {}, {}{}]",
                                            first_clause, detail_clause, cost_str
                                        )
                                        .red()
                                        .to_string()
                                    }
                                } else {
                                    // Without detail: just the issue type (before ":")
                                    let first_clause =
                                        r.split(':').next().unwrap_or(r.as_str()).trim();
                                    format!("  [BLOCKED: {}]", first_clause).red().to_string()
                                }
                            }
                            None => "  [BLOCKED]".red().to_string(),
                        }
                    } else {
                        String::new()
                    };

                    // Append timing/cost only for completed or in-progress steps with data
                    let timing_suffix =
                        if matches!(step.status, StepStatus::Complete | StepStatus::InProgress) {
                            if let Some(recs) = record_map.get(&key) {
                                let total_ms: u64 = recs.iter().map(|r| r.duration_ms).sum();
                                let total_cost: Option<f64> = {
                                    let costs: Vec<f64> =
                                        recs.iter().filter_map(|r| r.cost_usd).collect();
                                    if costs.is_empty() {
                                        None
                                    } else {
                                        Some(costs.iter().sum())
                                    }
                                };
                                let attempts = recs
                                    .iter()
                                    .filter(|r| r.stage == "execute")
                                    .count();

                                let time_str = format_duration_ms(total_ms);
                                let cost_str = match total_cost {
                                    Some(c) => format!("${:.4}", c),
                                    None => "-".to_string(),
                                };
                                let attempt_word =
                                    if attempts == 1 { "attempt" } else { "attempts" };
                                format!(" [{}, {}, {} {}]", time_str, cost_str, attempts, attempt_word)
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };
                    println!("  {} {} — {}{}{}", icon, step.id, step.title, suffix, timing_suffix);
                }
        }
        println!();
    }

    // Show next step
    if let Some((ph, tr, st)) = state.next_pending_step() {
        println!("{}", format!("Next: {}/{}/{}", ph, tr, st).cyan());
    }

    // Cost summary
    {
        use crate::budget;
        let total_cost = run_record::total_project_cost(&all_records);
        let budget_cfg = budget::load().unwrap_or_default();
        print!("\nCost: ${:.4} spent", total_cost);
        if let Some(limit) = budget_cfg.max_usd {
            let pct_remaining = if limit > 0.0 {
                ((limit - total_cost).max(0.0) / limit) * 100.0
            } else {
                0.0
            };
            print!(" / ${:.2} limit ({:.1}% remaining)", limit, pct_remaining);
        }
        println!();

        if detail {
            let summaries = run_record::phase_summaries(&all_records);
            if !summaries.is_empty() {
                println!("  {:<12} {:>10}", "Phase", "Cost");
                for ps in &summaries {
                    println!("  {:<12} ${:>9.4}", ps.phase_id, ps.cost_usd);
                }
            }
        }
    }

    Ok(())
}
