use crate::profiles::ExpressionProfile;
use crate::state::{ListenState, SpeakState, WhiteboxState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipKind {
    IdleSoft,
    IdleSleepy,
    ListenBounce,
    TalkSoft,
    TalkPet,
    TalkSerious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedClip {
    pub kind: ClipKind,
}

pub fn clip_for_state(state: &WhiteboxState) -> SelectedClip {
    if state.speak_state == SpeakState::Speaking {
        return SelectedClip {
            kind: match state.profile {
                ExpressionProfile::PetLike => ClipKind::TalkPet,
                ExpressionProfile::Serious => ClipKind::TalkSerious,
                _ => ClipKind::TalkSoft,
            },
        };
    }

    match state.listen_state {
        ListenState::Hearing | ListenState::Debouncing | ListenState::Ready => SelectedClip {
            kind: ClipKind::ListenBounce,
        },
        ListenState::Idle => SelectedClip {
            kind: match state.profile {
                ExpressionProfile::Sleepy => ClipKind::IdleSleepy,
                _ => ClipKind::IdleSoft,
            },
        },
    }
}
