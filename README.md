# PuceArcade CPS1

Émulateur **Capcom CPS1** cycle-exact en Rust (GUI egui), construit carte par carte par un agent local (Hermes Kanban, profil `rust-dev`).
Priorité : **Ghouls'n Ghosts** (`ghouls`), puis **Street Fighter II** (`sf2`, `sf2ce`, `sf2hf`), puis les autres jeux CPS1 non-QSound.

- CPU : crates de l'auteur, épinglées — [puce-m68k](https://github.com/Farquaad56/CPU-Motorola_68K_Crates_Rust) (68000) et [puce-z80](https://github.com/Farquaad56/CPU-Zilog-Z80_Crates_Rust) (Z80).
- Référence matérielle : MAME (`src/mame/capcom/cps1*.cpp`, copiés dans `ref/mame-cps1/`, BSD-3), ymfm (YM2151), MAME okim6295/okiadpcm (OKI).
- GUI : frontal JAMMA réutilisable [PuceArcade_JammaGui_Crates](https://github.com/Farquaad56/PuceArcade_JammaGui_Crates) (filtres, overlay, manettes, DIP, débogueur).
- Méthode : celle de Puce16 Neo Geo (`docs/METHOD.md`) — specs digérées (`docs/spec/`), cartes autoportantes (`tasks/`), gate unique, oracles.

## Démarrer
1. `copy cps1.example.toml cps1.toml` puis renseigner `roms_dir` (romsets MAME 0.287 : ghouls.zip, sf2.zip, sf2ce.zip, sf2hf.zip).
2. Importer les cartes dans Hermes : `powershell scripts/kanban_import.ps1` (board « cps1 », chaîne linéaire T000 -> T702).
3. Chaque carte se termine par `powershell scripts/gate.ps1`.

## Arborescence
`crates/` (9 crates `cps-*`), `docs/spec/` (00 horloges, 01 cartes mémoire/E-S, 02 vidéo CPS-A/B, 03 audio, 04 romsets), `ref/` (sources de référence en lecture seule),
`tools/` (générateurs de tables, oracles C++, scripts MAME golden), `tasks/` (53 cartes + `board.json` + `INDEX.md`), `tests-data/` (oracles YM2151 et OKI).

## Horloge
1 tick = 1/5,04 GHz (PPCM de 10, 12, 16 et 3,579545 MHz) : voir `docs/adr/001-tick-5040mhz.md`.

Aucune ROM n'est fournie ni versionnée.
