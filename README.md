<div align="center">

# whitebox

**Give your agent a body. It listens, speaks, and shows up on screen.**

[![Rust](https://img.shields.io/badge/Rust-1.78-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2-blue?logo=tauri&logoColor=white)](https://tauri.app/)
[![Platform](https://img.shields.io/badge/Platform-Desktop-lightgrey?logo=windows&logoColor=white)](https://tauri.app/)

</div>

---

Whitebox is a body runtime for desktop agents. You drive it with a small set of body verbs - `listen()`, `speak(text)`, `set_stance(name)` - and it handles everything else: rendering a layered cat avatar, managing voice sessions, and emitting lifecycle events so your agent knows what's happening.

It does not think, plan, or remember. That's your system's job. Whitebox just gives it a face.

## 🧠 Body API

The full contract is in [`WHITEBOX_API.md`](./WHITEBOX_API.md). The short version:

### Commands

| Command | What happens |
|---------|-------------|
| `listen()` | Start a listening session. Interrupts active speech first. Returns a session ID. |
| `stop_listening()` | End the active listening session. |
| `speak(text)` | Start speaking. Interrupts anything else first. Returns a session ID. |
| `stop_speaking()` | Stop current speech. |
| `set_stance(name)` | Change emotional posture. Affects avatar expression and voice tone. |
| `set_status(text)` | Show a short status string on screen. |
| `clear_status()` | Clear it. |

### Events

```
on_listen_started(id)
on_listen_partial(id, text)       # transcript in progress
on_listen_final(id, text)         # transcript complete
on_listen_stopped(id, reason)

on_speak_started(id, text)
on_speak_stopped(id, reason)

on_error(code, message, cause?)
```

Stop reasons: `completed` `cancelled` `interrupted` `error`

Error codes: `busy` `backend_unavailable` `invalid_stance` `no_active_session` `internal_error`

### Stances

```
neutral  warm  playful  curious  alert  focused  guarded  stern  tired  sad  angry
```

Stance sets the emotional baseline. Voice tone follows automatically: `warm` sounds warm, `playful` sounds pet-like, anything alert/stern/angry goes serious.

Activity (idle / listening / thinking / speaking) overlays on top. You don't set it directly - it follows body actions.

### Example

```rust
let mut body = WhiteboxBody::new();

body.set_stance(Stance::Curious)?;
body.set_status("Thinking...");

let id = body.speak("Hello there.")?;

body.tick(); // drive the event loop
let events = body.drain_events();
```

## 🐱 Avatar

A layered cat built from PNG parts. Each stance maps to an expression profile, which picks a combination of face, eyes, mouth, and ears assets.

<details>
<summary>Expression profiles</summary>

| Profile | Face | Eyes | Mouth | Ears |
|---------|------|------|-------|------|
| Neutral | blush | open_blush | flat_neutral | rounded |
| PetLike | rose | happy_closed | cat_smile | rounded |
| Serious | blush | serious_angry | chevron_serious | sharp |
| Curious | rose | open_rose | tiny_triangle | sharp |
| Sleepy | blush | sleepy_flat | flat_neutral | rounded |

</details>

<details>
<summary>Available assets</summary>

```
cat/
  face/     face_fill_blush, face_fill_rose
  eyes/     11 variants (open, happy, serious, sleepy, teary, worried, ...)
  mouth/    10 variants (flat, smile, frown, wavy, tongue, pout, ...)
  ears/     ears_style_rounded, ears_style_sharp
```

</details>

## 🎙️ Voice backends

Backends are swappable at runtime via `WhiteboxBody::replace_stt` / `replace_tts`. Drop in `mock` for tests, real backends for production.

| Backend | Kind | Notes |
|---------|------|-------|
| `whisper` | STT | OpenAI Whisper running locally |
| `moonshine_sidecar` | STT | Moonshine model via sidecar process |
| `piper` | TTS | Piper neural TTS |
| `system_tts` | TTS | OS-native TTS, zero setup |
| `mock` | STT + TTS | Silent, event-accurate, great for tests |

## 🚀 Getting started

```bash
npm install
npm run tauri dev
```

```bash
# release build
npm run tauri build

# tests
cargo test
npm test
```

## 🗂️ Project structure

<details>
<summary>Expand</summary>

```
src/                        Rust core library
  api.rs                    WhiteboxBody, Stance, Activity (public body runtime)
  state.rs                  WhiteboxState (internal render state)
  modes.rs                  Mode (Expression / Info / Interaction)
  profiles.rs               ExpressionProfile + PresetLibrary
  avatar.rs                 Avatar assembly
  animation.rs              Animation engine
  scene.rs                  Scene graph
  render/                   Render pipeline
  clips.rs                  Clip playback
  assets.rs                 Asset loading
  app.rs                    Top-level App (wires state, voice, scene)
  reducer.rs                State reducer
  events.rs                 Internal event types
  config.rs                 Configuration
  voice/
    mod.rs                  SttBackend + TtsBackend traits
    types.rs                SpeechTone, SttEvent, VoiceResult
    backends/               whisper, piper, moonshine_sidecar, system_tts, mock
src-tauri/                  Tauri shell (desktop window, IPC bridge)
src-web/                    Web frontend (Vite/JS)
cat/                        Avatar PNG assets
tests/                      Integration tests (Rust + JS)
docs/                       Design specs
```

</details>

## ⚙️ Tech

Rust · Tauri 2 · Vite/JS
