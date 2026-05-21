param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

if ($Release) {
    cargo build --release --target wasm32-wasip2
} else {
    cargo build --target wasm32-wasip2
    $src = "target/wasm32-wasip2/debug/pumpkinplus.wasm"
    $dst = ".server/plugins/pumpkinplus.wasm"
    Copy-Item $src $dst -Force
    Write-Host "Copied $src -> $dst"
}
