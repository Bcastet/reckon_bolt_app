# generate-api-client.ps1
# Downloads the OpenAPI schema from the dev backend and regenerates the Rust client crate.
#
# Usage:
#   .\generate-api-client.ps1                    # default: downloads from localhost:8000
#   .\generate-api-client.ps1 -SchemaUrl "http://other-host:8000/schema/"
#   .\generate-api-client.ps1 -SkipDownload      # regenerate from existing schema file

param(
    [string]$SchemaUrl = "http://localhost:8000/schema/?format=yaml",
    [switch]$SkipDownload
)

$ErrorActionPreference = "Stop"

$projectRoot = $PSScriptRoot
$schemaFile  = Join-Path $projectRoot "reckon-schema.yaml"
$outputDir   = Join-Path $projectRoot "reckon-api"

# ── 1. Download the schema ──────────────────────────────────────────────────
if (-not $SkipDownload) {
    Write-Host "Downloading OpenAPI schema from $SchemaUrl ..." -ForegroundColor Cyan
    try {
        Invoke-RestMethod -Uri $SchemaUrl -OutFile $schemaFile -TimeoutSec 30
        Write-Host "Schema saved to $schemaFile" -ForegroundColor Green
    }
    catch {
        Write-Error "Failed to download schema: $_"
        exit 1
    }
}
else {
    if (-not (Test-Path $schemaFile)) {
        Write-Error "Schema file not found at $schemaFile. Run without -SkipDownload first."
        exit 1
    }
    Write-Host "Skipping download, using existing $schemaFile" -ForegroundColor Yellow
}

# ── 2. Clean previous generation (keep Cargo.toml tweaks if any) ────────────
if (Test-Path $outputDir) {
    Write-Host "Removing previous generated client..." -ForegroundColor Yellow
    Remove-Item -Recurse -Force $outputDir
}

# ── 3. Run openapi-generator ────────────────────────────────────────────────
Write-Host "Generating Rust client in $outputDir ..." -ForegroundColor Cyan

try {
    $configFile_gen = Join-Path $projectRoot "openapi-generator-config.yaml"
    & openapi-generator-cli generate `
        -i $schemaFile `
        -g rust `
        -c $configFile_gen `
        -o $outputDir `
        --skip-validate-spec
    if ($LASTEXITCODE -ne 0) {
        Write-Error "openapi-generator exited with code $LASTEXITCODE"
        exit 1
    }
}
catch {
    Write-Error "Failed to run openapi-generator-cli. Make sure it is installed: npm install -g @openapitools/openapi-generator-cli`n$_"
    exit 1
}

# ── 4. Post-generation fixes ─────────────────────────────────────────────────
# Fix known openapi-generator bug: `models::serde_json::Value` → `serde_json::Value`
Write-Host "Applying post-generation fixes..." -ForegroundColor Cyan
$apiDir = Join-Path (Join-Path $outputDir "src") "apis"
Get-ChildItem -Path $apiDir -Filter "*.rs" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    if ($content -match 'models::serde_json::Value') {
        $content = $content -replace 'models::serde_json::Value', 'serde_json::Value'
        Set-Content -Path $_.FullName -Value $content -NoNewline
        Write-Host "  Fixed $($_.Name)" -ForegroundColor Gray
    }
}

# ── 4b. additional_filters: use bracket notation (additional_filters[key]=value, arrays comma-separated) ─
Write-Host "Patching additional_filters query serialization (bracket notation)..." -ForegroundColor Cyan
$modPath = Join-Path $apiDir "mod.rs"
$modContent = Get-Content $modPath -Raw
$helperInsert = @'

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

'@
# Insert helper after parse_deep_object (before "/// Internal use only")
$modContent = $modContent -replace "(\}\r?\n\r?\n)(/// Internal use only)", "`$1$helperInsert`n`$2"
Set-Content -Path $modPath -Value $modContent -NoNewline

