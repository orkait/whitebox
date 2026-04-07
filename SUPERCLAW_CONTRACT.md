# Superclaw Contract

How superclaw drives the pet. This is the only interface superclaw needs.

## Setup

```rust
use whitebox::superclaw::{Pet, Mood, PetEvent};
use whitebox::api::WhiteboxBody;

let body = WhiteboxBody::new();
let mut pet = Pet::new(body);
```

## Driving the pet

```rust
// Make the pet say something
pet.say("Hello!", None);

// Say something with a mood
pet.say("I found something interesting.", Some(Mood::Curious));

// Change mood silently
pet.feel(Mood::Tired);

// Show status text on screen
pet.show("Thinking...");
pet.hide();

// Listen for voice input
pet.listen();
pet.stop_listening();
```

## Tick loop

Call `tick()` every 83ms. Call `drain()` after to receive events.

```rust
loop {
    sleep(83ms);
    pet.tick();

    for event in pet.drain() {
        match event {
            PetEvent::Heard(text)        => { /* user said something */ }
            PetEvent::SpokeDone          => { /* pet finished speaking */ }
            PetEvent::SpokeStarted(text) => { /* pet started speaking */ }
        }
    }
}
```

## Checking pet state

```rust
let state = pet.state();
state.mood       // current mood
state.speaking   // true if currently speaking
state.listening  // true if currently listening
state.status     // on-screen status text
```

## Moods

```
Neutral  Warm  Playful  Curious  Alert
Focused  Guarded  Stern  Tired  Sad  Angry
```

## What superclaw does NOT need to know

- Session IDs
- Stop reasons (completed / cancelled / interrupted)
- Internal error codes
- Avatar asset names
- Tick count
- Debounce timing
- Voice backend selection

Those are whitebox internals.
