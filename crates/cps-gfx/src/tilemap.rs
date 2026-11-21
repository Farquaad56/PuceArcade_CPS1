//! Tilemaps CPS1 : scan, décodage pixel par pixel, transparence, défilement.
// license:BSD-3-Clause (port de ref/mame-cps1/cps1_v.cpp l.2434-2531 et ref/mame-emu/tilemap.cpp)
// copyright-holders:Barry Rodowick, Nicola Salmoria, Aaron Giles

use crate::decode::{tile_pixel, Layout};
use crate::mapper::{gfxrom_bank_mapper, GFXTYPE_SCROLL1, GFXTYPE_SCROLL2, GFXTYPE_SCROLL3};
use crate::regs::{Video, ROWSCROLL_OFFS, VIDEOCONTROL};

/// Dimensions des tilemaps (largeur = hauteur) en pixels.
pub const TILEMAP_DIM: [u32; 3] = [512, 1024, 2048];

/// Résultat du décodage d'un pixel de tuile avec ses flags de priorité.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TilePixel {
    pub pen: u16,
    pub layer0: bool,
    pub layer1: bool,
}

/// Scan tilemap0 : (col,row) -> index dans la RAM de défilement.
/// spec: docs/spec/02 §Tilemaps scroll1 ; MAME l.2434-2438
pub fn tilemap0_scan(col: u32, row: u32) -> u32 {
    (row & 0x1F) + ((col & 0x3F) << 5) + ((row & 0x20) << 6)
}

/// Scan tilemap1 : (col,row) -> index dans la RAM de défilement.
/// spec: docs/spec/02 §Tilemaps scroll2 ; MAME l.2440-2444
pub fn tilemap1_scan(col: u32, row: u32) -> u32 {
    (row & 0x0F) + ((col & 0x3F) << 4) + ((row & 0x30) << 6)
}

/// Scan tilemap2 : (col,row) -> index dans la RAM de défilement.
/// spec: docs/spec/02 §Tilemaps scroll3 ; MAME l.2446-2450
pub fn tilemap2_scan(col: u32, row: u32) -> u32 {
    (row & 0x07) + ((col & 0x3F) << 3) + ((row & 0x38) << 6)
}

impl Video {
    /// Calcule les masques de transparence pour les 4 groupes de priorité.
    /// spec: docs/spec/02 §Tilemaps ; MAME l.2515-2531
    pub fn update_transmasks(&self) -> [u16; 4] {
        let mut masks = [0xFFFFu16; 4];
        for (i, mask) in masks.iter_mut().enumerate() {
            if self.cfg.cpsb.priority[i] != -1 {
                let reg_idx = (self.cfg.cpsb.priority[i] / 2) as usize;
                *mask = self.regs_b[reg_idx] ^ 0xFFFF;
            }
        }
        masks
    }

