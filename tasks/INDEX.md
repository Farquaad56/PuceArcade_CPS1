# Index des tâches (CHAÎNE LINÉAIRE : chaque carte dépend de la précédente ; profil rust-dev)

| Ordre | Id | Phase | Titre |
|---|---|---|---|
| 1 | T000 | P0 Fondations | Workspace Cargo vérifié, gate, métriques |
| 2 | T001 | P0 Fondations | cps-common : horloge (ticks 5,04 GHz), framebuffer, entrées, CRC, COMBINE_DATA |
| 3 | T002 | P0 Fondations | cps-cart : configuration locale (cps1.toml) et erreurs |
| 4 | T003 | P0 Fondations | Dépendances CPU : puce-m68k et puce-z80 (git épinglé) et test de fumée |
| 5 | T004 | P0 Fondations | cps-testkit : comparateur d'images golden (PNG de MAME contre framebuffer) |
| 6 | T100 | P1 ROMs et bus | cps-cart : romsets générés (cps1.cpp) et sources de fichiers (zip, dossier, parent) |
| 7 | T101 | P1 ROMs et bus | cps-cart : construction des régions, DIP par défaut, chargement d'un jeu |
| 8 | T102 | P1 ROMs et bus | cps-bus : carte mémoire 68000 et E/S (vidéo derrière le trait VideoPort) |
| 9 | T103 | P1 ROMs et bus | cps-bus : carte son Z80 (SoundBoard, serveur de broches au T-state) |
| 10 | T200 | P2 Vidéo CPS-A/CPS-B | cps-gfx : configurations CPS-B et mappers (générés) + gfxrom_bank_mapper |
| 11 | T201 | P2 Vidéo CPS-A/CPS-B | cps-gfx : décodage des tuiles (layouts 8x8, 8x8_2, 16x16, 32x32) |
| 12 | T202 | P2 Vidéo CPS-A/CPS-B | cps-gfx : palette (cps1_build_palette) |
| 13 | T203 | P2 Vidéo CPS-A/CPS-B | cps-gfx : état vidéo (Video), registres, GFXRAM, cps1_base, bases vidéo (get_video_base) |
| 14 | T203b | P2 Vidéo CPS-A/CPS-B | cps-gfx : écritures CPS-A (copie de palette), CPS-B (lecture/écriture, multiplication), latch des sprites au VBLANK |
| 15 | T204 | P2 Vidéo CPS-A/CPS-B | cps-gfx : tilemaps scroll1/2/3 (scan, tuiles, transparence par groupe, défilement, rowscroll) |
| 16 | T205 | P2 Vidéo CPS-A/CPS-B | cps-gfx : sprites (1/2) — find_last_sprite et dessin prio_transpen d'une tuile 16x16 |
| 17 | T205b | P2 Vidéo CPS-A/CPS-B | cps-gfx : sprites (2/2) — render_sprites (mapper, blocs, flips, flip écran, ordre) |
| 18 | T206 | P2 Vidéo CPS-A/CPS-B | cps-gfx : composition de trame et priorités (screen_update_cps1, render_layers) |
| 19 | T207 | P2 Vidéo CPS-A/CPS-B | cps-bus : branchement de la vidéo (VideoPort pour cps_gfx::Video) |
| 20 | T300 | P3 Audio | cps-ym2151 : types communs ymfm (reprise de ng-ym2610/common.rs) |
| 21 | T301 | P3 Audio | cps-ym2151 : tables et fonctions FM (copie) + opm_key_code_to_phase_step |
| 22 | T302 | P3 Audio | cps-ym2151 : registres OPM (1/2) — structure, constantes, accesseurs, reset, tables, operator_map |
| 23 | T302b | P3 Audio | cps-ym2151 : registres OPM (2/2) — write, bruit et LFO, cache d'opérateur, pas de phase |
| 24 | T303 | P3 Audio | cps-ym2151 : opérateur FM (fm_operator sur registres OPM) |
| 25 | T304 | P3 Audio | cps-ym2151 : canal FM (fm_channel, algorithmes 4 opérateurs, bruit) |
| 26 | T305 | P3 Audio | cps-ym2151 : moteur FM (fm_engine : horloge, timers, LFO, sortie) |
| 27 | T306 | P3 Audio | cps-ym2151 : puce YM2151 (port de la classe ym2151) |
| 28 | T306b | P3 Audio | cps-ym2151 : adaptateur cadencé ClockedYm2151 (horloge YM, timers, busy, IRQ, échantillons) |
| 29 | T307 | P3 Audio | Oracle YM2151 : comparaison aux vecteurs ymfm (tests-data/ym2151) |
| 30 | T310 | P3 Audio | cps-oki : état ADPCM OKI (okiadpcm, tables générées) |
| 31 | T311 | P3 Audio | cps-oki : MSM6295 (voix, commandes, statut, broche 7) |
| 32 | T312 | P3 Audio | Oracle OKI : comparaison aux vecteurs MAME (tests-data/oki) |
| 33 | T313 | P3 Audio | cps-bus : puces son réelles (SoundChips : YM2151 cadencé + OKI) et mixage |
| 34 | T400 | P4 Intégration | cps-core : Machine (construction, reset, accès) |
| 35 | T401 | P4 Intégration | cps-core : hôte 68000 et ordonnanceur (catch-up son, événement VBLANK) |
| 36 | T402 | P4 Intégration | cps-core : interruptions et VBLANK (IRQ2, bases vidéo, latch sprites, rendu) |
| 37 | T403 | P4 Intégration | cps-core : boucle publique, entrées, audio par trame |
| 38 | T404 | P4 Intégration | Ghouls'n Ghosts (ghouls) : démarrage et attract mode (golden MAME) |
| 39 | T405 | P4 Intégration | Street Fighter II (sf2, World 910522) : démarrage et attract mode |
| 40 | T406 | P4 Intégration | Street Fighter II' Champion Edition (sf2ceea, World 920313) : 12 MHz, CPS-B-21, multiplication |
| 41 | T407 | P4 Intégration | Street Fighter II' Hyper Fighting (sf2hf) : démarrage et attract mode |
| 42 | T408 | P4 Intégration | cps-core : savestates (sérialisation complète) |
| 43 | T500 | P5 GUI (frontal JAMMA) | cps-gui : dépendances JAMMA épinglées et conversion des entrées (JammaInputs -> IN0/IN1/IN2) |
| 44 | T500b | P5 GUI (frontal JAMMA) | cps-gui : adaptateur Cps1Board (BoardInfo, vidéo, son, entrées) |
| 45 | T501 | P5 GUI (frontal JAMMA) | cps-gui : DIP décrits, savestates et driver (catalogue des jeux) |
| 46 | T502 | P5 GUI (frontal JAMMA) | cps-gui : point d'entrée (frontal puce-jamma-gui) et validation manuelle |
| 47 | T503 | P5 GUI (frontal JAMMA) | cps-core : primitives de débogage (pas à pas, lecture mémoire sans effet de bord) |
| 48 | T503b | P5 GUI (frontal JAMMA) | cps-gui : BoardDebug côté CPU (68000, Z80, mémoire, pas à pas) |
| 49 | T504 | P5 GUI (frontal JAMMA) | cps-gui : débogage vidéo et son (registres CPS-A/CPS-B, palette, tuiles, couches) |
| 50 | T505 | P5 GUI (frontal JAMMA) | cps-gui : tests d'intégration par le contrat JAMMA |
| 51 | T600 | P6 Autres jeux CPS1 | cps-gfx : étoiles (cps1_render_stars) |
| 52 | T601 | P6 Autres jeux CPS1 | cps-bus : Forgotten Worlds (forgottn) — uPD4701 (molettes) et LWIO |
| 53 | T602 | P6 Autres jeux CPS1 | cps-cart : machines supportées étendues (forgottn, sf2ee) et liste des jeux jouables |
| 54 | T603 | P6 Autres jeux CPS1 | cps-core : intégration forgottn (étoiles + molettes) et sf2ee (CPS-B en 8001C0) |
| 55 | T604 | P6 Autres jeux CPS1 | Validation du catalogue CPS1 (golden MAME par lot) |
| 56 | T605 | P6 Autres jeux CPS1 | cps-gfx : flip écran (tilemaps + sprites) |
| 57 | T700 | P7 Précision et outils | Banc de performance et mesure de référence |
| 58 | T701 | P7 Précision et outils | Précision : Q-VBL, Q-RESET68K, Q-WAIT (revue contre les golden) |
| 59 | T702 | P7 Précision et outils | cps-gfx : compteurs raster CPS-B-21 (IRQ4) — préparation ganbare/CPS2 |
