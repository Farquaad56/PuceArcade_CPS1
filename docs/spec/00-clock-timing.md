# 00 — Horloges et timing (source : ref/mame-cps1/cps1.h l.27-47, cps1.cpp l.3909-3965, ref/mame-sound/okim6295.cpp l.160-170, ref/ymfm/src/ymfm_opm.h l.111-117)

## Horloges du matériel (MAME, « verified on pcb »)
| Composant | Fréquence | Source MAME |
|---|---|---|
| 68000 (A-board 88617A / 89626A) | 10 MHz (XTAL 10 MHz) | `cps1_10MHz` : ghouls, sf2 |
| 68000 (A-board 89626A-4 DASH) | 12 MHz (XTAL 12 MHz) | `cps1_12MHz` : sf2ce, sf2hf |
| Z80 son | 3,579545 MHz (XTAL 3.579545) = 315/88 MHz | `cps1_10MHz` |
| YM2151 | 3,579545 MHz (même quartz) | `cps1_10MHz` |
| OKI MSM6295 | 16 MHz / 4 / 4 = 1 MHz, broche 7 haute au reset | `cps1_10MHz` |
| Pixel (CPS-A/CPS-B) | 16 MHz / 2 = 8 MHz | `CPS_PIXEL_CLOCK` |

## Horloge du projet : 1 tick = 1 / 5 040 000 000 s (DÉCISION, ADR-001)
Aucun quartz commun : on prend le plus petit multiple commun entier de 10 MHz, 12 MHz, 16 MHz et 315/88 MHz, soit **5,04 GHz**.
Tout l'état temporel est en `Tick = u64` (débordement après ~116 ans).
| Constante | Ticks | Calcul |
|---|---|---|
| `TICKS_PER_68K_10MHZ` | 504 | 5 040 000 000 / 10 000 000 |
| `TICKS_PER_68K_12MHZ` | 420 | 5 040 000 000 / 12 000 000 |
| `TICKS_PER_Z80` (= `TICKS_PER_YM`) | 1408 | 5 040 000 000 × 88 / 315 000 000 |
| `TICKS_PER_OKI` | 5040 | 5 040 000 000 / 1 000 000 |
| `TICKS_PER_PIXEL` | 630 | 5 040 000 000 / 8 000 000 |
| `TICKS_PER_LINE` | 322 560 | 512 × 630 |
| `TICKS_PER_FRAME` | 84 510 720 | 262 × 322 560 |
| `YM_CLOCKS_PER_SAMPLE` | 64 | prescale 2 × 32 opérateurs (ymfm `fm_engine_base::sample_rate`) |
| `TICKS_PER_YM_SAMPLE` | 90 112 | 64 × 1408 |
| `OKI_DIVISOR_PIN7_HIGH` / `_LOW` | 132 / 165 | okim6295.cpp l.168 |
| `TICKS_PER_OKI_SAMPLE` (pin7 haute) | 665 280 | 132 × 5040 (pin7 basse : 831 600) |
Fréquence trame = 5,04e9 / 84 510 720 = **59,637 Hz** (MAME : 59,6376 Hz ; GNG mesuré 59,61 Hz).
Horloges 68000 par trame : 167 680 (10 MHz), 201 216 (12 MHz). Fréquence d'échantillonnage YM2151 = 55 930,4 Hz ; OKI = 7 575,76 Hz.

## Trame (raw params MAME : `set_raw(8 MHz, 512, 64, 448, 262, 16, 240)`)
- Ligne : 512 pixels (HTOTAL). Zone visible x = 64..447 (HBEND=64, HBSTART=448) : **384 pixels**.
- Trame : 262 lignes (VTOTAL). Zone visible y = 16..239 (VBEND=16, VBSTART=240) : **224 lignes**.
- Coordonnées « écran MAME » (bitmap 512 × 262) utilisées par tout le rendu : le pixel du framebuffer (fx, fy), 0 ≤ fx < 384, 0 ≤ fy < 224,
  correspond à l'écran (x = fx + 64, y = fy + 16).
- Le temps 0 (reset) est le début de la ligne 0. Ligne courante = (now mod TICKS_PER_FRAME) / TICKS_PER_LINE.
- **Début du VBLANK = début de la ligne 240** (tick `240 × 322 560 = 77 414 400` dans la trame). C'est l'instant où MAME
  (1) déclenche l'IRQ2 du 68000 (`set_vblank_int` -> `cps1_interrupt`), (2) appelle `screen_vblank_cps1` (recalcul des bases vidéo)
  puis `cps1_objram_latch` (copie de la RAM des sprites), (3) dessine la trame entière (`screen_update_cps1`). Ordre retenu : 1, 2, 3 (voir QUESTIONS Q-VBL).
- Numérotation : la trame n°1 est celle rendue au premier début de VBLANK après le reset (ligne 240 de la première trame), comme MAME
  (`-autoboot_script` capture la trame N = N-ième appel de mise à jour d'écran).

## Wait states
MAME n'en émule aucun pour le CPS1 (« TODO: verify wait cycle when accessing ROM/RAMs », cps1.cpp l.16). Le projet : 0 (DTACK immédiat) partout.
Point ouvert Q-WAIT dans QUESTIONS.md.
