#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

PROFILE="debug"
if [[ "${1:-}" == "--release" ]] || [[ "${1:-}" == "-r" ]]; then
    PROFILE="release"
fi

# Ensure the latest Pumpkin server executable is available for local testing.
# This fetches the matching nightly binary from GitHub releases.
PUMPKIN_DIR=".server"
mkdir -p "$PUMPKIN_DIR"

OS=$(uname -s)
ARCH=$(uname -m)
ASSET=""
case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64) ASSET="pumpkin-X64-Linux" ;;
            aarch64) ASSET="pumpkin-ARM64-Linux" ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64) ASSET="pumpkin-X64-macOS" ;;
            arm64) ASSET="pumpkin-ARM64-macOS" ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
        case "$ARCH" in
            x86_64|AMD64) ASSET="pumpkin-X64-Windows.exe" ;;
            ARM64|aarch64) ASSET="pumpkin-ARM64-Windows.exe" ;;
        esac
        ;;
esac

if [[ -n "$ASSET" ]]; then
    PUMPKIN_BIN="$PUMPKIN_DIR/$ASSET"
    echo "Fetching latest Pumpkin nightly binary: $ASSET"
    curl -fsSL -L "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/$ASSET" -o "$PUMPKIN_BIN"
    chmod +x "$PUMPKIN_BIN"
    echo "Downloaded $PUMPKIN_BIN"
else
    echo "Warning: no nightly Pumpkin binary available for $OS $ARCH; skipping server fetch"
fi

if [[ "$PROFILE" == "release" ]]; then
    cargo build --target wasm32-wasip2 --release
else
    cargo build --target wasm32-wasip2
fi

if [ "$PROFILE" == "debug" ]; then
    SRC="target/wasm32-wasip2/debug/pumpkinplus.wasm"
    DST=".server/plugins/pumpkinplus.wasm"
    mkdir -p ".server/plugins"
    cp "$SRC" "$DST"
    echo "Copied $SRC -> $DST"
fi
