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
