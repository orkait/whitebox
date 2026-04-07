use super::Frame;

// 1 frame — static, flat drooped expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
];

// 12 ticks — barely speaks. Flat neutral dominates, mouth only opens at the end
// of a phrase and barely at that. Sluggish.
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  2 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  3 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  4 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  5 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  6 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  7 rest
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  8 rest (long pause)
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, //  9 barely opens
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, // 10 hold
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, // 11 hold
];

// 12 ticks — droopy even while listening, barely registers
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_sleepy_flat", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 11
];
