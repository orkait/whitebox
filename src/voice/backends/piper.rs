use std::io::Write;
use std::process::{Child, Command, Stdio};

use crate::voice::types::{SpeechTone, VoiceResult};
use crate::voice::TtsBackend;

pub struct PiperTts {
    command: String,
    model_path: Option<String>,
    config_path: Option<String>,
    child: Option<Child>,
    speaking: bool,
}

impl Default for PiperTts {
    fn default() -> Self {
        Self::with_command("piper")
    }
}

impl PiperTts {
    pub fn with_command(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            model_path: None,
            config_path: None,
            child: None,
            speaking: false,
        }
    }

    pub fn with_paths(mut self, model_path: Option<String>, config_path: Option<String>) -> Self {
        self.model_path = model_path;
        self.config_path = config_path;
        self
    }
}

impl TtsBackend for PiperTts {
    fn speak(&mut self, text: &str, _tone: SpeechTone) -> VoiceResult<()> {
        let mut command = Command::new(&self.command);
        if let Some(model_path) = &self.model_path {
            command.arg("--model").arg(model_path);
        }
        if let Some(config_path) = &self.config_path {
            command.arg("--config").arg(config_path);
        }

        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("piper backend unavailable: {e}"))?;

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| format!("piper stdin write failed: {e}"))?;
        }

        self.child = Some(child);
        self.speaking = true;
        Ok(())
    }

    fn stop(&mut self) -> VoiceResult<()> {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
        }
        self.child = None;
        self.speaking = false;
        Ok(())
    }

    fn poll_complete(&mut self) -> VoiceResult<bool> {
        let Some(child) = self.child.as_mut() else {
            self.speaking = false;
            return Ok(false);
        };

        match child
            .try_wait()
            .map_err(|e| format!("piper status check failed: {e}"))?
        {
            Some(_) => {
                self.child = None;
                self.speaking = false;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn is_speaking(&self) -> bool {
        self.speaking
    }
}
