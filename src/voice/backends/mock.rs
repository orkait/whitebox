use std::collections::VecDeque;
use std::path::Path;

use crate::voice::types::{SpeechTone, SttEvent, VoiceResult};
use crate::voice::{SttBackend, TtsBackend};

#[derive(Default)]
pub struct MockStt {
    listening: bool,
    events: VecDeque<SttEvent>,
}

impl MockStt {
    pub fn queue_event(&mut self, event: SttEvent) {
        self.events.push_back(event);
    }
}

impl SttBackend for MockStt {
    fn transcribe_file(&mut self, _path: &Path) -> VoiceResult<String> {
        Ok("mock transcript".into())
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
        self.events.pop_front()
    }

    fn is_listening(&self) -> bool {
        self.listening
    }
}

#[derive(Default)]
pub struct MockTts {
    speaking: bool,
}

impl TtsBackend for MockTts {
    fn speak(&mut self, _text: &str, _tone: SpeechTone) -> VoiceResult<()> {
        self.speaking = true;
        Ok(())
    }

    fn stop(&mut self) -> VoiceResult<()> {
        self.speaking = false;
        Ok(())
    }

    fn is_speaking(&self) -> bool {
        self.speaking
    }
}
