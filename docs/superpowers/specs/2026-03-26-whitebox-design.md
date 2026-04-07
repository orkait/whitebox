# Whitebox Design Spec

**Goal:** Build `whitebox` as a terminal-only embodied surface runtime that gives the agent visible life, conversational presence, and real-time system expression without embedding planning or intelligence.

**Architecture:** Whitebox is a dumb but expressive surface layer. It listens, debounces, speaks, renders, and reacts, while external systems decide content and intent. For v1, Whitebox is Ratatui-first and terminal-only, but it keeps a renderer-independent internal state and scene model so rendering logic does not leak into behavior logic.

**Tech Stack:** Rust, Ratatui, terminal backend, local avatar asset pipeline, event-driven runtime, TTS/STT integration hooks

---

## 1. Product Definition

Whitebox is the layer where the agent becomes perceivable to humans.

It is responsible for:

- visible expression
- interactive conversation surface behavior
- real-time information presentation
- speaking/listening state feedback
- screen-based embodied presence

Whitebox is not responsible for:

- planning
- reasoning
- choosing what to say
- tool execution
- memory retrieval
- orchestration

In one line:

> Whitebox is the agent’s embodied terminal surface.

## 2. Whitebox v1 Scope

Whitebox v1 is terminal-only and Ratatui-first.

This means:

- the only renderer in v1 is Ratatui
- the visible surface must work inside a terminal
- the canonical visual space is configurable, with `1024x1024` as the default design space
- the initial avatar system is built from the cat assets already staged under `whitebox/cat/`

Out of scope for v1:

- web/canvas renderer
- native desktop renderer
- advanced multi-avatar scenes
- large dashboard-like layout systems

## 3. Core Modes

Whitebox has three primary modes.

### 3.1 Expression Mode

The default mode.

Purpose:

- keep the agent visibly alive
- show emotional presence
- express mood, attention, and softness

Typical output:

- gaze
- blink
- breathing motion
- posture shifts
- emotional expression

### 3.2 Info Mode

A real-time monitoring and information surface.

Purpose:

- show compact live information from connected systems
- stream small words or phrases at a time
- act as a system-watch surface for the whole stack

Typical output:

- typewriter-style fragments
- compact status updates
- short source-tagged system cues
- alerts and small feed elements

### 3.3 Interaction Mode

A direct conversational engagement mode.

Purpose:

- support turn-taking
- react visibly while listening
- show speaking and response behavior

Typical output:

- listening cues
- speaking cues
- acknowledgement reactions
- immediate conversational feedback

## 4. Expression Profiles

Expression profiles are not top-level modes.
They are stylistic behaviors under expression rendering.

Initial profile set:

- neutral
- pet-like
- serious
- curious
- sleepy

The `pet-like` profile is especially important because the desired interaction
model is close to "person talks to pet, pet talks back".

## 5. Product Behavior

### 5.1 Whitebox Is Dumb

Whitebox must remain a surface runtime, not an intelligent agent.

It may:

- listen
- debounce
- speak
- animate
- render
- switch modes
- express based on signals

It must not:

- generate task plans
- decide final speech content
- infer long-term strategy
- mutate cognitive state by itself

### 5.2 Turn-Taking Ownership

Whitebox owns the interaction UX around speech and listening:

- detecting listening start
- debounce before utterance completion
- switching speaking vs listening state
- handling interruption and overlap

Whitebox does not decide the semantic response.

### 5.3 Speech Boundary

Superclaw should pass only speech-related directives to Whitebox.

Whitebox should receive things like:

- text to speak
- tone/style hints
- interaction intent hints
- interruption policy

Whitebox should not receive:

- raw planner trees
- arbitrary orchestration internals
- low-level reasoning traces as speech-layer inputs

## 6. External Signal Model

Whitebox should assume it receives normalized signals from other layers.

Those signals can originate from:

- `superclaw`
- `graphstore`
- `regate`
- `forge`
- `temporal`
- `pulse`
- `cloak`
- `keypooler`

But Whitebox should consume them through a small normalized model instead of
binding directly to every subsystem’s internal data structures.

### 6.1 Superclaw -> Whitebox

Superclaw should provide:

- speech payload
- speaking tone hint
- interaction intent
- optional expression hint

### 6.2 Other Systems -> Whitebox

Other systems should provide compact fragments for Info Mode.

Examples:

- `task delegated`
- `process reaped`
- `artifact staged`
- `graph linked`
- `quota low`
- `warning`

These fragments must stay small and streamable.

## 7. Runtime Architecture

Whitebox v1 should use an event-driven runtime with a fixed render tick.

### 7.1 Runtime Stages

1. Event intake
2. State update
3. Scene composition
4. Ratatui rendering
5. Side-effect execution
6. Event emission upward

### 7.2 Event Sources

The runtime should consume:

- terminal input events
- STT events
- TTS lifecycle events
- superclaw directives
- info-feed updates
- timer and animation ticks

### 7.3 Event Sinks

The runtime should emit:

- utterance-ready
- listening-started
- listening-stopped
- speaking-started
- speaking-finished
- interrupted

This keeps Whitebox integrated without giving it too much system authority.

## 8. Internal State Model

Whitebox should have a canonical internal state object that is independent of
the renderer.

Recommended state areas:

