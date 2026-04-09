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
        #[serde(default, rename = "cost_usd")]
        _cost_usd: Option<f64>,
        #[serde(default, rename = "duration_ms")]
        _duration_ms: Option<u64>,
        #[serde(default, rename = "num_turns")]
        _num_turns: Option<u32>,
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
}

impl StreamEvent {
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
            return Some(match key_arg {
                Some(arg) => format!("{} {}", tool, arg),
                None => tool.clone(),
            });
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

/// Parse a single NDJSON line from the stream-json output.
/// Returns `None` for unrecognized or malformed lines.
pub fn parse_stream_line(line: &str) -> Option<StreamEvent> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    match serde_json::from_str::<StreamEvent>(trimmed) {
        Ok(event) => Some(event),
        Err(_) => {
            // Silently ignore unknown event types (rate_limit_event, etc.)
            None
        }
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
        assert!(matches!(event, StreamEvent::Result { .. }));
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
        let line = r#"{"type":"tool_use","tool":"Bash","input":{"command":"ls"}}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), Some("Bash".to_string()));
    }

    #[test]
    fn test_tool_use_summary_on_non_tool_event() {
        let line = r#"{"type":"error","error":"oops"}"#;
        let event = parse_stream_line(line).expect("should parse");
        assert_eq!(event.tool_use_summary(), None);
    }
}
