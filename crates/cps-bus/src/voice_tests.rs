//! Voies du débogueur sur les puces CPS1 : le muet à 0 ne change rien, une voie coupée disparaît
//! du mixage mais reste visible à l'oscilloscope.

use crate::chips::Chips;
use crate::sound::SoundChips;
use cps_common::clock::TICKS_PER_YM_SAMPLE;

fn ym(c: &mut Chips, reg: u8, v: u8) {
    c.ym_write(0, reg);
    c.ym_write(1, v);
}

/// Canal FM 0 : algorithme 7 (4 opérateurs en sortie), gauche + droite, attaque maximale, note tenue ;
/// voix OKI 0 sur l'échantillon 1.
fn playing_chips() -> Chips {
    let mut rom = vec![0u8; 0x40000];
    rom[9] = 0x04;
    rom[12] = 0x04;
    rom[13] = 0x80;
    for (i, b) in rom[0x400..0x480].iter_mut().enumerate() {
        *b = if i % 2 == 0 { 0x77 } else { 0x99 };
    }
    let mut c = Chips::new(rom);
    ym(&mut c, 0x20, 0xC7);
    ym(&mut c, 0x28, 0x4A);
    for op in 0..4 {
        ym(&mut c, 0x60 + op * 8, 0x00); // TL
        ym(&mut c, 0x80 + op * 8, 0x1F); // AR
        ym(&mut c, 0x40 + op * 8, 0x01); // MUL
    }
    ym(&mut c, 0x08, 0x78);
    c.oki_write(0x81);
    c.oki_write(0x10);
    c
}

/// 2000 échantillons YM de plus (`advance` prend une date absolue).
fn run(c: &mut Chips) -> Vec<f32> {
    let t = c.ym.time() * cps_common::clock::TICKS_PER_YM;
    c.advance(t + TICKS_PER_YM_SAMPLE * 2000);
    c.take_samples()
}

#[test]
fn scope_and_zero_mask_leave_sound_unchanged() {
    let mut a = playing_chips();
    let mut b = playing_chips();
    b.set_scope(true);
    b.set_mute(0);
    let sa = run(&mut a);
    assert!(sa.iter().any(|s| *s != 0.0));
    assert_eq!(sa, run(&mut b));
}

#[test]
fn muted_fm_channel_leaves_mix_but_not_scope() {
    let mut a = playing_chips();
    a.oki_write(0x08); // OKI arrêté : ne reste que le canal FM 0
    a.set_scope(true);
    a.set_mute(1);
    let s = run(&mut a);
    assert!(s.iter().all(|x| *x == 0.0), "canal 0 coupé : silence");
    let sc = a.scopes.as_ref().unwrap();
    assert!(sc.0.voice(0).iter().any(|x| *x != 0.0), "toujours visible");
    assert!(sc.0.voice(1).iter().all(|x| *x == 0.0));

    a.set_mute(0);
    assert!(run(&mut a).iter().any(|x| *x != 0.0));
}

#[test]
fn muted_oki_voice() {
    let mut a = playing_chips();
    let mut b = playing_chips();
    b.set_scope(true);
    b.set_mute(1 << 8);
    let (sa, sb) = (run(&mut a), run(&mut b));
    assert_ne!(sa, sb, "voix OKI 0 retirée du mixage");
    let sc = b.scopes.as_ref().unwrap();
    assert!(sc.1.voice(0).iter().any(|x| *x != 0.0));
}
