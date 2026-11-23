// license:BSD-3-Clause
// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald
//! Carte mémoire 68000 du CPS2 : lectures (cps2_map, dead_cps2_map) et état des E/S propres.
//! spec: docs/spec/05 §Carte mémoire 68000, §Ports d'entrée
//! source: ref/mame-cps2/cps2.cpp l.1212-1234 (cps2_qsound_volume_r), l.1324-1347 (cps2_map),
//!         l.1363-1388 (dead_cps2_map)

use crate::map68k::{Bus68k, VideoPort};

/// Entrées IN0, IN1, IN2 (actives bas ; IN2 bit 0 = DO de l'EEPROM, fourni par l'hôte).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cps2Inputs {
    pub in0: u16,
    pub in1: u16,
    pub in2: u16,
}

impl Default for Cps2Inputs {
    fn default() -> Self {
        Cps2Inputs {
            in0: 0xFFFF,
            in1: 0xFFFF,
            in2: 0xFFFF,
        }
    }
}

/// État des E/S CPS2 tenu par le bus.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cps2Io {
    pub inputs: Cps2Inputs,
    /// 660000-663FFF : « extra adapter memory » (16 Kio, octets gros-boutistes).
    pub extra_ram: Vec<u8>,
    /// 664000-664001 : un mot de RAM.
    pub ram664: u16,
    /// dead_cps2_map : registres output aussi en FFFFF0-FFFFFB.
    pub dead: bool,
    /// init_singbrd : volume 0xD000.
    pub singbrd: bool,
}

impl Cps2Io {
    pub fn new(dead: bool, singbrd: bool) -> Self {
        Cps2Io {
            inputs: Cps2Inputs::default(),
            extra_ram: vec![0; 0x4000],
            ram664: 0,
            dead,
            singbrd,
        }
    }

    /// cps2_qsound_volume_r (l.1212-1234) : volume numérique maximal, pas de câble réseau.
    pub fn volume(&self) -> u16 {
        if self.singbrd {
            0xD000
        } else {
            0xE021
        }
    }
}

impl<V: VideoPort> Bus68k<V> {
    /// Lecture mot du 68000 CPS2 (`a` = adresse sur 24 bits). 618000-619FFF est servie par l'hôte.
    pub fn cps2_read16(&mut self, a: usize) -> u16 {
        let Some(io) = self.cps2.as_deref() else {
            return 0;
        };
        let dead = io.dead;
        match a {
            0x000000..=0x3FFFFF => self.read_rom_word(a),
            0x400000..=0x40000B => self.video.cps2_output_read((a - 0x400000) / 2),
            0xFFFFF0..=0xFFFFFB if dead => self.video.cps2_output_read((a - 0xFFFFF0) / 2),
            0x660000..=0x663FFF => {
                let o = (a - 0x660000) & !1;
                u16::from_be_bytes([io.extra_ram[o], io.extra_ram[o + 1]])
            }
            0x664000..=0x664001 => io.ram664,
            0x700000..=0x701FFF => 0,
            0x708000..=0x70FFFF => self.video.cps2_objram2_read((a - 0x708000) / 2 % 0x1000),
            0x800140..=0x80017F => self.video.cps_b_read((a - 0x800140) / 2),
            0x804140..=0x80417F => self.video.cps_b_read((a - 0x804140) / 2),
            0x804000..=0x804001 => io.inputs.in0,
            0x804010..=0x804011 => io.inputs.in1,
            0x804020..=0x804021 => io.inputs.in2,
            0x804030..=0x804031 => io.volume(),
            0x8040B0..=0x8040B3 => 0xFFFF,
            0x900000..=0x92FFFF => self.video.gfxram_read((a - 0x900000) / 2),
            0xFF0000..=0xFFFFFF => self.read_ram_word((a - 0xFF0000) & !1),
            _ => 0,
        }
    }
}

#[cfg(test)]
#[path = "cps2_io_tests.rs"]
mod tests;
