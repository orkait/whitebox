#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SttBackendKind {
    Mock,
    Whisper,
    MoonshineSidecar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtsBackendKind {
    Mock,
    Piper,
    System,
}
