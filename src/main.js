const invoke = window.__TAURI_INTERNALS__.invoke;

// ─── DOM refs ───
const stateLoading  = document.getElementById("state-loading");
const stateOffline  = document.getElementById("state-offline");
const stateError    = document.getElementById("state-error");
const stateMatches  = document.getElementById("state-matches");
const matchList     = document.getElementById("match-list");
const statusDot     = document.getElementById("status-indicator");
const statusText    = document.getElementById("status-text");
const errorMsg      = document.getElementById("error-msg");
const refreshBtn    = document.getElementById("refresh-btn");

// Reckon connection DOM refs
const reckonConnectBtn  = document.getElementById("reckon-connect-btn");
const reckonConnectText = document.getElementById("reckon-connect-text");
const loginOverlay      = document.getElementById("login-overlay");
const loginCloseBtn     = document.getElementById("login-close-btn");
const loginForm         = document.getElementById("login-form");
const loginUsername      = document.getElementById("login-username");
const loginPassword      = document.getElementById("login-password");
const loginError        = document.getElementById("login-error");
const loginSubmitBtn    = document.getElementById("login-submit-btn");
const loginSubmitText   = document.getElementById("login-submit-text");
const loginSpinner      = document.getElementById("login-spinner");

// Track which match detail panel is currently open
let openDetailId = null;

// Store loaded matches so detail panel can access summary info
let loadedMatches = [];

// Track Reckon Bolt connection
let reckonUser = null;

// ─── Global Reckon data (available to any component) ───
let reckonPlayers = [];
let reckonTeams = [];

// ─── State management ───
function showState(state) {
  stateLoading.classList.add("hidden");
  stateOffline.classList.add("hidden");
  stateError.classList.add("hidden");
  stateMatches.classList.add("hidden");
  state.classList.remove("hidden");
}

// ─── Toasts ───
function showToast(message, options = {}) {
  const container = document.getElementById("toast-container");
  if (!container) return;
  const { apiResponse, status, title = "Error" } = options;
  const el = document.createElement("div");
  el.className = "toast";
  el.setAttribute("role", "alert");
  let html = `<span class="toast-title">${escapeHtml(title)}</span><p class="toast-message">${escapeHtml(message)}</p>`;
  if (status != null) {
    html += `<p class="toast-message">Status: ${escapeHtml(String(status))}</p>`;
  }
  if (apiResponse != null && apiResponse !== "") {
    html += `<pre class="toast-api-response">${escapeHtml(apiResponse)}</pre>`;
  }
  html += '<button type="button" class="toast-close" aria-label="Close">&times;</button>';
  el.innerHTML = html;
  const closeBtn = el.querySelector(".toast-close");
  const dismiss = () => {
    el.style.animation = "toast-in 0.15s ease-out reverse";
    setTimeout(() => el.remove(), 150);
  };
  closeBtn.addEventListener("click", dismiss);
  container.appendChild(el);
  setTimeout(dismiss, 12000);
}

function setOnline(online) {
  statusDot.className = online ? "status online" : "status offline";
  statusText.textContent = online ? "Valorant Running" : "Offline";
}

// ─── Load match history ───
async function loadMatches() {
  showState(stateLoading);
  refreshBtn.disabled = true;
  openDetailId = null;

  try {
    const running = await invoke("check_valorant_running");

    if (!running) {
      setOnline(false);
      showState(stateOffline);
      refreshBtn.disabled = false;
      return;
    }

    setOnline(true);
    const matches = await invoke("get_match_history");

    if (matches.length === 0) {
      errorMsg.textContent = "No matches found";
      showState(stateError);
      refreshBtn.disabled = false;
      return;
    }

    loadedMatches = matches;
    renderMatches(matches);
    showState(stateMatches);
  } catch (err) {
    console.error(err);
    setOnline(false);
    errorMsg.textContent = typeof err === "string" ? err : "Failed to load match history";
    showState(stateError);
  }

  refreshBtn.disabled = false;
}

// ─── Render match cards ───
function renderMatches(matches) {
  matchList.innerHTML = "";

  for (const m of matches) {
    // Wrapper: holds the card + the detail panel
    const wrapper = document.createElement("div");
    wrapper.className = "match-wrapper";

    const card = document.createElement("div");
    card.className = `match-card ${m.won ? "win" : "loss"}`;
    card.dataset.matchId = m.matchId;

    // Result badge
    const resultClass = m.won ? "badge-win" : "badge-loss";
    const resultText  = m.won ? "WIN" : "LOSS";

    // Queue badge
    let queueClass = "badge-queue";
    if (m.isCustomGame) queueClass += " badge-custom";
    else if (m.isRanked) queueClass += " badge-ranked";

    // Format time
    const date = new Date(m.gameStartTime);
    const timeStr = formatDate(date);

    // Duration
    const duration = m.gameLengthSecs
      ? `${Math.floor(m.gameLengthSecs / 60)}m ${m.gameLengthSecs % 60}s`
      : "";

    card.innerHTML = `
      <div class="match-result">
        <span class="badge ${resultClass}">${resultText}</span>
        <span class="match-score">${m.roundsWon} - ${m.roundsLost}</span>
      </div>
      <div class="match-info">
        <span class="match-map">${escapeHtml(m.mapName)}</span>
        <span class="${queueClass}">${escapeHtml(m.queueDisplayName)}</span>
        ${m.isCustomGame && m.customGameName ? `<span class="match-custom-name">${escapeHtml(m.customGameName)}</span>` : ""}
      </div>
      <div class="match-agent">
        <span>${escapeHtml(m.playerAgent)}</span>
      </div>
      <div class="match-kda">
        <span class="kda-kills">${m.playerKills}</span>
        <span class="kda-sep">/</span>
        <span class="kda-deaths">${m.playerDeaths}</span>
        <span class="kda-sep">/</span>
        <span class="kda-assists">${m.playerAssists}</span>
      </div>
      <div class="match-meta">
        <span class="match-time">${timeStr}</span>
        ${duration ? `<span class="match-duration">${duration}</span>` : ""}
      </div>
    `;

    // Click handler: toggle detail panel
    card.addEventListener("click", () => toggleDetail(wrapper, m.matchId));

    wrapper.appendChild(card);
    matchList.appendChild(wrapper);
  }
}

