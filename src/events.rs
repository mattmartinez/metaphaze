use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ProgressEvent {
    PhaseStarted { phase_id: String, total_tracks: usize, total_steps: usize },
    TrackStarted { phase_id: String, track_id: String, track_title: String },
    StepStarted { phase_id: String, track_id: String, step_id: String, step_title: String, step_num: usize, total_steps: usize },
    StepCompleted { phase_id: String, track_id: String, step_id: String },
    StepFailed { phase_id: String, track_id: String, step_id: String, error: String },
    TrackCompleted { phase_id: String, track_id: String },
    ClaudeOutput { line: String },
    ExecutionFinished { completed: usize, blocked: usize },
}

pub type EventSender = mpsc::Sender<ProgressEvent>;
pub type EventReceiver = mpsc::Receiver<ProgressEvent>;

pub fn channel() -> (EventSender, EventReceiver) {
    mpsc::channel()
}
