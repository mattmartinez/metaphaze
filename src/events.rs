use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    PhaseStarted,
    StepStarted { track_id: String, step_num: usize, total_steps: usize },
    StepCompleted { track_id: String },
    StepFailed { track_id: String },
    StepBlocked { track_id: String, step_id: String, reason: String },
    TrackCompleted { track_id: String },
    ClaudeOutput { line: String, track_id: Option<String> },
    AssistantText { text: String, track_id: Option<String> },
    ToolUseStarted { tool: String, track_id: Option<String> },
    ToolResultReceived { tool: String, track_id: Option<String> },
    PhaseLabel { label: String, track_id: Option<String> },
    TokenDelta { text: String, track_id: Option<String> },
    ModelDetected { model: String },
    ExecutionFinished { completed: usize, blocked: usize },
    PhaseTransition { from: String, to: String },
    BudgetExhausted { spent: f64, limit: f64 },
    CostUpdate { spent: f64, limit: Option<f64> },
}

pub type EventSender = mpsc::Sender<ProgressEvent>;
pub type EventReceiver = mpsc::Receiver<ProgressEvent>;

pub fn channel() -> (EventSender, EventReceiver) {
    mpsc::channel()
}

/// Send a message to the TUI output panel, or print to stdout if no TUI.
pub fn emit(sender: Option<&EventSender>, msg: &str) {
    if let Some(tx) = sender {
        let _ = tx.send(ProgressEvent::ClaudeOutput { line: msg.to_string(), track_id: None });
    } else {
        println!("{}", msg);
    }
}
