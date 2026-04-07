use super::Frame;

// 32 ticks — natural blink at tick 7 (2 close, 1 hold, 3 open = asymmetric),
// ear secondary action at tick 19-20 (brief perk)
pub const IDLE: &[Frame] = &[
    // base — open, resting
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  6
    // blink — close fast (2 frames)
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  8
    // blink — hold closed (1 frame)
    Frame { face: "face_fill_blush", eyes: "eyes_soft_closed",   mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  9
    // blink — open slow (3 frames)
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 11
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 12
    // back to base
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 13
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 14
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 15
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 16
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 17
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 18
    // secondary action — ear twitch (perk up then relax)
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp"   }, // 19
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp"   }, // 20
    // relax
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 21
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 22
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 23
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 24
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 25
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 26
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 27
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 28
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 29
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 30
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 31
];

// 12 ticks — uneven rhythm: vowels hold 3 frames, transitions 1-2. Eyes stay engaged.
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  0 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  1 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, //  2 open (vowel)
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, //  3 hold
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, //  4 hold
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  5 close (transition)
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  6 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, //  7 open (next syllable)
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_open_flat",    ears: "ears_style_rounded" }, //  8 hold
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, //  9 close fast
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 10 rest
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_rounded" }, // 11 rest
];

// 12 ticks — ears sharp, eyes slightly more active, brief half-open glance
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  0
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  1
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  2
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  3
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  4
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  5
    Frame { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  6 attentive glance
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  7
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  8
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, //  9
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 10
    Frame { face: "face_fill_blush", eyes: "eyes_open_blush",    mouth: "mouth_flat_neutral", ears: "ears_style_sharp" }, // 11
];
