// license:BSD-3-Clause (port de ref/mame-cps1/cps1_v.cpp l.2720-2866)
// copyright-holders: Paul Leaman
//! Rendu des sprites CPS1 (spec: docs/spec/02 §Sprites)

use crate::mapper::{gfxrom_bank_mapper, GFXTYPE_SPRITES};
use crate::regs::Video;
use crate::sprites::prio_transpen16;

impl Video {
    /// Rendu des sprites (port de cps1_render_sprites, l.2720-2866).
    pub fn render_sprites(&mut self, gfx: &[u8], bitmap: &mut [u16], pri: &mut [u8], flip: bool) {
        let base = &self.buffered_obj;

        // Determine iteration order based on bootleg_kludge bit 6
        let mut base_offset = 0usize;
        if (self.cfg.bootleg_kludge >> 6) & 1 != 0 {
            // Reverse order for some sf2 hacks
            base_offset = self.last_sprite as usize;
        }

        let num_entries = (self.last_sprite / 4) + 1;
        for _ in 0..num_entries {
            let x = base[base_offset] as i32;
            let y = base[base_offset + 1] as i32;
            let code = base[base_offset + 2] as i32;
            let colour = base[base_offset + 3];

            let col = (colour & 0x1F) as u32;
            let flipx = (colour >> 5) & 1 != 0;
            let flipy = (colour >> 6) & 1 != 0;

            // Map code through gfxrom_bank_mapper
            let mapped_code = gfxrom_bank_mapper(&self.cfg.mapper, GFXTYPE_SPRITES, code);

            if mapped_code != -1 {
                if colour & 0xFF00 != 0 {
                    // Block sprite
                    let nx = ((colour >> 8) & 0xF) as i32 + 1;
                    let ny = ((colour >> 12) & 0xF) as i32 + 1;

                    if flipy {
                        if flipx {
                            // Y flip, X flip
                            for nys in 0..ny {
                                let sy = (y + nys * 16) & 0x1FF;
                                for nxs in 0..nx {
                                    let sx = (x + nxs * 16) & 0x1FF;
                                    let sub_code =
                                        (mapped_code & !0xF) | ((mapped_code + nx - 1 - nxs) & 0xF);
                                    let sub_code = sub_code | (0x10 * (ny - 1 - nys));
                                    self.draw_sprite(
                                        gfx,
                                        bitmap,
                                        pri,
                                        flip,
                                        sub_code as u32,
                                        col,
                                        true,
                                        true,
                                        sx,
                                        sy,
                                    );
                                }
                            }
                        } else {
                            // Y flip only
                            for nys in 0..ny {
                                let sy = (y + nys * 16) & 0x1FF;
                                for nxs in 0..nx {
                                    let sx = (x + nxs * 16) & 0x1FF;
                                    let sub_code =
                                        (mapped_code & !0xF) | ((mapped_code + nxs) & 0xF);
                                    let sub_code = sub_code | (0x10 * (ny - 1 - nys));
                                    self.draw_sprite(
                                        gfx,
                                        bitmap,
                                        pri,
                                        flip,
                                        sub_code as u32,
                                        col,
                                        false,
                                        true,
                                        sx,
                                        sy,
                                    );
                                }
                            }
                        }
                    } else {
                        if flipx {
                            // X flip only
                            for nys in 0..ny {
                                let sy = (y + nys * 16) & 0x1FF;
                                for nxs in 0..nx {
                                    let sx = (x + nxs * 16) & 0x1FF;
                                    let sub_code =
                                        (mapped_code & !0xF) | ((mapped_code + nx - 1 - nxs) & 0xF);
                                    let sub_code = sub_code | (0x10 * nys);
                                    self.draw_sprite(
                                        gfx,
                                        bitmap,
                                        pri,
                                        flip,
                                        sub_code as u32,
                                        col,
                                        true,
                                        false,
                                        sx,
                                        sy,
                                    );
                                }
                            }
                        } else {
                            // No flip
                            for nys in 0..ny {
                                let sy = (y + nys * 16) & 0x1FF;
                                for nxs in 0..nx {
                                    let sx = (x + nxs * 16) & 0x1FF;
                                    let sub_code =
                                        (mapped_code & !0xF) | ((mapped_code + nxs) & 0xF);
                                    let sub_code = sub_code | (0x10 * nys);
                                    self.draw_sprite(
                                        gfx,
                                        bitmap,
                                        pri,
                                        flip,
                                        sub_code as u32,
                                        col,
                                        false,
                                        false,
                                        sx,
                                        sy,
                                    );
                                }
                            }
                        }
                    }
                } else {
                    // Simple sprite
                    let sx = x & 0x1FF;
                    let sy = y & 0x1FF;
                    self.draw_sprite(
                        gfx,
                        bitmap,
                        pri,
                        flip,
                        mapped_code as u32,
                        col,
                        flipx,
                        flipy,
                        sx,
                        sy,
                    );
                }
            }

            // Advance to next entry
            if (self.cfg.bootleg_kludge >> 6) & 1 != 0 {
                base_offset -= 4;
            } else {
                base_offset += 4;
            }
        }
    }

