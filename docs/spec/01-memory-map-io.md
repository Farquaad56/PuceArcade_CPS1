# 01 — Cartes mémoire, E/S, interruptions (source : ref/mame-cps1/cps1.cpp l.257-420 et l.577-642, cps1_v.cpp l.2099-2244)

## Conventions
- Bus 68000 sur 24 bits (adresse & 0xFF_FFFF). Accès mot (uds+lds), octet pair (uds = D8-D15), octet impair (lds = D0-D7).
- **Lecture non mappée = 0x0000** (valeur de « unmap » par défaut de MAME), **écriture non mappée ignorée**.
- `COMBINE_DATA(&r, data, mask)` de MAME : `r = (r & !mask) | (data & mask)` avec mask = 0xFF00 (uds seul), 0x00FF (lds seul), 0xFFFF (mot).
- La ROM 68000 est un tampon **big-endian** (octet 0 = octet de poids fort du mot 0, tel que le 68000 le voit).

## 68000 — `main_map` (cps1.cpp l.577-597)
| Plage | Accès | Fonction |
|---|---|---|
| 000000-3FFFFF | R | ROM programme (région de 0x400000 octets, parties non chargées = 0x00). Écriture ignorée. |
| 800000-800007 | R | `IN1` (16 bits : octet bas = joueur 1, octet haut = joueur 2), même valeur aux 4 mots. |
| 800018-80001F | R | `cps1_dsw_r` : index de mot k = (adr - 0x800018)/2 ; k=0 IN0, k=1 DSWA, k=2 DSWB, k=3 DSWC ; valeur = `(port << 8) \| 0xFF`. |
| 800020-800021 | R | `nopr` : renvoie 0x0000. |
| 800030-800037 | W | `cps1_coinctrl_w` : si uds : compteurs pièces bits 8/9, verrous bits 10/11 (inversés). Aucun effet sur l'émulation (état exposé pour la GUI). |
| 800100-80013F | W | Registres **CPS-A** (32 mots, `COMBINE_DATA`). Lecture = 0x0000 (pas de gestionnaire de lecture). |
| 800140-80017F | R/W | Registres **CPS-B** (32 mots) : lecture `cps1_cps_b_r`, écriture `cps1_cps_b_w` (voir 02 §CPS-B). |
| 800180-800187 | W | `cps1_soundlatch_w` : si lds : latch1 = data & 0xFF ; sinon latch1 = data >> 8. |
| 800188-80018F | W | `cps1_soundlatch2_w` : si lds : latch2 = data & 0xFF (sinon rien). |
| 900000-92FFFF | R/W | **GFXRAM** 0x30000 octets (0x18000 mots), écriture `cps1_gfxram_w` = COMBINE_DATA (sf2ce y exécute du code). |
| FF0000-FFFFFF | R/W | RAM de travail 64 Kio. |
sf2ee (CPS-B à 8001C0) et les bootlegs sont hors périmètre (phase P6/P7).

## Espace CPU (acquittement d'interruption) — `cpu_space_map` / `irqack_r` (cps1.cpp l.407-423)
Un cycle d'acquittement (FC=7) **efface IPL1 et IPL2** puis répond VPA (autovecteur 24 + niveau).
Avec puce-m68k : `Bus::int_ack(level)` efface les deux lignes et renvoie `None`.

## Interruptions 68000
- IPL1 (niveau 2, IRQ2) affirmée au **début du VBLANK** (ligne 240), maintenue jusqu'à l'acquittement (`cps1_interrupt`, l.348-354).
- IPL2 (IRQ4) : jamais affirmée pour ghouls/sf2/sf2ce/sf2hf (uniquement `init_rasters`/ganbare, phase P6).
- Niveau présenté par `Bus::ipl()` : IPL1 seule -> 2 ; IPL2 seule -> 4 ; les deux -> 6 (`set_interrupt_mixer(false)` : lignes séparées).

