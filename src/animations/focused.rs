use super::Frame;

// 24 ticks — half-open eyes (concentrated look), single blink at tick 16
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",     mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // blink
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
];

// 12 ticks — controlled, minimal mouth movement
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
];

// 12 ticks — intent, half-open attentive eyes
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
];
