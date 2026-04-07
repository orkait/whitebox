use super::backends::{
    mock::MockStt, mock::MockTts, moonshine_sidecar::MoonshineSidecarStt, piper::PiperTts,
    system_tts::SystemTts, whisper::WhisperStt,
};
use super::registry::{SttBackendKind, TtsBackendKind};
use super::{SttBackend, TtsBackend};
use crate::config::VoiceConfig;

pub fn build_stt_backend(kind: SttBackendKind) -> Box<dyn SttBackend> {
    match kind {
        SttBackendKind::Mock => Box::new(MockStt::default()),
        SttBackendKind::Whisper => Box::new(WhisperStt::default()),
        SttBackendKind::MoonshineSidecar => Box::new(MoonshineSidecarStt::default()),
    }
}

pub fn build_tts_backend(kind: TtsBackendKind) -> Box<dyn TtsBackend> {
    match kind {
        TtsBackendKind::Mock => Box::new(MockTts::default()),
        TtsBackendKind::Piper => Box::new(PiperTts::default()),
        TtsBackendKind::System => Box::new(SystemTts::default()),
    }
}

pub fn build_stt_backend_from_config(cfg: &VoiceConfig) -> Box<dyn SttBackend> {
    match cfg.stt_backend {
        SttBackendKind::Mock => Box::new(MockStt::default()),
        SttBackendKind::Whisper => Box::new(
            WhisperStt::with_command(
                cfg.stt_command
                    .clone()
                    .unwrap_or_else(|| "whisper-cli".into()),
            )
            .with_model_path(cfg.stt_model.clone()),
        ),
        SttBackendKind::MoonshineSidecar => Box::new(MoonshineSidecarStt::with_command(
            cfg.stt_command
                .clone()
                .unwrap_or_else(|| "moonshine-sidecar".into()),
        )),
    }
}

pub fn build_tts_backend_from_config(cfg: &VoiceConfig) -> Box<dyn TtsBackend> {
    match cfg.tts_backend {
        TtsBackendKind::Mock => Box::new(MockTts::default()),
        TtsBackendKind::Piper => Box::new(
            PiperTts::with_command(cfg.tts_command.clone().unwrap_or_else(|| "piper".into()))
                .with_paths(cfg.tts_model.clone(), cfg.tts_config.clone()),
        ),
        TtsBackendKind::System => Box::new(SystemTts::with_command(
            cfg.tts_command.clone().unwrap_or_else(|| "espeak".into()),
        )),
    }
}
