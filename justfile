# Install `just` once: https://github.com/casey/just

[windows]
set shell := ["powershell.exe", "-NoProfile", "-Command"]

_default:
    @just --list

lint:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic -D warnings

fmt:
    cargo fmt --all

build:
    cargo build --release

[unix]
build-copy: build
    mkdir -p .server/plugins
    sh -c 'set -- target/wasm32-wasip3/release/*.wasm; if [ "$#" -ne 1 ]; then echo "expected exactly one *.wasm in target/wasm32-wasip3/release, found $#" >&2; exit 1; fi; cp "$1" .server/plugins/; echo "Copied $1 -> .server/plugins/"'

[windows]
build-copy: build
    New-Item -ItemType Directory -Force -Path .server/plugins | Out-Null; & { $wasm = @(Get-ChildItem target/wasm32-wasip3/release/*.wasm); if ($wasm.Count -ne 1) { throw "expected exactly one *.wasm in target/wasm32-wasip3/release, found $($wasm.Count)" }; Copy-Item $wasm[0].FullName .server/plugins/ -Force; Write-Host "Copied $($wasm[0].FullName) -> .server/plugins/$($wasm[0].Name)" }

doc:
    cargo doc --no-deps

validate: lint fmt build
