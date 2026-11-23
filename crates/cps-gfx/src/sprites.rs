//! Sprites CPS1 : find_last_sprite et prio_transpen16 (port MAME l.2684-2718, drawgfx.cpp l.910-938).
//! spec: docs/spec/02-video-cps-a-b.md §Sprites

use crate::decode::{tile_pixel, Layout};

pub const BITMAP_W: usize = 512;
pub const BITMAP_H: usize = 262;
pub const CLIP: (usize, usize, usize, usize) = (64, 447, 16, 239);

/// Bande d'une trame entière (lignes CLIP.2 incluse à CLIP.3 + 1 exclue), ADR-003 décision 2c.
pub const FULL_ROWS: (usize, usize) = (CLIP.2, CLIP.3 + 1);

/// Lignes dessinables d'une bande : `rows ∩ CLIP`, en (y0 inclus, y1 exclu).
pub fn band(rows: (usize, usize)) -> (usize, usize) {
    (rows.0.max(CLIP.2), rows.1.min(CLIP.3 + 1))
}

/// Trouve l'offset du dernier sprite dans le tampon obj.
/// spec: docs/spec/02-video-cps-a-b.md §Sprites (MAME l.2684-2718)
pub fn find_last_sprite(obj: &[u16], kludge: i32) -> i32 {
    let mut offset = 0;
    while offset < obj.len() {
        if (kludge & 15) == 3 {
            // captcommb - same end of sprite marker as CPS-2
            let marker = obj[offset + 1];
            if marker >= 0x8000 {
                return offset as i32 - 4;
            }
        } else {
            let marker = obj[offset + 3];
            if (marker & 0xFF00) == 0xFF00 {
                return offset as i32 - 4;
            }
        }
        offset += 4;
    }
    // Sprites must use full sprite RAM
    obj.len() as i32 - 4
}

