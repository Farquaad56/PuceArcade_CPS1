//! Tests des lectures du 68000 CPS2.

use super::*;
use crate::video_port::{new_cps_bus, CpsBus};
use cps_common::input::Dips;
use cps_gfx::regs::Video;

fn bus(dead: bool, singbrd: bool) -> CpsBus {
    let mut b = new_cps_bus(vec![0; 0x100], "ssf2ud", Dips::default());
    b.video = Video::new_cps2("ssf2ud");
    b.cps2 = Some(Box::new(Cps2Io::new(dead, singbrd)));
    b
}

#[test]
fn defaults() {
    assert_eq!(Cps2Inputs::default().in2, 0xFFFF);
    let io = Cps2Io::new(true, false);
    assert_eq!(
        (io.extra_ram.len(), io.dead, io.singbrd),
        (0x4000, true, false)
    );
}

#[test]
fn ports() {
    let mut b = bus(false, false);
    assert_eq!(b.read16(0x804030, true, true), 0xE021);
    assert_eq!(b.read16(0x8040B0, true, true), 0xFFFF);
    b.cps2.as_mut().unwrap().inputs.in0 = 0x1234;
    assert_eq!(b.read16(0x804000, true, true), 0x1234);
    assert_eq!(bus(false, true).read16(0x804030, true, true), 0xD000);
}

#[test]
fn objram2_and_mirror() {
    let mut b = bus(false, false);
    b.video.cps2.as_mut().unwrap().objram2[0] = 0xBEEF;
    assert_eq!(b.read16(0x708000, true, true), 0xBEEF);
    assert_eq!(b.read16(0x70E000, true, true), 0xBEEF);
    b.video.cps2.as_mut().unwrap().objram1[0] = 0x1111;
    assert_eq!(b.read16(0x700000, true, true), 0);
}

#[test]
fn output_registers() {
    let mut b = bus(false, false);
    b.video.cps2.as_mut().unwrap().output[4] = 0x40;
    assert_eq!(b.read16(0x400008, true, true), 0x40);
    assert_eq!(b.read16(0xFFFFF8, true, true), 0, "RAM sur cps2_map");
    let mut d = bus(true, false);
    d.video.cps2.as_mut().unwrap().output[4] = 0x40;
    assert_eq!(d.read16(0xFFFFF8, true, true), 0x40);
}

#[test]
fn extra_ram_and_unmapped() {
    let mut b = bus(false, false);
    b.cps2.as_mut().unwrap().extra_ram[..2].copy_from_slice(&[0x12, 0x34]);
    assert_eq!(b.read16(0x660000, true, true), 0x1234);
    assert_eq!(b.read16(0xA00000, true, true), 0);
    b.ram[0x10] = 0xAB;
    assert_eq!(b.read16(0xFF0010, true, true) >> 8, 0xAB);
}