// ─── Toggle match detail panel ───
async function toggleDetail(wrapper, matchId) {
  const existing = wrapper.querySelector(".detail-panel");

  // If already open, close it
  if (existing) {
    existing.remove();
    openDetailId = null;
    wrapper.querySelector(".match-card").classList.remove("expanded");
    return;
  }

  // Close any other open panel
  const prev = document.querySelector(".detail-panel");
  if (prev) {
    prev.closest(".match-wrapper").querySelector(".match-card").classList.remove("expanded");
    prev.remove();
  }

  openDetailId = matchId;
  wrapper.querySelector(".match-card").classList.add("expanded");

  // Show loading placeholder
  const panel = document.createElement("div");
  panel.className = "detail-panel";
  panel.innerHTML = `<div class="detail-loading"><div class="spinner"></div></div>`;
  wrapper.appendChild(panel);

  try {
    const detail = await invoke("get_match_detail", { matchId });
    // Check if user clicked away while loading
    if (openDetailId !== matchId) return;

    // Query SoloQAccount entries for all players in the game (when logged in).
    // Missing accounts are created using server and player names from the match.
    let accounts = [];
    if (reckonUser) {
      const allPlayers = [
        ...(detail.teamBlue || []),
        ...(detail.teamRed || []),
      ];
      const puuids = allPlayers.map((p) => p.puuid).filter(Boolean);
      const players = allPlayers.map((p) => ({
        puuid: p.puuid,
        accountName: p.name,
      }));
      try {
        accounts = await invoke("reckon_get_soloq_accounts", {
          puuids,
          server: detail.server || null,
          players: detail.server ? players : null,
        });
      } catch (e) {
        console.warn("Failed to fetch SoloQ accounts for match:", e);
        const msg = typeof e === "string" ? e : (e?.message || String(e));
        try {
          const data = JSON.parse(msg);
          if (data.soloqCreateError) {
            showToast(data.message, {
              title: "SoloQ account create failed",
              status: data.status,
              apiResponse: data.apiResponse,
            });
          } else {
            showToast(msg, { title: "Error" });
          }
        } catch (_) {
          showToast(msg, { title: "Error" });
        }
      }
    }

    if (openDetailId !== matchId) return;
    renderDetailPanel(panel, detail, accounts);
  } catch (err) {
    console.error(err);
    if (openDetailId !== matchId) return;
    panel.innerHTML = `<p class="detail-error">Failed to load match details</p>`;
  }
}

// ─── Render the expanded scoreboard ───
function renderDetailPanel(panel, detail, accounts = []) {
  const blueWon = detail.blueRoundsWon > detail.redRoundsWon;
  const redWon  = detail.redRoundsWon > detail.blueRoundsWon;

  // Map puuid -> account so we can show player_id below name when set
  const accountByPuuid = new Map();
  for (const a of accounts) {
    if (a.puuid) accountByPuuid.set(a.puuid, a);
  }

  // Check if this match is eligible for upload (competitive or custom game)
  const canUpload = reckonUser && (detail.isCustomGame || detail.isRanked);

  panel.innerHTML = `
    <div class="scoreboard">
      <div class="scoreboard-team blue">
        ${canUpload ? '<div class="team-selector-slot" data-side="blue"></div>' : ''}
        <div class="team-header">
          <span class="team-label blue-label">ATTACKERS</span>
          <span class="team-rounds ${blueWon ? 'team-won' : ''}">${detail.blueRoundsWon}</span>
        </div>
        <div class="team-table">
          <div class="table-header">
            <span class="col-name">Player</span>
            <span class="col-agent">Agent</span>
            <span class="col-k">K</span>
            <span class="col-d">D</span>
            <span class="col-a">A</span>
            <span class="col-score">Score</span>
          </div>
          ${detail.teamBlue.map(p => playerRow(p, accountByPuuid)).join("")}
        </div>
      </div>
      <div class="scoreboard-divider"></div>
      <div class="scoreboard-team red">
        ${canUpload ? '<div class="team-selector-slot" data-side="red"></div>' : ''}
        <div class="team-header">
          <span class="team-label red-label">DEFENDERS</span>
          <span class="team-rounds ${redWon ? 'team-won' : ''}">${detail.redRoundsWon}</span>
        </div>
        <div class="team-table">
          <div class="table-header">
            <span class="col-name">Player</span>
            <span class="col-agent">Agent</span>
            <span class="col-k">K</span>
            <span class="col-d">D</span>
            <span class="col-a">A</span>
            <span class="col-score">Score</span>
          </div>
          ${detail.teamRed.map(p => playerRow(p, accountByPuuid)).join("")}
        </div>
      </div>
    </div>
    ${canUpload ? renderUploadSection() : ''}
  `;

  // Mount team selectors into the slots if upload is available
  if (canUpload) {
    const blueSlot = panel.querySelector('.team-selector-slot[data-side="blue"]');
    const redSlot = panel.querySelector('.team-selector-slot[data-side="red"]');

    const blueSelector = createTeamSelector(blueSlot, {
      placeholder: "Select blue side team\u2026",
    });

    const redSelector = createTeamSelector(redSlot, {
      placeholder: "Select red side team\u2026",
    });

    // Wire up the upload button
    const uploadBtn = panel.querySelector(".upload-btn");
    const uploadError = panel.querySelector(".upload-error");
    const uploadSuccess = panel.querySelector(".upload-success");

    if (uploadBtn) {
      uploadBtn.addEventListener("click", () => {
        handleUploadMatch({
          matchId: detail.matchId,
          blueSelector,
          redSelector,
          uploadBtn,
          uploadError,
          uploadSuccess,
        });
      });
    }
  }
}

