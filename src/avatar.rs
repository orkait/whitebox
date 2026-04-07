use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use image::{imageops, DynamicImage, GenericImageView, ImageReader, RgbaImage};

use crate::animations::{self, Frame};
use crate::api::Stance;
use crate::state::{SpeakState, ListenState, WhiteboxState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvatarView {
    pub face: &'static str,
    pub eyes: &'static str,
    pub mouth: &'static str,
    pub ears: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetPaths {
    pub face: PathBuf,
    pub eyes: PathBuf,
    pub mouth: PathBuf,
    pub ears: PathBuf,
}

impl AssetPaths {
    pub fn from_view(root: &Path, view: &AvatarView) -> Self {
        Self {
            face: root.join("face").join(format!("{}.png", view.face)),
            eyes: root.join("eyes").join(format!("{}.png", view.eyes)),
            mouth: root.join("mouth").join(format!("{}.png", view.mouth)),
            ears: root.join("ears").join(format!("{}.png", view.ears)),
        }
    }
}

pub fn compose_avatar_image(root: &Path, view: &AvatarView) -> image::ImageResult<DynamicImage> {
    let paths = AssetPaths::from_view(root, view);
    let face  = ImageReader::open(paths.face)?.decode()?.to_rgba8();
    let eyes  = ImageReader::open(paths.eyes)?.decode()?.to_rgba8();
    let mouth = ImageReader::open(paths.mouth)?.decode()?.to_rgba8();
    let ears  = ImageReader::open(paths.ears)?.decode()?.to_rgba8();

    let width  = face.width();
    let height = face.height();
    let mut canvas: RgbaImage = RgbaImage::new(width, height);
    imageops::overlay(&mut canvas, &face,  0, 0);
    imageops::overlay(&mut canvas, &ears,  0, 0);
    imageops::overlay(&mut canvas, &eyes,  0, 0);
    imageops::overlay(&mut canvas, &mouth, 0, 0);

    let image = DynamicImage::ImageRgba8(canvas);
    Ok(prepare_terminal_avatar(image))
}

pub fn resolve_avatar(state: &WhiteboxState) -> AvatarView {
    let frames = select_frames(state);
    let frame  = &frames[state.tick_count as usize % frames.len()];
    AvatarView {
        face:  frame.face,
        eyes:  frame.eyes,
        mouth: frame.mouth,
        ears:  frame.ears,
    }
}

fn select_frames(state: &WhiteboxState) -> &'static [Frame] {
    let speaking  = state.speak_state == SpeakState::Speaking;
    let listening = state.listen_state != ListenState::Idle;

    match (state.stance, speaking, listening) {
        (Stance::Neutral, true,  _    ) => animations::neutral::SPEAKING,
        (Stance::Neutral, false, true ) => animations::neutral::LISTENING,
        (Stance::Neutral, false, false) => animations::neutral::IDLE,

        (Stance::Warm,    true,  _    ) => animations::warm::SPEAKING,
        (Stance::Warm,    false, true ) => animations::warm::LISTENING,
        (Stance::Warm,    false, false) => animations::warm::IDLE,

        (Stance::Playful, true,  _    ) => animations::playful::SPEAKING,
        (Stance::Playful, false, true ) => animations::playful::LISTENING,
        (Stance::Playful, false, false) => animations::playful::IDLE,

        (Stance::Curious, true,  _    ) => animations::curious::SPEAKING,
        (Stance::Curious, false, true ) => animations::curious::LISTENING,
        (Stance::Curious, false, false) => animations::curious::IDLE,

        (Stance::Alert,   true,  _    ) => animations::alert::SPEAKING,
        (Stance::Alert,   false, true ) => animations::alert::LISTENING,
        (Stance::Alert,   false, false) => animations::alert::IDLE,

        (Stance::Focused, true,  _    ) => animations::focused::SPEAKING,
        (Stance::Focused, false, true ) => animations::focused::LISTENING,
        (Stance::Focused, false, false) => animations::focused::IDLE,

        (Stance::Guarded, true,  _    ) => animations::guarded::SPEAKING,
        (Stance::Guarded, false, true ) => animations::guarded::LISTENING,
        (Stance::Guarded, false, false) => animations::guarded::IDLE,

        (Stance::Stern,   true,  _    ) => animations::stern::SPEAKING,
        (Stance::Stern,   false, true ) => animations::stern::LISTENING,
        (Stance::Stern,   false, false) => animations::stern::IDLE,

        (Stance::Tired,   true,  _    ) => animations::tired::SPEAKING,
        (Stance::Tired,   false, true ) => animations::tired::LISTENING,
        (Stance::Tired,   false, false) => animations::tired::IDLE,

        (Stance::Sad,     true,  _    ) => animations::sad::SPEAKING,
        (Stance::Sad,     false, true ) => animations::sad::LISTENING,
        (Stance::Sad,     false, false) => animations::sad::IDLE,

        (Stance::Angry,   true,  _    ) => animations::angry::SPEAKING,
        (Stance::Angry,   false, true ) => animations::angry::LISTENING,
        (Stance::Angry,   false, false) => animations::angry::IDLE,
    }
}

fn prepare_terminal_avatar(image: DynamicImage) -> DynamicImage {
    let (full_width, full_height) = image.dimensions();
    let cropped = non_transparent_bounds(&image)
        .map(|(left, top, width, height)| image.crop_imm(left, top, width, height))
        .unwrap_or(image);
    cropped
        .resize_exact(full_width, full_height, FilterType::CatmullRom)
        .adjust_contrast(28.0)
        .unsharpen(1.4, 1)
}

fn non_transparent_bounds(image: &DynamicImage) -> Option<(u32, u32, u32, u32)> {
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for (x, y, pixel) in rgba.enumerate_pixels() {
        if pixel[3] == 0 {
            continue;
        }
        found = true;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    if !found {
        return None;
    }

    let margin = 24;
    let left   = min_x.saturating_sub(margin);
    let top    = min_y.saturating_sub(margin);
    let right  = (max_x + margin).min(width.saturating_sub(1));
    let bottom = (max_y + margin).min(height.saturating_sub(1));

    Some((left, top, right - left + 1, bottom - top + 1))
}
