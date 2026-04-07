#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeechDirective {
    pub text: String,
    pub tone: String,
    pub interaction_intent: String,
    pub interruptible: bool,
}
