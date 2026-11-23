//! Composition de trame CPS1 (port MAME l.2933-3052)
// license:BSD-3-Clause
// copyright-holders:Barry Rodowick, Nicola Salmoria, Aaron Giles
// source: ref/mame-cps1/cps1_v.cpp l.2933-3052

use crate::regs::{Video, VIDEOCONTROL};
use crate::sprites::{band, BITMAP_H, BITMAP_W, CLIP};
use crate::tilemap::{screen_to_tilemap, TileInfo, TILEMAP_DIM};
use cps_common::video::{Framebuffer, SCREEN_H, SCREEN_W};

/// Flip écran des tilemaps (tilemap.cpp l.27-75 et draw_common l.1008-1012) : MAME dessine la
/// pixmap retournée décalée de `extent - largeur + scroll` (xextent = 64 + 447 + 1 = 512,
/// yextent = 16 + 239 + 1 = 256) ; le pixel écran (sx, sy) montre donc le pixel de la tilemap
/// non retournée en (511 - sx + scrollx, 255 - sy + scrolly), et le rowscroll de la ligne
/// (255 - sy). spec: docs/spec/02 §Étoiles et flip écran
fn flipped(v: u32, extent_minus_1: u32, flip: bool) -> u32 {
    if flip {
        extent_minus_1 - v
    } else {
        v
    }
}

impl Video {
    /// flip_screen_set(BIT(videocontrol, 15)) (cps1_v.cpp l.3005).
    pub fn flip_screen(&self) -> bool {
        (self.regs_a[VIDEOCONTROL] >> 15) & 1 != 0
    }

    /// Dessine une couche tilemap (LAYER1 : pixels opaques -> bitmap).
    /// Optimisation T703 : décode les infos de tuile une fois par tuile, pas par pixel.
    /// spec: docs/spec/02 §Composition ; MAME l.2943 + tilemap.cpp draw_common
    pub fn draw_tilemap(
        &self,
        gfx: &[u8],
        layer: usize,
        bitmap: &mut [u16],
        masks: &[u16; 4],
        rows: (usize, usize),
    ) {
        if !self.bases.layer_enabled[layer] {
            return;
        }

        let dim = TILEMAP_DIM[layer];
        let tile_size = match layer {
            0 => 8,
            1 => 16,
            _ => 32,
        };
        let flip = self.flip_screen();
        let (y0, y1) = band(rows);
        for sy in y0..y1 {
            let ey = flipped(sy as u32, 255, flip);
            let (scrollx, scrolly) = self.tilemap_scroll(layer, ey);

            // Pour chaque ligne écran, on suit la tuile courante pour éviter de recalculer
            // les infos à chaque pixel. On ne recalcule qu'au changement de tuile.
            let mut current_tile_col: Option<u32> = None;
            let mut current_info: Option<TileInfo> = None;

            for sx in CLIP.0..=CLIP.1 {
                let ex = flipped(sx as u32, 511, flip);
                let (tx, ty) = screen_to_tilemap(ex, ey, scrollx, scrolly, dim);

                // Déterminer la colonne de tuile pour ce pixel
                let tile_col = tx / tile_size;
                let tile_row = ty / tile_size;

                // Si on change de tuile (colonne ou ligne), recalculer les infos
                if current_tile_col != Some(tile_col) || current_info.is_none() {
                    let info = self.tilemap_tile(layer, tile_col, tile_row);
                    current_info = Some(info);
                    current_tile_col = Some(tile_col);
                }

                // Position dans la tuile
                let px = tx % tile_size;
                let py = ty % tile_size;

                // Décode le pixel à partir des infos de la tuile (une fois par pixel)
                if let Some(info) = current_info {
                    let p = self.tile_info_pixel(gfx, &info, px, py, masks);
                    if p.layer1 {
                        bitmap[sy * BITMAP_W + sx] = p.pen;
                    }
                }
            }
        }
    }

