use anyhow::Result;
use regex::Regex;
use std::fs;

use crate::{claude, events, prompt, state};

pub fn generate_roadmap(project_state: &state::ProjectState, sender: Option<&events::EventSender>) -> Result<()> {
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;

    // Build completed phases context string
    let completed_ids = state::completed_phase_ids()?;
    let state_snapshot = state::load()?;
    let mut completed_phases = String::new();
    for phase_id in &completed_ids {
        let title = state_snapshot
            .phases
            .iter()
            .find(|ph| &ph.id == phase_id)
            .map(|ph| ph.title.as_str())
            .unwrap_or("(unknown)");
        completed_phases.push_str(&format!("## {} — {} [COMPLETED]\n", phase_id, title));
    }

    let existing_roadmap = {
        let path = state::roadmap_global_path();
        if path.exists() {
            fs::read_to_string(&path).unwrap_or_default()
        } else {
            String::new()
        }
    };

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "completed_phases", &completed_phases);
    prompt::set(&mut vars, "existing_roadmap", &existing_roadmap);

    let rendered = prompt::render(prompt::templates::PLAN_ROADMAP, &vars);

    let sys_prompt = format!(
        "You are a senior software architect creating a multi-phase roadmap for '{}'.",
        project_state.name,
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(30)
        .system_prompt(&sys_prompt);

    events::emit(sender, "Generating roadmap...");
    let result = claude::run(opts, sender)?;

    // Write .mz/ROADMAP.md
    let roadmap_path = state::roadmap_global_path();
    if let Some(parent) = roadmap_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&roadmap_path, &result)?;

    // Parse and create skeleton phases
    let phases = parse_roadmap(&result);
    let phase_count = phases.len();
    apply_roadmap_phases(&phases)?;

    events::emit(
        sender,
        &format!("Roadmap written to .mz/ROADMAP.md — {} phases planned", phase_count),
    );
    Ok(())
}

pub fn run(project_state: &state::ProjectState, phase_id: &str, sender: Option<&events::EventSender>) -> Result<()> {
    let phase_id = &state::normalize_phase_id(phase_id);
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let context = state::read_context(phase_id)?;

    let mut vars = prompt::vars();
    prompt::set(&mut vars, "project", &project_md);
    prompt::set(&mut vars, "decisions", &decisions);
    prompt::set(&mut vars, "context", &context);
    prompt::set(&mut vars, "phase_id", phase_id);

    let rendered = prompt::render(prompt::templates::PLAN_PHASE, &vars);

    let sys_prompt = format!(
        "You are a senior software architect planning phase {} for '{}'. \
         Decompose the work into tracks (demoable vertical features) and steps \
         (single-context-window units of work). Output the plan in the exact format specified.",
        phase_id, project_state.name,
    );

    let opts = claude::ClaudeOptions::new(rendered)
        .model("opus")
        .max_turns(60)
        .system_prompt(&sys_prompt);

    events::emit(sender, "Generating phase plan...");
    let result = claude::run(opts, sender)?;

    // Write ROADMAP.md
    let ph_dir = state::phase_dir(phase_id);
    fs::create_dir_all(&ph_dir)?;
    fs::write(state::roadmap_path(phase_id), &result)?;

    // Parse the plan output and create step files + update state
    parse_and_create_steps(project_state, phase_id, &result)?;

    // Advance current_phase if this phase is ahead of where we are
    state::advance_phase(phase_id)?;

    events::emit(sender, &format!("Plan written to {}/", ph_dir.display()));
    events::emit(sender, "Review the plan, then run `mz auto` to start execution.");
    Ok(())
}

