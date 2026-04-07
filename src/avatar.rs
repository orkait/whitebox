use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use image::{imageops, DynamicImage, GenericImageView, ImageReader, RgbaImage};

use crate::api::Stance;
use crate::clips::{clip_for_state, ClipKind};
use crate::profiles::{ExpressionPreset, PresetLibrary};
use crate::state::{ListenState, WhiteboxState};

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
    let face = ImageReader::open(paths.face)?.decode()?.to_rgba8();
    let eyes = ImageReader::open(paths.eyes)?.decode()?.to_rgba8();
    let mouth = ImageReader::open(paths.mouth)?.decode()?.to_rgba8();
    let ears = ImageReader::open(paths.ears)?.decode()?.to_rgba8();

    let width = face.width();
    let height = face.height();
    let mut canvas: RgbaImage = RgbaImage::new(width, height);
    imageops::overlay(&mut canvas, &face, 0, 0);
    imageops::overlay(&mut canvas, &ears, 0, 0);
    imageops::overlay(&mut canvas, &eyes, 0, 0);
    imageops::overlay(&mut canvas, &mouth, 0, 0);

    let image = DynamicImage::ImageRgba8(canvas);
    Ok(prepare_terminal_avatar(image))
}

pub fn resolve_avatar(state: &WhiteboxState) -> AvatarView {
    let preset = preset_for_stance(state.stance, PresetLibrary.get(state.profile));
    let clip = clip_for_state(state);
    let animation_phase = state.tick_count % 12;

    let (eyes, mouth, ears) = match clip.kind {
        ClipKind::IdleSoft | ClipKind::IdleSleepy => {
            idle_animation_for_stance(state.stance, animation_phase, &preset)
        }
        ClipKind::ListenBounce => listen_animation_for_stance(
            state.stance,
            state.listen_state,
            animation_phase,
            &preset,
        ),
        ClipKind::TalkSoft | ClipKind::TalkPet | ClipKind::TalkSerious => {
            speaking_animation_for_stance(state.stance, animation_phase, &preset)
        }
    };

    AvatarView {
        face: preset.face,
        eyes,
        mouth,
        ears,
    }
}

fn idle_animation_for_stance(
    stance: Stance,
    phase: u64,
    preset: &ExpressionPreset,
) -> (&'static str, &'static str, &'static str) {
    match stance {
        Stance::Neutral => (
            if phase == 8 { "eyes_soft_closed" } else { "eyes_open_blush" },
            "mouth_flat_neutral",
            preset.ears,
        ),
        Stance::Warm => (
            if phase == 5 {
                "eyes_soft_closed"
            } else if phase >= 8 {
                "eyes_half_open_rose"
            } else {
                "eyes_open_rose"
            },
            "mouth_soft_smile",
            "ears_style_rounded",
        ),
        Stance::Playful => (
            if phase < 3 {
                "eyes_excited_squint"
            } else if phase < 6 {
                "eyes_happy_closed"
            } else {
                "eyes_happy_closed"
            },
            if phase < 3 {
                "mouth_wavy_cat"
            } else if phase < 6 {
                "mouth_cat_smile"
            } else if phase < 9 {
                "mouth_open_tongue"
            } else {
                "mouth_wavy_cat"
            },
            "ears_style_rounded",
        ),
        Stance::Curious => (
            if phase == 5 || phase == 11 {
                "eyes_half_open_rose"
            } else {
                "eyes_open_rose"
            },
            "mouth_tiny_triangle",
            "ears_style_sharp",
        ),
        Stance::Alert => (
            if phase == 5 { "eyes_half_open_blush" } else { "eyes_open_rose" },
            "mouth_tiny_triangle",
            "ears_style_sharp",
        ),
        Stance::Focused => (
            if phase == 8 { "eyes_soft_closed" } else { "eyes_half_open_blush" },
            "mouth_flat_neutral",
            "ears_style_sharp",
        ),
        Stance::Guarded => (
            "eyes_worried_angled",
            "mouth_small_frown",
            "ears_style_sharp",
        ),
        Stance::Stern => (
            "eyes_serious_angry",
            "mouth_chevron_serious",
            "ears_style_sharp",
        ),
        Stance::Tired => (
            if phase == 6 || phase == 7 { "eyes_soft_closed" } else { "eyes_sleepy_flat" },
            "mouth_flat_neutral",
            "ears_style_rounded",
        ),
        Stance::Sad => (
            "eyes_teary",
            "mouth_small_frown",
            "ears_style_rounded",
        ),
        Stance::Angry => (
            "eyes_serious_angry",
            "mouth_pout_loop",
            "ears_style_sharp",
        ),
    }
}

