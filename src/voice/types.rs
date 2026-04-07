pub type VoiceResult<T> = Result<T, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechTone {
    Neutral,
    Warm,
    Playful,
    Serious,
    Urgent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SttEvent {
    TranscriptPartial(String),
    TranscriptFinal(String),
}
