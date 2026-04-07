use crate::events::WhiteboxEvent;
use crate::modes::Mode;
use crate::state::{CurrentTextKind, ListenState, SpeakState, WhiteboxState};

pub fn reduce(state: &mut WhiteboxState, event: WhiteboxEvent) {
    match event {
        WhiteboxEvent::Tick => {
            state.tick_count += 1;
        }
        WhiteboxEvent::SwitchMode(mode) => {
            state.mode = mode;
        }
        WhiteboxEvent::ListenStarted => {
            state.mode = Mode::Interaction;
            state.listen_state = ListenState::Hearing;
            state.speak_state = SpeakState::Idle;
            state.listen_debounce_ticks_remaining = 0;
            state.current_text = None;
            state.current_text_kind = None;
            state.current_tone = None;
        }
        WhiteboxEvent::ListenStopped => {
            state.listen_state = ListenState::Debouncing;
            state.listen_debounce_ticks_remaining = 9;
        }
        WhiteboxEvent::SpeakRequested { text, tone } => {
            state.mode = Mode::Interaction;
            state.listen_state = ListenState::Idle;
            state.speak_state = SpeakState::Speaking;
            state.listen_debounce_ticks_remaining = 0;
            state.current_text = Some(text);
            state.current_text_kind = Some(CurrentTextKind::Spoken);
            state.current_tone = Some(tone.clone());
        }
        WhiteboxEvent::SpeechFinished => {
            state.speak_state = SpeakState::Idle;
            state.listen_debounce_ticks_remaining = 0;
            state.current_text = None;
            state.current_text_kind = None;
            state.current_tone = None;
            state.mode = Mode::Expression;
        }
        WhiteboxEvent::InfoFragment(fragment) => {
            state.mode = Mode::Info;
            state.info_fragments.push(fragment);
            if state.info_fragments.len() > 6 {
                let overflow = state.info_fragments.len() - 6;
                state.info_fragments.drain(0..overflow);
            }
        }
    }
}
