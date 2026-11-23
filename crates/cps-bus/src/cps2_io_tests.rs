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

#[test]
fn output_write_combine() {
    let mut b = bus(false, false);
    assert_eq!(b.write16(0x400008, true, true, 0x0050), None);
    assert_eq!(b.read16(0x400008, true, true), 0x50);
    b.write16(0x400008, true, false, 0x1200);
    assert_eq!(b.read16(0x400008, true, true), 0x1250);
}

#[test]
fn dead_output_mirror_vs_ram() {
    let mut d = bus(true, false);
    d.write16(0xFFFFF8, true, true, 0x40);
    assert_eq!(d.video.cps2.as_ref().unwrap().output[4], 0x40);
    assert_eq!(&d.ram[0xFFF8..0xFFFA], &[0, 0]);
    d.write16(0xFFFFFC, true, true, 0x5566);
    assert_eq!(&d.ram[0xFFFC..0xFFFE], &[0x55, 0x66]);
    let mut b = bus(false, false);
    b.write16(0xFFFFF8, true, true, 0x40);
    assert_eq!(&b.ram[0xFFF8..0xFFFA], &[0, 0x40]);
    assert_eq!(b.video.cps2.as_ref().unwrap().output[4], 0);
}

#[test]
fn objects_and_bank() {
    let mut b = bus(false, false);
    b.write16(0x700002, true, true, 0x2222);
    assert_eq!(b.video.cps2.as_ref().unwrap().objram1[1], 0x2222);
    b.write16(0x8040E0, true, true, 1);
    b.write16(0x708000, true, true, 0x1111);
    assert_eq!(b.video.cps2.as_ref().unwrap().objram1[0], 0x1111);
}

#[test]
fn cps_a_b_gfxram_and_ram() {
    let mut b = bus(false, false);
    assert_eq!(b.write16(0x804100 + 2 * 5, true, true, 0x1234), None);
    assert_eq!(b.video.regs_a[5], 0x1234);
    b.write16(0x800100 + 2 * 6, true, true, 0x5678);
    assert_eq!(b.video.regs_a[6], 0x5678);
    b.write16(0x804150, true, true, 0x00AA);
    assert_eq!(b.video.regs_b[8], 0x00AA);
    b.write16(0x900010, true, true, 0xBEEF);
    assert_eq!(b.read16(0x900010, true, true), 0xBEEF);
    b.write16(0x660002, true, true, 0xCAFE);
    assert_eq!(b.read16(0x660002, true, true), 0xCAFE);
    b.write16(0x664000, false, true, 0x0077);
    assert_eq!(b.read16(0x664000, true, true), 0x0077);
    b.write16(0xFF0100, true, true, 0x4321);
    assert_eq!(b.read16(0xFF0100, true, true), 0x4321);
    assert_eq!(b.write16(0x000010, true, true, 0xFFFF), None);
    assert_eq!(b.rom[0x10], 0, "ROM non modifiable");
}