- app state
- mode state
- expression state
- speech/listen state
- turn-taking state
- info-feed state
- alert state
- scene state

This should be updated through a reducer-like mechanism so runtime behavior is
predictable and testable.

## 9. Scene Model

Even though Ratatui is the only renderer in v1, Whitebox should not render
directly from raw application state.

Instead it should compose a small scene model:

- avatar layer
- text layer
- overlay layer
- alert layer
- feed layer

The scene should live in configurable virtual space with `1024x1024` as the
default logical design size.

Ratatui then becomes the adapter that maps that scene into terminal cells.

## 10. Ratatui Constraints

Ratatui is terminal-cell-native, not a raw pixel framebuffer.

That means:

- Whitebox must not depend on literal pixel output for correctness
- the virtual `1024x1024` space is conceptual
- rendering needs projection from virtual coordinates to terminal layout
- dense visual styles must degrade gracefully to terminal cell density

Ratatui should be treated as the renderer adapter, not the owner of Whitebox
semantics.

## 11. Avatar System

The avatar system should start from the existing cat asset set.

Current organized asset groups:

- `face/`
- `eyes/`
- `mouth/`
- `ears/`
- `icons/`
- `preview/`

### 11.1 Avatar Composition

Whitebox should compose an avatar from:

- one face fill
- one eye set
- one mouth set
- one ear set

The composition rules should stay deterministic and data-driven.

### 11.2 Expression Mapping

Expression should be driven by mappings, not hardcoded one-off widget logic.

Examples:

- happy -> `eyes_happy_closed` + `mouth_soft_smile`
- sleepy -> `eyes_sleepy_flat` + `mouth_flat_neutral`
- worried -> `eyes_worried_angled` + `mouth_small_frown`
- playful -> `eyes_open_rose` + `mouth_cat_smile`

This mapping system is the basis for future emotional expansion.

## 12. Voice Integration

TTS and STT are part of Whitebox behavior, even if the low-level audio
implementation sits in separate modules or adapters.

### 12.1 Whitebox Owns

- visible listening state
- visible speaking state
- debounce timing
- interruption handling
- transcript readiness boundary

### 12.2 Whitebox Does Not Own

- final language generation
- conversational reasoning
- long-form content planning

## 13. UI/UX Intent

Whitebox should feel:

- alive
- readable
- emotionally coherent
- non-overwhelming
- fast to understand

### 13.1 Main UX Rules

- every listening state must be visible
- every speaking state must be visible
- every transition must feel intentional
- expression must match the current interaction state
- info mode must be compact and continuous, not noisy
- diagnostics should not dominate the emotional surface

### 13.2 Interaction Philosophy

The target behavior is closer to:

- speaking to a pet
- a companion reacting back
- a living terminal face

And less like:

- a sysadmin dashboard
- a generic chatbot window

## 14. Core Modules

Recommended v1 module layout:

```text
src/
├── main.rs
├── app.rs
├── config.rs
├── events.rs
├── state.rs
├── modes.rs
├── profiles.rs
├── reducer.rs
├── scene.rs
├── timing.rs
├── avatar/
│   ├── mod.rs
│   ├── assets.rs
│   ├── compose.rs
│   └── expressions.rs
├── voice/
│   ├── mod.rs
│   ├── stt.rs
│   ├── tts.rs
│   └── turn_taking.rs
├── integrations/
│   ├── mod.rs
│   ├── superclaw.rs
│   └── info_feed.rs
├── render/
│   ├── mod.rs
│   ├── ratatui.rs
│   ├── layout.rs
│   └── canvas_map.rs
└── widgets/
    ├── mod.rs
    ├── avatar.rs
    ├── info_feed.rs
    ├── subtitles.rs
    └── alerts.rs
```

## 15. Testing Strategy

Whitebox should be tested as a runtime and behavior system, not just as a
visual output binary.

### 15.1 Unit Tests

- reducer transitions
- mode switching
- expression mapping
- debounce logic
- turn-taking rules

### 15.2 Integration Tests

- STT event -> utterance-ready event
- superclaw speech payload -> speaking lifecycle
- info fragment stream -> info mode rendering state
- interruption handling during speaking

### 15.3 Snapshot / Render Tests

- avatar composition snapshots
- mode layout snapshots
- terminal rendering snapshots where practical

## 16. Development Order

### Phase 1

- canonical state and events
- reducer and mode framework
- timing and event loop skeleton

### Phase 2

- asset loader
- avatar composition
- expression profile mapping

### Phase 3

- Ratatui renderer
- base layout
- expression mode visuals

### Phase 4

- info mode feed
- interaction mode visuals
- TTS/STT state handling

### Phase 5

- superclaw adapter
- info-feed adapter
- test hardening

## 17. Success Criteria

Whitebox v1 is successful if:

- the agent visibly feels alive in terminal
- the cat avatar composes correctly from parts
- expression, info, and interaction modes are all distinct and legible
- voice/listening state is obvious
- Whitebox remains dumb and non-planning
- the runtime can evolve later without rewriting mode semantics

## 18. Non-Goals

Whitebox v1 should not try to be:

- a general dashboard platform
- a web app
- a full graphics engine
- a chatbot brain
- a memory system
- a planner

The goal is a great embodied terminal surface, not a general-purpose everything-layer.