pub fn replan(project_state: &state::ProjectState, phase_id: &str, decision: &str, sender: Option<&events::EventSender>) -> Result<()> {
    let project_md = state::read_project_md()?;
    let decisions = state::read_decisions()?;
    let context = state::read_context(phase_id)?;

    // Build context about what's already completed (so Claude knows what's frozen)
    let mut completed_context = String::new();
    for ph in &project_state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &ph.tracks {
            let has_complete = track.steps.iter().any(|s| s.status == state::StepStatus::Complete);
            if !has_complete {
                continue;
            }
            let all_complete = track.steps.iter().all(|s| s.status == state::StepStatus::Complete);
            let done_label = if all_complete { " (FULLY COMPLETE — OMIT FROM OUTPUT)" } else { "" };
            completed_context.push_str(&format!(
                "\n## {} — {}{}\n",
                track.id, track.title, done_label
            ));
            for step in &track.steps {
                if step.status == state::StepStatus::Complete {
                    completed_context.push_str(&format!(
                        "### {} — {} [COMPLETED]\n",
                        step.id, step.title
                    ));
                    let summary = state::read_step_summary(phase_id, &track.id, &step.id)?;
                    if !summary.is_empty() {
                        completed_context.push_str(&format!("{}\n\n", summary));
                    }
                }
            }
        }
    }

    let prompt_text = format!(
        "# Re-planning Required\n\n\
         A new decision has been made:\n\n> {}\n\n\
         ## Project\n\n{}\n\n\
         ## Decisions\n\n{}\n\n\
         ## Context\n\n{}\n\n\
         ## Completed Work (frozen — do not include in output)\n\n{}\n\n\
         ## Instructions\n\n\
         Output ONLY the remaining (non-completed) tracks and steps for phase {} \
         using this exact format:\n\n\
         ```\n\
         ## TR{{nn}} — Track Title\n\
         ### ST{{nn}} — Step Title\n\
         (step plan content)\n\
         ```\n\n\
         Rules:\n\
         - Do NOT include fully-completed tracks or completed steps in your output\n\
         - For tracks that are partially complete: output only the remaining pending steps \
           under the same track heading\n\
         - You may add new tracks or reorganize pending work to reflect the new decision\n\
         - Use TR/ST numbering that does not conflict with completed tracks/steps\n\
         - Each step must be completable in a single Claude Code session\n\
         - Output raw markdown only — no preamble, no explanation",
        decision, project_md, decisions, context, completed_context, phase_id,
    );

    let sys_prompt = format!(
        "You are a senior software architect re-planning phase {} for '{}'. \
         Output ONLY the remaining tracks and steps in the exact format specified. \
         Completed work is frozen — do not include it in your output.",
        phase_id, project_state.name,
    );

    let opts = claude::ClaudeOptions::new(prompt_text)
        .model("opus")
        .max_turns(40)
        .system_prompt(&sys_prompt);

    events::emit(sender, "Re-planning remaining steps...");
    let result = claude::run(opts, sender)?;

    // Update ROADMAP.md with the re-plan
    let roadmap_content = format!(
        "# Phase {} Re-plan\n\n\
         > Steering decision: {}\n\n\
         ## Remaining Work\n\n{}",
        phase_id, decision, result
    );
    fs::write(state::roadmap_path(phase_id), &roadmap_content)?;

    // Delete plan files for pending steps — they'll be replaced or dropped
    for ph in &project_state.phases {
        if ph.id != phase_id {
            continue;
        }
        for track in &ph.tracks {
            for step in &track.steps {
                if step.status == state::StepStatus::Pending {
                    let plan_path = state::step_plan_path(phase_id, &track.id, &step.id);
                    if plan_path.exists() {
                        fs::remove_file(&plan_path)?;
                    }
                }
            }
        }
    }

    // Parse Claude's output and write new step/track plan files
    let new_tracks = parse_and_write_tracks(phase_id, &result)?;

    // Merge new tracks into existing state (preserving completed/in-progress steps)
    let mut new_state = project_state.clone();
    for ph in &mut new_state.phases {
        if ph.id == phase_id {
            state::merge_replan(ph, new_tracks);
            state::save(&new_state)?;
            events::emit(sender, "Re-plan complete. ROADMAP.md updated.");
            events::emit(sender, "Run `mz status --detail` to see the updated plan.");
            return Ok(());
        }
    }

    // Phase not found in state yet — shouldn't happen after a steer, but handle gracefully
    events::emit(sender, &format!("Warning: phase {} not found in state; no merge performed.", phase_id));
    Ok(())
}

