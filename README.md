# whitebox

Whitebox is the **Surface Plane**: the layer the agent uses to express itself visibly and audibly.

It owns:
- a **layered cat avatar** rendered in a Tauri desktop window
- a **voice layer** (speech-to-text + text-to-speech) with swappable backends
- a **body runtime** (`WhiteboxBody`) with a small, semantic API

It does not own reasoning, planning, memory, or conversation logic. Those belong to the agent. Whitebox only gives the agent a body.

## Role in the architecture

```
Master Agent
    └── InteractionPlane (POWER)   <- human/agent interaction
    └── Surface Plane (whitebox)   <- visible + spoken expression  <- you are here
    └── Graphstore                 <- memory + cognition
```

The agent drives whitebox with body verbs: `listen()`, `speak(text)`, `set_stance(name)`, `set_status(text)`. Whitebox handles rendering, voice backends, and lifecycle events.

## Body API

The full API contract lives in [`WHITEBOX_API.md`](./WHITEBOX_API.md). Summary:

### Commands

| Command | What |
|---------|------|
| `listen()` | Start a listening session. Interrupts active speech first. Returns `session_id`. |
| `stop_listening()` | End the active listening session. |
| `speak(text)` | Start a speaking session. Interrupts active listen/speech first. Returns `session_id`. |
| `stop_speaking()` | End the active speaking session. |
| `set_stance(name)` | Set the emotional posture. Changes expression baseline. |
| `set_status(text)` | Set a short visible status string. |
| `clear_status()` | Clear the visible status string. |

### Events

```
on_listen_started(id)
on_listen_partial(id, text)
on_listen_final(id, text)
on_listen_stopped(id, reason)      # reason: completed | cancelled | interrupted | error

on_speak_started(id, text)
on_speak_stopped(id, reason)

on_error(code, message, cause?)    # codes: busy | backend_unavailable | invalid_stance
                                   #        no_active_session | internal_error
```

### Stance vocabulary

```
neutral  warm  playful  curious  alert  focused  guarded  stern  tired  sad  angry
```

Stance sets the emotional baseline. Activity (idle / listening / thinking / speaking) is derived from body actions and overlays on top of stance.

## Avatar

The avatar is a layered cat assembled from PNG assets in `cat/`:

```
cat/
  face/     face_fill_blush, face_fill_rose
  eyes/     11 variants (open, happy, serious, sleepy, teary, ...)
  mouth/    10 variants (flat, smile, frown, wavy, tongue, ...)
  ears/     ears_style_rounded, ears_style_sharp
```

Expression profiles map a stance to a specific combination of assets:

| Profile | Face | Eyes | Mouth | Ears |
|---------|------|------|-------|------|
| Neutral | blush | open_blush | flat_neutral | rounded |
| PetLike | rose | happy_closed | cat_smile | rounded |
| Serious | blush | serious_angry | chevron_serious | sharp |
| Curious | rose | open_rose | tiny_triangle | sharp |
| Sleepy | blush | sleepy_flat | flat_neutral | rounded |

## Voice backends

Backends are swappable at runtime via `WhiteboxBody::replace_stt` / `replace_tts`.

| Backend | Type | Notes |
|---------|------|-------|
| `whisper` | STT | OpenAI Whisper local inference |
| `moonshine_sidecar` | STT | Moonshine model via sidecar process |
| `piper` | TTS | Piper neural TTS |
| `system_tts` | TTS | OS-native TTS (fallback) |
| `mock` | STT + TTS | For tests, no audio I/O |

Speech tone is derived automatically from stance. `warm` uses warm tone, `playful` uses pet-like tone, alert/focused/stern/angry use serious tone, everything else is neutral.

## Development

```bash
npm install
npm run tauri dev
```

Build release:

```bash
npm run tauri build
```

Run tests:

```bash
cargo test
npm test
```

## Project structure

```
src/                        Rust core library
  api.rs                    Stance, Activity, WhiteboxBody (public body runtime)
  state.rs                  WhiteboxState (internal render state)
  modes.rs                  Mode enum (Expression / Info / Interaction)
  profiles.rs               ExpressionProfile + PresetLibrary (stance to avatar assets)
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
    mod.rs                  SttBackend + TtsBackend traits, SttManager, TtsManager
    types.rs                SpeechTone, SttEvent, VoiceResult
    factory.rs              Backend factory
    registry.rs             Backend registry
    backends/               whisper, piper, moonshine_sidecar, system_tts, mock
  integrations/
    superclaw.rs            Superclaw agent integration
    info_feed.rs            Info feed integration
src-tauri/                  Tauri shell (desktop window + IPC bridge)
src-web/                    Web frontend (Vite/JS)
  avatar-contract.js        Avatar rendering contract
  browser-runtime.js        Browser-side runtime
  control-policy.js         Input control policy
  main.js                   Entry point
  styles.css
cat/                        Avatar PNG assets (face, eyes, mouth, ears, icons, previews)
tests/                      Integration tests (Rust + JS)
docs/                       Design specs
```

## Tech

Rust · Tauri 2 · Vite/JS
