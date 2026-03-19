param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$Version
)

$ErrorActionPreference = "Stop"

if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "Error: Version must be a valid semver string (e.g. 1.0.5)." -ForegroundColor Red
    exit 1
}

$ProjectRoot = $PSScriptRoot
$TauriConfPath = Join-Path $ProjectRoot "src-tauri\tauri.conf.json"
$CargoTomlPath = Join-Path $ProjectRoot "src-tauri\Cargo.toml"
$ReleasesDir = Join-Path $ProjectRoot "releases"
$VersionsJsonPath = Join-Path $ReleasesDir "versions.json"

function WriteFileNoBom([string]$Path, [string]$Content) {
    [System.IO.File]::WriteAllText($Path, $Content, (New-Object System.Text.UTF8Encoding($false)))
}

$tauriContent = [System.IO.File]::ReadAllText($TauriConfPath)
if ($tauriContent -match '"version"\s*:\s*"(\d+\.\d+\.\d+)"') {
    $currentVersion = $Matches[1]
} else {
    throw "Could not find version in tauri.conf.json"
}

Write-Host ""
Write-Host "=== Reckon Bolt Release ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Current version : $currentVersion"
Write-Host "  New version     : $Version"
Write-Host ""

$confirm = Read-Host "Proceed with release v$Version? (y/N)"
if ($confirm -ne 'y') {
    Write-Host "Aborted." -ForegroundColor Yellow
    exit 0
}

# Step 1: Update version in config files via string replacement
Write-Host ""
Write-Host "[1/4] Updating version..." -ForegroundColor Cyan

$tauriContent = $tauriContent -replace "`"version`"\s*:\s*`"$currentVersion`"", "`"version`": `"$Version`""
WriteFileNoBom $TauriConfPath $tauriContent
Write-Host "  tauri.conf.json -> $Version" -ForegroundColor DarkGray

$cargoContent = [System.IO.File]::ReadAllText($CargoTomlPath)
$cargoContent = $cargoContent -replace "version = `"$currentVersion`"", "version = `"$Version`""
WriteFileNoBom $CargoTomlPath $cargoContent
Write-Host "  Cargo.toml      -> $Version" -ForegroundColor DarkGray

# Step 2: Build
Write-Host ""
Write-Host "[2/4] Building release..." -ForegroundColor Cyan
Push-Location (Join-Path $ProjectRoot "src-tauri")
try {
    cargo tauri build
    if ($LASTEXITCODE -ne 0) { throw "Build failed with exit code $LASTEXITCODE" }
} finally {
    Pop-Location
}

# Step 3: Copy installer to releases folder
Write-Host ""
Write-Host "[3/4] Copying installer..." -ForegroundColor Cyan

$installerName = "Reckon Bolt_${Version}_x64-setup.exe"
$installerSource = Join-Path $ProjectRoot "src-tauri\target\release\bundle\nsis\$installerName"

if (-not (Test-Path $installerSource)) {
    throw "Installer not found at: $installerSource"
}

$versionDir = Join-Path $ReleasesDir $Version
if (-not (Test-Path $versionDir)) {
    New-Item -ItemType Directory -Path $versionDir -Force | Out-Null
}

Copy-Item $installerSource -Destination $versionDir -Force
Write-Host "  -> releases\$Version\$installerName" -ForegroundColor DarkGray

# Step 4: Update versions.json
Write-Host ""
Write-Host "[4/4] Updating versions.json..." -ForegroundColor Cyan

if (Test-Path $VersionsJsonPath) {
    $versionsData = Get-Content $VersionsJsonPath -Raw | ConvertFrom-Json
} else {
    $versionsData = [PSCustomObject]@{
        latest   = ""
        versions = @()
    }
}

$entry = [PSCustomObject]@{
    version   = $Version
    date      = (Get-Date -Format "yyyy-MM-dd")
    installer = $installerName
}

$existingEntry = $versionsData.versions | Where-Object { $_.version -eq $Version }
if ($existingEntry) {
    $versionsData.versions = @($versionsData.versions | ForEach-Object {
        if ($_.version -eq $Version) { $entry } else { $_ }
    })
} else {
    $versionsData.versions = @($versionsData.versions) + $entry
}

$versionsData.latest = $Version
$json = $versionsData | ConvertTo-Json -Depth 10
WriteFileNoBom $VersionsJsonPath $json
Write-Host "  -> versions.json updated (latest: $Version)" -ForegroundColor DarkGray

Write-Host ""
Write-Host "=== Release v$Version complete! ===" -ForegroundColor Green
Write-Host ""