// ─── Upload section HTML ───
function renderUploadSection() {
  return `
    <div class="upload-section">
      <div class="upload-body">
        <button class="upload-btn" type="button">
          <span class="upload-btn-text">Upload to Reckon Bolt</span>
          <div class="spinner-small upload-spinner hidden"></div>
        </button>
      </div>
      <p class="upload-error hidden"></p>
      <p class="upload-success hidden"></p>
      <div class="unlinked-accounts hidden"></div>
    </div>
  `;
}

// ─── Upload match handler ───
async function handleUploadMatch({ matchId, blueSelector, redSelector, uploadBtn, uploadError, uploadSuccess }) {
  const uploadSection = uploadBtn.closest(".upload-section");
  const unlinkedContainer = uploadSection.querySelector(".unlinked-accounts");

  // Reset messages & unlinked UI
  uploadError.classList.add("hidden");
  uploadSuccess.classList.add("hidden");
  unlinkedContainer.classList.add("hidden");
  unlinkedContainer.innerHTML = "";

  // Validate team selections
  const blueTeam = blueSelector.getValue();
  const redTeam = redSelector.getValue();

  if (!blueTeam) {
    uploadError.textContent = "Please select a team for the blue (attacker) side.";
    uploadError.classList.remove("hidden");
    return;
  }
  if (!redTeam) {
    uploadError.textContent = "Please select a team for the red (defender) side.";
    uploadError.classList.remove("hidden");
    return;
  }

  // Show loading state
  setUploadLoading(uploadBtn, true);

  try {
    const result = await invoke("reckon_upload_match", {
      team1: String(blueTeam.id),
      team2: String(redTeam.id),
      matchId: matchId,
    });

    // Check if the response contains unlinked accounts
    if (result && result.unlinkedAccounts && result.unlinkedAccounts.length > 0) {
      renderUnlinkedAccounts(unlinkedContainer, result.unlinkedAccounts, result.server, {
        matchId,
        blueSelector,
        redSelector,
        uploadBtn,
        uploadError,
        uploadSuccess,
      });
    } else if (result && result.success) {
      uploadSuccess.textContent = "Match uploaded successfully!";
      uploadSuccess.classList.remove("hidden");
    } else if (result && result.error) {
      uploadError.textContent = result.error;
      uploadError.classList.remove("hidden");
    }
  } catch (err) {
    console.error("Upload failed:", err);
    uploadError.textContent = typeof err === "string" ? err : "Upload failed. Please try again.";
    uploadError.classList.remove("hidden");
  }

  setUploadLoading(uploadBtn, false);
}

function setUploadLoading(btn, loading) {
  const btnText = btn.querySelector(".upload-btn-text");
  const btnSpinner = btn.querySelector(".upload-spinner");
  btn.disabled = loading;
  btnText.classList.toggle("hidden", loading);
  btnSpinner.classList.toggle("hidden", !loading);
}

// ─── Unlinked accounts resolution UI ───

function renderUnlinkedAccounts(container, accounts, server, uploadContext) {
  container.innerHTML = "";
  container.classList.remove("hidden");

  // Header
  const header = document.createElement("div");
  header.className = "unlinked-header";
  header.innerHTML = `
    <span class="unlinked-icon">&#9888;</span>
    <span>Some accounts need to be linked to players before uploading.</span>
  `;
  container.appendChild(header);

  // Track player selectors for each account
  const selectorMap = [];

  // Account rows
  const list = document.createElement("div");
  list.className = "unlinked-list";

  for (const account of accounts) {
    const row = document.createElement("div");
    row.className = "unlinked-row";

    const label = document.createElement("div");
    label.className = "unlinked-account-label";
    label.innerHTML = `<span class="unlinked-account-name">${escapeHtml(account.accountName)}</span>`;

    const selectorContainer = document.createElement("div");
    selectorContainer.className = "unlinked-player-selector";

    row.appendChild(label);
    row.appendChild(selectorContainer);
    list.appendChild(row);

    const selector = createPlayerSelector(selectorContainer, {
      placeholder: "Link to player\u2026",
    });

    selectorMap.push({ account, selector });
  }

  container.appendChild(list);

  // Action row
  const actions = document.createElement("div");
  actions.className = "unlinked-actions";

  const linkBtn = document.createElement("button");
  linkBtn.type = "button";
  linkBtn.className = "upload-btn link-retry-btn";
  linkBtn.innerHTML = `
    <span class="upload-btn-text">Link Accounts &amp; Retry Upload</span>
    <div class="spinner-small upload-spinner hidden"></div>
  `;

  const linkError = document.createElement("p");
  linkError.className = "upload-error hidden";

  actions.appendChild(linkBtn);
  container.appendChild(actions);
  container.appendChild(linkError);

  linkBtn.addEventListener("click", () => {
    handleLinkAndRetry(selectorMap, server, linkBtn, linkError, container, uploadContext);
  });
}

