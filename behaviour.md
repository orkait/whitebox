# Whitebox Behaviour

Whitebox is the embodied surface runtime for the agent.

It is responsible for:

- expression
- interaction
- information display
- visible listening/speaking feedback

It is not responsible for:

- planning
- reasoning
- deciding content

## Modes

Whitebox has three top-level modes:

1. `Expression Mode`
2. `Info Mode`
3. `Interaction Mode`

## Expression Profiles

Expression Mode supports profiles such as:

- neutral
- pet-like
- serious
- curious
- sleepy

## Speech Boundary

Superclaw sends Whitebox:

- speech text
- speech tone
- interaction intent
- interruption policy

Whitebox owns:

- listening
- debounce
- speaking lifecycle
- visible conversational response

## Info Mode

Info Mode shows compact real-time fragments from:

- superclaw
- graphstore
- regate
- forge
- temporal
- pulse
- cloak
- keypooler

## Avatar

The v1 avatar is the cat asset system.

Whitebox composes:

- face
- eyes
- mouth
- ears

## Animations

Supported animation families:

- idle breathing
- blink
- speaking mouth loops
- listening ear lift
- alert ear flick
- acknowledgement nod
- mood shifts

## Rendering

Whitebox v1 is:

- Tauri-first
- backed by the Rust embodiment core
- driven by a simple command/event contract
- backed by a configurable virtual render space
- `1024x1024` by default
