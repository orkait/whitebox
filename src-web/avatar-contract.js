const STANCE_EXPRESSIONS = {
  neutral: {
    face: "face_fill_blush",
    eyes: "eyes_open_blush",
    mouth: "mouth_flat_neutral",
    ears: "ears_style_rounded",
  },
  warm: {
    face: "face_fill_rose",
    eyes: "eyes_half_open_rose",
    mouth: "mouth_soft_smile",
    ears: "ears_style_rounded",
  },
  playful: {
    face: "face_fill_rose",
    eyes: "eyes_excited_squint",
    mouth: "mouth_wavy_cat",
    ears: "ears_style_rounded",
  },
  curious: {
    face: "face_fill_rose",
    eyes: "eyes_open_rose",
    mouth: "mouth_tiny_triangle",
    ears: "ears_style_sharp",
  },
  alert: {
    face: "face_fill_blush",
    eyes: "eyes_open_rose",
    mouth: "mouth_tiny_triangle",
    ears: "ears_style_sharp",
  },
  focused: {
    face: "face_fill_blush",
    eyes: "eyes_half_open_blush",
    mouth: "mouth_flat_neutral",
    ears: "ears_style_sharp",
  },
  guarded: {
    face: "face_fill_blush",
    eyes: "eyes_worried_angled",
    mouth: "mouth_small_frown",
    ears: "ears_style_sharp",
  },
  stern: {
    face: "face_fill_blush",
    eyes: "eyes_serious_angry",
    mouth: "mouth_chevron_serious",
    ears: "ears_style_sharp",
  },
  tired: {
    face: "face_fill_blush",
    eyes: "eyes_sleepy_flat",
    mouth: "mouth_flat_neutral",
    ears: "ears_style_rounded",
  },
  sad: {
    face: "face_fill_blush",
    eyes: "eyes_teary",
    mouth: "mouth_small_frown",
    ears: "ears_style_rounded",
  },
  angry: {
    face: "face_fill_blush",
    eyes: "eyes_serious_angry",
    mouth: "mouth_pout_loop",
    ears: "ears_style_sharp",
  },
};

export const STANCE_IDS = [
  "neutral",
  "warm",
  "playful",
  "curious",
  "alert",
  "focused",
  "guarded",
  "stern",
  "tired",
  "sad",
  "angry",
];

