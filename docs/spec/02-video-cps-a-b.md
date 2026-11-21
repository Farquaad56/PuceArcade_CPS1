# 02 — Vidéo CPS-A / CPS-B (source : ref/mame-cps1/cps1_v.cpp, cps1.h l.180-205, cps1.cpp l.3837-3886 ; moteur MAME : ref/mame-emu/tilemap.cpp, drawgfx.cpp, drawgfxt.ipp)
Référence pixel = MAME. Toutes les coordonnées sont celles de l'écran MAME 512 × 262 (00 §Trame) ; framebuffer = rectangle x 64..447, y 16..239.

## Registres CPS-A (800100-80013F, index de mot ; cps1.h l.180-200, commentaire cps1_v.cpp « CPS-A Registers »)
| Index | Octet | Rôle |
|---|---|---|
| 0 | 0x00 | OBJ_BASE (base RAM sprites / 256) |
| 1 | 0x02 | SCROLL1_BASE (8×8) |
| 2 | 0x04 | SCROLL2_BASE (16×16) |
| 3 | 0x06 | SCROLL3_BASE (32×32) |
| 4 | 0x08 | OTHER_BASE (table de rowscroll) |
| 5 | 0x0A | PALETTE_BASE : **une écriture déclenche immédiatement la copie de palette** (`cps1_cps_a_w`, l.2115-2134) |
| 6/7 | 0x0C/0x0E | SCROLL1 X / Y |
| 8/9 | 0x10/0x12 | SCROLL2 X / Y |
| 10/11 | 0x14/0x16 | SCROLL3 X / Y |
| 12-15 | 0x18-0x1E | STARS1 X, STARS1 Y, STARS2 X, STARS2 Y |
| 16 | 0x20 | ROWSCROLL_OFFS (début dans la table OTHER) |
| 17 | 0x22 | VIDEOCONTROL : bit0 rowscroll couche 2, bit2 active scroll2, bit3 active scroll3, bit15 flip écran |

`cps1_base(reg, boundary)` (l.2099-2111) : `base = regs_a[reg] * 256 ; base &= !(boundary - 1) ; mot = (base & 0x3FFFF) / 2` dans GFXRAM.
PORT-NOTE : l'index de mot est pris modulo 0x18000 (MAME lirait hors tableau ; jamais atteint par les jeux).
Frontières : scroll 0x4000, obj 0x800, other 0x800, palette 0x400 (`video_start`, l.2533-2543).

## Reset vidéo (`video_start`, l.2533-2596)
GFXRAM = 0, regs CPS-A = 0, regs CPS-B = 0, puis OBJ_BASE=0x9200, SCROLL1_BASE=0x9000, SCROLL2_BASE=0x9040, SCROLL3_BASE=0x9080, OTHER_BASE=0x9100.
Tampon sprites (`buffered_obj`, 0x400 mots) = 0. Palette (0xC00 entrées) = noir.

