use std::path::Path;
use std::process::Command;

use crate::voice::types::{SttEvent, VoiceResult};
use crate::voice::SttBackend;

pub struct MoonshineSidecarStt {
    command: String,
    listening: bool,
}

impl Default for MoonshineSidecarStt {
    fn default() -> Self {
        Self::with_command("moonshine-sidecar")
    }
}

impl MoonshineSidecarStt {
    pub fn with_command(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            listening: false,
        }
    }
}

impl SttBackend for MoonshineSidecarStt {
    fn transcribe_file(&mut self, path: &Path) -> VoiceResult<String> {
        let output = Command::new(&self.command)
            .arg(path)
            .output()
            .map_err(|e| format!("moonshine sidecar backend unavailable: {e}"))?;
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(text)
        } else {
            Err(format!(
                "moonshine sidecar transcription failed with status {}",
                output.status
            ))
        }
    }

    fn start_listening(&mut self) -> VoiceResult<()> {
        self.listening = true;
        Ok(())
    }

    fn stop_listening(&mut self) -> VoiceResult<()> {
        self.listening = false;
        Ok(())
    }

    fn poll_event(&mut self) -> Option<SttEvent> {
        None
    }

    fn is_listening(&self) -> bool {
        self.listening
    }
}
