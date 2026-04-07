import test from "node:test";
import assert from "node:assert/strict";

import { canRunCommand, deriveControlState } from "../src-web/control-policy.js";

test("stance controls stay enabled while speaking", () => {
  const state = deriveControlState({
    pending: false,
    snapshot: {
      listenState: "Idle",
      speakState: "Speaking",
      statusText: null,
    },
  });

  assert.equal(state.stanceDisabled, false);
  assert.equal(state.actionDisabled.listen, false);
  assert.equal(state.actionDisabled.stopSpeaking, false);
});

test("listen remains disabled while a listen session is actively hearing", () => {
  const state = deriveControlState({
    pending: false,
    snapshot: {
      listenState: "Hearing",
      speakState: "Idle",
      statusText: null,
    },
  });

  assert.equal(state.actionDisabled.listen, true);
  assert.equal(state.actionDisabled.stopListening, false);
});

test("speak remains allowed while speaking because it interrupts the prior speech", () => {
  assert.equal(
    canRunCommand({
      pending: false,
      snapshot: {
        listenState: "Idle",
        speakState: "Speaking",
        statusText: null,
      },
      command: "speak",
      payload: { text: "override" },
    }),
    true,
  );
});

test("set_status is blocked only while a command is pending", () => {
  assert.equal(
    canRunCommand({
      pending: false,
      snapshot: {
        listenState: "Idle",
        speakState: "Idle",
        statusText: null,
      },
      command: "set_status",
      payload: { text: "watching" },
    }),
    true,
  );
});
