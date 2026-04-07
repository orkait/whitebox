use super::Frame;

// 1 frame — static, locked stern expression
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
];

// 12 ticks — tight controlled mouth, serious eyes throughout
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
];

// 12 ticks — static stern attention
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious", ears: "ears_style_sharp" },
];