    /// Dessine une couche tilemap haute priorité (LAYER0 : seulement pri |= 1).
    /// Optimisation T703 : décode les infos de tuile une fois par tuile, pas par pixel.
    /// spec: docs/spec/02 §Composition ; MAME l.2958 + tilemap.cpp scanline_draw_opaque_null
    pub fn draw_tilemap_high(
        &self,
        gfx: &[u8],
        layer: usize,
        pri: &mut [u8],
        masks: &[u16; 4],
        rows: (usize, usize),
    ) {
        if !self.bases.layer_enabled[layer] {
            return;
        }

        let dim = TILEMAP_DIM[layer];
        let tile_size = match layer {
            0 => 8,
            1 => 16,
            _ => 32,
        };
        let flip = self.flip_screen();
        let (y0, y1) = band(rows);
        for sy in y0..y1 {
            let ey = flipped(sy as u32, 255, flip);
            let (scrollx, scrolly) = self.tilemap_scroll(layer, ey);

            // Pour chaque ligne écran, on suit la tuile courante pour éviter de recalculer
            // les infos à chaque pixel. On ne recalcule qu'au changement de tuile.
            let mut current_tile_col: Option<u32> = None;
            let mut current_info: Option<TileInfo> = None;

            for sx in CLIP.0..=CLIP.1 {
                let ex = flipped(sx as u32, 511, flip);
                let (tx, ty) = screen_to_tilemap(ex, ey, scrollx, scrolly, dim);

                // Déterminer la colonne de tuile pour ce pixel
                let tile_col = tx / tile_size;
                let tile_row = ty / tile_size;

                // Si on change de tuile (colonne ou ligne), recalculer les infos
                if current_tile_col != Some(tile_col) || current_info.is_none() {
                    let info = self.tilemap_tile(layer, tile_col, tile_row);
                    current_info = Some(info);
                    current_tile_col = Some(tile_col);
                }

                // Position dans la tuile
                let px = tx % tile_size;
                let py = ty % tile_size;

                // Décode le pixel à partir des infos de la tuile (une fois par pixel)
                if let Some(info) = current_info {
                    let p = self.tile_info_pixel(gfx, &info, px, py, masks);
                    if p.layer0 {
                        pri[sy * BITMAP_W + sx] |= 1;
                    }
                }
            }
        }
    }

    /// Rendu complet d'une trame (screen_update_cps1 + render_layers).
    /// spec: docs/spec/02 §Composition ; MAME l.3001-3052, l.2970-2999
    ///
    /// `stars` : région « stars » si le jeu en a une ; `frame_number` : numéro de trame MAME
    /// (`screen.frame_number()`), utilisé par le clignotement des étoiles.
    pub fn render_frame(
        &mut self,
        gfx: &[u8],
        stars: Option<&[u8]>,
        frame_number: u64,
        fb: &mut Framebuffer,
    ) {
        self.begin_frame();
        self.finish_frame(gfx, stars, frame_number, fb);
    }

    /// Début d'une trame à rendu partiel : bitmap à 0xBFF, priorités à 0, aucune ligne rendue.
    pub fn begin_frame(&mut self) {
        self.frame_bitmap.clear();
        self.frame_bitmap.resize(BITMAP_W * BITMAP_H, 0xBFF);
        self.frame_pri.clear();
        self.frame_pri.resize(BITMAP_W * BITMAP_H, 0);
        self.rendered_to = CLIP.2;
    }

    /// Rend les lignes [rendered_to, min(y_end, CLIP.3 + 1)) avec l'état vidéo courant (bases,
    /// scroll, flip, dernier sprite, masques recalculés comme screen_update_cps1).
    pub fn render_lines(
        &mut self,
        gfx: &[u8],
        stars: Option<&[u8]>,
        frame_number: u64,
        y_end: usize,
    ) {
        if y_end <= self.rendered_to {
            return;
        }
        if self.frame_bitmap.len() != BITMAP_W * BITMAP_H {
            self.begin_frame();
        }
        let rows = (self.rendered_to, y_end.min(CLIP.3 + 1));

        // flip = VIDEOCONTROL bit15 : tilemaps (T605), sprites (T205), étoiles (T600)
        let flip = self.flip_screen();

        // Get video memory base registers
        self.get_video_base();

        // Find the offset of the last sprite in the sprite table
        self.last_sprite =
            crate::sprites::find_last_sprite(&self.buffered_obj, self.cfg.bootleg_kludge);
        // CPS2 : marque de fin du tampon d'objets CPS2
        if let Some(c2) = self.cps2.as_mut() {
            c2.find_last_sprite();
        }

        // Update transparency masks
        let masks = self.update_transmasks();

        let mut bitmap = std::mem::take(&mut self.frame_bitmap);
        let mut pri = std::mem::take(&mut self.frame_pri);
        self.compose(
            gfx,
            stars,
            frame_number,
            &mut bitmap,
            &mut pri,
            masks,
            flip,
            rows,
        );
        self.frame_bitmap = bitmap;
        self.frame_pri = pri;
        self.rendered_to = y_end;
    }

