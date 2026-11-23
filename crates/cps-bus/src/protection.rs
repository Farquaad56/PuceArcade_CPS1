// license:BSD-3-Clause
// copyright-holders:Paul Leaman
//! Protections de bootlegs CPS1 : lectures à valeurs fixes ou latch d'une valeur écrite.
//! spec: docs/spec/04 §Bootlegs
//! source: ref/mame-cps1/cps1.cpp l.14556-14592 (sf2rb, sf2rb2), l.14639-14675 (sf2dongb, sf2ceblp),
//!         l.14829-14838 (init_pang3b4)

use crate::map68k::{Bus68k, VideoPort};

/// Protection installée par la fonction d'init MAME.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Protection {
    #[default]
    None,
    /// init_sf2rb : 200000-2FFFFF, 201200 = 0x0002, 281200 = 0x0040, sinon 0.
    Sf2rb,
    /// init_sf2rb2 : 200000-2FFFFF, 201200 = 0x0000, 281200 = 0x0040, sinon 0.
    Sf2rb2,
    /// init_sf2dongb : 180000-1FFFFF, 180000 et 1F7040 = 0x0210, sinon 0.
    Sf2dongb,
    /// init_sf2ceblp : latch écrit en 5762B0, lu en 57A2B0.
    Sf2ceblp,
    /// init_pang3b4 : comme sf2ceblp, et 80017A (EEPROM absente) sans effet, lu 0.
    Pang3b4,
}

impl Protection {
    pub fn from_init(init: &str) -> Self {
        match init {
            "init_sf2rb" => Protection::Sf2rb,
            "init_sf2rb2" => Protection::Sf2rb2,
            "init_sf2dongb" => Protection::Sf2dongb,
            "init_sf2ceblp" => Protection::Sf2ceblp,
            "init_pang3b4" => Protection::Pang3b4,
            _ => Protection::None,
        }
    }
}

impl<V: VideoPort> Bus68k<V> {
    /// Lecture servie par la protection (avant la ROM et le reste de la carte mémoire).
    pub(crate) fn protection_read(&self, a: usize) -> Option<u16> {
        match (self.protection, a) {
            (Protection::Sf2rb | Protection::Sf2rb2, 0x200000..=0x2FFFFF) => {
                Some(match (self.protection, a & !1) {
                    (Protection::Sf2rb, 0x201200) => 0x0002,
                    (_, 0x281200) => 0x0040,
                    _ => 0,
                })
            }
            (Protection::Sf2dongb, 0x180000..=0x1FFFFF) => {
                Some(if matches!(a & !1, 0x180000 | 0x1F7040) {
                    0x0210
                } else {
                    0
                })
            }
            (Protection::Sf2ceblp | Protection::Pang3b4, 0x57A2B0 | 0x57A2B1) => {
                Some(match self.prot_latch {
                    0x0000 => 0x1992,
                    0x0004 => 0x0408,
                    _ => 0xFFFF,
                })
            }
            (Protection::Pang3b4, 0x80017A | 0x80017B) => Some(0),
            _ => None,
        }
    }

    /// Écriture servie par la protection ; true si l'adresse est prise.
    pub(crate) fn protection_write(&mut self, a: usize, data: u16) -> bool {
        match (self.protection, a) {
            (Protection::Sf2ceblp | Protection::Pang3b4, 0x5762B0 | 0x5762B1) => {
                // PORT-NOTE: m_sf2ceblp_prot n'est pas dans les savestates (le jeu l'écrit avant
                // chaque lecture) ; le format des sauvegardes CPS1 ne change pas.
                self.prot_latch = data;
                true
            }
            (Protection::Pang3b4, 0x80017A | 0x80017B) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::video_port::new_cps_bus;
    use cps_common::input::Dips;

    fn bus(p: Protection) -> crate::CpsBus {
        let mut b = new_cps_bus(vec![0x55; 0x400000], "sf2", Dips::default());
        b.protection = p;
        b
    }

    #[test]
    fn fixed_reads() {
        let mut b = bus(Protection::Sf2rb);
        assert_eq!(b.read16(0x201200, true, true), 0x0002);
        assert_eq!(b.read16(0x281200, true, true), 0x0040);
        assert_eq!(b.read16(0x200000, true, true), 0);
        assert_eq!(b.read16(0x100000, true, true), 0x5555, "ROM ailleurs");
        let mut b = bus(Protection::Sf2rb2);
        assert_eq!(b.read16(0x201200, true, true), 0);
        assert_eq!(b.read16(0x281200, true, true), 0x0040);
        let mut b = bus(Protection::Sf2dongb);
        assert_eq!(b.read16(0x180000, true, true), 0x0210);
        assert_eq!(b.read16(0x1F7040, true, true), 0x0210);
        assert_eq!(b.read16(0x180002, true, true), 0);
        assert_eq!(bus(Protection::None).read16(0x201200, true, true), 0x5555);
    }

    #[test]
    fn latch() {
        for p in [Protection::Sf2ceblp, Protection::Pang3b4] {
            let mut b = bus(p);
            assert_eq!(b.read16(0x57A2B0, true, true), 0x1992);
            b.write16(0x5762B0, true, true, 4);
            assert_eq!(b.read16(0x57A2B0, true, true), 0x0408);
            b.write16(0x5762B0, true, true, 7);
            assert_eq!(b.read16(0x57A2B0, true, true), 0xFFFF);
        }
        let mut b = bus(Protection::Pang3b4);
        assert_eq!(b.read16(0x80017A, true, true), 0);
        assert_eq!(Protection::from_init("init_sf2rb2"), Protection::Sf2rb2);
    }
}
