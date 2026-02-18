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

    // 2. Re-fetch the raw match details from Riot to get the full JSON
    let lockfile = valorant::lockfile::read_lockfile()?;
    let riot_client = valorant::api::build_http_client()?;
    let entitlements = valorant::auth::get_entitlements(&riot_client, &lockfile).await?;
    let (_region, shard, client_version) = valorant::api::get_session_info(&riot_client, &lockfile).await?;

    let match_details = valorant::api::fetch_match_details(
        &riot_client,
        &shard,
        &match_id,
        &entitlements.access_token,
        &entitlements.token,
        &client_version,
    )
    .await?;

    // 3. Serialize the match details to a JSON string
    let match_json = serde_json::to_string(&match_details)
        .map_err(|e| format!("Failed to serialize match details: {}", e))?;

    // 4. Build the Reckon API configuration with token auth
    let config = reckon_api::apis::configuration::Configuration::prod().with_token(&token);

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
    account_name: String,
    server: String,
) -> Result<serde_json::Value, String> {
    let token = {
        let guard = state.lock().map_err(|_| "State lock poisoned".to_string())?;
        guard.token.clone().ok_or("Not connected to Reckon Bolt")?
    };

    let config = reckon_api::apis::configuration::Configuration::prod().with_token(&token);

    let body = reckon_api::models::AddAccount::new(account_name, server);

    let result = reckon_api::apis::add_account_api::add_player_account(
        &config,
        &player_id,
        body,
    )
    .await
    .map_err(|e| format!("Failed to link account: {}", e))?;

    Ok(serde_json::json!({ "accounts": result }))
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
