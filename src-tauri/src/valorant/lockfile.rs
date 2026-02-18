use std::env;
use std::fs;

#[derive(Debug)]
pub struct LockfileData {
    pub port: u16,
    pub password: String,
}

/// Reads and parses the Riot Client lockfile.
/// Returns `Err` if Valorant / Riot Client is not running.
pub fn read_lockfile() -> Result<LockfileData, String> {
    let local_app_data =
        env::var("LOCALAPPDATA").map_err(|_| "LOCALAPPDATA environment variable not found")?;

    let path = format!(
        "{}\\Riot Games\\Riot Client\\Config\\lockfile",
        local_app_data
    );

    let content = fs::read_to_string(&path).map_err(|_| {
        "Could not read Riot Client lockfile. Is Valorant running?".to_string()
    })?;

    let parts: Vec<&str> = content.split(':').collect();
    if parts.len() < 5 {
        return Err("Invalid lockfile format".to_string());
    }

    let port = parts[2]
        .parse::<u16>()
        .map_err(|_| "Invalid port in lockfile")?;

    Ok(LockfileData {
        port,
        password: parts[3].to_string(),
    })
}
