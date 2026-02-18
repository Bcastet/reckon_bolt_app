#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Local API: Entitlements Token ───

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsResponse {
    pub access_token: String,
    pub token: String,
    pub subject: String,
}

// ─── Local API: Sessions ───

pub type SessionsResponse = HashMap<String, Session>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub launch_configuration: LaunchConfiguration,
    pub product_id: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct LaunchConfiguration {
    pub arguments: Vec<String>,
}

// ─── Remote API: Match History ───

#[derive(Debug, Deserialize)]
pub struct MatchHistoryResponse {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "BeginIndex")]
    pub begin_index: u32,
    #[serde(rename = "EndIndex")]
    pub end_index: u32,
    #[serde(rename = "Total")]
    pub total: u32,
    #[serde(rename = "History")]
    pub history: Vec<MatchHistoryEntry>,
}

#[derive(Debug, Deserialize)]
pub struct MatchHistoryEntry {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "GameStartTime")]
    pub game_start_time: u64,
    #[serde(rename = "QueueID")]
    pub queue_id: String,
}

// ─── Remote API: Match Details ───

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDetailsResponse {
    pub match_info: MatchInfo,
    pub players: Vec<Player>,
    pub teams: Option<Vec<Team>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub match_id: String,
    pub map_id: String,
    pub game_mode: String,
    #[serde(default)]
    pub game_length_millis: Option<u64>,
    pub game_start_millis: u64,
    #[serde(rename = "provisioningFlowID")]
    pub provisioning_flow_id: String,
    pub custom_game_name: String,
    #[serde(rename = "queueID")]
    pub queue_id: String,
    pub is_ranked: bool,
    pub season_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub subject: String,
    pub game_name: String,
    pub tag_line: String,
    pub team_id: String,
    pub character_id: String,
    pub stats: Option<PlayerStats>,
    pub competitive_tier: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub score: u32,
    pub rounds_played: u32,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub team_id: String,
    pub won: bool,
    pub rounds_played: u32,
    pub rounds_won: u32,
}

// ─── valorant-api.com static data ───

#[derive(Debug, Deserialize)]
pub struct ValorantApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    pub display_name: String,
    pub map_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentData {
    pub uuid: String,
    pub display_name: String,
    pub is_playable_character: bool,
}

// ─── Frontend-facing summary (match list card) ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchSummary {
    pub match_id: String,
    pub map_name: String,
    pub game_start_time: u64,
    pub game_length_secs: Option<u64>,
    pub queue_display_name: String,
    pub is_ranked: bool,
    pub is_custom_game: bool,
    pub custom_game_name: String,
    pub player_agent: String,
    pub player_kills: u32,
    pub player_deaths: u32,
    pub player_assists: u32,
    pub player_score: u32,
    pub won: bool,
    pub rounds_won: u32,
    pub rounds_lost: u32,
}

// ─── Frontend-facing detail (expanded scoreboard) ───

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDetailView {
    pub match_id: String,
    pub map_name: String,
    pub queue_display_name: String,
    pub is_custom_game: bool,
    pub is_ranked: bool,
    pub team_blue: Vec<PlayerSummary>,
    pub team_red: Vec<PlayerSummary>,
    pub blue_rounds_won: u32,
    pub red_rounds_won: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSummary {
    pub name: String,
    pub agent: String,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub score: u32,
    pub is_current_player: bool,
}
