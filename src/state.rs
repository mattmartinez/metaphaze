use anyhow::{bail, Context, Result};
use chrono::Utc;
use colored::Colorize;
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const MZ_DIR: &str = ".mz";

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
        // Second pass: return next pending step
        for ph in &self.phases {
            if ph.id != self.current_phase {
                continue;
            }
            for track in &ph.tracks {
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

fn mz_dir() -> PathBuf {
    PathBuf::from(MZ_DIR)
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

#[allow(dead_code)]
pub fn mz_root() -> PathBuf {
    mz_dir()
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

pub fn context_path(phase_id: &str) -> PathBuf {
    phase_dir(phase_id).join("CONTEXT.md")
}

pub fn roadmap_path(phase_id: &str) -> PathBuf {
    phase_dir(phase_id).join("ROADMAP.md")
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
    let contents = fs::read_to_string(&path).context("Failed to read state.yaml")?;
    let state: ProjectState = serde_yaml::from_str(&contents).context("Failed to parse state.yaml")?;
    Ok(state)
}

pub fn save(state: &ProjectState) -> Result<()> {
    let yaml = serde_yaml::to_string(state).context("Failed to serialize state")?;
    fs::write(state_path(), yaml).context("Failed to write state.yaml")?;
    Ok(())
}

pub fn mark_step_complete(phase_id: &str, track_id: &str, step_id: &str) -> Result<()> {
    let mut state = load()?;
    update_step_status(&mut state, phase_id, track_id, step_id, StepStatus::Complete, None)?;
    save(&state)
}

pub fn mark_step_blocked(phase_id: &str, track_id: &str, step_id: &str, reason: &str) -> Result<()> {
    let mut state = load()?;
    update_step_status(
        &mut state,
        phase_id,
        track_id,
        step_id,
        StepStatus::Blocked,
        Some(reason.to_string()),
    )?;
    save(&state)
}

pub fn mark_step_in_progress(phase_id: &str, track_id: &str, step_id: &str) -> Result<()> {
    let mut state = load()?;
    update_step_status(&mut state, phase_id, track_id, step_id, StepStatus::InProgress, None)?;
    save(&state)
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
    let mut summaries = String::new();

    for ph in &state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &ph.tracks {
            if track.id != track_id {
                continue;
            }
            for step in &track.steps {
                if step.id == step_id {
                    break;
                }
                if step.status == StepStatus::Complete {
                    let summary = read_step_summary(phase_id, track_id, &step.id)?;
                    if !summary.is_empty() {
                        summaries.push_str(&format!(
                            "\n### {} — {}\n\n{}\n",
                            step.id, step.title, summary
                        ));
                    }
                }
            }
        }
    }

    Ok(summaries)
}

pub fn print_status(state: &ProjectState, detail: bool) -> Result<()> {
    let (total, done, in_progress, blocked) = state.stats();
    let pending = total.saturating_sub(done + in_progress + blocked);

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

            if detail {
                for step in &track.steps {
                    let icon = match step.status {
                        StepStatus::Complete => "  ✓".green().to_string(),
                        StepStatus::InProgress => "  ▶".yellow().to_string(),
                        StepStatus::Blocked => "  ✗".red().to_string(),
                        StepStatus::Pending => "  ○".normal().to_string(),
                    };
                    let suffix = match &step.blocked_reason {
                        Some(r) => format!(" ({})", r).red().to_string(),
                        None => String::new(),
                    };
                    println!("  {} {} — {}{}", icon, step.id, step.title, suffix);
                }
            }
        }
        println!();
    }

    // Show next step
    if let Some((ph, tr, st)) = state.next_pending_step() {
        println!("{}", format!("Next: {}/{}/{}", ph, tr, st).cyan());
    }

    Ok(())
}
