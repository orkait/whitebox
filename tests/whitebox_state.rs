use std::path::PathBuf;
use std::sync::mpsc;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use whitebox::animation::{AnimationClip, AnimationFrame, AnimationPlayer};
use whitebox::api::Stance;
use whitebox::app::App;
use whitebox::assets::AssetManifest;
use whitebox::avatar::{resolve_avatar, AssetPaths};
use whitebox::clips::{clip_for_state, ClipKind};
use whitebox::config::{AppConfig, RenderConfig, VoiceConfig};
use whitebox::events::WhiteboxEvent;
use whitebox::integrations::info_feed::InfoDirective;
use whitebox::integrations::superclaw::SpeechDirective;
use whitebox::modes::Mode;
use whitebox::profiles::{ExpressionProfile, PresetLibrary};
use whitebox::reducer::reduce;
use whitebox::state::{CurrentTextKind, ListenState, SpeakState, WhiteboxState};
use whitebox::voice::backends::piper::PiperTts;
use whitebox::voice::backends::system_tts::SystemTts;
use whitebox::voice::backends::whisper::WhisperStt;
use whitebox::voice::factory::{build_stt_backend, build_tts_backend};
use whitebox::voice::registry::{SttBackendKind, TtsBackendKind};
use whitebox::voice::types::{SpeechTone, SttEvent};
use whitebox::voice::{
    MockSttBackend, MockTtsBackend, SttBackend, SttManager, TtsBackend, TtsManager,
};

#[test]
fn default_render_config_uses_1024_square_surface() {
    let cfg = RenderConfig::default();
    assert_eq!(cfg.width, 1024);
    assert_eq!(cfg.height, 1024);
}

#[test]
fn mode_set_is_expression_info_and_interaction() {
    let modes = [Mode::Expression, Mode::Info, Mode::Interaction];
    assert_eq!(modes.len(), 3);
}

#[test]
fn cat_manifest_discovers_expected_asset_counts() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cat_root = repo_root.join("cat");
    let manifest = AssetManifest::from_cat_dir(&cat_root).expect("cat manifest should load");

    assert_eq!(manifest.face.len(), 2);
    assert_eq!(manifest.eyes.len(), 11);
    assert_eq!(manifest.mouth.len(), 10);
    assert_eq!(manifest.ears.len(), 2);
}

#[test]
fn default_state_starts_in_expression_mode() {
    let state = WhiteboxState::default();
    assert_eq!(state.mode, Mode::Expression);
    assert_eq!(state.listen_state, ListenState::Idle);
    assert_eq!(state.speak_state, SpeakState::Idle);
}

#[test]
fn reducer_switches_modes_and_speaking_states() {
    let mut state = WhiteboxState::default();
    reduce(&mut state, WhiteboxEvent::SwitchMode(Mode::Info));
    assert_eq!(state.mode, Mode::Info);

    reduce(
        &mut state,
        WhiteboxEvent::SpeakRequested {
            text: "hello".into(),
            tone: "warm".into(),
        },
    );
    assert_eq!(state.mode, Mode::Interaction);
    assert_eq!(state.speak_state, SpeakState::Speaking);
    assert_eq!(state.current_text.as_deref(), Some("hello"));
}

#[test]
fn preset_library_resolves_pet_like_profile() {
    let lib = PresetLibrary::default();
    let preset = lib.get(ExpressionProfile::PetLike);
    assert_eq!(preset.profile, ExpressionProfile::PetLike);
    assert!(preset.eyes.contains("happy") || preset.eyes.contains("open"));
}

#[test]
fn animation_player_loops_frames_by_tick() {
    let clip = AnimationClip {
        id: "blink_soft".into(),
        frames: vec![
            AnimationFrame::new("eyes_open_blush", "mouth_flat_neutral", 1),
            AnimationFrame::new("eyes_soft_closed", "mouth_flat_neutral", 1),
        ],
        looping: true,
    };

    let mut player = AnimationPlayer::new(clip);
    assert_eq!(player.current().eyes, "eyes_open_blush");
    player.tick();
    assert_eq!(player.current().eyes, "eyes_soft_closed");
    player.tick();
    assert_eq!(player.current().eyes, "eyes_open_blush");
}

