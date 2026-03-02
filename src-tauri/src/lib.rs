mod valorant;

use std::sync::Mutex;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use valorant::{api, auth, lockfile, types::{MatchDetailView, MatchSummary}};
use reckon_api::apis::configuration::BASE_URL_DEV;

// ─── Reckon Bolt connection state ───────────────────────────────────────────

/// Persistent app state holding the Reckon Bolt auth token, user info,
/// and cached reference data (players & teams).
#[derive(Default)]
struct ReckonState {
    token: Option<String>,
    user: Option<ReckonUser>,
    players: Vec<serde_json::Value>,
    teams: Vec<serde_json::Value>,
}

/// User info returned to the frontend after a successful login.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReckonUser {
    username: String,
    email: String,
    team: String,
    organization: String,
}

/// The login response from the Reckon Bolt API.
/// We define our own struct instead of using the generated `models::Login`
/// because the server (correctly) omits `password` from the response, but
/// the generated model marks it as required which breaks deserialization.
#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    username: String,
    email: String,
    team: String,
    organization: String,
}

/// What gets persisted to disk between app launches.
#[derive(Serialize, Deserialize)]
struct PersistedSession {
    token: String,
    user: ReckonUser,
}

// ─── Session persistence helpers ────────────────────────────────────────────

const SESSION_FILE: &str = "reckon_session.json";

/// Resolve the path to the session file inside the app data directory.
fn session_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {}", e))?;
    Ok(dir.join(SESSION_FILE))
}

/// Save the current session (token + user) to disk.
fn save_session(app: &tauri::AppHandle, token: &str, user: &ReckonUser) -> Result<(), String> {
    let path = session_path(app)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create app data dir: {}", e))?;
    }
    let session = PersistedSession {
        token: token.to_owned(),
        user: user.clone(),
    };
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Cannot serialize session: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Cannot write session file: {}", e))?;
    Ok(())
}

/// Delete the persisted session file.
fn delete_session(app: &tauri::AppHandle) {
    if let Ok(path) = session_path(app) {
        let _ = std::fs::remove_file(path);
    }
}

/// Try to load a previously saved session from disk.
fn load_session(app: &tauri::AppHandle) -> Option<PersistedSession> {
    let path = session_path(app).ok()?;
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

// ─── Reckon Bolt commands ───────────────────────────────────────────────────

/// Log in to the Reckon Bolt backend, store the auth token, and persist to disk.
#[tauri::command]
async fn reckon_login(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<ReckonState>>,
    username: String,
    password: String,
) -> Result<ReckonUser, String> {
    let client = reqwest::Client::new();

    let params = [
        ("username", username.as_str()),
        ("password", password.as_str()),
        ("token", ""),
    ];

    let resp = client
        .post(format!("{}/Login", BASE_URL_DEV))
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Login request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Login failed ({}): {}", status, body));
    }

    let result: LoginResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse login response: {}", e))?;

    let user = ReckonUser {
        username: result.username,
        email: result.email,
        team: result.team,
        organization: result.organization,
    };

    // Persist session to disk so the user stays logged in across app restarts
    save_session(&app, &result.token, &user)?;

    let mut guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    guard.token = Some(result.token);
    guard.user = Some(user.clone());

    Ok(user)
}

/// Clear stored token and user info (disconnect), and remove from disk.
#[tauri::command]
fn reckon_logout(app: tauri::AppHandle, state: tauri::State<'_, Mutex<ReckonState>>) -> Result<(), String> {
    delete_session(&app);
    let mut guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    guard.token = None;
    guard.user = None;
    Ok(())
}

/// Return the currently logged-in user, or null if not connected.
#[tauri::command]
fn reckon_get_status(state: tauri::State<'_, Mutex<ReckonState>>) -> Option<ReckonUser> {
    let guard = state.lock().ok()?;
    guard.user.clone()
}

// ─── Reckon Bolt data fetching ──────────────────────────────────────────────

/// Helper: build a GET request with optional token auth.
fn authed_get(client: &reqwest::Client, url: &str, token: &Option<String>) -> reqwest::RequestBuilder {
    let mut req = client.get(url);
    if let Some(ref t) = token {
        req = req.header("Authorization", format!("Token {}", t));
    }
    req
}

