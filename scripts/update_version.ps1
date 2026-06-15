# update_version.ps1 — Bump the project version across all manifest files,
# commit, tag, and push to every configured remote.
#
# Usage:   powershell -ExecutionPolicy Bypass -File scripts/update_version.ps1 0.1.0
#          .\scripts\update_version.ps1 0.1.0
param(
    [Parameter(Mandatory = $true, HelpMessage = "New semver version, e.g. 0.1.0 or 0.1.0-beta.1")]
    [string]$Version
)

$ErrorActionPreference = "Stop"

# ── helpers ──────────────────────────────────────────────────────────────
function Die {
    Write-Host "ERROR: $args" -ForegroundColor Red
    exit 1
}

function IsValidVersion {
    param([string]$v)
    return $v -match '^\d+\.\d+\.\d+(-[A-Za-z0-9.]+)?$'
}

# ── guard ────────────────────────────────────────────────────────────────
if (-not (IsValidVersion $Version)) {
    Die "invalid version '$Version' — expected semver, e.g. 0.1.0 or 0.1.0-beta.1"
}

$RepoRoot = Split-Path -Parent (Split-Path -Parent $PSCommandPath)
Set-Location $RepoRoot

# ── discover current version from package.json ───────────────────────────
$packageJson = [System.IO.File]::ReadAllText((Join-Path $RepoRoot "package.json")) | ConvertFrom-Json
$CurrentVersion = $packageJson.version

if (-not $CurrentVersion) {
    Die "could not determine current version from package.json"
}

if ($CurrentVersion -eq $Version) {
    Die "version $Version is already the current version — nothing to do"
}

Write-Host "Bumping $CurrentVersion → $Version" -ForegroundColor Cyan

# ── update files ─────────────────────────────────────────────────────────
# Use the .NET File API rather than Get-Content/Set-Content: Windows
# PowerShell 5.1 defaults to the system ANSI codepage for text I/O, which
# corrupts non-ASCII bytes (e.g. the Chinese description in package.json), and
# Set-Content -Encoding UTF8 would prepend a BOM that breaks TOML/JSON
# manifests. Set-Location does not sync to .NET's current directory, so read
# and write through absolute paths joined to $RepoRoot.
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)

$files = @(
    "package.json",
    "src-tauri/Cargo.toml",
    "src-tauri/Cargo.lock",
    "src-tauri/tauri.conf.json"
)

foreach ($file in $files) {
    if (-not (Test-Path $file)) {
        Die "expected manifest file not found: $file"
    }
    $fullPath = Join-Path $RepoRoot $file
    $content = [System.IO.File]::ReadAllText($fullPath)
    $escapedCurrent = [regex]::Escape($CurrentVersion)
    # Replace first occurrence of the version string only (Cargo.lock has a
    # top-level entry plus dependency entries; we want the top-level one).
    $newContent = $content -replace $escapedCurrent, $Version
    if ($newContent -eq $content) {
        Die "version '$CurrentVersion' not found in $file"
    }
    [System.IO.File]::WriteAllText($fullPath, $newContent, $utf8NoBom)
    Write-Host "  updated $file"
}

# ── commit & tag ─────────────────────────────────────────────────────────
git add $files

$commitMsg = "chore(release): bump version to $Version"
Write-Host ""
Write-Host "Committing: $commitMsg" -ForegroundColor Green
git commit -m $commitMsg

$tag = "v$Version"
Write-Host "Tagging: $tag" -ForegroundColor Green
git tag -a $tag -m $tag

# ── push to all remotes ──────────────────────────────────────────────────
$remotes = git remote
if (-not $remotes) {
    Die "no git remotes configured — cannot push"
}

$branch = git rev-parse --abbrev-ref HEAD

Write-Host ""
foreach ($remote in $remotes) {
    Write-Host "Pushing $branch → $remote" -ForegroundColor Yellow
    git push $remote $branch

    Write-Host "Pushing tag $tag → $remote" -ForegroundColor Yellow
    git push $remote $tag
}

Write-Host ""
Write-Host "Done — version $Version pushed to all remotes" -ForegroundColor Green