#[test]
fn player_can_override_only_animated_parts() {
    let clip = AnimationClip {
        id: "talk_soft".into(),
        frames: vec![
            AnimationFrame::new("eyes_open_blush", "mouth_soft_smile", 1),
            AnimationFrame::new("eyes_open_blush", "mouth_open_flat", 1),
        ],
        looping: true,
    };

    let mut player = AnimationPlayer::new(clip);
    assert_eq!(player.current().mouth, "mouth_soft_smile");
    player.tick();
    assert_eq!(player.current().mouth, "mouth_open_flat");
    assert_eq!(player.current().eyes, "eyes_open_blush");
}

#[test]
fn avatar_resolver_uses_talk_animation_while_speaking() {
    let state = WhiteboxState {
        profile: ExpressionProfile::PetLike,
        speak_state: SpeakState::Speaking,
        tick_count: 1,
        ..WhiteboxState::default()
    };

    let view = resolve_avatar(&state);
    assert!(view.mouth.contains("mouth_"));
    assert_ne!(view.mouth, "mouth_cat_smile");
}

#[test]
fn asset_paths_resolve_existing_pngs_for_avatar_view() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cat_root = repo_root.join("cat");
    let state = WhiteboxState::default();
    let view = resolve_avatar(&state);
    let paths = AssetPaths::from_view(&cat_root, &view);

    assert!(paths.face.exists());
    assert!(paths.eyes.exists());
    assert!(paths.mouth.exists());
    assert!(paths.ears.exists());
}

#[test]
fn clip_selector_chooses_talk_clip_while_speaking() {
    let state = WhiteboxState {
        profile: ExpressionProfile::PetLike,
        speak_state: SpeakState::Speaking,
        ..WhiteboxState::default()
    };

    let clip = clip_for_state(&state);
    assert_eq!(clip.kind, ClipKind::TalkPet);
}

#[test]
fn clip_selector_chooses_listen_clip_while_hearing() {
    let state = WhiteboxState {
        listen_state: ListenState::Hearing,
        ..WhiteboxState::default()
    };

    let clip = clip_for_state(&state);
    assert_eq!(clip.kind, ClipKind::ListenBounce);
}

#[test]
fn listening_overlay_keeps_guarded_stance_visible() {
    let state = WhiteboxState {
        stance: Stance::Guarded,
        profile: ExpressionProfile::Serious,
        listen_state: ListenState::Hearing,
        ..WhiteboxState::default()
    };

    let view = resolve_avatar(&state);
    assert_eq!(view.eyes, "eyes_worried_angled");
    assert_eq!(view.mouth, "mouth_small_frown");
    assert_eq!(view.ears, "ears_style_sharp");
}

#[test]
fn speaking_overlay_keeps_angry_stance_visible() {
    let state = WhiteboxState {
        stance: Stance::Angry,
        profile: ExpressionProfile::Serious,
        speak_state: SpeakState::Speaking,
        tick_count: 0,
        ..WhiteboxState::default()
    };

    let view = resolve_avatar(&state);
    assert_eq!(view.eyes, "eyes_serious_angry");
    assert_eq!(view.mouth, "mouth_pout_loop");
    assert_eq!(view.ears, "ears_style_sharp");
}

#[test]
fn app_keeps_listen_debounce_stable_across_higher_frame_rates() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);

    app.begin_listening().unwrap();
    app.end_listening().unwrap();

    for _ in 0..8 {
        app.tick();
    }
    assert_eq!(app.state.listen_state, ListenState::Debouncing);

    app.tick();
    assert_eq!(app.state.listen_state, ListenState::Ready);
}

#[test]
fn playful_idle_animation_has_a_distinct_signature_loop() {
    let mut state = WhiteboxState {
        stance: Stance::Playful,
        profile: ExpressionProfile::PetLike,
        ..WhiteboxState::default()
    };

    state.tick_count = 0;
    let first = resolve_avatar(&state);
    state.tick_count = 3;
    let second = resolve_avatar(&state);
    state.tick_count = 6;
    let third = resolve_avatar(&state);

    assert_eq!((first.eyes, first.mouth), ("eyes_excited_squint", "mouth_wavy_cat"));
    assert_eq!((second.eyes, second.mouth), ("eyes_happy_closed", "mouth_cat_smile"));
    assert_eq!((third.eyes, third.mouth), ("eyes_happy_closed", "mouth_open_tongue"));
}

