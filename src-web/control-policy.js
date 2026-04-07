export function deriveControlState({ pending, snapshot }) {
  const canListen = snapshot.listenState === "Idle";
  const canStopListening = snapshot.listenState === "Hearing";
  const canSpeak = true;
  const canStopSpeaking = snapshot.speakState === "Speaking";

  return {
    stanceDisabled: pending,
    statusDisabled: {
      set: pending,
      clear: pending || !snapshot.statusText,
    },
    actionDisabled: {
      listen: pending || !canListen,
      stopListening: pending || !canStopListening,
      speak: pending || !canSpeak,
      stopSpeaking: pending || !canStopSpeaking,
    },
  };
}

export function canRunCommand({ pending, snapshot, command, payload = {} }) {
  if (pending) {
    return false;
  }
  if (command === "snapshot" || command === "tick" || command === "drain_events") {
    return true;
  }
  if (!snapshot) {
    return false;
  }

  if (command === "listen" && snapshot.listenState !== "Idle") {
    return false;
  }
  if (command === "stop_listening" && snapshot.listenState !== "Hearing") {
    return false;
  }
  if (command === "stop_speaking" && snapshot.speakState !== "Speaking") {
    return false;
  }
  if (command === "speak") {
    return `${payload.text ?? ""}`.trim().length > 0;
  }
  return true;
}
