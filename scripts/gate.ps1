$ErrorActionPreference = "Stop"
function Step($cmd) { Invoke-Expression $cmd; if ($LASTEXITCODE -ne 0) { Write-Host "GATE FAIL : $cmd"; exit 1 } }
Step "cargo fmt --all --check"
Step "cargo clippy --workspace --all-targets -- -D warnings"
Step "cargo test --workspace"
# Fichiers générés : doivent correspondre exactement à leur générateur (jamais édités à la main)
if (Test-Path crates/cps-gfx/src/config_data.rs)   { Step "python tools/gen_cps_tables.py --check" }
if (Test-Path crates/cps-cart/src/romsets_data.rs) { Step "python tools/extract_romsets.py --check" }
if (Test-Path crates/cps-cart/src/romsets2_data.rs) { Step "python tools/extract_romsets.py --cps2 --check" }
if (Test-Path crates/cps-cart/src/cps2crypt_tables.rs) { Step "python tools/gen_cps2crypt_tables.py --check" }
if (Test-Path crates/cps-cart/src/dips_data.rs)    { Step "python tools/extract_dips.py --check" }
if (Test-Path crates/cps-oki/src/tables.rs)        { Step "python tools/gen_oki_tables.py --check" }
if (Test-Path tools/check_metrics.ps1) { Step "powershell -NoProfile -File tools/check_metrics.ps1" }
Write-Host "GATE OK"
