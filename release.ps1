<#
.SYNOPSIS
    Build a versioned release of the Reckon Bolt app.

.DESCRIPTION
    Updates version numbers in tauri.conf.json and Cargo.toml, runs the Tauri
    build, copies the resulting installers into releases/<version>/, and appends
    an entry to versions.json.

.PARAMETER Version
    Semantic version string for the new release (e.g. "1.0.0").

.PARAMETER SkipBuild
    If set, skip the Tauri build step (useful for testing the script itself).

.PARAMETER GitTag
    If set, create a git tag for the release version.

.EXAMPLE
    .\release.ps1 -Version 1.0.0
    .\release.ps1 -Version 1.2.0 -GitTag
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$Version,

    [switch]$SkipBuild,
    [switch]$GitTag
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = $PSScriptRoot
$tauriDir = Join-Path $root "src-tauri"
$tauriConf = Join-Path $tauriDir "tauri.conf.json"
$cargoToml = Join-Path $tauriDir "Cargo.toml"
$versionsFile = Join-Path $root "versions.json"
$releasesDir = Join-Path $root "releases"

# ── Validate version format ──────────────────────────────────────────────────
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Version must be in semver format: MAJOR.MINOR.PATCH (e.g. 1.0.0)"
    exit 1
}

# ── Check for duplicate version ──────────────────────────────────────────────
if (Test-Path $versionsFile) {
    $existing = Get-Content $versionsFile -Raw | ConvertFrom-Json
    if ($existing.versions | Where-Object { $_.version -eq $Version }) {
        Write-Error "Version $Version already exists in versions.json. Choose a new version."
        exit 1
    }
}

Write-Host "`n=== Reckon Bolt Release v$Version ===" -ForegroundColor Cyan

# ── 1. Update tauri.conf.json ────────────────────────────────────────────────
Write-Host "`n[1/5] Updating tauri.conf.json ..." -ForegroundColor Yellow
$confJson = Get-Content $tauriConf -Raw | ConvertFrom-Json
$oldVersion = $confJson.version
$confJson.version = $Version
$confJson | ConvertTo-Json -Depth 10 | Set-Content $tauriConf -Encoding UTF8
Write-Host "      $oldVersion -> $Version"

# ── 2. Update Cargo.toml ────────────────────────────────────────────────────
Write-Host "[2/5] Updating Cargo.toml ..." -ForegroundColor Yellow
$cargoContent = Get-Content $cargoToml -Raw
$cargoContent = $cargoContent -replace '(?m)^(version\s*=\s*")[\d\.]+(")', "`${1}$Version`${2}"
Set-Content $cargoToml -Value $cargoContent -Encoding UTF8 -NoNewline
Write-Host "      version = `"$Version`""

# ── 3. Build ─────────────────────────────────────────────────────────────────
if ($SkipBuild) {
    Write-Host "[3/5] Skipping build (SkipBuild flag set)" -ForegroundColor DarkGray
} else {
    Write-Host "[3/5] Building Tauri app (this may take a few minutes) ..." -ForegroundColor Yellow
    Push-Location $root
    try {
        $prevPref = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        cargo tauri build 2>&1 | ForEach-Object { Write-Host "      $_" }
        $ErrorActionPreference = $prevPref
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Tauri build failed with exit code $LASTEXITCODE"
            exit 1
        }
    } finally {
        Pop-Location
    }
}

# ── 4. Collect installer into releases/<version>/ ────────────────────────────
Write-Host "[4/5] Collecting release artifacts ..." -ForegroundColor Yellow
$versionDir = Join-Path $releasesDir $Version
if (-not (Test-Path $versionDir)) {
    New-Item -ItemType Directory -Path $versionDir -Force | Out-Null
}

$bundleDir = Join-Path $tauriDir "target\release\bundle"
$productName = $confJson.productName
$installerName = "${productName}_${Version}_x64-setup.exe"
$installerFound = $false

if (Test-Path $bundleDir) {
    $setupExe = Get-ChildItem -Path $bundleDir -Recurse -File |
        Where-Object { $_.Name -like "*_x64-setup.exe" } |
        Select-Object -First 1

    if ($setupExe) {
        $dest = Join-Path $versionDir $installerName
        Copy-Item $setupExe.FullName -Destination $dest -Force
        $installerFound = $true
        Write-Host "      -> $installerName ($([math]::Round($setupExe.Length / 1MB, 2)) MB)"
    }
}

if (-not $installerFound) {
    Write-Warning "No installer artifact found in $bundleDir"
}

# ── 5. Update versions.json ──────────────────────────────────────────────────
Write-Host "[5/5] Updating versions.json ..." -ForegroundColor Yellow

$entry = [ordered]@{
    version   = $Version
    date      = (Get-Date -Format "yyyy-MM-dd")
    installer = $installerName
}

if ((Test-Path $versionsFile) -and ((Get-Content $versionsFile -Raw).Trim() -ne "")) {
    $versionsData = Get-Content $versionsFile -Raw | ConvertFrom-Json
} else {
    $versionsData = [PSCustomObject]@{
        latest   = ""
        versions = @()
    }
}

$versionsData.latest = $Version
$versionsList = [System.Collections.ArrayList]@($versionsData.versions)
$versionsList.Add([PSCustomObject]$entry) | Out-Null
$versionsData.versions = $versionsList.ToArray()

$versionsData | ConvertTo-Json -Depth 10 | Set-Content $versionsFile -Encoding UTF8
Write-Host "      Recorded version $Version in versions.json (latest: $Version)"

# ── Optional: git tag ────────────────────────────────────────────────────────
if ($GitTag) {
    Write-Host "`nCreating git tag v$Version ..." -ForegroundColor Yellow
    git tag "v$Version"
    Write-Host "      Tagged as v$Version (use 'git push --tags' to push)"
}

Write-Host "`n=== Release v$Version complete! ===" -ForegroundColor Green
if ($installerFound) {
    Write-Host "Installer saved to: $versionDir\$installerName" -ForegroundColor Green
}
Write-Host ""
