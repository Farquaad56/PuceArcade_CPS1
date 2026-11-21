# 04 — Romsets et jeux (source : ref/mame-cps1/cps1.cpp ROM_START…ROM_END et lignes GAME l.14956-15196)

## Données générées
`python tools/extract_romsets.py` -> `crates/cps-cart/src/romsets_data.rs` : 233 romsets, régions maincpu / gfx / audiocpu / oki / stars,
opérations dans l'ordre du source, et la table `GAMES` (nom, parent, config machine, ports d'entrée, init, titre, NOT_WORKING).
Les régions PLD, qsound, user, timekeeper, protection sont ignorées (listées en commentaire au-dessus de chaque romset).

## Sémantique des opérations (MAME romload ; région = tableau d'octets, octet 0 = poids fort pour maincpu)
| Op | Effet (i = index d'octet du fichier lu pour cette op) |
|---|---|
| `Load::Byte` (ROM_LOAD) | `region[offset + i] = file[i]` |
| `Load::Byte16` (ROM_LOAD16_BYTE) | `region[offset + 2*i] = file[i]` |
| `Load::Word16` (ROM_LOAD16_WORD) | `region[offset + i] = file[i]` (fichier déjà big-endian) |
| `Load::Word16Swap` (ROM_LOAD16_WORD_SWAP) | pour chaque paire : `region[offset+2k] = file[2k+1]`, `region[offset+2k+1] = file[2k]` |
| `Load::Word64` (ROM_LOAD64_WORD) | pour chaque mot k : `region[offset + 8k + j] = file[2k + j]`, j = 0,1 |
| `Load::Byte64` (ROM_LOAD64_BYTE) | `region[offset + 8*i] = file[i]` |
| `Load::Byte32` (ROM_LOAD32_BYTE) | `region[offset + 4*i] = file[i]` |
| `Continue { offset, size }` | continue la lecture du DERNIER fichier chargé (à la suite de ce qui a déjà été lu) avec le même type, à `offset` |
| `Reload { offset, size }` | relit le dernier fichier depuis son début, même type, à `offset` |
| `Ignore { size }` | saute `size` octets du dernier fichier |
| `Copy { src, src_offset, offset, size }` | copie depuis la région `src` (déjà construite) |
| `Fill { offset, size, value }` | remplit |
| `NoDump` | fichier absent par définition : ignoré (le jeu est probablement NOT_WORKING) |
Région : taille `size`, remplie à `fill` (0x00, ou 0xFF si ROMREGION_ERASEFF) avant les opérations. `size` de Load = nombre d'octets lus du fichier.
Fichier introuvable / taille / CRC différents : erreur explicite (`CartError`) avec le nom du fichier — sauf option `allow_bad_crc` (avertissement).
Recherche des fichiers : `<roms_dir>/<jeu>.zip`, puis `<roms_dir>/<jeu>/`, puis le parent (`<parent>.zip`, `<parent>/`) ; dans un zip, recherche par
nom (insensible à la casse) puis par CRC.

## Jeux cibles (priorité)
| Ordre | Jeu | Parent | Machine | 68000 | CPS-B / mapper | Particularités |
|---|---|---|---|---|---|---|
| 1 | ghouls | — | cps1_10MHz | 10 MHz | CPS-B-01 / DM620 | ROM 68k : 4 × LOAD16_BYTE + 1 LOAD16_WORD ; gfx 0x300000 ; pas d'échantillons OKI (région 0xFF) |
| 2 | sf2 | — | cps1_10MHz | 10 MHz | CPS-B-11 / STF29 | 6 boutons (IN2 via CPS-B 0x36) ; gfx 0x600000 ; OKI 0x40000 |
| 3 | sf2ce | — | cps1_12MHz | 12 MHz | CPS-B-21 (défaut) / S9263B | ROM 68k LOAD16_WORD_SWAP ; multiplication CPS-B ; exécute du code en GFXRAM |
| 4 | sf2hf | — | cps1_12MHz | 12 MHz | CPS-B-21 (défaut) / S9263B | idem sf2ce |
Ensuite (phase P6) : tous les autres jeux `cps1_10MHz` / `cps1_12MHz` à `empty_init` sans bootleg ; puis forgottn (étoiles + uPD4701),
ganbare (rasters), sf2ee (CPS-B en 8001C0, `init_sf2ee` l.14594-14601). Hors périmètre : `qsound` (Kabuki + QSound), bootlegs (cps1bl_*.cpp), CPS Changer.

## Hachages de référence des 4 jeux (premiers fichiers, pour les tests) — copiés du source
- ghouls : dme_29.10h CRC 166A58A2, dm-17.7j 3EA1B0F2, dm-05.3a 0BA9C0B0, 26.10a (audio) 3692F6E5.
- sf2 : sf2e_30g.11e FE39EE33, sf2-5m.4a 22C9CC8E, sf2_9.12a (audio) A4823A1B, sf2_18.11c (oki) 7F162009.
- sf2ce : s92e_23b.8f 0AAA1A3A, s92-1m.3a 03B0D852, s92_09.11a 08F6B60E. sf2hf : s2te_23.8f 2DD72514 (gfx/audio/oki identiques à sf2ce).
