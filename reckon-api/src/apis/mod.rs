use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}


/// Serialize additional_filters as query params: additional_filters[key]=value (arrays as comma-separated).
pub fn additional_filters_query_pairs(value: &serde_json::Value) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    if let serde_json::Value::Object(map) = value {
        for (key, val) in map {
            let param_name = format!("additional_filters[{}]", key);
            let param_value = match val {
                serde_json::Value::Array(arr) => arr.iter()
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(","),
                serde_json::Value::String(s) => s.clone(),
                _ => val.to_string(),
            };
            pairs.push((param_name, param_value));
        }
    }
    pairs
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod add_account_api;
pub mod aggregate_api;
pub mod app_api;
pub mod change_password_api;
pub mod client_organization_api;
pub mod competitive_draft_slots_api;
pub mod competitive_game_summaries_api;
pub mod competitive_games_api;
pub mod competitive_round_summaries_api;
pub mod competitive_team_round_summaries_api;
pub mod create_api;
pub mod delete_api;
pub mod download_api;
pub mod field_values_api;
pub mod game_metrics_api;
pub mod get_api;
pub mod league_api;
pub mod list_api;
pub mod login_api;
pub mod maps_api;
pub mod patch_api;
pub mod player_api;
pub mod post_api;
pub mod releases_api;
pub mod riot_account_api;
pub mod schema_api;
pub mod scrim_game_summaries_api;
pub mod scrim_games_api;
pub mod scrim_round_summaries_api;
pub mod scrim_team_round_summaries_api;
pub mod scrims_data_api;
pub mod solo_q_accounts_api;
pub mod team_api;
pub mod upload_api;
pub mod user_api;
pub mod variable_distribution_api;

pub mod configuration;
