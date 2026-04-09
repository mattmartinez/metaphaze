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

    // Gather current state for context
    let current_state = serde_yaml::to_string(project_state)?;

    let prompt_text = format!(
        "# Re-planning Required\n\n\
         A new decision has been made:\n\n> {}\n\n\
         ## Current Project\n\n{}\n\n\
         ## Current State\n\n```yaml\n{}\n```\n\n\
         ## Decisions\n\n{}\n\n\
         ## Context\n\n{}\n\n\
         Re-plan the REMAINING (pending) steps for phase {}. \
         Keep completed steps as-is. Output updated step plans for any \
         steps that need to change. Use the same format as the original plan.",
        decision, project_md, current_state, decisions, context, phase_id,
    );

    let opts = claude::ClaudeOptions::new(prompt_text)
        .model("opus")
        .max_turns(40);

    println!("Re-planning...\n");
    let result = claude::run(opts)?;
    println!("{}", result);
    Ok(())
}

fn parse_and_create_steps(
    project_state: &state::ProjectState,
    phase_id: &str,
    plan_output: &str,
) -> Result<()> {
    // Parse tracks and steps from the plan output
    // Expected format:
    //   ## TR01 — Track Title
    //   ### ST01 — Step Title
    //   (step plan content...)

    let track_re = Regex::new(r"(?m)^## (TR\d+)\s*[—-]\s*(.+)$")?;
    let step_re = Regex::new(r"(?m)^### (ST\d+)\s*[—-]\s*(.+)$")?;

    let mut phases = project_state.phases.clone();
    let mut phase_entry = state::PhaseEntry {
        id: phase_id.to_string(),
        title: format!("Phase {}", phase_id),
        tracks: vec![],
    };

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

        // Create track directory
        let track_dir = state::track_dir(phase_id, &track_id);
        let steps_dir = track_dir.join("steps");
        fs::create_dir_all(&steps_dir)?;

        let mut track_entry = state::TrackEntry {
            id: track_id.clone(),
            title: track_title.clone(),
            steps: vec![],
        };

        // Parse steps within this track
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

            // Write PLAN.md for this step
            let plan_path = state::step_plan_path(phase_id, &track_id, &step_id);
            fs::write(&plan_path, step_content.trim())?;

            track_entry.steps.push(state::StepEntry {
                id: step_id,
                title: step_title,
                status: state::StepStatus::Pending,
                blocked_reason: None,
            });
        }

        // Write track PLAN.md
        fs::write(track_dir.join("PLAN.md"), track_content.trim())?;

        phase_entry.tracks.push(track_entry);
    }

    // Update state with the new phase
    phases.retain(|p| p.id != phase_id);
    phases.push(phase_entry);
    phases.sort_by(|a, b| a.id.cmp(&b.id));

    let mut new_state = project_state.clone();
    new_state.phases = phases;
    state::save(&new_state)?;

    Ok(())
}
