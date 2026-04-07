#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationFrame {
    pub eyes: String,
    pub mouth: String,
    pub ticks: u32,
}

impl AnimationFrame {
    pub fn new(eyes: impl Into<String>, mouth: impl Into<String>, ticks: u32) -> Self {
        Self {
            eyes: eyes.into(),
            mouth: mouth.into(),
            ticks: ticks.max(1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationClip {
    pub id: String,
    pub frames: Vec<AnimationFrame>,
    pub looping: bool,
}

#[derive(Debug, Clone)]
pub struct AnimationPlayer {
    clip: AnimationClip,
    frame_index: usize,
    frame_tick: u32,
}

impl AnimationPlayer {
    pub fn new(clip: AnimationClip) -> Self {
        assert!(
            !clip.frames.is_empty(),
            "animation clip must contain at least one frame"
        );
        Self {
            clip,
            frame_index: 0,
            frame_tick: 0,
        }
    }

    pub fn current(&self) -> &AnimationFrame {
        &self.clip.frames[self.frame_index]
    }

    pub fn tick(&mut self) {
        self.frame_tick += 1;
        let current_ticks = self.current().ticks;
        if self.frame_tick < current_ticks {
            return;
        }

        self.frame_tick = 0;
        if self.frame_index + 1 < self.clip.frames.len() {
            self.frame_index += 1;
        } else if self.clip.looping {
            self.frame_index = 0;
        }
    }
}
