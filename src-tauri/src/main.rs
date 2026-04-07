use std::io::Cursor;
use std::sync::Mutex;

use base64::Engine;
use image::ImageFormat;
use serde::Serialize;
use tauri::State;
use whitebox::api::{BodyEvent, Stance, WhiteboxBody};
use whitebox::app::App;
use whitebox::config::AppConfig;
use whitebox::state::CurrentTextKind;

struct WhiteboxRuntime {
    body: Mutex<WhiteboxBody>,
    // Cache the last rendered avatar so we only recompose when parts change.
    // Stores (face, eyes, mouth, ears, base64_png).
    avatar_cache: Mutex<Option<(String, String, String, String, String)>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AvatarParts {
    face: String,
    eyes: String,
    mouth: String,
    ears: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UiSnapshot {
    stance: String,
    stance_label: String,
    activity: String,
    listen_state: String,
    speak_state: String,
    current_text: Option<String>,
    current_text_kind: Option<String>,
    tone: Option<String>,
    status_text: Option<String>,
    info_fragments: Vec<String>,
    diagnostics: Vec<String>,
    render_size: (u32, u32),
    stt_backend: String,
    tts_backend: String,
    avatar_parts: AvatarParts,
    avatar_data_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UiEvent {
    kind: String,
    id: Option<u64>,
    text: Option<String>,
    reason: Option<String>,
    code: Option<String>,
    message: Option<String>,
    cause: Option<String>,
}

#[tauri::command]
fn snapshot(runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn tick(runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.tick();
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn listen(runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.listen().map_err(format_body_error)?;
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn stop_listening(runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.stop_listening().map_err(format_body_error)?;
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn speak(text: String, runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.speak(text).map_err(format_body_error)?;
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn stop_speaking(runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.stop_speaking().map_err(format_body_error)?;
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn set_stance(stance: String, runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    let stance = parse_stance(&stance)?;
    body.set_stance(stance).map_err(format_body_error)?;
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn set_status(text: String, runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.set_status(text);
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn clear_status(runtime: State<'_, WhiteboxRuntime>) -> Result<UiSnapshot, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    body.clear_status();
    let mut cache = runtime.avatar_cache.lock().map_err(|_| "cache lock failed".to_string())?;
    make_snapshot(&body, &mut cache)
}

#[tauri::command]
fn drain_events(runtime: State<'_, WhiteboxRuntime>) -> Result<Vec<UiEvent>, String> {
    let mut body = runtime
        .body
        .lock()
        .map_err(|_| "failed to lock body state".to_string())?;
    Ok(body
        .drain_events()
        .into_iter()
        .map(body_event_to_ui)
        .collect())
}

fn main() {
    tauri::Builder::default()
        .manage(WhiteboxRuntime {
            body: Mutex::new(build_body().expect("whitebox body should initialize")),
            avatar_cache: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            snapshot,
            tick,
            listen,
            stop_listening,
            speak,
            stop_speaking,
            set_stance,
            set_status,
            clear_status,
            drain_events
        ])
        .run(tauri::generate_context!())
        .expect("error while running whitebox");
}

fn build_body() -> Result<WhiteboxBody, String> {
    let cfg_path = std::env::var("WHITEBOX_CONFIG").ok();
    let cfg = AppConfig::load(cfg_path.as_deref().map(std::path::Path::new))?;
    let app = App::from_config(cfg);
    Ok(WhiteboxBody::from_app(app))
}

type AvatarCache = Option<(String, String, String, String, String)>;

fn make_snapshot(body: &WhiteboxBody, cache: &mut AvatarCache) -> Result<UiSnapshot, String> {
    let body_snapshot = body.snapshot();
    let scene = body.app().scene();
    let avatar = &scene.avatar;
    let data_url = avatar_data_url_cached(body, avatar, cache)?;
    Ok(UiSnapshot {
        stance: body_snapshot.stance.as_str().to_string(),
        stance_label: body_snapshot.stance.label().to_string(),
        activity: body_snapshot.activity.as_str().to_string(),
        listen_state: format!("{:?}", scene.listen_state),
        speak_state: format!("{:?}", scene.speak_state),
        current_text: scene.current_text,
        current_text_kind: scene.current_text_kind.map(text_kind_label),
        tone: scene.tone,
        status_text: body_snapshot.status_text,
        info_fragments: scene.info_fragments,
        diagnostics: scene.diagnostics.into_iter().rev().take(8).collect(),
        render_size: scene.render_size,
        stt_backend: scene.stt_backend.unwrap_or_else(|| "-".into()),
        tts_backend: scene.tts_backend.unwrap_or_else(|| "-".into()),
        avatar_parts: AvatarParts {
            face: avatar.face.to_string(),
            eyes: avatar.eyes.to_string(),
            mouth: avatar.mouth.to_string(),
            ears: avatar.ears.to_string(),
        },
        avatar_data_url: data_url,
    })
}

fn avatar_data_url_cached(
    body: &WhiteboxBody,
    avatar: &whitebox::avatar::AvatarView,
    cache: &mut AvatarCache,
) -> Result<String, String> {
    let key = (
        avatar.face.to_string(),
        avatar.eyes.to_string(),
        avatar.mouth.to_string(),
        avatar.ears.to_string(),
    );
    if let Some((cf, ce, cm, cr, ref url)) = *cache {
        if cf == key.0 && ce == key.1 && cm == key.2 && cr == key.3 {
            return Ok(url.clone());
        }
    }
    let image = body.app().composed_avatar_image().map_err(|e| e.to_string())?;
    let mut png = Cursor::new(Vec::new());
    image.write_to(&mut png, ImageFormat::Png).map_err(|e| e.to_string())?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(png.into_inner());
    let url = format!("data:image/png;base64,{encoded}");
    *cache = Some((key.0, key.1, key.2, key.3, url.clone()));
    Ok(url)
}

fn text_kind_label(kind: CurrentTextKind) -> String {
    match kind {
        CurrentTextKind::Spoken => "spoken_text".into(),
        CurrentTextKind::Heard => "heard_text".into(),
    }
}

fn parse_stance(value: &str) -> Result<Stance, String> {
    match value {
        "neutral" => Ok(Stance::Neutral),
        "warm" => Ok(Stance::Warm),
        "playful" => Ok(Stance::Playful),
        "curious" => Ok(Stance::Curious),
        "alert" => Ok(Stance::Alert),
        "focused" => Ok(Stance::Focused),
        "guarded" => Ok(Stance::Guarded),
        "stern" => Ok(Stance::Stern),
        "tired" => Ok(Stance::Tired),
        "sad" => Ok(Stance::Sad),
        "angry" => Ok(Stance::Angry),
        other => Err(format!("unknown stance: {other}")),
    }
}

fn format_body_error(error: whitebox::api::BodyError) -> String {
    match error.cause {
        Some(cause) => format!("{}: {} ({cause})", error.code.as_str(), error.message),
        None => format!("{}: {}", error.code.as_str(), error.message),
    }
}

fn body_event_to_ui(event: BodyEvent) -> UiEvent {
    match event {
        BodyEvent::ListenStarted { id } => UiEvent {
            kind: "listen_started".into(),
            id: Some(id.0),
            text: None,
            reason: None,
            code: None,
            message: None,
            cause: None,
        },
        BodyEvent::ListenPartial { id, text } => UiEvent {
            kind: "listen_partial".into(),
            id: Some(id.0),
            text: Some(text),
            reason: None,
            code: None,
            message: None,
            cause: None,
        },
        BodyEvent::ListenFinal { id, text } => UiEvent {
            kind: "listen_final".into(),
            id: Some(id.0),
            text: Some(text),
            reason: None,
            code: None,
            message: None,
            cause: None,
        },
        BodyEvent::ListenStopped { id, reason } => UiEvent {
            kind: "listen_stopped".into(),
            id: Some(id.0),
            text: None,
            reason: Some(reason.as_str().into()),
            code: None,
            message: None,
            cause: None,
        },
        BodyEvent::SpeakStarted { id, text } => UiEvent {
            kind: "speak_started".into(),
            id: Some(id.0),
            text: Some(text),
            reason: None,
            code: None,
            message: None,
            cause: None,
        },
        BodyEvent::SpeakStopped { id, reason } => UiEvent {
            kind: "speak_stopped".into(),
            id: Some(id.0),
            text: None,
            reason: Some(reason.as_str().into()),
            code: None,
            message: None,
            cause: None,
        },
        BodyEvent::Error {
            code,
            message,
            cause,
        } => UiEvent {
            kind: "error".into(),
            id: None,
            text: None,
            reason: None,
            code: Some(code.as_str().into()),
            message: Some(message),
            cause,
        },
    }
}
