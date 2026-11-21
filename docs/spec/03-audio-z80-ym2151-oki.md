# 03 — Audio : Z80, YM2151, OKI MSM6295 (source : ref/mame-cps1/cps1.cpp l.292-314, l.631-642, l.3909-3947 ; ref/ymfm/src/ymfm_opm.* ; ref/mame-sound/okim6295.*, okiadpcm.*)

## Chaîne son (cps1_10MHz)
68000 --latch1 (800181/800180) / latch2 (800189)--> Z80 (3,579545 MHz) --F000/F001--> YM2151 (3,579545 MHz) ; Z80 --F002--> OKI (1 MHz).
YM2151 IRQ -> Z80 INT. Mélange MAME : haut-parleur mono ; YM2151 sorties 0 et 1 avec gain 0,35 chacune ; OKI gain 0,30.

## YM2151 = ymfm `ym2151` (ymfm_opm.h l.104-310, ymfm_opm.cpp l.40-539, moteur générique ymfm_fm.h / ymfm_fm.ipp)
- Registres OPM : 8 canaux × 4 opérateurs, 0x100 registres, `DEFAULT_PRESCALE = 2`, `EG_CLOCK_DIVIDER = 3`, `CSM_TRIGGER_MASK = 0xFF`, `REG_MODE = 0x14`,
  statut : bit0 timer A, bit1 timer B, bit7 busy. Pas de SSG-EG (`EG_HAS_SSG = false`), `MODULATOR_DELAY = false`, bruit sur l'opérateur 4 du canal 7.
- Interface ymfm (`ymfm_interface`) : `set_timer(n, durée_en_horloges_YM)` (durée < 0 = arrêt), `set_busy_end(horloges)`, `is_busy()`, `update_irq(bool)`.
- Échantillon : 1 toutes les 64 horloges YM (00 §Horloge). Sortie `output_data` = 2 voies i32 (gauche, droite), écrêtées à ±32767 par l'oracle.
- Le port Rust réutilise le moteur FM déjà porté pour le YM2610 de Puce16 Neo Geo (`ref/puce16-neogeo/ng-ym2610/` : fm_tables.rs, fm_operator.rs,
  fm_channel.rs, fm_engine.rs, common.rs), en remplaçant les registres OPN (`opn_regs.rs`) par les registres OPM portés de ymfm_opm.cpp.
- Oracle : `tests-data/ym2151/*.bin` (tools/ymfm_opm_golden.cpp, voir tools/README-golden.md).

## OKI MSM6295 (okim6295.cpp l.212-361, okiadpcm.cpp l.1-145)
- 4 voix ADPCM OKI (table de pas 49 × 16, `s_index_shift = {-1,-1,-1,-1,2,4,6,8}`), signal 12 bits borné à -2048..2047.
- `stepval(step) = floor(16 × 1,1^step)` ; `diff[step*16+nib] = signe(nib bit3) × (stepval×b2 + stepval/2×b1 + stepval/4×b0 + stepval/8)`
  avec b2 = bit2, b1 = bit1, b0 = bit0 de nib (table nbl2bit). Les 49 valeurs de stepval sont figées dans `tools/gen_oki_tables.py` (pas de flottant dans le cœur).
- Commande (`write`) : octet avec bit7 -> mémorise le numéro d'échantillon (bits 0-6) ; octet suivant : bits 4-7 = voix à démarrer, bits 0-3 = volume ;
  sans commande en attente et bit7 = 0 : arrêt des voix des bits 3-6. Démarrage ignoré si la voix joue déjà ou si start >= stop.
  start/stop = 3 octets big-endian lus à `numéro*8` et `numéro*8+3`, masqués 0x3FFFF. count = 2 × (stop - start + 1) nibbles.
- Statut (`read`) : 0xF0 | bit n si la voix n joue.
- Volume (`s_volume_table`, l.59-77) : numérateurs {0x20,0x16,0x10,0x0B,0x08,0x06,0x04,0x03,0x02,0,0,0,0,0,0,0} sur 0x20.
- Échantillon : pour chaque voix active, nibble = `rom[base + sample/2] >> (((sample & 1) << 2) ^ 4)` (nibble haut d'abord), `s = clock(nibble) × vol_num`,
  somme entière des 4 voix (i32). Valeur flottante MAME = somme / (2048 × 32). `++sample >= count` -> la voix s'arrête.
- Période : `clock / 132` (broche 7 = 1, défaut) ou `clock / 165` (broche 7 = 0). Région ROM « oki » 0x40000 (ghouls : remplie de 0xFF).
- Oracle : `tests-data/oki/*.bin` (tools/oki_golden.cpp).

## Mixage hôte (DÉCISION, carte T313)
Le cœur produit des échantillons YM à 55 930 Hz et OKI à 7 576 Hz. Mélange à la cadence YM :
`m = (yl + yr) / 32768 × 0,35 + oki_courant / 65536 × 0,30` où `oki_courant` = dernier échantillon OKI produit (maintien d'ordre 0).
Puis rééchantillonnage linéaire vers la fréquence de l'hôte (48 000 Hz par défaut). PORT-NOTE : MAME utilise son propre rééchantillonneur ;
l'audio n'est pas comparé bit à bit à MAME, seuls les oracles de puce (YM, OKI) le sont.
