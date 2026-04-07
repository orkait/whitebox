use crate::api::Stance;
use crate::modes::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentTextKind {
    Spoken,
    Heard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenState {
    Idle,
    Hearing,
    Debouncing,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakState {
    Idle,
    Speaking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteboxState {
    pub mode: Mode,
    pub stance: Stance,
    pub listen_state: ListenState,
    pub speak_state: SpeakState,
    pub listen_debounce_ticks_remaining: u8,
    pub current_text: Option<String>,
    pub current_text_kind: Option<CurrentTextKind>,
    pub current_tone: Option<String>,
    pub status_text: Option<String>,
    pub info_fragments: Vec<String>,
    pub diagnostics: Vec<String>,
    pub tick_count: u64,
}

impl Default for WhiteboxState {
    fn default() -> Self {
        Self {
            mode: Mode::Expression,
            stance: Stance::Neutral,
            listen_state: ListenState::Idle,
            speak_state: SpeakState::Idle,
            listen_debounce_ticks_remaining: 0,
            current_text: None,
            current_text_kind: None,
            current_tone: None,
            status_text: None,
            info_fragments: Vec::new(),
            diagnostics: Vec::new(),
            tick_count: 0,
        }
    }
}
