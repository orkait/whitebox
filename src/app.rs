use std::path::{Path, PathBuf};

use crate::api::Stance;
use crate::avatar::{compose_avatar_image, resolve_avatar};
use crate::config::{AppConfig, VoiceConfig};
use crate::events::WhiteboxEvent;
use crate::reducer::reduce;
use crate::scene::Scene;
use crate::state::{CurrentTextKind, ListenState, WhiteboxState};
use crate::voice::factory::{build_stt_backend_from_config, build_tts_backend_from_config};
use crate::voice::types::{SpeechTone, SttEvent};
use crate::voice::{SttBackend, SttManager, TtsBackend, TtsManager};

pub struct App {
    pub state: WhiteboxState,
    pub config: AppConfig,
    pub cat_root: PathBuf,
    pub stt: SttManager,
    pub tts: TtsManager,
}

impl App {
    pub fn new() -> Self {
        Self::from_config(AppConfig::default())
    }

    pub fn with_voice_config(cfg: VoiceConfig) -> Self {
        Self::from_config(AppConfig {
            voice: cfg,
            ..AppConfig::default()
        })
    }

    pub fn from_config(cfg: AppConfig) -> Self {
        Self {
            state: WhiteboxState::default(),
            config: cfg.clone(),
            cat_root: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cat"),
            stt: SttManager::new(build_stt_backend_from_config(&cfg.voice)),
            tts: TtsManager::new(build_tts_backend_from_config(&cfg.voice)),
        }
    }

    pub fn cat_root(&self) -> &Path {
        &self.cat_root
    }

    pub fn replace_stt(&mut self, backend: Box<dyn SttBackend>) {
        self.stt = SttManager::new(backend);
    }

    pub fn replace_tts(&mut self, backend: Box<dyn TtsBackend>) {
        self.tts = TtsManager::new(backend);
    }

    pub fn begin_listening(&mut self) -> Result<(), String> {
        if self.tts.is_speaking() {
            self.tts.stop()?;
            self.log("speaking interrupted for listen".to_string());
        }
        self.stt.start_listening()?;
        reduce(&mut self.state, WhiteboxEvent::ListenStarted);
        self.log("listening started".to_string());
        Ok(())
    }

    pub fn end_listening(&mut self) -> Result<(), String> {
        self.stt.stop_listening()?;
        reduce(&mut self.state, WhiteboxEvent::ListenStopped);
        self.log("listening stopped".to_string());
        Ok(())
    }

    pub fn drain_voice(&mut self) -> Vec<SttEvent> {
        let mut drained = Vec::new();
        while let Some(event) = self.stt.poll_event() {
            match &event {
                SttEvent::TranscriptPartial(partial) => {
                    self.log(format!("stt partial: {}", partial));
                    self.state.listen_state = ListenState::Hearing;
                }
                SttEvent::TranscriptFinal(final_text) => {
                    self.log(format!("stt final: {}", final_text));
                    self.state.listen_state = ListenState::Ready;
                    self.state.listen_debounce_ticks_remaining = 0;
                    self.state.mode = crate::modes::Mode::Interaction;
                    self.state.current_text = Some(final_text.clone());
                    self.state.current_text_kind = Some(CurrentTextKind::Heard);
                    self.state.current_tone = None;
                }
            }
            drained.push(event);
        }
        drained
    }

    pub fn poll_speech_completion(&mut self) -> Result<bool, String> {
        match self.tts.poll_complete() {
            Ok(true) => {
                reduce(&mut self.state, WhiteboxEvent::SpeechFinished);
                self.log("speech finished".to_string());
                Ok(true)
            }
            Ok(false) => Ok(false),
            Err(err) => Err(err),
        }
    }

    pub fn advance_tick(&mut self) {
        reduce(&mut self.state, WhiteboxEvent::Tick);
        if self.state.listen_state == ListenState::Debouncing {
            if self.state.listen_debounce_ticks_remaining > 0 {
                self.state.listen_debounce_ticks_remaining -= 1;
            }
            if self.state.listen_debounce_ticks_remaining == 0 {
                self.state.listen_state = ListenState::Ready;
                self.log("utterance became ready".to_string());
            }
        }
    }

    pub fn tick(&mut self) {
        self.drain_voice();
        if let Err(err) = self.poll_speech_completion() {
            self.log(format!("speech completion check failed: {err}"));
        }
        self.advance_tick();
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.stt.is_listening() {
            self.stt.stop_listening()?;
        }
        if self.tts.is_speaking() {
            self.tts.stop()?;
        }
        self.log("app shutdown".to_string());
        Ok(())
    }

    pub fn scene(&self) -> Scene {
        Scene::from_state(
            &self.state,
            (self.config.render.width, self.config.render.height),
            format!("{:?}", self.config.voice.stt_backend),
            format!("{:?}", self.config.voice.tts_backend),
        )
    }

    pub fn composed_avatar_image(&self) -> image::ImageResult<image::DynamicImage> {
        compose_avatar_image(&self.cat_root, &resolve_avatar(&self.state))
    }

    pub fn set_stance(&mut self, stance: Stance) {
        self.state.stance = stance;
        self.state.profile = profile_for_stance(stance);
    }

    pub fn set_status_text(&mut self, text: Option<String>) {
        self.state.status_text = text;
    }

    pub fn request_speak(&mut self, text: String, tone: String) -> Result<(), String> {
        if self.stt.is_listening() {
            self.stt.stop_listening()?;
            self.log("listening interrupted for speech".to_string());
        }
        let backend_tone = map_tone(&tone);
        self.tts.speak(&text, backend_tone)?;
        reduce(
            &mut self.state,
            WhiteboxEvent::SpeakRequested { text, tone },
        );
        Ok(())
    }

    pub fn stop_speaking(&mut self) -> Result<(), String> {
        if self.tts.is_speaking() {
            self.tts.stop()?;
        }
        reduce(&mut self.state, WhiteboxEvent::SpeechFinished);
        self.log("speaking stopped".to_string());
        Ok(())
    }

    fn log(&mut self, message: String) {
        self.state.diagnostics.push(message);
        if self.state.diagnostics.len() > 50 {
            let overflow = self.state.diagnostics.len() - 50;
            self.state.diagnostics.drain(0..overflow);
        }
    }
}

fn map_tone(tone: &str) -> SpeechTone {
    match tone {
        "warm" => SpeechTone::Warm,
        "playful" | "pet-like" => SpeechTone::Playful,
        "serious" => SpeechTone::Serious,
        "urgent" => SpeechTone::Urgent,
        _ => SpeechTone::Neutral,
    }
}

fn profile_for_stance(stance: Stance) -> crate::profiles::ExpressionProfile {
    match stance {
        Stance::Neutral => crate::profiles::ExpressionProfile::Neutral,
        Stance::Warm | Stance::Playful => crate::profiles::ExpressionProfile::PetLike,
        Stance::Curious => crate::profiles::ExpressionProfile::Curious,
        Stance::Alert
        | Stance::Focused
        | Stance::Guarded
        | Stance::Stern
        | Stance::Angry => crate::profiles::ExpressionProfile::Serious,
        Stance::Tired | Stance::Sad => crate::profiles::ExpressionProfile::Sleepy,
    }
}