## Entrées (actives à 0, `IP_ACTIVE_LOW`) — cps1.cpp l.829-946 (`cps1_3b`, `cps1_6b`)
- IN0 (8 bits) : bit0 Pièce1, bit1 Pièce2, bit2 Service1, bit3 inutilisé(1), bit4 Start1, bit5 Start2, bit6 Service/Test, bit7 inutilisé(1). Repos 0xFF.
- IN1 (16 bits) : bits 0-7 joueur 1, bits 8-15 joueur 2 ; par joueur : bit0 Droite, bit1 Gauche, bit2 Bas, bit3 Haut, bit4 Bouton1, bit5 Bouton2, bit6 Bouton3, bit7 inutilisé(1). Repos 0xFFFF.
- IN2 (8 bits, SF2 uniquement, lu par le CPS-B à l'offset `in2_addr` = 0x36) : bit0 P1 Short Kick (B4), bit1 P1 Forward (B5), bit2 P1 Roundhouse (B6), bit3 (1), bit4-6 idem P2, bit7 (1). Repos 0xFF ; valeur lue sur 16 bits = 0x00FF au repos (bits 8-15 non définis = 0).
- ghouls (`cps1_2b_4way`) : bouton 3 absent mais le bit lit 1.

## DIP switches par défaut (cps1.cpp l.1070-1124 ghouls, l.2047-2089 sf2 ; sf2ce et sf2hf utilisent `sf2`)
| Jeu | DSWA | DSWB | DSWC |
|---|---|---|---|
| ghouls | 0xFF | 0xFD (difficulté « 2 » = 0x05) | 0xFF |
| sf2, sf2ce, sf2hf | 0xFF | 0xFC (difficulté 3 = 0x04) | 0x9F (Demo Sounds On = 0, Allow Continue Yes = 0) |
DSWC bit 7 = « Game Mode » (1 = jeu, 0 = test). Les DIP sont modifiables par la configuration et la GUI.

## Z80 — `sub_map` (cps1.cpp l.631-642) et PAL SOU1 (l.602-628)
| Plage | Accès | Fonction |
|---|---|---|
| 0000-7FFF | R | ROM son (région audiocpu 0x18000, octets 0x0000-0x7FFF) |
| 8000-BFFF | R | Banque : `audiocpu[0x10000 + bank*0x4000 + (a - 0x8000)]`, bank ∈ {0,1} (`MACHINE_START cps1`, l.3899-3902) |
| D000-D7FF | R/W | RAM 2 Kio |
| F000-F001 | R/W | YM2151 : offset 0 = adresse (écriture) / lecture 0xFF ; offset 1 = donnée (écriture) / statut (lecture) |
| F002 | R/W | OKI MSM6295 : lecture statut, écriture commande |
| F004 | W | banque = data & 1 (`cps1_snd_bankswitch_w`) |
| F006 | W | broche 7 de l'OKI = data & 1 (`cps1_oki_pin7_w`) : change la période d'échantillon |
| F008 | R | latch1 (commande son) |
| F00A | R | latch2 (fondu/timer) |
Tout le reste : lecture 0x00, écriture ignorée. Ports E/S Z80 (IN/OUT) : non connectés (MAME ne déclare pas de carte IO pour ce Z80) : lecture 0x00, écriture ignorée.
Interruption Z80 : sortie IRQ du YM2151 reliée à INT (niveau, `irq_handler().set_inputline(m_audiocpu, 0)`). Pas de NMI. Octet présenté au cycle
d'acquittement (IM 0/2) : 0xFF (valeur par défaut du Z80 MAME).
Latches : `generic_latch_8` (valeur initiale 0, lecture non destructive).

## Reset
Reset machine : 68000 reset (vecteurs SSP = ROM[0..4], PC = ROM[4..8]), Z80 reset, banque Z80 = 0, broche 7 OKI = 1, latches = 0, IPL effacées,
registres CPS-A/CPS-B effacés puis valeurs par défaut de `video_start` (02 §Reset vidéo), GFXRAM et RAM = 0.
