# pumpkinplus task runner
# Install `just` once: https://github.com/casey/just

_default:
    @just --list

# ── Pumpkin nightly asset detection (pure `just` functions, cross-platform) ──

_arch-tag := replace(replace(arch(), "x86_64", "X64"), "aarch64", "ARM64")
_os-tag := replace(replace(replace(os(), "windows", "Windows"), "linux", "Linux"), "macos", "macOS")
_ext := if os() == "windows" { ".exe" } else { "" }
_asset := "pumpkin-" + _arch-tag + "-" + _os-tag + _ext
_url := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/" + _asset
_supported := if os() + "-" + arch() =~ "^(windows|linux|macos)-(x86_64|aarch64)$" { "true" } else { "false" }

# Fetch the latest Pumpkin nightly server binary into .server/
[unix]
fetch-server:
    #!/usr/bin/env sh
    set -eu
    if [ "{{_supported}}" = "false" ]; then
        echo "Warning: no nightly Pumpkin binary available for $(uname -s) $(uname -m); skipping server fetch"
        exit 0
    fi
    mkdir -p .server
    echo "Fetching latest Pumpkin nightly binary: {{_asset}}"
    curl -fsSL "{{_url}}" -o ".server/{{_asset}}"
    chmod +x ".server/{{_asset}}"
    echo "Downloaded .server/{{_asset}}"

[windows]
fetch-server:
    #!/usr/bin/env powershell
    $ErrorActionPreference = "Stop"
    if ("{{_supported}}" -eq "false") {
        Write-Host "Warning: no nightly Pumpkin binary available for $env:OS $env:PROCESSOR_ARCHITECTURE; skipping server fetch"
        exit 0
    }
    New-Item -ItemType Directory -Force -Path .server | Out-Null
    Write-Host "Fetching latest Pumpkin nightly binary: {{_asset}}"
    Invoke-RestMethod -Uri "{{_url}}" -OutFile ".server/{{_asset}}"
    Write-Host "Downloaded .server/{{_asset}}"

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

# Build debug WASM, fetch the Pumpkin server, and copy the plugin to .server/plugins/
[unix]
build-copy: fetch-server
    #!/usr/bin/env sh
    set -eu
    cargo build --target wasm32-wasip2
    mkdir -p .server/plugins
    cp target/wasm32-wasip2/debug/pumpkinplus.wasm .server/plugins/pumpkinplus.wasm
    echo "Copied target/wasm32-wasip2/debug/pumpkinplus.wasm -> .server/plugins/pumpkinplus.wasm"

[windows]
build-copy: fetch-server
    #!/usr/bin/env powershell
    $ErrorActionPreference = "Stop"
    cargo build --target wasm32-wasip2
    New-Item -ItemType Directory -Force -Path .server/plugins | Out-Null
    Copy-Item target/wasm32-wasip2/debug/pumpkinplus.wasm .server/plugins/pumpkinplus.wasm -Force
    Write-Host "Copied target/wasm32-wasip2/debug/pumpkinplus.wasm -> .server/plugins/pumpkinplus.wasm"

# Fetch the Pumpkin server and build the release WASM plugin
build-copy-release: fetch-server build

# Generate rustdoc for the WASI target
doc:
    cargo doc --no-deps --target wasm32-wasip2

# Run the full validation suite used in CI
validate: lint fmt-check build
