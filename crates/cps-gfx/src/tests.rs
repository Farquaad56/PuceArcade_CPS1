//! Tests pour cps-gfx : configurations CPS-B et mappers (spec: docs/spec/02 §Config, §ROM graphique).

use crate::config::find_config;
use crate::mapper::{
    gfxrom_bank_mapper, GFXTYPE_SCROLL1, GFXTYPE_SCROLL2, GFXTYPE_SCROLL3, GFXTYPE_SPRITES,
};

#[test]
fn games_count() {
    assert_eq!(crate::config_data::GAMES.len(), 261);
}

#[test]
fn find_config_ghouls() {
    let cfg = find_config("ghouls");
    assert_eq!(cfg.cpsb.cpsb_value, 0);
    assert_eq!(cfg.cpsb.cpsb_addr, -1);
}

#[test]
fn find_config_sf2() {
    let cfg = find_config("sf2");
    assert_eq!(cfg.cpsb.cpsb_addr, 0x32);
    assert_eq!(cfg.cpsb.cpsb_value, 0x0401);
    assert_eq!(cfg.in2_addr, 0x36);
}

#[test]
fn find_config_sf2ce() {
    let cfg = find_config("sf2ce");
    assert_eq!(cfg.cpsb.mult_result_lo, 0x04);
}

#[test]
fn find_config_unknown_fallback_to_cps2() {
    let cfg = find_config("inconnu");
    assert_eq!(cfg.name, "cps2");
}

// Tests mapper pour ghouls (DM620)
#[test]
fn mapper_ghouls_sprites_1000() {
    let cfg = find_config("ghouls");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SPRITES, 0x1000),
        0x5000
    );
}

#[test]
fn mapper_ghouls_scroll3_1000() {
    let cfg = find_config("ghouls");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL3, 0x1000),
        0x1000
    );
}

#[test]
fn mapper_ghouls_scroll1_123() {
    let cfg = find_config("ghouls");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL1, 0x123),
        0x123
    );
}

#[test]
fn mapper_ghouls_sprites_800() {
    let cfg = find_config("ghouls");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SPRITES, 0x800),
        0x800
    );
}

#[test]
fn mapper_ghouls_sprites_1() {
    let cfg = find_config("ghouls");
    assert_eq!(gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SPRITES, 1), 1);
}

// Tests mapper pour sf2 (STF29)
#[test]
fn mapper_sf2_scroll2_2900() {
    let cfg = find_config("sf2");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL2, 0x2900),
        0xA900
    );
}

#[test]
fn mapper_sf2_scroll1_4000() {
    let cfg = find_config("sf2");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL1, 0x4000),
        0x14000
    );
}

#[test]
fn mapper_sf2_scroll1_5000() {
    let cfg = find_config("sf2");
    assert_eq!(gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL1, 0x5000), -1);
}

#[test]
fn mapper_sf2_sprites_9000() {
    let cfg = find_config("sf2");
    assert_eq!(gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SPRITES, 0x9000), -1);
}

#[test]
fn mapper_sf2_sprites_4800() {
    let cfg = find_config("sf2");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SPRITES, 0x4800),
        0x4800
    );
}

#[test]
fn mapper_sf2_scroll1_100() {
    let cfg = find_config("sf2");
    assert_eq!(gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL1, 0x100), -1);
}

#[test]
fn mapper_sf2_scroll3_400() {
    let cfg = find_config("sf2");
    assert_eq!(
        gfxrom_bank_mapper(&cfg.mapper, GFXTYPE_SCROLL3, 0x400),
        0x2400
    );
}

#[test]
fn new_cps2_uses_cps2_config() {
    let v = crate::regs::Video::new_cps2("ssf2ud");
    let c = crate::config::find_config("cps2");
    assert!(std::ptr::eq(v.cfg, c));
    assert_eq!(v.cfg.cpsb.layer_control, c.cpsb.layer_control);
    assert!(v.cps2.is_some());
    assert!(crate::regs::Video::new("sf2").cps2.is_none());
}