async function handleLinkAndRetry(selectorMap, server, linkBtn, linkError, container, uploadContext) {
  linkError.classList.add("hidden");

  // Validate all selectors have a player chosen
  for (const { account, selector } of selectorMap) {
    if (!selector.getValue()) {
      linkError.textContent = `Please select a player for ${account.accountName}.`;
      linkError.classList.remove("hidden");
      return;
    }
  }

  setUploadLoading(linkBtn, true);

  // Link each account sequentially
  for (const { account, selector } of selectorMap) {
    const player = selector.getValue();
    try {
      await invoke("reckon_link_account", {
        playerId: player.id,
        puuid: account.puuid,
        accountName: account.accountName || null,
      });
    } catch (err) {
      console.error(`Failed to link ${account.accountName}:`, err);
      linkError.textContent = `Failed to link ${account.accountName}: ${typeof err === "string" ? err : "Unknown error"}`;
      linkError.classList.remove("hidden");
      setUploadLoading(linkBtn, false);
      return;
    }
  }

  // All linked — hide the linking UI and retry upload
  setUploadLoading(linkBtn, false);
  container.classList.add("hidden");
  container.innerHTML = "";

  // Retry upload with the same parameters
  handleUploadMatch(uploadContext);
}

function playerRow(p, accountByPuuid = new Map()) {
  const highlight = p.isCurrentPlayer ? " player-self" : "";
  const account = accountByPuuid.get(p.puuid);
  const playerId = account?.playerId;
  const playerIdText = (playerId != null && playerId !== "") ? escapeHtml(String(playerId)) : "";
  const nameContent = `${escapeHtml(p.name)}<span class="player-id-label">${playerIdText}</span>`;
  return `
    <div class="player-row${highlight}">
      <span class="col-name">${nameContent}</span>
      <span class="col-agent">${escapeHtml(p.agent)}</span>
      <span class="col-k">${p.kills}</span>
      <span class="col-d">${p.deaths}</span>
      <span class="col-a">${p.assists}</span>
      <span class="col-score">${p.score}</span>
    </div>
  `;
}

// ─── Helpers ───
function formatDate(date) {
  const now = new Date();
  const diff = now - date;
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (hours < 1) return "Just now";
  if (hours < 24) return `${hours}h ago`;
  if (days < 7) return `${days}d ago`;

  return date.toLocaleDateString(undefined, { month: "short", day: "numeric" });
}

function escapeHtml(str) {
  const el = document.createElement("span");
  el.textContent = str;
  return el.innerHTML;
}

// ─── Reckon Bolt connection ───

function updateReckonUI() {
  const dot = reckonConnectBtn.querySelector(".connect-dot");
  if (reckonUser) {
    dot.className = "connect-dot connected";
    reckonConnectText.textContent = reckonUser.username;
    reckonConnectBtn.title = `Connected as ${reckonUser.username} (${reckonUser.organization}) — click to disconnect`;
    reckonConnectBtn.classList.add("is-connected");
  } else {
    dot.className = "connect-dot disconnected";
    reckonConnectText.textContent = "Connect";
    reckonConnectBtn.title = "Connect to Reckon Bolt";
    reckonConnectBtn.classList.remove("is-connected");
  }
}

function showLoginModal() {
  loginForm.reset();
  loginError.classList.add("hidden");
  loginOverlay.classList.remove("hidden");
  loginUsername.focus();
}

function hideLoginModal() {
  loginOverlay.classList.add("hidden");
}

async function handleLogin(e) {
  e.preventDefault();
  const username = loginUsername.value.trim();
  const password = loginPassword.value;
  if (!username || !password) return;

  // Show loading state
  loginSubmitBtn.disabled = true;
  loginSubmitText.classList.add("hidden");
  loginSpinner.classList.remove("hidden");
  loginError.classList.add("hidden");

  try {
    reckonUser = await invoke("reckon_login", { username, password });
    updateReckonUI();
    hideLoginModal();
    // Re-fetch data now that we have an auth token
    fetchReckonData();
  } catch (err) {
    console.error("Reckon login failed:", err);
    loginError.textContent = typeof err === "string" ? err : "Login failed. Check your credentials.";
    loginError.classList.remove("hidden");
  }

  loginSubmitBtn.disabled = false;
  loginSubmitText.classList.remove("hidden");
  loginSpinner.classList.add("hidden");
}

async function handleReckonDisconnect() {
  try {
    await invoke("reckon_logout");
  } catch (err) {
    console.error("Reckon logout error:", err);
  }
  reckonUser = null;
  updateReckonUI();
}

