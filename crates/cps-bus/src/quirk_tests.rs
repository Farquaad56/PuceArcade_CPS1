//! Tests des particularités de carte des bootlegs (Bus68k::hack_dsw, in2_high, cpsb_mirror).

use crate::video_port::new_cps_bus;
use cps_common::input::Dips;
use cps_common::input::InputState;

#[test]
fn hack_dsw_on_both_bytes() {
    let mut b = new_cps_bus(vec![0; 0x100], "sf2", Dips::default());
    b.inputs.in0 = 0xEF;
    assert_eq!(b.read16(0x800018, true, true), 0xEFFF);
    b.hack_dsw = true;
    assert_eq!(b.read16(0x800018, true, true), 0xEFEF);
}

#[test]
fn in2_on_high_byte() {
    let mut b = new_cps_bus(vec![0; 0x100], "sf2", Dips::default());
    let i = InputState {
        in2: 0xFE,
        ..InputState::default()
    };
    b.set_inputs(i);
    assert_eq!(b.video.in2, 0x00FE);
    b.in2_high = true;
    b.set_inputs(i);
    assert_eq!(b.video.in2, 0xFEFF);
}

#[test]
fn cpsb_mirror() {
    let mut b = new_cps_bus(vec![0; 0x100], "sf2", Dips::default());
    b.cpsb_mirror = Some(0x8001C0);
    b.write16(0x8001C0 + 0x0E, true, true, 0x1234);
    assert_eq!(b.video.regs_b[7], 0x1234);
    b.write16(0x800140 + 0x10, true, true, 0x5678);
    assert_eq!(b.video.regs_b[8], 0x5678);
}

#[test]
fn dinohunt_io() {
    let mut b = new_cps_bus(vec![0; 0x100], "dinohunt", Dips::default());
    b.inputs.in2 = 0xFB;
    assert_eq!(b.read16(0xF18000, true, true), 0);
    b.dinohunt_io = true;
    assert_eq!(b.read16(0xF18000, true, true), 0x00FF);
    assert_eq!(b.read16(0xF19FFE, true, true), 0x00FF);
    assert_eq!(b.read16(0xFC0000, true, true), 0x00FB);
}
