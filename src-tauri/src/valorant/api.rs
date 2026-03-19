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
        return Err(format!(
            "Match details request failed with status {}",
            resp.status()
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
        return Err(format!(
            "Match details request failed with status {}",
            resp.status()
        ));
    }

    resp.text()
        .await
        .map_err(|e| format!("Failed to read match details: {}", e))
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

        let summary = PlayerSummary {
            puuid: p.subject.clone(),
            name: format!("{}#{}", p.game_name, p.tag_line),
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

fn extract_map_fallback(map_id: &str) -> String {
    map_id
        .split('/')
        .last()
        .unwrap_or("Unknown")
        .to_string()
}

fn queue_display_name(queue_id: &str, is_custom: bool) -> String {
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
        "" => "Custom Game",
        other => other,
    }
    .to_string()
}