function onConnectBtnClick() {
  if (reckonUser) {
    handleReckonDisconnect();
  } else {
    showLoginModal();
  }
}

async function checkReckonStatus() {
  try {
    const user = await invoke("reckon_get_status");
    if (user) {
      reckonUser = user;
      updateReckonUI();
    }
  } catch (_) {
    // Not connected — that's fine
  }
}

// ─── Reckon Bolt data loading ───

async function fetchReckonData() {
  try {
    const data = await invoke("reckon_fetch_data");
    reckonPlayers = data.players || [];
    reckonTeams = data.teams || [];
    console.log(`Loaded ${reckonPlayers.length} players and ${reckonTeams.length} teams from Reckon Bolt`);
  } catch (err) {
    console.warn("Could not fetch Reckon data:", err);
  }
}

// Close modal on overlay background click
loginOverlay.addEventListener("click", (e) => {
  if (e.target === loginOverlay) hideLoginModal();
});

// Close modal with Escape
document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && !loginOverlay.classList.contains("hidden")) {
    hideLoginModal();
  }
});

// ─── TeamSelector component ───

/**
 * Creates an autocomplete team selector that reads from the global reckonTeams array.
 *
 * @param {HTMLElement} container  – the element to mount the selector into
 * @param {Object}      [opts]    – optional overrides
 * @param {string}      [opts.placeholder]       – input placeholder text
 * @param {Function}    [opts.onChange]           – callback(team | null) when selection changes
 * @param {Function}    [opts.getTeams]           – custom teams provider (default: () => reckonTeams)
 * @returns {{ getValue, setValue, clear, destroy }}
 */