#[test]
fn focused_idle_animation_stays_restrained_until_a_late_blink() {
    let mut state = WhiteboxState {
        stance: Stance::Focused,
        profile: ExpressionProfile::Serious,
        ..WhiteboxState::default()
    };

    state.tick_count = 0;
    let steady = resolve_avatar(&state);
    state.tick_count = 8;
    let blink = resolve_avatar(&state);

    assert_eq!((steady.eyes, steady.mouth), ("eyes_half_open_blush", "mouth_flat_neutral"));
    assert_eq!((blink.eyes, blink.mouth), ("eyes_soft_closed", "mouth_flat_neutral"));
}

#[test]
fn angry_idle_animation_never_falls_back_to_generic_open_eyes() {
    let mut state = WhiteboxState {
        stance: Stance::Angry,
        profile: ExpressionProfile::Serious,
        ..WhiteboxState::default()
    };

    state.tick_count = 10;
    let frame = resolve_avatar(&state);
    assert_eq!((frame.eyes, frame.mouth), ("eyes_serious_angry", "mouth_pout_loop"));
}

#[test]
fn app_consumes_superclaw_speech_directive() {
    let (tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    tx.send(SpeechDirective {
        text: "hello from superclaw".into(),
        tone: "warm".into(),
        interaction_intent: "answer".into(),
        interruptible: true,
    })
    .unwrap();

    let mut app = App::new(rx, irx);
    app.drain_external();

    assert_eq!(app.state.speak_state, SpeakState::Speaking);
    assert_eq!(
        app.state.current_text.as_deref(),
        Some("hello from superclaw")
    );
    assert_eq!(app.state.current_text_kind, Some(CurrentTextKind::Spoken));
}

#[test]
fn app_can_build_voice_from_config() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let cfg = VoiceConfig::default();
    let app = App::with_voice_config(rx, irx, cfg);

    assert!(!app.stt.is_listening());
    assert!(!app.tts.is_speaking());
}

#[test]
fn stt_manager_tracks_listening_state() {
    let backend = MockSttBackend::default();
    let mut stt = SttManager::new(Box::new(backend));

    assert!(!stt.is_listening());
    stt.start_listening().unwrap();
    assert!(stt.is_listening());
    stt.stop_listening().unwrap();
    assert!(!stt.is_listening());
}

#[test]
fn stt_manager_polls_transcript_events() {
    let mut backend = MockSttBackend::default();
    backend.queue_event(SttEvent::TranscriptFinal("hello human".into()));
    let mut stt = SttManager::new(Box::new(backend));

    let event = stt.poll_event().expect("event should exist");
    assert_eq!(event, SttEvent::TranscriptFinal("hello human".into()));
}

#[test]
fn tts_manager_tracks_speaking_state() {
    let backend = MockTtsBackend::default();
    let mut tts = TtsManager::new(Box::new(backend));

    assert!(!tts.is_speaking());
    tts.speak("hello", SpeechTone::Warm).unwrap();
    assert!(tts.is_speaking());
    tts.stop().unwrap();
    assert!(!tts.is_speaking());
}

#[test]
fn stt_factory_builds_mock_backend() {
    let backend = build_stt_backend(SttBackendKind::Mock);
    let mut stt = SttManager::new(backend);
    stt.start_listening().unwrap();
    assert!(stt.is_listening());
}

#[test]
fn tts_factory_builds_mock_backend() {
    let backend = build_tts_backend(TtsBackendKind::Mock);
    let mut tts = TtsManager::new(backend);
    tts.speak("hi", SpeechTone::Neutral).unwrap();
    assert!(tts.is_speaking());
}

#[derive(Default)]
struct CompletingTts {
    speaking: bool,
    complete_on_poll: bool,
}

impl TtsBackend for CompletingTts {
    fn speak(&mut self, _text: &str, _tone: SpeechTone) -> Result<(), String> {
        self.speaking = true;
        self.complete_on_poll = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        self.speaking = false;
        self.complete_on_poll = false;
        Ok(())
    }

    fn poll_complete(&mut self) -> Result<bool, String> {
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
fn request_speak_stops_listening_before_speaking() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);

