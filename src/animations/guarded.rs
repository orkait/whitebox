use super::Frame;

// 1 frame — static, frozen worried expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
];

// 12 ticks — tense, minimal mouth movement. tiny_triangle as open position
// (mouth barely opens — guarded speech is tight and careful)
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, //  2 rest
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  3 open (barely)
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  4 hold
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, //  5 close fast
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, //  6 rest
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, //  7 rest
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  8 open
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  9 hold
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, // 10 close
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" }, // 11 rest
];

// 12 ticks — tense, unblinking attention
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" }, // 11
];
