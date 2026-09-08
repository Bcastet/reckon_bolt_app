use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use std::collections::HashMap;

use super::lockfile::LockfileData;
use super::types::*;

/// Fixed base64-encoded client platform string expected by Riot endpoints.
const CLIENT_PLATFORM: &str = "ew0KCSJwbGF0Zm9ybVR5cGUiOiAiUEMiLA0KCSJwbGF0Zm9ybU9TIjogIldpbmRvd3MiLA0KCSJwbGF0Zm9ybU9TVmVyc2lvbiI6ICIxMC4wLjE5MDQyLjEuMjU2LjY0Yml0IiwNCgkicGxhdGZvcm1DaGlwc2V0IjogIlVua25vd24iDQp9";

/// Builds an HTTP client that accepts the Riot Client's self-signed certificate.
pub fn build_http_client() -> Result<Client, String> {
    Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))
}

// ─── Region / Shard detection ───

/// Fetches session info from the local Riot Client and extracts (region, shard, client_version).
pub async fn get_session_info(
    client: &Client,
    lockfile: &LockfileData,
) -> Result<(String, String, String), String> {
    let credentials = STANDARD.encode(format!("riot:{}", lockfile.password));

    let resp = client
        .get(format!(
            "https://127.0.0.1:{}/product-session/v1/external-sessions",
            lockfile.port
        ))
        .header("Authorization", format!("Basic {}", credentials))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch sessions: {}", e))?;

    let sessions: SessionsResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse sessions: {}", e))?;

    // Find the Valorant session
    let val_session = sessions
        .values()
        .find(|s| s.product_id == "valorant")
        .ok_or("No active Valorant session found")?;

    // Extract region from -ares-deployment= launch argument
    let region = val_session
        .launch_configuration
        .arguments
        .iter()
        .find_map(|arg| arg.strip_prefix("-ares-deployment="))
        .ok_or("Could not determine region from session arguments")?
        .to_string();

    let shard = region_to_shard(&region);
    let client_version = val_session.version.clone();

    Ok((region, shard, client_version))
}

pub fn region_to_shard(region: &str) -> String {
    match region {
        "na" | "br" | "latam" => "na".to_string(),
        "pbe" => "pbe".to_string(),
        "eu" => "eu".to_string(),
        "ap" => "ap".to_string(),
        "kr" => "kr".to_string(),
        other => other.to_string(),
    }
}

pub fn glz_base_url(region: &str, shard: &str) -> String {
    format!("https://glz-{}-1.{}.a.pvp.net", region, shard)
}

pub fn build_riot_headers(
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("Authorization", format!("Bearer {}", auth_token)),
        ("X-Riot-Entitlements-JWT", entitlement_token.to_string()),
        ("X-Riot-ClientVersion", client_version.to_string()),
        ("X-Riot-ClientPlatform", CLIENT_PLATFORM.to_string()),
    ]
}

pub fn local_auth_header(lockfile: &LockfileData) -> String {
    let credentials = STANDARD.encode(format!("riot:{}", lockfile.password));
    format!("Basic {}", credentials)
}

/// Fetch a usable Riot client version without requiring Valorant to be running.
pub async fn fetch_riot_client_version(client: &Client) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct VersionData {
        riot_client_version: String,
    }
    #[derive(serde::Deserialize)]
    struct VersionResponse {
        data: VersionData,
    }

    let resp = client
        .get("https://valorant-api.com/v1/version")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch client version: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Client version request failed with status {}",
            resp.status()
        ));
    }

    let body: VersionResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse client version: {}", e))?;

    if body.data.riot_client_version.is_empty() {
        return Err("Empty riotClientVersion from valorant-api.com".to_string());
    }
    Ok(body.data.riot_client_version)
}

/// Infer PD shard from match pod / GLZ zone strings in match-details JSON.
pub fn shard_from_match_info(info: &MatchInfo) -> Option<String> {
    shard_from_pod_string(&info.game_pod_id)
        .or_else(|| shard_from_pod_string(&info.game_loop_zone))
}

fn shard_from_pod_string(s: &str) -> Option<String> {
    let lower = s.to_lowercase();
    // Prefer longer / more specific tokens first (latam before na, etc.)
    for key in ["latam", "pbe", "br", "na", "eu", "ap", "kr"] {
        let dotted = format!(".{key}-");
        let dashed = format!("-{key}-");
        let dotted_end = format!(".{key}.");
        if lower.contains(&dotted) || lower.contains(&dashed) || lower.contains(&dotted_end) {
            return Some(region_to_shard(key));
        }
    }
    None
}