/// A parsed step with its raw content (before filesystem writes).
pub(crate) struct ParsedStep {
    pub id: String,
    pub title: String,
    pub content: String,
}

/// A parsed track with its raw content and steps (before filesystem writes).
pub(crate) struct ParsedTrack {
    pub id: String,
    pub title: String,
    pub depends_on: Vec<String>,
    pub content: String,
    pub steps: Vec<ParsedStep>,
}

/// Pure parsing: extract tracks and steps from plan text without touching the filesystem.
pub(crate) fn parse_plan(plan_text: &str) -> Vec<ParsedTrack> {
    let track_re = match Regex::new(r"(?m)^## (TR\d+)\s*[—-]\s*(.+)$") {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    let step_re = match Regex::new(r"(?m)^### (ST\d+)\s*[—-]\s*(.+)$") {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    let dep_re = match Regex::new(r"TR(\d+)\s+depends on\s+TR(\d+)") {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    // Extract the preamble (content before the first ## TR header) for dependency parsing
    let first_track_pos = track_re.find(plan_text).map(|m| m.start()).unwrap_or(0);
    let preamble = &plan_text[..first_track_pos];

    // Build a map of track_id -> depends_on from the preamble
    let mut dep_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for caps in dep_re.captures_iter(preamble) {
        let dependent = format!("TR{}", &caps[1]);
        let dependency = format!("TR{}", &caps[2]);
        dep_map.entry(dependent).or_default().push(dependency);
    }

    let mut tracks = vec![];
    let track_matches: Vec<_> = track_re.find_iter(plan_text).collect();

    for (i, track_match) in track_matches.iter().enumerate() {
        let caps = match track_re.captures(track_match.as_str()) {
            Some(c) => c,
            None => continue,
        };
        let track_id = caps[1].to_string();
        let track_title = caps[2].trim().to_string();

        let track_start = track_match.start();
        let track_end = if i + 1 < track_matches.len() {
            track_matches[i + 1].start()
        } else {
            plan_text.len()
        };
        let track_content = &plan_text[track_start..track_end];

        let depends_on = dep_map.remove(&track_id).unwrap_or_default();

        let mut parsed_track = ParsedTrack {
            id: track_id,
            title: track_title,
            depends_on,
            content: track_content.trim().to_string(),
            steps: vec![],
        };

        let step_matches: Vec<_> = step_re.find_iter(track_content).collect();

        for (j, step_match) in step_matches.iter().enumerate() {
            let scaps = match step_re.captures(step_match.as_str()) {
                Some(c) => c,
                None => continue,
            };
            let step_id = scaps[1].to_string();
            let step_title = scaps[2].trim().to_string();

            let step_start = step_match.start();
            let step_end = if j + 1 < step_matches.len() {
                step_matches[j + 1].start()
            } else {
                track_content.len()
            };
            let step_content = &track_content[step_start..step_end];

            parsed_track.steps.push(ParsedStep {
                id: step_id,
                title: step_title,
                content: step_content.trim().to_string(),
            });
        }

        tracks.push(parsed_track);
    }

    tracks
}

/// A parsed phase from a roadmap (before filesystem writes).
pub(crate) struct ParsedPhase {
    pub id: String,
    pub title: String,
    pub description: String,
}

/// Parse a roadmap response into a list of non-completed phases.
///
/// Matches headings of the form `## PNNN — Title` (em-dash or hyphen).
/// The description is all text between the heading and the next `## PNNN` heading (or end of
/// string), trimmed. Phases whose title contains `[COMPLETED]` are filtered out. Phase IDs are
/// normalized to uppercase (Decision 7).
pub(crate) fn parse_roadmap(text: &str) -> Vec<ParsedPhase> {
    let phase_re = match Regex::new(r"(?m)^## (P\d{3})\s*[—-]\s*(.+)$") {
        Ok(r) => r,
        Err(_) => return vec![],
    };

    let mut phases = vec![];
    let matches: Vec<_> = phase_re.find_iter(text).collect();

    for (i, m) in matches.iter().enumerate() {
        let caps = match phase_re.captures(m.as_str()) {
            Some(c) => c,
            None => continue,
        };

        let id = caps[1].to_uppercase();
        let title = caps[2].trim().to_string();

        // Filter out completed phases
        if title.contains("[COMPLETED]") {
            continue;
        }

        let heading_end = m.end();
        let section_end = if i + 1 < matches.len() {
            matches[i + 1].start()
        } else {
            text.len()
        };
        let description = text[heading_end..section_end].trim().to_string();

        phases.push(ParsedPhase { id, title, description });
    }

    phases
}

fn apply_roadmap_phases(phases: &[ParsedPhase]) -> Result<()> {
    for phase in phases {
        state::create_skeleton_phase(&phase.id, &phase.title)?;
    }
    let new_ids: std::collections::HashSet<&str> =
        phases.iter().map(|p| p.id.as_str()).collect();
    let prior = state::load()?;
    for ph in &prior.phases {
        if !new_ids.contains(ph.id.as_str()) && ph.tracks.is_empty() {
            state::remove_skeleton_phase(&ph.id)?;
        }
    }
    Ok(())
}

/// Parse tracks and steps from plan output, write plan files to disk, return track entries.
fn parse_and_write_tracks(phase_id: &str, plan_output: &str) -> Result<Vec<state::TrackEntry>> {
    let parsed_tracks = parse_plan(plan_output);
    let mut track_entries = vec![];

    for parsed_track in parsed_tracks {
        let track_dir = state::track_dir(phase_id, &parsed_track.id);
        let steps_dir = track_dir.join("steps");
        fs::create_dir_all(&steps_dir)?;

        let mut track_entry = state::TrackEntry {
            id: parsed_track.id.clone(),
            title: parsed_track.title.clone(),
            steps: vec![],
            depends_on: parsed_track.depends_on,
        };

        for parsed_step in &parsed_track.steps {
            let plan_path = state::step_plan_path(phase_id, &parsed_track.id, &parsed_step.id);
            fs::write(&plan_path, &parsed_step.content)?;

            track_entry.steps.push(state::StepEntry {
                id: parsed_step.id.clone(),
                title: parsed_step.title.clone(),
                status: state::StepStatus::Pending,
                blocked_reason: None,
                attempts: 0,
            });
        }

        fs::write(track_dir.join("PLAN.md"), &parsed_track.content)?;
        track_entries.push(track_entry);
    }

    Ok(track_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_track() {
        let plan = "\
## TR01 — State Management

### ST01 — Initialize state file

Set up the initial state.rs module.

### ST02 — Add persistence

Write state to disk.
";
        let tracks = parse_plan(plan);
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].id, "TR01");
        assert_eq!(tracks[0].title, "State Management");
        assert_eq!(tracks[0].steps.len(), 2);
        assert_eq!(tracks[0].steps[0].id, "ST01");
        assert_eq!(tracks[0].steps[0].title, "Initialize state file");
        assert_eq!(tracks[0].steps[1].id, "ST02");
        assert_eq!(tracks[0].steps[1].title, "Add persistence");
    }

    #[test]
    fn test_parse_multi_track() {
        let plan = "\
## TR01 — Foundation

### ST01 — Bootstrap project

Create Cargo.toml and main.rs.

## TR02 — CLI

### ST01 — Add clap

Wire up argument parsing.

### ST02 — Add subcommands

Implement next and auto.

## TR03 — Testing

### ST01 — Unit tests

Write tests for state machine.
";
        let tracks = parse_plan(plan);
        assert_eq!(tracks.len(), 3);
        assert_eq!(tracks[0].id, "TR01");
        assert_eq!(tracks[0].steps.len(), 1);
        assert_eq!(tracks[1].id, "TR02");
        assert_eq!(tracks[1].steps.len(), 2);
        assert_eq!(tracks[2].id, "TR03");
        assert_eq!(tracks[2].steps.len(), 1);
    }

    #[test]
    fn test_parse_dependencies() {
        let plan = "\
## Dependencies Between Tracks

TR02 depends on TR01
TR03 depends on TR01

## TR01 — Foundation

### ST01 — Bootstrap

Initial setup.

## TR02 — CLI

### ST01 — Add CLI

Argument parsing.

## TR03 — Testing

### ST01 — Tests

Write tests.
";
        let tracks = parse_plan(plan);
        assert_eq!(tracks.len(), 3);

        let tr01 = tracks.iter().find(|t| t.id == "TR01").unwrap();
        assert!(tr01.depends_on.is_empty());

        let tr02 = tracks.iter().find(|t| t.id == "TR02").unwrap();
        assert_eq!(tr02.depends_on, vec!["TR01"]);

        let tr03 = tracks.iter().find(|t| t.id == "TR03").unwrap();
        assert_eq!(tr03.depends_on, vec!["TR01"]);
    }

    #[test]
    fn test_parse_empty_plan() {
        let tracks = parse_plan("");
        assert!(tracks.is_empty());

        let tracks = parse_plan("# Some preamble\n\nNo tracks here.\n");
        assert!(tracks.is_empty());
    }

    #[test]
    fn test_parse_step_content() {
        let plan = "\
## TR01 — State Machine

### ST01 — Initialize state

**Must-haves:**
- state.rs exists
- ProjectState serializes to YAML

**Action:**
Create src/state.rs with the ProjectState struct.

### ST02 — Add transitions

**Must-haves:**
- mark_complete works
- status reflects change

**Action:**
Implement mark_complete on ProjectState.
";
        let tracks = parse_plan(plan);
        assert_eq!(tracks.len(), 1);
        let track = &tracks[0];
        assert_eq!(track.steps.len(), 2);

        let st01 = &track.steps[0];
        assert!(st01.content.contains("Must-haves"));
        assert!(st01.content.contains("state.rs exists"));
        assert!(st01.content.contains("Create src/state.rs"));

        let st02 = &track.steps[1];
        assert!(st02.content.contains("mark_complete works"));
        assert!(st02.content.contains("Implement mark_complete"));
    }

    #[test]
    fn test_parse_roadmap_single_phase() {
        let text = "## P001 — Bootstrap\n\nSet up the project skeleton.\n";
        let phases = parse_roadmap(text);
        assert_eq!(phases.len(), 1);
        assert_eq!(phases[0].id, "P001");
        assert_eq!(phases[0].title, "Bootstrap");
        assert_eq!(phases[0].description, "Set up the project skeleton.");
    }

    #[test]
    fn test_parse_roadmap_multiple_phases() {
        let text = "\
## P001 — Bootstrap

Initialize project.

## P002 — Core Logic

Build the engine.

## P003 — Polish

Final touches.
";
        let phases = parse_roadmap(text);
        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0].id, "P001");
        assert_eq!(phases[0].title, "Bootstrap");
        assert!(phases[0].description.contains("Initialize project."));
        assert_eq!(phases[1].id, "P002");
        assert_eq!(phases[2].id, "P003");
    }

    #[test]
    fn test_parse_roadmap_filters_completed() {
        let text = "\
## P001 — Bootstrap [COMPLETED]

Done already.

## P002 — Core Logic

Still to do.
";
        let phases = parse_roadmap(text);
        assert_eq!(phases.len(), 1);
        assert_eq!(phases[0].id, "P002");
    }

    #[test]
    fn test_parse_roadmap_normalizes_id_to_uppercase() {
        // Regex requires P\d{3} so lower-case 'p' won't match — normalization applies to the
        // captured group only (e.g. if Claude emits "P001" we still uppercase to be safe).
        let text = "## P001 — Title\n\nDesc.\n";
        let phases = parse_roadmap(text);
        assert_eq!(phases.len(), 1);
        assert_eq!(phases[0].id, "P001"); // already uppercase, to_uppercase() is a no-op
    }

    #[test]
    fn test_parse_roadmap_em_dash_and_hyphen() {
        let text = "## P001 - Hyphen Title\n\n## P002 — Em-dash Title\n";
        let phases = parse_roadmap(text);
        assert_eq!(phases.len(), 2);
        assert_eq!(phases[0].title, "Hyphen Title");
        assert_eq!(phases[1].title, "Em-dash Title");
    }

    #[test]
    fn test_parse_roadmap_empty_returns_empty() {
        let phases = parse_roadmap("");
        assert!(phases.is_empty());
    }

    #[test]
    fn test_roadmap_regeneration_preserves_completed() {
        use std::fs;
        use tempfile::TempDir;

        let dir = TempDir::new().unwrap();
        let mz_path = dir.path().join(".mz");
        fs::create_dir_all(&mz_path).unwrap();
        state::set_test_mz_dir(Some(mz_path));

        // Set up state: P001 has tracks (simulates completed), P002 is skeleton (no tracks)
        let initial_state = state::ProjectState {
            name: "test".to_string(),
            description: "test project".to_string(),
            current_phase: "P001".to_string(),
            phases: vec![
                state::PhaseEntry {
                    id: "P001".to_string(),
                    title: "Foundation".to_string(),
                    tracks: vec![state::TrackEntry {
                        id: "TR01".to_string(),
                        title: "Track 1".to_string(),
                        depends_on: vec![],
                        steps: vec![state::StepEntry {
                            id: "ST01".to_string(),
                            title: "Step 1".to_string(),
                            status: state::StepStatus::Complete,
                            blocked_reason: None,
                            attempts: 1,
                        }],
                    }],
                },
                state::PhaseEntry {
                    id: "P002".to_string(),
                    title: "Old Skeleton".to_string(),
                    tracks: vec![],
                },
            ],
        };
        state::save(&initial_state).unwrap();

        // Re-plan with P001 and P003 (P002 is dropped, P003 is new)
        let new_phases = vec![
            ParsedPhase {
                id: "P001".to_string(),
                title: "Foundation".to_string(),
                description: String::new(),
            },
            ParsedPhase {
                id: "P003".to_string(),
                title: "New Phase".to_string(),
                description: String::new(),
            },
        ];
        apply_roadmap_phases(&new_phases).unwrap();

        let s = state::load().unwrap();

        // P001 untouched (has tracks — create_skeleton_phase is a no-op for it)
        let p1 = s.phases.iter().find(|p| p.id == "P001").unwrap();
        assert_eq!(p1.title, "Foundation");
        assert!(!p1.tracks.is_empty());

        // P002 removed (was skeleton, not in new plan)
        assert!(s.phases.iter().find(|p| p.id == "P002").is_none());

        // P003 created as skeleton
        let p3 = s.phases.iter().find(|p| p.id == "P003").unwrap();
        assert_eq!(p3.title, "New Phase");
        assert!(p3.tracks.is_empty());

        state::set_test_mz_dir(None);
    }
}

fn parse_and_create_steps(
    project_state: &state::ProjectState,
    phase_id: &str,
    plan_output: &str,
) -> Result<()> {
    let new_tracks = parse_and_write_tracks(phase_id, plan_output)?;

    let mut phases = project_state.phases.clone();
    let phase_entry = state::PhaseEntry {
        id: phase_id.to_string(),
        title: format!("Phase {}", phase_id),
        tracks: new_tracks,
    };

    phases.retain(|p| p.id != phase_id);
    phases.push(phase_entry);
    phases.sort_by(|a, b| a.id.cmp(&b.id));

    let mut new_state = project_state.clone();
    new_state.phases = phases;
    state::save(&new_state)?;

    Ok(())
}