/// Dessine une tuile sprite 16x16 avec priorité et transparence.
/// spec: docs/spec/02-video-cps-a-b.md §Sprites (MAME drawgfx.cpp l.910-938, drawgfxt.ipp l.216-227)
#[allow(clippy::too_many_arguments)]
pub fn prio_transpen16(
    gfx: &[u8],
    bitmap: &mut [u16],
    pri: &mut [u8],
    code: u32,
    color: u32,
    flipx: bool,
    flipy: bool,
    sx: i32,
    sy: i32,
    pmask: u32,
    rows: (usize, usize),
) {
    let (clip_x0, clip_x1, _, _) = CLIP;
    // bande de lignes (y1 exclu) ∩ CLIP
    let (clip_y0, y_end) = band(rows);
    // high bit of the mask is implicitly on
    let pmask = pmask | (1 << 31);
    // colorbase + granularity * (color % colors()) -> base = (color % 0x80) * 16
    let base = ((color % 0x80) as u16) << 4;

    for y in 0..16 {
        for x in 0..16 {
            // source coordinates with flip
            let src_x = if flipx { 15 - x } else { x };
            let src_y = if flipy { 15 - y } else { y };

            // destination coordinates
            let dst_x = sx + x;
            let dst_y = sy + y;

            // clip check
            if dst_x < clip_x0 as i32 || dst_x > clip_x1 as i32 {
                continue;
            }
            if dst_y < clip_y0 as i32 || dst_y >= y_end as i32 {
                continue;
            }

            let v = tile_pixel(gfx, Layout::L16x16, code, src_x as u32, src_y as u32);

            if v != 15 {
                let dst_idx = (dst_y as usize) * BITMAP_W + (dst_x as usize);
                let current_pri = pri[dst_idx];
                // PIXEL_OP_REBASE_TRANSPEN_PRIORITY: check priority, draw if allowed
                if ((1u32 << (current_pri & 0x1F)) & pmask) == 0 {
                    bitmap[dst_idx] = base + v as u16;
                }
                // always set priority to 31
                pri[dst_idx] = 31;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_last_sprite_marker_at_3() {
        let mut obj = vec![0u16; 0x400];
        obj[3] = 0xFF00;
        assert_eq!(find_last_sprite(&obj, 0), -4);
    }

    #[test]
    fn find_last_sprite_marker_at_7() {
        let mut obj = vec![0u16; 0x400];
        obj[3] = 0;
        obj[7] = 0xFF00;
        assert_eq!(find_last_sprite(&obj, 0), 0);
    }

    #[test]
    fn find_last_sprite_no_marker() {
        let obj = vec![0u16; 0x400];
        assert_eq!(find_last_sprite(&obj, 0), 0x3FC);
    }

    #[test]
    fn find_last_sprite_kludge_3() {
        let mut obj = vec![0u16; 0x400];
        obj[5] = 0x8000; // marker at offset+1 for sprite at offset 4
        assert_eq!(find_last_sprite(&obj, 3), 0);
    }

    #[test]
    fn prio_transpen_basic() {
        let mut gfx = vec![0xFFu8; 0x8000];
        // tile code 1: set pixel (0,0) to raw pen 8 via plan bit3 only
        gfx[128] = 0x7F;
        gfx[128 + 1] = 0x7F;
        gfx[128 + 2] = 0x7F;
        gfx[128 + 3] = 0xFF;

        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        prio_transpen16(
            &gfx,
            &mut bitmap,
            &mut pri,
            1,
            2,
            false,
            false,
            100,
            50,
            0x02,
            FULL_ROWS,
        );

        // pixel (0,0) raw=8 -> pen = base(32) + 8 = 40 = 0x28
        let idx = 50 * BITMAP_W + 100;
        assert_eq!(bitmap[idx], 0x28);
        assert_eq!(pri[idx], 31);

        // pixel (1,0) is transparent (raw=15), unchanged
        let idx2 = 50 * BITMAP_W + 101;
        assert_eq!(bitmap[idx2], 0);
    }

    #[test]
    fn prio_transpen_priority_blocks() {
        let mut gfx = vec![0xFFu8; 0x8000];
        gfx[128] = 0x7F;
        gfx[128 + 1] = 0x7F;
        gfx[128 + 2] = 0x7F;
        gfx[128 + 3] = 0xFF; // pixel (0,0) raw=8

        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        let idx = 50 * BITMAP_W + 100;
        pri[idx] = 1; // set priority to block (pmask=0x02, bit 1 is set)

        prio_transpen16(
            &gfx,
            &mut bitmap,
            &mut pri,
            1,
            2,
            false,
            false,
            100,
            50,
            0x02,
            FULL_ROWS,
        );

        // blocked: bitmap unchanged
        assert_eq!(bitmap[idx], 0);
        // but priority still set to 31
        assert_eq!(pri[idx], 31);
    }

    #[test]
    fn prio_transpen_flipx() {
        let mut gfx = vec![0xFFu8; 0x8000];
        gfx[128] = 0x7F;
        gfx[128 + 1] = 0x7F;
        gfx[128 + 2] = 0x7F;
        gfx[128 + 3] = 0xFF; // pixel (0,0) raw=8

        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        prio_transpen16(
            &gfx,
            &mut bitmap,
            &mut pri,
            1,
            2,
            true,
            false,
            100,
            50,
            0x02,
            FULL_ROWS,
        );

        // with flipx, pixel (0,0) source goes to dest x=100+15=115
        let idx = 50 * BITMAP_W + 115;
        assert_eq!(bitmap[idx], 0x28);
    }

    #[test]
    fn prio_transpen_clip_right() {
        let mut gfx = vec![0xFFu8; 0x8000];
        gfx[128] = 0x7F;
        gfx[128 + 1] = 0x7F;
        gfx[128 + 2] = 0x7F;
        gfx[128 + 3] = 0xFF; // pixel (0,0) raw=8

        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        // sx=440: only x<=7 fit in clip (clip_x1=447)
        prio_transpen16(
            &gfx,
            &mut bitmap,
            &mut pri,
            1,
            2,
            false,
            false,
            440,
            50,
            0x02,
            FULL_ROWS,
        );

        // pixel at dest x=440 should be drawn
        let idx = 50 * BITMAP_W + 440;
        assert_eq!(bitmap[idx], 0x28);

        // pixel at dest x=448 (source x=8) is clipped
        let idx2 = 50 * BITMAP_W + 448;
        assert_eq!(bitmap[idx2], 0);
    }

    #[test]
    fn prio_transpen_clip_outside() {
        let mut gfx = vec![0xFFu8; 0x8000];
        gfx[128] = 0x7F;
        gfx[128 + 1] = 0x7F;
        gfx[128 + 2] = 0x7F;
        gfx[128 + 3] = 0xFF; // pixel (0,0) raw=8

        let mut bitmap = vec![0u16; BITMAP_W * BITMAP_H];
        let mut pri = vec![0u8; BITMAP_W * BITMAP_H];

        // sx=504: completely outside clip (clip_x1=447)
        prio_transpen16(
            &gfx,
            &mut bitmap,
            &mut pri,
            1,
            2,
            false,
            false,
            504,
            50,
            0x02,
            FULL_ROWS,
        );

        // nothing drawn
        let idx = 50 * BITMAP_W + 504;
        assert_eq!(bitmap[idx], 0);
    }
}
