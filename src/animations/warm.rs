use super::Frame;

// 24 ticks — open rose eyes, single soft blink at tick 8
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_soft_closed", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // blink
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
];

// 12 ticks — gentle mouth open, happy eye close at tick 5
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // happy close mid-speech
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
];

// 12 ticks — bright attentive, ears stay rounded (warm stays soft even listening)
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" },
];
