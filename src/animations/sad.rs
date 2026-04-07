use super::Frame;

// 1 frame — static, locked teary expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
];

// 12 ticks — sad mouth movement, teary eyes throughout
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_open_flat",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_open_flat",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_open_flat",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
];

// 12 ticks — teary and still, ears stay rounded (sad stays soft)
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
];
