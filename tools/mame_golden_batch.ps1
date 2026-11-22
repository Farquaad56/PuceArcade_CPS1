# T604 — golden MAME (trames 600 et 1500) pour chaque jeu jouable présent dans roms_dir.
# Exemple : powershell tools/mame_golden_batch.ps1            (saute les jeux déjà capturés)
#           powershell tools/mame_golden_batch.ps1 -Force     (recapture tout)
param(
  [int[]]$Frames = @(600, 1500),
  [switch]$Force
)
$ErrorActionPreference = "Stop"
# Liste des jeux : test list_games (jeux de supported_games() dont les ROM sont présentes)
cargo test -q -p cps-core --release --test catalogue -- --ignored list_games | Out-Null
if (-not (Test-Path "target/games.txt")) { throw "target/games.txt absent (cps1.toml ?)" }
$games = Get-Content "target/games.txt" | Where-Object { $_ -ne "" }
$i = 0
foreach ($g in $games) {
  $i++
  $out = "tests-data/golden/$g"
  $missing = $Frames | Where-Object { -not (Test-Path ("$out/f{0:D6}.png" -f $_)) }
  if (-not $Force -and -not $missing) { continue }
  Write-Host ("[{0}/{1}] {2}" -f $i, $games.Count, $g)
  & .\tools\mame_golden.ps1 -Game $g -Frames $Frames -Out $out   # en processus : -File ne passe pas un int[]
}