/// Fetch all players and teams from the Reckon Bolt API, cache them in state,
/// and return both lists. Called on app launch and after login.
#[tauri::command]
async fn reckon_fetch_data(
    state: tauri::State<'_, Mutex<ReckonState>>,
) -> Result<serde_json::Value, String> {
    let token = {
        let guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
        guard.token.clone()
    };

    let client = reqwest::Client::new();

    let players: Vec<serde_json::Value> = authed_get(&client, &format!("{}/Player/list", BASE_URL_DEV), &token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch players: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse players: {}", e))?;

    let teams: Vec<serde_json::Value> = authed_get(&client, &format!("{}/Team/list", BASE_URL_DEV), &token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch teams: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse teams: {}", e))?;

    // Cache in state
    let mut guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
    guard.players = players.clone();
    guard.teams = teams.clone();

    Ok(serde_json::json!({
        "players": players,
        "teams": teams,
    }))
}

/// Return cached players list.
#[tauri::command]
fn reckon_get_players(state: tauri::State<'_, Mutex<ReckonState>>) -> Vec<serde_json::Value> {
    state.lock().map(|g| g.players.clone()).unwrap_or_default()
}

/// Return cached teams list.
#[tauri::command]
fn reckon_get_teams(state: tauri::State<'_, Mutex<ReckonState>>) -> Vec<serde_json::Value> {
    state.lock().map(|g| g.teams.clone()).unwrap_or_default()
}

// ─── Reckon Bolt upload ─────────────────────────────────────────────────────

/// Upload a match to Reckon Bolt via the ScrimsData upload endpoint.
///
/// Returns a JSON value:
///   - On success: `{ "success": true, "gameId": "…", … }`
///   - On unlinked accounts: `{ "error": "…", "unlinkedAccounts": [{ "puuid", "accountName" }], "server": "…" }`
///   - On other errors: returns Err(String)
#[tauri::command]
async fn reckon_upload_match(
    state: tauri::State<'_, Mutex<ReckonState>>,
    team1: String,
    team2: String,
    match_id: String,
) -> Result<serde_json::Value, String> {
    // 1. Get auth token from state
    let token = {
        let guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
        guard.token.clone().ok_or("Not connected to Reckon Bolt")?
    };

    // 2. Fetch the full match-details response as raw JSON (preserves roundResults, kills, etc.)
    let lockfile = valorant::lockfile::read_lockfile()?;
    let riot_client = valorant::api::build_http_client()?;
    let entitlements = valorant::auth::get_entitlements(&riot_client, &lockfile).await?;
    let (_region, shard, client_version) = valorant::api::get_session_info(&riot_client, &lockfile).await?;

    let match_json = valorant::api::fetch_match_details_raw(
        &riot_client,
        &shard,
        &match_id,
        &entitlements.access_token,
        &entitlements.token,
        &client_version,
    )
    .await?;

    // 4. Build the Reckon API configuration with token auth
    let config = reckon_api::apis::configuration::Configuration::dev().with_token(&token);

    // 5. Build the upload request
    let upload_req = reckon_api::models::UploadScrimGameRequest::new(
        team1,
        team2,
        match_json,
    );

    // 6. Call the upload endpoint, using the Riot match ID as the resource id
    use reckon_api::apis::{Error as ApiError, scrims_data_api};

    match scrims_data_api::upload_scrim_game(&config, &match_id, upload_req).await {
        Ok(success) => {
            Ok(serde_json::to_value(success)
                .unwrap_or(serde_json::json!({"success": true})))
        }
        Err(ApiError::ResponseError(resp)) => {
            // Try to extract the typed 400 error with unlinked accounts
            if let Some(scrims_data_api::UploadScrimGameError::Status400(err_body)) = resp.entity {
                if let Some(ref accounts) = err_body.unlinked_accounts {
                    if !accounts.is_empty() {
                        // Return structured data so frontend can show the linking UI.
                        // Include the shard so the link_account command knows the server.
                        let accounts_json: Vec<serde_json::Value> = accounts.iter().map(|a| {
                            serde_json::json!({
                                "puuid": a.puuid,
                                "accountName": a.account_name,
                            })
                        }).collect();

                        return Ok(serde_json::json!({
                            "error": err_body.error,
                            "unlinkedAccounts": accounts_json,
                            "server": shard,
                        }));
                    }
                }
                // 400 error but no unlinked accounts — regular error
                Err(err_body.error)
            } else {
                Err(format!("Upload failed ({}): {}", resp.status, resp.content))
            }
        }
        Err(e) => Err(format!("Upload failed: {}", e)),
    }
}

/// Link a Riot account to an existing Reckon Bolt player via
/// `POST /Player/item/{player_id}/add_account`.
#[tauri::command]
async fn reckon_link_account(
    state: tauri::State<'_, Mutex<ReckonState>>,
    player_id: String,
    puuid: String,
    account_name: Option<String>,
) -> Result<serde_json::Value, String> {
    let token = {
        let guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
        guard.token.clone().ok_or("Not connected to Reckon Bolt")?
    };

    let config = reckon_api::apis::configuration::Configuration::dev().with_token(&token);

    let mut body = reckon_api::models::AddAccount::new(puuid);
    body.account_name = account_name;

    match reckon_api::apis::add_account_api::add_player_account(
        &config,
        &player_id,
        body,
    )
    .await
    {
        Ok(result) => Ok(serde_json::json!({ "accounts": result })),
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("invalid type: map") || err_str.contains("expected a sequence") {
                // Backend returned success but response is now an object, not array; link succeeded
                return Ok(serde_json::json!({ "accounts": [] }));
            }
            Err(format!("Failed to link account: {}", e))
        }
    }
}

/// Player ref for creating missing SoloQ accounts (puuid + display name).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerRef {
    puuid: String,
    account_name: String,
}

