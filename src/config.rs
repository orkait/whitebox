use std::fs;
use std::path::Path;

use crate::voice::registry::{SttBackendKind, TtsBackendKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceConfig {
    pub stt_backend: SttBackendKind,
    pub tts_backend: TtsBackendKind,
    pub stt_command: Option<String>,
    pub tts_command: Option<String>,
    pub stt_model: Option<String>,
    pub tts_model: Option<String>,
    pub tts_config: Option<String>,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            stt_backend: SttBackendKind::Mock,
            tts_backend: TtsBackendKind::Mock,
            stt_command: None,
            tts_command: None,
            stt_model: None,
            tts_model: None,
            tts_config: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub render: RenderConfig,
    pub voice: VoiceConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            render: RenderConfig::default(),
            voice: VoiceConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn load(path: Option<&Path>) -> Result<Self, String> {
        let mut cfg = if let Some(path) = path {
            Self::from_file(path)?
        } else {
            Self::default()
        };

        let env_cfg = Self::from_env();
        cfg = merge_config(cfg, env_cfg);
        Ok(cfg)
    }

    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(width) = std::env::var("WHITEBOX_RENDER_WIDTH") {
            if let Ok(width) = width.parse::<u32>() {
                cfg.render.width = width;
            }
        }

        if let Ok(height) = std::env::var("WHITEBOX_RENDER_HEIGHT") {
            if let Ok(height) = height.parse::<u32>() {
                cfg.render.height = height;
            }
        }

        if let Ok(stt) = std::env::var("WHITEBOX_STT_BACKEND") {
            cfg.voice.stt_backend = match stt.as_str() {
                "whisper" => SttBackendKind::Whisper,
                "moonshine" | "moonshine-sidecar" => SttBackendKind::MoonshineSidecar,
                _ => SttBackendKind::Mock,
            };
        }
        if let Ok(stt_command) = std::env::var("WHITEBOX_STT_COMMAND") {
            if !stt_command.trim().is_empty() {
                cfg.voice.stt_command = Some(stt_command);
            }
        }
        if let Ok(stt_model) = std::env::var("WHITEBOX_STT_MODEL") {
            if !stt_model.trim().is_empty() {
                cfg.voice.stt_model = Some(stt_model);
            }
        }

        if let Ok(tts) = std::env::var("WHITEBOX_TTS_BACKEND") {
            cfg.voice.tts_backend = match tts.as_str() {
                "piper" => TtsBackendKind::Piper,
                "system" => TtsBackendKind::System,
                _ => TtsBackendKind::Mock,
            };
        }
        if let Ok(tts_command) = std::env::var("WHITEBOX_TTS_COMMAND") {
            if !tts_command.trim().is_empty() {
                cfg.voice.tts_command = Some(tts_command);
            }
        }
        if let Ok(tts_model) = std::env::var("WHITEBOX_TTS_MODEL") {
            if !tts_model.trim().is_empty() {
                cfg.voice.tts_model = Some(tts_model);
            }
        }
        if let Ok(tts_config) = std::env::var("WHITEBOX_TTS_CONFIG") {
            if !tts_config.trim().is_empty() {
                cfg.voice.tts_config = Some(tts_config);
            }
        }

        cfg
    }

    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("failed to read config file {}: {e}", path.display()))?;
        let mut cfg = Self::default();

        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim();

            match key {
                "WHITEBOX_RENDER_WIDTH" => {
                    if let Ok(width) = value.parse::<u32>() {
                        cfg.render.width = width;
                    }
                }
                "WHITEBOX_RENDER_HEIGHT" => {
                    if let Ok(height) = value.parse::<u32>() {
                        cfg.render.height = height;
                    }
                }
                "WHITEBOX_STT_BACKEND" => {
                    cfg.voice.stt_backend = match value {
                        "whisper" => SttBackendKind::Whisper,
                        "moonshine" | "moonshine-sidecar" => SttBackendKind::MoonshineSidecar,
                        _ => SttBackendKind::Mock,
                    };
                }
                "WHITEBOX_TTS_BACKEND" => {
                    cfg.voice.tts_backend = match value {
                        "piper" => TtsBackendKind::Piper,
                        "system" => TtsBackendKind::System,
                        _ => TtsBackendKind::Mock,
                    };
                }
                "WHITEBOX_STT_COMMAND" => {
                    if !value.is_empty() {
                        cfg.voice.stt_command = Some(value.to_string());
                    }
                }
                "WHITEBOX_STT_MODEL" => {
                    if !value.is_empty() {
                        cfg.voice.stt_model = Some(value.to_string());
                    }
                }
                "WHITEBOX_TTS_COMMAND" => {
                    if !value.is_empty() {
                        cfg.voice.tts_command = Some(value.to_string());
                    }
                }
                "WHITEBOX_TTS_MODEL" => {
                    if !value.is_empty() {
                        cfg.voice.tts_model = Some(value.to_string());
                    }
                }
                "WHITEBOX_TTS_CONFIG" => {
                    if !value.is_empty() {
                        cfg.voice.tts_config = Some(value.to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(cfg)
    }
}

fn merge_config(mut base: AppConfig, env_cfg: AppConfig) -> AppConfig {
    if env_cfg.render.width != AppConfig::default().render.width {
        base.render.width = env_cfg.render.width;
    }
    if env_cfg.render.height != AppConfig::default().render.height {
        base.render.height = env_cfg.render.height;
    }
    if env_cfg.voice.stt_backend != AppConfig::default().voice.stt_backend {
        base.voice.stt_backend = env_cfg.voice.stt_backend;
    }
    if env_cfg.voice.tts_backend != AppConfig::default().voice.tts_backend {
        base.voice.tts_backend = env_cfg.voice.tts_backend;
    }
    if env_cfg.voice.stt_command.is_some() {
        base.voice.stt_command = env_cfg.voice.stt_command;
    }
    if env_cfg.voice.tts_command.is_some() {
        base.voice.tts_command = env_cfg.voice.tts_command;
    }
    if env_cfg.voice.stt_model.is_some() {
        base.voice.stt_model = env_cfg.voice.stt_model;
    }
    if env_cfg.voice.tts_model.is_some() {
        base.voice.tts_model = env_cfg.voice.tts_model;
    }
    if env_cfg.voice.tts_config.is_some() {
        base.voice.tts_config = env_cfg.voice.tts_config;
    }
    base
}
