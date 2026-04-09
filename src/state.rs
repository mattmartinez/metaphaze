use anyhow::{bail, Context, Result};
use chrono::Utc;
use colored::Colorize;
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const MZ_DIR: &str = ".mz";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEntry {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Complete,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceEntry {
    pub id: String,
    pub title: String,
    pub tasks: Vec<TaskEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneEntry {
    pub id: String,
    pub title: String,
    pub slices: Vec<SliceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub name: String,
    pub description: String,
    pub current_milestone: String,
    pub milestones: Vec<MilestoneEntry>,
}

impl ProjectState {
    pub fn current_milestone(&self) -> &str {
        &self.current_milestone
    }

    pub fn next_pending_task(&self) -> Option<(String, String, String)> {
        for ms in &self.milestones {
            if ms.id != self.current_milestone {
                continue;
            }
            for slice in &ms.slices {
                for task in &slice.tasks {
                    if task.status == TaskStatus::Pending {
                        return Some((ms.id.clone(), slice.id.clone(), task.id.clone()));
                    }
                }
            }
        }
        None
    }

    pub fn is_slice_complete(&self, milestone_id: &str, slice_id: &str) -> bool {
        for ms in &self.milestones {
            if ms.id != milestone_id {
                continue;
            }
            for slice in &ms.slices {
                if slice.id != slice_id {
                    continue;
                }
                return slice.tasks.iter().all(|t| t.status == TaskStatus::Complete);
            }
        }
        false
    }

    pub fn stats(&self) -> (usize, usize, usize, usize) {
        let mut total = 0;
        let mut done = 0;
        let mut blocked = 0;
        let mut in_progress = 0;
        for ms in &self.milestones {
            for slice in &ms.slices {
                for task in &slice.tasks {
                    total += 1;
                    match task.status {
                        TaskStatus::Complete => done += 1,
                        TaskStatus::Blocked => blocked += 1,
                        TaskStatus::InProgress => in_progress += 1,
                        TaskStatus::Pending => {}
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

pub fn milestones_dir() -> PathBuf {
    mz_dir().join("milestones")
}

pub fn milestone_dir(milestone_id: &str) -> PathBuf {
    milestones_dir().join(milestone_id)
}

pub fn slice_dir(milestone_id: &str, slice_id: &str) -> PathBuf {
    milestone_dir(milestone_id).join("slices").join(slice_id)
}

pub fn task_plan_path(milestone_id: &str, slice_id: &str, task_id: &str) -> PathBuf {
    slice_dir(milestone_id, slice_id)
        .join("tasks")
        .join(format!("{}-PLAN.md", task_id))
}

pub fn task_summary_path(milestone_id: &str, slice_id: &str, task_id: &str) -> PathBuf {
    slice_dir(milestone_id, slice_id)
        .join("tasks")
        .join(format!("{}-SUMMARY.md", task_id))
}

pub fn context_path(milestone_id: &str) -> PathBuf {
    milestone_dir(milestone_id).join("CONTEXT.md")
}

pub fn roadmap_path(milestone_id: &str) -> PathBuf {
    milestone_dir(milestone_id).join("ROADMAP.md")
}

pub fn init_project() -> Result<ProjectState> {
    if mz_dir().exists() {
        bail!(".mz/ directory already exists. Delete it first to re-initialize.");
    }

    let name: String = Input::new()
        .with_prompt("Project name")
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("What are you building? (one sentence)")
        .interact_text()?;

    let tech_stack: String = Input::new()
        .with_prompt("Tech stack (e.g., Next.js, Rust, Supabase)")
        .interact_text()?;

    let constraints: String = Input::new()
        .with_prompt("Key constraints or requirements (or 'none')")
        .default("none".to_string())
        .interact_text()?;

    // Create directory structure
    fs::create_dir_all(mz_dir())?;
    fs::create_dir_all(milestones_dir())?;

    // Write PROJECT.md
    let project_md = format!(
        "# {name}\n\n\
         ## Description\n\n{description}\n\n\
         ## Tech Stack\n\n{tech_stack}\n\n\
         ## Constraints\n\n{constraints}\n",
    );
    fs::write(project_md_path(), &project_md)?;

    // Write DECISIONS.md
    let decisions_md = format!(
        "# Decisions\n\n\
         Append-only register of project decisions.\n\n\
         ---\n"
    );
    fs::write(decisions_path(), &decisions_md)?;

    // Write initial state
    let state = ProjectState {
        name: name.clone(),
        description: description.clone(),
        current_milestone: "M001".to_string(),
        milestones: vec![],
    };
    save(&state)?;

    Ok(state)
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

pub fn mark_task_complete(milestone_id: &str, slice_id: &str, task_id: &str) -> Result<()> {
    let mut state = load()?;
    update_task_status(&mut state, milestone_id, slice_id, task_id, TaskStatus::Complete, None)?;
    save(&state)
}

pub fn mark_task_blocked(milestone_id: &str, slice_id: &str, task_id: &str, reason: &str) -> Result<()> {
    let mut state = load()?;
    update_task_status(
        &mut state,
        milestone_id,
        slice_id,
        task_id,
        TaskStatus::Blocked,
        Some(reason.to_string()),
    )?;
    save(&state)
}

pub fn mark_task_in_progress(milestone_id: &str, slice_id: &str, task_id: &str) -> Result<()> {
    let mut state = load()?;
    update_task_status(&mut state, milestone_id, slice_id, task_id, TaskStatus::InProgress, None)?;
    save(&state)
}

fn update_task_status(
    state: &mut ProjectState,
    milestone_id: &str,
    slice_id: &str,
    task_id: &str,
    status: TaskStatus,
    blocked_reason: Option<String>,
) -> Result<()> {
    for ms in &mut state.milestones {
        if ms.id != milestone_id {
            continue;
        }
        for slice in &mut ms.slices {
            if slice.id != slice_id {
                continue;
            }
            for task in &mut slice.tasks {
                if task.id == task_id {
                    task.status = status;
                    task.blocked_reason = blocked_reason;
                    return Ok(());
                }
            }
        }
    }
    bail!("Task {}/{}/{} not found", milestone_id, slice_id, task_id)
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

pub fn read_context(milestone_id: &str) -> Result<String> {
    let path = context_path(milestone_id);
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

pub fn read_task_plan(milestone_id: &str, slice_id: &str, task_id: &str) -> Result<String> {
    let path = task_plan_path(milestone_id, slice_id, task_id);
    fs::read_to_string(&path).with_context(|| format!("Failed to read task plan: {}", path.display()))
}

pub fn read_task_summary(milestone_id: &str, slice_id: &str, task_id: &str) -> Result<String> {
    let path = task_summary_path(milestone_id, slice_id, task_id);
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(String::new())
    }
}

pub fn collect_dependency_summaries(
    state: &ProjectState,
    milestone_id: &str,
    slice_id: &str,
    task_id: &str,
) -> Result<String> {
    let mut summaries = String::new();

    for ms in &state.milestones {
        if ms.id != milestone_id {
            continue;
        }
        for slice in &ms.slices {
            if slice.id != slice_id {
                continue;
            }
            for task in &slice.tasks {
                if task.id == task_id {
                    break;
                }
                if task.status == TaskStatus::Complete {
                    let summary = read_task_summary(milestone_id, slice_id, &task.id)?;
                    if !summary.is_empty() {
                        summaries.push_str(&format!(
                            "\n### {} — {}\n\n{}\n",
                            task.id, task.title, summary
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
    println!("Current milestone: {}\n", state.current_milestone);

    if total == 0 {
        println!("No tasks yet. Run `mz plan` to decompose into tasks.");
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

    for ms in &state.milestones {
        println!("{}", format!("{} — {}", ms.id, ms.title).bold());
        for slice in &ms.slices {
            let slice_done = slice.tasks.iter().filter(|t| t.status == TaskStatus::Complete).count();
            let slice_total = slice.tasks.len();
            let marker = if slice_done == slice_total && slice_total > 0 {
                "✓".green().to_string()
            } else {
                "○".normal().to_string()
            };
            println!("  {} {} — {} ({}/{})", marker, slice.id, slice.title, slice_done, slice_total);

            if detail {
                for task in &slice.tasks {
                    let icon = match task.status {
                        TaskStatus::Complete => "  ✓".green().to_string(),
                        TaskStatus::InProgress => "  ▶".yellow().to_string(),
                        TaskStatus::Blocked => "  ✗".red().to_string(),
                        TaskStatus::Pending => "  ○".normal().to_string(),
                    };
                    let suffix = match &task.blocked_reason {
                        Some(r) => format!(" ({})", r).red().to_string(),
                        None => String::new(),
                    };
                    println!("  {} {} — {}{}", icon, task.id, task.title, suffix);
                }
            }
        }
        println!();
    }

    // Show next task
    if let Some((ms, sl, tk)) = state.next_pending_task() {
        println!("{}", format!("Next: {}/{}/{}", ms, sl, tk).cyan());
    }

    Ok(())
}
