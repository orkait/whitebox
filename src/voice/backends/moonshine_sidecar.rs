use std::collections::VecDeque;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::env;

use crate::voice::types::{SttEvent, VoiceResult};
use crate::voice::SttBackend;

pub struct MoonshineSidecarStt {
    command: String,
    recorder_command: String,
    listening: bool,
    recorder_child: Option<Child>,
    pending_audio: Option<std::path::PathBuf>,
    events: VecDeque<SttEvent>,
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
            recorder_command: "arecord".into(),
            listening: false,
            recorder_child: None,
            pending_audio: None,
            events: VecDeque::new(),
        }
    }

    pub fn with_commands(command: impl Into<String>, recorder_command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            recorder_command: recorder_command.into(),
            listening: false,
            recorder_child: None,
            pending_audio: None,
            events: VecDeque::new(),
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
        let temp_path = env::temp_dir().join("whitebox-moonshine-input.wav");
        let child = Command::new(&self.recorder_command)
            .arg("-q")
            .arg("-f")
            .arg("cd")
            .arg("-t")
            .arg("wav")
            .arg(&temp_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("moonshine stt recorder unavailable: {e}"))?;

        self.pending_audio = Some(temp_path);
        self.recorder_child = Some(child);
        self.listening = true;
        Ok(())
    }

    fn stop_listening(&mut self) -> VoiceResult<()> {
        if let Some(child) = self.recorder_child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.recorder_child = None;
        self.listening = false;

        if let Some(path) = self.pending_audio.take() {
            match self.transcribe_file(&path) {
                Ok(text) if !text.is_empty() => {
                    self.events.push_back(SttEvent::TranscriptFinal(text));
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("moonshine transcription error: {e}");
                }
            }
        }
        Ok(())
    }

    fn poll_event(&mut self) -> Option<SttEvent> {
        self.events.pop_front()
    }

    fn is_listening(&self) -> bool {
        self.listening
    }
}
