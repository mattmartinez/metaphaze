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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_substitutes_single_var() {
        let mut v = vars();
        set(&mut v, "name", "world");
        assert_eq!(render("hello {{name}}", &v), "hello world");
    }

    #[test]
    fn test_render_substitutes_multiple_vars() {
        let mut v = vars();
        set(&mut v, "a", "1");
        set(&mut v, "b", "2");
        set(&mut v, "c", "3");
        let out = render("{{a}}-{{b}}-{{c}}", &v);
        assert_eq!(out, "1-2-3");
    }

    #[test]
    fn test_render_repeated_var_replaces_all_occurrences() {
        let mut v = vars();
        set(&mut v, "x", "X");
        assert_eq!(render("{{x}} and {{x}} again", &v), "X and X again");
    }

    #[test]
    fn test_render_unknown_var_left_as_placeholder() {
        // Templates with unresolved vars: the placeholder remains in the output
        // (and a warn is logged to stderr).
        let v = vars();
        let out = render("hello {{missing}}!", &v);
        assert_eq!(out, "hello {{missing}}!");
    }

    #[test]
    fn test_render_empty_template() {
        let v = vars();
        assert_eq!(render("", &v), "");
    }

    #[test]
    fn test_render_no_placeholders() {
        let mut v = vars();
        set(&mut v, "unused", "x");
        assert_eq!(render("plain text", &v), "plain text");
    }

    #[test]
    fn test_render_value_containing_braces_is_preserved() {
        // Inlined content may legitimately contain {{...}} — render should NOT
        // re-process those after substitution (replace is one-shot per key).
        let mut v = vars();
        set(&mut v, "body", "see {{other}} for details");
        let out = render("{{body}}", &v);
        assert_eq!(out, "see {{other}} for details");
    }

    #[test]
    fn test_render_multiline_value() {
        let mut v = vars();
        set(&mut v, "block", "line1\nline2\nline3");
        let out = render("--start--\n{{block}}\n--end--", &v);
        assert_eq!(out, "--start--\nline1\nline2\nline3\n--end--");
    }

    #[test]
    fn test_render_var_with_special_chars() {
        let mut v = vars();
        set(&mut v, "msg", "$0.42 cost (50% reduction)");
        assert_eq!(render("[{{msg}}]", &v), "[$0.42 cost (50% reduction)]");
    }

    #[test]
    fn test_set_overwrites_existing_value() {
        let mut v = vars();
        set(&mut v, "k", "first");
        set(&mut v, "k", "second");
        assert_eq!(render("{{k}}", &v), "second");
    }

    #[test]
    fn test_render_compiled_template_smoke() {
        // Make sure each compiled template at least renders (no panic) with empty vars.
        let v = vars();
        let _ = render(templates::DISCUSS, &v);
        let _ = render(templates::PLAN_PHASE, &v);
        let _ = render(templates::PLAN_ROADMAP, &v);
        let _ = render(templates::PLAN_TRACK, &v);
        let _ = render(templates::EXECUTE_STEP, &v);
        let _ = render(templates::VERIFY_STEP, &v);
        let _ = render(templates::VERIFY_TRACK, &v);
        let _ = render(templates::SUMMARIZE, &v);
    }
}
