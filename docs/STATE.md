# STATE — journal de passation (garder < 150 lignes)

## Phase courante
P1 ROMs et bus. Prochaine carte : T102.

## Fait
- (conception) Squelette du workspace (9 crates vides qui compilent), specs digérées docs/spec/00-04, ref/ (MAME, ymfm, crates CPU, modèle Neo Geo),
  générateurs tools/gen_cps_tables.py, tools/extract_romsets.py, tools/gen_oki_tables.py, oracles tests-data/ym2151 et tests-data/oki.
- T000 : smoke tests dans les 9 crates, vérification check_metrics.ps1 (baseline + détection régression), metrics/.gitkeep, gate OK.
- T001 : interfaces gelées cps-common (clock.rs, video.rs, input.rs, crc.rs, bits.rs) avec 19 tests unitaires ; gate OK. Piège : clippy needless_range-loop sur la table CRC — corrigé par enumerate().
- T002 : configuration locale cps-cart (error.rs + config.rs parse/load toml::Table) avec 5 tests unitaires ; gate OK. Piège : type annotation nécessaire pour le paramètre de closure dans map_err(|e: toml::de::Error|).
- T003 : dépendances CPU puce-m68k et puce-z80 épinglées (révisions identiques à Puce16 Neo Geo) ; test de fumée cpu_crates.rs avec 3 tests (reset+MOVE.W+STOP, attente wait states, Z80 step API) ; gate OK. Piège : boucle step_clock doit stepper AVANT de vérifier at_instruction_boundary() sinon elle ne progresse pas après reset.
- T004 : comparateur d'images golden cps-testkit (golden.rs adapté de Puce16 Neo Geo pour 384x224, ratio(), golden_dir()) ; 7 tests unitaires passants ; gate OK. Piège : test golden_dir doit vérifier les composants du chemin plutôt que la chaîne complète (séparateur différent sur Windows).
- T100 : romsets CPS1 générés depuis ref/mame-cps1/cps1.cpp via tools/extract_romsets.py (229 jeux, 4 régions conservées) ; types Load/Op/RegionDef/RomSetDef/GameInfo dans romset.rs avec find_game() ; sources de fichiers DirSource (recherche insensible à la casse), ZipSource (nom puis CRC32), ChainSource et sources_for() ; 23 tests unitaires passants. Piège : zip crate v9.x exige type paramètre explicite pour start_file::<&str, ()> ; GAMES est &[GameInfo] donc .iter() donne &&GameInfo — utiliser .iter().copied() ou boucle for directe.

- T101 : construction des régions (regions.rs, 7 types de chargement + Continue/Reload/Ignore/Copy/Fill/NoDump, erreurs BadCrc/BadSize/OutOfRegion), DIP par défaut et décrits (dips.rs + dips_data.rs générés), game.rs (load_game, ticks_per_68k) ; 49 tests dont chargement réel ghouls, sf2, sf2ceea ; gate OK. Terminée par Claude après blocage de l'agent : Load écrivait le fichier entier au lieu de `size` octets, Fill/Copy écrivaient dans une région écrasée ensuite, tests en `static` non compilables (RegionDef n'est pas Copy -> définitions de test construites avec Box::leak).
- (2026-11-21) cps1.toml créé (roms_dir = roms/, MAME X:/Mame_0287) ; golden MAME générés : tests-data/golden/{ghouls,sf2,sf2ceea}/f000120..f003000.png (non versionnés).
- T102 : carte mémoire 68000 et E/S cps-bus (map68k.rs) avec trait VideoPort, Bus68k<V: VideoPort>, décodage exact des plages d'adresse spec docs/spec/01 §68000 ; IN1 lecture, cps1_dsw_r (IN0 + DIPs), nopr, coinctrl, latches son Latch1/Latch2 avec retour SoundCmd enum ; routage vidéo CPS-A write/CPS-B read-write/GFXRAM read-write via stub TestVideo ; 21 tests unitaires passants. Piège : clippy manual_range_contains sur tous les if a >= X && a < Y — corrigé par (X..Y).contains(&a) ; clippy manual_slice_fill sur la boucle de reset RAM — corrigé par self.ram.fill(0).

## Blocages
(aucun)