/// Resolve Riot IDs (gameName#tagLine) for a list of PUUIDs via the name-service.
/// Match-details often returns empty gameName/tagLine; this fills them in.
pub async fn resolve_player_names(
    client: &Client,
    shard: &str,
    puuids: &[String],
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> HashMap<String, String> {
    if puuids.is_empty() || shard.is_empty() || client_version.is_empty() {
        crate::journal::warn(
            "NameService",
            &format!(
                "Skipping resolve (puuids={}, shard='{}', version empty={})",
                puuids.len(),
                shard,
                client_version.is_empty()
            ),
        );
        return HashMap::new();
    }

    let url = format!("https://pd.{}.a.pvp.net/name-service/v2/players", shard);
    let headers = build_riot_headers(auth_token, entitlement_token, client_version);

    let mut req = client.put(&url);
    for (name, value) in headers {
        req = req.header(name, value);
    }
    req = req.json(puuids);

    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            crate::journal::warn("NameService", &format!("Request failed: {}", e));
            return HashMap::new();
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        crate::journal::warn(
            "NameService",
            &format!("HTTP {} for {} puuids: {}", status, puuids.len(), body),
        );
        return HashMap::new();
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct NameEntry {
        subject: String,
        game_name: String,
        tag_line: String,
    }

    let entries: Vec<NameEntry> = match resp.json().await {
        Ok(e) => e,
        Err(e) => {
            crate::journal::warn("NameService", &format!("Failed to parse response: {}", e));
            return HashMap::new();
        }
    };

    let map: HashMap<String, String> = entries
        .into_iter()
        .filter(|e| !e.game_name.is_empty())
        .map(|e| (e.subject, format!("{}#{}", e.game_name, e.tag_line)))
        .collect();

    crate::journal::info(
        "NameService",
        &format!("Resolved {}/{} names on shard={}", map.len(), puuids.len(), shard),
    );
    map
}

/// Write resolved Riot IDs back into match-details JSON (`gameName` / `tagLine`).
pub fn apply_names_to_match_json(
    raw_json: &str,
    names: &HashMap<String, String>,
) -> Result<String, String> {
    if names.is_empty() {
        return Ok(raw_json.to_string());
    }

    let mut value: serde_json::Value = serde_json::from_str(raw_json)
        .map_err(|e| format!("Invalid match JSON: {}", e))?;

    let Some(players) = value.get_mut("players").and_then(|p| p.as_array_mut()) else {
        return Ok(raw_json.to_string());
    };

    for player in players {
        let subject = player
            .get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let Some(full) = names.get(&subject) else {
            continue;
        };
        let Some((game, tag)) = full.split_once('#') else {
            continue;
        };
        if game.is_empty() {
            continue;
        }
        player["gameName"] = serde_json::Value::String(game.to_string());
        player["tagLine"] = serde_json::Value::String(tag.to_string());
    }

    serde_json::to_string(&value).map_err(|e| format!("Failed to serialize match JSON: {}", e))
}

/// True when a Riot ID string looks usable (non-empty name and tagline).
pub fn is_usable_riot_id(name: &str) -> bool {
    let t = name.trim();
    if t.is_empty() || t == "#" || t == "Unknown" {
        return false;
    }
    match t.split_once('#') {
        Some((game, tag)) => !game.is_empty() && !tag.is_empty(),
        None => false,
    }
}

/// Resolve missing player names for a match using Riot Client auth when available.
/// Does not require an active Valorant session (uses match pod + public client version).
pub async fn resolve_missing_match_names(
    client: &Client,
    details: &MatchDetailsResponse,
    auth_token: &str,
    entitlement_token: &str,
    preferred_shard: Option<&str>,
    preferred_version: Option<&str>,
) -> HashMap<String, String> {
    let needs_resolve: Vec<String> = details
        .players
        .iter()
        .filter(|p| p.game_name.trim().is_empty())
        .map(|p| p.subject.clone())
        .collect();

    if needs_resolve.is_empty() {
        return HashMap::new();
    }

    let shard = preferred_shard
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| shard_from_match_info(&details.match_info));

    let Some(shard) = shard else {
        crate::journal::warn("NameService", "Could not determine shard for name resolve");
        return HashMap::new();
    };

    let version = if let Some(v) = preferred_version.filter(|v| !v.is_empty()) {
        v.to_string()
    } else {
        match fetch_riot_client_version(client).await {
            Ok(v) => v,
            Err(e) => {
                crate::journal::warn("NameService", &format!("No client version: {}", e));
                return HashMap::new();
            }
        }
    };

    resolve_player_names(
        client,
        &shard,
        &needs_resolve,
        auth_token,
        entitlement_token,
        &version,
    )
    .await
}

