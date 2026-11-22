# Régénère les oracles audio (déjà versionnés dans tests-data/). Requiert g++ (MSYS2/MinGW) ou WSL.
$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path tests-data/ym2151, tests-data/oki, target/oracles | Out-Null
g++ -O2 -std=c++14 -Iref/ymfm/src tools/ymfm_opm_golden.cpp ref/ymfm/src/ymfm_opm.cpp -o target/oracles/ymfm_opm_golden.exe
if ($LASTEXITCODE -ne 0) { throw "compilation ymfm_opm_golden" }
& target/oracles/ymfm_opm_golden.exe
g++ -O2 -std=c++14 -Iref/ymfm/src tools/ymfm_opm_fuzz.cpp ref/ymfm/src/ymfm_opm.cpp -o target/oracles/ymfm_opm_fuzz.exe
if ($LASTEXITCODE -ne 0) { throw "compilation ymfm_opm_fuzz" }
& target/oracles/ymfm_opm_fuzz.exe
g++ -O2 -std=c++14 -Itools/oki_shim -Iref/mame-sound tools/oki_golden.cpp ref/mame-sound/okiadpcm.cpp -o target/oracles/oki_golden.exe
if ($LASTEXITCODE -ne 0) { throw "compilation oki_golden" }
& target/oracles/oki_golden.exe
Write-Host "Oracles régénérés : vérifier git diff tests-data/ (doit être vide)"
