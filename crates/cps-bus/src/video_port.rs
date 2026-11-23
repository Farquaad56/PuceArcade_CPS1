//! Branchement de la vidéo : impl VideoPort pour cps_gfx::Video.
//! spec: docs/spec/01 §68000

use crate::map68k::{Bus68k, VideoPort};
use cps_common::input::Dips;
use cps_gfx::regs::Video;

impl VideoPort for Video {
    fn cps_a_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        Video::cps_a_write(self, word, data, uds, lds);
    }

    fn cps_b_read(&mut self, word: usize) -> u16 {
        Video::cps_b_read(self, word)
    }

    fn cps_b_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        Video::cps_b_write(self, word, data, uds, lds);
    }

    fn gfxram_read(&self, word: usize) -> u16 {
        Video::gfxram_read(self, word)
    }

    fn gfxram_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        Video::gfxram_write(self, word, data, uds, lds);
    }

    fn set_inputs(&mut self, in2: u16, in3: u16) {
        Video::set_inputs(self, in2, in3);
    }

    // CPS2 : délégation à Video::cps2 (sans état CPS2 : lecture 0, écriture ignorée)
    fn cps2_output_read(&self, word: usize) -> u16 {
        self.cps2.as_ref().map_or(0, |c| c.output_r(word))
    }

    fn cps2_output_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        if let Some(c) = self.cps2.as_mut() {
            c.output_w(word, data, uds, lds);
        }
    }

    fn cps2_objram1_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        if let Some(c) = self.cps2.as_mut() {
            c.objram1_w(word, data, uds, lds);
        }
    }

    fn cps2_objram2_read(&self, word: usize) -> u16 {
        self.cps2.as_ref().map_or(0, |c| c.objram2_r(word))
    }

    fn cps2_objram2_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        if let Some(c) = self.cps2.as_mut() {
            c.objram2_w(word, data, uds, lds);
        }
    }

    fn cps2_objram_bank_write(&mut self, data: u16, lds: bool) {
        if let Some(c) = self.cps2.as_mut() {
            c.objram_bank_w(data, lds);
        }
    }
}

/// Bus CPS1 complet avec vidéo réelle.
pub type CpsBus = Bus68k<Video>;

/// Construit un bus CPS1 pour le jeu donné.
pub fn new_cps_bus(rom: Vec<u8>, game: &str, dips: Dips) -> CpsBus {
    let video = Video::new(game);
    Bus68k::new(rom, video, dips)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cps_common::input::InputState;

    #[test]
    fn sf2_cps_b_read_returns_0401() {
        let mut bus = new_cps_bus(vec![], "sf2", Dips::default());
        assert_eq!(bus.read16(0x800172, true, true), 0x0401);
    }

    #[test]
    fn ghouls_cps_b_read_returns_0000() {
        let mut bus = new_cps_bus(vec![], "ghouls", Dips::default());
        assert_eq!(bus.read16(0x800140, true, true), 0x0000);
    }

    #[test]
    fn ghouls_cps_b_read_nonmatching_returns_ffff() {
        let mut bus = new_cps_bus(vec![], "ghouls", Dips::default());
        assert_eq!(bus.read16(0x80014A, true, true), 0xFFFF);
    }

    #[test]
    fn gfxram_write_read_roundtrip() {
        let mut bus = new_cps_bus(vec![], "ghouls", Dips::default());
        bus.write16(0x900000, true, true, 0xBEEF);
        assert_eq!(bus.read16(0x900000, true, true), 0xBEEF);
    }

    #[test]
    fn gfxram_write_read_high_address() {
        let mut bus = new_cps_bus(vec![], "ghouls", Dips::default());
        bus.write16(0x92FFFE, true, false, 0x1200);
        assert_eq!(bus.read16(0x92FFFE, true, true), 0x1200);
    }

    #[test]
    fn cps_a_read_returns_zero() {
        let mut bus = new_cps_bus(vec![], "ghouls", Dips::default());
        assert_eq!(bus.read16(0x800100, true, true), 0);
    }

    #[test]
    fn sf2_inputs_via_set_inputs() {
        let mut bus = new_cps_bus(vec![], "sf2", Dips::default());
        bus.set_inputs(InputState {
            in2: 0xFB,
            ..Default::default()
        });
        assert_eq!(bus.read16(0x800176, true, true), 0x00FB);
    }

    #[test]
    fn palette_write_and_build_ghouls() {
        let mut bus = new_cps_bus(vec![], "ghouls", Dips::default());
        // Write palette control (regs_b[0x18]) via CPS-B write
        bus.write16(0x800170, true, true, 0x003F);
        // Write white color word to gfxram at palette base location
        bus.write16(0x914000, true, true, 0xF800);
        // Trigger palette copy by writing PALETTE_BASE via CPS-A write
        bus.write16(0x80010A, true, true, 0x9140);
        assert_eq!(bus.video.palette[0], 0x880000);
    }
}
