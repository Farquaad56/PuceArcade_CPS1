// license:BSD-3-Clause
// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald
//! Composition CPS2 : trois couches tilemap avec code de priorité (pri |= 1, 2, 4), puis les
//! sprites CPS2 filtrés par les primasks. spec: docs/spec/05 §Vidéo (composition)
//! source: ref/mame-cps2/cps2.cpp l.1040-1102 (cps2_state::render_layers) ;
//!         ref/mame-emu/tilemap.cpp l.188-210 (scanline_draw_masked_ind16 : pri = (pri & 0xFF) | pcode)

use crate::regs::Video;
use crate::sprites::{band, BITMAP_W, CLIP};
use crate::tilemap::{screen_to_tilemap, TILEMAP_DIM};

/// Masques de priorité des sprites (l.1076-1095), après le retrait de la couche sprites CPS1.
pub fn cps2_primasks(l0pri: u32, l1pri: u32, l2pri: u32) -> [u32; 8] {
    let mut primasks = [0u32; 8];
    let mut mask0 = 0xaa;
    let mut mask1 = 0xcc;
    if l0pri > l1pri {
        mask0 &= !0x88;
    }
    if l0pri > l2pri {
        mask0 &= !0xa0;
    }
    if l1pri > l2pri {
        mask1 &= !0xc0;
    }

    primasks[0] = 0xff;
    for (i, m) in primasks.iter_mut().enumerate().skip(1) {
        let i = i as u32;
        if i <= l0pri && i <= l1pri && i <= l2pri {
            *m = 0xfe;
            continue;
        }
        *m = 0;
        if i <= l0pri {
            *m |= mask0;
        }
        if i <= l1pri {
            *m |= mask1;
        }
        if i <= l2pri {
            *m |= 0xf0;
        }
    }
    primasks
}

impl Video {
    /// Ordre des couches et primasks (l.1043-1095) : layer control du CPS-B, priorités de
    /// `cps2.pri_ctrl`, retrait de la couche sprites CPS1 (0).
    pub fn cps2_layer_order(&self) -> ([usize; 3], [u32; 8]) {
        let pri_ctrl = self.cps2.as_ref().map_or(0, |c| u32::from(c.pri_ctrl));
        let layercontrol = self.regs_b[(self.cfg.cpsb.layer_control / 2) as usize];
        let mut l0 = usize::from((layercontrol >> 0x06) & 0x03);
        let mut l1 = usize::from((layercontrol >> 0x08) & 0x03);
        let mut l2 = usize::from((layercontrol >> 0x0a) & 0x03);
        let mut l3 = usize::from((layercontrol >> 0x0c) & 0x03);

        let mut l0pri = (pri_ctrl >> (4 * l0)) & 0x0f;
        let mut l1pri = (pri_ctrl >> (4 * l1)) & 0x0f;
        let mut l2pri = (pri_ctrl >> (4 * l2)) & 0x0f;
        let l3pri = (pri_ctrl >> (4 * l3)) & 0x0f;

        // take out the CPS1 sprites layer
        if l0 == 0 {
            l0 = l1;
            l1 = 0;
            l0pri = l1pri;
        }
        if l1 == 0 {
            l1 = l2;
            l2 = 0;
            l1pri = l2pri;
        }
        if l2 == 0 {
            l2 = l3;
            l3 = 0;
            l2pri = l3pri;
        }
        let _ = l3;
        ([l0, l1, l2], cps2_primasks(l0pri, l1pri, l2pri))
    }

