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
    /// Claude is invoking a tool.
    ToolUse {
        tool: String,
        #[serde(rename = "input")]
        _input: Value,
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
