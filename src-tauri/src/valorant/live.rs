use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::watch;

use super::{api, auth, lockfile};

// ─── Types ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GamePhase {
    Menus,
    PreGame,
    InGame,
    Replay,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveRosterPlayer {
    pub puuid: String,
    pub name: String,
    pub team_id: String,
    pub agent_id: String,
    pub agent_name: String,
    pub rank_tier: u32,
    pub rank_name: String,
    pub is_locked: bool,
    pub is_current_player: bool,
    pub account_level: u32,
    pub is_coach: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveGameState {
    pub phase: Option<GamePhase>,
    pub match_id: Option<String>,
    pub map_id: Option<String>,
    pub map_name: Option<String>,
    pub queue_id: Option<String>,
    pub queue_name: Option<String>,
    pub game_mode: Option<String>,
    pub is_ranked: bool,
    pub is_custom: bool,
    pub server_id: Option<String>,
    pub is_spectating: bool,
    pub spectate_score_ally: Option<i32>,
    pub spectate_score_enemy: Option<i32>,
    pub ally_team: Vec<LiveRosterPlayer>,
    pub enemy_team: Vec<LiveRosterPlayer>,
    pub pregame_time_remaining_ms: Option<u64>,
    pub party: Option<PartyInfo>,
}

// ─── Party types (serialized to frontend) ───────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyInfo {
    pub id: String,
    pub state: String,
    pub accessibility: String,
    pub members: Vec<PartyMember>,
    pub max_party_size: u32,
    pub queue_id: String,
    pub queue_entry_time: Option<String>,
    pub custom_game_name: Option<String>,
    pub map_name: Option<String>,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyMember {
    pub puuid: String,
    pub name: String,
    pub competitive_tier: u32,
    pub rank_name: String,
    pub account_level: u32,
    pub is_owner: bool,
    pub is_ready: bool,
    pub is_current_player: bool,
    pub team: Option<String>,
}

pub type SharedLiveState = Arc<Mutex<LiveGameState>>;

pub fn new_shared_state() -> SharedLiveState {
    Arc::new(Mutex::new(LiveGameState::default()))
}

// ─── Shared auth context for lobby actions ──────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct LobbyAuth {
    pub glz_base: String,
    pub puuid: String,
    pub party_id: String,
    pub access_token: String,
    pub entitlement_token: String,
    pub client_version: String,
}

pub type SharedLobbyAuth = Arc<Mutex<LobbyAuth>>;

pub fn new_shared_lobby_auth() -> SharedLobbyAuth {
    Arc::new(Mutex::new(LobbyAuth::default()))
}

// ─── Replay injection state ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayInjectionConfig {
    pub host_match_id: String,
    pub injection_path: String,
    pub active: bool,
    pub injected: bool,
    pub backup_path: Option<String>,
}

impl Default for ReplayInjectionConfig {
    fn default() -> Self {
        Self {
            host_match_id: String::new(),
            injection_path: String::new(),
            active: false,
            injected: false,
            backup_path: None,
        }
    }
}

pub type SharedInjectionState = Arc<Mutex<ReplayInjectionConfig>>;

pub fn new_shared_injection_state() -> SharedInjectionState {
    Arc::new(Mutex::new(ReplayInjectionConfig::default()))
}

/// Start monitoring: set host and injection files.
pub fn injection_start(state: &SharedInjectionState, host_match_id: &str, injection_path: &str) -> Result<(), String> {
    let demos_dir = super::super::find_valorant_demos_dir()
        .ok_or("Could not find the Valorant Demos folder")?;
    let host_file = demos_dir.join(format!("{}.vrf", host_match_id));
    if !host_file.exists() {
        return Err(format!("Host replay file not found: {}", host_file.display()));
    }
    let inj = std::path::Path::new(injection_path);
    if !inj.exists() {
        return Err(format!("Injection file not found: {}", injection_path));
    }
    let mut cfg = state.lock().map_err(|_| "Lock poisoned".to_string())?;
    cfg.host_match_id = host_match_id.to_string();
    cfg.injection_path = injection_path.to_string();
    cfg.active = true;
    cfg.injected = false;
    cfg.backup_path = None;
    eprintln!("[Injection] Armed: host={}, injection={}", host_match_id, injection_path);
    Ok(())
}

/// Stop monitoring and restore original file if needed.
pub fn injection_stop(state: &SharedInjectionState) -> Result<(), String> {
    let mut cfg = state.lock().map_err(|_| "Lock poisoned".to_string())?;
    if cfg.injected {
        restore_original(&cfg)?;
    }
    cfg.active = false;
    cfg.injected = false;
    cfg.backup_path = None;
    eprintln!("[Injection] Stopped");
    Ok(())
}

/// Perform the file swap: backup host, copy injection over it.
fn perform_injection(cfg: &mut ReplayInjectionConfig) -> Result<(), String> {
    let demos_dir = super::super::find_valorant_demos_dir()
        .ok_or("Could not find the Valorant Demos folder")?;
    let host_file = demos_dir.join(format!("{}.vrf", cfg.host_match_id));

    let backup_name = format!("{}_backup.vrf", cfg.host_match_id);
    let backup_path = demos_dir.join(&backup_name);

    std::fs::copy(&host_file, &backup_path)
        .map_err(|e| format!("Failed to create backup: {}", e))?;
    cfg.backup_path = Some(backup_path.to_string_lossy().into_owned());
    eprintln!("[Injection] Backup created: {}", backup_path.display());

    std::fs::copy(&cfg.injection_path, &host_file)
        .map_err(|e| format!("Failed to inject replay: {}", e))?;
    cfg.injected = true;
    eprintln!("[Injection] File swapped successfully");
    Ok(())
}

/// Restore the original host file from backup.
fn restore_original(cfg: &ReplayInjectionConfig) -> Result<(), String> {
    if let Some(ref backup) = cfg.backup_path {
        let demos_dir = super::super::find_valorant_demos_dir()
            .ok_or("Could not find the Valorant Demos folder")?;
        let host_file = demos_dir.join(format!("{}.vrf", cfg.host_match_id));
        let backup_path = std::path::Path::new(backup);

        if backup_path.exists() {
            std::fs::copy(backup_path, &host_file)
                .map_err(|e| format!("Failed to restore backup: {}", e))?;
            let _ = std::fs::remove_file(backup_path);
            eprintln!("[Injection] Original file restored");
        }
    }
    Ok(())
}

