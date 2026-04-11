// Mock `claude` binary used by metaphaze integration tests.
//
// Behaviour is controlled entirely by environment variables so a test can
// configure a single mock and exercise different code paths without rebuilding:
//
//   MOCK_CLAUDE_MODE      one of: success | tool_use | fail | exit_nonzero | bad_json
//                         (default: "success")
//   MOCK_CLAUDE_TEXT      assistant text to emit (default: "ok")
//   MOCK_CLAUDE_TOOL      tool name to emit a tool_use for (default: "Read")
//   MOCK_CLAUDE_TOOL_ARG  tool input file_path/command/pattern arg
//   MOCK_CLAUDE_RESULT    final result string (default: same as MOCK_CLAUDE_TEXT)
//   MOCK_CLAUDE_COST      cost_usd in the result event (default: 0.01)
//   MOCK_CLAUDE_TURNS     num_turns in the result event (default: 1)
//   MOCK_CLAUDE_INPUT_TOK input_tokens in the result event (default: 100)
//   MOCK_CLAUDE_OUTPUT_TOK output_tokens in the result event (default: 50)
//   MOCK_CLAUDE_WRITE_PATH if set, write MOCK_CLAUDE_WRITE_BODY to this path before exiting
//   MOCK_CLAUDE_WRITE_BODY contents to write (default: "mock summary\n")
//   MOCK_CLAUDE_STDERR    if set, written to stderr before exit
//
// The mock prints valid stream-json NDJSON to stdout (one JSON object per line)
// matching the shape that `src/stream.rs` parses.

use std::env;
use std::io::{self, Write};

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn emit(line: &str) {
    let mut out = io::stdout().lock();
    let _ = writeln!(out, "{}", line);
}

/// Escape a string so it is safe to embed inside a JSON string literal.
/// Handles `"`, `\`, and the common control chars (\n, \r, \t).
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn main() {
    let mode = env_or("MOCK_CLAUDE_MODE", "success");

    if mode == "exit_nonzero" {
        if let Ok(msg) = env::var("MOCK_CLAUDE_STDERR") {
            eprintln!("{}", msg);
        }
        std::process::exit(2);
    }

    if mode == "bad_json" {
        emit("this is not json at all");
        emit("{ also not valid");
        // Exit zero so claude::run hits the "no result event" branch.
        return;
    }

    let text = env_or("MOCK_CLAUDE_TEXT", "ok");
    let result_text = env_or("MOCK_CLAUDE_RESULT", &text);
    let cost = env_or("MOCK_CLAUDE_COST", "0.01");
    let turns = env_or("MOCK_CLAUDE_TURNS", "1");
    let input_tok = env_or("MOCK_CLAUDE_INPUT_TOK", "100");
    let output_tok = env_or("MOCK_CLAUDE_OUTPUT_TOK", "50");

    // 1. message_start with model + initial usage
    emit(&format!(
        r#"{{"type":"message_start","message":{{"model":"claude-mock-1","usage":{{"input_tokens":{},"output_tokens":0}}}}}}"#,
        input_tok
    ));

    // 2. assistant turn with text (and optional tool_use)
    if mode == "tool_use" {
        let tool = env_or("MOCK_CLAUDE_TOOL", "Read");
        let arg = env_or("MOCK_CLAUDE_TOOL_ARG", "src/main.rs");
        let arg_esc = json_escape(&arg);
        let input_field = match tool.as_str() {
            "Bash" => format!(r#"{{"command":"{}"}}"#, arg_esc),
            "Grep" | "Glob" => format!(r#"{{"pattern":"{}"}}"#, arg_esc),
            _ => format!(r#"{{"file_path":"{}"}}"#, arg_esc),
        };
        emit(&format!(
            r#"{{"type":"assistant","message":{{"content":[{{"type":"text","text":"{}"}},{{"type":"tool_use","name":"{}","input":{}}}]}}}}"#,
            json_escape(&text),
            tool,
            input_field
        ));
        emit(&format!(
            r#"{{"type":"tool_result","tool":"{}","content":"mock tool output"}}"#,
            tool
        ));
    } else {
        emit(&format!(
            r#"{{"type":"assistant","message":{{"content":[{{"type":"text","text":"{}"}}]}}}}"#,
            json_escape(&text)
        ));
    }

    // 3. Optional file write side-effect (so executor/verifier integration tests
    //    can observe a "summary" being produced).
    if let Ok(path) = env::var("MOCK_CLAUDE_WRITE_PATH") {
        let body = env_or("MOCK_CLAUDE_WRITE_BODY", "mock summary\n");
        if let Some(parent) = std::path::Path::new(&path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, body);
    }

    // 4. result event (terminal). For "fail" mode emit an error event before result.
    if mode == "fail" {
        emit(r#"{"type":"error","error":"mock failure"}"#);
    }

    emit(&format!(
        r#"{{"type":"result","result":"{}","cost_usd":{},"duration_ms":42,"num_turns":{},"input_tokens":{},"output_tokens":{}}}"#,
        json_escape(&result_text),
        cost,
        turns,
        input_tok,
        output_tok
    ));
}
