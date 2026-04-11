//! Per-step context injection.
//!
//! By default the executor bundles a phase's full project context, all
//! decisions, and every prior step summary in the current track plus every
//! dependency track. As a project grows this becomes expensive — and noisy:
//! the model has to scan dozens of summaries to find the two it actually
//! needs.
//!
//! Step plans can opt into fine-grained control via an optional YAML
//! frontmatter block at the very top of the PLAN.md file:
//!
//! ```markdown
//! ---
//! context:
//!   include_summaries:
//!     - TR01/ST02
//!     - TR03/ST01
//!   include_files:
//!     - src/foo.rs
//!     - docs/architecture.md
//!   exclude_default_summaries: false
//! ---
//!
//! ## Action
//! ...
//! ```
//!
//! Semantics:
//! - `include_summaries`: explicit allow-list of `TRxx/STxx` summaries to
//!   bundle. When the list is non-empty AND `exclude_default_summaries` is
//!   true, only these summaries are injected. When the list is non-empty
//!   AND `exclude_default_summaries` is false, the explicit list is *added
//!   on top of* the default dependency-track collection (handy for pulling
//!   in a sibling track that isn't formally a dependency).
//! - `include_files`: paths (relative to repo root) whose contents should
//!   be injected verbatim into the prompt. Use sparingly — files larger
//!   than a few KB blow up the cacheable prefix.
//! - `exclude_default_summaries`: when true, the executor skips its
//!   automatic dependency-summary collection entirely. Combine with
//!   `include_summaries` to get a strict allow-list.
//!
//! When the frontmatter is absent (the common case), `parse` returns
//! `StepContextSpec::default()` and the executor falls back to its prior
//! "include everything" behavior — so existing projects don't need to be
//! migrated.

use serde::{Deserialize, Serialize};

/// Parsed `context:` frontmatter block from a step PLAN.md file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepContextSpec {
    /// Explicit `TR/ST` identifiers to inject. Each entry must be in the
    /// form `TRxx/STxx`. Unrecognized entries are silently ignored at use
    /// time so a typo never blocks a step.
    #[serde(default)]
    pub include_summaries: Vec<String>,

    /// Repo-relative file paths to inject verbatim.
    #[serde(default)]
    pub include_files: Vec<String>,

    /// Skip the executor's default dependency-summary bundling. Combine
    /// with `include_summaries` for a strict allow-list.
    #[serde(default)]
    pub exclude_default_summaries: bool,
}

/// Wrapper used purely for serde to round-trip the `context:` key.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Frontmatter {
    #[serde(default)]
    context: Option<StepContextSpec>,
}

/// Parse a step plan body for an optional YAML frontmatter block. Returns
/// the spec if present, or `StepContextSpec::default()` (which is
/// equivalent to the legacy "include everything" behavior) when no
/// frontmatter exists or when it doesn't contain a `context:` key.
///
/// A frontmatter block is delimited by `---` lines at the very top of the
/// file. Anything before the first `---` (e.g. a UTF-8 BOM, blank lines)
/// is tolerated; once we find an opening `---` we collect lines until the
/// closing `---` and parse the body as YAML.
///
/// Parse failures are silently downgraded to defaults — a malformed
/// frontmatter must NEVER block a step from running. Callers can check
/// `spec.is_empty()` to detect "no spec / fell back to defaults".
pub fn parse(plan_text: &str) -> StepContextSpec {
    let trimmed = plan_text.trim_start_matches('\u{feff}'); // strip optional BOM
    let mut lines = trimmed.lines();

    // Skip leading blank lines, then expect a `---` opener.
    let mut opened = false;
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            continue;
        }
        if line.trim() == "---" {
            opened = true;
        }
        break;
    }
    if !opened {
        return StepContextSpec::default();
    }

    // Collect frontmatter body until the closing `---`.
    let mut body = String::new();
    let mut closed = false;
    for line in lines {
        if line.trim() == "---" {
            closed = true;
            break;
        }
        body.push_str(line);
        body.push('\n');
    }
    if !closed {
        // Unterminated frontmatter — refuse to interpret it (the rest of
        // the file is plan content the model should see, not config).
        return StepContextSpec::default();
    }

    match serde_yaml::from_str::<Frontmatter>(&body) {
        Ok(fm) => fm.context.unwrap_or_default(),
        Err(_) => StepContextSpec::default(),
    }
}

