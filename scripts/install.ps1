<# 
.SYNOPSIS
    Universal installer for Soma AI on Windows

.DESCRIPTION
    Downloads and installs the latest Soma AI release for Windows.
    Usage: irm https://raw.githubusercontent.com/skbhati199/soma-ai/main/scripts/install.ps1 | iex

.PARAMETER Version
    Version to install (default: latest)

.PARAMETER InstallDir
    Installation directory (default: $env:USERPROFILE\.soma\bin)
#>

param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.soma\bin"
)

$REPO = "skbhati199/soma-ai"
$ProgressPreference = 'SilentlyContinue'

function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Green }
function Write-Warn { param($msg) Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Error { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }

function Get-LatestVersion {
    $api = "https://api.github.com/repos/$REPO/releases/latest"
    $response = Invoke-RestMethod -Uri $api -Headers @{Accept = "application/vnd.github.v3+json"}
    return $response.tag_name
}

function Download-And-Install {
    param($version)
    
    $assetName = "soma-x86_64-pc-windows-msvc.zip"
    $url = "https://github.com/$REPO/releases/download/$version/$assetName"
    
    Write-Info "Downloading $assetName..."
    Write-Info "From: $url"
    
    $tmpDir = [System.IO.Path]::GetTempPath()
    $zipPath = Join-Path $tmpDir $assetName
    
    try {
        Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing
    } catch {
        Write-Error "Failed to download $assetName: $_"
        exit 1
    }
    
    Write-Info "Extracting..."
    $extractDir = Join-Path $tmpDir "soma-extract"
    if (Test-Path $extractDir) { Remove-Item $extractDir -Recurse -Force }
    Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force
    
    Write-Info "Installing to $InstallDir..."
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item (Join-Path $extractDir "soma.exe") (Join-Path $InstallDir "soma.exe") -Force
    
    Write-Info "✓ Installed soma.exe to $InstallDir"
}

# Main
Write-Info "Installing Soma AI..."

# Get version
if ($Version -eq "latest") {
    $Version = Get-LatestVersion
    Write-Info "Latest version: $Version"
}

# Download and install
Download-And-Install $Version

# Verify
if (Get-Command soma.exe -ErrorAction SilentlyContinue) {
    Write-Info "✓ Installation verified: $(soma.exe --version)"
} else {
    Write-Warn "soma.exe not in PATH. Add $InstallDir to your PATH:"
    Write-Warn "  \$env:PATH += ';$InstallDir'"
    Write-Warn "Or run: [Environment]::SetEnvironmentVariable('PATH', \$env:PATH + ';$InstallDir', 'User')"
}

Write-Info ""
Write-Info "Next steps:"
Write-Info "  1. Restart your shell or run: . \$PROFILE"
Write-Info "  2. Install shell integration: soma install --shell pwsh"
Write-Info "  3. Try: soma ask 'list all files'"
