pub mod backends;
pub mod factory;
pub mod registry;
pub mod types;

use std::path::Path;

use self::types::{SpeechTone, SttEvent, VoiceResult};

pub trait SttBackend: Send {
    fn transcribe_file(&mut self, path: &Path) -> VoiceResult<String>;
    fn start_listening(&mut self) -> VoiceResult<()>;
    fn stop_listening(&mut self) -> VoiceResult<()>;
    fn poll_event(&mut self) -> Option<SttEvent>;
    fn is_listening(&self) -> bool;
}

pub trait TtsBackend: Send {
    fn speak(&mut self, text: &str, tone: SpeechTone) -> VoiceResult<()>;
    fn stop(&mut self) -> VoiceResult<()>;
    fn poll_complete(&mut self) -> VoiceResult<bool> {
        Ok(false)
    }
    fn is_speaking(&self) -> bool;
}

pub struct SttManager {
    backend: Box<dyn SttBackend>,
}

impl SttManager {
    pub fn new(backend: Box<dyn SttBackend>) -> Self {
        Self { backend }
    }

    pub fn transcribe_file(&mut self, path: &Path) -> VoiceResult<String> {
        self.backend.transcribe_file(path)
    }

    pub fn start_listening(&mut self) -> VoiceResult<()> {
        self.backend.start_listening()
    }

    pub fn stop_listening(&mut self) -> VoiceResult<()> {
        self.backend.stop_listening()
    }

    pub fn poll_event(&mut self) -> Option<SttEvent> {
        self.backend.poll_event()
    }

    pub fn is_listening(&self) -> bool {
        self.backend.is_listening()
    }
}

pub struct TtsManager {
    backend: Box<dyn TtsBackend>,
}

impl TtsManager {
    pub fn new(backend: Box<dyn TtsBackend>) -> Self {
        Self { backend }
    }

    pub fn speak(&mut self, text: &str, tone: SpeechTone) -> VoiceResult<()> {
        self.backend.speak(text, tone)
    }

    pub fn stop(&mut self) -> VoiceResult<()> {
        self.backend.stop()
    }

    pub fn poll_complete(&mut self) -> VoiceResult<bool> {
        self.backend.poll_complete()
    }

    pub fn is_speaking(&self) -> bool {
        self.backend.is_speaking()
    }
}

pub use backends::mock::{MockStt as MockSttBackend, MockTts as MockTtsBackend};