    app.begin_listening().unwrap();
    app.request_speak("hello".into(), "pet-like".into()).unwrap();

    assert!(!app.stt.is_listening());
    assert_eq!(app.state.listen_state, ListenState::Idle);
    assert_eq!(app.state.speak_state, SpeakState::Speaking);
}

#[test]
fn tick_finishes_speech_when_backend_process_completes() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);
    app.replace_tts(Box::new(CompletingTts::default()));

    app.request_speak("hello".into(), "serious".into()).unwrap();
    assert_eq!(app.state.speak_state, SpeakState::Speaking);

    app.tick();

    assert_eq!(app.state.speak_state, SpeakState::Idle);
    assert_eq!(app.state.mode, Mode::Expression);
    assert_eq!(app.state.profile, ExpressionProfile::Neutral);
}

#[test]
fn default_voice_config_uses_mock_backends() {
    let cfg = VoiceConfig::default();
    assert_eq!(cfg.stt_backend, SttBackendKind::Mock);
    assert_eq!(cfg.tts_backend, TtsBackendKind::Mock);
}

#[test]
fn piper_backend_returns_clear_error_for_missing_binary() {
    let mut tts = PiperTts::with_command("definitely-not-a-real-piper-binary");
    let err = tts.speak("hello", SpeechTone::Neutral).unwrap_err();
    assert!(err.contains("piper"));
}

#[test]
fn whisper_backend_returns_clear_error_for_missing_binary() {
    let mut stt = WhisperStt::with_command("definitely-not-a-real-whisper-binary");
    let err = stt
        .transcribe_file(std::path::Path::new("missing.wav"))
        .unwrap_err();
    assert!(err.contains("whisper"));
}

#[test]
fn whisper_live_capture_start_failure_is_clear() {
    let mut stt = WhisperStt::with_commands(
        "definitely-not-a-real-whisper-binary",
        "definitely-not-a-real-recorder-binary",
    );
    let err = stt.start_listening().unwrap_err();
    assert!(err.contains("recorder"));
}

#[test]
fn piper_backend_stop_clears_speaking_even_without_process() {
    let mut tts = PiperTts::with_command("definitely-not-a-real-piper-binary");
    let _ = tts.speak("hello", SpeechTone::Neutral);
    let _ = tts.stop();
    assert!(!tts.is_speaking());
}

#[test]
fn system_tts_backend_returns_clear_error_for_missing_binary() {
    let mut tts = SystemTts::with_command("definitely-not-a-real-system-tts-binary");
    let err = tts.speak("hello", SpeechTone::Neutral).unwrap_err();
    assert!(err.contains("system tts"));
}

#[test]
fn app_consumes_info_directive() {
    let (_tx, rx) = mpsc::channel();
    let (itx, irx) = mpsc::channel();
    itx.send(InfoDirective {
        source: "graphstore".into(),
        fragment: "memory linked".into(),
    })
    .unwrap();

    let mut app = App::new(rx, irx);
    app.drain_external();

    assert_eq!(app.state.mode, Mode::Info);
    assert_eq!(
        app.state.info_fragments.last().map(String::as_str),
        Some("[graphstore] memory linked")
    );
    assert!(app
        .state
        .diagnostics
        .last()
        .map(String::as_str)
        .unwrap_or("")
        .contains("graphstore"));
}

#[test]
fn app_polls_stt_events_into_runtime_state() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);

    app.replace_stt(Box::new({
        let mut backend = MockSttBackend::default();
        backend.queue_event(SttEvent::TranscriptFinal("hello there".into()));
        backend
    }));

    app.drain_voice();

    assert_eq!(app.state.listen_state, ListenState::Ready);
    assert_eq!(app.state.current_text.as_deref(), Some("hello there"));
    assert_eq!(app.state.current_text_kind, Some(CurrentTextKind::Heard));
}