function createTeamSelector(container, opts = {}) {
  const {
    placeholder = "Search for a team…",
    onChange = () => {},
    getTeams = () => reckonTeams,
  } = opts;

  // ── State ──
  let selectedTeam = null;
  let activeIndex = -1;
  let filteredTeams = [];
  let isOpen = false;

  // ── Build DOM ──
  const root = document.createElement("div");
  root.className = "team-selector";

  const input = document.createElement("input");
  input.type = "text";
  input.className = "team-selector-input";
  input.placeholder = placeholder;
  input.setAttribute("autocomplete", "off");
  input.setAttribute("spellcheck", "false");
  input.setAttribute("role", "combobox");
  input.setAttribute("aria-expanded", "false");
  input.setAttribute("aria-autocomplete", "list");

  const searchIcon = document.createElement("span");
  searchIcon.className = "team-selector-icon";
  searchIcon.textContent = "\u{1F50D}"; // 🔍
  searchIcon.setAttribute("aria-hidden", "true");

  const selectedLogo = document.createElement("img");
  selectedLogo.className = "team-selector-selected-logo";
  selectedLogo.alt = "";

  const clearBtn = document.createElement("button");
  clearBtn.className = "team-selector-clear";
  clearBtn.type = "button";
  clearBtn.textContent = "\u00D7"; // ×
  clearBtn.title = "Clear selection";
  clearBtn.setAttribute("aria-label", "Clear selection");

  const dropdown = document.createElement("div");
  dropdown.className = "team-selector-dropdown";
  dropdown.setAttribute("role", "listbox");

  root.appendChild(input);
  root.appendChild(searchIcon);
  root.appendChild(selectedLogo);
  root.appendChild(clearBtn);
  root.appendChild(dropdown);
  container.appendChild(root);

  // ── Helpers ──

  function escapeRegex(str) {
    return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  }

  function highlightMatch(text, query) {
    if (!query) return escapeHtml(text);
    const regex = new RegExp(`(${escapeRegex(query)})`, "gi");
    return escapeHtml(text).replace(regex, "<mark>$1</mark>");
  }

  function teamInitials(name) {
    return name
      .split(/\s+/)
      .map((w) => w[0])
      .join("")
      .slice(0, 2)
      .toUpperCase();
  }

  function teamLogoEl(team) {
    if (team.logo_url) {
      const img = document.createElement("img");
      img.className = "team-selector-logo";
      img.src = team.logo_url;
      img.alt = team.name;
      img.loading = "lazy";
      // Fallback to initials on error
      img.onerror = () => {
        const ph = document.createElement("span");
        ph.className = "team-selector-logo-placeholder";
        ph.style.backgroundColor = team.logo_primary || "#2a3a47";
        ph.textContent = teamInitials(team.name);
        img.replaceWith(ph);
      };
      return img;
    }
    const ph = document.createElement("span");
    ph.className = "team-selector-logo-placeholder";
    ph.style.backgroundColor = team.logo_primary || "#2a3a47";
    ph.textContent = teamInitials(team.name);
    return ph;
  }

  // ── Rendering ──

  function renderOptions(query) {
    dropdown.innerHTML = "";
    const teams = getTeams();
    const q = (query || "").trim().toLowerCase();

    filteredTeams = q
      ? teams.filter((t) => t.name.toLowerCase().includes(q))
      : [...teams];

    // Sort: exact-start matches first, then alphabetical
    if (q) {
      filteredTeams.sort((a, b) => {
        const aStarts = a.name.toLowerCase().startsWith(q) ? 0 : 1;
        const bStarts = b.name.toLowerCase().startsWith(q) ? 0 : 1;
        if (aStarts !== bStarts) return aStarts - bStarts;
        return a.name.localeCompare(b.name);
      });
    } else {
      filteredTeams.sort((a, b) => a.name.localeCompare(b.name));
    }

    if (filteredTeams.length === 0) {
      const empty = document.createElement("div");
      empty.className = "team-selector-empty";
      empty.textContent = q ? `No teams matching "${query.trim()}"` : "No teams available";
      dropdown.appendChild(empty);
      return;
    }

    filteredTeams.forEach((team, i) => {
      const option = document.createElement("div");
      option.className = "team-selector-option";
      option.setAttribute("role", "option");
      option.dataset.index = i;

      option.appendChild(teamLogoEl(team));

      const nameSpan = document.createElement("span");
      nameSpan.className = "team-selector-name";
      nameSpan.innerHTML = highlightMatch(team.name, query ? query.trim() : "");
      option.appendChild(nameSpan);

      if (team.current_league) {
        const league = document.createElement("span");
        league.className = "team-selector-league";
        league.textContent = team.current_league;
        option.appendChild(league);
      }

      option.addEventListener("mousedown", (e) => {
        e.preventDefault(); // prevent input blur
        selectTeam(team);
      });

      option.addEventListener("mouseenter", () => {
        setActiveIndex(i);
      });

      dropdown.appendChild(option);
    });
  }

  function setActiveIndex(index) {
    const options = dropdown.querySelectorAll(".team-selector-option");
    options.forEach((opt) => opt.classList.remove("active"));
    activeIndex = index;
    if (index >= 0 && index < options.length) {
      options[index].classList.add("active");
      options[index].scrollIntoView({ block: "nearest" });
    }
  }

  function openDropdown() {
    if (isOpen) return;
    isOpen = true;
    root.classList.add("open");
    input.setAttribute("aria-expanded", "true");
    renderOptions(input.value);
    activeIndex = -1;
  }

  function closeDropdown() {
    if (!isOpen) return;
    isOpen = false;
    root.classList.remove("open");
    input.setAttribute("aria-expanded", "false");
    activeIndex = -1;
  }

  function selectTeam(team) {
    selectedTeam = team;
    input.value = team.name;
    root.classList.add("has-value");

    if (team.logo_url) {
      selectedLogo.src = team.logo_url;
      selectedLogo.style.display = "";
    } else {
      selectedLogo.style.display = "none";
    }

    closeDropdown();
    onChange(team);
  }

  function clearSelection() {
    selectedTeam = null;
    input.value = "";
    root.classList.remove("has-value");
    selectedLogo.style.display = "none";
    onChange(null);
  }

  // ── Event handlers ──

  input.addEventListener("focus", () => {
    openDropdown();
    // If we have a selected team, select all text for easy re-search
    if (selectedTeam) {
      input.select();
    }
  });

  input.addEventListener("input", () => {
    // If user is typing, clear the current selection visually
    if (selectedTeam && input.value !== selectedTeam.name) {
      selectedTeam = null;
      root.classList.remove("has-value");
      selectedLogo.style.display = "none";
    }
    openDropdown();
    renderOptions(input.value);
    // Auto-highlight first option
    if (filteredTeams.length > 0) {
      setActiveIndex(0);
    } else {
      activeIndex = -1;
    }
  });

  input.addEventListener("keydown", (e) => {
    if (!isOpen) {
      if (e.key === "ArrowDown" || e.key === "ArrowUp") {
        e.preventDefault();
        openDropdown();
        renderOptions(input.value);
        if (filteredTeams.length > 0) setActiveIndex(0);
      }
      return;
    }

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        if (activeIndex < filteredTeams.length - 1) {
          setActiveIndex(activeIndex + 1);
        } else {
          setActiveIndex(0); // wrap around
        }
        break;

      case "ArrowUp":
        e.preventDefault();
        if (activeIndex > 0) {
          setActiveIndex(activeIndex - 1);
        } else {
          setActiveIndex(filteredTeams.length - 1); // wrap around
        }
        break;

      case "Enter":
        e.preventDefault();
        if (activeIndex >= 0 && activeIndex < filteredTeams.length) {
          selectTeam(filteredTeams[activeIndex]);
        }
        break;

      case "Escape":
        e.preventDefault();
        closeDropdown();
        // Restore previous selection text if any
        if (selectedTeam) {
          input.value = selectedTeam.name;
        }
        input.blur();
        break;

      case "Tab":
        closeDropdown();
        break;
    }
  });

  input.addEventListener("blur", () => {
    // Small delay to allow mousedown on options to fire first
    setTimeout(() => {
      closeDropdown();
      // Restore previous selection text if we didn't pick a new one
      if (selectedTeam) {
        input.value = selectedTeam.name;
        root.classList.add("has-value");
      } else if (input.value.trim() === "") {
        clearSelection();
      }
    }, 150);
  });

  clearBtn.addEventListener("click", (e) => {
    e.stopPropagation();
    clearSelection();
    input.focus();
  });

  // ── Public API ──

  return {
    /** @returns {Object|null} the currently selected team */
    getValue() {
      return selectedTeam;
    },

    /** Programmatically select a team by id */
    setValue(teamId) {
      const teams = getTeams();
      const team = teams.find((t) => t.id === teamId);
      if (team) {
        selectTeam(team);
      }
    },

    /** Clear the current selection */
    clear() {
      clearSelection();
    },

    /** Remove the component from the DOM and clean up */
    destroy() {
      root.remove();
    },

    /** Get the root DOM element (for external positioning / visibility control) */
    get element() {
      return root;
    },
  };
}

