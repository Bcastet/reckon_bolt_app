use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;

use super::lockfile::LockfileData;
use super::types::EntitlementsResponse;

/// Fetches auth + entitlement tokens from the local Riot Client API.
pub async fn get_entitlements(
    client: &Client,
    lockfile: &LockfileData,
) -> Result<EntitlementsResponse, String> {
    let credentials = STANDARD.encode(format!("riot:{}", lockfile.password));

    let resp = client
        .get(format!(
            "https://127.0.0.1:{}/entitlements/v1/token",
            lockfile.port
        ))
        .header("Authorization", format!("Basic {}", credentials))
        .send()
        .await
        .map_err(|e| format!("Failed to reach Riot Client API: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Entitlements request failed with status {}",
            resp.status()
        ));
    }

    resp.json::<EntitlementsResponse>()
        .await
        .map_err(|e| format!("Failed to parse entitlements response: {}", e))
}
