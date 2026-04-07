use crate::modes::Mode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhiteboxEvent {
    Tick,
    SwitchMode(Mode),
    ListenStarted,
    ListenStopped,
    SpeakRequested { text: String, tone: String },
    SpeechFinished,
    InfoFragment(String),
}
