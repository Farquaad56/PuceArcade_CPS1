// license:BSD-3-Clause
// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald
//! Rendu des sprites CPS2 : codes directs (sans mapper), priorité par sprite (primasks), blocs.
//! spec: docs/spec/05 §Vidéo (sprites)
//! source: ref/mame-cps2/cps2.cpp l.896-1038 (cps2_render_sprites)

use crate::cps2::Cps2Video;
use crate::regs::Video;
use crate::sprites::prio_transpen16;

/// DRAWSPRITE (l.898-914). `pmask` = primasks[priority].
#[allow(clippy::too_many_arguments)]
fn draw_sprite2(
    gfx: &[u8],
    bitmap: &mut [u16],
    pri: &mut [u8],
    code: i32,
    color: i32,
    flipx: bool,
    flipy: bool,
    sx: i32,
    sy: i32,
    pmask: u32,
    flip: bool,
    rows: (usize, usize),
) {
    let (code, color) = (code as u32, color as u32);
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
            pmask,
            rows,
        );
    } else {
        prio_transpen16(
            gfx, bitmap, pri, code, color, flipx, flipy, sx, sy, pmask, rows,
        );
    }
}

/// Boucle des blocs (l.944-1023) : `tile(nxs, nys)` donne le code de chaque tuile.
#[allow(clippy::too_many_arguments)]
fn draw_block(
    gfx: &[u8],
    bitmap: &mut [u16],
    pri: &mut [u8],
    (x, y, xoffs, yoffs): (i32, i32, i32, i32),
    (nx, ny): (i32, i32),
    col: i32,
    (flipx, flipy): (bool, bool),
    pmask: u32,
    flip: bool,
    rows: (usize, usize),
    tile: impl Fn(i32, i32) -> i32,
) {
    for nys in 0..ny {
        for nxs in 0..nx {
            let sx = (x + nxs * 16 + xoffs) & 0x3ff;
            let sy = (y + nys * 16 + yoffs) & 0x3ff;
            draw_sprite2(
                gfx,
                bitmap,
                pri,
                tile(nxs, nys),
                col & 0x1f,
                flipx,
                flipy,
                sx,
                sy,
                pmask,
                flip,
                rows,
            );
        }
    }
}

impl Video {
    /// cps2_render_sprites (l.896-1038). Sans effet si l'état CPS2 est absent.
    pub fn cps2_render_sprites(
        &self,
        gfx: &[u8],
        bitmap: &mut [u16],
        pri: &mut [u8],
        primasks: &[u32; 8],
        flip: bool,
        rows: (usize, usize),
    ) {
        let Some(c2) = self.cps2.as_deref() else {
            return;
        };
        render(c2, gfx, bitmap, pri, primasks, flip, rows);
    }
}

fn render(
    c2: &Cps2Video,
    gfx: &[u8],
    bitmap: &mut [u16],
    pri: &mut [u8],
    primasks: &[u32; 8],
    flip: bool,
    rows: (usize, usize),
) {
    let base = &c2.buffered_obj;
    let xoffs = 64 - i32::from(c2.output[4]);
    let yoffs = 16 - i32::from(c2.output[5]);

    let mut i = c2.last_sprite;
    while i >= 0 {
        let b = i as usize;
        let mut x = i32::from(base[b]);
        let mut y = i32::from(base[b + 1]);
        let priority = ((x >> 13) & 0x07) as usize;
        let code = i32::from(base[b + 2]) + ((y & 0x6000) << 3);
        let colour = i32::from(base[b + 3]);
        let col = colour & 0x1f;
        let pmask = primasks[priority];

        if colour & 0x80 != 0 {
            x += i32::from(c2.output[4]); // fix the offset of some games
            y += i32::from(c2.output[5]); // like Marvel vs. Capcom ending credits
        }

        if colour & 0xff00 != 0 {
            // handle blocked sprites
            let nx = ((colour & 0x0f00) >> 8) + 1;
            let ny = ((colour & 0xf000) >> 12) + 1;
            let pos = (x, y, xoffs, yoffs);
            let n = (nx, ny);
            if colour & 0x40 != 0 {
                // Y flip
                if colour & 0x20 != 0 {
                    draw_block(
                        gfx,
                        bitmap,
                        pri,
                        pos,
                        n,
                        col,
                        (true, true),
                        pmask,
                        flip,
                        rows,
                        |nxs, nys| code + (nx - 1) - nxs + 0x10 * (ny - 1 - nys),
                    );
                } else {
                    draw_block(
                        gfx,
                        bitmap,
                        pri,
                        pos,
                        n,
                        col,
                        (false, true),
                        pmask,
                        flip,
                        rows,
                        |nxs, nys| code + nxs + 0x10 * (ny - 1 - nys),
                    );
                }
            } else if colour & 0x20 != 0 {
                draw_block(
                    gfx,
                    bitmap,
                    pri,
                    pos,
                    n,
                    col,
                    (true, false),
                    pmask,
                    flip,
                    rows,
                    |nxs, nys| code + (nx - 1) - nxs + 0x10 * nys,
                );
            } else {
                draw_block(
                    gfx,
                    bitmap,
                    pri,
                    pos,
                    n,
                    col,
                    (false, false),
                    pmask,
                    flip,
                    rows,
                    |nxs, nys| {
                        (code & !0xf) + ((code + nxs) & 0xf) + 0x10 * nys // pgear fix
                    },
                );
            }
        } else {
            // Simple case... 1 sprite
            draw_sprite2(
                gfx,
                bitmap,
                pri,
                code,
                col & 0x1f,
                colour & 0x20 != 0,
                colour & 0x40 != 0,
                (x + xoffs) & 0x3ff,
                (y + yoffs) & 0x3ff,
                pmask,
                flip,
                rows,
            );
        }
        i -= 4;
    }
}

#[cfg(test)]
#[path = "cps2_sprites_tests.rs"]
mod tests;
