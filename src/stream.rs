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
        duration_ms: Option<u64>,
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
    /// Return the model name from a `MessageStart` event, or `None` for other variants.
    pub fn model_name(&self) -> Option<&str> {
        if let StreamEvent::MessageStart { message } = self {
            Some(message.model.as_str())
        } else {
            None
        }
    }


    /// Extract the text from a `ContentBlockDelta` event with a `text_delta`.
    /// Returns `None` for non-text deltas or non-delta events.
    pub fn delta_text(&self) -> Option<&str> {
        if let StreamEvent::ContentBlockDelta { delta } = self {
            if let DeltaContent::TextDelta { text } = delta {
                return Some(text.as_str());
            }
        }
        None
    }

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
        assert_eq!(event.delta_text(), Some("Hello"));
    }

    #[test]
    fn test_content_block_delta_non_text() {
        // input_json_delta has no `text` field — should parse to Unknown delta
        let line = r#"{"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"{}"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert!(matches!(event, StreamEvent::ContentBlockDelta { .. }));
        assert_eq!(event.delta_text(), None);
    }

    #[test]
    fn test_delta_text_on_non_delta_event() {
        let line = r#"{"type":"error","error":"something went wrong"}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.delta_text(), None);
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
        if let StreamEvent::Result { result, cost_usd, duration_ms, num_turns, .. } = event {
            assert_eq!(result, "final answer");
            assert_eq!(cost_usd, Some(0.01));
            assert_eq!(duration_ms, Some(1200));
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
        assert!(matches!(event, StreamEvent::MessageStart { .. }));
        assert_eq!(event.model_name(), Some("claude-sonnet-4-20250514"));
    }

    #[test]
    fn test_message_start_minimal() {
        let line = r#"{"type":"message_start","message":{"model":"claude-opus-4-6"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.model_name(), Some("claude-opus-4-6"));
    }

    #[test]
    fn test_model_name_on_non_message_start() {
        let line = r#"{"type":"error","error":"oops"}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.model_name(), None);
    }
}
