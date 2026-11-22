# T604 — golden MAME (trames 600 et 1500) pour chaque jeu jouable présent dans roms_dir.
# Exemple : powershell tools/mame_golden_batch.ps1            (saute les jeux déjà capturés)
#           powershell tools/mame_golden_batch.ps1 -Force     (recapture tout)
# Les PLD manquants (fichiers de moins de 1 Kio que MAME 0.287 exige mais n'utilise pas) sont créés
# à zéro dans tests-data/golden/_pld_stubs/<romset>/ (voir tools/README-golden.md).
param(
  [int[]]$Frames = @(600, 1500),
  [string]$Mame = "X:\Mame_0287\mame.exe",
  [switch]$Force
)
$ErrorActionPreference = "Stop"
$root = (Get-Location).Path
$roms = (Select-String -Path "cps1.toml" -Pattern '^\s*roms_dir\s*=\s*"(.*)"' | Select-Object -First 1).Matches[0].Groups[1].Value
$stubs = Join-Path $root "tests-data\golden\_pld_stubs"
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
  $ErrorActionPreference = "Continue"
  $v = & $Mame -rompath $roms -verifyroms $g 2>&1 | Out-String
  $ErrorActionPreference = "Stop"
  foreach ($l in $v.Split("`n")) {
    if ($l -match '^\S+\s*:\s*(\S+)\s+\((\d+) bytes\) - NOT FOUND(?:\s+\((\w+)\))?' -and [int]$matches[2] -lt 1024) {
      $set = if ($matches[3]) { $matches[3] } else { $g }
      $d = Join-Path $stubs $set
      New-Item -ItemType Directory -Force $d | Out-Null
      [IO.File]::WriteAllBytes((Join-Path $d $matches[1]), (New-Object byte[] ([int]$matches[2])))
    }
  }
  & .\tools\mame_golden.ps1 -Game $g -Frames $Frames -Out $out   # en processus : -File ne passe pas un int[]
}