// ─── Player Selector Component ───

/**
 * Creates a PlayerSelector autocomplete component.
 *
 * @param {HTMLElement} container  – the DOM element to mount into
 * @param {object}      [opts]
 * @param {string}      [opts.placeholder]  – input placeholder text
 * @param {function}    [opts.onSelect]     – callback(player, team) when a player is picked
 * @returns {{ getValue, setValue, clear, refresh }}
 */
function createPlayerSelector(container, opts = {}) {
  const placeholder = opts.placeholder || "Search players\u2026";
  const onSelect = opts.onSelect || (() => {});

  // ── Build DOM ──
  container.classList.add("player-selector");

  const wrap = document.createElement("div");
  wrap.className = "player-selector-input-wrap";

  const icon = document.createElement("span");
  icon.className = "player-selector-icon";
  icon.innerHTML = "&#x1F50D;"; // magnifying glass

  const input = document.createElement("input");
  input.type = "text";
  input.className = "player-selector-input";
  input.placeholder = placeholder;
  input.autocomplete = "off";
  input.spellcheck = false;

  const clearBtn = document.createElement("button");
  clearBtn.type = "button";
  clearBtn.className = "player-selector-clear";
  clearBtn.innerHTML = "&times;";
  clearBtn.title = "Clear selection";

  const selectedChip = document.createElement("div");
  selectedChip.className = "player-selector-selected";

  wrap.appendChild(icon);
  wrap.appendChild(input);
  wrap.appendChild(selectedChip);
  wrap.appendChild(clearBtn);

  const dropdown = document.createElement("div");
  dropdown.className = "player-selector-dropdown";

  container.appendChild(wrap);
  container.appendChild(dropdown);

  // ── State ──
  let selectedPlayer = null;
  let activeIndex = -1;
  let visibleOptions = [];

  // ── Helpers ──

  /** Build a team lookup map: teamId → team object */
  function teamMap() {
    const map = {};
    for (const t of reckonTeams) {
      map[t.id] = t;
    }
    return map;
  }

  /** Group players by team, using the global reckonPlayers + reckonTeams */
  function groupedPlayers(filter) {
    const teams = teamMap();
    const query = (filter || "").trim().toLowerCase();

    // Build groups: { team, players[] }
    const groups = {};
    const noTeamKey = "__none__";

    for (const p of reckonPlayers) {
      const name = (p.id || "").toLowerCase();
      const role = (p.role || "").toLowerCase();
      const teamId = p.current_team || null;
      const team = teamId ? teams[teamId] : null;
      const teamName = team ? (team.name || "").toLowerCase() : "";

      // Filter: match player name, role, or team name
      if (query && !name.includes(query) && !role.includes(query) && !teamName.includes(query)) {
        continue;
      }

      const key = teamId || noTeamKey;
      if (!groups[key]) {
        groups[key] = { team: team || null, players: [] };
      }
      groups[key].players.push(p);
    }

    // Sort groups: teams with names first (alphabetical), then unknown
    return Object.values(groups).sort((a, b) => {
      if (!a.team && b.team) return 1;
      if (a.team && !b.team) return -1;
      if (a.team && b.team) return a.team.name.localeCompare(b.team.name);
      return 0;
    });
  }

  /** Highlight matching substring in text */
  function highlight(text, query) {
    if (!query) return escapeHtml(text);
    const escaped = escapeHtml(text);
    const idx = escaped.toLowerCase().indexOf(query.toLowerCase());
    if (idx === -1) return escaped;
    return (
      escaped.slice(0, idx) +
      "<mark>" +
      escaped.slice(idx, idx + query.length) +
      "</mark>" +
      escaped.slice(idx + query.length)
    );
  }

  /** Render the dropdown contents */
  function renderDropdown(filter) {
    dropdown.innerHTML = "";
    visibleOptions = [];
    activeIndex = -1;

    const groups = groupedPlayers(filter);

    if (groups.length === 0 || groups.every(g => g.players.length === 0)) {
      dropdown.innerHTML = '<div class="ps-no-results">No players found</div>';
      return;
    }

    const query = (filter || "").trim();

    for (const group of groups) {
      const section = document.createElement("div");
      section.className = "ps-team-group";

      // Team header
      const header = document.createElement("div");
      header.className = "ps-team-header";

      if (group.team) {
        if (group.team.logo_url) {
          const logo = document.createElement("img");
          logo.className = "ps-team-logo";
          logo.src = group.team.logo_url;
          logo.alt = group.team.name;
          logo.loading = "lazy";
          header.appendChild(logo);
        }
        const nameSpan = document.createElement("span");
        nameSpan.className = "ps-team-name";
        nameSpan.textContent = group.team.name;
        header.appendChild(nameSpan);
      } else {
        const nameSpan = document.createElement("span");
        nameSpan.className = "ps-team-name";
        nameSpan.textContent = "No Team";
        header.appendChild(nameSpan);
      }

      section.appendChild(header);

      // Player options
      for (const player of group.players) {
        const opt = document.createElement("div");
        opt.className = "ps-player-option";
        opt.dataset.playerId = player.id;

        const nameSpan = document.createElement("span");
        nameSpan.className = "ps-player-name";
        nameSpan.innerHTML = highlight(player.id, query);

        opt.appendChild(nameSpan);

        if (player.role) {
          const roleSpan = document.createElement("span");
          roleSpan.className = "ps-player-role";
          roleSpan.textContent = player.role;
          opt.appendChild(roleSpan);
        }

        // Click handler
        opt.addEventListener("mousedown", (e) => {
          e.preventDefault(); // prevent blur
          selectPlayer(player, group.team);
        });

        section.appendChild(opt);
        visibleOptions.push(opt);
      }

      dropdown.appendChild(section);
    }
  }

  /** Select a player */
  function selectPlayer(player, team) {
    selectedPlayer = { player, team };
    input.value = player.id;
    container.classList.add("has-value");
    container.classList.remove("editing");
    closeDropdown();
    renderSelectedChip(player, team);
    onSelect(player, team);
  }

  /** Render the selected chip overlay */
  function renderSelectedChip(player, team) {
    selectedChip.innerHTML = "";
    if (team && team.logo_url) {
      const logo = document.createElement("img");
      logo.className = "ps-selected-logo";
      logo.src = team.logo_url;
      logo.alt = team.name;
      selectedChip.appendChild(logo);
    }
    const name = document.createElement("span");
    name.className = "ps-selected-name";
    name.textContent = player.id;
    selectedChip.appendChild(name);
  }

  /** Open the dropdown */
  function openDropdown() {
    dropdown.classList.add("open");
  }

  /** Close the dropdown */
  function closeDropdown() {
    dropdown.classList.remove("open");
    activeIndex = -1;
    clearActive();
  }

  /** Set the active (keyboard-highlighted) option */
  function setActive(idx) {
    clearActive();
    if (idx < 0 || idx >= visibleOptions.length) return;
    activeIndex = idx;
    visibleOptions[idx].classList.add("active");
    visibleOptions[idx].scrollIntoView({ block: "nearest" });
  }

  function clearActive() {
    for (const opt of visibleOptions) {
      opt.classList.remove("active");
    }
  }

  // ── Events ──

  input.addEventListener("focus", () => {
    if (selectedPlayer) {
      container.classList.add("editing");
      input.value = "";
    }
    renderDropdown(input.value);
    openDropdown();
  });

  input.addEventListener("input", () => {
    if (selectedPlayer) {
      // User is typing over the selection – clear it
      selectedPlayer = null;
      container.classList.remove("has-value");
      container.classList.remove("editing");
      selectedChip.innerHTML = "";
    }
    renderDropdown(input.value);
    openDropdown();
  });

  input.addEventListener("blur", () => {
    // Delay to allow mousedown on option
    setTimeout(() => {
      closeDropdown();
      container.classList.remove("editing");
      // Restore display if selection exists
      if (selectedPlayer) {
        input.value = selectedPlayer.player.id;
        container.classList.add("has-value");
      }
    }, 150);
  });

  input.addEventListener("keydown", (e) => {
    if (!dropdown.classList.contains("open")) {
      if (e.key === "ArrowDown" || e.key === "ArrowUp") {
        renderDropdown(input.value);
        openDropdown();
        e.preventDefault();
        return;
      }
    }

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        setActive(Math.min(activeIndex + 1, visibleOptions.length - 1));
        break;

      case "ArrowUp":
        e.preventDefault();
        setActive(Math.max(activeIndex - 1, 0));
        break;

      case "Enter":
        e.preventDefault();
        if (activeIndex >= 0 && activeIndex < visibleOptions.length) {
          const playerId = visibleOptions[activeIndex].dataset.playerId;
          const player = reckonPlayers.find(p => p.id === playerId);
          if (player) {
            const teams = teamMap();
            const team = player.current_team ? teams[player.current_team] : null;
            selectPlayer(player, team);
          }
        }
        break;

      case "Escape":
        closeDropdown();
        input.blur();
        break;
    }
  });

  clearBtn.addEventListener("click", () => {
    clearSelection();
    input.focus();
  });

  // ── Public API ──

  function clearSelection() {
    selectedPlayer = null;
    input.value = "";
    container.classList.remove("has-value");
    container.classList.remove("editing");
    selectedChip.innerHTML = "";
    onSelect(null, null);
  }

  function getValue() {
    return selectedPlayer ? selectedPlayer.player : null;
  }

  function setValue(playerId) {
    const player = reckonPlayers.find(p => p.id === playerId);
    if (!player) return;
    const teams = teamMap();
    const team = player.current_team ? teams[player.current_team] : null;
    selectPlayer(player, team);
  }

  /** Re-render if the underlying data changes */
  function refresh() {
    if (selectedPlayer) {
      // Re-resolve the selected player in case data changed
      const player = reckonPlayers.find(p => p.id === selectedPlayer.player.id);
      if (player) {
        const teams = teamMap();
        const team = player.current_team ? teams[player.current_team] : null;
        selectedPlayer = { player, team };
        renderSelectedChip(player, team);
      } else {
        clearSelection();
      }
    }
  }

  return { getValue, setValue, clear: clearSelection, refresh };
}

// ─── Init ───
refreshBtn.addEventListener("click", loadMatches);
reckonConnectBtn.addEventListener("click", onConnectBtnClick);
loginCloseBtn.addEventListener("click", hideLoginModal);
loginForm.addEventListener("submit", handleLogin);

checkReckonStatus();
fetchReckonData();
loadMatches();