    /// Fin de trame : lignes restantes puis conversion palette vers `fb`.
    pub fn finish_frame(
        &mut self,
        gfx: &[u8],
        stars: Option<&[u8]>,
        frame_number: u64,
        fb: &mut Framebuffer,
    ) {
        self.render_lines(gfx, stars, frame_number, CLIP.3 + 1);
        let bitmap = std::mem::take(&mut self.frame_bitmap);
        self.to_framebuffer(&bitmap, fb);
        self.frame_bitmap = bitmap;
    }

    /// Composition d'une bande de lignes `rows` (étoiles, couches, sprites) dans `bitmap` / `pri`,
    /// avec les bases, le dernier sprite et les masques déjà calculés pour la trame.
    /// Plusieurs bandes disjointes donnent le même résultat que la trame entière (ADR-003 2c).
    #[allow(clippy::too_many_arguments)]
    pub fn compose(
        &mut self,
        gfx: &[u8],
        stars: Option<&[u8]>,
        frame_number: u64,
        bitmap: &mut [u16],
        pri: &mut [u8],
        masks: [u16; 4],
        flip: bool,
        rows: (usize, usize),
    ) {
        // MAME l.3048 : if (m_region_stars) cps1_render_stars(...)
        if let Some(stars) = stars {
            if (self.debug_layer_mask >> 4) & 1 != 0 {
                self.render_stars(stars, bitmap, frame_number, flip, rows);
            }
        }

        // CPS2 : composition CPS2 (cps2.cpp l.1040-1102)
        if self.cps2.is_some() {
            self.cps2_render_layers(gfx, bitmap, pri, &masks, flip, rows);
            return;
        }

        // Parse layer control register
        let lc = self.regs_b[(self.cfg.cpsb.layer_control / 2) as usize];
        let l0 = (lc >> 6) & 3;
        let l1 = (lc >> 8) & 3;
        let l2 = (lc >> 10) & 3;
        let l3 = (lc >> 12) & 3;

        // Draw sequence: draw(l0); if l1==0 high(l0); draw(l1); ...
        self.draw_layer(gfx, l0 as usize, bitmap, pri, masks, flip, rows);
        if l1 == 0 {
            self.high_layer(gfx, l0 as usize, pri, masks, rows);
        }
        self.draw_layer(gfx, l1 as usize, bitmap, pri, masks, flip, rows);
        if l2 == 0 {
            self.high_layer(gfx, l1 as usize, pri, masks, rows);
        }
        self.draw_layer(gfx, l2 as usize, bitmap, pri, masks, flip, rows);
        if l3 == 0 {
            self.high_layer(gfx, l2 as usize, pri, masks, rows);
        }
        self.draw_layer(gfx, l3 as usize, bitmap, pri, masks, flip, rows);
    }

    /// Convertit le bitmap en framebuffer via la palette.
    fn to_framebuffer(&self, bitmap: &[u16], fb: &mut Framebuffer) {
        for fy in 0..SCREEN_H {
            let sy = fy + CLIP.2;
            for fx in 0..SCREEN_W {
                let sx = fx + CLIP.0;
                let pen = bitmap[sy * BITMAP_W + sx];
                fb.pixels[fy * SCREEN_W + fx] = self.palette[pen as usize];
            }
        }
    }

    /// Dessine une couche (sprites ou tilemap) selon son numéro.
    #[allow(clippy::too_many_arguments)]
    fn draw_layer(
        &mut self,
        gfx: &[u8],
        layer: usize,
        bitmap: &mut [u16],
        pri: &mut [u8],
        masks: [u16; 4],
        flip: bool,
        rows: (usize, usize),
    ) {
        // Debug layer mask check (bit0 = sprites, bit n = scroll n)
        let debug_bit = if layer == 0 { 0 } else { layer };
        if ((self.debug_layer_mask >> debug_bit) & 1) == 0 {
            return;
        }

        match layer {
            0 => self.render_sprites(gfx, bitmap, pri, flip, rows),
            1..=3 => self.draw_tilemap(gfx, layer - 1, bitmap, &masks, rows),
            _ => {}
        }
    }