## CPS-B (800140-80017F) et configuration par jeu (`cps1_config_table`, l.1766-2043 ; macros CPS_B_xx l.486-515)
Les données de configuration sont GÉNÉRÉES (`tools/gen_cps_tables.py` -> `crates/cps-gfx/src/config_data.rs`). Champs : cpsb_addr, cpsb_value,
mult_factor1/2, mult_result_lo/hi, unknown1-3, layer_control, priority[4], palette_control, layer_enable_mask[5], bank_sizes[4] + plages du mapper, in2/in3/out2, bootleg_kludge.
| Jeu | CPS-B | mapper | in2_addr |
|---|---|---|---|
| ghouls | CPS_B_01 : addr -1, value 0x0000, mult -1, layer 0x26, prio {0x28,0x2A,0x2C,0x2E}, pal 0x30, masks {0x02,0x04,0x08,0x30,0x30} | DM620 | 0 |
| sf2 | CPS_B_11 : addr 0x32, value 0x0401, mult -1, layer 0x26, prio {0x28,0x2A,0x2C,0x2E}, pal 0x30, masks {0x08,0x10,0x20,0,0} | STF29 | 0x36 |
| sf2ce, sf2hf | CPS_B_21_DEF : addr 0x32, value -1, mult 0x00/0x02 -> 0x04/0x06, unk 0x08, layer 0x26, prio {0x28..0x2E}, pal 0x30, masks {0x02,0x04,0x08,0x30,0x30} | S9263B | 0x36 |
**Lecture** `cps1_cps_b_r(offset)` (l.2136-2181), dans cet ordre (division entière C++ : `-1 / 2 == 0`, identique en Rust i32) :
1. `offset == cpsb_addr/2` -> `cpsb_value` (tronqué en u16 : -1 -> 0xFFFF). ghouls : offset 0 -> 0x0000.
2. `offset == mult_result_lo/2` -> `(regs_b[f1/2] * regs_b[f2/2]) & 0xFFFF` ; `offset == mult_result_hi/2` -> `>> 16` (produit u32 de deux u16).
3. `in2_addr != 0 && offset == in2_addr/2` -> IN2 (0x00FF au repos) ; `in3_addr` idem IN3.
4. (raster : seulement si `init_rasters`, hors périmètre P2.) 5. Sinon -> 0xFFFF.
**Écriture** `cps1_cps_b_w` : `COMBINE_DATA(regs_b[offset])` (out2 non utilisé par les 4 jeux cibles).

