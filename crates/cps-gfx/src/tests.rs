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
