use super::Frame;

// 1 frame — static, locked teary expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" },
];

// 12 ticks — sad speech: small_frown anchors, opens slowly, holds briefly, closes back.
// More rest before opening than after — heavy and reluctant.
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  2 rest
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  3 rest (heavy pause)
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_open_flat",   ears: "ears_style_rounded" }, //  4 open
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_open_flat",   ears: "ears_style_rounded" }, //  5 hold
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_open_flat",   ears: "ears_style_rounded" }, //  6 hold
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  7 close back to frown
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  8 rest
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  9 rest
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, // 10 rest
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, // 11 rest
];

// 12 ticks — teary and still, ears stay soft/rounded
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_teary", mouth: "mouth_small_frown", ears: "ears_style_rounded" }, // 11
];
