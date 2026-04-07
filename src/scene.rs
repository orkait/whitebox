use crate::avatar::{resolve_avatar, AvatarView};
use crate::api::Stance;
use crate::modes::Mode;
use crate::state::{CurrentTextKind, ListenState, SpeakState, WhiteboxState};

#[derive(Debug, Clone)]
pub struct Scene {
    pub mode: Mode,
    pub stance: String,
    pub avatar: AvatarView,
    pub listen_state: ListenState,
    pub speak_state: SpeakState,
    pub info_fragments: Vec<String>,
    pub diagnostics: Vec<String>,
    pub current_text: Option<String>,
    pub current_text_kind: Option<CurrentTextKind>,
    pub tone: Option<String>,
    pub status_text: Option<String>,
    pub stt_backend: Option<String>,
    pub tts_backend: Option<String>,
    pub render_size: (u32, u32),
}

impl Scene {
    pub fn from_state(
        state: &WhiteboxState,
        render_size: (u32, u32),
        stt_backend: String,
        tts_backend: String,
    ) -> Self {
        Self {
            mode: state.mode,
            stance: stance_label(state.stance),
            avatar: resolve_avatar(state),
            listen_state: state.listen_state,
            speak_state: state.speak_state,
            info_fragments: state.info_fragments.clone(),
            diagnostics: state.diagnostics.clone(),
            current_text: state.current_text.clone(),
            current_text_kind: state.current_text_kind,
            tone: state.current_tone.clone(),
            status_text: state.status_text.clone(),
            stt_backend: Some(stt_backend),
            tts_backend: Some(tts_backend),
            render_size,
        }
    }
}

fn stance_label(stance: Stance) -> String {
    stance.label().to_string()
}
