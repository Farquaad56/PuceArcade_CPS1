# Génère les captures de référence (golden frames) avec MAME.
# Exemple : powershell tools/mame_golden.ps1 -Game ghouls -Frames 120,600,1500,3000 -Out tests-data/golden/ghouls
param(
  [Parameter(Mandatory=$true)][string]$Game,
  [Parameter(Mandatory=$true)][int[]]$Frames,
  [Parameter(Mandatory=$true)][string]$Out,
  [string]$Mame = "X:\Mame_0287\mame.exe",
  [string]$RomPath = "",          # dossier des ROMs ; par défaut : celui de cps1.toml (paths.roms_dir)
  [switch]$Show                   # retire -video none si les captures sortent vides
)
$ErrorActionPreference = "Stop"
if (-not (Test-Path $Mame)) { throw "MAME introuvable : $Mame" }
if ($RomPath -eq "" -and (Test-Path "cps1.toml")) {
  $m = Select-String -Path "cps1.toml" -Pattern '^\s*roms_dir\s*=\s*"(.*)"' | Select-Object -First 1
  if ($m) { $RomPath = $m.Matches[0].Groups[1].Value }
}
if ($RomPath -eq "") { throw "Indiquer -RomPath ou renseigner paths.roms_dir dans cps1.toml" }
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$outAbs = (Resolve-Path $Out).Path
$env:CPS_PREFIX = "f"
$lua = (Resolve-Path "tools/mame_snap.lua").Path
# Démarrage à froid reproductible : configuration MAME vierge (cfg/nvram), hors du dossier de l'utilisateur.
# Une exécution MAME par capture : une capture (ou toute interaction Lua) en cours de route perturbe
# l'émulation de MAME . Une exécution
# par trame donne la référence « non perturbée », seule comparable à un émulateur déterministe.
$ErrorActionPreference = "Continue"
foreach ($frame in $Frames) {
  $fresh = Join-Path ([System.IO.Path]::GetTempPath()) ("mame_fresh_" + $Game)
  if (Test-Path $fresh) { Remove-Item -Recurse -Force $fresh }
  New-Item -ItemType Directory -Force -Path (Join-Path $fresh "nvram"), (Join-Path $fresh "cfg") | Out-Null
  $env:CPS_FRAMES = "$frame"
  $args = @($Game, "-rompath", $RomPath, "-sound", "none", "-nothrottle", "-skip_gameinfo",
            "-nvram_directory", (Join-Path $fresh "nvram"), "-cfg_directory", (Join-Path $fresh "cfg"),
            "-snapshot_directory", $outAbs, "-snapname", "%i", "-autoboot_script", $lua)
  if (-not $Show) { $args += @("-video", "none") }
  # MAME écrit ses avertissements sur stderr : ne pas les traiter comme des erreurs bloquantes.
  & $Mame @args 2>&1 | Out-Null
}
$ErrorActionPreference = "Stop"
# MAME range les captures dans <snapshot_directory>/<jeu>/ : les remonter.
Get-ChildItem -Path (Join-Path $outAbs $Game) -Filter "*.png" -ErrorAction SilentlyContinue |
  ForEach-Object { Move-Item -Force $_.FullName (Join-Path $outAbs $_.Name) }
Get-ChildItem $outAbs -Filter "f*.png" | Select-Object Name, Length