#[test]
fn app_routes_speech_directive_into_tts_manager() {
    let (tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    tx.send(SpeechDirective {
        text: "speak now".into(),
        tone: "warm".into(),
        interaction_intent: "answer".into(),
        interruptible: true,
    })
    .unwrap();

    let mut app = App::new(rx, irx);
    app.replace_tts(Box::new(MockTtsBackend::default()));
    app.drain_external();

    assert!(app.tts.is_speaking());
    assert_eq!(app.state.speak_state, SpeakState::Speaking);
}

#[derive(Default)]
struct FailingTtsBackend;

impl TtsBackend for FailingTtsBackend {
    fn speak(&mut self, _text: &str, _tone: SpeechTone) -> Result<(), String> {
        Err("tts failed".into())
    }

    fn stop(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn is_speaking(&self) -> bool {
        false
    }
}

#[test]
fn app_does_not_claim_speaking_when_tts_fails() {
    let (tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    tx.send(SpeechDirective {
        text: "fail to speak".into(),
        tone: "warm".into(),
        interaction_intent: "answer".into(),
        interruptible: true,
    })
    .unwrap();

    let mut app = App::new(rx, irx);
    app.replace_tts(Box::new(FailingTtsBackend));
    app.drain_external();

    assert_eq!(app.state.speak_state, SpeakState::Idle);
    assert!(app
        .state
        .diagnostics
        .last()
        .map(String::as_str)
        .unwrap_or("")
        .contains("speech directive failed"));
}

#[test]
fn app_begin_listening_stops_speaking_backend_first() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);
    app.tts.speak("hello", SpeechTone::Warm).unwrap();
    app.state.speak_state = SpeakState::Speaking;
    app.state.current_text = Some("hello".into());
    app.state.current_text_kind = Some(CurrentTextKind::Spoken);

    app.begin_listening().unwrap();

    assert!(!app.tts.is_speaking());
    assert_eq!(app.state.speak_state, SpeakState::Idle);
    assert_eq!(app.state.current_text, None);
}

#[test]
fn app_shutdown_stops_voice_backends() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);

    app.stt.start_listening().unwrap();
    app.tts.speak("bye", SpeechTone::Neutral).unwrap();
    app.shutdown().unwrap();

    assert!(!app.stt.is_listening());
    assert!(!app.tts.is_speaking());
}

#[test]
fn app_begin_listening_uses_stt_backend() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);

    app.begin_listening().unwrap();

    assert!(app.stt.is_listening());
    assert_eq!(app.state.listen_state, ListenState::Hearing);
}

#[test]
fn app_end_listening_stops_backend_and_sets_debounce() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let mut app = App::new(rx, irx);

    app.begin_listening().unwrap();
    app.end_listening().unwrap();

    assert!(!app.stt.is_listening());
    assert_eq!(app.state.listen_state, ListenState::Debouncing);
}

#[test]
fn app_config_parses_backend_names_from_env() {
    std::env::set_var("WHITEBOX_STT_BACKEND", "whisper");
    std::env::set_var("WHITEBOX_TTS_BACKEND", "piper");
    std::env::set_var("WHITEBOX_STT_COMMAND", "custom-whisper");
    std::env::set_var("WHITEBOX_TTS_COMMAND", "custom-piper");
    std::env::set_var("WHITEBOX_STT_MODEL", "/models/stt/whisper.bin");
    std::env::set_var("WHITEBOX_TTS_MODEL", "/models/tts/piper.onnx");
    std::env::set_var("WHITEBOX_TTS_CONFIG", "/models/tts/piper.onnx.json");
    std::env::set_var("WHITEBOX_RENDER_WIDTH", "800");
    std::env::set_var("WHITEBOX_RENDER_HEIGHT", "600");

    let cfg = AppConfig::from_env();
    assert_eq!(cfg.voice.stt_backend, SttBackendKind::Whisper);
    assert_eq!(cfg.voice.tts_backend, TtsBackendKind::Piper);
    assert_eq!(cfg.voice.stt_command.as_deref(), Some("custom-whisper"));
    assert_eq!(cfg.voice.tts_command.as_deref(), Some("custom-piper"));
    assert_eq!(
        cfg.voice.stt_model.as_deref(),
        Some("/models/stt/whisper.bin")
    );
    assert_eq!(
        cfg.voice.tts_model.as_deref(),
        Some("/models/tts/piper.onnx")
    );
    assert_eq!(
        cfg.voice.tts_config.as_deref(),
        Some("/models/tts/piper.onnx.json")
    );
    assert_eq!(cfg.render.width, 800);
    assert_eq!(cfg.render.height, 600);

    std::env::remove_var("WHITEBOX_STT_BACKEND");
    std::env::remove_var("WHITEBOX_TTS_BACKEND");
    std::env::remove_var("WHITEBOX_STT_COMMAND");
    std::env::remove_var("WHITEBOX_TTS_COMMAND");
    std::env::remove_var("WHITEBOX_STT_MODEL");
    std::env::remove_var("WHITEBOX_TTS_MODEL");
    std::env::remove_var("WHITEBOX_TTS_CONFIG");
    std::env::remove_var("WHITEBOX_RENDER_WIDTH");
    std::env::remove_var("WHITEBOX_RENDER_HEIGHT");
}

