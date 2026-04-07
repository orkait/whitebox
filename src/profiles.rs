#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExpressionProfile {
    Neutral,
    PetLike,
    Serious,
    Curious,
    Sleepy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionPreset {
    pub profile: ExpressionProfile,
    pub face: &'static str,
    pub eyes: &'static str,
    pub mouth: &'static str,
    pub ears: &'static str,
}

#[derive(Debug, Default, Clone)]
pub struct PresetLibrary;

impl PresetLibrary {
    pub fn get(&self, profile: ExpressionProfile) -> ExpressionPreset {
        match profile {
            ExpressionProfile::Neutral => ExpressionPreset {
                profile,
                face: "face_fill_blush",
                eyes: "eyes_open_blush",
                mouth: "mouth_flat_neutral",
                ears: "ears_style_rounded",
            },
            ExpressionProfile::PetLike => ExpressionPreset {
                profile,
                face: "face_fill_rose",
                eyes: "eyes_happy_closed",
                mouth: "mouth_cat_smile",
                ears: "ears_style_rounded",
            },
            ExpressionProfile::Serious => ExpressionPreset {
                profile,
                face: "face_fill_blush",
                eyes: "eyes_serious_angry",
                mouth: "mouth_chevron_serious",
                ears: "ears_style_sharp",
            },
            ExpressionProfile::Curious => ExpressionPreset {
                profile,
                face: "face_fill_rose",
                eyes: "eyes_open_rose",
                mouth: "mouth_tiny_triangle",
                ears: "ears_style_sharp",
            },
            ExpressionProfile::Sleepy => ExpressionPreset {
                profile,
                face: "face_fill_blush",
                eyes: "eyes_sleepy_flat",
                mouth: "mouth_flat_neutral",
                ears: "ears_style_rounded",
            },
        }
    }
}
