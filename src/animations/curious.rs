use super::Frame;

// 32 ticks — asymmetric blink at tick 8 using half_open_rose intermediate,
// ears already sharp (no secondary ear action needed)
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  7
    // blink close fast (2 frames)
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  9
    // hold closed (1 frame)
    Frame { face: "face_fill_rose", eyes: "eyes_soft_closed",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 10
    // open slow (3 frames)
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 11
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 12
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 13
    // back to open — long attentive stretch
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 14
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 15
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 16
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 17
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 18
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 19
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 20
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 21
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 22
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 23
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 24
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 25
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 26
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 27
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 28
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 29
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 30
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 31
];

// 12 ticks — tiny triangle rest, open for vowels (uneven rhythm)
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  0 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  1 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  2 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  3 vowel open
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  4 hold
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  5 hold
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  6 close (transition)
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  7 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  8 next phrase
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" }, //  9 hold
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 10 close
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 11 rest
];

// 12 ticks — alert and attentive, ears stay sharp
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  6 processing dip
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // 11
];
