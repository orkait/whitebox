use std::collections::VecDeque;
use std::path::Path;

use whitebox::api::{BodyEvent, ErrorCode, Stance, StopReason, WhiteboxBody};
use whitebox::voice::types::{SpeechTone, SttEvent, VoiceResult};
use whitebox::voice::{SttBackend, TtsBackend};

#[derive(Default)]
struct FinalizingStt {
    listening: bool,
    events: VecDeque<SttEvent>,
}

#[derive(Default)]
struct PartialThenFinalStt {
    listening: bool,
    events: VecDeque<SttEvent>,
}

impl SttBackend for PartialThenFinalStt {
    fn transcribe_file(&mut self, _path: &Path) -> VoiceResult<String> {
        Ok("mock transcript".into())
    }

    fn start_listening(&mut self) -> VoiceResult<()> {
        self.listening = true;
        self.events
            .push_back(SttEvent::TranscriptPartial("partial".into()));
        Ok(())
    }

    fn stop_listening(&mut self) -> VoiceResult<()> {
        self.listening = false;
        self.events
            .push_back(SttEvent::TranscriptFinal("heard from api".into()));
        Ok(())
    }

    fn poll_event(&mut self) -> Option<SttEvent> {
        self.events.pop_front()
    }

    fn is_listening(&self) -> bool {
        self.listening
    }
}

impl SttBackend for FinalizingStt {
    fn transcribe_file(&mut self, _path: &Path) -> VoiceResult<String> {
        Ok("mock transcript".into())
    }

    fn start_listening(&mut self) -> VoiceResult<()> {
        self.listening = true;
        Ok(())
    }

    fn stop_listening(&mut self) -> VoiceResult<()> {
        self.listening = false;
        self.events
            .push_back(SttEvent::TranscriptFinal("heard from api".into()));
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
struct CompletingTts {
    speaking: bool,
    complete_on_poll: bool,
}

impl TtsBackend for CompletingTts {
    fn speak(&mut self, _text: &str, _tone: SpeechTone) -> VoiceResult<()> {
        self.speaking = true;
        self.complete_on_poll = true;
        Ok(())
    }

    fn stop(&mut self) -> VoiceResult<()> {
        self.speaking = false;
        self.complete_on_poll = false;
        Ok(())
    }

    fn poll_complete(&mut self) -> VoiceResult<bool> {
        if self.speaking && self.complete_on_poll {
            self.speaking = false;
            self.complete_on_poll = false;
            return Ok(true);
        }
        Ok(false)
    }

    fn is_speaking(&self) -> bool {
        self.speaking
    }
}

#[test]
fn speak_lifecycle_emits_started_and_completed_events() {
    let mut body = WhiteboxBody::new();
    body.replace_tts(Box::new(CompletingTts::default()));

    let speak_id = body.speak("hello").expect("speech should start");
    let events = body.drain_events();
    assert_eq!(
        events,
        vec![BodyEvent::SpeakStarted {
            id: speak_id,
            text: "hello".into(),
        }]
    );

    body.tick();
    let events = body.drain_events();
    assert_eq!(
        events,
        vec![BodyEvent::SpeakStopped {
            id: speak_id,
            reason: StopReason::Completed,
        }]
    );
}

#[test]
fn listen_lifecycle_emits_started_final_and_completed_events() {
    let mut body = WhiteboxBody::new();
    body.replace_stt(Box::new(FinalizingStt::default()));

    let listen_id = body.listen().expect("listen should start");
    assert_eq!(
        body.drain_events(),
        vec![BodyEvent::ListenStarted { id: listen_id }]
    );

    body.stop_listening().expect("listen should stop");
    body.tick();

    assert_eq!(
        body.drain_events(),
        vec![
            BodyEvent::ListenFinal {
                id: listen_id,
                text: "heard from api".into(),
            },
            BodyEvent::ListenStopped {
                id: listen_id,
                reason: StopReason::Completed,
            },
        ]
    );
}

#[test]
fn speak_interrupts_active_listen_session() {
    let mut body = WhiteboxBody::new();
    let listen_id = body.listen().expect("listen should start");
    body.drain_events();

    let speak_id = body.speak("override").expect("speech should interrupt listen");
    assert_eq!(
        body.drain_events(),
        vec![
            BodyEvent::ListenStopped {
                id: listen_id,
                reason: StopReason::Interrupted,
            },
            BodyEvent::SpeakStarted {
                id: speak_id,
                text: "override".into(),
            },
        ]
    );
}

#[test]
fn stance_and_status_persist_through_activity_changes() {
    let mut body = WhiteboxBody::new();
    body.replace_tts(Box::new(CompletingTts::default()));

    body.set_stance(Stance::Angry).expect("stance should set");
    body.set_status("watching you");
    body.speak("hello").expect("speech should start");
    body.tick();

    let snapshot = body.snapshot();
    assert_eq!(snapshot.stance, Stance::Angry);
    assert_eq!(snapshot.status_text.as_deref(), Some("watching you"));
}

#[test]
fn stopping_without_active_session_returns_no_active_session() {
    let mut body = WhiteboxBody::new();

    let error = body.stop_speaking().expect_err("stop should fail");
    assert_eq!(error.code, ErrorCode::NoActiveSession);
}

#[test]
fn completed_listen_returns_body_to_idle_activity() {
    let mut body = WhiteboxBody::new();
    body.replace_stt(Box::new(FinalizingStt::default()));

    body.listen().expect("listen should start");
    body.drain_events();
    body.stop_listening().expect("listen should stop");
    body.tick();

    assert_eq!(body.snapshot().activity, whitebox::api::Activity::Idle);
}

#[test]
fn partial_transcripts_are_forwarded_as_body_events() {
    let mut body = WhiteboxBody::new();
    body.replace_stt(Box::new(PartialThenFinalStt::default()));

    let listen_id = body.listen().expect("listen should start");
    body.drain_events();
    body.tick();

    assert_eq!(
        body.drain_events(),
        vec![BodyEvent::ListenPartial {
            id: listen_id,
            text: "partial".into(),
        }]
    );
}

#[test]
fn grouped_stances_resolve_to_distinct_visible_expressions() {
    let mut body = WhiteboxBody::new();
    let stances = [
        (Stance::Focused, ("eyes_half_open_blush", "mouth_flat_neutral")),
        (Stance::Guarded, ("eyes_worried_angled", "mouth_small_frown")),
        (Stance::Stern, ("eyes_serious_angry", "mouth_chevron_serious")),
        (Stance::Playful, ("eyes_excited_squint", "mouth_wavy_cat")),
    ];

    for (stance, expected) in stances {
        body.set_stance(stance).expect("stance should set");
        let avatar = body.app().scene().avatar;
        assert_eq!((avatar.eyes, avatar.mouth), expected);
    }
}