    /// draw_tilemap avec code de priorité : `pri |= primask` sur chaque pixel LAYER1 dessiné.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_tilemap_pri(
        &self,
        gfx: &[u8],
        layer: usize,
        bitmap: &mut [u16],
        pri: &mut [u8],
        masks: &[u16; 4],
        primask: u8,
        rows: (usize, usize),
    ) {
        if !self.bases.layer_enabled[layer] {
            return;
        }
        let dim = TILEMAP_DIM[layer];
        let flip = self.flip_screen();
        let (y0, y1) = band(rows);
        for sy in y0..y1 {
            let ey = if flip { 255 - sy as u32 } else { sy as u32 };
            let (scrollx, scrolly) = self.tilemap_scroll(layer, ey);
            for sx in CLIP.0..=CLIP.1 {
                let ex = if flip { 511 - sx as u32 } else { sx as u32 };
                let (tx, ty) = screen_to_tilemap(ex, ey, scrollx, scrolly, dim);
                let p = self.tilemap_pixel(gfx, layer, tx, ty, masks);
                if p.layer1 {
                    let i = sy * BITMAP_W + sx;
                    bitmap[i] = p.pen;
                    pri[i] |= primask;
                }
            }
        }
    }

    /// cps1_render_layer appelé par le CPS2 : 0 = sprites CPS1, 1-3 = tilemaps avec primask.
    #[allow(clippy::too_many_arguments)]
    fn cps2_render_layer(
        &mut self,
        gfx: &[u8],
        layer: usize,
        primask: u8,
        bitmap: &mut [u16],
        pri: &mut [u8],
        masks: &[u16; 4],
        flip: bool,
        rows: (usize, usize),
    ) {
        let debug_bit = if layer == 0 { 0 } else { layer };
        if (self.debug_layer_mask >> debug_bit) & 1 == 0 {
            return;
        }
        match layer {
            0 => self.render_sprites(gfx, bitmap, pri, flip, rows),
            1..=3 => self.draw_tilemap_pri(gfx, layer - 1, bitmap, pri, masks, primask, rows),
            _ => {}
        }
    }

    /// cps2_state::render_layers (l.1040-1102). `pri` doit être rempli de 0.
    pub fn cps2_render_layers(
        &mut self,
        gfx: &[u8],
        bitmap: &mut [u16],
        pri: &mut [u8],
        masks: &[u16; 4],
        flip: bool,
        rows: (usize, usize),
    ) {
        let ([l0, l1, l2], primasks) = self.cps2_layer_order();
        self.cps2_render_layer(gfx, l0, 1, bitmap, pri, masks, flip, rows);
        self.cps2_render_layer(gfx, l1, 2, bitmap, pri, masks, flip, rows);
        self.cps2_render_layer(gfx, l2, 4, bitmap, pri, masks, flip, rows);
        if self.debug_layer_mask & 1 != 0 {
            self.cps2_render_sprites(gfx, bitmap, pri, &primasks, flip, rows);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprites::BITMAP_H;

    #[test]
    fn primasks_values() {
        assert_eq!(cps2_primasks(1, 2, 3), [0xFF, 0xFE, 0xFC, 0xF0, 0, 0, 0, 0]);
        assert_eq!(
            cps2_primasks(0, 5, 3),
            [0xFF, 0xFC, 0xFC, 0xFC, 0x0C, 0x0C, 0, 0]
        );
    }

    fn video(lc: u16, pri_ctrl: u16) -> Video {
        let mut v = Video::new_cps2("ssf2ud");
        let w = (v.cfg.cpsb.layer_control / 2) as usize;
        v.regs_b[w] = lc;
        v.cps2.as_mut().unwrap().pri_ctrl = pri_ctrl;
        v
    }

    #[test]
    fn layer_order() {
        let v = video(0x0E40, 0x3210);
        assert_eq!(v.cps2_layer_order(), ([1, 2, 3], cps2_primasks(1, 2, 3)));
        let v = video(0x12C0, 0x0531);
        assert_eq!(v.cps2_layer_order(), ([3, 2, 1], cps2_primasks(0, 5, 3)));
    }

    #[test]
    fn tilemap_priority_code() {
        let mut v = Video::new_cps2("ssf2ud");
        // même scène que render.rs single_tile : scroll1 (0, 0), tuile 8 × 8 n° 2, pixel (0, 0) opaque
        v.bases.layer_enabled[0] = true;
        v.bases.scrollx[0] = -64;
        v.bases.scrolly[0] = -16;
        v.gfxram[0] = 2;
        v.gfxram[1] = 0x0003;
        let mut gfx = vec![0xFFu8; 0x10000];
        gfx[128..131].fill(0x7F);
        let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
        let mut pri = vec![1u8; BITMAP_W * BITMAP_H];
        let masks = v.update_transmasks();
        v.draw_tilemap_pri(
            &gfx,
            0,
            &mut bitmap,
            &mut pri,
            &masks,
            4,
            crate::sprites::FULL_ROWS,
        );
        let drawn: Vec<usize> = (0..bitmap.len()).filter(|&i| bitmap[i] != 0xBFF).collect();
        assert!(!drawn.is_empty());
        assert!(drawn.iter().all(|&i| pri[i] == 5));
        let blank = (0..bitmap.len()).find(|&i| bitmap[i] == 0xBFF).unwrap();
        assert_eq!(pri[blank], 1);
    }
}
