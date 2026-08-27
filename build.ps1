param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

# Ensure the latest Pumpkin server executable is available for local testing.
# This fetches the matching nightly binary from GitHub releases.
$serverDir = ".server"
New-Item -ItemType Directory -Force -Path $serverDir | Out-Null

$os = "Windows"
$arch = if ([System.Environment]::Is64BitOperatingSystem) { "x64" } else { "ARM64" }
$asset = switch ($os) {
    "Windows" {
        switch ($arch) {
            "x64" { "pumpkin-X64-Windows.exe" }
            "ARM64" { "pumpkin-ARM64-Windows.exe" }
        }
    }
}

if ($asset) {
    $pumpkinBin = Join-Path $serverDir $asset
    Write-Host "Fetching latest Pumpkin nightly binary: $asset"
    Invoke-RestMethod -Uri "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/$asset" -OutFile $pumpkinBin
    Write-Host "Downloaded $pumpkinBin"
} else {
    Write-Host "Warning: no nightly Pumpkin binary available for $os $arch; skipping server fetch"
}

if ($Release) {
    cargo build --release --target wasm32-wasip2
} else {
    cargo build --target wasm32-wasip2
    $src = "target/wasm32-wasip2/debug/pumpkinplus.wasm"
    $dst = ".server/plugins/pumpkinplus.wasm"
    New-Item -ItemType Directory -Force -Path ".server/plugins" | Out-Null
    Copy-Item $src $dst -Force
    Write-Host "Copied $src -> $dst"
}
