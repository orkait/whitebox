use super::Frame;

// 24 ticks — locked, intense. Single tense blink at tick 20.
// No intermediate (angry blink is sharp and instant — no soft half-open)
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 11
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 12
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 13
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 14
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 15
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 16
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 17
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 18
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 19
    // tense blink — instant close and reopen (no soft intermediate for angry)
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",   mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 20
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 21
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 22
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 23
];

// 12 ticks — aggressive mouth rhythm, locked eyes
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  2 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" }, //  3 open (aggressive)
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" }, //  4 hold
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" }, //  5 hold
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" }, //  6 close back to pout
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" }, //  7 rest
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" }, //  8 open again
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_open_flat",  ears: "ears_style_sharp" }, //  9 hold
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" }, // 10 close
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop",  ears: "ears_style_sharp" }, // 11 rest
];

// 12 ticks — locked angry attention
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  6
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_serious_angry", mouth: "mouth_pout_loop", ears: "ears_style_sharp" }, // 11
];
