# pumpkinplus task runner
# Install `just` once: https://github.com/casey/just

set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

_default:
    @just --list

# ── Pumpkin nightly asset detection (pure `just` functions, cross-platform) ──

_arch-tag := replace(replace(arch(), "x86_64", "X64"), "aarch64", "ARM64")
_os-tag := replace(replace(replace(os(), "windows", "Windows"), "linux", "Linux"), "macos", "macOS")
_ext := if os() == "windows" { ".exe" } else { "" }
_asset := "pumpkin-" + _arch-tag + "-" + _os-tag + _ext
_url := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/" + _asset
_api_url := "https://api.github.com/repos/Pumpkin-MC/Pumpkin/releases/tags/nightly"
_version_file := ".server/.pumpkin-version"

# Fetch the latest Pumpkin nightly server binary into .server/
[unix]
fetch-server:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p .server
    latest=$(curl -fsSL "{{_api_url}}" | grep -o '"published_at": *"[^"]*"' | cut -d'"' -f4)
    if [[ -f "{{_version_file}}" ]] && [[ "$(cat {{_version_file}})" == "$latest" ]]; then echo "Pumpkin nightly is up to date ($latest)"; else echo "Fetching latest Pumpkin nightly binary: {{_asset}}"; curl -fsSL "{{_url}}" -o ".server/{{_asset}}"; chmod +x ".server/{{_asset}}"; echo "$latest" > "{{_version_file}}"; echo "Downloaded .server/{{_asset}}"; fi

[windows]
fetch-server:
    New-Item -ItemType Directory -Force -Path .server | Out-Null; $latest = (Invoke-RestMethod -Uri "{{_api_url}}").published_at; $versionFile = "{{_version_file}}"; if ((Test-Path $versionFile) -and ((Get-Content $versionFile -Raw).Trim() -eq $latest)) { Write-Host "Pumpkin nightly is up to date ($latest)" } else { Write-Host "Fetching latest Pumpkin nightly binary: {{_asset}}"; Invoke-RestMethod -Uri "{{_url}}" -OutFile ".server/{{_asset}}"; Set-Content -Path $versionFile -Value $latest; Write-Host "Downloaded .server/{{_asset}}" }

# Lint with pedantic lints enabled and warnings as errors
lint:
    cargo clippy --all-targets --all-features --target wasm32-wasip2 -- -W clippy::pedantic -D warnings

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Format the project
fmt:
    cargo fmt --all

# Build the release WASM plugin
build:
    cargo build --release --target wasm32-wasip2

# Build release WASM, fetch the Pumpkin server, and copy the plugin to .server/plugins/
#
# NOTE: Debug builds fail to load in the Pumpkin runtime due to a wasmtime
# locals limit in the wit-bindgen `handle-event` export. Always use release.
[unix]
build-copy: fetch-server
    cargo build --release --target wasm32-wasip2
    mkdir -p .server/plugins
    cp target/wasm32-wasip2/release/pumpkinplus.wasm .server/plugins/pumpkinplus.wasm
    echo "Copied target/wasm32-wasip2/release/pumpkinplus.wasm -> .server/plugins/pumpkinplus.wasm"

[windows]
build-copy: fetch-server
    cargo build --release --target wasm32-wasip2
    New-Item -ItemType Directory -Force -Path .server/plugins | Out-Null
    Copy-Item target/wasm32-wasip2/release/pumpkinplus.wasm .server/plugins/pumpkinplus.wasm -Force
    Write-Host "Copied target/wasm32-wasip2/release/pumpkinplus.wasm -> .server/plugins/pumpkinplus.wasm"

# Fetch the Pumpkin server and build the release WASM plugin
build-copy-release: fetch-server build

# Generate rustdoc for the WASI target
doc:
    cargo doc --no-deps --target wasm32-wasip2

# Run the full validation suite used in CI
validate: lint fmt-check build
