use super::Frame;

// 1 frame — static, flat drooped expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
];

// 12 ticks — sluggish, barely opens mouth (only 3 ticks open out of 12)
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
];

// 12 ticks — droopy even while listening, ears stay rounded
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
];
