use super::Frame;

// 1 frame — static, frozen worried expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
];

// 12 ticks — tense mouth movement, worried eyes throughout
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_tiny_triangle", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown",   ears: "ears_style_sharp" },
];

// 12 ticks — tense, ears already sharp, worried throughout
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_worried_angled", mouth: "mouth_small_frown", ears: "ears_style_sharp" },
];
