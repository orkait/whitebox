/// Superclaw contract — the only interface superclaw needs to drive the pet.
///
/// Wraps WhiteboxBody and hides session IDs, event queues, error codes,
/// and all other implementation details. Superclaw calls these methods
/// and polls drain() for responses.
use crate::api::{BodyEvent, Stance, WhiteboxBody};

// ── Types ────────────────────────────────────────────────────────────────────

/// The pet's emotional state. Maps 1:1 to Stance internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Neutral,
    Warm,
    Playful,
    Curious,
    Alert,
    Focused,
    Guarded,
    Stern,
    Tired,
    Sad,
    Angry,
}

impl Mood {
    pub fn as_str(self) -> &'static str {
        match self {
            Mood::Neutral  => "neutral",
            Mood::Warm     => "warm",
            Mood::Playful  => "playful",
            Mood::Curious  => "curious",
            Mood::Alert    => "alert",
            Mood::Focused  => "focused",
            Mood::Guarded  => "guarded",
            Mood::Stern    => "stern",
            Mood::Tired    => "tired",
            Mood::Sad      => "sad",
            Mood::Angry    => "angry",
        }
    }
}

impl From<Mood> for Stance {
    fn from(m: Mood) -> Self {
        match m {
            Mood::Neutral  => Stance::Neutral,
            Mood::Warm     => Stance::Warm,
            Mood::Playful  => Stance::Playful,
            Mood::Curious  => Stance::Curious,
            Mood::Alert    => Stance::Alert,
            Mood::Focused  => Stance::Focused,
            Mood::Guarded  => Stance::Guarded,
            Mood::Stern    => Stance::Stern,
            Mood::Tired    => Stance::Tired,
            Mood::Sad      => Stance::Sad,
            Mood::Angry    => Stance::Angry,
        }
    }
}

/// What superclaw receives back from the pet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PetEvent {
    /// Pet heard this text from the user.
    Heard(String),
    /// Pet finished speaking.
    SpokeDone,
    /// Pet started speaking this text.
    SpokeStarted(String),
}

/// A point-in-time snapshot of the pet's state.
#[derive(Debug, Clone)]
pub struct PetState {
    pub mood: Mood,
    pub speaking: bool,
    pub listening: bool,
    pub status: Option<String>,
}

// ── Pet ──────────────────────────────────────────────────────────────────────

/// The pet controller. Superclaw holds one of these and drives the tamagotchi.
pub struct Pet {
    body: WhiteboxBody,
}

impl Pet {
    pub fn new(body: WhiteboxBody) -> Self {
        Self { body }
    }

    /// Make the pet say something. Optionally change mood at the same time.
    /// Interrupts any current speech.
    pub fn say(&mut self, text: impl Into<String>, mood: Option<Mood>) {
        if let Some(m) = mood {
            let _ = self.body.set_stance(m.into());
        }
        let _ = self.body.speak(text.into());
    }

    /// Change the pet's mood without speaking.
    pub fn feel(&mut self, mood: Mood) {
        let _ = self.body.set_stance(mood.into());
    }

    /// Start listening for user voice input.
    pub fn listen(&mut self) {
        let _ = self.body.listen();
    }

    /// Stop listening.
    pub fn stop_listening(&mut self) {
        let _ = self.body.stop_listening();
    }

    /// Show a short text on screen (thinking, waiting, etc.).
    pub fn show(&mut self, text: impl Into<String>) {
        self.body.set_status(text);
    }

    /// Clear the on-screen text.
    pub fn hide(&mut self) {
        self.body.clear_status();
    }

    /// Advance the pet by one tick. Call this every 83ms.
    pub fn tick(&mut self) {
        self.body.tick();
    }

    /// Current pet state snapshot.
    pub fn state(&self) -> PetState {
        let s = self.body.snapshot();
        PetState {
            mood:      stance_to_mood(s.stance),
            speaking:  matches!(s.activity, crate::api::Activity::Speaking),
            listening: matches!(s.activity, crate::api::Activity::Listening),
            status:    s.status_text,
        }
    }

    /// Drain pending events. Call after tick() to receive responses.
    pub fn drain(&mut self) -> Vec<PetEvent> {
        self.body
            .drain_events()
            .into_iter()
            .filter_map(|e| match e {
                BodyEvent::ListenFinal { text, .. }  => Some(PetEvent::Heard(text)),
                BodyEvent::SpeakStopped { .. }        => Some(PetEvent::SpokeDone),
                BodyEvent::SpeakStarted { text, .. }  => Some(PetEvent::SpokeStarted(text)),
                _ => None,
            })
            .collect()
    }

    /// Access the underlying body if needed (advanced use only).
    pub fn body_mut(&mut self) -> &mut WhiteboxBody {
        &mut self.body
    }
}

fn stance_to_mood(stance: Stance) -> Mood {
    match stance {
        Stance::Neutral  => Mood::Neutral,
        Stance::Warm     => Mood::Warm,
        Stance::Playful  => Mood::Playful,
        Stance::Curious  => Mood::Curious,
        Stance::Alert    => Mood::Alert,
        Stance::Focused  => Mood::Focused,
        Stance::Guarded  => Mood::Guarded,
        Stance::Stern    => Mood::Stern,
        Stance::Tired    => Mood::Tired,
        Stance::Sad      => Mood::Sad,
        Stance::Angry    => Mood::Angry,
    }
}