#[test]
fn cps2_vblank_latches_objram_only() {
    let mut v = crate::regs::Video::new_cps2("ssf2ud");
    for (i, w) in v.gfxram.iter_mut().enumerate() {
        *w = i as u16 | 1;
    }
    {
        let c = v.cps2.as_mut().unwrap();
        c.objram1[7] = 0x1234;
        c.objram2[7] = 0x5678;
    }
    v.vblank();
    let c = v.cps2.as_ref().unwrap();
    assert_eq!(c.buffered_obj[7], 0x1234);
    assert_eq!(c.buffered_obj, c.objram1);
    assert!(v.buffered_obj.iter().all(|&w| w == 0));
}

#[test]
fn cps2_render_frame_draws_sprite() {
    let mut v = crate::regs::Video::new_cps2("ssf2ud");
    {
        let c = v.cps2.as_mut().unwrap();
        c.output[4] = 0x40;
        c.output[5] = 0x10;
        // priorité 1 : primasks[1] = 0 avec pri_ctrl = 0 (primasks[0] = 0xFF masque tout)
        c.objram1[..8].copy_from_slice(&[100 | 1 << 13, 50, 1, 3, 0, 0x8000, 0, 0]);
    }
    v.vblank();
    v.palette[0x30] = 0x00FF_0000;
    let mut gfx = vec![0xFFu8; 0x10000];
    gfx[128..256].fill(0);
    let mut fb = cps_common::video::Framebuffer::new();
    v.render_frame(&gfx, None, 0, &mut fb);
    assert_eq!(v.cps2.as_ref().unwrap().last_sprite, 0);
    let (x, y) = (100 - crate::sprites::CLIP.0, 50 - crate::sprites::CLIP.2);
    assert_eq!(fb.pixels[y * cps_common::video::SCREEN_W + x], 0x00FF_0000);
}

#[test]
fn cps2_raster_registers() {
    let mut v = crate::regs::Video::new_cps2("ssf2ud");
    v.cps_b_write(0x10 / 2, 0x8010, true, true);
    let r = v.raster.as_ref().unwrap();
    assert_eq!((r.reload[0], r.counter[0]), (0x010, 0x010));
    assert_eq!(v.cps_b_read(0x10 / 2), 0x010);

    v.raster.as_mut().unwrap().counter[2] = 100;
    v.hpos = 40;
    v.regs_b[7] = 1;
    assert_eq!(v.cps_b_read(0x0e / 2), 0xA1);
}

#[test]
fn cps1_has_no_raster() {
    let v = crate::regs::Video::new("ghouls");
    assert!(v.raster.is_none());
    assert_eq!(v.cps_b_read(0x10 / 2), 0xFFFF);
}

/// Générateur pseudo-aléatoire déterministe (xorshift) pour les scènes de test.
fn xorshift(seed: &mut u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed
}

/// Scène aléatoire : GFXRAM, ROM graphique, couches actives, scrolls, sprites CPS1 (et CPS2).
fn random_scene(cps2: bool) -> (crate::regs::Video, Vec<u8>) {
    let mut seed = 0x1234_5678;
    let mut v = if cps2 {
        crate::regs::Video::new_cps2("ssf2ud")
    } else {
        crate::regs::Video::new("ghouls")
    };
    for w in v.gfxram.iter_mut() {
        *w = xorshift(&mut seed) as u16;
    }
    let gfx: Vec<u8> = (0..0x40000).map(|_| xorshift(&mut seed) as u8).collect();
    for l in 0..3 {
        v.bases.layer_enabled[l] = true;
        v.bases.scrollx[l] = (xorshift(&mut seed) & 0x3FF) as i32;
        v.bases.scrolly[l] = (xorshift(&mut seed) & 0x3FF) as i32;
    }
    let lc = (v.cfg.cpsb.layer_control / 2) as usize;
    v.regs_b[lc] = 0x12C0; // ordre 3, 2, 0 (sprites), 1
    for w in v.buffered_obj.iter_mut() {
        *w = xorshift(&mut seed) as u16 & 0x33FF; // blocs d'au plus 4 × 4 tuiles
    }
    v.last_sprite = 0x7C;
    if let Some(c2) = v.cps2.as_mut() {
        for w in c2.buffered_obj.iter_mut() {
            *w = xorshift(&mut seed) as u16 & 0x33FF;
        }
        c2.buffered_obj[0x81] = 0x8000; // fin de table après 32 sprites
        c2.output[4] = 0x40;
        c2.output[5] = 0x10;
        c2.pri_ctrl = 0x0531;
        c2.find_last_sprite();
    }
    (v, gfx)
}

