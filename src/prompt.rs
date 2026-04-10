use std::collections::HashMap;

// Prompt templates compiled into the binary
#[allow(dead_code)]
pub mod templates {
    pub const DISCUSS: &str = include_str!("../prompts/discuss.md");
    pub const PLAN_PHASE: &str = include_str!("../prompts/plan_phase.md");
    pub const PLAN_ROADMAP: &str = include_str!("../prompts/plan_roadmap.md");
    pub const PLAN_TRACK: &str = include_str!("../prompts/plan_track.md");
    pub const EXECUTE_STEP: &str = include_str!("../prompts/execute_step.md");
    pub const VERIFY_STEP: &str = include_str!("../prompts/verify_step.md");
    pub const VERIFY_TRACK: &str = include_str!("../prompts/verify_track.md");
    pub const SUMMARIZE: &str = include_str!("../prompts/summarize.md");
}

pub fn render(template: &str, vars: &HashMap<&str, String>) -> String {
    // BUG-28: check for unresolved variables in the TEMPLATE itself, not the
    // rendered output (since inlined content may legitimately contain {{...}})
    if let Ok(re) = regex::Regex::new(r"\{\{(\w+)\}\}") {
        for cap in re.captures_iter(template) {
            let key = &cap[1];
            if !vars.contains_key(key) {
                eprintln!("[warn] unresolved template variable: {{{{{}}}}}", key);
            }
        }
    }
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

pub fn vars<'a>() -> HashMap<&'a str, String> {
    HashMap::new()
}

pub fn set<'a>(map: &mut HashMap<&'a str, String>, key: &'a str, value: impl Into<String>) {
    map.insert(key, value.into());
}
