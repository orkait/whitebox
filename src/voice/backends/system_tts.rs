use std::process::{Child, Command, Stdio};

use crate::voice::types::{SpeechTone, VoiceResult};
use crate::voice::TtsBackend;

pub struct SystemTts {
    command: String,
    child: Option<Child>,
    speaking: bool,
}

impl Default for SystemTts {
    fn default() -> Self {
        Self::with_command("espeak")
    }
}

impl SystemTts {
    pub fn with_command(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            child: None,
            speaking: false,
        }
    }
}

impl TtsBackend for SystemTts {
    fn speak(&mut self, text: &str, _tone: SpeechTone) -> VoiceResult<()> {
        let child = Command::new(&self.command)
            .arg(text)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("system tts backend unavailable: {e}"))?;

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
            .map_err(|e| format!("system tts status check failed: {e}"))?
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