/// Rend les bandes données, dans l'ordre, dans un même bitmap.
fn render_bands(cps2: bool, bands: &[(usize, usize)]) -> (Vec<u16>, Vec<u8>) {
    use crate::sprites::{BITMAP_H, BITMAP_W};
    let (mut v, gfx) = random_scene(cps2);
    let masks = v.update_transmasks();
    let mut bitmap = vec![0xBFFu16; BITMAP_W * BITMAP_H];
    let mut pri = vec![0u8; BITMAP_W * BITMAP_H];
    for &rows in bands {
        v.compose(&gfx, None, 0, &mut bitmap, &mut pri, masks, false, rows);
    }
    (bitmap, pri)
}

#[test]
fn bands_equal_full_frame_cps1() {
    let full = render_bands(false, &[(16, 240)]);
    assert!(
        full.0.iter().filter(|&&p| p != 0xBFF).count() > 10_000,
        "scène non vide"
    );
    assert!(full == render_bands(false, &[(16, 100), (100, 240)]));
}

#[test]
fn bands_equal_full_frame_cps2() {
    let full = render_bands(true, &[(16, 240)]);
    assert!(
        full.0.iter().filter(|&&p| p != 0xBFF).count() > 10_000,
        "scène non vide"
    );
    assert!(full == render_bands(true, &[(16, 100), (100, 240)]));
}

#[test]
fn band_outside_rows_is_untouched() {
    use crate::sprites::BITMAP_W;
    let (b, _) = render_bands(false, &[(100, 101)]);
    assert!(b[..100 * BITMAP_W].iter().all(|&p| p == 0xBFF));
    assert!(b[101 * BITMAP_W..].iter().all(|&p| p == 0xBFF));
    assert!(b[100 * BITMAP_W..101 * BITMAP_W]
        .iter()
        .any(|&p| p != 0xBFF));
}

/// Trame de la scène aléatoire CPS1 avec le scroll X du scroll1 donné ; `split` : changement de
/// scroll après `render_lines(…, 120)`.
fn scroll_frame(first: u16, then: Option<u16>) -> Vec<u32> {
    use crate::regs::{SCROLL1_X, VIDEOCONTROL};
    let (mut v, gfx) = random_scene(false);
    for (i, p) in v.palette.iter_mut().enumerate() {
        *p = i as u32;
    }
    let lc = (v.cfg.cpsb.layer_control / 2) as usize;
    v.regs_b[lc] = 0x0040 | v.cfg.cpsb.layer_enable_mask[0] as u16; // scroll1 seul
    v.regs_a[VIDEOCONTROL] = 0;
    v.regs_a[SCROLL1_X] = first;
    let mut fb = cps_common::video::Framebuffer::new();
    match then {
        None => v.render_frame(&gfx, None, 0, &mut fb),
        Some(s) => {
            v.begin_frame();
            v.render_lines(&gfx, None, 0, 120);
            v.regs_a[SCROLL1_X] = s;
            v.finish_frame(&gfx, None, 0, &mut fb);
        }
    }
    fb.pixels
}

#[test]
fn partial_render_follows_scroll_change() {
    use cps_common::video::SCREEN_W;
    let a = scroll_frame(0x10, None);
    let b = scroll_frame(0x37, None);
    let c = scroll_frame(0x10, Some(0x37));
    let split = (120 - crate::sprites::CLIP.2) * SCREEN_W;
    assert_ne!(a[..split], b[..split]);
    assert_ne!(a[split..], b[split..]);
    assert_eq!(c[..split], a[..split], "lignes < 120 : ancien scroll");
    assert_eq!(c[split..], b[split..], "lignes >= 120 : nouveau scroll");
}

#[test]
fn render_frame_equals_partial_in_one_call() {
    assert_eq!(scroll_frame(0x22, None), scroll_frame(0x22, Some(0x22)));
}
