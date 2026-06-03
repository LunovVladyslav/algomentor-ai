#!/usr/bin/env pwsh
# setup-gui.ps1  —  Build AlgoMentor portable desktop app
# Run once (or after code changes) from the project root.
# Requires: Rust/Cargo, Node.js (for Tauri CLI)
# Usage:  .\setup-gui.ps1
#         .\setup-gui.ps1 -Release   (optimised build)

param([switch]$Release)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$root = $PSScriptRoot

Write-Host ""
Write-Host "  ╔═══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "  ║   AlgoMentor GUI — build script           ║" -ForegroundColor Cyan
Write-Host "  ╚═══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# ── Helper functions ──────────────────────────────────────────
function Say($msg, $color = "White") { Write-Host "  · $msg" -ForegroundColor $color }
function Ok ($msg)                   { Write-Host "  ✓ $msg" -ForegroundColor Green  }
function Err($msg)                   { Write-Host "  ✗ $msg" -ForegroundColor Red; exit 1 }
function Step($msg)                  { Write-Host "`n  ── $msg" -ForegroundColor Cyan }

# ── 1. Check prerequisites ───────────────────────────────────
Step "Checking prerequisites"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) { Err "Rust/Cargo not found. Install from https://rustup.rs" }
Ok "Rust $(cargo --version)"

if (-not (Get-Command node -ErrorAction SilentlyContinue)) { Err "Node.js not found. Install from https://nodejs.org" }
Ok "Node $(node --version)"

# ── 2. Install Tauri CLI (via cargo if missing) ───────────────
Step "Tauri CLI"
if (-not (Get-Command cargo-tauri -ErrorAction SilentlyContinue)) {
    Say "Installing tauri-cli via cargo (one-time, ~2 min)…"
    cargo install tauri-cli --version "^2" --locked
    Ok "tauri-cli installed"
} else {
    Ok "tauri-cli already installed ($(cargo tauri --version))"
}

# ── 3. Generate icons ─────────────────────────────────────────
Step "Generating icons"
$iconsDir = Join-Path $root "src-tauri\icons"
New-Item -ItemType Directory -Force -Path $iconsDir | Out-Null

function New-AlgoMentorIcon {
    param([int]$Size, [string]$OutPath)
    try {
        Add-Type -AssemblyName PresentationCore, WindowsBase
        $dv  = New-Object System.Windows.Media.DrawingVisual
        $ctx = $dv.RenderOpen()

        # Background: #0d1117
        $bg = New-Object System.Windows.Media.SolidColorBrush(
            [System.Windows.Media.Color]::FromRgb(13, 17, 23))
        # Accent: #58a6ff
        $ac = New-Object System.Windows.Media.SolidColorBrush(
            [System.Windows.Media.Color]::FromRgb(88, 166, 255))

        $rect = New-Object System.Windows.Rect(0, 0, $Size, $Size)
        $ctx.DrawRectangle($bg, $null, $rect)

        $tf   = New-Object System.Windows.Media.Typeface("Segoe UI")
        $fSz  = $Size * 0.52
        $ft   = New-Object System.Windows.Media.FormattedText(
            "A",
            [System.Globalization.CultureInfo]::InvariantCulture,
            [System.Windows.FlowDirection]::LeftToRight,
            $tf, $fSz, $ac, 1.0)
        $x = ($Size - $ft.Width)  / 2
        $y = ($Size - $ft.Height) / 2 - ($Size * 0.04)
        $ctx.DrawText($ft, (New-Object System.Windows.Point($x, $y)))
        $ctx.Close()

        $bmp = New-Object System.Windows.Media.Imaging.RenderTargetBitmap(
            $Size, $Size, 96, 96,
            [System.Windows.Media.PixelFormats]::Pbgra32)
        $bmp.Render($dv)

        $enc = New-Object System.Windows.Media.Imaging.PngBitmapEncoder
        $enc.Frames.Add([System.Windows.Media.Imaging.BitmapFrame]::Create($bmp))
        $fs = [System.IO.FileStream]::new($OutPath, [System.IO.FileMode]::Create)
        $enc.Save($fs)
        $fs.Close()
        return $true
    } catch {
        return $false
    }
}

$generated = $true
foreach ($sz in @(32, 128, 256)) {
    $out = Join-Path $iconsDir "${sz}x${sz}.png"
    if (-not (Test-Path $out)) {
        $ok = New-AlgoMentorIcon -Size $sz -OutPath $out
        if ($ok) { Say "Icon ${sz}x${sz} created" } else { $generated = $false }
    } else {
        Say "Icon ${sz}x${sz} already exists"
    }
}

# icon.png — copy from 128x128
$iconMain = Join-Path $iconsDir "icon.png"
if (-not (Test-Path $iconMain)) {
    $src128 = Join-Path $iconsDir "128x128.png"
    if (Test-Path $src128) { Copy-Item $src128 $iconMain }
}

if ($generated) { Ok "Icons ready" }
else { Say "Icon generation failed (WPF not available?). Add icons manually to src-tauri\icons\" "Yellow" }

# ── 4. Build ──────────────────────────────────────────────────
Step "Building AlgoMentor GUI"
Push-Location $root
try {
    if ($Release) {
        Say "Building RELEASE (optimised, ~5–10 min)…"
        cargo tauri build --no-bundle
        $exePath = Join-Path $root "target\release\algomentor-gui.exe"
    } else {
        Say "Building DEBUG (fast)…"
        cargo build -p algomentor-gui
        $exePath = Join-Path $root "target\debug\algomentor-gui.exe"
    }
} finally {
    Pop-Location
}

# ── 5. Result ─────────────────────────────────────────────────
if (Test-Path $exePath) {
    $sizeMB = [math]::Round((Get-Item $exePath).Length / 1MB, 1)
    Write-Host ""
    Ok "Build complete!  (${sizeMB} MB)"
    Write-Host ""
    Write-Host "  Executable:" -ForegroundColor Cyan
    Write-Host "  $exePath"
    Write-Host ""
    Write-Host "  To create a Desktop shortcut, run:" -ForegroundColor Cyan
    Write-Host "  .\setup-gui.ps1 -CreateShortcut" -ForegroundColor White

    # Optional: create Desktop shortcut
    $desktopLink = Join-Path ([Environment]::GetFolderPath("Desktop")) "AlgoMentor.lnk"
    $wsh = New-Object -ComObject WScript.Shell
    $sc  = $wsh.CreateShortcut($desktopLink)
    $sc.TargetPath       = $exePath
    $sc.WorkingDirectory = Split-Path $exePath
    $sc.Description      = "AlgoMentor — AI Coding Mentor"
    $sc.Save()
    Ok "Desktop shortcut created: $desktopLink"
    Write-Host ""
} else {
    Err "Build failed — exe not found at: $exePath"
}
