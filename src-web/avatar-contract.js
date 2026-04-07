import * as neutral from "./animations/neutral.js";
import * as warm    from "./animations/warm.js";
import * as playful from "./animations/playful.js";
import * as curious from "./animations/curious.js";
import * as alert   from "./animations/alert.js";
import * as focused from "./animations/focused.js";
import * as guarded from "./animations/guarded.js";
import * as stern   from "./animations/stern.js";
import * as tired   from "./animations/tired.js";
import * as sad     from "./animations/sad.js";
import * as angry   from "./animations/angry.js";

const ANIMATIONS = {
  neutral,
  warm,
  playful,
  curious,
  alert,
  focused,
  guarded,
  stern,
  tired,
  sad,
  angry,
};

// Fallback static expressions used when avatarParts is not present in a snapshot
// (should not happen in normal operation — kept for defensive rendering only)
const STANCE_EXPRESSIONS = {
  neutral: { face: "face_fill_blush", eyes: "eyes_open_blush",      mouth: "mouth_flat_neutral",    ears: "ears_style_rounded" },
  warm:    { face: "face_fill_rose",  eyes: "eyes_open_rose",       mouth: "mouth_soft_smile",      ears: "ears_style_rounded" },
  playful: { face: "face_fill_rose",  eyes: "eyes_excited_squint",  mouth: "mouth_wavy_cat",        ears: "ears_style_rounded" },
  curious: { face: "face_fill_rose",  eyes: "eyes_open_rose",       mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp"   },
  alert:   { face: "face_fill_blush", eyes: "eyes_open_rose",       mouth: "mouth_tiny_triangle",   ears: "ears_style_sharp"   },
  focused: { face: "face_fill_blush", eyes: "eyes_half_open_blush", mouth: "mouth_flat_neutral",    ears: "ears_style_sharp"   },
  guarded: { face: "face_fill_blush", eyes: "eyes_worried_angled",  mouth: "mouth_small_frown",     ears: "ears_style_sharp"   },
  stern:   { face: "face_fill_blush", eyes: "eyes_serious_angry",   mouth: "mouth_chevron_serious", ears: "ears_style_sharp"   },
  tired:   { face: "face_fill_blush", eyes: "eyes_sleepy_flat",     mouth: "mouth_flat_neutral",    ears: "ears_style_rounded" },
  sad:     { face: "face_fill_blush", eyes: "eyes_teary",           mouth: "mouth_small_frown",     ears: "ears_style_rounded" },
  angry:   { face: "face_fill_blush", eyes: "eyes_serious_angry",   mouth: "mouth_pout_loop",       ears: "ears_style_sharp"   },
};

export const STANCE_IDS = [
  "neutral", "warm", "playful", "curious", "alert",
  "focused", "guarded", "stern", "tired", "sad", "angry",
];

const STANCE_LABELS = {
  neutral: "Neutral", warm: "Warm",    playful: "Playful", curious: "Curious",
  alert:   "Alert",   focused: "Focused", guarded: "Guarded", stern: "Stern",
  tired:   "Tired",   sad: "Sad",      angry: "Angry",
};

export function normalizeId(value) {
  return value
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .replace(/[_\s]+/g, "-")
    .toLowerCase();
}

export function stanceLabelFromId(stanceId) {
  return STANCE_LABELS[normalizeId(stanceId)] ?? normalizeId(stanceId);
}

export function expressionPartsForStance(stanceId) {
  const normalized = normalizeId(stanceId);
  return structuredClone(STANCE_EXPRESSIONS[normalized] ?? STANCE_EXPRESSIONS.neutral);
}

// Mirrors Rust's select_frames() in avatar.rs exactly.
// Each call returns the frame at tickCount % frames.length.
export function resolveAvatarPartsForState({
  stance,
  listenState = "Idle",
  speakState  = "Idle",
  tickCount   = 0,
}) {
  const id       = normalizeId(stance ?? "neutral");
  const anim     = ANIMATIONS[id] ?? ANIMATIONS.neutral;
  const speaking  = speakState === "Speaking";
  const listening = listenState !== "Idle";

  const frames = speaking ? anim.SPEAKING : listening ? anim.LISTENING : anim.IDLE;
  return frames[tickCount % frames.length];
}

export function layerAssetNames(parts) {
  return [
    `face/${parts.face}.png`,
    `ears/${parts.ears}.png`,
    `eyes/${parts.eyes}.png`,
    `mouth/${parts.mouth}.png`,
  ];
}