/// Fetch SoloQAccount entries for the given PUUIDs (e.g. all players in a match).
/// Uses additional_filters[puuid__in] on the SoloQAccounts list endpoint.
/// If `server` and `players` are provided, any PUUID not returned by the list is created
/// via POST SoloQAccounts/item/{puuid} with account_name and server from `players`.
#[tauri::command]
async fn reckon_get_soloq_accounts(
    state: tauri::State<'_, Mutex<ReckonState>>,
    puuids: Vec<String>,
    server: Option<String>,
    players: Option<Vec<PlayerRef>>,
) -> Result<Vec<serde_json::Value>, String> {
    let token = {
        let guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
        guard.token.clone().ok_or("Not connected to Reckon Bolt")?
    };

    if puuids.is_empty() {
        return Ok(vec![]);
    }

    let config = reckon_api::apis::configuration::Configuration::dev().with_token(&token);
    let additional_filters = serde_json::json!({ "puuid__in": puuids });

    use reckon_api::apis::solo_q_accounts_api;
    use reckon_api::models::SoloQAccounts;

    let mut accounts = solo_q_accounts_api::solo_q_accounts_list(
        &config,
        Some(additional_filters),
        None,
        None,
        None,
        None,
    )
    .await
    .map_err(|e| format!("Failed to fetch SoloQ accounts: {}", e))?;

    // Create any missing accounts when server and players are provided
    if let (Some(_server), Some(players)) = (server, players) {
        let returned_puuids: std::collections::HashSet<String> =
            accounts.iter().map(|a| a.puuid.clone()).collect();
        let name_by_puuid: std::collections::HashMap<String, String> = players
            .iter()
            .map(|p| (p.puuid.clone(), p.account_name.clone()))
            .collect();

        for puuid in &puuids {
            if returned_puuids.contains(puuid) {
                continue;
            }
            let account_name = name_by_puuid
                .get(puuid)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());

            // Generated client: POST /SoloQAccounts/item/new with puuid + account_name in body
            let body = SoloQAccounts::new(0, puuid.clone(), account_name);

            match solo_q_accounts_api::solo_q_accounts_create(&config, puuid, body).await {
                Ok(created) => accounts.push(created),
                Err(e) => {
                    use reckon_api::apis::{Error as ApiError, ResponseContent};
                    let (msg, status, api_response) = match &e {
                        ApiError::ResponseError(ResponseContent { status, content, .. }) => (
                            format!("Failed to create SoloQ account for {}", puuid),
                            status.as_u16(),
                            content.clone(),
                        ),
                        _ => (
                            format!("Failed to create SoloQ account for {}: {}", puuid, e),
                            0u16,
                            String::new(),
                        ),
                    };
                    let err_json = serde_json::json!({
                        "soloqCreateError": true,
                        "message": msg,
                        "status": status,
                        "apiResponse": api_response,
                    });
                    return Err(err_json.to_string());
                }
            }
        }
    }

    let json: Vec<serde_json::Value> = accounts
        .into_iter()
        .map(|a| {
            serde_json::json!({
                "id": a.id,
                "puuid": a.puuid,
                "accountName": a.account_name,
                "rankTier": a.rank_tier,
                "playerId": a.player_id,
                "server": a.server,
            })
        })
        .collect();

    Ok(json)
}

