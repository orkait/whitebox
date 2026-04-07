use super::Frame;

// 24 ticks — wide open, very rare blink (tick 20 only), stays hyper-alert
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",  mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // rare blink
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose",    mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
];

// 12 ticks — sharp and quick mouth, alert eyes throughout
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
];

// 12 ticks — fully alert, ears sharp, eyes wide open
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
];
