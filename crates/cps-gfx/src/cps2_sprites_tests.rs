//! Tests du rendu des sprites CPS2 (convention : tuile n = octets n × 128 .. n × 128 + 128).

use crate::regs::Video;
use crate::sprites::{BITMAP_H, BITMAP_W};

struct Scene {
    v: Video,
    gfx: Vec<u8>,
    bitmap: Vec<u16>,
    pri: Vec<u8>,
    primasks: [u32; 8],
}

impl Scene {
    /// `tiles` tuiles transparentes ; sprite unique (x, y, code, colour).
    fn new(tiles: usize, obj: [u16; 4]) -> Self {
        let mut v = Video::new_cps2("ssf2ud");
        let c = v.cps2.as_mut().unwrap();
        c.output[4] = 0x40;
        c.output[5] = 0x10;
        c.buffered_obj[..4].copy_from_slice(&obj);
        c.last_sprite = 0;
        Scene {
            v,
            gfx: vec![0xFF; tiles * 128],
            bitmap: vec![0xBFF; BITMAP_W * BITMAP_H],
            pri: vec![0; BITMAP_W * BITMAP_H],
            primasks: [0; 8],
        }
    }
    fn opaque(&mut self, tile: usize) {
        self.gfx[tile * 128..tile * 128 + 128].fill(0);
    }
    fn render(&mut self, flip: bool) {
        self.v.cps2_render_sprites(
            &self.gfx,
            &mut self.bitmap,
            &mut self.pri,
            &self.primasks,
            flip,
            crate::sprites::FULL_ROWS,
        );
    }
    fn at(&self, x: usize, y: usize) -> u16 {
        self.bitmap[y * BITMAP_W + x]
    }
}

#[test]
fn simple_sprite() {
    let mut s = Scene::new(2, [100, 50, 1, 3]);
    s.opaque(1);
    s.render(false);
    assert_eq!(s.at(100, 50), 0x30);
    assert_eq!(s.at(116, 50), 0xBFF);
    assert_eq!(s.pri[50 * BITMAP_W + 100], 0x1F);
}

#[test]
fn priority_mask() {
    let mut s = Scene::new(2, [100 | 2 << 13, 50, 1, 3]);
    s.opaque(1);
    s.primasks[2] = 0x02;
    s.pri.fill(1);
    s.render(false);
    assert_eq!(s.at(100, 50), 0xBFF);
}

#[test]
fn offset_fix_bit() {
    let mut s = Scene::new(2, [100, 50, 1, 0x83]);
    s.opaque(1);
    s.v.cps2.as_mut().unwrap().output[4] = 0x50;
    s.render(false);
    assert_eq!(s.at(164, 66), 0x30);
    assert_eq!(s.at(163, 66), 0xBFF);
}

#[test]
fn block_without_flip_wraps_column() {
    let mut s = Scene::new(0x11, [100, 50, 0x0F, 0x0103]);
    s.opaque(0x0F);
    s.opaque(0x10);
    s.render(false);
    assert_eq!(s.at(100, 50), 0x30);
    assert_eq!(s.at(116, 50), 0xBFF, "2e tuile = 0x00, pas 0x10");
}

#[test]
fn block_flips() {
    // bloc 2 × 2 de code 0x20 : tuile 0x20 opaque, les autres transparentes
    for (colour, x, y) in [(0x1123, 116, 50), (0x1143, 100, 66), (0x1163, 116, 66)] {
        let mut s = Scene::new(0x40, [100, 50, 0x20, colour]);
        s.opaque(0x20);
        s.render(false);
        assert_eq!(s.at(x, y), 0x30, "colour {colour:#x}");
        let others = [(100, 50), (116, 50), (100, 66), (116, 66)];
        let drawn = others.iter().filter(|&&(a, b)| s.at(a, b) != 0xBFF).count();
        assert_eq!(drawn, 1, "colour {colour:#x}");
    }
}

#[test]
fn flipped_screen() {
    let mut s = Scene::new(2, [100, 50, 1, 3]);
    s.opaque(1);
    s.render(true);
    assert_eq!(s.at(396, 190), 0x30);
    assert_eq!(s.at(100, 50), 0xBFF);
}

#[test]
fn code_high_bits_from_y() {
    let mut s = Scene::new(0x10002, [100, 50 | 0x2000, 1, 3]);
    s.opaque(0x10001);
    s.render(false);
    assert_eq!(s.at(100, 50), 0x30);
}

#[test]
fn no_cps2_state_draws_nothing() {
    let v = Video::new("sf2");
    let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
    let mut pri = vec![0u8; BITMAP_W * BITMAP_H];
    v.cps2_render_sprites(
        &[0; 256],
        &mut bitmap,
        &mut pri,
        &[0; 8],
        false,
        crate::sprites::FULL_ROWS,
    );
    assert!(bitmap.iter().all(|&p| p == 0xBFF));
}