#[test]
fn app_config_parses_backend_names_from_file() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("whitebox-config-{stamp}.env"));
    fs::write(
        &path,
        "WHITEBOX_STT_BACKEND=whisper\nWHITEBOX_TTS_BACKEND=piper\nWHITEBOX_STT_COMMAND=file-whisper\nWHITEBOX_TTS_COMMAND=file-piper\nWHITEBOX_STT_MODEL=/file/stt/model.bin\nWHITEBOX_TTS_MODEL=/file/tts/model.onnx\nWHITEBOX_TTS_CONFIG=/file/tts/model.onnx.json\nWHITEBOX_RENDER_WIDTH=640\nWHITEBOX_RENDER_HEIGHT=480\n",
    )
    .unwrap();

    let cfg = AppConfig::from_file(&path).unwrap();
    assert_eq!(cfg.voice.stt_backend, SttBackendKind::Whisper);
    assert_eq!(cfg.voice.tts_backend, TtsBackendKind::Piper);
    assert_eq!(cfg.voice.stt_command.as_deref(), Some("file-whisper"));
    assert_eq!(cfg.voice.tts_command.as_deref(), Some("file-piper"));
    assert_eq!(cfg.voice.stt_model.as_deref(), Some("/file/stt/model.bin"));
    assert_eq!(cfg.voice.tts_model.as_deref(), Some("/file/tts/model.onnx"));
    assert_eq!(
        cfg.voice.tts_config.as_deref(),
        Some("/file/tts/model.onnx.json")
    );
    assert_eq!(cfg.render.width, 640);
    assert_eq!(cfg.render.height, 480);

    let _ = fs::remove_file(path);
}

#[test]
fn app_config_prefers_env_over_file() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("whitebox-config-override-{stamp}.env"));
    fs::write(
        &path,
        "WHITEBOX_STT_BACKEND=mock\nWHITEBOX_TTS_BACKEND=mock\nWHITEBOX_RENDER_WIDTH=640\nWHITEBOX_RENDER_HEIGHT=480\n",
    )
    .unwrap();

    std::env::set_var("WHITEBOX_STT_BACKEND", "whisper");
    std::env::set_var("WHITEBOX_RENDER_WIDTH", "900");

    let cfg = AppConfig::load(Some(&path)).unwrap();
    assert_eq!(cfg.voice.stt_backend, SttBackendKind::Whisper);
    assert_eq!(cfg.render.width, 900);
    assert_eq!(cfg.render.height, 480);

    std::env::remove_var("WHITEBOX_STT_BACKEND");
    std::env::remove_var("WHITEBOX_RENDER_WIDTH");
    let _ = fs::remove_file(path);
}

#[test]
fn app_state_exposes_runtime_backend_labels() {
    let (_tx, rx) = mpsc::channel();
    let (_itx, irx) = mpsc::channel();
    let cfg = AppConfig {
        render: RenderConfig {
            width: 900,
            height: 700,
        },
        voice: VoiceConfig {
            stt_backend: SttBackendKind::Whisper,
            tts_backend: TtsBackendKind::System,
            stt_command: Some("custom-whisper".into()),
            tts_command: Some("custom-system".into()),
            stt_model: None,
            tts_model: None,
            tts_config: None,
        },
    };
    let app = App::from_config(rx, irx, cfg);
    let scene = app.scene();

    assert_eq!(scene.stt_backend.as_deref(), Some("Whisper"));
    assert_eq!(scene.tts_backend.as_deref(), Some("System"));
    assert_eq!(scene.render_size, (900, 700));
}
