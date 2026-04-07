import test from "node:test";
import assert from "node:assert/strict";

import {
  createBrowserState,
  applyBrowserCommand,
} from "../src-web/browser-runtime.js";
import {
  layerAssetNames,
  expressionPartsForStance,
  normalizeId,
  resolveAvatarPartsForState,
} from "../src-web/avatar-contract.js";

test("stance ids normalize enum-like names to kebab-case ids", () => {
  assert.equal(normalizeId("Neutral"), "neutral");
  assert.equal(normalizeId("PetLike"), "pet-like");
  assert.equal(normalizeId("Focused"), "focused");
});

test("stance changes update canonical avatar parts", () => {
  const state = createBrowserState("/cat.png");
  const updated = applyBrowserCommand(state, "set_stance", { stance: "angry" });

  assert.equal(updated.stance, "angry");
  assert.equal(updated.stanceLabel, "Angry");
  assert.deepEqual(updated.avatarParts, expressionPartsForStance("angry"));
});

test("layer asset names include all four avatar layers in render order", () => {
  assert.deepEqual(
    layerAssetNames({
      face: "face_fill_blush",
      eyes: "eyes_open_blush",
      mouth: "mouth_flat_neutral",
      ears: "ears_style_rounded",
    }),
    [
      "face/face_fill_blush.png",
      "ears/ears_style_rounded.png",
      "eyes/eyes_open_blush.png",
      "mouth/mouth_flat_neutral.png",
    ],
  );
});

test("browser listen flow emits final and completed events in order", () => {
  let state = createBrowserState("/cat.png");
  state = applyBrowserCommand(state, "listen");
  state = applyBrowserCommand(state, "tick");

  state = applyBrowserCommand(state, "stop_listening");
  for (let i = 0; i < 8; i += 1) {
    state = applyBrowserCommand(state, "tick");
  }

  assert.equal(state.listenState, "Debouncing");

  state = applyBrowserCommand(state, "tick");

  assert.equal(state.listenState, "Ready");
  assert.equal(state.activity, "idle");
  assert.equal(state.currentTextKind, "heard_text");
  assert.equal(state.currentText, "heard from browser mock");

  const events = applyBrowserCommand(state, "drain_events");
  assert.deepEqual(events, [
    { kind: "listen_started", id: 1 },
    { kind: "listen_partial", id: 1, text: "listening..." },
    { kind: "listen_final", id: 1, text: "heard from browser mock" },
    { kind: "listen_stopped", id: 1, reason: "completed" },
  ]);
});

test("browser speech can interrupt active listen and auto-finish", () => {
  let state = createBrowserState("/cat.png");
  state = applyBrowserCommand(state, "listen");
  state = applyBrowserCommand(state, "speak", { text: "Hello from Whitebox" });

  assert.equal(state.listenState, "Idle");
  assert.equal(state.speakState, "Speaking");
  assert.equal(state.activity, "speaking");
  assert.equal(state.currentTextKind, "spoken_text");

  for (let i = 0; i < 8; i += 1) {
    state = applyBrowserCommand(state, "tick");
  }

  assert.equal(state.speakState, "Idle");
  assert.equal(state.activity, "idle");
  assert.equal(state.currentText, null);
  assert.equal(state.tone, null);

  const events = applyBrowserCommand(state, "drain_events");
  assert.deepEqual(events, [
    { kind: "listen_started", id: 1 },
    { kind: "listen_stopped", id: 1, reason: "interrupted" },
    { kind: "speak_started", id: 2, text: "Hello from Whitebox" },
    { kind: "speak_stopped", id: 2, reason: "completed" },
  ]);
});

test("browser status can be set and cleared independently of stance", () => {
  let state = createBrowserState("/cat.png");
  state = applyBrowserCommand(state, "set_stance", { stance: "warm" });
  state = applyBrowserCommand(state, "set_status", { text: "watching" });

  assert.equal(state.stance, "warm");
  assert.equal(state.statusText, "watching");

  state = applyBrowserCommand(state, "clear_status");
  assert.equal(state.statusText, null);
});

test("focused guarded stern and playful map to distinct visible expressions", () => {
  assert.deepEqual(expressionPartsForStance("focused"), {
    face: "face_fill_blush",
    eyes: "eyes_half_open_blush",
    mouth: "mouth_flat_neutral",
    ears: "ears_style_sharp",
  });
  assert.deepEqual(expressionPartsForStance("guarded"), {
    face: "face_fill_blush",
    eyes: "eyes_worried_angled",
    mouth: "mouth_small_frown",
    ears: "ears_style_sharp",
  });
  assert.deepEqual(expressionPartsForStance("stern"), {
    face: "face_fill_blush",
    eyes: "eyes_serious_angry",
    mouth: "mouth_chevron_serious",
    ears: "ears_style_sharp",
  });
  assert.deepEqual(expressionPartsForStance("playful"), {
    face: "face_fill_rose",
    eyes: "eyes_excited_squint",
    mouth: "mouth_wavy_cat",
    ears: "ears_style_rounded",
  });
});

test("browser listen and speak keep stance-specific expression cues", () => {
  let state = createBrowserState("/cat.png");
  state = applyBrowserCommand(state, "set_stance", { stance: "guarded" });
  state = applyBrowserCommand(state, "listen");

  assert.deepEqual(state.avatarParts, {
    face: "face_fill_blush",
    eyes: "eyes_worried_angled",
    mouth: "mouth_small_frown",
    ears: "ears_style_sharp",
  });

  state = applyBrowserCommand(state, "set_stance", { stance: "angry" });
  state = applyBrowserCommand(state, "speak", { text: "No." });

  assert.deepEqual(state.avatarParts, {
    face: "face_fill_blush",
    eyes: "eyes_serious_angry",
    mouth: "mouth_pout_loop",
    ears: "ears_style_sharp",
  });
});

test("resolveAvatarPartsForState gives playful and focused distinct 12fps-style loops", () => {
  assert.deepEqual(
    resolveAvatarPartsForState({
      stance: "playful",
      listenState: "Idle",
      speakState: "Idle",
      tickCount: 0,
    }),
    {
      face: "face_fill_rose",
      eyes: "eyes_excited_squint",
      mouth: "mouth_wavy_cat",
      ears: "ears_style_rounded",
    },
  );

  assert.deepEqual(
    resolveAvatarPartsForState({
      stance: "playful",
      listenState: "Idle",
      speakState: "Idle",
      tickCount: 3,
    }),
    {
      face: "face_fill_rose",
      eyes: "eyes_happy_closed",
      mouth: "mouth_cat_smile",
      ears: "ears_style_rounded",
    },
  );

  assert.deepEqual(
    resolveAvatarPartsForState({
      stance: "focused",
      listenState: "Idle",
      speakState: "Idle",
      tickCount: 8,
    }),
    {
      face: "face_fill_blush",
      eyes: "eyes_soft_closed",
      mouth: "mouth_flat_neutral",
      ears: "ears_style_sharp",
    },
  );

  assert.deepEqual(
    resolveAvatarPartsForState({
      stance: "angry",
      listenState: "Idle",
      speakState: "Idle",
      tickCount: 10,
    }),
    {
      face: "face_fill_blush",
      eyes: "eyes_serious_angry",
      mouth: "mouth_pout_loop",
      ears: "ears_style_sharp",
    },
  );
});
