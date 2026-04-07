use std::collections::VecDeque;
use std::sync::mpsc;

use crate::app::App;
use crate::state::{ListenState, SpeakState};
use crate::voice::{SttBackend, TtsBackend};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stance {
    Neutral,
    Warm,
    Playful,
    Curious,
    Alert,
    Focused,
    Guarded,
    Stern,
    Tired,
    Sad,
    Angry,
}

impl Stance {
    pub fn as_str(self) -> &'static str {
        match self {
            Stance::Neutral => "neutral",
            Stance::Warm => "warm",
            Stance::Playful => "playful",
            Stance::Curious => "curious",
            Stance::Alert => "alert",
            Stance::Focused => "focused",
            Stance::Guarded => "guarded",
            Stance::Stern => "stern",
            Stance::Tired => "tired",
            Stance::Sad => "sad",
            Stance::Angry => "angry",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Stance::Neutral => "Neutral",
            Stance::Warm => "Warm",
            Stance::Playful => "Playful",
            Stance::Curious => "Curious",
            Stance::Alert => "Alert",
            Stance::Focused => "Focused",
            Stance::Guarded => "Guarded",
            Stance::Stern => "Stern",
            Stance::Tired => "Tired",
            Stance::Sad => "Sad",
            Stance::Angry => "Angry",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    Idle,
    Listening,
    Thinking,
    Speaking,
}

impl Activity {
    pub fn as_str(self) -> &'static str {
        match self {
            Activity::Idle => "idle",
            Activity::Listening => "listening",
            Activity::Thinking => "thinking",
            Activity::Speaking => "speaking",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Activity::Idle => "Idle",
            Activity::Listening => "Listening",
            Activity::Thinking => "Thinking",
            Activity::Speaking => "Speaking",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    Completed,
    Cancelled,
    Interrupted,
    Error,
}

impl StopReason {
    pub fn as_str(self) -> &'static str {
        match self {
            StopReason::Completed => "completed",
            StopReason::Cancelled => "cancelled",
            StopReason::Interrupted => "interrupted",
            StopReason::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Busy,
    BackendUnavailable,
    InvalidStance,
    NoActiveSession,
    InternalError,
}

impl ErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::Busy => "busy",
            ErrorCode::BackendUnavailable => "backend_unavailable",
            ErrorCode::InvalidStance => "invalid_stance",
            ErrorCode::NoActiveSession => "no_active_session",
            ErrorCode::InternalError => "internal_error",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyError {
    pub code: ErrorCode,
    pub message: String,
    pub cause: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyEvent {
    ListenStarted { id: SessionId },
    ListenPartial { id: SessionId, text: String },
    ListenFinal { id: SessionId, text: String },
    ListenStopped { id: SessionId, reason: StopReason },
    SpeakStarted { id: SessionId, text: String },
    SpeakStopped { id: SessionId, reason: StopReason },
    Error {
        code: ErrorCode,
        message: String,
        cause: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodySnapshot {
    pub stance: Stance,
    pub activity: Activity,
    pub status_text: Option<String>,
}

pub struct WhiteboxBody {
    app: App,
    events: VecDeque<BodyEvent>,
    next_session_id: u64,
    active_listen: Option<SessionId>,
    active_speak: Option<SessionId>,
    listen_completion_pending: bool,
}

impl WhiteboxBody {
    pub fn new() -> Self {
        let (_speech_tx, speech_rx) = mpsc::channel();
        let (_info_tx, info_rx) = mpsc::channel();
        Self::from_app(App::new(speech_rx, info_rx))
    }

    pub fn from_app(mut app: App) -> Self {
        app.state.stance = Stance::Neutral;
        app.state.status_text = None;
        Self {
            app,
            events: VecDeque::new(),
            next_session_id: 1,
            active_listen: None,
            active_speak: None,
            listen_completion_pending: false,
        }
    }

    pub fn replace_stt(&mut self, backend: Box<dyn SttBackend>) {
        self.app.replace_stt(backend);
    }

    pub fn replace_tts(&mut self, backend: Box<dyn TtsBackend>) {
        self.app.replace_tts(backend);
    }

    pub fn listen(&mut self) -> Result<SessionId, BodyError> {
        if self.active_listen.is_some() {
            return Err(self.fail(ErrorCode::Busy, "listen session already active", None));
        }
        if let Some(id) = self.active_speak.take() {
            self.app
                .stop_speaking()
                .map_err(|cause| self.fail(ErrorCode::BackendUnavailable, "failed to interrupt speech", Some(cause)))?;
            self.events
                .push_back(BodyEvent::SpeakStopped { id, reason: StopReason::Interrupted });
        }
        self.app
            .begin_listening()
            .map_err(|cause| self.fail(ErrorCode::BackendUnavailable, "failed to start listening", Some(cause)))?;
        let id = self.alloc_session_id();
        self.active_listen = Some(id);
        self.listen_completion_pending = false;
        self.events.push_back(BodyEvent::ListenStarted { id });
        Ok(id)
    }

    pub fn stop_listening(&mut self) -> Result<(), BodyError> {
        if self.active_listen.is_none() {
            return Err(self.fail(ErrorCode::NoActiveSession, "no active listening session", None));
        }
        self.app
            .end_listening()
            .map_err(|cause| self.fail(ErrorCode::BackendUnavailable, "failed to stop listening", Some(cause)))?;
        self.listen_completion_pending = true;
        Ok(())
    }

    pub fn speak(&mut self, text: impl Into<String>) -> Result<SessionId, BodyError> {
        let text = text.into();
        if let Some(id) = self.active_listen.take() {
            self.listen_completion_pending = false;
            self.events
                .push_back(BodyEvent::ListenStopped { id, reason: StopReason::Interrupted });
        }
        if let Some(id) = self.active_speak.take() {
            self.app
                .stop_speaking()
                .map_err(|cause| self.fail(ErrorCode::BackendUnavailable, "failed to interrupt speech", Some(cause)))?;
            self.events
                .push_back(BodyEvent::SpeakStopped { id, reason: StopReason::Interrupted });
        }
        let tone = tone_for_stance(self.app.state.stance).to_string();
        self.app
            .request_speak(text.clone(), tone)
            .map_err(|cause| self.fail(ErrorCode::BackendUnavailable, "failed to start speech", Some(cause)))?;
        let id = self.alloc_session_id();
        self.active_speak = Some(id);
        self.events
            .push_back(BodyEvent::SpeakStarted { id, text });
        Ok(id)
    }

    pub fn stop_speaking(&mut self) -> Result<(), BodyError> {
        let Some(id) = self.active_speak.take() else {
            return Err(self.fail(ErrorCode::NoActiveSession, "no active speaking session", None));
        };
        self.app
            .stop_speaking()
            .map_err(|cause| self.fail(ErrorCode::BackendUnavailable, "failed to stop speech", Some(cause)))?;
        self.events
            .push_back(BodyEvent::SpeakStopped { id, reason: StopReason::Cancelled });
        Ok(())
    }

    pub fn set_stance(&mut self, stance: Stance) -> Result<(), BodyError> {
        self.app.set_stance(stance);
        Ok(())
    }

    pub fn set_status(&mut self, text: impl Into<String>) {
        self.app.set_status_text(Some(text.into()));
    }

    pub fn clear_status(&mut self) {
        self.app.set_status_text(None);
    }

    pub fn tick(&mut self) {
        let prev_speak = self.app.state.speak_state;
        let voice_events = self.app.drain_voice();

        for event in voice_events {
            match event {
                crate::voice::types::SttEvent::TranscriptPartial(text) => {
                    if let Some(id) = self.active_listen {
                        self.events.push_back(BodyEvent::ListenPartial { id, text });
                    }
                }
                crate::voice::types::SttEvent::TranscriptFinal(text) => {
                    if let Some(id) = self.active_listen {
                        self.events.push_back(BodyEvent::ListenFinal { id, text });
                    }
                }
            }
        }

        match self.app.poll_speech_completion() {
            Ok(true) => {}
            Ok(false) => {}
            Err(cause) => {
                let error = self.fail(
                    ErrorCode::BackendUnavailable,
                    "failed to poll speech completion",
                    Some(cause),
                );
                if let Some(id) = self.active_speak.take() {
                    self.events.push_back(BodyEvent::SpeakStopped {
                        id,
                        reason: StopReason::Error,
                    });
                }
                let _ = error;
            }
        }
        self.app.advance_tick();

        if let Some(id) = self.active_speak {
            if prev_speak == SpeakState::Speaking && self.app.state.speak_state == SpeakState::Idle {
                self.active_speak = None;
                self.events
                    .push_back(BodyEvent::SpeakStopped { id, reason: StopReason::Completed });
            }
        }

        if self.listen_completion_pending && self.app.state.listen_state == ListenState::Ready {
            if let Some(id) = self.active_listen.take() {
                self.events
                    .push_back(BodyEvent::ListenStopped { id, reason: StopReason::Completed });
            }
            self.listen_completion_pending = false;
        }
    }

    pub fn snapshot(&self) -> BodySnapshot {
        BodySnapshot {
            stance: self.app.state.stance,
            activity: current_activity(self.active_listen, self.active_speak),
            status_text: self.app.state.status_text.clone(),
        }
    }

    pub fn drain_events(&mut self) -> Vec<BodyEvent> {
        self.events.drain(..).collect()
    }

    pub fn app(&self) -> &App {
        &self.app
    }

    fn alloc_session_id(&mut self) -> SessionId {
        let id = SessionId(self.next_session_id);
        self.next_session_id += 1;
        id
    }

    fn fail(
        &mut self,
        code: ErrorCode,
        message: impl Into<String>,
        cause: Option<String>,
    ) -> BodyError {
        let message = message.into();
        self.events.push_back(BodyEvent::Error {
            code,
            message: message.clone(),
            cause: cause.clone(),
        });
        BodyError { code, message, cause }
    }
}

fn tone_for_stance(stance: Stance) -> &'static str {
    match stance {
        Stance::Warm => "warm",
        Stance::Playful => "pet-like",
        Stance::Alert | Stance::Focused | Stance::Guarded | Stance::Stern | Stance::Angry => {
            "serious"
        }
        _ => "neutral",
    }
}

fn current_activity(active_listen: Option<SessionId>, active_speak: Option<SessionId>) -> Activity {
    if active_speak.is_some() {
        Activity::Speaking
    } else if active_listen.is_some() {
        Activity::Listening
    } else {
        Activity::Idle
    }
}
