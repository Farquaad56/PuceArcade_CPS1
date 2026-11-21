# Architecture

## Horloge (ADR-001)
1 tick = 1/5 040 000 000 s (PPCM entier de 10 MHz, 12 MHz, 16 MHz et 315/88 MHz). Voir `docs/spec/00-clock-timing.md`.
68000 : 504 ticks/horloge (10 MHz) ou 420 (12 MHz, selon le jeu). Z80 et YM2151 : 1408. OKI : 5040. Pixel : 630. Trame : 84 510 720 ticks (59,637 Hz).

## Workspace
```
PuceArcade_CPS1/
  crates/
    cps-common   Tick + constantes d'horloge, Framebuffer 384×224, entrées, CRC   (INTERFACES GELÉES)
    cps-gfx      vidéo CPS-A/CPS-B : config par jeu (généré), mapper, décodage tuiles, palette, tilemaps, sprites, priorités, composition
    cps-ym2151   YM2151 (port ymfm OPM, moteur FM repris de Puce16 Neo Geo)
    cps-oki      OKI MSM6295 (port MAME okim6295 + okiadpcm)
    cps-cart     romsets (généré depuis cps1.cpp), chargement zip/dossier, construction des régions, configuration cps1.toml
    cps-bus      carte mémoire 68000, E/S, latches, ports CPS-A/CPS-B ; carte son Z80 (SoundBoard : Z80 + YM + OKI)
    cps-core     Machine, ordonnanceur (68000 maître), interruptions, trame, audio, savestates, débogage
    cps-testkit  comparateur d'images golden (PNG MAME) et utilitaires d'oracle
    cps-gui      adaptateur JAMMA (Cps1Board, Cps1Driver) + binaire qui lance le frontal puce-jamma-gui (ADR-002)
  tools/        générateurs (tables CPS-B/mappers, romsets, ADPCM), oracles C++ (ymfm, OKI), scripts MAME golden
  ref/          sources de référence EN LECTURE SEULE (MAME capcom, MAME sound, MAME emu tilemap/drawgfx, ymfm, crates CPU, modèle Puce16 Neo Geo)
  docs/ spec/ adr/
  tasks/        cartes Txxx.md + board.json (import Hermes Kanban : scripts/kanban_import.ps1)
  metrics/      scores des suites (versionnés)
  roms/ saves/ tests-data/golden/   (ignorés par git)
```
**CPU = crates de l'utilisateur** (dépendances git épinglées, jamais copiées ni modifiées) : `puce-m68k` (68000 cycle-exact, API `Bus`/`step_clock`)
et `puce-z80` (Z80, API broches `tick`). Docs d'API : `ref/cpu-crates/`.
Graphe (sans cycle) : `cps-common` <- (`cps-gfx`, `cps-ym2151`, `cps-oki`, `cps-cart`) <- `cps-bus` (+ puce-z80) <- `cps-core` (+ puce-m68k) <- `cps-gui`.
`cps-testkit` dépend de `cps-common` ; `cps-core` l'utilise en dev-dependency.

## Ordonnanceur (DÉCISION, cartes T401-T402)
Le 68000 est MAÎTRE et avance d'une horloge à la fois (`puce_m68k::cpu::Cpu::step_clock`). `Bus::on_tick` : `now += ticks_68k` puis
(1) catch-up de la carte son : tant que `sound.time + 1408 <= now` : un T-state Z80 (`SoundBoard::tick`) ; le YM2151 produit un échantillon
toutes les 64 de ses horloges (90 112 ticks) et traite ses timers ; l'OKI produit un échantillon toutes les 132 (ou 165) horloges OKI ;
(2) événements vidéo : si `now >= next_vblank` : IRQ2 + bases vidéo + latch sprites + rendu de la trame (02 §Instant du rendu).
Les accès bus 68000 vers les latches son se font donc avec la carte son à l'heure (`sound.time` ∈ ]now-1408, now]).
`Bus::ipl` = niveau IRQ (échantillonné par le CPU aux frontières d'instruction), `Bus::int_ack` = autovecteur + effacement IPL1/IPL2.
`Bus::wait_clocks` = 0 partout (Q-WAIT). Savestate / débogueur uniquement à une frontière d'instruction.

## Rendu
Composition de trame complète au début du VBLANK, à l'image de MAME (`screen_update_cps1`). Le framebuffer 384×224 (0x00RRGGBB) est la référence
comparée aux PNG MAME. Pas de rendu par ligne tant qu'aucun jeu cible n'utilise d'IRQ raster (ADR à écrire si besoin : ganbare, P6).

## GUI (ADR-002)
Le GUI est le frontal réutilisable `puce-jamma-gui` (dépôt PuceArcade_JammaGui_Crates). `cps-gui` implémente le contrat `JammaBoard`
(`ref/puce-jamma/00-contrat-jamma.md`) : entrées JAMMA actives hautes -> IN0/IN1/IN2 actifs bas, image 384×224 (rotation ROT0/ROT270 du jeu),
son mono 55 930 Hz, DIP décrits, savestates, débogage (68000, Z80, CPS-A/B, palette, tuiles, couches). Le cœur ne connaît ni egui ni threads.

## Dépendances autorisées
`puce-jamma`, `puce-jamma-gui` (git épinglé, cps-gui uniquement) ; `serde` + `bincode`, `toml`, `zip`, `png`, `tempfile` (dev). Tout autre ajout = ADR.
