use super::Frame;

// 32 ticks — asymmetric blink at tick 8 using half_open_rose as intermediate,
// brief ear perk at tick 22-23 as secondary action
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  6
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  7
    // blink close (fast — 2 frames)
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  8
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  9
    // hold closed (1 frame)
    Frame { face: "face_fill_rose", eyes: "eyes_soft_closed",   mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 10
    // blink open (slow — 3 frames)
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 11
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 12
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 13
    // back to open
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 14
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 15
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 16
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 17
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 18
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 19
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 20
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 21
    // secondary — subtle ear perk
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_sharp"   }, // 22
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_sharp"   }, // 23
    // relax
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 24
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 25
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 26
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 27
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 28
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 29
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 30
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 31
];

// 12 ticks — gentle speech: soft_smile anchors rest, open_flat for vowels,
// happy_closed mid-speech as warm secondary action (eyes briefly close with smile)
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  0 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  1 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" }, //  2 open (vowel)
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" }, //  3 hold
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" }, //  4 hold
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  5 warm happy close — secondary action
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  6 open again
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" }, //  7 next phrase
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_open_flat",  ears: "ears_style_rounded" }, //  8 hold
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  9 close smooth
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 10 rest
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",    mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 11 rest
];

// 12 ticks — stays bright and warm, ears rounded (soft even while listening)
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_half_open_rose", mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  6 gentle attentive dip
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  7
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  8
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, //  9
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 10
    Frame { face: "face_fill_rose", eyes: "eyes_open_rose",     mouth: "mouth_soft_smile", ears: "ears_style_rounded" }, // 11
];