## ROM graphique : mapper (`gfxrom_bank_mapper`, l.2385-2430)
Types : SPRITES=1, SCROLL1=2, SCROLL2=4, SCROLL3=8, STARS=16. Décalage : sprites 1, scroll1 0, scroll2 1, scroll3 3.
`code <<= shift` ; première plage (dans l'ordre) avec `start <= code <= end` ET `type & range.kind != 0` -> `base = somme(bank_sizes[0..bank])` ;
résultat `(base + (code & (bank_sizes[bank] - 1))) >> shift`. Aucune plage -> **-1** (tuile vide : tous pixels = 15). La plage est testée
avant le type : une plage qui contient le code mais pas le type est ignorée et la recherche continue.

## Décodage des tuiles (gfx_layout cps1.cpp l.3837-3879) — région « gfx » vue comme un tableau d'octets
4 plans, offsets de plan {24,16,8,0} : plan 0 (offset 24) = bit 3 de la couleur, offset 0 = bit 0. Bit d'offset b = octet b/8, bit (7 - b%8).
| Layout | taille | octets/tuile | ligne y | pixel x |
|---|---|---|---|---|
| 8x8 (gfx 0) | 8×8 | 64 | base = code*64 + y*8 | octets base+0..3, bit 7-x |
| 8x8_2 (gfx 1) | 8×8 | 64 | idem | octets base+4..7, bit 7-x |
| 16x16 (gfx 2) | 16×16 | 128 | base = code*128 + y*8 | x<8 : octets base+0..3 ; x≥8 : base+4..7 ; bit 7-(x&7) |
| 32x32 (gfx 3) | 32×32 | 512 | base = code*512 + y*16 | groupe g=x/8 : octets base+4g..4g+3, bit 7-(x&7) |
Pour chaque pixel : `pen = (b[+3]>>s &1)<<3 | (b[+2]>>s &1)<<2 | (b[+1]>>s &1)<<1 | (b[+0]>>s &1)` avec s = 7-(x&7) (+3 = octet du plan offset 24).
Nombre d'éléments = taille_région / octets_par_tuile ; `code %= éléments` (drawgfx `code %= elements()`). Couleurs : `color % 0x80`, pen final = color*16 + pen.

## Palette (`cps1_build_palette`, l.2612-2646)
Source = `cps1_base(PALETTE_BASE, 0x400)`. `ctrl = regs_b[palette_control/2]`. Pour page 0..5 : si bit `page` de ctrl : copier 0x200 mots
vers les entrées `0x200*page + i` ; sinon, si au moins une page a déjà été copiée, sauter 0x200 mots dans la source.
Couleur d'un mot p : `bright = 0x0F + ((p >> 12) << 1)` ; `r = ((p>>8)&0xF) * 0x11 * bright / 0x2D` (division entière), idem g (bits 4-7), b (bits 0-3).
Exemples : 0xFFFF -> (255,255,255) ; 0x0FFF -> (85,85,85) ; 0xF800 -> (136,0,0) ; 0x1F00 -> (96,0,0).
Plages de pens : sprites 0x000-0x1FF, scroll1 0x200-0x3FF (couleur+0x20), scroll2 0x400-0x5FF (+0x40), scroll3 0x600-0x7FF (+0x60),
étoiles 0x800/0xA00, **fond = pen 0xBFF**.

## Tilemaps (l.2434-2513 ; création l.2546-2548 : 64×64 tuiles chacune)
| Couche | tuile | taille | scan (col, row) -> index | code | couleur | gfx |
|---|---|---|---|---|---|---|
| scroll1 | 8×8 | 512×512 | `(row&0x1F) + ((col&0x3F)<<5) + ((row&0x20)<<6)` | mot[2i] | (attr&0x1F)+0x20 | gfx 0 si bit5 de l'index = 0, sinon gfx 1 |
| scroll2 | 16×16 | 1024×1024 | `(row&0x0F) + ((col&0x3F)<<4) + ((row&0x30)<<6)` | mot[2i] | (attr&0x1F)+0x40 | gfx 2 |
| scroll3 | 32×32 | 2048×2048 | `(row&0x07) + ((col&0x3F)<<3) + ((row&0x38)<<6)` | mot[2i] & 0x3FFF | (attr&0x1F)+0x60 | gfx 3 |
attr = mot[2i+1] : bit5 flipX, bit6 flipY, groupe = (attr & 0x180) >> 7. Code passé par le mapper (type de la couche).
Masques de transparence (`cps1_update_transmasks`, l.2515-2531), recalculés à chaque trame : pour g = 0..3 :
`mask_g = regs_b[priority[g]/2] ^ 0xFFFF` (si priority[g] == -1 : 0xFFFF). Pixel brut v (0..15) d'une tuile du groupe g :
**LAYER1 opaque ⇔ v != 15** (bgmask 0x8000) ; **LAYER0 opaque ⇔ bit v de mask_g == 0** (tilemap.cpp set_transmask l.574-583).
Défilement (tilemap.cpp effective_rowscroll l.27-46, sans flip) : pixel de tilemap affiché en (sx, sy) = `((sx + scrollx) mod W, (sy + scrolly) mod H)`.
Rowscroll (scroll2 si VIDEOCONTROL bit0, screen_update l.3018-3029) : pour la ligne écran sy (0..255) :
`scrollx(sy) = scrollx2 + other[(sy + regs_a[ROWSCROLL_OFFS]) & 0x3FF]` (arithmétique i32 puis modulo 1024).
Activation (`cps1_get_video_base`, l.2327-2336) : `lc = regs_b[layer_control/2]`, `vc = regs_a[17]` :
scroll1 = `lc & mask[0] != 0` ; scroll2 = `lc & mask[1] != 0 && vc bit2` ; scroll3 = `lc & mask[2] != 0 && vc bit3` ; étoiles 1/2 = `lc & mask[3]` / `mask[4]`.
Une couche désactivée ne dessine rien (ni couleur ni priorité, tilemap.cpp draw_common l.993-995).

## Sprites (l.2648-2866)
Format (4 mots) : X, Y, CODE, ATTR ; ATTR bits 0-4 couleur, bit5 flipX, bit6 flipY, bits 8-11 nx-1, bits 12-15 ny-1.
- **Latch** (`cps1_objram_latch`, l.3063-3070) : au début du VBLANK, copie de 0x400 mots depuis `cps1_base(OBJ_BASE, 0x800)` dans `buffered_obj`.
- `find_last_sprite` (l.2684-2718) sur `buffered_obj` : premier offset (pas de 4) où `(mot[o+3] & 0xFF00) == 0xFF00` -> last = o - 4 ; sinon last = 0x3FC.
- Ordre : `i` de last à 0 par pas de -4 (le pointeur avance de +4 depuis le début du tampon : l'entrée lue à l'itération k est l'entrée k).
  PORT-NOTE : boucle `for k in 0..=(last/4)` sur l'entrée k, nombre d'itérations = last/4 + 1 (si last < 0 : aucune).
- Code via mapper SPRITES ; -1 -> sprite ignoré. Bloc si `attr & 0xFF00 != 0` : nx = ((attr>>8)&0xF)+1, ny = ((attr>>12)&0xF)+1 ;
  sous-sprite (nxs, nys) en `sx = (x + nxs*16) & 0x1FF`, `sy = (y + nys*16) & 0x1FF`, code :
  sans flip `(code & !0xF) + ((code + nxs) & 0xF) + 0x10*nys` ; flipX : `((code + (nx-1) - nxs) & 0xF)` ; flipY : `0x10*(ny-1-nys)`.
  Sinon sprite simple en `(x & 0x1FF, y & 0x1FF)`.
- Dessin `prio_transpen` (drawgfx.cpp l.910-938 ; drawgfxt.ipp l.216-227), gfx 2 (16×16), couleur col*16, pen transparent 15, **pmask = 0x02 | 0x80000000** :
  pour chaque pixel source v != 15 dans le rectangle de clip (64..447 × 16..239) : si `(1 << (pri & 0x1F)) & pmask == 0` -> dest = col*16 + v ;
  **dans tous les cas pri = 31**. Pas de rebouclage horizontal (sx ≥ 448 : invisible). Flip écran : position `(512-16-sx, 256-16-sy)` et flips inversés.

## Composition d'une trame (`screen_update_cps1` l.3001-3052, `render_layers` l.2970-2999)
1. flip = VIDEOCONTROL bit15 ; `cps1_get_video_base()` ; `find_last_sprite()` ; `cps1_update_transmasks()` ; scrolls des tilemaps (+ rowscroll).
2. bitmap = pen 0xBFF partout ; (étoiles si la région existe — aucun des 4 jeux cibles).
3. priorité (u8 par pixel) = 0. `lc = regs_b[layer_control/2]` ; l0 = (lc>>6)&3, l1 = (lc>>8)&3, l2 = (lc>>10)&3, l3 = (lc>>12)&3 (0 = sprites, 1..3 = scroll1..3).
4. dessiner(l0) ; si l1 == 0 : haute(l0) ; dessiner(l1) ; si l2 == 0 : haute(l1) ; dessiner(l2) ; si l3 == 0 : haute(l2) ; dessiner(l3).
   - dessiner(couche tilemap) = pixels **LAYER1** opaques -> bitmap = pen, priorité inchangée (priority 0).
   - haute(couche tilemap) = pixels **LAYER0** opaques -> **seulement** priorité |= 1 (dessin dans un bitmap factice) ; haute(sprites) = rien.
   - dessiner(sprites) = §Sprites.
5. framebuffer(fx, fy) = RGB(palette[bitmap(fx+64, fy+16)]) au format 0x00RRGGBB.

## Instant du rendu (DÉCISION, voir 00 §Trame et QUESTIONS Q-VBL)
Au début du VBLANK (ligne 240) : IRQ2, puis `cps1_get_video_base()`, puis latch sprites, puis composition complète de la trame avec l'état courant.
Aucun rendu par ligne n'est nécessaire pour les 4 jeux cibles (MAME ne fait pas de mise à jour partielle sans `init_rasters`).

## Étoiles et flip écran (phase P6, `cps1_render_stars` l.2869-2931 ; flip tilemap : tilemap.cpp l.35-38 et draw_common l.1008-1012)
Non utilisés par ghouls/sf2/sf2ce/sf2hf en configuration par défaut (DIP flip = Off) : cartes T600/T605.
