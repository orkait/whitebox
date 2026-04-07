use super::Frame;

// 32 ticks — base is half_open_blush (concentrated look). Blink goes:
// half_open → soft_closed (1 frame close, already halfway) → half_open (2 frame slow open).
// Blink at tick 16. No ear secondary (already sharp = no change to make).
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 11
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 12
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 13
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 14
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 15
    // blink — close (1 frame fast, already half-open so just 1 to close)
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",     mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 16
    // hold (1 frame)
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",     mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 17
    // open slow — back through half_open (2 frames, deliberate)
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 18
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 19
    // settled back into concentrated half-open
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 20
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 21
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 22
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 23
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 24
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 25
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 26
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 27
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 28
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 29
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 30
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 31
];

// 12 ticks — controlled, minimal mouth movement, half-open eyes throughout
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  2 rest
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" }, //  3 open
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" }, //  4 hold
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  5 close
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  6 rest
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  7 rest
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" }, //  8 open
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_sharp" }, //  9 hold
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 10 close
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 11 rest
];

// 12 ticks — intent listening, half-open stays (focused even while listening)
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 11
];