const STANCE_LABELS = {
  neutral: "Neutral",
  warm: "Warm",
  playful: "Playful",
  curious: "Curious",
  alert: "Alert",
  focused: "Focused",
  guarded: "Guarded",
  stern: "Stern",
  tired: "Tired",
  sad: "Sad",
  angry: "Angry",
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

export function resolveAvatarPartsForState({
  stance,
  listenState = "Idle",
  speakState = "Idle",
  tickCount = 0,
}) {
  const normalized = normalizeId(stance ?? "neutral");
  const phase = tickCount % 12;
  const base = expressionPartsForStance(normalized);

  if (speakState === "Speaking") {
    return speakingAnimationForStance(normalized, phase, base);
  }

  if (listenState !== "Idle") {
    return listenAnimationForStance(normalized, listenState, phase, base);
  }

  return idleAnimationForStance(normalized, phase, base);
}

export function layerAssetNames(parts) {
  return [
    `face/${parts.face}.png`,
    `ears/${parts.ears}.png`,
    `eyes/${parts.eyes}.png`,
    `mouth/${parts.mouth}.png`,
  ];
}

function idleAnimationForStance(stanceId, phase, base) {
  switch (stanceId) {
    case "neutral":
      return { ...base, eyes: phase === 8 ? "eyes_soft_closed" : "eyes_open_blush" };
    case "warm":
      return {
        ...base,
        eyes: phase === 5 ? "eyes_soft_closed" : phase >= 8 ? "eyes_half_open_rose" : "eyes_open_rose",
        mouth: "mouth_soft_smile",
      };
    case "playful":
      return {
        ...base,
        eyes: phase < 3 ? "eyes_excited_squint" : "eyes_happy_closed",
        mouth: phase < 3 ? "mouth_wavy_cat" : phase < 6 ? "mouth_cat_smile" : phase < 9 ? "mouth_open_tongue" : "mouth_wavy_cat",
      };
    case "curious":
      return {
        ...base,
        eyes: phase === 5 || phase === 11 ? "eyes_half_open_rose" : "eyes_open_rose",
        mouth: "mouth_tiny_triangle",
      };
    case "alert":
      return {
        ...base,
        eyes: phase === 5 ? "eyes_half_open_blush" : "eyes_open_rose",
        mouth: "mouth_tiny_triangle",
      };
    case "focused":
      return {
        ...base,
        eyes: phase === 8 ? "eyes_soft_closed" : "eyes_half_open_blush",
        mouth: "mouth_flat_neutral",
      };
    case "guarded":
      return { ...base, eyes: "eyes_worried_angled", mouth: "mouth_small_frown" };
    case "stern":
      return { ...base, eyes: "eyes_serious_angry", mouth: "mouth_chevron_serious" };
    case "tired":
      return {
        ...base,
        eyes: phase === 6 || phase === 7 ? "eyes_soft_closed" : "eyes_sleepy_flat",
        mouth: "mouth_flat_neutral",
      };
    case "sad":
      return { ...base, eyes: "eyes_teary", mouth: "mouth_small_frown" };
    case "angry":
      return {
        ...base,
        eyes: "eyes_serious_angry",
        mouth: "mouth_pout_loop",
      };
    default:
      return base;
  }
}

function listenAnimationForStance(stanceId, listenState, phase, base) {
  const idle = idleAnimationForStance(stanceId, phase, base);
  let eyes = idle.eyes;
  if (listenState === "Debouncing") {
    switch (stanceId) {
      case "warm":
      case "curious":
      case "alert":
        eyes = "eyes_half_open_rose";
        break;
      case "playful":
        eyes = phase % 2 === 0 ? "eyes_excited_squint" : "eyes_happy_closed";
        break;
      case "focused":
      case "neutral":
      case "tired":
        eyes = "eyes_half_open_blush";
        break;
      case "guarded":
        eyes = "eyes_worried_angled";
        break;
      case "stern":
      case "angry":
        eyes = "eyes_serious_angry";
        break;
      case "sad":
        eyes = "eyes_teary";
        break;
    }
  } else if (listenState !== "Idle") {
    switch (stanceId) {
      case "neutral":
        eyes = phase % 6 === 0 ? "eyes_half_open_blush" : "eyes_open_blush";
        break;
      case "warm":
      case "curious":
      case "alert":
        eyes = phase % 4 === 0 ? "eyes_half_open_rose" : "eyes_open_rose";
        break;
      case "playful":
        eyes = phase % 4 < 2 ? "eyes_excited_squint" : "eyes_happy_closed";
        break;
      case "focused":
        eyes = "eyes_half_open_blush";
        break;
      case "guarded":
        eyes = "eyes_worried_angled";
        break;
      case "stern":
      case "angry":
        eyes = "eyes_serious_angry";
        break;
      case "tired":
        eyes = "eyes_sleepy_flat";
        break;
      case "sad":
        eyes = "eyes_teary";
        break;
    }
  }
  let ears = idle.ears;
  if (!["Idle"].includes(listenState) && !["warm", "playful", "tired", "sad"].includes(stanceId)) {
    ears = "ears_style_sharp";
  }
  return { ...idle, eyes, ears };
}

function speakingAnimationForStance(stanceId, phase, base) {
  const idle = idleAnimationForStance(stanceId, phase, base);
  let eyes = idle.eyes;
  if (stanceId === "playful") {
    eyes = phase < 6 ? "eyes_excited_squint" : "eyes_happy_closed";
  } else if (stanceId === "warm") {
    eyes = phase === 5 ? "eyes_happy_closed" : "eyes_half_open_rose";
  }

  let mouth;
  switch (stanceId) {
    case "warm":
      mouth = phase < 3 || (phase >= 6 && phase < 9) ? "mouth_soft_smile" : "mouth_open_flat";
      break;
    case "playful":
      mouth = phase < 3 ? "mouth_wavy_cat" : phase < 6 ? "mouth_open_tongue" : phase < 9 ? "mouth_cat_smile" : "mouth_open_flat";
      break;
    case "curious":
    case "alert":
      mouth = phase < 6 ? "mouth_tiny_triangle" : "mouth_open_flat";
      break;
    case "focused":
    case "neutral":
      mouth = phase < 6 ? "mouth_flat_neutral" : "mouth_open_flat";
      break;
    case "tired":
      mouth = phase < 9 ? "mouth_flat_neutral" : "mouth_open_flat";
      break;
    case "guarded":
      mouth = phase < 6 || phase >= 9 ? "mouth_small_frown" : "mouth_tiny_triangle";
      break;
    case "sad":
      mouth = phase < 6 || phase >= 9 ? "mouth_small_frown" : "mouth_open_flat";
      break;
    case "stern":
      mouth = phase < 5 || (phase >= 7 && phase < 11) ? "mouth_chevron_serious" : "mouth_tiny_triangle";
      break;
    case "angry":
      mouth = phase < 4 || (phase >= 7 && phase < 10) ? "mouth_pout_loop" : "mouth_open_flat";
      break;
    default:
      mouth = "mouth_open_flat";
      break;
  }
  return { ...idle, eyes, mouth };
}
