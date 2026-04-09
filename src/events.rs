use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    PhaseStarted,
    StepStarted { track_id: String, step_num: usize, total_steps: usize },
    StepCompleted { track_id: String },
    StepFailed { track_id: String },
    TrackCompleted { track_id: String },
    ClaudeOutput { line: String },
    AssistantText { text: String },
    ToolUseStarted { tool: String },
    ToolResultReceived { tool: String },
    PhaseLabel { label: String },
    TokenDelta { text: String },
    ExecutionFinished { completed: usize, blocked: usize },
}

pub type EventSender = mpsc::Sender<ProgressEvent>;
pub type EventReceiver = mpsc::Receiver<ProgressEvent>;

pub fn channel() -> (EventSender, EventReceiver) {
    mpsc::channel()
}

/// Send a message to the TUI output panel, or print to stdout if no TUI.
pub fn emit(sender: Option<&EventSender>, msg: &str) {
    if let Some(tx) = sender {
        let _ = tx.send(ProgressEvent::ClaudeOutput { line: msg.to_string() });
    } else {
        println!("{}", msg);
    }
}
