# STATE — journal de passation (garder < 150 lignes)

## Phase courante
P0 Fondations. Prochaine carte : T000.

## Fait
- (conception) Squelette du workspace (9 crates vides qui compilent), specs digérées docs/spec/00-04, ref/ (MAME, ymfm, crates CPU, modèle Neo Geo),
  générateurs tools/gen_cps_tables.py, tools/extract_romsets.py, tools/gen_oki_tables.py, oracles tests-data/ym2151 et tests-data/oki.
- T000 : smoke tests dans les 9 crates, vérification check_metrics.ps1 (baseline + détection régression), metrics/.gitkeep, gate OK.
- T001 : interfaces gelées cps-common (clock.rs, video.rs, input.rs, crc.rs, bits.rs) avec 19 tests unitaires ; gate OK. Piège : clippy needless_range-loop sur la table CRC — corrigé par enumerate().
- T002 : configuration locale cps-cart (error.rs + config.rs parse/load toml::Table) avec 5 tests unitaires ; gate OK. Piège : type annotation nécessaire pour le paramètre de closure dans map_err(|e: toml::de::Error|).
- T003 : dépendances CPU puce-m68k et puce-z80 épinglées (révisions identiques à Puce16 Neo Geo) ; test de fumée cpu_crates.rs avec 3 tests (reset+MOVE.W+STOP, attente wait states, Z80 step API) ; gate OK. Piège : boucle step_clock doit stepper AVANT de vérifier at_instruction_boundary() sinon elle ne progresse pas après reset.
- T004 : comparateur d'images golden cps-testkit (golden.rs adapté de Puce16 Neo Geo pour 384x224, ratio(), golden_dir()) ; 7 tests unitaires passants ; gate OK. Piège : test golden_dir doit vérifier les composants du chemin plutôt que la chaîne complète (séparateur différent sur Windows).

- (2026-11-21) cps1.toml créé (roms_dir = roms/, MAME X:/Mame_0287) ; golden MAME générés : tests-data/golden/{ghouls,sf2,sf2ceea}/f000120..f003000.png (non versionnés).

## Blocages
(aucun)
