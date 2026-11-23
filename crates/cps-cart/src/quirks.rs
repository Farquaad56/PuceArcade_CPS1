// license:BSD-3-Clause
// copyright-holders:Paul Leaman
//! Particularités de carte des bootlegs CPS1 réglées par la fonction d'init ou la disposition
//! d'entrées MAME. spec: docs/spec/04 §Bootlegs
//! source: ref/mame-cps1/cps1.cpp l.274-289 (cps1_hack_dsw_r), l.2138-2151 (INPUT_PORTS sf2hack),
//!         l.14603-14636 (init_sf2thndr, init_sf2hack, init_sf2rk)

use crate::romset::GameInfo;

/// Différences de carte d'un bootleg par rapport à la carte CPS1 d'origine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Quirks {
    /// cps1_hack_dsw_r (init_sf2hack) : IN0 et DIP de 800018-80001F sur les deux octets.
    pub hack_dsw: bool,
    /// INPUT_PORTS sf2hack : boutons 4-6 (IN2) sur l'octet fort au lieu de l'octet faible.
    pub in2_high: bool,
    /// init_sf2thndr : registres CPS-B recopiés en 8001C0-8001FF (en plus de 800140).
    pub cpsb_mirror_1c0: bool,
}

/// Dispositions d'entrées dérivées de sf2hack (PORT_INCLUDE( sf2hack )).
const SF2HACK_INPUTS: [&str; 5] = ["sf2hack", "sf2amf", "sf2amfx", "sf2m2", "sf2m4"];

pub fn quirks_of(info: &GameInfo) -> Quirks {
    Quirks {
        hack_dsw: matches!(info.init, "init_sf2hack" | "init_sf2rk"),
        in2_high: SF2HACK_INPUTS.contains(&info.input),
        cpsb_mirror_1c0: info.init == "init_sf2thndr",
    }
}

/// init_sf2rk (l.14616-14636) : graphismes à bits 0 et 7 échangés, sauf 400000-47FFFF (couche
/// supplémentaire, non brouillée).
pub fn descramble_gfx(init: &str, gfx: &mut [u8]) {
    if init != "init_sf2rk" {
        return;
    }
    let len = gfx.len();
    for i in (0..0x400000.min(len)).chain(0x480000.min(len)..0x600000.min(len)) {
        let x = gfx[i];
        // bitswap(x, 0, 6, 5, 4, 3, 2, 1, 7)
        gfx[i] = (x & 0x7E) | (x >> 7) | (x << 7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find_game;

    #[test]
    fn quirks_by_game() {
        let q = |n: &str| quirks_of(find_game(n).unwrap());
        assert_eq!(q("sf2"), Quirks::default());
        let ebbl = q("sf2ebbl");
        assert!(ebbl.hack_dsw && ebbl.in2_high && !ebbl.cpsb_mirror_1c0);
        assert!(q("sf2m2").in2_high);
        assert!(q("sf2rk").hack_dsw);
        let thndr = q("sf2thndr");
        assert!(thndr.cpsb_mirror_1c0 && !thndr.hack_dsw);
    }

    #[test]
    fn sf2rk_bitswap() {
        let mut g = vec![0x81u8, 0x01, 0x80, 0x7E];
        descramble_gfx("init_sf2rk", &mut g);
        assert_eq!(g, [0x81, 0x80, 0x01, 0x7E]);
        let mut big = vec![0x01u8; 0x600000];
        descramble_gfx("init_sf2rk", &mut big);
        assert_eq!(
            (big[0], big[0x400000], big[0x47FFFF], big[0x480000]),
            (0x80, 0x01, 0x01, 0x80)
        );
        let mut other = vec![0x01u8; 4];
        descramble_gfx("empty_init", &mut other);
        assert_eq!(other, [1; 4]);
    }
}
