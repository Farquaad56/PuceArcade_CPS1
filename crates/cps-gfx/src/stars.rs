//! Étoiles CPS1 (Forgotten Worlds…).
// license:BSD-3-Clause
// copyright-holders:Paul Leaman
// source: ref/mame-cps1/cps1_v.cpp l.2869-2931 (cps1_render_stars)

use crate::regs::Video;
use crate::sprites::{BITMAP_W, CLIP};

/// m_stars_rom_size (l.2543) : seuls les 0x2000 premiers octets comptent.
pub const STARS_ROM_SIZE: usize = 0x2000;

impl Video {
    /// spec: docs/spec/02 §Étoiles et flip écran — port 1:1 de cps1_render_stars.
    pub fn render_stars(&self, stars: &[u8], bitmap: &mut [u16], frame_number: u64, flip: bool) {
        // PORT-NOTE: `!stars_rom && enabled` -> popmessage + return ; ici une région vide.
        if stars.is_empty() {
            return;
        }
        let b = &self.bases;
        // Couche 0 (m_stars_enabled[0]) lit stars_rom[8*offs + 4] avec starsx/y[1], pens 0xA00
        if b.stars_enabled[0] {
            self.star_layer(stars, bitmap, frame_number, flip, 4, 1, 0xA00);
        }
        // Couche 1 (m_stars_enabled[1]) lit stars_rom[8*offs] avec starsx/y[0], pens 0x800
        if b.stars_enabled[1] {
            self.star_layer(stars, bitmap, frame_number, flip, 0, 0, 0x800);
        }
    }

    // PORT-NOTE: les deux boucles identiques de MAME (l.2879-2903 et 2905-2929) sont factorisées
    // par (octet lu, index de scroll, pen de base) ; le calcul est inchangé.
    #[allow(clippy::too_many_arguments)]
    fn star_layer(
        &self,
        stars: &[u8],
        bitmap: &mut [u16],
        frame_number: u64,
        flip: bool,
        byte: usize,
        scroll: usize,
        pen_base: u16,
    ) {
        let (x0, x1, y0, y1) = CLIP;
        for offs in 0..STARS_ROM_SIZE / 2 {
            let Some(&c) = stars.get(8 * offs + byte) else {
                return;
            };
            let mut col = i32::from(c);
            if (col & 0x1f) == 0x0f {
                continue;
            }
            let mut sx = (offs as i32 / 256) * 32;
            let mut sy = offs as i32 % 256;
            sx = (sx - self.bases.starsx[scroll] + (col & 0x1f)) & 0x1ff;
            sy = (sy - self.bases.starsy[scroll]) & 0xff;
            if flip {
                sx = 512 - sx;
                sy = 256 - sy;
            }
            let period = if col & 0x80 != 0 { 15 } else { 16 };
            let cnt = ((frame_number / 16) % period) as i32;
            col = ((col & 0xe0) >> 1) + cnt;
            let (ux, uy) = (sx as usize, sy as usize);
            if (x0..=x1).contains(&ux) && (y0..=y1).contains(&uy) {
                bitmap[uy * BITMAP_W + ux] = pen_base + col as u16;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::regs::Video;
    use crate::sprites::{BITMAP_H, BITMAP_W};

    fn rom() -> Vec<u8> {
        let mut r = vec![0x0F; 0x8000];
        r[8 * (256 * 3 + 40)] = 0x05;
        r
    }

    #[test]
    fn card_value() {
        let mut v = Video::new("forgottn");
        v.bases.stars_enabled = [false, true];
        v.bases.starsx = [0, 0];
        v.bases.starsy = [0, 0];
        let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
        v.render_stars(&rom(), &mut bitmap, 32, false);
        assert_eq!(bitmap[40 * 512 + 101], 0x802);
        assert_eq!(bitmap.iter().filter(|&&p| p != 0xBFF).count(), 1);
    }

    #[test]
    fn flip_and_disabled() {
        let mut v = Video::new("forgottn");
        v.bases.stars_enabled = [false, true];
        let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
        v.render_stars(&rom(), &mut bitmap, 0, true);
        // flip : sx = 512 - 101 = 411, sy = 256 - 40 = 216 ; cnt = 0
        assert_eq!(bitmap[216 * 512 + 411], 0x800);
        v.bases.stars_enabled = [false, false];
        let mut b2 = vec![0xBFFu16; BITMAP_W * BITMAP_H];
        v.render_stars(&rom(), &mut b2, 0, false);
        assert!(b2.iter().all(|&p| p == 0xBFF));
    }

    #[test]
    fn blink_period_15_when_bit7() {
        let mut v = Video::new("forgottn");
        v.bases.stars_enabled = [true, false];
        let mut r = vec![0x0F; 0x8000];
        r[4] = 0x80 | 0x10; // offs 0, couche 0 : sx = 0x10 (hors clip) -> déplacer via starsx
        v.bases.starsx[1] = -100; // sx = 0 + 100 + 16 = 116
        v.bases.starsy[1] = -20; // sy = 20
        let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
        v.render_stars(&r, &mut bitmap, 16 * 15, false);
        // cnt = 15 % 15 = 0 ; col = (0x80 >> 1) + 0 = 0x40
        assert_eq!(bitmap[20 * 512 + 116], 0xA40);
    }
}
