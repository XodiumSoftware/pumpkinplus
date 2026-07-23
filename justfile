# pumpkinplus task runner
# Install `just` once: https://github.com/casey/just

_default:
    @just --list

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

# Build with debug info and copy to .server/plugins/ (Windows)
build-copy:
    powershell -ExecutionPolicy Bypass -File build.ps1

# Build release and copy to .server/plugins/ (Windows)
build-copy-release:
    powershell -ExecutionPolicy Bypass -File build.ps1 -Release

# Generate rustdoc for the WASI target
doc:
    cargo doc --no-deps --target wasm32-wasip2

# Run the full validation suite used in CI
validate: lint fmt-check build