    /// Dessine une couche haute priorité (LAYER0) selon son numéro.
    fn high_layer(
        &self,
        gfx: &[u8],
        layer: usize,
        pri: &mut [u8],
        masks: [u16; 4],
        rows: (usize, usize),
    ) {
        match layer {
            0 => {} // no high priority sprites
            1..=3 => self.draw_tilemap_high(gfx, layer - 1, pri, &masks, rows),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sprites::{BITMAP_H, BITMAP_W};

    /// Tuile scroll1 en (0,0) avec un seul pixel opaque en (0,0) de la tuile.
    fn single_tile(flip: bool) -> Vec<u16> {
        let mut v = Video::new("ghouls");
        v.gfxram[0] = 2; // tilemap0_scan(0, 0) = 0 : code
        v.gfxram[1] = 0x0003; // attr
        let mut gfx = vec![0xFFu8; 0x10000];
        gfx[128] = 0x7F;
        gfx[129] = 0x7F;
        gfx[130] = 0x7F;
        v.bases.layer_enabled[0] = true;
        v.bases.scrollx[0] = -64;
        v.bases.scrolly[0] = -16;
        if flip {
            v.regs_a[VIDEOCONTROL] = 0x8000;
        }
        let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
        v.draw_tilemap(
            &gfx,
            0,
            &mut bitmap,
            &[0xFFFF; 4],
            crate::sprites::FULL_ROWS,
        );
        bitmap
    }

    fn opaque(b: &[u16]) -> Vec<(usize, usize)> {
        (0..b.len())
            .filter(|&i| b[i] != 0xBFF)
            .map(|i| (i % BITMAP_W, i / BITMAP_W))
            .collect()
    }

    #[test]
    fn flip_screen_tilemap() {
        // Sans flip : tx = sx + scrollx = 0 -> sx = 64 ; ty = sy + scrolly = 0 -> sy = 16.
        assert_eq!(opaque(&single_tile(false)), vec![(64, 16)]);
        // Flip : 511 - sx + scrollx = 0 -> sx = 511 - 64 = 447 ; 255 - sy - 16 = 0 -> sy = 239,
        // coin inférieur droit de la zone visible (64..447, 16..239).
        assert_eq!(opaque(&single_tile(true)), vec![(447, 239)]);
    }
    use super::*;
    use crate::tilemap::tilemap0_scan;

    fn make_gfx() -> Vec<u8> {
        let mut gfx = vec![0xFFu8; 0x10000];
        // Tile code 4 (L8x8): set all pixels to raw pen 8 via plan bit3 only
        // L8x8: base=code*64+y*8, bytes 0-2 must be 0 for bits 0-2 of pen, byte 3=FF for bit 3
        for y in 0..8 {
            gfx[256 + y * 8] = 0x00;
            gfx[256 + y * 8 + 1] = 0x00;
            gfx[256 + y * 8 + 2] = 0x00;
            gfx[256 + y * 8 + 3] = 0xFF;
        }
        // Sprite tile code 8 (L16x16): set pixel (0,0) to raw pen 8
        gfx[1024] = 0x7F;
        gfx[1025] = 0x7F;
        gfx[1026] = 0x7F;
        gfx[1027] = 0xFF;
        gfx
    }

    #[test]
    fn test_empty_frame() {
        let mut v = Video::new("ghouls");
        // No layers enabled, no sprites
        v.regs_b[0x13] = 0;
        v.buffered_obj[3] = 0xFF00; // marker at first entry

        let gfx = make_gfx();
        let mut fb = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb);

        // All pixels should be background color (palette[0xBFF])
        for p in &fb.pixels {
            assert_eq!(*p, v.palette[0xBFF]);
        }

        // Determinism: second render produces identical output
        let mut fb2 = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb2);
        assert_eq!(fb.crc32(), fb2.crc32());
    }

    #[test]
    fn test_priority_basic() {
        let mut v = Video::new("ghouls");
        // Tile at col 8, row 2 (index 258) -> gfxram[516]=4, gfxram[517]=0x0003
        let idx = tilemap0_scan(8, 2) as usize;
        v.gfxram[idx * 2] = 4;
        v.gfxram[idx * 2 + 1] = 0x0003;

        // Sprite at (64, 16), code 8, color 1
        v.buffered_obj[0] = 64;
        v.buffered_obj[1] = 16;
        v.buffered_obj[2] = 8;
        v.buffered_obj[3] = 0x0001;
        // Marker at entry 1
        v.buffered_obj[7] = 0xFF00;

        // Layer control: enable scroll1 (bit1), l0=scroll1, l1=sprites, l2=scroll2, l3=scroll3
        v.regs_b[0x13] = 0x3842; // enable 0x02, l0=1, l1=0, l2=2, l3=3

        // Palette entries for the test colors
        v.palette[0x18] = 0xAA0000; // sprite (couleur 1, raw 8 -> pen 0x18)
        v.palette[0x238] = 0x00BB00; // tuile scroll1 (couleur 3+0x20, raw 8 -> pen 0x238)

        let gfx = make_gfx();
        let mut fb = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb);

