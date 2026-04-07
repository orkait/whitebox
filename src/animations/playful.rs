use super::Frame;

// 24 ticks — excited_squint dominant, brief happy_closed at ticks 9-10 and 21-22
// mouth cycles through playful expressions across the loop
pub const IDLE: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // happy close
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // happy close
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // happy close
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // happy close
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
];

// 12 ticks — energetic mouth, excited eyes throughout with brief happy close
pub const SPEAKING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_tongue", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_flat",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_open_flat",   ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // happy close
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile",   ears: "ears_style_rounded" }, // happy close
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_wavy_cat",    ears: "ears_style_rounded" },
];

// 12 ticks — ears perk up, excited eyes attentive
pub const LISTENING: &[Frame] = &[
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_happy_closed",   mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
    Frame { face: "face_fill_rose", eyes: "eyes_excited_squint", mouth: "mouth_cat_smile", ears: "ears_style_rounded" },
];
