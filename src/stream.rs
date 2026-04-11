use serde::Deserialize;
use serde_json::Value;

/// A single event from Claude Code's `--output-format stream-json` NDJSON stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    /// Partial assistant text chunk.
    Assistant {
        message: AssistantMessage,
    },
    /// Token-level delta from the model (stream-json verbose mode).
    ContentBlockDelta {
        delta: DeltaContent,
    },
    /// Claude is invoking a tool.
    ToolUse {
        tool: String,
        input: Value,
    },
    /// Result returned from a tool invocation.
    ToolResult {
        tool: String,
        #[serde(default, rename = "content")]
        _content: Option<String>,
        #[serde(default, rename = "output")]
        _output: Option<String>,
    },
    /// Final event — contains the assembled response and run metadata.
    Result {
        result: String,
        #[serde(default)]
        cost_usd: Option<f64>,
        #[serde(default)]
        num_turns: Option<u32>,
        #[serde(default)]
        input_tokens: Option<u64>,
        #[serde(default)]
        output_tokens: Option<u64>,
    },
    /// An error occurred during the run.
    Error {
        error: String,
    },
    /// User message (tool results sent back to Claude — ignored for display).
    User {
        #[serde(flatten)]
        _extra: Value,
    },
    /// System-level message (ignored for display).
    System {
        #[serde(flatten)]
        _extra: Value,
    },
    /// First event in a stream — carries the model name.
    MessageStart {
        message: MessageStartData,
    },
}

impl StreamEvent {
    /// Return a one-line summary for a `ToolUse` event: "tool_name key_arg".
    /// For file-oriented tools, extracts the path/pattern from `input`.
    /// Returns `None` for non-ToolUse events.
    pub fn tool_use_summary(&self) -> Option<String> {
        if let StreamEvent::ToolUse { tool, input } = self {
            let key_arg = match tool.as_str() {
                "Read" | "Edit" | "Write" | "NotebookEdit" => {
                    input.get("file_path").and_then(|v| v.as_str()).map(|s| s.to_string())
                }
                "Bash" => {
                    input.get("command").and_then(|v| v.as_str()).map(|s| s.to_string())
                }
                "Grep" => {
                    input.get("path").and_then(|v| v.as_str()).map(|s| s.to_string())
                        .or_else(|| input.get("pattern").and_then(|v| v.as_str()).map(|s| s.to_string()))
                }
                "Glob" => {
                    input.get("pattern").and_then(|v| v.as_str()).map(|s| s.to_string())
                        .or_else(|| input.get("path").and_then(|v| v.as_str()).map(|s| s.to_string()))
                }
                _ => None,
            };
            let summary = match key_arg {
                Some(arg) => {
                    let s = format!("{} {}", tool, arg);
                    if s.chars().count() > 80 {
                        let truncated: String = s.chars().take(79).collect();
                        format!("{}…", truncated)
                    } else {
                        s
                    }
                }
                None => tool.clone(),
            };
            return Some(summary);
        }
        None
    }
}

/// Standalone summary helper — same logic as `StreamEvent::tool_use_summary` but
/// callable with just a name + input (used when tool_use blocks are extracted from
/// inside `assistant.message.content` rather than as top-level events).
pub fn tool_use_summary_from_parts(name: &str, input: &serde_json::Value) -> String {
    let key_arg = match name {
        "Read" | "Edit" | "Write" | "NotebookEdit" => {
            input.get("file_path").and_then(|v| v.as_str()).map(|s| s.to_string())
        }
        "Bash" => {
            input.get("command").and_then(|v| v.as_str()).map(|s| s.to_string())
        }
        "Grep" => {
            input.get("path").and_then(|v| v.as_str()).map(|s| s.to_string())
                .or_else(|| input.get("pattern").and_then(|v| v.as_str()).map(|s| s.to_string()))
        }
        "Glob" => {
            input.get("pattern").and_then(|v| v.as_str()).map(|s| s.to_string())
                .or_else(|| input.get("path").and_then(|v| v.as_str()).map(|s| s.to_string()))
        }
        _ => None,
    };
    let summary = match key_arg {
        Some(arg) => {
            let s = format!("{} {}", name, arg);
            if s.chars().count() > 80 {
                let truncated: String = s.chars().take(79).collect();
                format!("{}…", truncated)
            } else {
                s
            }
        }
        None => name.to_string(),
    };
    summary
}

