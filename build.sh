#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

PROFILE="debug"
if [[ "${1:-}" == "--release" ]] || [[ "${1:-}" == "-r" ]]; then
    PROFILE="release"
fi

if [[ "$PROFILE" == "release" ]]; then
    cargo build --target wasm32-wasip2 --release
else
    cargo build --target wasm32-wasip2
fi

if [ "$PROFILE" == "debug" ]; then
    SRC="target/wasm32-wasip2/debug/pumpkinplus.wasm"
    DST=".server/plugins/pumpkinplus.wasm"
    cp "$SRC" "$DST"
    echo "Copied $SRC -> $DST"
fi