/// List .vrf replay files from the Valorant Demos directory.
pub fn list_local_replays() -> Vec<LocalReplayInfo> {
    let demos_dir = match super::super::find_valorant_demos_dir() {
        Some(d) => d,
        None => return vec![],
    };

    let mut replays = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&demos_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("vrf") {
                let filename = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
                let match_id = path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                let modified = entry.metadata().ok()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);

                replays.push(LocalReplayInfo {
                    filename,
                    match_id,
                    path: path.to_string_lossy().into_owned(),
                    size_bytes: size,
                    modified_ms: modified,
                });
            }
        }
    }

    replays.sort_by(|a, b| b.modified_ms.cmp(&a.modified_ms));
    replays
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalReplayInfo {
    pub filename: String,
    pub match_id: String,
    pub path: String,
    pub size_bytes: u64,
    pub modified_ms: u64,
}

// ─── Lobby action commands ──────────────────────────────────────────────────

pub async fn lobby_toggle_accessibility(
    auth: &SharedLobbyAuth,
    accessibility: &str,
) -> Result<String, String> {
    let ctx = auth.lock().map_err(|_| "Lock poisoned".to_string())?.clone();
    if ctx.party_id.is_empty() {
        return Err("No active party".to_string());
    }

    let client = api::build_http_client()?;
    let url = format!(
        "{}/parties/v1/parties/{}/accessibility",
        ctx.glz_base, ctx.party_id
    );

    glz_mutate(&client, reqwest::Method::POST, &url, &ctx, &serde_json::json!({ "accessibility": accessibility })).await
}

pub async fn lobby_get_recording_state(
    auth: &SharedLobbyAuth,
) -> Result<bool, String> {
    let ctx = auth.lock().map_err(|_| "Lock poisoned".to_string())?.clone();
    if ctx.party_id.is_empty() {
        return Err("No active party".to_string());
    }

    let client = api::build_http_client()?;
    let party_url = format!("{}/parties/v1/parties/{}", ctx.glz_base, ctx.party_id);
    let party: serde_json::Value = fetch_glz(
        &client, &party_url,
        &ctx.access_token, &ctx.entitlement_token, &ctx.client_version,
    ).await?;

    let is_recording = party
        .pointer("/CustomGameData/Settings/GameRules/TournamentMode")
        .and_then(|v| v.as_str())
        .map(|s| s == "true")
        .unwrap_or(false);

    Ok(is_recording)
}

pub async fn lobby_get_cheats_state(
    auth: &SharedLobbyAuth,
) -> Result<bool, String> {
    let ctx = auth.lock().map_err(|_| "Lock poisoned".to_string())?.clone();
    if ctx.party_id.is_empty() {
        return Err("No active party".to_string());
    }

    let client = api::build_http_client()?;
    let party_url = format!("{}/parties/v1/parties/{}", ctx.glz_base, ctx.party_id);
    let party: serde_json::Value = fetch_glz(
        &client, &party_url,
        &ctx.access_token, &ctx.entitlement_token, &ctx.client_version,
    ).await?;

    let cheats_on = party
        .pointer("/CustomGameData/Cheats/IsEnabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    Ok(cheats_on)
}

pub async fn lobby_set_cheats(
    auth: &SharedLobbyAuth,
    enabled: bool,
) -> Result<String, String> {
    let ctx = auth.lock().map_err(|_| "Lock poisoned".to_string())?.clone();
    if ctx.party_id.is_empty() {
        return Err("No active party".to_string());
    }

    let client = api::build_http_client()?;
    let url = format!(
        "{}/parties/v1/parties/{}/cheats",
        ctx.glz_base, ctx.party_id
    );

    glz_mutate(
        &client,
        reqwest::Method::PUT,
        &url,
        &ctx,
        &serde_json::json!({ "IsEnabled": enabled }),
    ).await
}

pub async fn lobby_set_recording(
    auth: &SharedLobbyAuth,
    enabled: bool,
) -> Result<bool, String> {
    let ctx = auth.lock().map_err(|_| "Lock poisoned".to_string())?.clone();
    if ctx.party_id.is_empty() {
        return Err("No active party".to_string());
    }

    let client = api::build_http_client()?;

    // GET current party to read existing custom game settings
    let party_url = format!("{}/parties/v1/parties/{}", ctx.glz_base, ctx.party_id);
    let party: serde_json::Value = fetch_glz(
        &client, &party_url,
        &ctx.access_token, &ctx.entitlement_token, &ctx.client_version,
    ).await?;

    let raw_settings = party
        .pointer("/CustomGameData/Settings")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));

    let tm_value = if enabled { "true" } else { "false" };

    let mut modified = raw_settings;
    if let Some(obj) = modified.as_object_mut() {
        let rules = obj.entry("GameRules")
            .or_insert_with(|| serde_json::json!({}));
        if let Some(rules_obj) = rules.as_object_mut() {
            rules_obj.insert("TournamentMode".to_string(), serde_json::json!(tm_value));
        }
    }

    let settings_url = format!(
        "{}/parties/v1/parties/{}/customgamesettings",
        ctx.glz_base, ctx.party_id
    );
    glz_mutate(&client, reqwest::Method::POST, &settings_url, &ctx, &modified).await?;

    eprintln!("[LiveAPI] Recording set to {}", enabled);
    Ok(enabled)
}

async fn glz_mutate(
    client: &reqwest::Client,
    method: reqwest::Method,
    url: &str,
    ctx: &LobbyAuth,
    body: &serde_json::Value,
) -> Result<String, String> {
    let headers = api::build_riot_headers(
        &ctx.access_token, &ctx.entitlement_token, &ctx.client_version,
    );

    let body_str = serde_json::to_string_pretty(body).unwrap_or_default();

    let mut req = client.request(method.clone(), url).json(body);
    for (name, value) in headers {
        req = req.header(name, value);
    }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("GLZ {} {} failed: {}\n\nRequest body:\n{}", method, url, e, body_str))?;

    let status = resp.status();
    let resp_text = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!(
            "GLZ {} {} returned {}:\n{}\n\nRequest body:\n{}",
            method, url, status, resp_text, body_str
        ));
    }

    Ok(resp_text)
}