    /// Décode un pixel dans la tilemap à la position (tx, ty).
    /// spec: docs/spec/02 §Tilemaps ; MAME l.2452-2511 + drawgfx
    pub fn tilemap_pixel(
        &self,
        gfx: &[u8],
        layer: usize,
        tx: u32,
        ty: u32,
        masks: &[u16; 4],
    ) -> TilePixel {
        let _size = TILEMAP_DIM[layer];
        let tile_size = match layer {
            0 => 8,
            1 => 16,
            _ => 32,
        };

        // Position de la tuile dans la tilemap
        let col = tx / tile_size;
        let row = ty / tile_size;

        // Index dans la RAM de défilement selon le scan de la couche
        let index = match layer {
            0 => tilemap0_scan(col, row),
            1 => tilemap1_scan(col, row),
            _ => tilemap2_scan(col, row),
        };

        // Lecture du code et des attributs (index modulo GFXRAM_WORDS)
        let scroll_base = self.bases.scroll[layer];
        let idx = (scroll_base + 2 * index as usize) % crate::regs::GFXRAM_WORDS;
        let mut code = self.gfxram[idx] as i32;
        if layer == 2 {
            code &= 0x3FFF; // scroll3 : 14 bits seulement
        }
        let attr_idx = (scroll_base + 2 * index as usize + 1) % crate::regs::GFXRAM_WORDS;
        let attr = self.gfxram[attr_idx];

        // Mapper de ROM graphique
        let kind = match layer {
            0 => GFXTYPE_SCROLL1,
            1 => GFXTYPE_SCROLL2,
            _ => GFXTYPE_SCROLL3,
        };
        code = gfxrom_bank_mapper(&self.cfg.mapper, kind, code);

        // Layout selon la couche et l'index (scroll1 alterne L8x8/L8x8b)
        let layout = match layer {
            0 => {
                if index & 0x20 == 0 {
                    Layout::L8x8
                } else {
                    Layout::L8x8b
                }
            }
            1 => Layout::L16x16,
            _ => Layout::L32x32,
        };

        // Position dans la tuile
        let mut px = tx % tile_size;
        let mut py = ty % tile_size;

        // Flip X/Y (attr bits 5 et 6)
        if attr & 0x20 != 0 {
            px = tile_size - 1 - px;
        }
        if attr & 0x40 != 0 {
            py = tile_size - 1 - py;
        }

        // Décode le pixel brut (ou 15 si code == -1)
        let raw = if code == -1 {
            15u8
        } else {
            tile_pixel(gfx, layout, code as u32, px, py)
        };

        // Couleur : (attr & 0x1F) + offset de couche
        let color_offset = match layer {
            0 => 0x20,
            1 => 0x40,
            _ => 0x60,
        };
        let color = ((attr & 0x1F) + color_offset) % 0x80;

        // Pen final : couleur * 16 + raw
        let pen = color * 16 + raw as u16;

        // Groupe de priorité (attr bits 7-8)
        let group = ((attr & 0x180) >> 7) as usize;

        // Flags de transparence : layer1 opaque si raw != 15 ;
        // layer0 opaque si bit raw du masque du groupe == 0
        let layer1 = raw != 15;
        let layer0 = ((masks[group] >> raw) & 1) == 0;

        TilePixel {
            pen,
            layer0,
            layer1,
        }
    }

    /// Retourne le défilement effectif pour une couche à la ligne écran sy.
    /// spec: docs/spec/02 §Tilemaps ; MAME l.3018-3029 (rowscroll)
    pub fn tilemap_scroll(&self, layer: usize, sy: u32) -> (i32, i32) {
        let scrollx = self.bases.scrollx[layer];
        let scrolly = self.bases.scrolly[layer];

        // Rowscroll : couche 1 si VIDEOCONTROL bit0 est actif
        if layer == 1 && (self.regs_a[VIDEOCONTROL] & 1) != 0 && sy < 256 {
            let offset = ((sy as i32 + self.regs_a[ROWSCROLL_OFFS] as i32) & 0x3FF) as usize;
            let scroll_val = self.gfxram[self.bases.other + offset] as i32;
            return (scrollx + scroll_val, scrolly);
        }

        (scrollx, scrolly)
    }
}

