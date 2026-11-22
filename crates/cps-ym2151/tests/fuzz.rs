//! Oracle YM2151 aléatoire : écritures pseudo-aléatoires dans tous les registres (hors timers),
//! comparées échantillon par échantillon à ymfm (tools/ymfm_opm_fuzz.cpp, tests-data/ym2151/fuzz<g>_*.bin).
//! Sorties comparées AVANT écrêtage (valeurs i32 de `generate`).

use cps_ym2151::common::{Output, YmfmInterface};
use cps_ym2151::ym2151::Ym2151;
use std::path::PathBuf;

struct NullIntf;
impl YmfmInterface for NullIntf {
    fn sync_mode_write(&mut self, _data: u8) {}
    fn sync_check_interrupts(&mut self) {}
}

fn data(name: &str) -> Vec<u8> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests-data/ym2151")
        .join(name);
    std::fs::read(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn run(seed: u32) {
    let w = data(&format!("fuzz{seed}_w.bin"));
    let o = data(&format!("fuzz{seed}_o.bin"));
    let mut intf = NullIntf;
    let mut chip = Ym2151::new();
    chip.reset(&mut intf);
    let mut wi = 0;
    for n in 1..=o.len() / 8 {
        while wi + 6 <= w.len()
            && u32::from_le_bytes([w[wi], w[wi + 1], w[wi + 2], w[wi + 3]]) as usize == n
        {
            chip.write(0, w[wi + 4], &mut intf);
            chip.write(1, w[wi + 5], &mut intf);
            wi += 6;
        }
        let mut out = Output::<2> { data: [0, 0] };
        chip.generate(&mut out, 1);
        let b = (n - 1) * 8;
        let l = i32::from_le_bytes([o[b], o[b + 1], o[b + 2], o[b + 3]]);
        let r = i32::from_le_bytes([o[b + 4], o[b + 5], o[b + 6], o[b + 7]]);
        assert_eq!(out.data, [l, r], "fuzz{seed} : échantillon {n}");
    }
    assert_eq!(wi, w.len(), "fuzz{seed} : écritures non consommées");
}

#[test]
fn fuzz1() {
    run(1);
}
#[test]
fn fuzz2() {
    run(2);
}
#[test]
fn fuzz3() {
    run(3);
}
#[test]
fn fuzz4() {
    run(4);
}
