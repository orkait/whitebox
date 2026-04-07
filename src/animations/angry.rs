use super::Frame;

// 24 ticks — locked angry, single very rare blink at tick 20
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",   mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // rare tense blink
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
];

// 12 ticks — aggressive mouth, locked angry eyes
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" },
];

// 12 ticks — locked angry, ears sharp
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" },
];