// ─── Saved match types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedMatchEntry {
    pub match_id: String,
    pub map_name: Option<String>,
    pub queue_name: Option<String>,
    pub is_spectated: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SavedMatchIndex {
    pub matches: Vec<SavedMatchEntry>,
}

fn matches_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {}", e))?;
    let matches = dir.join("matches");
    std::fs::create_dir_all(&matches)
        .map_err(|e| format!("Cannot create matches dir: {}", e))?;
    Ok(matches)
}

fn load_match_index(app: &AppHandle) -> SavedMatchIndex {
    let dir = match matches_dir(app) {
        Ok(d) => d,
        Err(_) => return SavedMatchIndex::default(),
    };
    let path = dir.join("index.json");
    match std::fs::read_to_string(&path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => SavedMatchIndex::default(),
    }
}

fn save_match_index(app: &AppHandle, index: &SavedMatchIndex) -> Result<(), String> {
    let dir = matches_dir(app)?;
    let path = dir.join("index.json");
    let data = serde_json::to_string_pretty(index)
        .map_err(|e| format!("Serialize index: {}", e))?;
    std::fs::write(&path, data)
        .map_err(|e| format!("Write index: {}", e))
}

fn save_match_json(app: &AppHandle, match_id: &str, json: &str) -> Result<(), String> {
    let dir = matches_dir(app)?;
    let path = dir.join(format!("{}.json", match_id));
    std::fs::write(&path, json)
        .map_err(|e| format!("Write match JSON: {}", e))
}

pub fn get_saved_match_list(app: &AppHandle) -> SavedMatchIndex {
    load_match_index(app)
}

pub fn get_saved_match_json(app: &AppHandle, match_id: &str) -> Result<String, String> {
    let dir = matches_dir(app)?;
    let path = dir.join(format!("{}.json", match_id));
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Read match {}: {}", match_id, e))
}

pub fn get_saved_match_path(app: &AppHandle, match_id: &str) -> Result<PathBuf, String> {
    let dir = matches_dir(app)?;
    let path = dir.join(format!("{}.json", match_id));
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("Match file not found: {}", match_id))
    }
}

async fn auto_save_match(
    app: &AppHandle,
    client: &reqwest::Client,
    shard: &str,
    match_id: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
    live_state: &LiveGameState,
) {
    eprintln!("[LiveAPI] Match ended — fetching details for {}", match_id);

    let delays_secs: &[u64] = &[5, 10, 20, 30, 60];

    for (attempt, &delay) in delays_secs.iter().enumerate() {
        tokio::time::sleep(Duration::from_secs(delay)).await;

        match api::fetch_match_details_raw(
            client, shard, match_id,
            auth_token, entitlement_token, client_version,
        ).await {
            Ok(raw_json) => {
                if let Err(e) = save_match_json(app, match_id, &raw_json) {
                    eprintln!("[LiveAPI] Failed to save match JSON: {}", e);
                    return;
                }

                let entry = SavedMatchEntry {
                    match_id: match_id.to_string(),
                    map_name: live_state.map_name.clone(),
                    queue_name: live_state.queue_name.clone(),
                    is_spectated: live_state.is_spectating,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis() as u64)
                        .unwrap_or(0),
                };

                let mut index = load_match_index(app);
                if !index.matches.iter().any(|m| m.match_id == match_id) {
                    index.matches.insert(0, entry.clone());
                    if let Err(e) = save_match_index(app, &index) {
                        eprintln!("[LiveAPI] Failed to save index: {}", e);
                    }
                }

                eprintln!("[LiveAPI] Match saved on attempt {}: {} ({:?} on {:?})",
                    attempt + 1, match_id, live_state.queue_name, live_state.map_name);

                let _ = app.emit("match-saved", &entry);
                return;
            }
            Err(e) => {
                eprintln!("[LiveAPI] Attempt {}/{} failed for {}: {}",
                    attempt + 1, delays_secs.len(), match_id, e);
            }
        }
    }

    eprintln!("[LiveAPI] All {} attempts exhausted for match {}", delays_secs.len(), match_id);
}

// ─── Presence types ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PresencesResponse {
    presences: Vec<Presence>,
}