// ─── Match History ───

pub async fn fetch_match_history(
    client: &Client,
    shard: &str,
    puuid: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
    start_index: u32,
    end_index: u32,
) -> Result<MatchHistoryResponse, String> {
    let url = format!(
        "https://pd.{}.a.pvp.net/match-history/v1/history/{}?startIndex={}&endIndex={}",
        shard, puuid, start_index, end_index
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", auth_token))
        .header("X-Riot-Entitlements-JWT", entitlement_token)
        .header("X-Riot-ClientVersion", client_version)
        .header("X-Riot-ClientPlatform", CLIENT_PLATFORM)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch match history: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Match history request failed with status {}",
            resp.status()
        ));
    }

    resp.json::<MatchHistoryResponse>()
        .await
        .map_err(|e| format!("Failed to parse match history: {}", e))
}

// ─── Match Details ───

pub async fn fetch_match_details(
    client: &Client,
    shard: &str,
    match_id: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> Result<MatchDetailsResponse, String> {
    let url = format!(
        "https://pd.{}.a.pvp.net/match-details/v1/matches/{}",
        shard, match_id
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", auth_token))
        .header("X-Riot-Entitlements-JWT", entitlement_token)
        .header("X-Riot-ClientVersion", client_version)
        .header("X-Riot-ClientPlatform", CLIENT_PLATFORM)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch match details: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!(
            "Match details request failed with status {} | URL: {} | Body: {}",
            status, url, body
        ));
    }

    resp.json::<MatchDetailsResponse>()
        .await
        .map_err(|e| format!("Failed to parse match details: {}", e))
}

/// Fetch the full match-details response as raw JSON (same endpoint as `fetch_match_details`).
/// Use this for uploads so the full payload (roundResults, kills, etc.) is preserved instead of
/// the subset we deserialize into `MatchDetailsResponse`.
pub async fn fetch_match_details_raw(
    client: &Client,
    shard: &str,
    match_id: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> Result<String, String> {
    let url = format!(
        "https://pd.{}.a.pvp.net/match-details/v1/matches/{}",
        shard, match_id
    );

    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", auth_token))
        .header("X-Riot-Entitlements-JWT", entitlement_token)
        .header("X-Riot-ClientVersion", client_version)
        .header("X-Riot-ClientPlatform", CLIENT_PLATFORM)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch match details: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!(
            "Match details request failed with status {} | URL: {} | Body: {}",
            status, url, body
        ));
    }

    resp.text()
        .await
        .map_err(|e| format!("Failed to read match details: {}", e))
}

fn sgp_cluster_from_shard(shard: &str) -> &'static str {
    match shard {
        "eu" => "euc1",
        "na" | "pbe" => "usw2",
        "kr" => "apne1",
        _ => "apse1",
    }
}

pub fn is_nonself_match_details_error(err: &str) -> bool {
    err.contains("NONSELF_OPERATION")
}

/// Best-effort Riot `errorCode` (or HTTP status class) from a PD/SUMMARY error string.
pub fn riot_error_code(err: &str) -> &'static str {
    for key in [
        "NONSELF_OPERATION",
        "BAD_CLAIMS",
        "RESOURCE_NOT_FOUND",
        "ACCESS_DENIED",
        "UNAUTHORIZED",
    ] {
        if err.contains(key) {
            return key;
        }
    }
    if err.contains("404") {
        return "HTTP_404";
    }
    if err.contains("403") {
        return "HTTP_403";
    }
    if err.contains("400") {
        return "HTTP_400";
    }
    "OTHER"
}

pub fn is_match_auth_error(err: &str) -> bool {
    matches!(
        riot_error_code(err),
        "BAD_CLAIMS" | "UNAUTHORIZED" | "ACCESS_DENIED"
    ) || err.contains("401 Unauthorized")
}

fn truncate_for_log(s: &str, max: usize) -> String {
    let one_line: String = s
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    if one_line.chars().count() <= max {
        one_line
    } else {
        format!("{}…", one_line.chars().take(max).collect::<String>())
    }
}

