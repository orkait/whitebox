import {
  expressionPartsForStance,
  resolveAvatarPartsForState,
  stanceLabelFromId,
  normalizeId,
} from "./avatar-contract.js";

export function createBrowserState(browserAvatar) {
  return {
    stance: "neutral",
    stanceLabel: "Neutral",
    activity: "idle",
    listenState: "Idle",
    speakState: "Idle",
    currentText: null,
    currentTextKind: null,
    tone: null,
    statusText: null,
    infoFragments: [],
    diagnostics: ["browser mock active"],
    renderSize: [1024, 1024],
    sttBackend: "Mock",
    ttsBackend: "Mock",
    avatarParts: expressionPartsForStance("neutral"),
    avatarDataUrl: browserAvatar,
    _tickCount: 0,
    _debounceTicksRemaining: 0,
    _speakTicksRemaining: 0,
    _activeListenId: null,
    _activeSpeakId: null,
    _nextSessionId: 1,
    _events: [],
  };
}

export function applyBrowserCommand(state, command, payload = {}) {
  const next = structuredClone(state);

  switch (command) {
    case "snapshot":
      return next;
    case "tick":
      return tickBrowserState(next);
    case "listen":
      return beginListening(next);
    case "stop_listening":
      return stopListening(next);
    case "speak":
      return speak(next, payload.text ?? "Hello from Whitebox");
    case "stop_speaking":
      return stopSpeaking(next, "cancelled");
    case "set_stance":
      return setStance(next, payload.stance);
    case "set_status":
      next.statusText = payload.text ?? null;
      pushDiagnostic(next, "status updated");
      return next;
    case "clear_status":
      next.statusText = null;
      pushDiagnostic(next, "status cleared");
      return next;
    case "drain_events": {
      const events = next._events.slice();
      next._events = [];
      return events;
    }
    default:
      throw new Error(`Unsupported browser command: ${command}`);
  }
}

function tickBrowserState(state) {
  state._tickCount += 1;
  if (state.listenState === "Hearing") {
    pushEvent(state, {
      kind: "listen_partial",
      id: state._activeListenId,
      text: "listening...",
    });
  }

  if (state._debounceTicksRemaining > 0) {
    state._debounceTicksRemaining -= 1;
    if (state._debounceTicksRemaining === 0 && state._activeListenId !== null) {
      state.listenState = "Ready";
      state.activity = "idle";
      state.currentText = "heard from browser mock";
      state.currentTextKind = "heard_text";
      pushEvent(state, {
        kind: "listen_final",
        id: state._activeListenId,
        text: state.currentText,
      });
      pushEvent(state, {
        kind: "listen_stopped",
        id: state._activeListenId,
        reason: "completed",
      });
      state._activeListenId = null;
      pushDiagnostic(state, "utterance became ready");
    }
  }

  if (state._speakTicksRemaining > 0) {
    state._speakTicksRemaining -= 1;
    if (state._speakTicksRemaining === 0 && state._activeSpeakId !== null) {
      finishSpeech(state, "completed", "speech finished");
    }
  }

  syncAvatarParts(state);
  return state;
}

function beginListening(state) {
  if (state._activeListenId !== null) {
    throw new Error("busy: listen session already active");
  }
  if (state._activeSpeakId !== null) {
    pushEvent(state, {
      kind: "speak_stopped",
      id: state._activeSpeakId,
      reason: "interrupted",
    });
    finishSpeechState(state);
  }

  state._activeListenId = allocSessionId(state);
  state.activity = "listening";
  state.listenState = "Hearing";
  state.speakState = "Idle";
  state.currentText = null;
  state.currentTextKind = null;
  state.tone = null;
  state._debounceTicksRemaining = 0;
  pushEvent(state, {
    kind: "listen_started",
    id: state._activeListenId,
  });
  pushDiagnostic(state, "listening started");
  syncAvatarParts(state);
  return state;
}

function stopListening(state) {
  if (state._activeListenId === null) {
    throw new Error("no_active_session: no active listening session");
  }
  state.listenState = "Debouncing";
  state._debounceTicksRemaining = 9;
  pushDiagnostic(state, "listening stopped");
  syncAvatarParts(state);
  return state;
}

function speak(state, text) {
  const normalizedText = `${text}`.trim() || "Hello from Whitebox";

  if (state._activeListenId !== null) {
    pushEvent(state, {
      kind: "listen_stopped",
      id: state._activeListenId,
      reason: "interrupted",
    });
    state._activeListenId = null;
    state.listenState = "Idle";
    state._debounceTicksRemaining = 0;
  }

  if (state._activeSpeakId !== null) {
    pushEvent(state, {
      kind: "speak_stopped",
      id: state._activeSpeakId,
      reason: "interrupted",
    });
  }

  state._activeSpeakId = allocSessionId(state);
  state.activity = "speaking";
  state.listenState = "Idle";
  state.speakState = "Speaking";
  state.currentText = normalizedText;
  state.currentTextKind = "spoken_text";
  state.tone = toneForStance(state.stance);
  state._speakTicksRemaining = 8;
  pushEvent(state, {
    kind: "speak_started",
    id: state._activeSpeakId,
    text: normalizedText,
  });
  pushDiagnostic(state, `speech started: ${normalizedText}`);
  syncAvatarParts(state);
  return state;
}

function stopSpeaking(state, reason) {
  if (state._activeSpeakId === null) {
    throw new Error("no_active_session: no active speaking session");
  }
  finishSpeech(state, reason, "speaking stopped");
  return state;
}

function finishSpeech(state, reason, message) {
  pushEvent(state, {
    kind: "speak_stopped",
    id: state._activeSpeakId,
    reason,
  });
  finishSpeechState(state);
  pushDiagnostic(state, message);
}

function finishSpeechState(state) {
  state._activeSpeakId = null;
  state._speakTicksRemaining = 0;
  state.speakState = "Idle";
  state.activity = state._activeListenId !== null ? "listening" : "idle";
  state.currentText = null;
  state.currentTextKind = null;
  state.tone = null;
  syncAvatarParts(state);
}

function setStance(state, stance) {
  const normalized = normalizeId(stance ?? "neutral");
  state.stance = normalized;
  state.stanceLabel = stanceLabelFromId(normalized);
  syncAvatarParts(state);
  pushDiagnostic(state, `stance set to ${normalized}`);
  return state;
}

function pushDiagnostic(state, message) {
  state.diagnostics.push(message);
  if (state.diagnostics.length > 8) {
    state.diagnostics.splice(0, state.diagnostics.length - 8);
  }
}

function pushEvent(state, event) {
  state._events.push(event);
}

function allocSessionId(state) {
  const next = state._nextSessionId;
  state._nextSessionId += 1;
  return next;
}

function toneForStance(stance) {
  switch (normalizeId(stance)) {
    case "warm":
      return "warm";
    case "playful":
      return "pet-like";
    case "alert":
    case "focused":
    case "guarded":
    case "stern":
    case "angry":
      return "serious";
    default:
      return "neutral";
  }
}

function syncAvatarParts(state) {
  state.avatarParts = resolveAvatarPartsForState({
    stance: state.stance,
    listenState: state.listenState,
    speakState: state.speakState,
    tickCount: state._tickCount,
  });
}
