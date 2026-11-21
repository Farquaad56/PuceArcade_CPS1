# STATE — journal de passation (garder < 150 lignes)

## Phase courante
P0 Fondations. Prochaine carte : T000.

## Fait
- (conception) Squelette du workspace (9 crates vides qui compilent), specs digérées docs/spec/00-04, ref/ (MAME, ymfm, crates CPU, modèle Neo Geo),
  générateurs tools/gen_cps_tables.py, tools/extract_romsets.py, tools/gen_oki_tables.py, oracles tests-data/ym2151 et tests-data/oki.
- T000 : smoke tests dans les 9 crates, vérification check_metrics.ps1 (baseline + détection régression), metrics/.gitkeep, gate OK.

## Blocages
(aucun)