/// Download match JSON via SGP SUMMARY using `subject_puuid`'s match history.
/// Works for your own games, and for a friend's games (party members count).
/// Spectators cannot use PD match-details (`NONSELF_OPERATION`); this is the fallback.
pub async fn fetch_match_summary_json(
    client: &Client,
    shard: &str,
    subject_puuid: &str,
    match_id: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
) -> Result<String, String> {
    let cluster = sgp_cluster_from_shard(shard);
    let url = format!(
        "https://{}.pp.sgp.pvp.net/match-history-query/v3/products/valorant/players/{}/infoTypes/SUMMARY?id={}",
        cluster, subject_puuid, match_id
    );

    let headers = build_riot_headers(auth_token, entitlement_token, client_version);
    let mut req = client.get(&url);
    for (name, value) in &headers {
        req = req.header(*name, value);
    }

    crate::journal::info(
        "LiveAPI",
        &format!(
            "SUMMARY GET {} (subject={}…)",
            url,
            &subject_puuid[..8.min(subject_puuid.len())]
        ),
    );

    let resp = req
        .send()
        .await
        .map_err(|e| format!("SUMMARY request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        crate::journal::info(
            "LiveAPI",
            &format!(
                "SUMMARY errorCode={} status={} subject={}… body={}",
                riot_error_code(&body),
                status,
                &subject_puuid[..8.min(subject_puuid.len())],
                truncate_for_log(&body, 400)
            ),
        );
        return Err(format!(
            "SUMMARY request failed with status {} | URL: {} | Body: {}",
            status, url, body
        ));
    }

    let parsed: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse SUMMARY response: {}", e))?;

    let url_keys: Vec<String> = parsed
        .as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    crate::journal::info(
        "LiveAPI",
        &format!(
            "SUMMARY 200 keys=[{}] for {}",
            url_keys.join(","),
            match_id
        ),
    );

    let urls = parsed
        .get("matchFileUrlsMap")
        .or_else(|| parsed.get("MatchFileUrlsMap"))
        .and_then(|v| v.as_object())
        .ok_or_else(|| format!("SUMMARY response missing matchFileUrlsMap: {}", parsed))?;

    let lower_id = match_id.to_lowercase();
    let file_url = urls
        .iter()
        .find(|(k, _)| k.to_lowercase() == lower_id)
        .or_else(|| urls.iter().next())
        .map(|(_, v)| v.as_str().unwrap_or(""))
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("SUMMARY response had no file URL for {}", match_id))?;

    crate::journal::info(
        "LiveAPI",
        &format!(
            "SUMMARY file URL for {} (map keys={}): {}",
            match_id,
            urls.keys().cloned().collect::<Vec<_>>().join(","),
            file_url.split('?').next().unwrap_or(file_url)
        ),
    );

    let file_resp = client
        .get(file_url)
        .send()
        .await
        .map_err(|e| format!("SUMMARY file download failed: {}", e))?;

    if !file_resp.status().is_success() {
        let status = file_resp.status();
        let body = file_resp.text().await.unwrap_or_default();
        crate::journal::info(
            "LiveAPI",
            &format!(
                "SUMMARY file download failed status={} body={}",
                status,
                truncate_for_log(&body, 300)
            ),
        );
        return Err(format!(
            "SUMMARY file download failed with status {} | Body: {}",
            status, body
        ));
    }

    file_resp
        .text()
        .await
        .map_err(|e| format!("Failed to read SUMMARY file: {}", e))
}