        // Sans priorité : tuile dessinée d'abord, sprite par-dessus -> sprite
        assert_eq!(fb.pixels[0], 0xAA0000);

        // Avec priorité (regs_b[0x14] = 0x0100) : la tuile haute masque le sprite en (0,0)
        v.regs_b[0x14] = 0x0100;
        let mut fb2 = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb2);
        assert_eq!(fb2.pixels[0], 0x00BB00);

        // Pixel (1,0) : pas de pixel sprite, la tuile reste visible dans les deux cas
        assert_eq!(fb.pixels[1], 0x00BB00);
        assert_eq!(fb2.pixels[1], 0x00BB00);
    }

    #[test]
    fn test_reverse_layer_order() {
        let mut v = Video::new("ghouls");
        // Same setup as priority test but with reversed layer order
        let idx = tilemap0_scan(8, 2) as usize;
        v.gfxram[idx * 2] = 4;
        v.gfxram[idx * 2 + 1] = 0x0003;

        v.buffered_obj[0] = 64;
        v.buffered_obj[1] = 16;
        v.buffered_obj[2] = 8;
        v.buffered_obj[3] = 0x0001;
        v.buffered_obj[7] = 0xFF00;

        // l0=sprites, l1=scroll1 (reversed)
        v.regs_b[0x13] = 0x3902;

        v.palette[0x18] = 0xAA0000;
        v.palette[0x238] = 0x00BB00;

        let gfx = make_gfx();
        let mut fb = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb);

        // Sprite dessiné d'abord, tuile par-dessus -> tuile même sans priorité
        assert_eq!(fb.pixels[0], 0x00BB00);
    }

    #[test]
    fn test_debug_layer_mask() {
        let mut v = Video::new("ghouls");
        // Same setup as priority test
        let idx = tilemap0_scan(8, 2) as usize;
        v.gfxram[idx * 2] = 4;
        v.gfxram[idx * 2 + 1] = 0x0003;

        v.buffered_obj[0] = 64;
        v.buffered_obj[1] = 16;
        v.buffered_obj[2] = 8;
        v.buffered_obj[3] = 0x0001;
        v.buffered_obj[7] = 0xFF00;

        v.regs_b[0x13] = 0x3842; // l0=scroll1, l1=sprites

        v.palette[0x18] = 0xAA0000;
        v.palette[0x238] = 0x00BB00;

        // Mask out sprites (bit0)
        v.debug_layer_mask = 0xFFFF_FFFE;

        let gfx = make_gfx();
        let mut fb = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb);

        // Sprites masqués -> tuile même sans priorité
        assert_eq!(fb.pixels[0], 0x00BB00);
    }

    #[test]
    fn test_rowscroll_render() {
        use crate::mapper::{gfxrom_bank_mapper, GFXTYPE_SCROLL2};
        use crate::regs::{OTHER_BASE, ROWSCROLL_OFFS, SCROLL2_BASE, SCROLL2_X, SCROLL2_Y};

        let mut v = Video::new("ghouls");
        v.regs_a[SCROLL2_BASE] = 0x9040; // scroll2 au mot 0x2000
        v.regs_a[OTHER_BASE] = 0x9100; // table de rowscroll au mot 0x8000
        v.regs_a[SCROLL2_X] = 0;
        v.regs_a[SCROLL2_Y] = 0;
        v.regs_a[VIDEOCONTROL] = 1 | 4; // rowscroll + scroll2 actif
        v.regs_a[ROWSCROLL_OFFS] = 0x0010;
        v.buffered_obj[3] = 0xFF00; // aucun sprite
                                    // lc : enable scroll2 (0x04), l0 = 2 (scroll2), l1..l3 = 0 (sprites)
        v.regs_b[0x13] = 0x0084;

        // Tuile 16x16 entièrement raw 0 (opaque), code 8, en col 4 row 1 :
        // elle couvre la tilemap x 64..79, y 16..31.
        let mut gfx = vec![0xFFu8; 0x40000];
        let code = gfxrom_bank_mapper(&v.cfg.mapper, GFXTYPE_SCROLL2, 8);
        assert!(code >= 0);
        let t = code as usize * 128;
        gfx[t..t + 128].fill(0x00);
        let idx = crate::tilemap::tilemap1_scan(4, 1) as usize;
        v.gfxram[0x2000 + 2 * idx] = 8;
        v.gfxram[0x2000 + 2 * idx + 1] = 0x0003; // couleur 3 -> pen 0x430

        // Rowscroll de 5 sur la seule ligne 20
        v.gfxram[0x8000 + 20 + 0x10] = 5;

        v.palette[0x430] = 0xAA0000;
        v.palette[0xBFF] = 0x123456;

        let mut fb = Framebuffer::new();
        v.render_frame(&gfx, None, 0, &mut fb);

        // Oracle : tilemap_scroll donne +5 sur la ligne 20, rien sur la 21
        assert_eq!(v.tilemap_scroll(1, 20).0, 5);
        assert_eq!(v.tilemap_scroll(1, 21).0, 0);

        // Ligne 21 (fb y = 5) : tuile en sx 64..79 -> fb x 0..15
        let r21 = 5 * SCREEN_W;
        assert_eq!(fb.pixels[r21], 0xAA0000);
        assert_eq!(fb.pixels[r21 + 15], 0xAA0000);
        assert_eq!(fb.pixels[r21 + 16], 0x123456);
        // Ligne 20 (fb y = 4) : tx = sx + 5 -> tuile en sx 59..74 -> fb x 0..10
        let r20 = 4 * SCREEN_W;
        assert_eq!(fb.pixels[r20 + 10], 0xAA0000);
        assert_eq!(fb.pixels[r20 + 11], 0x123456);
    }

    /// Test T703 : l'optimisation par tuile produit le même bitmap que la boucle par pixel.
    #[test]
    fn test_tilemap_optimization_equivalence() {
        use crate::tilemap::{screen_to_tilemap, TILEMAP_DIM};

        // Graine fixe pour reproductibilité
        let seed = 42u32;
        let mut rng = seed;
        fn next_rand(rng: &mut u32) -> u32 {
            *rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            *rng >> 8
        }

        // Créer une tilemap pseudo-aléatoire avec plusieurs tuiles différentes
        let mut v = Video::new("ghouls");
        v.bases.layer_enabled[0] = true;

        // Remplir la RAM de défilement avec des codes et attributs aléatoires
        for i in 0..1024 {
            rng = next_rand(&mut rng);
            let code = (rng % 64) as u16;
            rng = next_rand(&mut rng);
            let attr = (rng % 512) as u16;
            v.gfxram[2 * i] = code;
            v.gfxram[2 * i + 1] = attr;
        }

        // Créer des données GFX aléatoires
        let mut gfx = vec![0xFFu8; 0x4000];
        for byte in gfx.iter_mut() {
            rng = next_rand(&mut rng);
            *byte = (rng % 256) as u8;
        }

        // Tester avec 4 valeurs de scroll différentes
        let scrolls = [(0, 0), (32, 16), (-64, -32), (127, 99)];
        for (sx_scroll, sy_scroll) in &scrolls {
            v.bases.scrollx[0] = *sx_scroll;
            v.bases.scrolly[0] = *sy_scroll;

            // Version optimisée (par tuile)
            let mut bitmap_optimized = vec![0xBFFu16; BITMAP_W * BITMAP_H];
            v.draw_tilemap(
                &gfx,
                0,
                &mut bitmap_optimized,
                &[0xFFFF; 4],
                crate::sprites::FULL_ROWS,
            );

            // Version de référence (par pixel, ancienne implémentation)
            let mut bitmap_reference = vec![0xBFFu16; BITMAP_W * BITMAP_H];
            for sy in CLIP.2..=CLIP.3 {
                let ey = sy as u32;
                let (scrollx, scrolly) = v.tilemap_scroll(0, ey);
                for sx in CLIP.0..=CLIP.1 {
                    let ex = sx as u32;
                    let (tx, ty) = screen_to_tilemap(ex, ey, scrollx, scrolly, TILEMAP_DIM[0]);
                    let p = v.tilemap_pixel(&gfx, 0, tx, ty, &[0xFFFF; 4]);
                    if p.layer1 {
                        bitmap_reference[sy * BITMAP_W + sx] = p.pen;
                    }
                }
            }

            // Les deux bitmaps doivent être identiques
            assert_eq!(
                bitmap_optimized, bitmap_reference,
                "Optimization mismatch at scroll ({}, {})",
                sx_scroll, sy_scroll
            );
        }
    }
}