#[derive(Debug, Deserialize)]
struct Presence {
    puuid: String,
    private: Option<String>,
    product: Option<String>,
    game_name: Option<String>,
    game_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrivatePresence {
    #[serde(default)]
    match_presence_data: Option<MatchPresenceData>,
    #[serde(default)]
    party_presence_data: Option<PartyPresenceData>,
    #[serde(default)]
    player_presence_data: Option<PlayerPresenceData>,
    #[serde(default)]
    party_owner_match_score_ally_team: Option<i32>,
    #[serde(default)]
    party_owner_match_score_enemy_team: Option<i32>,
    #[serde(default)]
    party_id: Option<String>,
    #[serde(default)]
    party_size: Option<u32>,
    #[serde(default)]
    max_party_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MatchPresenceData {
    session_loop_state: Option<String>,
    match_map: Option<String>,
    queue_id: Option<String>,
    provisioning_flow: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartyPresenceData {
    custom_game_team: Option<String>,
    custom_game_name: Option<String>,
    party_state: Option<String>,
    party_id: Option<String>,
    party_accessibility: Option<String>,
    party_size: Option<u32>,
    max_party_size: Option<u32>,
    is_party_owner: Option<bool>,
    queue_entry_time: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerPresenceData {
    account_level: Option<u32>,
    competitive_tier: Option<u32>,
}

// ─── GLZ types: Pre-game ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GlzPlayerResponse {
    subject: String,
    #[serde(rename = "MatchID")]
    match_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PreGameMatch {
    #[serde(rename = "ID")]
    id: String,
    teams: Option<Vec<PreGameTeam>>,
    ally_team: Option<PreGameTeam>,
    #[serde(rename = "MapID")]
    map_id: String,
    mode: Option<String>,
    #[serde(rename = "QueueID")]
    queue_id: Option<String>,
    is_ranked: Option<bool>,
    #[serde(rename = "PhaseTimeRemainingNS")]
    phase_time_remaining_ns: Option<u64>,
    provisioning_flow_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PreGameTeam {
    #[serde(rename = "TeamID")]
    team_id: String,
    players: Vec<PreGamePlayer>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PreGamePlayer {
    subject: String,
    #[serde(rename = "CharacterID")]
    character_id: String,
    character_selection_state: Option<String>,
    competitive_tier: Option<u32>,
    player_identity: Option<PlayerIdentity>,
    is_captain: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlayerIdentity {
    subject: String,
    account_level: Option<u32>,
    incognito: Option<bool>,
}

// ─── GLZ types: Core-game ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CoreGameMatch {
    #[serde(rename = "MatchID")]
    match_id: String,
    #[serde(rename = "MapID")]
    map_id: String,
    #[serde(rename = "ModeID")]
    mode_id: Option<String>,
    provisioning_flow: Option<String>,
    #[serde(rename = "GamePodID")]
    game_pod_id: Option<String>,
    is_reconnectable: Option<bool>,
    players: Vec<CoreGamePlayer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CoreGamePlayer {
    subject: String,
    #[serde(rename = "TeamID")]
    team_id: String,
    #[serde(rename = "CharacterID")]
    character_id: String,
    player_identity: Option<PlayerIdentity>,
    seasonal_badge_info: Option<SeasonalBadge>,
    is_coach: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SeasonalBadge {
    rank: Option<u32>,
}

// ─── Rank tier mapping ──────────────────────────────────────────────────────

fn rank_name(tier: u32) -> &'static str {
    match tier {
        0 => "Unranked",
        1 | 2 => "Unused",
        3 => "Iron 1", 4 => "Iron 2", 5 => "Iron 3",
        6 => "Bronze 1", 7 => "Bronze 2", 8 => "Bronze 3",
        9 => "Silver 1", 10 => "Silver 2", 11 => "Silver 3",
        12 => "Gold 1", 13 => "Gold 2", 14 => "Gold 3",
        15 => "Platinum 1", 16 => "Platinum 2", 17 => "Platinum 3",
        18 => "Diamond 1", 19 => "Diamond 2", 20 => "Diamond 3",
        21 => "Ascendant 1", 22 => "Ascendant 2", 23 => "Ascendant 3",
        24 => "Immortal 1", 25 => "Immortal 2", 26 => "Immortal 3",
        27 => "Radiant",
        _ => "Unknown",
    }
}

fn queue_display_name(queue_id: &str, is_custom: bool) -> String {
    if is_custom {
        return "Custom".to_string();
    }
    match queue_id {
        "competitive" => "Competitive",
        "unrated" => "Unrated",
        "spikerush" => "Spike Rush",
        "deathmatch" => "Deathmatch",
        "ggteam" => "Escalation",
        "onefa" => "Replication",
        "swiftplay" => "Swiftplay",
        "premier" => "Premier",
        "newmap" => "New Map",
        "" => "Custom",
        other => other,
    }
    .to_string()
}

// ─── Presence fetching ──────────────────────────────────────────────────────

struct PresenceResult {
    phase: GamePhase,
    private: PrivatePresence,
    is_spectating: bool,
    party: Option<PartyInfo>,
}

async fn fetch_presence(
    client: &reqwest::Client,
    lockfile: &lockfile::LockfileData,
    my_puuid: &str,
) -> Result<Option<PresenceResult>, String> {
    let auth = api::local_auth_header(lockfile);
    let url = format!("https://127.0.0.1:{}/chat/v4/presences", lockfile.port);

    let resp = client
        .get(&url)
        .header("Authorization", &auth)
        .send()
        .await
        .map_err(|e| format!("Presence fetch failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Presence returned {}", resp.status()));
    }

    let data: PresencesResponse = resp
        .json()
        .await
        .map_err(|e| format!("Presence parse failed: {}", e))?;

    // Keep all Valorant presences for party member matching
    let val_presences: Vec<&Presence> = data
        .presences
        .iter()
        .filter(|p| p.product.as_deref() == Some("valorant"))
        .collect();

    let my_presence = val_presences.iter().find(|p| p.puuid == my_puuid);

    let presence = match my_presence {
        Some(p) => *p,
        None => return Ok(None),
    };

    let private_b64 = match &presence.private {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return Ok(None),
    };

    let decoded = STANDARD
        .decode(&private_b64)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    let private: PrivatePresence = serde_json::from_slice(&decoded)
        .map_err(|e| format!("Private presence parse failed: {}", e))?;

    let match_data = private.match_presence_data.as_ref();

    let raw_state = match_data
        .and_then(|m| m.session_loop_state.as_deref())
        .unwrap_or("");

    let is_spectating = private.party_presence_data.as_ref()
        .and_then(|p| p.custom_game_team.as_deref())
        .map(|t| t == "TeamSpectate")
        .unwrap_or(false);

    eprintln!("[LiveAPI] sessionLoopState='{}', spectating={}, map={:?}",
        raw_state, is_spectating,
        match_data.and_then(|m| m.match_map.as_deref()),
    );

    let phase = match raw_state {
        "PREGAME" => GamePhase::PreGame,
        "INGAME" => GamePhase::InGame,
        "REPLAY" => GamePhase::Replay,
        _ => GamePhase::Menus,
    };

    // Build party info from presences
    let party = build_party_from_presences(
        &private, my_puuid, presence, &val_presences,
    );

    Ok(Some(PresenceResult { phase, private, is_spectating, party }))
}

fn extract_map_display_name(map_path: &str) -> String {
    map_path.split('/').last().unwrap_or("Unknown").to_string()
}

fn build_party_from_presences(
    my_private: &PrivatePresence,
    my_puuid: &str,
    my_presence: &Presence,
    all_presences: &[&Presence],
) -> Option<PartyInfo> {
    let party_data = my_private.party_presence_data.as_ref()?;

    let my_party_id = my_private.party_id.as_deref()
        .or(party_data.party_id.as_deref())?;

    if my_party_id.is_empty() {
        return None;
    }

    let party_state = party_data.party_state.clone().unwrap_or_else(|| "DEFAULT".to_string());
    let accessibility = party_data.party_accessibility.clone().unwrap_or_else(|| "CLOSED".to_string());
    let max_party_size = party_data.max_party_size
        .or(my_private.max_party_size)
        .unwrap_or(5);
    let queue_entry_time = party_data.queue_entry_time.as_ref()
        .filter(|t| !t.starts_with("0001"))
        .cloned();

    let match_data = my_private.match_presence_data.as_ref();
    let match_queue_id = match_data
        .and_then(|m| m.queue_id.clone())
        .unwrap_or_default();

    let is_custom = party_state == "CUSTOM_GAME_SETUP";
    let custom_game_name = party_data.custom_game_name.as_ref()
        .filter(|n| !n.is_empty())
        .cloned();
    let map_name = match_data
        .and_then(|m| m.match_map.as_deref())
        .map(extract_map_display_name);

    // Build my own member entry
    let my_player_data = my_private.player_presence_data.as_ref();
    let my_tier = my_player_data.and_then(|p| p.competitive_tier).unwrap_or(0);
    let my_level = my_player_data.and_then(|p| p.account_level).unwrap_or(0);
    let my_name = match (&my_presence.game_name, &my_presence.game_tag) {
        (Some(gn), Some(tl)) if !gn.is_empty() => format!("{}#{}", gn, tl),
        (Some(gn), _) if !gn.is_empty() => gn.clone(),
        _ => String::new(),
    };

    let mut members = vec![PartyMember {
        puuid: my_puuid.to_string(),
        name: my_name,
        competitive_tier: my_tier,
        rank_name: rank_name(my_tier).to_string(),
        account_level: my_level,
        is_owner: party_data.is_party_owner.unwrap_or(false),
        is_ready: true,
        is_current_player: true,
        team: party_data.custom_game_team.clone(),
    }];

    for pres in all_presences {
        if pres.puuid == my_puuid {
            continue;
        }

        let priv_b64 = match &pres.private {
            Some(s) if !s.is_empty() => s,
            _ => continue,
        };

        let decoded = match STANDARD.decode(priv_b64) {
            Ok(d) => d,
            Err(_) => continue,
        };

        let other_private: PrivatePresence = match serde_json::from_slice(&decoded) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let other_party_id = other_private.party_id.as_deref()
            .or_else(|| other_private.party_presence_data.as_ref()
                .and_then(|pd| pd.party_id.as_deref()));

        if other_party_id != Some(my_party_id) {
            continue;
        }

        let other_party_data = other_private.party_presence_data.as_ref();
        let other_player = other_private.player_presence_data.as_ref();
        let tier = other_player.and_then(|p| p.competitive_tier).unwrap_or(0);
        let level = other_player.and_then(|p| p.account_level).unwrap_or(0);
        let is_owner = other_party_data
            .and_then(|pd| pd.is_party_owner)
            .unwrap_or(false);
        let team = other_party_data
            .and_then(|pd| pd.custom_game_team.clone());

        let name = match (&pres.game_name, &pres.game_tag) {
            (Some(gn), Some(tl)) if !gn.is_empty() => format!("{}#{}", gn, tl),
            (Some(gn), _) if !gn.is_empty() => gn.clone(),
            _ => String::new(),
        };

        members.push(PartyMember {
            puuid: pres.puuid.clone(),
            name,
            competitive_tier: tier,
            rank_name: rank_name(tier).to_string(),
            account_level: level,
            is_owner,
            is_ready: true,
            is_current_player: false,
            team,
        });
    }

    members.sort_by(|a, b| {
        b.is_owner.cmp(&a.is_owner).then(a.name.cmp(&b.name))
    });

    Some(PartyInfo {
        id: my_party_id.to_string(),
        state: party_state,
        accessibility,
        members,
        max_party_size,
        queue_id: match_queue_id,
        queue_entry_time,
        custom_game_name,
        map_name,
        is_custom,
    })
}

// ─── GLZ fetch functions ────────────────────────────────────────────────────

async fn fetch_glz<T: serde::de::DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> Result<T, String> {
    let headers = api::build_riot_headers(auth_token, entitlement_token, client_version);

    let mut req = client.get(url);
    for (name, value) in headers {
        req = req.header(name, value);
    }

    let resp = req
        .send()
        .await
        .map_err(|e| format!("GLZ GET {} failed: {}", url, e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("GLZ GET {} returned {}: {}", url, status, body));
    }

    resp.json::<T>()
        .await
        .map_err(|e| format!("GLZ GET {} parse failed: {}", url, e))
}

async fn fetch_pregame_state(
    client: &reqwest::Client,
    glz_base: &str,
    puuid: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
    agents: &HashMap<String, String>,
    maps: &HashMap<String, String>,
) -> Result<LiveGameState, String> {
    let player_url = format!("{}/pregame/v1/players/{}", glz_base, puuid);
    let player_resp: GlzPlayerResponse =
        fetch_glz(client, &player_url, auth_token, entitlement_token, client_version).await?;

    let match_url = format!("{}/pregame/v1/matches/{}", glz_base, player_resp.match_id);
    let match_data: PreGameMatch =
        fetch_glz(client, &match_url, auth_token, entitlement_token, client_version).await?;

    let is_custom = match_data.provisioning_flow_id.as_deref() == Some("CustomGame");
    let queue_id = match_data.queue_id.as_deref().unwrap_or("");
    let map_name = maps.get(&match_data.map_id).cloned().unwrap_or_else(|| {
        match_data.map_id.split('/').last().unwrap_or("Unknown").to_string()
    });

    let time_remaining_ms = match_data
        .phase_time_remaining_ns
        .map(|ns| ns / 1_000_000);

    let mut ally_team = Vec::new();
    let mut enemy_team = Vec::new();

    let all_teams: Vec<PreGameTeam> = match (&match_data.teams, &match_data.ally_team) {
        (Some(teams), _) => teams.to_vec(),
        (None, Some(ally)) => vec![ally.clone()],
        _ => Vec::new(),
    };

    let my_team_id = all_teams.iter()
        .find(|t| t.players.iter().any(|p| p.subject == puuid))
        .map(|t| t.team_id.clone())
        .unwrap_or_default();

    for team in &all_teams {
        let is_ally = team.team_id == my_team_id;
        for p in &team.players {
            let tier = p.competitive_tier.unwrap_or(0);
            let agent_name = agents
                .get(&p.character_id.to_lowercase())
                .cloned()
                .unwrap_or_default();
            let is_locked = p.character_selection_state.as_deref() == Some("locked");
            let account_level = p.player_identity.as_ref()
                .and_then(|pi| pi.account_level)
                .unwrap_or(0);

            let player = LiveRosterPlayer {
                puuid: p.subject.clone(),
                name: String::new(),
                team_id: team.team_id.clone(),
                agent_id: p.character_id.clone(),
                agent_name,
                rank_tier: tier,
                rank_name: rank_name(tier).to_string(),
                is_locked,
                is_current_player: p.subject == puuid,
                account_level,
                is_coach: false,
            };

            if is_ally {
                ally_team.push(player);
            } else {
                enemy_team.push(player);
            }
        }
    }

    Ok(LiveGameState {
        phase: Some(GamePhase::PreGame),
        match_id: Some(player_resp.match_id),
        map_id: Some(match_data.map_id.clone()),
        map_name: Some(map_name),
        queue_id: Some(queue_id.to_string()),
        queue_name: Some(queue_display_name(queue_id, is_custom)),
        game_mode: match_data.mode.clone(),
        is_ranked: match_data.is_ranked.unwrap_or(false),
        is_custom,
        server_id: None,
        is_spectating: false,
        spectate_score_ally: None,
        spectate_score_enemy: None,
        ally_team,
        enemy_team,
        pregame_time_remaining_ms: time_remaining_ms,
        party: None,
    })
}

async fn fetch_coregame_state(
    client: &reqwest::Client,
    glz_base: &str,
    puuid: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
    agents: &HashMap<String, String>,
    maps: &HashMap<String, String>,
) -> Result<LiveGameState, String> {
    let player_url = format!("{}/core-game/v1/players/{}", glz_base, puuid);
    let player_resp: GlzPlayerResponse =
        fetch_glz(client, &player_url, auth_token, entitlement_token, client_version).await?;

    let match_url = format!("{}/core-game/v1/matches/{}", glz_base, player_resp.match_id);
    let match_data: CoreGameMatch =
        fetch_glz(client, &match_url, auth_token, entitlement_token, client_version).await?;

    let is_custom = match_data.provisioning_flow.as_deref() == Some("CustomGame");
    let map_name = maps.get(&match_data.map_id).cloned().unwrap_or_else(|| {
        match_data.map_id.split('/').last().unwrap_or("Unknown").to_string()
    });

    let my_team_id = match_data.players.iter()
        .find(|p| p.subject == puuid)
        .map(|p| p.team_id.clone())
        .unwrap_or_default();

    // If the player's team is a standard game team, split relative to them.
    // Otherwise (spectator/coach with Neutral team), split by Blue/Red.
    let is_standard_team = my_team_id == "Blue" || my_team_id == "Red";

    let mut ally_team = Vec::new();
    let mut enemy_team = Vec::new();

    for p in &match_data.players {
        if p.is_coach.unwrap_or(false) {
            continue;
        }
        let tier = p.seasonal_badge_info.as_ref()
            .and_then(|b| b.rank)
            .unwrap_or(0);
        let agent_name = agents
            .get(&p.character_id.to_lowercase())
            .cloned()
            .unwrap_or_default();
        let account_level = p.player_identity.as_ref()
            .and_then(|pi| pi.account_level)
            .unwrap_or(0);

        let player = LiveRosterPlayer {
            puuid: p.subject.clone(),
            name: String::new(),
            team_id: p.team_id.clone(),
            agent_id: p.character_id.clone(),
            agent_name,
            rank_tier: tier,
            rank_name: rank_name(tier).to_string(),
            is_locked: true,
            is_current_player: p.subject == puuid,
            account_level,
            is_coach: p.is_coach.unwrap_or(false),
        };

        if is_standard_team {
            if p.team_id == my_team_id {
                ally_team.push(player);
            } else {
                enemy_team.push(player);
            }
        } else {
            // Spectator: split by Blue/Red
            if p.team_id == "Blue" {
                ally_team.push(player);
            } else {
                enemy_team.push(player);
            }
        }
    }

    Ok(LiveGameState {
        phase: Some(GamePhase::InGame),
        match_id: Some(match_data.match_id),
        map_id: Some(match_data.map_id.clone()),
        map_name: Some(map_name),
        queue_id: None,
        queue_name: None,
        game_mode: match_data.mode_id.clone(),
        is_ranked: false,
        is_custom,
        server_id: match_data.game_pod_id.clone(),
        is_spectating: false,
        spectate_score_ally: None,
        spectate_score_enemy: None,
        ally_team,
        enemy_team,
        pregame_time_remaining_ms: None,
        party: None,
    })
}

// ─── Name resolution via /name-service/v2/players ───────────────────────────

async fn resolve_names(
    client: &reqwest::Client,
    shard: &str,
    puuids: &[String],
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> HashMap<String, String> {
    let url = format!("https://pd.{}.a.pvp.net/name-service/v2/players", shard);
    let headers = api::build_riot_headers(auth_token, entitlement_token, client_version);

    let mut req = client.put(&url);
    for (name, value) in headers {
        req = req.header(name, value);
    }
    req = req.json(puuids);

    let resp = match req.send().await {
        Ok(r) if r.status().is_success() => r,
        _ => return HashMap::new(),
    };

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct NameEntry {
        subject: String,
        game_name: String,
        tag_line: String,
    }

    let entries: Vec<NameEntry> = match resp.json().await {
        Ok(e) => e,
        Err(_) => return HashMap::new(),
    };

    entries
        .into_iter()
        .map(|e| (e.subject, format!("{}#{}", e.game_name, e.tag_line)))
        .collect()
}

// ─── Spectator state from presence data ─────────────────────────────────────

fn build_spectator_state(
    private: &PrivatePresence,
    maps: &HashMap<String, String>,
    phase: GamePhase,
) -> Result<LiveGameState, String> {
    let match_data = private.match_presence_data.as_ref();
    let match_map = match_data.and_then(|m| m.match_map.clone());
    let queue_id = match_data.and_then(|m| m.queue_id.clone()).unwrap_or_default();
    let is_custom = match_data
        .and_then(|m| m.provisioning_flow.as_deref())
        .map(|p| p == "CustomGame")
        .unwrap_or(false);

    let map_name = match_map.as_ref().and_then(|m| {
        maps.get(m).cloned().or_else(|| {
            Some(m.split('/').last().unwrap_or("Unknown").to_string())
        })
    });

    Ok(LiveGameState {
        phase: Some(phase),
        match_id: None,
        map_id: match_map.clone(),
        map_name,
        queue_id: Some(queue_id.clone()),
        queue_name: Some(queue_display_name(&queue_id, is_custom)),
        game_mode: None,
        is_ranked: false,
        is_custom,
        server_id: None,
        is_spectating: true,
        spectate_score_ally: private.party_owner_match_score_ally_team,
        spectate_score_enemy: private.party_owner_match_score_enemy_team,
        ally_team: Vec::new(),
        enemy_team: Vec::new(),
        pregame_time_remaining_ms: None,
        party: None,
    })
}

async fn resolve_and_attach_names(
    client: &reqwest::Client,
    shard: &str,
    game_state: &mut LiveGameState,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) {
    let puuids: Vec<String> = game_state.ally_team.iter()
        .chain(game_state.enemy_team.iter())
        .map(|p| p.puuid.clone())
        .collect();

    if puuids.is_empty() {
        return;
    }

    let names = resolve_names(client, shard, &puuids, auth_token, entitlement_token, client_version).await;

    for p in game_state.ally_team.iter_mut().chain(game_state.enemy_team.iter_mut()) {
        if let Some(name) = names.get(&p.puuid) {
            p.name = name.clone();
        }
    }
}

// ─── Background poller ─────────────────────────────────────────────────────

pub fn start_live_poller(
    app_handle: AppHandle,
    state: SharedLiveState,
    lobby_auth: SharedLobbyAuth,
    injection_state: SharedInjectionState,
    stop_rx: watch::Receiver<bool>,
) {
    tauri::async_runtime::spawn(async move {
        eprintln!("[LiveAPI] Poller started");

        let mut prev_phase: Option<GamePhase> = None;
        let mut last_ingame_state: Option<LiveGameState> = None;
        let mut agents: HashMap<String, String> = HashMap::new();
        let mut maps: HashMap<String, String> = HashMap::new();
        let mut static_data_loaded = false;

        loop {
            if *stop_rx.borrow() {
                break;
            }

            let client = match api::build_http_client() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[LiveAPI] Client build failed: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            let lockfile = match lockfile::read_lockfile() {
                Ok(lf) => lf,
                Err(_) => {
                    // Valorant not running — clear state and wait
                    {
                        let mut s = state.lock().unwrap();
                        if s.phase.is_some() {
                            *s = LiveGameState::default();
                            let _ = app_handle.emit("live-game-state", LiveGameState::default());
                        }
                    }
                    prev_phase = None;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            let entitlements = match auth::get_entitlements(&client, &lockfile).await {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("[LiveAPI] Entitlements failed: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            let (region, shard, client_version) =
                match api::get_session_info(&client, &lockfile).await {
                    Ok(info) => info,
                    Err(e) => {
                        eprintln!("[LiveAPI] Session info failed: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

            if !static_data_loaded {
                if let Ok(m) = api::fetch_maps(&client).await {
                    maps = m;
                }
                if let Ok(a) = api::fetch_agents(&client).await {
                    agents = a;
                }
                static_data_loaded = true;
                eprintln!("[LiveAPI] Static data loaded ({} maps, {} agents)", maps.len(), agents.len());
            }

            let glz_base = api::glz_base_url(&region, &shard);

            // Inner poll loop — reuse auth until it fails
            loop {
                if *stop_rx.borrow() {
                    break;
                }

                let presence = match fetch_presence(&client, &lockfile, &entitlements.subject).await
                {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("[LiveAPI] Presence error: {}", e);
                        break; // re-auth on next outer loop
                    }
                };

                let pres = match presence {
                    Some(p) => p,
                    None => {
                        tokio::time::sleep(Duration::from_secs(3)).await;
                        continue;
                    }
                };

                let phase = pres.phase;
                let is_spectating = pres.is_spectating;

                let old_phase = prev_phase;
                let phase_changed = old_phase != Some(phase);
                if phase_changed {
                    eprintln!("[LiveAPI] Phase: {:?} -> {:?} (spectating={})", old_phase, phase, is_spectating);
                }
                prev_phase = Some(phase);

                match phase {
                    GamePhase::Menus => {
                        // Auto-restore injected replay when leaving REPLAY state
                        if old_phase == Some(GamePhase::Replay) {
                            let should_restore = {
                                let inj = injection_state.lock().unwrap();
                                inj.injected
                            };
                            if should_restore {
                                let result = {
                                    let mut inj = injection_state.lock().unwrap();
                                    let r = restore_original(&inj);
                                    inj.injected = false;
                                    r
                                };
                                match result {
                                    Ok(()) => {
                                        let _ = app_handle.emit("replay-injection", serde_json::json!({
                                            "status": "restored"
                                        }));
                                    }
                                    Err(e) => {
                                        eprintln!("[Injection] Restore failed: {}", e);
                                        let _ = app_handle.emit("replay-injection", serde_json::json!({
                                            "status": "restore-error",
                                            "message": e,
                                        }));
                                    }
                                }
                            }
                        }

                        // Check for a pending match to save regardless of what old_phase was,
                        // since lockfile errors can reset prev_phase to None and mask the
                        // InGame -> Menus transition.
                        if let Some(ref last_state) = last_ingame_state {
                            if let Some(ref mid) = last_state.match_id {
                                eprintln!("[LiveAPI] Match ended (old_phase={:?}), saving {}", old_phase, mid);
                                let app_h = app_handle.clone();
                                let client_c = client.clone();
                                let shard_c = shard.clone();
                                let mid_c = mid.clone();
                                let at = entitlements.access_token.clone();
                                let et = entitlements.token.clone();
                                let cv = client_version.clone();
                                let ls = last_state.clone();
                                tauri::async_runtime::spawn(async move {
                                    auto_save_match(
                                        &app_h, &client_c, &shard_c, &mid_c,
                                        &at, &et, &cv, &ls,
                                    ).await;
                                });
                            } else {
                                eprintln!("[LiveAPI] Match ended but no match_id in last state");
                            }
                            last_ingame_state = None;
                        }

                        if phase_changed {
                            if let Some(ref party) = pres.party {
                                eprintln!("[LiveAPI] Party: id={}, state={}, members={}, queue={}",
                                    party.id, party.state, party.members.len(), party.queue_id);
                            }
                        }

                        // Keep lobby auth context up-to-date for action commands
                        {
                            let mut la = lobby_auth.lock().unwrap();
                            la.glz_base = glz_base.clone();
                            la.puuid = entitlements.subject.clone();
                            la.access_token = entitlements.access_token.clone();
                            la.entitlement_token = entitlements.token.clone();
                            la.client_version = client_version.clone();
                            la.party_id = pres.party.as_ref()
                                .map(|p| p.id.clone())
                                .unwrap_or_default();
                        }

                        let menus_state = LiveGameState {
                            phase: Some(GamePhase::Menus),
                            party: pres.party.clone(),
                            ..Default::default()
                        };
                        {
                            let mut s = state.lock().unwrap();
                            *s = menus_state.clone();
                        }
                        let _ = app_handle.emit("live-game-state", &menus_state);

                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                    GamePhase::PreGame => {
                        let result = if is_spectating {
                            // Spectators can't query pregame player endpoint; build from presence
                            build_spectator_state(&pres.private, &maps, GamePhase::PreGame)
                        } else {
                            match fetch_pregame_state(
                                &client, &glz_base, &entitlements.subject,
                                &entitlements.access_token, &entitlements.token,
                                &client_version, &agents, &maps,
                            ).await {
                                Ok(mut gs) => {
                                    resolve_and_attach_names(
                                        &client, &shard, &mut gs,
                                        &entitlements.access_token, &entitlements.token,
                                        &client_version,
                                    ).await;
                                    Ok(gs)
                                }
                                Err(e) => Err(e),
                            }
                        };

                        match result {
                            Ok(game_state) => {
                                {
                                    let mut s = state.lock().unwrap();
                                    *s = game_state.clone();
                                }
                                let _ = app_handle.emit("live-game-state", &game_state);
                            }
                            Err(e) => eprintln!("[LiveAPI] PreGame fetch failed: {}", e),
                        }
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                    GamePhase::InGame => {
                        let result = if is_spectating {
                            match fetch_coregame_state(
                                &client, &glz_base, &entitlements.subject,
                                &entitlements.access_token, &entitlements.token,
                                &client_version, &agents, &maps,
                            ).await {
                                Ok(mut gs) => {
                                    eprintln!("[LiveAPI] Spectator: GLZ coregame OK ({} ally, {} enemy)",
                                        gs.ally_team.len(), gs.enemy_team.len());
                                    gs.is_spectating = true;
                                    gs.spectate_score_ally = pres.private.party_owner_match_score_ally_team;
                                    gs.spectate_score_enemy = pres.private.party_owner_match_score_enemy_team;
                                    resolve_and_attach_names(
                                        &client, &shard, &mut gs,
                                        &entitlements.access_token, &entitlements.token,
                                        &client_version,
                                    ).await;
                                    Ok(gs)
                                }
                                Err(e) => {
                                    eprintln!("[LiveAPI] Spectator: GLZ failed ({}), using presence fallback", e);
                                    build_spectator_state(&pres.private, &maps, GamePhase::InGame)
                                }
                            }
                        } else {
                            match fetch_coregame_state(
                                &client, &glz_base, &entitlements.subject,
                                &entitlements.access_token, &entitlements.token,
                                &client_version, &agents, &maps,
                            ).await {
                                Ok(mut gs) => {
                                    resolve_and_attach_names(
                                        &client, &shard, &mut gs,
                                        &entitlements.access_token, &entitlements.token,
                                        &client_version,
                                    ).await;
                                    Ok(gs)
                                }
                                Err(e) => Err(e),
                            }
                        };

                        match result {
                            Ok(game_state) => {
                                eprintln!("[LiveAPI] Emitting InGame state: map={:?}, spectating={}, score={:?}:{:?}, ally={}, enemy={}",
                                    game_state.map_name, game_state.is_spectating,
                                    game_state.spectate_score_ally, game_state.spectate_score_enemy,
                                    game_state.ally_team.len(), game_state.enemy_team.len());
                                last_ingame_state = Some(game_state.clone());
                                {
                                    let mut s = state.lock().unwrap();
                                    *s = game_state.clone();
                                }
                                let _ = app_handle.emit("live-game-state", &game_state);
                            }
                            Err(e) => eprintln!("[LiveAPI] CoreGame fetch failed: {}", e),
                        }
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                    GamePhase::Replay => {
                        if phase_changed {
                            eprintln!("[LiveAPI] Entered REPLAY state");

                            // Check if injection is armed
                            let should_inject = {
                                let inj = injection_state.lock().unwrap();
                                inj.active && !inj.injected
                            };
                            if should_inject {
                                let result = {
                                    let mut inj = injection_state.lock().unwrap();
                                    perform_injection(&mut inj)
                                };
                                match result {
                                    Ok(()) => {
                                        let _ = app_handle.emit("replay-injection", serde_json::json!({
                                            "status": "injected"
                                        }));
                                    }
                                    Err(e) => {
                                        eprintln!("[Injection] Failed: {}", e);
                                        let _ = app_handle.emit("replay-injection", serde_json::json!({
                                            "status": "error",
                                            "message": e,
                                        }));
                                    }
                                }
                            }

                            let replay_state = LiveGameState {
                                phase: Some(GamePhase::Replay),
                                ..Default::default()
                            };
                            {
                                let mut s = state.lock().unwrap();
                                *s = replay_state.clone();
                            }
                            let _ = app_handle.emit("live-game-state", &replay_state);
                        }
                        tokio::time::sleep(Duration::from_secs(3)).await;
                    }
                }
            }
        }

        // Auto-restore on poller shutdown if injection was active
        {
            let mut inj = injection_state.lock().unwrap();
            if inj.injected {
                let _ = restore_original(&inj);
                inj.injected = false;
            }
        }

        eprintln!("[LiveAPI] Poller stopped");
    });
}