/// PD match-details for participants; SGP SUMMARY via self, roster, or party for observers.
///
/// Auth errors (`BAD_CLAIMS`) are returned immediately so the caller can refresh
/// tokens. Every other PD failure (including `NONSELF_OPERATION` and 404) falls
/// through to SUMMARY — observers never get a usable PD body.
pub async fn fetch_match_json_any_source(
    client: &Client,
    shard: &str,
    match_id: &str,
    auth_token: &str,
    entitlement_token: &str,
    client_version: &str,
    viewer_puuid: &str,
    roster_puuids: &[String],
) -> Result<String, String> {
    let viewer_prefix = if viewer_puuid.len() >= 8 {
        &viewer_puuid[..8]
    } else {
        viewer_puuid
    };
    crate::journal::info(
        "LiveAPI",
        &format!(
            "PD match-details GET https://pd.{}.a.pvp.net/match-details/v1/matches/{} viewer={}… roster={}",
            shard, match_id, viewer_prefix, roster_puuids.len()
        ),
    );

    match fetch_match_details_raw(
        client, shard, match_id, auth_token, entitlement_token, client_version,
    )
    .await
    {
        Ok(json) => {
            crate::journal::info(
                "LiveAPI",
                &format!("PD match-details OK ({} bytes) for {}", json.len(), match_id),
            );
            return Ok(json);
        }
        Err(e) => {
            let code = riot_error_code(&e);
            crate::journal::info(
                "LiveAPI",
                &format!(
                    "PD match-details errorCode={} for {}: {}",
                    code,
                    match_id,
                    truncate_for_log(&e, 500)
                ),
            );
            if is_nonself_match_details_error(&e) {
                crate::journal::info(
                    "LiveAPI",
                    "PD returned NONSELF_OPERATION — viewer is not a participant; trying SUMMARY",
                );
            }
            if is_match_auth_error(&e) {
                return Err(e);
            }
        }
    }

    let mut candidates = Vec::new();
    if !viewer_puuid.is_empty() {
        candidates.push(viewer_puuid.to_string());
    }
    for p in roster_puuids {
        if !p.is_empty() && !candidates.iter().any(|c| c == p) {
            candidates.push(p.clone());
        }
    }

    crate::journal::info(
        "LiveAPI",
        &format!(
            "Trying SUMMARY via {} PUUID(s) for {} (first={}…)",
            candidates.len(),
            match_id,
            candidates
                .first()
                .map(|p| &p[..8.min(p.len())])
                .unwrap_or("-"),
        ),
    );

    let mut last_err = "No roster/party PUUIDs to try for SUMMARY".to_string();
    for puuid in &candidates {
        match fetch_match_summary_json(
            client, shard, puuid, match_id, auth_token, entitlement_token, client_version,
        )
        .await
        {
            Ok(json) => {
                crate::journal::info(
                    "LiveAPI",
                    &format!(
                        "SUMMARY OK via puuid {}… ({} bytes)",
                        &puuid[..8.min(puuid.len())],
                        json.len()
                    ),
                );
                return Ok(json);
            }
            Err(e) => {
                crate::journal::info(
                    "LiveAPI",
                    &format!(
                        "SUMMARY failed via {}… errorCode={}: {}",
                        &puuid[..8.min(puuid.len())],
                        riot_error_code(&e),
                        truncate_for_log(&e, 400)
                    ),
                );
                last_err = e;
            }
        }
    }

    Err(last_err)
}

// ─── Static data (maps & agents from valorant-api.com) ───

pub async fn fetch_maps(client: &Client) -> Result<HashMap<String, String>, String> {
    let resp = client
        .get("https://valorant-api.com/v1/maps")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch maps: {}", e))?;

    let data: ValorantApiResponse<Vec<MapData>> = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse maps: {}", e))?;

    // Build lookup: mapUrl -> displayName
    let map = data
        .data
        .into_iter()
        .map(|m| (m.map_url, m.display_name))
        .collect();

    Ok(map)
}

pub async fn fetch_agents(client: &Client) -> Result<HashMap<String, String>, String> {
    let resp = client
        .get("https://valorant-api.com/v1/agents?isPlayableCharacter=true")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch agents: {}", e))?;

    let data: ValorantApiResponse<Vec<AgentData>> = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse agents: {}", e))?;

    // Build lookup: uuid (lowercase) -> displayName
    let map = data
        .data
        .into_iter()
        .map(|a| (a.uuid.to_lowercase(), a.display_name))
        .collect();

    Ok(map)
}

// ─── Build a MatchSummary from match details ───

