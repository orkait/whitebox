use super::Frame;

// 1 frame — static, locked stern expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
];

// 12 ticks — deliberate, measured speech. Chevron rests dominate,
// tiny_triangle as the tight open position. Long rests between phrases.
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  2 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  3 rest (longer pause before speaking)
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" }, //  4 open (deliberate)
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" }, //  5 hold
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  6 close measured
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  7 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" }, //  8 open again
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" }, //  9 hold
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, // 10 close
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, // 11 rest
];

// 12 ticks — locked, unblinking stern attention
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" }, // 11
];
