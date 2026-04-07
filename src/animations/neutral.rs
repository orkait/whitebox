use super::Frame;

// 24 ticks — open eyes, single soft blink at tick 8, long open stretch after
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // blink
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",  mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
];

// 12 ticks — mouth opens on ticks 3-4 and 8-9, eyes stay open
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" },
];

// 12 ticks — ears perk sharp, eyes slightly more open/alert
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" },
];