/// Strip the YAML frontmatter (if any) from a plan body, returning the
/// remaining markdown content. The executor passes the stripped content to
/// the model so the user-facing step instructions don't include the
/// machine-readable config block. If there's no frontmatter the input is
/// returned unchanged.
pub fn strip_frontmatter(plan_text: &str) -> &str {
    let trimmed = plan_text.trim_start_matches('\u{feff}');
    let bytes_len = trimmed.len();
    let mut cursor = 0usize;

    // Skip leading blank lines and find the first non-blank line. If it
    // isn't `---`, there's no frontmatter and we hand back the input as-is.
    let mut saw_open = false;
    while cursor < bytes_len {
        let nl = trimmed[cursor..]
            .find('\n')
            .map(|i| cursor + i)
            .unwrap_or(bytes_len);
        let line = &trimmed[cursor..nl];
        let stripped = line.trim();
        if stripped.is_empty() {
            cursor = nl + 1;
            continue;
        }
        if stripped == "---" {
            saw_open = true;
            cursor = nl + 1;
        }
        break;
    }
    if !saw_open {
        return trimmed;
    }

    // Walk forward to the closing `---` and return the slice that follows.
    while cursor < bytes_len {
        let nl = trimmed[cursor..]
            .find('\n')
            .map(|i| cursor + i)
            .unwrap_or(bytes_len);
        let line = &trimmed[cursor..nl];
        if line.trim() == "---" {
            return trimmed[(nl + 1).min(bytes_len)..].trim_start_matches('\n');
        }
        cursor = nl + 1;
    }

    // Unterminated frontmatter — match parse()'s fall-back: treat the body
    // as plain content rather than silently dropping it.
    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_no_frontmatter_returns_default() {
        let plan = "## Action\nDo the thing\n";
        let spec = parse(plan);
        assert!(spec.include_summaries.is_empty());
        assert!(spec.include_files.is_empty());
        assert!(!spec.exclude_default_summaries);
    }

    #[test]
    fn test_parse_full_frontmatter() {
        let plan = "---\ncontext:\n  include_summaries:\n    - TR01/ST01\n    - TR02/ST03\n  include_files:\n    - src/foo.rs\n  exclude_default_summaries: true\n---\n\n## Action\nDo it\n";
        let spec = parse(plan);
        assert_eq!(spec.include_summaries, vec!["TR01/ST01", "TR02/ST03"]);
        assert_eq!(spec.include_files, vec!["src/foo.rs"]);
        assert!(spec.exclude_default_summaries);
    }

    #[test]
    fn test_parse_partial_frontmatter() {
        // Only include_files specified — other fields stay at default.
        let plan = "---\ncontext:\n  include_files:\n    - docs/api.md\n---\n## Action\n";
        let spec = parse(plan);
        assert_eq!(spec.include_files, vec!["docs/api.md"]);
        assert!(spec.include_summaries.is_empty());
        assert!(!spec.exclude_default_summaries);
    }

    #[test]
    fn test_parse_frontmatter_without_context_key_returns_default() {
        // The YAML block is valid but doesn't contain a `context:` key.
        // This is the case for plans that have other frontmatter
        // (e.g. a planner-emitted `id: ST01`).
        let plan = "---\nid: ST01\ntitle: Foo\n---\n## Action\n";
        let spec = parse(plan);
        assert_eq!(spec, StepContextSpec::default());
    }

    #[test]
    fn test_parse_malformed_yaml_falls_back_to_default() {
        // Tabs inside YAML mappings are an error — must NOT panic or block
        // the executor.
        let plan = "---\ncontext:\n\tinclude_files: [a, b]\n---\n## Action\n";
        let spec = parse(plan);
        assert_eq!(spec, StepContextSpec::default());
    }

    #[test]
    fn test_parse_unterminated_frontmatter_returns_default() {
        let plan = "---\ncontext:\n  include_files:\n    - foo\n## Action\n";
        let spec = parse(plan);
        assert_eq!(spec, StepContextSpec::default());
    }

    #[test]
    fn test_parse_with_leading_blank_lines() {
        let plan = "\n\n---\ncontext:\n  include_summaries:\n    - TR01/ST01\n---\n";
        let spec = parse(plan);
        assert_eq!(spec.include_summaries, vec!["TR01/ST01"]);
    }

    #[test]
    fn test_parse_with_bom() {
        let plan = "\u{feff}---\ncontext:\n  include_files:\n    - foo.rs\n---\n";
        let spec = parse(plan);
        assert_eq!(spec.include_files, vec!["foo.rs"]);
    }

    #[test]
    fn test_strip_frontmatter_removes_block() {
        let plan = "---\ncontext:\n  include_files:\n    - foo.rs\n---\n## Action\nDo it\n";
        let stripped = strip_frontmatter(plan);
        assert_eq!(stripped, "## Action\nDo it\n");
    }

    #[test]
    fn test_strip_frontmatter_no_frontmatter_unchanged() {
        let plan = "## Action\nDo it\n";
        assert_eq!(strip_frontmatter(plan), plan);
    }

    #[test]
    fn test_strip_frontmatter_unterminated_returns_input() {
        let plan = "---\ncontext: oops\n## Action\n";
        // Same fall-back behavior as parse() — treat the whole thing as content.
        assert_eq!(strip_frontmatter(plan), plan);
    }

    #[test]
    fn test_strip_frontmatter_then_parse_roundtrip() {
        // The executor's flow: parse() to get the spec, strip_frontmatter()
        // to get the plan body it shows the model. Both must agree about
        // where the frontmatter ends.
        let plan = "---\ncontext:\n  include_files: [a.rs]\n---\n# Step Title\n\nDo the thing\n";
        let spec = parse(plan);
        let body = strip_frontmatter(plan);
        assert_eq!(spec.include_files, vec!["a.rs"]);
        assert_eq!(body, "# Step Title\n\nDo the thing\n");
    }

}