/// Convertit des coordonnées écran en coordonnées tilemap compte tenu du défilement.
pub fn screen_to_tilemap(sx: u32, sy: u32, scrollx: i32, scrolly: i32, dim: u32) -> (u32, u32) {
    let tx = ((sx as i32 + scrollx).rem_euclid(dim as i32)) as u32;
    let ty = ((sy as i32 + scrolly).rem_euclid(dim as i32)) as u32;
    (tx, ty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_functions() {
        assert_eq!(tilemap0_scan(0x3F, 0x3F), 4095);
        assert_eq!(tilemap1_scan(2, 0x15), 1061);
        assert_eq!(tilemap2_scan(1, 9), 521);
    }

    #[test]
    fn test_tilemap_pixel_basic() {
        let mut v = Video::new("ghouls");
        // Tuile col 1 row 0 -> index 32 (scroll1 base mot 0)
        v.gfxram[64] = 2; // code
        v.gfxram[65] = 0x0003; // attr

        let mut gfx = vec![0xFFu8; 0x10000];
        // L8x8b : x=0 -> b0..b2 bit7=0, b3 bit7=1 -> raw 8
        gfx[132] = 0x7F;
        gfx[133] = 0x7F;
        gfx[134] = 0x7F;

        let masks = [0xFFFFu16; 4];
        let p = v.tilemap_pixel(&gfx, 0, 8, 0, &masks);
        assert_eq!(p.pen, 0x238);
        assert!(!p.layer0);
        assert!(p.layer1);
    }

    #[test]
    fn test_transparency_mask() {
        let mut v = Video::new("ghouls");
        v.gfxram[64] = 2;
        v.gfxram[65] = 0x0003;

        // priority[0] = 0x28 -> regs_b[0x14]
        v.regs_b[0x14] = 0x0100;
        let masks = v.update_transmasks();
        assert_eq!(masks[0], 0xFEFF);

        let mut gfx = vec![0xFFu8; 0x10000];
        gfx[132] = 0x7F;
        gfx[133] = 0x7F;
        gfx[134] = 0x7F;

        let p = v.tilemap_pixel(&gfx, 0, 8, 0, &masks);
        assert!(p.layer0); // bit 8 de 0xFEFF est 0 -> opaque layer0
    }

    #[test]
    fn test_flip_x() {
        let mut v = Video::new("ghouls");
        v.gfxram[64] = 2;
        v.gfxram[65] = 0x0023; // flipX (bit5)

        let mut gfx = vec![0xFFu8; 0x10000];
        gfx[132] = 0x7F;
        gfx[133] = 0x7F;
        gfx[134] = 0x7F;

        let masks = [0xFFFFu16; 4];
        // Avec flipX, le pixel raw 8 apparaît en tx=15 (colonne 1 de la tuile)
        let p = v.tilemap_pixel(&gfx, 0, 15, 0, &masks);
        assert!(p.layer1);

        // En tx=8 : pas de pixel -> raw 15 -> transparent
        let p2 = v.tilemap_pixel(&gfx, 0, 8, 0, &masks);
        assert!(!p2.layer1);
    }

    #[test]
    fn test_sf2_mapper() {
        let mut v = Video::new("sf2");
        // STF29 mapper : code 0x0100 -> -1 (hors plage)
        v.gfxram[64] = 0x0100;
        v.gfxram[65] = 3;

        let gfx: Vec<u8> = vec![0xFFu8; 0x10000];
        let masks = [0xFFFFu16; 4];
        let p = v.tilemap_pixel(&gfx, 0, 8, 0, &masks);
        assert!(!p.layer1); // code -1 -> raw 15 -> transparent
        assert_eq!(p.pen, 0x23 * 16 + 15);
    }

    #[test]
    fn test_rowscroll() {
        let mut v = Video::new("ghouls");
        // Activer rowscroll (VIDEOCONTROL bit0)
        v.regs_a[VIDEOCONTROL] |= 1;
        v.regs_a[ROWSCROLL_OFFS] = 0x0010;

        // other base mot 0x8000, offset 36 -> gfxram[0x8000 + 36]
        let other_idx = v.cps1_base(4, crate::regs::OTHER_SIZE);
        v.gfxram[other_idx + 36] = 5;

        // scroll2 base X = 0x100
        v.regs_a[8] = 0x0100;
        v.get_video_base();

        let (sx, _sy) = v.tilemap_scroll(1, 20);
        assert_eq!(sx, 0x105); // 0x100 + 5

        // Sans rowscroll : pas d'ajustement
        v.regs_a[VIDEOCONTROL] &= !1;
        let (sx2, _sy2) = v.tilemap_scroll(1, 20);
        assert_eq!(sx2, 0x100);
    }

    #[test]
    fn test_screen_to_tilemap() {
        // Pas de défilement : coordonnées identiques
        let (tx, ty) = screen_to_tilemap(100, 50, 0, 0, 512);
        assert_eq!((tx, ty), (100, 50));

        // Avec défilement positif
        let (tx2, ty2) = screen_to_tilemap(100, 50, 32, 16, 512);
        assert_eq!((tx2, ty2), (132, 66));

        // Défilement négatif avec wrap-around
        let (tx3, ty3) = screen_to_tilemap(10, 5, -20, -10, 512);
        assert_eq!((tx3, ty3), (502, 507)); // rem_euclid: (-10+512)%512=502, (-5+512)%512=507
    }
}