/// The `message` field inside an `assistant` event.
#[derive(Debug, Clone, Deserialize)]
pub struct AssistantMessage {
    pub content: Vec<ContentBlock>,
}

/// A content block within an assistant message.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolUse {
        #[serde(rename = "name")]
        name: String,
        #[serde(default)]
        input: serde_json::Value,
    },
    #[serde(other)]
    Unknown,
}

/// The `delta` field inside a `content_block_delta` event.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DeltaContent {
    TextDelta { text: String },
    #[serde(other)]
    Unknown,
}

/// The `message` field inside a `message_start` event.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageStartData {
    pub model: String,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens: Option<u64>,
}

/// Parse a single NDJSON line from the stream-json output.
/// Returns `None` for unrecognized or malformed lines.
pub fn parse_stream_line(line: &str) -> Option<StreamEvent> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    match serde_json::from_str::<StreamEvent>(trimmed) {
        Ok(event) => Some(event),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_block_delta_text() {
        let line = r#"{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::ContentBlockDelta { .. }));
        if let StreamEvent::ContentBlockDelta { delta: DeltaContent::TextDelta { text } } = event {
            assert_eq!(text, "Hello");
        } else {
            panic!("expected text delta");
        }
    }

    #[test]
    fn test_content_block_delta_non_text() {
        // input_json_delta has no `text` field — should parse to Unknown delta
        let line = r#"{"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"{}"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::ContentBlockDelta { .. }));
        if let StreamEvent::ContentBlockDelta { delta } = event {
            assert!(!matches!(delta, DeltaContent::TextDelta { .. }));
        }
    }

    #[test]
    fn test_assistant_event_still_parses() {
        let line = r#"{"type":"assistant","message":{"content":[{"type":"text","text":"Hi there"}]}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::Assistant { .. }));
    }

    #[test]
    fn test_tool_use_event_still_parses() {
        let line = r#"{"type":"tool_use","tool":"Read","input":{"file_path":"src/main.rs"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::ToolUse { .. }));
        assert_eq!(event.tool_use_summary(), Some("Read src/main.rs".to_string()));
    }

    #[test]
    fn test_tool_result_event_still_parses() {
        let line = r#"{"type":"tool_result","tool":"Read","content":"file contents"}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::ToolResult { .. }));
    }

    #[test]
    fn test_result_event_still_parses() {
        let line = r#"{"type":"result","result":"final answer","cost_usd":0.01,"duration_ms":1200,"num_turns":3}"#;
        let event = parse_stream_line(line).expect("should parse");
        if let StreamEvent::Result { result, cost_usd, num_turns, .. } = event {
            assert_eq!(result, "final answer");
            assert_eq!(cost_usd, Some(0.01));
            assert_eq!(num_turns, Some(3));
        } else {
            panic!("expected Result variant");
        }
    }

    #[test]
    fn test_error_event_still_parses() {
        let line = r#"{"type":"error","error":"rate limit exceeded"}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::Error { .. }));
    }

    #[test]
    fn test_malformed_line_returns_none() {
        assert!(parse_stream_line("not json at all").is_none());
        assert!(parse_stream_line("").is_none());
        assert!(parse_stream_line("   ").is_none());
    }

    #[test]
    fn test_unknown_event_type_returns_none() {
        let line = r#"{"type":"rate_limit_event","data":{}}"#;
        assert!(parse_stream_line(line).is_none());
    }

    #[test]
    fn test_tool_use_summary_grep_with_path() {
        let line = r#"{"type":"tool_use","tool":"Grep","input":{"pattern":"fn main","path":"src/"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), Some("Grep src/".to_string()));
    }

    #[test]
    fn test_tool_use_summary_glob() {
        let line = r#"{"type":"tool_use","tool":"Glob","input":{"pattern":"**/*.rs"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), Some("Glob **/*.rs".to_string()));
    }

    #[test]
    fn test_tool_use_summary_unknown_tool() {
        let line = r#"{"type":"tool_use","tool":"UnknownTool","input":{"irrelevant":"data"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), Some("UnknownTool".to_string()));
    }

    #[test]
    fn test_tool_use_summary_bash_with_command() {
        let line = r#"{"type":"tool_use","tool":"Bash","input":{"command":"git status && git diff"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), Some("Bash git status && git diff".to_string()));
    }

    #[test]
    fn test_tool_use_summary_bash_no_command() {
        let line = r#"{"type":"tool_use","tool":"Bash","input":{}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), Some("Bash".to_string()));
    }

    #[test]
    fn test_tool_use_summary_truncation() {
        // command of 77 chars + "Bash " (5) = 82 chars total, which exceeds 80
        let long_cmd = "a".repeat(77);
        let input = serde_json::json!({"command": long_cmd});
        let event = StreamEvent::ToolUse { tool: "Bash".to_string(), input };
        let summary = event.tool_use_summary().expect("should return summary");
        // result should be exactly 80 chars (79 chars + "…" which is 1 char but 3 bytes)
        assert_eq!(summary.chars().count(), 80);
        assert!(summary.ends_with('…'));
    }

    #[test]
    fn test_tool_use_summary_no_truncation_at_80() {
        // "Bash " (5) + 75 chars = 80 chars total, should NOT be truncated
        let cmd = "b".repeat(75);
        let input = serde_json::json!({"command": cmd});
        let event = StreamEvent::ToolUse { tool: "Bash".to_string(), input };
        let summary = event.tool_use_summary().expect("should return summary");
        assert_eq!(summary.chars().count(), 80);
        assert!(!summary.ends_with('…'));
    }

    #[test]
    fn test_tool_use_summary_on_non_tool_event() {
        let line = r#"{"type":"error","error":"oops"}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), None);
    }

    #[test]
    fn test_message_start_parses_model() {
        let line = r#"{"type":"message_start","message":{"model":"claude-sonnet-4-20250514","id":"msg_xxx","type":"message","role":"assistant","content":[],"stop_reason":null,"stop_sequence":null,"usage":{"input_tokens":10,"output_tokens":0}}}"#;
        let event = parse_stream_line(line).expect("should parse");
        if let StreamEvent::MessageStart { message } = event {
            assert_eq!(message.model, "claude-sonnet-4-20250514");
        } else {
            panic!("expected MessageStart");
        }
    }

    #[test]
    fn test_message_start_minimal() {
        let line = r#"{"type":"message_start","message":{"model":"claude-opus-4-6"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        if let StreamEvent::MessageStart { message } = event {
            assert_eq!(message.model, "claude-opus-4-6");
        } else {
            panic!("expected MessageStart");
        }
    }

    #[test]
    fn test_message_start_with_usage() {
        let line = r#"{"type":"message_start","message":{"model":"claude-sonnet-4-6","usage":{"input_tokens":1234,"output_tokens":56}}}"#;
        let event = parse_stream_line(line).expect("should parse");
        if let StreamEvent::MessageStart { message } = event {
            let usage = message.usage.expect("usage should be present");
            assert_eq!(usage.input_tokens, Some(1234));
            assert_eq!(usage.output_tokens, Some(56));
        } else {
            panic!("expected MessageStart variant");
        }
    }

    #[test]
    fn test_message_start_without_usage() {
        let line = r#"{"type":"message_start","message":{"model":"claude-opus-4-6"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        if let StreamEvent::MessageStart { message } = event {
            assert!(message.usage.is_none());
        } else {
            panic!("expected MessageStart variant");
        }
    }

    #[test]
    fn test_result_with_tokens() {
        let line = r#"{"type":"result","result":"done","cost_usd":0.05,"duration_ms":2000,"num_turns":4,"input_tokens":8000,"output_tokens":1500}"#;
        let event = parse_stream_line(line).expect("should parse");
        if let StreamEvent::Result { result, cost_usd, num_turns, input_tokens, output_tokens } = event {
            assert_eq!(result, "done");
            assert_eq!(cost_usd, Some(0.05));
            assert_eq!(num_turns, Some(4));
            assert_eq!(input_tokens, Some(8000));
            assert_eq!(output_tokens, Some(1500));
        } else {
            panic!("expected Result variant");
        }
    }

    // ── Mock-claude integration: pipe a real subprocess through parse_stream_line ──

    fn mock_claude_binary() -> std::path::PathBuf {
        // unit test exe is at target/<profile>/deps/<bin>-<hash> — go up two
        // dirs to land in target/<profile>/ where mock_claude lives.
        let mut p = std::env::current_exe().expect("current_exe");
        p.pop(); // deps/
        p.pop(); // <profile>/
        p.push("mock_claude");
        if !p.exists() {
            panic!(
                "mock_claude binary not found at {} — run `cargo build --bin mock_claude` first",
                p.display()
            );
        }
        p
    }

    #[test]
    fn test_mock_claude_full_stream_parses() {
        use std::process::Command;
        let out = Command::new(mock_claude_binary())
            .env("MOCK_CLAUDE_TEXT", "hello from mock")
            .env("MOCK_CLAUDE_RESULT", "final answer")
            .env("MOCK_CLAUDE_COST", "0.07")
            .env("MOCK_CLAUDE_TURNS", "3")
            .output()
            .expect("spawn mock_claude");
        assert!(out.status.success(), "mock should exit 0");
        let stdout = String::from_utf8_lossy(&out.stdout);
        let events: Vec<_> = stdout.lines().filter_map(parse_stream_line).collect();
        // Expect: message_start, assistant, result
        assert_eq!(events.len(), 3, "expected 3 events, got {}: {}", events.len(), stdout);
        assert!(matches!(events[0], StreamEvent::MessageStart { .. }));
        assert!(matches!(events[1], StreamEvent::Assistant { .. }));
        if let StreamEvent::Result { ref result, cost_usd, num_turns, .. } = events[2] {
            assert_eq!(result, "final answer");
            assert_eq!(cost_usd, Some(0.07));
            assert_eq!(num_turns, Some(3));
        } else {
            panic!("expected Result event last, got {:?}", events[2]);
        }
    }

    #[test]
    fn test_mock_claude_tool_use_mode_stream_parses() {
        use std::process::Command;
        let out = Command::new(mock_claude_binary())
            .env("MOCK_CLAUDE_MODE", "tool_use")
            .env("MOCK_CLAUDE_TEXT", "looking up")
            .env("MOCK_CLAUDE_TOOL", "Read")
            .env("MOCK_CLAUDE_TOOL_ARG", "src/main.rs")
            .output()
            .expect("spawn mock_claude");
        assert!(out.status.success());
        let stdout = String::from_utf8_lossy(&out.stdout);
        let events: Vec<_> = stdout.lines().filter_map(parse_stream_line).collect();
        // Expect: message_start, assistant (with tool_use block), tool_result, result
        assert_eq!(events.len(), 4, "got: {}", stdout);
        assert!(matches!(events[0], StreamEvent::MessageStart { .. }));
        // Assistant event should carry one Text block + one ToolUse block
        if let StreamEvent::Assistant { ref message } = events[1] {
            let has_text = message.content.iter().any(|b| matches!(b, ContentBlock::Text { .. }));
            let tool_block = message.content.iter().find_map(|b| {
                if let ContentBlock::ToolUse { name, input } = b {
                    Some((name.clone(), input.clone()))
                } else {
                    None
                }
            });
            assert!(has_text, "assistant should have text block");
            let (name, input) = tool_block.expect("should have tool_use block");
            assert_eq!(name, "Read");
            assert_eq!(input.get("file_path").and_then(|v| v.as_str()), Some("src/main.rs"));
        } else {
            panic!("expected Assistant event at index 1");
        }
        assert!(matches!(events[2], StreamEvent::ToolResult { .. }));
        assert!(matches!(events[3], StreamEvent::Result { .. }));
    }

    #[test]
    fn test_mock_claude_bad_json_yields_no_events() {
        use std::process::Command;
        let out = Command::new(mock_claude_binary())
            .env("MOCK_CLAUDE_MODE", "bad_json")
            .output()
            .expect("spawn mock_claude");
        assert!(out.status.success());
        let stdout = String::from_utf8_lossy(&out.stdout);
        let events: Vec<_> = stdout.lines().filter_map(parse_stream_line).collect();
        assert!(events.is_empty(), "bad_json mode should yield no parseable events");
    }
}