pub fn build_match_summary(
    details: &MatchDetailsResponse,
    puuid: &str,
    maps: &HashMap<String, String>,
    agents: &HashMap<String, String>,
) -> MatchSummary {
    let info = &details.match_info;

    // Find the current player
    let player = details.players.iter().find(|p| p.subject == puuid);

    let (kills, deaths, assists, score, player_team, agent_id) = match player {
        Some(p) => {
            let stats = p.stats.as_ref();
            (
                stats.map_or(0, |s| s.kills),
                stats.map_or(0, |s| s.deaths),
                stats.map_or(0, |s| s.assists),
                stats.map_or(0, |s| s.score),
                p.team_id.clone(),
                p.character_id.clone(),
            )
        }
        None => (0, 0, 0, 0, String::new(), String::new()),
    };

    // Determine win/loss and round scores
    let (won, rounds_won, rounds_lost) = match &details.teams {
        Some(teams) => {
            let player_team_data = teams.iter().find(|t| t.team_id == player_team);
            let enemy_team_data = teams.iter().find(|t| t.team_id != player_team);
            (
                player_team_data.map_or(false, |t| t.won),
                player_team_data.map_or(0, |t| t.rounds_won),
                enemy_team_data.map_or(0, |t| t.rounds_won),
            )
        }
        None => (false, 0, 0),
    };

    // Resolve display names
    let map_name = maps
        .get(&info.map_id)
        .cloned()
        .unwrap_or_else(|| extract_map_fallback(&info.map_id));

    let agent_name = agents
        .get(&agent_id.to_lowercase())
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());

    let is_custom = info.provisioning_flow_id == "CustomGame";

    MatchSummary {
        match_id: info.match_id.clone(),
        map_name,
        game_start_time: info.game_start_millis,
        game_length_secs: info.game_length_millis.map(|ms| ms / 1000),
        queue_display_name: queue_display_name(&info.queue_id, is_custom),
        is_ranked: info.is_ranked,
        is_custom_game: is_custom,
        custom_game_name: info.custom_game_name.clone(),
        player_agent: agent_name,
        player_kills: kills,
        player_deaths: deaths,
        player_assists: assists,
        player_score: score,
        won,
        rounds_won,
        rounds_lost,
    }
}

pub fn build_match_detail_view(
    details: &MatchDetailsResponse,
    puuid: &str,
    shard: &str,
    maps: &HashMap<String, String>,
    agents: &HashMap<String, String>,
    resolved_names: &HashMap<String, String>,
) -> MatchDetailView {
    let info = &details.match_info;
    let is_custom = info.provisioning_flow_id == "CustomGame";

    let map_name = maps
        .get(&info.map_id)
        .cloned()
        .unwrap_or_else(|| extract_map_fallback(&info.map_id));

    // Build player summaries, split by team
    let mut team_blue: Vec<PlayerSummary> = Vec::new();
    let mut team_red: Vec<PlayerSummary> = Vec::new();

    for p in &details.players {
        let stats = p.stats.as_ref();
        let agent_name = agents
            .get(&p.character_id.to_lowercase())
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let from_match = format!("{}#{}", p.game_name, p.tag_line);
        let name = if is_usable_riot_id(&from_match) {
            from_match
        } else {
            resolved_names
                .get(&p.subject)
                .cloned()
                .unwrap_or(from_match)
        };

        let summary = PlayerSummary {
            puuid: p.subject.clone(),
            name,
            agent: agent_name,
            kills: stats.map_or(0, |s| s.kills),
            deaths: stats.map_or(0, |s| s.deaths),
            assists: stats.map_or(0, |s| s.assists),
            score: stats.map_or(0, |s| s.score),
            is_current_player: p.subject == puuid,
        };

        if p.team_id == "Blue" {
            team_blue.push(summary);
        } else {
            team_red.push(summary);
        }
    }

    // Sort each team by score descending (top fragger first)
    team_blue.sort_by(|a, b| b.score.cmp(&a.score));
    team_red.sort_by(|a, b| b.score.cmp(&a.score));

    // Round scores per team
    let (blue_rounds, red_rounds) = match &details.teams {
        Some(teams) => {
            let blue = teams.iter().find(|t| t.team_id == "Blue");
            let red = teams.iter().find(|t| t.team_id != "Blue");
            (
                blue.map_or(0, |t| t.rounds_won),
                red.map_or(0, |t| t.rounds_won),
            )
        }
        None => (0, 0),
    };

    MatchDetailView {
        match_id: info.match_id.clone(),
        map_name,
        queue_display_name: queue_display_name(&info.queue_id, is_custom),
        is_custom_game: is_custom,
        is_ranked: info.is_ranked,
        server: shard.to_string(),
        team_blue,
        team_red,
        blue_rounds_won: blue_rounds,
        red_rounds_won: red_rounds,
    }
}

pub fn extract_map_fallback(map_id: &str) -> String {
    map_id
        .split('/')
        .last()
        .unwrap_or("Unknown")
        .to_string()
}

pub fn queue_display_name(queue_id: &str, is_custom: bool) -> String {
    if is_custom {
        return "Custom Game".to_string();
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
        "snowball" => "Snowball Fight",
        "" => "Unknown",
        other => other,
    }
    .to_string()
}