// ─── Valorant commands ──────────────────────────────────────────────────────

/// Quick check: can we read the lockfile? (= is Valorant running?)
#[tauri::command]
fn check_valorant_running() -> bool {
    lockfile::read_lockfile().is_ok()
}

/// Full flow: read lockfile → get tokens → get session info → fetch match history + details.
#[tauri::command]
async fn get_match_history() -> Result<Vec<MatchSummary>, String> {
    // 1. Read the lockfile
    let lockfile = lockfile::read_lockfile()?;

    // 2. Build an HTTP client that tolerates the self-signed cert
    let client = api::build_http_client()?;

    // 3. Get auth tokens from the local Riot Client
    let entitlements = auth::get_entitlements(&client, &lockfile).await?;

    // 4. Get region, shard, and client version from the active session
    let (_region, shard, client_version) = api::get_session_info(&client, &lockfile).await?;

    // 5. Fetch static data (maps & agents) for display names
    let maps = api::fetch_maps(&client).await.unwrap_or_default();
    let agents = api::fetch_agents(&client).await.unwrap_or_default();

    // 6. Fetch the last 15 matches
    let history = api::fetch_match_history(
        &client,
        &shard,
        &entitlements.subject,
        &entitlements.access_token,
        &entitlements.token,
        &client_version,
        0,
        15,
    )
    .await?;

    // 7. Fetch details for each match and build summaries
    let mut summaries = Vec::new();
    for entry in &history.history {
        match api::fetch_match_details(
            &client,
            &shard,
            &entry.match_id,
            &entitlements.access_token,
            &entitlements.token,
            &client_version,
        )
        .await
        {
            Ok(details) => {
                summaries.push(api::build_match_summary(
                    &details,
                    &entitlements.subject,
                    &maps,
                    &agents,
                ));
            }
            Err(e) => {
                eprintln!("Skipping match {}: {}", entry.match_id, e);
            }
        }
    }

    Ok(summaries)
}

/// Fetch the full scoreboard for a single match (both teams, all players).
#[tauri::command]
async fn get_match_detail(match_id: String) -> Result<MatchDetailView, String> {
    let lockfile = lockfile::read_lockfile()?;
    let client = api::build_http_client()?;
    let entitlements = auth::get_entitlements(&client, &lockfile).await?;
    let (_region, shard, client_version) = api::get_session_info(&client, &lockfile).await?;

    let agents = api::fetch_agents(&client).await.unwrap_or_default();
    let maps = api::fetch_maps(&client).await.unwrap_or_default();

    let details = api::fetch_match_details(
        &client,
        &shard,
        &match_id,
        &entitlements.access_token,
        &entitlements.token,
        &client_version,
    )
    .await?;

    Ok(api::build_match_detail_view(
        &details,
        &entitlements.subject,
        &shard,
        &maps,
        &agents,
    ))
}

pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(ReckonState::default()))
        .setup(|app| {
            // Restore persisted session from a previous launch
            if let Some(session) = load_session(&app.handle()) {
                if let Ok(mut guard) = app.state::<Mutex<ReckonState>>().lock() {
                    guard.token = Some(session.token);
                    guard.user = Some(session.user);
                    eprintln!("Restored Reckon session from disk");
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_valorant_running,
            get_match_history,
            get_match_detail,
            reckon_login,
            reckon_logout,
            reckon_get_status,
            reckon_fetch_data,
            reckon_get_players,
            reckon_get_teams,
            reckon_upload_match,
            reckon_link_account,
            reckon_get_soloq_accounts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