    /// Macro DRAWSPRITE translated to a method.
    #[allow(clippy::too_many_arguments)]
    fn draw_sprite(
        &self,
        gfx: &[u8],
        bitmap: &mut [u16],
        pri: &mut [u8],
        flip: bool,
        code: u32,
        color: u32,
        flipx: bool,
        flipy: bool,
        sx: i32,
        sy: i32,
    ) {
        if flip {
            prio_transpen16(
                gfx,
                bitmap,
                pri,
                code,
                color,
                !flipx,
                !flipy,
                512 - 16 - sx,
                256 - 16 - sy,
                0x02,
            );
        } else {
            prio_transpen16(gfx, bitmap, pri, code, color, flipx, flipy, sx, sy, 0x02);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprites::{BITMAP_H, BITMAP_W};

    fn make_gfx() -> Vec<u8> {
        let mut gfx = vec![0xFFu8; 0x10000];
        // Tile code 1: set pixel (0,0) to raw pen 8 via plan bit3 only
        gfx[128] = 0x7F;
        gfx[129] = 0x7F;
        gfx[130] = 0x7F;
        gfx[131] = 0xFF;
        // Tile code 2: set pixel (0,0) to raw pen 8
        gfx[256] = 0x7F;
        gfx[257] = 0x7F;
        gfx[258] = 0x7F;
        gfx[259] = 0xFF;
        // Tile code 16 (0x10): set pixel (0,0) to raw pen 8
        gfx[2048] = 0x7F;
        gfx[2049] = 0x7F;
        gfx[2050] = 0x7F;
        gfx[2051] = 0xFF;
        // Tile code 17 (0x11): set pixel (0,0) to raw pen 8
        gfx[2176] = 0x7F;
        gfx[2177] = 0x7F;
        gfx[2178] = 0x7F;
        gfx[2179] = 0xFF;
        // Tile code 32 (0x20): set pixel (0,0) to raw pen 8
        gfx[4096] = 0x7F;
        gfx[4097] = 0x7F;
        gfx[4098] = 0x7F;
        gfx[4099] = 0xFF;
        gfx
    }

    #[test]
    fn render_simple_sprite() {
        let mut v = Video::new("ghouls");
        // Entry 0: x=100, y=50, code=1, attr=0x0002 (color 2)
        v.buffered_obj[0] = 100;
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 1;
        v.buffered_obj[3] = 0x0002;
        // Entry 1: marker to end sprite list
        v.buffered_obj[7] = 0xFF00;
        v.last_sprite = 0;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        v.render_sprites(&gfx, &mut bitmap, &mut pri, false);

        // Pixel at (100, 50) should be drawn with color base 2*16=32 + pen 8 = 40 = 0x28
        let idx = 50 * BITMAP_W + 100;
        assert_eq!(bitmap[idx], 0x28);
    }

    #[test]
    fn render_sprite_order() {
        let mut v = Video::new("ghouls");
        // Entry 0: color 2 at (100, 50)
        v.buffered_obj[0] = 100;
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 1;
        v.buffered_obj[3] = 0x0002;
        // Entry 1: color 3 at same position (drawn later, should be blocked by pri=31)
        v.buffered_obj[4] = 100;
        v.buffered_obj[5] = 50;
        v.buffered_obj[6] = 2;
        v.buffered_obj[7] = 0x0003;
        // Entry 2: marker
        v.buffered_obj[11] = 0xFF00;
        v.last_sprite = 8;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        v.render_sprites(&gfx, &mut bitmap, &mut pri, false);

        // Entry 0 drawn first sets pri=31, entry 1 blocked -> pixel is color 2 (0x28)
        let idx = 50 * BITMAP_W + 100;
        assert_eq!(bitmap[idx], 0x28);
    }

    #[test]
    fn render_block_no_flip() {
        let mut v = Video::new("ghouls");
        // Block sprite: nx=2, ny=1 (attr bits 8-11 = 1 -> nx=2)
        v.buffered_obj[0] = 100;
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 0x10; // code for tile 16
        v.buffered_obj[3] = 0x0102; // nx=2, color 2
                                    // Marker
        v.buffered_obj[7] = 0xFF00;
        v.last_sprite = 0;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        v.render_sprites(&gfx, &mut bitmap, &mut pri, false);

        // Sub-sprite 0: code 0x10 at (100, 50) -> pen 0x28
        let idx0 = 50 * BITMAP_W + 100;
        assert_eq!(bitmap[idx0], 0x28);

        // Sub-sprite 1: code 0x11 at (116, 50) -> pen 0x28
        let idx1 = 50 * BITMAP_W + 116;
        assert_eq!(bitmap[idx1], 0x28);
    }

    #[test]
    fn render_block_flipx() {
        let mut v = Video::new("ghouls");
        // Block sprite with flipX: nx=2, ny=1, flipX bit set
        v.buffered_obj[0] = 100;
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 0x10;
        v.buffered_obj[3] = 0x0122; // nx=2, flipX (bit 5), color 2
                                    // Marker
        v.buffered_obj[7] = 0xFF00;
        v.last_sprite = 0;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        v.render_sprites(&gfx, &mut bitmap, &mut pri, false);

        // With flipX on sub-sprites: source pixel (0,0) maps to dest x+15
        // Sub-sprite 0 at (100, 50): pixel appears at (115, 50)
        let idx0 = 50 * BITMAP_W + 115;
        assert_eq!(bitmap[idx0], 0x28);

        // Sub-sprite 1 at (116, 50): pixel appears at (131, 50)
        let idx1 = 50 * BITMAP_W + 131;
        assert_eq!(bitmap[idx1], 0x28);
    }

    #[test]
    fn render_block_flipy() {
        let mut v = Video::new("ghouls");
        // Block sprite with flipY: nx=1, ny=2 (attr bits 12-15 = 1 -> ny=2)
        v.buffered_obj[0] = 100;
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 0x10;
        v.buffered_obj[3] = 0x1002; // ny=2, color 2
                                    // Marker
        v.buffered_obj[7] = 0xFF00;
        v.last_sprite = 0;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        v.render_sprites(&gfx, &mut bitmap, &mut pri, false);

        // Sub-sprite 0: code 0x10 at (100, 50) -> pen 0x28
        let idx0 = 50 * BITMAP_W + 100;
        assert_eq!(bitmap[idx0], 0x28);

        // Sub-sprite 1: code 0x20 at (100, 66) -> pen 0x28
        let idx1 = 66 * BITMAP_W + 100;
        assert_eq!(bitmap[idx1], 0x28);
    }

    #[test]
    fn render_screen_flip() {
        let mut v = Video::new("ghouls");
        // Simple sprite at (100, 50) with flips set so they cancel with screen flip
        v.buffered_obj[0] = 100;
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 1;
        v.buffered_obj[3] = 0x0062; // color 2, flipx=true (bit5), flipy=true (bit6)
                                    // Marker
        v.buffered_obj[7] = 0xFF00;
        v.last_sprite = 0;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        // Render with screen flip: flips become !true=false, so no effective flips
        v.render_sprites(&gfx, &mut bitmap, &mut pri, true);

        // With screen flip: position becomes (512-16-100, 256-16-50) = (396, 190)
        let idx = 190 * BITMAP_W + 396;
        assert_eq!(bitmap[idx], 0x28);
    }

    #[test]
    fn render_sprite_clipped() {
        let mut v = Video::new("ghouls");
        // Sprite at x=504 (outside clip region, clip_x1=447)
        v.buffered_obj[0] = 0x3F8; // 504 in hex
        v.buffered_obj[1] = 50;
        v.buffered_obj[2] = 1;
        v.buffered_obj[3] = 0x0002;
        // Marker
        v.buffered_obj[7] = 0xFF00;
        v.last_sprite = 0;

        let gfx = make_gfx();
        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        v.render_sprites(&gfx, &mut bitmap, &mut pri, false);

        // Nothing should be drawn (completely outside clip)
        let idx = 50 * BITMAP_W + 504;
        assert_eq!(bitmap[idx], 0);
    }
}
