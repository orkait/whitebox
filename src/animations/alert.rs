use super::Frame;

// 32 ticks — hyper-alert: very rare blink at tick 24 (extremely brief, just 4 frames total).
// half_open_rose as intermediate for the blink.
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 11
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 12
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 13
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 14
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 15
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 16
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 17
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 18
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 19
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 20
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 21
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 22
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 23
    // rare blink — single fast close (1 frame down, 1 closed, 2 opening = 4 total)
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 24 close
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 25 closed
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 26 opening
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 27 opening
    // back immediately
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 28
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 29
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 30
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 31
];

// 12 ticks — alert speech is clipped and sharp (short vowel holds, quick transitions)
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  2 open (vowel)
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  3 hold
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  4 close fast
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  5 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  6 next phrase
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  7 hold
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  8 hold
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  9 close
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 10 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 11 rest
];

// 12 ticks — fully alert, ears sharp, eyes unblinking
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 11
];
