use super::Frame;

// 24 ticks — excited_squint dominant; happy_closed acts as the natural "blink"
// (no separate half-open needed — squint→happy_closed reads as squeezing eyes shut with joy)
// Mouth cycles through full range of playful expressions
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, //  6
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, //  7
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, //  8
    // happy close — squint→happy_closed is a natural "joy squish", 2 frames (fast in)
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  9
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // 10
    // reopen — 1 frame (squint is already mostly closed, reopening is fast)
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 11
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 12
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 13
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // 14
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // 15
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, // 16
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, // 17
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, // 18
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 19
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 20
    // second happy close
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // 21
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // 22
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 23
];

// 12 ticks — energetic mouth with excited eyes, happy close mid-speech as secondary
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, //  1 vowel
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, //  2 hold
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" }, //  3 hold
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  4 transition
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_flat",   ears: "ears_style_rounded" }, //  6 next phrase open
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_flat",   ears: "ears_style_rounded" }, //  7 hold
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, //  8 bounce back
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, //  9 happy close (secondary)
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // 10 hold
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" }, // 11 reopen
];

// 12 ticks — ears stay rounded, excited eyes attentive, happy close shows engagement
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  0
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  1
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  2
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  3
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  4
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  5
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  6 engaged nod
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  7
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  8
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, //  9
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, // 10
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" }, // 11
];
