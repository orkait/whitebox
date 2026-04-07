import "./styles.css";
import {
  layerAssetNames,
  expressionPartsForStance,
  STANCE_IDS,
  stanceLabelFromId,
} from "./avatar-contract.js";
import { applyBrowserCommand, createBrowserState } from "./browser-runtime.js";
import { canRunCommand, deriveControlState } from "./control-policy.js";

const app = document.querySelector("#app");
const browserAvatar = new URL("../cat/preview/cat_preview_full.png", import.meta.url).href;
const catAssets = import.meta.glob("../cat/**/*.png", {
  eager: true,
  import: "default",
});
const assetUrls = Object.fromEntries(
  Object.entries(catAssets).map(([path, url]) => [path.replace("../cat/", ""), url]),
);

const stanceButtons = STANCE_IDS.map(
  (stance) => `<button data-stance="${stance}">${stanceLabelFromId(stance)}</button>`,
).join("");

app.innerHTML = `
  <div class="shell">
    <main class="workspace">
      <section class="avatar-panel panel">
        <div class="panel-header">
          <div>
            <p class="panel-label">Avatar</p>
          </div>
          <div class="chips">
            <span class="chip" id="stance-chip">Neutral</span>
            <span class="chip" id="activity-chip">Idle</span>
          </div>
        </div>
        <div class="avatar-stage">
          <div class="avatar-frame">
            <img class="avatar-layer" id="avatar-face" alt="" aria-hidden="true" />
            <img class="avatar-layer" id="avatar-ears" alt="" aria-hidden="true" />
            <img class="avatar-layer" id="avatar-eyes" alt="" aria-hidden="true" />
            <img class="avatar-layer" id="avatar-mouth" alt="" aria-hidden="true" />
            <img class="avatar-fallback" id="avatar-fallback" alt="Whitebox avatar" />
          </div>
        </div>
      </section>

      <aside class="sidebar">
        <section class="controls panel">
          <div class="panel-header">
            <div>
              <p class="panel-label">Body</p>
              <h2 class="sidebar-title">Actions</h2>
            </div>
          </div>
          <div class="group" id="speech-controls">
            <span class="group-label">Speech</span>
            <input id="speech-input" type="text" value="Hello from Whitebox" spellcheck="false" />
            <button data-action="listen">Listen</button>
            <button data-action="stop_listening">Stop Listen</button>
            <button data-action="speak">Speak</button>
            <button data-action="stop_speaking">Stop Speech</button>
          </div>
          <div class="group" id="stance-controls">
            <span class="group-label">Stance</span>
            ${stanceButtons}
          </div>
          <div class="group" id="status-controls">
            <span class="group-label">Status</span>
            <input id="status-input" type="text" placeholder="Visible status" spellcheck="false" />
            <button data-action="set_status">Set Status</button>
            <button data-action="clear_status">Clear Status</button>
          </div>
        </section>

        <section class="state-panel panel">
          <div class="panel-header">
            <div>
              <p class="panel-label">Runtime</p>
              <h2 class="sidebar-title">State</h2>
            </div>
          </div>
          <div class="state-scroll">
            <dl class="state-grid" id="state-grid"></dl>
          </div>

          <div class="stream-block stream-block-info">
            <h3 class="stream-title">Info Fragments</h3>
            <div class="stream-scroll">
              <ul id="info-list"></ul>
            </div>
          </div>

          <div class="stream-block stream-block-diagnostics">
            <h3 class="stream-title">Diagnostics</h3>
            <div class="stream-scroll">
              <ul id="diagnostics-list"></ul>
            </div>
          </div>
        </section>

        <footer class="status panel">
          <div>
            <p class="panel-label">Status</p>
            <h2 id="status-title">Neutral</h2>
          </div>
          <p id="status-detail">Idle body surface.</p>
        </footer>
      </aside>
    </main>
  </div>
`;

const stateGrid = document.querySelector("#state-grid");
const infoList = document.querySelector("#info-list");
const diagnosticsList = document.querySelector("#diagnostics-list");
const avatarFace = document.querySelector("#avatar-face");
const avatarEars = document.querySelector("#avatar-ears");
const avatarEyes = document.querySelector("#avatar-eyes");
const avatarMouth = document.querySelector("#avatar-mouth");
const avatarFallback = document.querySelector("#avatar-fallback");
const statusTitle = document.querySelector("#status-title");
const statusDetail = document.querySelector("#status-detail");
const stanceChip = document.querySelector("#stance-chip");
const activityChip = document.querySelector("#activity-chip");
const speechInput = document.querySelector("#speech-input");
const statusInput = document.querySelector("#status-input");

let pending = false;
let transport;
let currentSnapshot = null;
let browserState = createBrowserState(browserAvatar);

const activityCopy = {
  idle: "Idle body surface.",
  listening: "Listening for a live utterance.",
  thinking: "Holding a quiet internal posture.",
  speaking: "Speaking through the body shell.",
};

async function initTransport() {
  if ("__TAURI_INTERNALS__" in window) {
    const { invoke } = await import("@tauri-apps/api/core");
    return { invoke };
  }
  return {
    invoke(command, payload = {}) {
      const result = applyBrowserCommand(browserState, command, payload);
      if (command === "drain_events") {
        return structuredClone(result);
      }
      browserState = result;
      return structuredClone(browserState);
    },
  };
}

