# Whitebox - Status Summary

## What works

- Frame-based animation system (Rust `src/animations/` + JS `src-web/animations/`)
- 11 stances with IDLE / SPEAKING / LISTENING frame arrays, Disney animation principles applied
- WhiteboxBody API (listen, speak, set_stance, set_status, clear_status, tick, drain_events)
- Superclaw contract (`src/superclaw.rs`) - Pet wrapper with say/feel/listen/drain
- Tauri shell (`src-tauri/src/main.rs`) - all commands wired, snapshot rendering
- JS frontend (avatar-contract.js uses frame arrays, browser-runtime.js matches Tauri path)
- Voice backends (whisper, piper, moonshine_sidecar, system_tts, mock)
- 12fps tick rate (83ms), correct for this frame-array system

## What needs fixing

### Broken tests

`tests/whitebox_state.rs`:
- Imports `whitebox::integrations::info_feed::InfoDirective` - module was deleted
- Imports `whitebox::integrations::superclaw::SpeechDirective` - module was deleted
- References `profile: ExpressionProfile::PetLike` on WhiteboxState - field was removed
- Calls `App::new(rx, irx)` - signature changed to `App::new()` (no params)
- Multiple test functions affected (lines 16-17, 131, 159, 183, 198, 214, 232, 252, 269, 282-290, 306)

`tests/whitebox_api.rs`:
- Likely has similar issues - needs verification

### Dead files on disk (delete these)

- `src/clips.rs` - not in lib.rs, orphaned. Old clip selection logic replaced by frame arrays.
- `src/profiles.rs` - not in lib.rs, orphaned. ExpressionProfile/PresetLibrary replaced by frame arrays.
- `src/animation.rs` - not in lib.rs, orphaned. Old AnimationPlayer/AnimationClip/AnimationFrame replaced by frame arrays.
- `src/bin/` - empty directory
- `src/render/` - empty directory

### Dead code paths

- `Mode::Info` + `WhiteboxEvent::InfoFragment` - the reducer handles InfoFragment events (reducer.rs:43-50) and sets `Mode::Info`, but nothing produces InfoFragment events anymore. The integrations (superclaw, info_feed) that sent these were removed. This is a live code path that can never fire.
- `AssetManifest` in `src/assets.rs` - only referenced by the broken test file, not used in library code

### Decisions needed

1. `Mode::Info` / `InfoFragment` - remove entirely, or keep for when superclaw integration is built?
2. `AssetManifest` - remove, or keep as a utility for future asset validation?
3. Tests - rewrite to match new API (no profile, no integrations, new App::new() signature), or start fresh?
