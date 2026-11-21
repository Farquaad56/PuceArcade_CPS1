# Tests et oracles
| Cible | Oracle | Format | Cartes |
|---|---|---|---|
| 68000 / Z80 | crates puce-m68k / puce-z80 + test de fumée | cargo test | T003 |
| ROMs | CRC des fichiers (table générée) | cargo test (#[ignore] si ROM absente) | T100-T101 |
| Vidéo | valeurs calculées dans les cartes + golden MAME | CRC / PNG | T200-T208, T404-T407 |
| YM2151 | ymfm (tests-data/ym2151) | i16 LE stéréo | T300-T307 |
| OKI | MAME okiadpcm (tests-data/oki) | i32 LE + statut | T310-T312 |
| Jeux | captures MAME 0.287 (`tools/mame_golden.ps1`) | PNG 384×224, seuil 2 % | T404-T407, T604 |
## Règles
- `metrics/<suite>.json` = `{pass, fail, total}` ; le gate refuse une baisse (tools/check_metrics.ps1).
- Tests nécessitant une ROM : `#[ignore]`, chemin lu dans `cps1.toml` (jamais commité) ; lancés avec `cargo test -- --include-ignored`.
- Pas de ROM ni de capture MAME dans le dépôt (tests-data/golden/ ignoré).