$oldQueryLine = '        req_builder = req_builder.query(&[("additional_filters", &serde_json::to_string(param_value)?)]);'
$newQueryBlock = @'
        for (k, v) in crate::apis::additional_filters_query_pairs(param_value) {
            req_builder = req_builder.query(&[(k.as_str(), v.as_str())]);
        }
'@
Get-ChildItem -Path $apiDir -Filter "*.rs" | ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    if ($content -match [regex]::Escape($oldQueryLine)) {
        $content = $content.Replace($oldQueryLine, $newQueryBlock)
        Set-Content -Path $_.FullName -Value $content -NoNewline
        Write-Host "  Patched additional_filters in $($_.Name)" -ForegroundColor Gray
    }
}

# ── 4c. upload_scrim_game: send multipart/form-data (team1, team2, agnostic_match_history file) ─
Write-Host "Patching upload_scrim_game to use multipart/form-data..." -ForegroundColor Cyan
$uploadJsonLine = '    req_builder = req_builder.json(&p_body_upload_scrim_game_request);'
$uploadMultipartBlock = @'
    let json_part = reqwest::multipart::Part::bytes(p_body_upload_scrim_game_request.agnostic_match_history.into_bytes())
        .file_name("agnostic_match_history.json")
        .mime_str("application/json")?;
    let form = reqwest::multipart::Form::new()
        .text("team1", p_body_upload_scrim_game_request.team1.clone())
        .text("team2", p_body_upload_scrim_game_request.team2.clone())
        .part("agnostic_match_history", json_part);
    req_builder = req_builder.multipart(form);
'@
foreach ($apiFile in @("scrims_data_api.rs", "upload_api.rs")) {
    $path = Join-Path $apiDir $apiFile
    if (Test-Path $path) {
        $content = Get-Content $path -Raw
        if ($content -match [regex]::Escape($uploadJsonLine)) {
            $content = $content.Replace($uploadJsonLine, $uploadMultipartBlock)
            Set-Content -Path $path -Value $content -NoNewline
            Write-Host "  Patched upload_scrim_game in $apiFile" -ForegroundColor Gray
        }
    }
}

# ── 5. Patch Configuration to support dev/prod base URLs ─────────────────────
$configFile = Join-Path (Join-Path (Join-Path $outputDir "src") "apis") "configuration.rs"
Write-Host "Patching Configuration with dev/prod base URLs..." -ForegroundColor Cyan

$configContent = @'
/*
 * Reckon Bolt API
 *
 * Backend API for Reckon Bolt
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */


/// Base URL for the development backend (localhost).
pub const BASE_URL_DEV: &str = "http://localhost:8000";

/// Base URL for the production backend.
pub const BASE_URL_PROD: &str = "https://api.reckon-bolt.com";

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }

    /// Create a configuration pointing at the **development** backend.
    pub fn dev() -> Configuration {
        Configuration {
            base_path: BASE_URL_DEV.to_owned(),
            ..Default::default()
        }
    }

    /// Create a configuration pointing at the **production** backend.
    pub fn prod() -> Configuration {
        Configuration {
            base_path: BASE_URL_PROD.to_owned(),
            ..Default::default()
        }
    }

    /// Create a configuration with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Configuration {
        Configuration {
            base_path: base_url.to_owned(),
            ..Default::default()
        }
    }

    /// Set the API token for authentication (token auth).
    pub fn with_token(mut self, token: &str) -> Configuration {
        self.api_key = Some(ApiKey {
            prefix: Some("Token".to_owned()),
            key: token.to_owned(),
        });
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: BASE_URL_DEV.to_owned(),
            user_agent: Some("ReckonBoltApp/1.0.0".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
'@
Set-Content -Path $configFile -Value $configContent -NoNewline

Write-Host "`nRust client generated successfully in $outputDir" -ForegroundColor Green
Write-Host "Don't forget to run 'cargo check' in src-tauri/ to verify compilation." -ForegroundColor Cyan