function renderAvatar(snapshot) {
  const parts = snapshot.avatarParts ?? expressionPartsForStance(snapshot.stance ?? "neutral");
  const layerNames = layerAssetNames(parts);
  const layerUrls = layerNames.map((name) => assetUrls[name] ?? null);
  const hasAllLayers = layerUrls.every(Boolean);

  if (hasAllLayers) {
    [avatarFace, avatarEars, avatarEyes, avatarMouth].forEach((node, index) => {
      node.src = layerUrls[index];
      node.hidden = false;
    });
    avatarFallback.hidden = true;
    avatarFallback.removeAttribute("src");
    return;
  }

  [avatarFace, avatarEars, avatarEyes, avatarMouth].forEach((node) => {
    node.hidden = true;
    node.removeAttribute("src");
  });
  avatarFallback.hidden = false;
  avatarFallback.src = snapshot.avatarDataUrl;
}

function renderList(node, items, emptyLabel) {
  if (!items.length) {
    node.innerHTML = `<li class="empty">${emptyLabel}</li>`;
    return;
  }
  node.innerHTML = items.map((item) => `<li>${item}</li>`).join("");
}

function renderState(snapshot) {
  currentSnapshot = snapshot;
  renderAvatar(snapshot);
  statusTitle.textContent = snapshot.stanceLabel;
  statusDetail.textContent = snapshot.currentText
    ? `${snapshot.currentTextKind}: ${snapshot.currentText}`
    : snapshot.statusText ?? activityCopy[snapshot.activity] ?? "Visible body runtime.";
  stanceChip.textContent = snapshot.stanceLabel;
  activityChip.textContent = capitalize(snapshot.activity);
  statusInput.value = snapshot.statusText ?? "";

  const pairs = [
    ["Stance", snapshot.stanceLabel],
    ["Activity", capitalize(snapshot.activity)],
    ["Listen", snapshot.listenState],
    ["Speak", snapshot.speakState],
    ["Status", snapshot.statusText ?? "-"],
    ["STT", snapshot.sttBackend],
    ["TTS", snapshot.ttsBackend],
    ["Surface", `${snapshot.renderSize[0]}x${snapshot.renderSize[1]}`],
    ["Tone", snapshot.tone ?? "-"],
    ["Avatar", `${snapshot.avatarParts.eyes} / ${snapshot.avatarParts.mouth}`],
  ];

  stateGrid.innerHTML = pairs
    .map(([label, value]) => `<dt>${label}</dt><dd>${value}</dd>`)
    .join("");

  renderList(infoList, snapshot.infoFragments, "No live fragments yet.");
  renderList(diagnosticsList, snapshot.diagnostics, "No diagnostics yet.");

  document.querySelectorAll("[data-stance]").forEach((button) => {
    button.classList.toggle("is-active", button.dataset.stance === snapshot.stance);
  });

  syncControls(snapshot);
}

function syncControls(snapshot) {
  const controlState = deriveControlState({ pending, snapshot });

  document.querySelectorAll("[data-stance]").forEach((button) => {
    button.disabled = controlState.stanceDisabled;
  });

  document.querySelectorAll("[data-action]").forEach((button) => {
    const actionMap = {
      listen: controlState.actionDisabled.listen,
      stop_listening: controlState.actionDisabled.stopListening,
      speak: controlState.actionDisabled.speak,
      stop_speaking: controlState.actionDisabled.stopSpeaking,
      set_status: controlState.statusDisabled.set,
      clear_status: controlState.statusDisabled.clear,
    };
    button.disabled = actionMap[button.dataset.action];
  });
}

function canRun(command, payload = {}) {
  return canRunCommand({ pending, snapshot: currentSnapshot, command, payload });
}

async function call(command, payload = {}) {
  if (!transport) {
    return;
  }
  if (!canRun(command, payload)) {
    return;
  }
  pending = true;
  syncControls(currentSnapshot ?? createBrowserState(browserAvatar));
  try {
    const snapshot = await transport.invoke(command, payload);
    renderState(snapshot);
  } catch (error) {
    console.error(error);
    statusTitle.textContent = "Error";
    statusDetail.textContent = String(error);
  } finally {
    pending = false;
    if (currentSnapshot) {
      syncControls(currentSnapshot);
    }
  }
}

document.querySelector("#speech-controls").addEventListener("click", (event) => {
  const target = event.target.closest("button[data-action]");
  if (!target) return;
  if (target.dataset.action === "speak") {
    call("speak", { text: speechInput.value });
    return;
  }
  call(target.dataset.action);
});

document.querySelector("#stance-controls").addEventListener("click", (event) => {
  const target = event.target.closest("button[data-stance]");
  if (!target) return;
  call("set_stance", { stance: target.dataset.stance });
});

document.querySelector("#status-controls").addEventListener("click", (event) => {
  const target = event.target.closest("button[data-action]");
  if (!target) return;
  if (target.dataset.action === "set_status") {
    call("set_status", { text: statusInput.value });
    return;
  }
  call(target.dataset.action);
});

window.addEventListener("keydown", (event) => {
  const key = event.key.toLowerCase();
  if (key === "l") call("listen");
  if (key === "k") call("stop_listening");
  if (key === "s") call("speak", { text: speechInput.value });
  if (key === "f") call("stop_speaking");
});

async function bootstrap() {
  transport = await initTransport();
  await call("snapshot");
  setInterval(() => {
    if (!pending) {
      call("tick");
    }
  }, 41);
}

function capitalize(value) {
  return value.charAt(0).toUpperCase() + value.slice(1);
}

bootstrap();
