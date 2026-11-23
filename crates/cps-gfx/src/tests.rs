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
