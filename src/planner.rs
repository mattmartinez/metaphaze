use anyhow::Result;
use regex::Regex;
use std::fs;

use crate::{claude, prompt, state};

pub fn run(project_state: &state::ProjectState, phase_id: &str) -> Result<()> {
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

    println!("Generating phase plan...\n");
    let result = claude::run(opts)?;

    // Write ROADMAP.md
    let ph_dir = state::phase_dir(phase_id);
    fs::create_dir_all(&ph_dir)?;
    fs::write(state::roadmap_path(phase_id), &result)?;

    // Parse the plan output and create step files + update state
    parse_and_create_steps(project_state, phase_id, &result)?;

    println!("Plan written to {}/", ph_dir.display());
    println!("\nReview the plan, then run `mz auto` to start execution.");
    Ok(())
}

pub fn replan(project_state: &state::ProjectState, phase_id: &str, decision: &str) -> Result<()> {
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

    println!("Re-planning remaining steps...\n");
    let result = claude::run(opts)?;

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
            println!("\nRe-plan complete. ROADMAP.md updated.");
            println!("Run `mz status --detail` to see the updated plan.");
            return Ok(());
        }
    }

    // Phase not found in state yet — shouldn't happen after a steer, but handle gracefully
    println!("\nWarning: phase {} not found in state; no merge performed.", phase_id);
    Ok(())
}

/// Parse tracks and steps from plan output, write plan files to disk, return track entries.
fn parse_and_write_tracks(phase_id: &str, plan_output: &str) -> Result<Vec<state::TrackEntry>> {
    let track_re = Regex::new(r"(?m)^## (TR\d+)\s*[—-]\s*(.+)$")?;
    let step_re = Regex::new(r"(?m)^### (ST\d+)\s*[—-]\s*(.+)$")?;
    let dep_re = Regex::new(r"TR(\d+)\s+depends on\s+TR(\d+)")?;

    // Extract the preamble (content before the first ## TR header) for dependency parsing
    let first_track_pos = track_re.find(plan_output).map(|m| m.start()).unwrap_or(0);
    let preamble = &plan_output[..first_track_pos];

    // Build a map of track_id -> depends_on from the preamble
    let mut dep_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for caps in dep_re.captures_iter(preamble) {
        let dependent = format!("TR{}", &caps[1]);
        let dependency = format!("TR{}", &caps[2]);
        dep_map.entry(dependent).or_default().push(dependency);
    }

    let mut tracks = vec![];
    let track_matches: Vec<_> = track_re.find_iter(plan_output).collect();

    for (i, track_match) in track_matches.iter().enumerate() {
        let caps = track_re.captures(track_match.as_str()).unwrap();
        let track_id = caps[1].to_string();
        let track_title = caps[2].trim().to_string();

        let track_start = track_match.start();
        let track_end = if i + 1 < track_matches.len() {
            track_matches[i + 1].start()
        } else {
            plan_output.len()
        };
        let track_content = &plan_output[track_start..track_end];

        let track_dir = state::track_dir(phase_id, &track_id);
        let steps_dir = track_dir.join("steps");
        fs::create_dir_all(&steps_dir)?;

        let depends_on = dep_map.remove(&track_id).unwrap_or_default();

        let mut track_entry = state::TrackEntry {
            id: track_id.clone(),
            title: track_title.clone(),
            steps: vec![],
            depends_on,
        };

        let step_matches: Vec<_> = step_re.find_iter(track_content).collect();

        for (j, step_match) in step_matches.iter().enumerate() {
            let scaps = step_re.captures(step_match.as_str()).unwrap();
            let step_id = scaps[1].to_string();
            let step_title = scaps[2].trim().to_string();

            let step_start = step_match.start();
            let step_end = if j + 1 < step_matches.len() {
                step_matches[j + 1].start()
            } else {
                track_content.len()
            };
            let step_content = &track_content[step_start..step_end];

            let plan_path = state::step_plan_path(phase_id, &track_id, &step_id);
            fs::write(&plan_path, step_content.trim())?;

            track_entry.steps.push(state::StepEntry {
                id: step_id,
                title: step_title,
                status: state::StepStatus::Pending,
                blocked_reason: None,
                attempts: 0,
            });
        }

        fs::write(track_dir.join("PLAN.md"), track_content.trim())?;
        tracks.push(track_entry);
    }

    Ok(tracks)
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