fn listen_animation_for_stance(
    stance: Stance,
    listen_state: ListenState,
    phase: u64,
    preset: &ExpressionPreset,
) -> (&'static str, &'static str, &'static str) {
    let (base_eyes, base_mouth, base_ears) = idle_animation_for_stance(stance, phase, preset);
    let eyes = match listen_state {
        ListenState::Idle => base_eyes,
        ListenState::Debouncing => match stance {
            Stance::Warm | Stance::Curious | Stance::Alert => "eyes_half_open_rose",
            Stance::Playful => {
                if phase % 2 == 0 { "eyes_excited_squint" } else { "eyes_happy_closed" }
            }
            Stance::Focused | Stance::Neutral | Stance::Tired => "eyes_half_open_blush",
            Stance::Guarded => "eyes_worried_angled",
            Stance::Stern | Stance::Angry => "eyes_serious_angry",
            Stance::Sad => "eyes_teary",
        },
        ListenState::Hearing | ListenState::Ready => match stance {
            Stance::Neutral => {
                if phase % 6 == 0 { "eyes_half_open_blush" } else { "eyes_open_blush" }
            }
            Stance::Warm | Stance::Curious | Stance::Alert => {
                if phase % 4 == 0 { "eyes_half_open_rose" } else { "eyes_open_rose" }
            }
            Stance::Playful => {
                if phase % 4 < 2 { "eyes_excited_squint" } else { "eyes_happy_closed" }
            }
            Stance::Focused => "eyes_half_open_blush",
            Stance::Guarded => "eyes_worried_angled",
            Stance::Stern | Stance::Angry => "eyes_serious_angry",
            Stance::Tired => "eyes_sleepy_flat",
            Stance::Sad => "eyes_teary",
        },
    };
    let ears = match listen_state {
        ListenState::Idle => base_ears,
        ListenState::Hearing | ListenState::Ready | ListenState::Debouncing => match stance {
            Stance::Warm | Stance::Playful | Stance::Tired | Stance::Sad => base_ears,
            _ => "ears_style_sharp",
        },
    };
    (eyes, base_mouth, ears)
}

fn speaking_animation_for_stance(
    stance: Stance,
    phase: u64,
    preset: &ExpressionPreset,
) -> (&'static str, &'static str, &'static str) {
    let (base_eyes, _, base_ears) = idle_animation_for_stance(stance, phase, preset);
    let eyes = match stance {
        Stance::Playful => {
            if phase < 6 { "eyes_excited_squint" } else { "eyes_happy_closed" }
        }
        Stance::Warm => {
            if phase == 5 { "eyes_happy_closed" } else { "eyes_half_open_rose" }
        }
        _ => base_eyes,
    };
    let mouth = match stance {
        Stance::Warm => {
            if phase < 3 || (6..9).contains(&phase) { "mouth_soft_smile" } else { "mouth_open_flat" }
        }
        Stance::Playful => {
            if phase < 3 {
                "mouth_wavy_cat"
            } else if phase < 6 {
                "mouth_open_tongue"
            } else if phase < 9 {
                "mouth_cat_smile"
            } else {
                "mouth_open_flat"
            }
        }
        Stance::Curious | Stance::Alert => {
            if phase < 6 { "mouth_tiny_triangle" } else { "mouth_open_flat" }
        }
        Stance::Focused | Stance::Neutral => {
            if phase < 6 { "mouth_flat_neutral" } else { "mouth_open_flat" }
        }
        Stance::Tired => {
            if phase < 9 { "mouth_flat_neutral" } else { "mouth_open_flat" }
        }
        Stance::Guarded => {
            if phase < 6 || phase >= 9 { "mouth_small_frown" } else { "mouth_tiny_triangle" }
        }
        Stance::Sad => {
            if phase < 6 || phase >= 9 { "mouth_small_frown" } else { "mouth_open_flat" }
        }
        Stance::Stern => {
            if phase < 5 || (7..11).contains(&phase) { "mouth_chevron_serious" } else { "mouth_tiny_triangle" }
        }
        Stance::Angry => {
            if phase < 4 || (7..10).contains(&phase) { "mouth_pout_loop" } else { "mouth_open_flat" }
        }
    };
    (eyes, mouth, base_ears)
}

fn preset_for_stance(stance: Stance, fallback: ExpressionPreset) -> ExpressionPreset {
    match stance {
        Stance::Neutral => fallback,
        Stance::Warm => ExpressionPreset {
            face: "face_fill_rose",
            eyes: "eyes_half_open_rose",
            mouth: "mouth_soft_smile",
            ears: "ears_style_rounded",
            ..fallback
        },
        Stance::Playful => ExpressionPreset {
            face: "face_fill_rose",
            eyes: "eyes_excited_squint",
            mouth: "mouth_wavy_cat",
            ears: "ears_style_rounded",
            ..fallback
        },
        Stance::Curious => ExpressionPreset {
            face: "face_fill_rose",
            eyes: "eyes_open_rose",
            mouth: "mouth_tiny_triangle",
            ears: "ears_style_sharp",
            ..fallback
        },
        Stance::Alert => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_open_rose",
            mouth: "mouth_tiny_triangle",
            ears: "ears_style_sharp",
            ..fallback
        },
        Stance::Focused => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_half_open_blush",
            mouth: "mouth_flat_neutral",
            ears: "ears_style_sharp",
            ..fallback
        },
        Stance::Guarded => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_worried_angled",
            mouth: "mouth_small_frown",
            ears: "ears_style_sharp",
            ..fallback
        },
        Stance::Stern => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_serious_angry",
            mouth: "mouth_chevron_serious",
            ears: "ears_style_sharp",
            ..fallback
        },
        Stance::Tired => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_sleepy_flat",
            mouth: "mouth_flat_neutral",
            ears: "ears_style_rounded",
            ..fallback
        },
        Stance::Sad => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_teary",
            mouth: "mouth_small_frown",
            ears: "ears_style_rounded",
            ..fallback
        },
        Stance::Angry => ExpressionPreset {
            face: "face_fill_blush",
            eyes: "eyes_serious_angry",
            mouth: "mouth_pout_loop",
            ears: "ears_style_sharp",
            ..fallback
        },
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
    let left = min_x.saturating_sub(margin);
    let top = min_y.saturating_sub(margin);
    let right = (max_x + margin).min(width.saturating_sub(1));
    let bottom = (max_y + margin).min(height.saturating_sub(1));

    Some((left, top, right - left + 1, bottom - top + 1))
}
