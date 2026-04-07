use super::Frame;

// 24 ticks — open rose eyes, clean single blink at tick 8, long attentive stretch
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_soft_closed", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" }, // blink
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
];

// 12 ticks — mouth opens briefly, eyes stay alert
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_open_flat",     ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
];

// 12 ticks — ears already sharp, eyes open wide and attentive
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
];
