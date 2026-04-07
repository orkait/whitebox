# Whitebox API v1

Whitebox is a body runtime.

It does not own reasoning, planning, memory, or conversation logic.
It owns:

- listening
- speaking
- visible stance
- visible status
- body lifecycle events

The API is intentionally small and semantic so it can be used by Superclaw or any other agent controller.

## Principles

- Whitebox exposes body verbs, not internal implementation details.
- Activity is derived from body actions like `listen()` and `speak(text)`.
- Stance is emotional posture and composes with activity.
- Whitebox supports one active listen session and one active speak session at a time.
- Lifecycle events are explicit and correlated with session IDs.

## Commands

### `listen()`

Start a new listening session.

Returns:

```text
listen_session_id
```

Behavior:

- If speech is active, Whitebox interrupts speech first, then starts listening.
- If listening is already active, Whitebox must return an explicit error or explicit no-op result.

### `stop_listening()`

Stop the active listening session.

Behavior:

- If no listening session is active, Whitebox returns `no_active_session`.

### `speak(text)`

Start a new speaking session.

Arguments:

```text
text: string
```

Returns:

```text
speak_session_id
```

Behavior:

- If listening is active, Whitebox interrupts listening first, then starts speaking.
- If speech is already active, Whitebox interrupts the previous speech, then starts the new one.

### `stop_speaking()`

Stop the active speaking session.

Behavior:

- If no speaking session is active, Whitebox returns `no_active_session`.

### `set_stance(name)`

Set the visible emotional stance of the body.

Arguments:

```text
name: stance
```

This changes posture and expression baseline only.
It does not directly start or stop activity.

### `set_status(text)`

Set a short visible status string.

Arguments:

```text
text: string
```

### `clear_status()`

Clear the visible status string.

## Events

### Listen events

```text
on_listen_started(id)
on_listen_partial(id, text)
on_listen_final(id, text)
on_listen_stopped(id, reason)
```

Ordering:

```text
on_listen_started
... optional on_listen_partial ...
... optional on_listen_final ...
on_listen_stopped
```

If a final transcript exists, `on_listen_final` must occur before `on_listen_stopped(completed)`.

### Speak events

```text
on_speak_started(id, text)
on_speak_stopped(id, reason)
```

Ordering:

```text
on_speak_started
on_speak_stopped
```

## Stop Reasons

Whitebox uses these stable stop reasons:

```text
completed
cancelled
interrupted
error
```

Applies to:

- `on_listen_stopped(id, reason)`
- `on_speak_stopped(id, reason)`

## Error Events

Whitebox emits:

```text
on_error(code, message, cause?)
```

Stable error codes:

```text
busy
backend_unavailable
invalid_stance
no_active_session
internal_error
```

## Stance Vocabulary

Whitebox v1 supports this fixed stance set:

```text
neutral
warm
playful
curious
alert
focused
guarded
stern
tired
sad
angry
```

Unknown stance values must return `invalid_stance`.

## Activity Model

Whitebox has internal activity states such as:

```text
idle
listening
thinking
speaking
```

These are internal runtime states.

Whitebox v1 does not expose a public `set_activity()` command.

Activity is derived from:

- `listen()`
- `stop_listening()`
- `speak(text)`
- `stop_speaking()`
- internal idle transitions

This keeps the public API small and prevents split-brain control.

## Composition Rules

Final visible expression is:

```text
stance + activity
```

Rules:

- stance defines the emotional baseline
- activity defines the active body behavior
- activity wins for motion-critical parts

Examples:

- speaking controls mouth motion
- listening controls attentive eyes and ears
- stance still influences the overall face while those actions are active

## Single-Body Policy

Whitebox behaves like one body, not a task queue.

Rules:

- one active listening session at a time
- one active speaking session at a time
- `listen()` interrupts active speech
- `speak(text)` interrupts active listening
- a new `speak(text)` interrupts previous speech

## Example

```python
listen_id = box.listen()

box.set_stance("curious")

speak_id = box.speak("Hello.")

box.set_status("Scanning the room")

box.stop_speaking()
box.clear_status()
```

## Non-Goals

Whitebox v1 does not expose:

- raw avatar asset names
- raw mood/profile enums
- valence/arousal/dominance axes
- direct activity forcing
- planning or memory APIs

Those remain internal implementation details.
