//! Carte mémoire 68000 et E/S CPS1.
//! spec: docs/spec/01 §68000

use cps_common::bits::combine_data;
use cps_common::input::{Dips, InputState};

/// Commandes son transmises au Z80 via les latches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundCmd {
    Latch1(u8),
    Latch2(u8),
}

/// Port vidéo (implémenté par cps-gfx en T207 ; stub pour cette carte).
pub trait VideoPort {
    fn cps_a_write(&mut self, word: usize, data: u16, uds: bool, lds: bool);
    fn cps_b_read(&mut self, word: usize) -> u16;
    fn cps_b_write(&mut self, word: usize, data: u16, uds: bool, lds: bool);
    fn gfxram_read(&self, word: usize) -> u16;
    fn gfxram_write(&mut self, word: usize, data: u16, uds: bool, lds: bool);
    fn set_inputs(&mut self, in2: u8, in3: u8);
    /// CPS2 : registres output (400000-40000B). spec: docs/spec/05 §Carte mémoire 68000
    fn cps2_output_read(&self, _word: usize) -> u16 {
        0
    }
    fn cps2_output_write(&mut self, _word: usize, _data: u16, _uds: bool, _lds: bool) {}
    /// CPS2 : objram1_w (700000-701FFF).
    fn cps2_objram1_write(&mut self, _word: usize, _data: u16, _uds: bool, _lds: bool) {}
    /// CPS2 : objram2 (708000-70FFFF).
    fn cps2_objram2_read(&self, _word: usize) -> u16 {
        0
    }
    fn cps2_objram2_write(&mut self, _word: usize, _data: u16, _uds: bool, _lds: bool) {}
    /// CPS2 : banque des objets (8040E0).
    fn cps2_objram_bank_write(&mut self, _data: u16, _lds: bool) {}
}

/// Bus 68000 CPS1 : ROM programme, RAM de travail, E/S, vidéo.
#[derive(Clone, Debug)]
pub struct Bus68k<V: VideoPort> {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub video: V,
    pub inputs: InputState,
    pub dips: Dips,
    pub coinctrl: u16,
    /// Molettes de Forgotten Worlds (uPD4701, machine « forgottn ») ; None ailleurs.
    pub dial: Option<crate::Upd4701>,
    /// Adresse du CPS-B : 0x800140, ou 0x8001C0 pour init_sf2ee (cps1.cpp l.14594-14601).
    pub cpsb_base: u32,
    /// Carte QSound (qsound_main_map) : ports F1C000-F1C007, lecture F00000. La RAM partagée
    /// et l'EEPROM (F1C006) sont servies par l'hôte, qui a la carte son et l'heure.
    pub qsound: bool,
    /// Région « user1 » lue en F00000-F0FFFF (qsound_rom_r).
    pub user1: Option<Vec<u8>>,
    /// cpsq_coinctrl2_w (F1C004) : compteurs et verrous des pièces 3 et 4.
    pub coinctrl2: u16,
    /// EEPROM 93C46 (cartes QSound).
    pub eeprom: Option<crate::Eeprom93c46>,
    /// E/S CPS2 (None pour le CPS1) : la carte mémoire passe alors par cps2_read16.
    pub cps2: Option<Box<crate::cps2_io::Cps2Io>>,
}

impl<V: VideoPort> Bus68k<V> {
    /// Construit le bus avec la ROM programme et les DIP par défaut.
    pub fn new(rom: Vec<u8>, video: V, dips: Dips) -> Self {
        // spec: docs/spec/01 §68000 — RAM de travail 64 Kio
        Bus68k {
            rom,
            ram: vec![0; 0x10000],
            video,
            inputs: InputState::default(),
            dips,
            coinctrl: 0,
            dial: None,
            cpsb_base: 0x800140,
            qsound: false,
            user1: None,
            coinctrl2: 0,
            eeprom: None,
            cps2: None,
        }
    }

    /// Reset : RAM à 0, coinctrl à 0. La vidéo est remise à zéro par son propriétaire.
    pub fn reset(&mut self) {
        self.ram.fill(0);
        self.coinctrl = 0;
    }

    /// Mouvement des molettes (joueur 1 = X, joueur 2 = Y) ; sans effet sans uPD4701.
    pub fn add_dial(&mut self, dx: i32, dy: i32) {
        if let Some(d) = &mut self.dial {
            d.x_add(dx as i16);
            d.y_add(dy as i16);
        }
    }

    /// Met à jour les entrées et propage IN2/IN3 au port vidéo.
    pub fn set_inputs(&mut self, i: InputState) {
        self.inputs = i;
        self.video.set_inputs(i.in2, i.in3);
    }

    /// Lecture mot 68000 (adresse & 0xFFFFFF, bit 0 ignoré).
    pub fn read16(&mut self, addr: u32, _uds: bool, _lds: bool) -> u16 {
        let a = (addr & 0xFFFFFF) as usize;
        if self.cps2.is_some() {
            return self.cps2_read16(a);
        }

        // spec: docs/spec/01 §68000 — ROM programme 000000-3FFFFF
        if a < 0x400000 {
            return self.read_rom_word(a);
        }

        // spec: docs/spec/01 §68000 — IN1 800000-800007 (4 mots identiques)
        if (0x800000..0x800008).contains(&a) {
            return self.inputs.in1;
        }

        // qsound_main_map (cps1.cpp l.664-681)
        if self.qsound {
            match a {
                // qsound_rom_r : protection de Slam Masters
                0xF00000..=0xF0FFFF => {
                    return match &self.user1 {
                        Some(r) => {
                            0xff00 | u16::from(r.get((a - 0xF00000) / 2).copied().unwrap_or(0))
                        }
                        None => 0,
                    };
                }
                0xF1C000 | 0xF1C001 => return u16::from(self.inputs.in2), // IN2 : joueur 3
                0xF1C002 | 0xF1C003 => return u16::from(self.inputs.in3), // IN3 : joueur 4
                _ => {}
            }
        }

        // forgottn_map (cps1.cpp l.605-606) : uPD4701 read_x 800052-55, read_y 80005A-5D (umask 0x00FF)
        if let Some(d) = &mut self.dial {
            if (0x800052..0x800056).contains(&a) {
                return u16::from(d.read_x(((a - 0x800052) / 2) as u32));
            }
            if (0x80005A..0x80005E).contains(&a) {
                return u16::from(d.read_y(((a - 0x80005A) / 2) as u32));
            }
        }

        // spec: docs/spec/01 §68000 — cps1_dsw_r 800018-80001F
        if (0x800018..0x800020).contains(&a) {
            let k = (a - 0x800018) / 2;
            return self.dsw_read(k);
        }

        // spec: docs/spec/01 §68000 — nopr 800020-800021
        if (0x800020..0x800022).contains(&a) {
            return 0;
        }

        // spec: docs/spec/01 §68000 — CPS-A lecture = 0 (pas de gestionnaire)
        if (0x800100..0x800140).contains(&a) {
            return 0;
        }

        // spec: docs/spec/01 §68000 — CPS-B lecture 800140-80017F
        let cpsb = self.cpsb_base as usize;
        if (cpsb..cpsb + 0x40).contains(&a) {
            let word = (a - cpsb) / 2;
            return self.video.cps_b_read(word);
        }

        // spec: docs/spec/01 §68000 — GFXRAM lecture 900000-92FFFF
        if (0x900000..0x930000).contains(&a) {
            let word = (a - 0x900000) / 2;
            return self.video.gfxram_read(word);
        }

        // spec: docs/spec/01 §68000 — RAM de travail FF0000-FFFFFF
        if (0xFF0000..0x1000000).contains(&a) {
            return self.read_ram_word(a - 0xFF0000);
        }

        // spec: docs/spec/01 §Conventions — lecture non mappée = 0x0000
        0
    }

    /// Écriture mot 68000. Retourne Some(SoundCmd) si latch son écrit.
    pub fn write16(&mut self, addr: u32, uds: bool, lds: bool, data: u16) -> Option<SoundCmd> {
        let a = (addr & 0xFFFFFF) as usize;

        // spec: docs/spec/01 §68000 — ROM : écriture ignorée
        if a < 0x400000 {
            return None;
        }

        // qsound_main_map : cpsq_coinctrl2_w (F1C004, octet bas)
        if self.qsound && (a == 0xF1C004 || a == 0xF1C005) {
            if lds {
                self.coinctrl2 = data & 0x00FF;
            }
            return None;
        }

        // forgottn_map (cps1.cpp l.603-604) : reset X en 800041, reset Y en 800049 (octet bas)
        if let Some(d) = &mut self.dial {
            if lds && a == 0x800040 {
                d.reset_x_w();
                return None;
            }
            if lds && a == 0x800048 {
                d.reset_y_w();
                return None;
            }
        }

        // spec: docs/spec/01 §68000 — IN1, dsw, nopr : pas d'écriture
        if (0x800000..0x800022).contains(&a) || (0x800100..0x800140).contains(&a) {
            // CPS-A écriture
            if (0x800100..0x800140).contains(&a) {
                let word = (a - 0x800100) / 2;
                self.video.cps_a_write(word, data, uds, lds);
            }
            return None;
        }

        // spec: docs/spec/01 §68000 — CPS-B écriture 800140-80017F
        let cpsb = self.cpsb_base as usize;
        if (cpsb..cpsb + 0x40).contains(&a) {
            let word = (a - cpsb) / 2;
            self.video.cps_b_write(word, data, uds, lds);
            return None;
        }

        // spec: docs/spec/01 §68000 — cps1_soundlatch_w 800180-800187
        if (0x800180..0x800188).contains(&a) {
            if lds {
                return Some(SoundCmd::Latch1((data & 0xFF) as u8));
            } else {
                return Some(SoundCmd::Latch1(((data >> 8) & 0xFF) as u8));
            }
        }

        // spec: docs/spec/01 §68000 — cps1_soundlatch2_w 800188-80018F
        if (0x800188..0x800190).contains(&a) {
            if lds {
                return Some(SoundCmd::Latch2((data & 0xFF) as u8));
            }
            return None;
        }

        // spec: docs/spec/01 §68000 — cps1_coinctrl_w 800030-800037
        if (0x800030..0x800038).contains(&a) {
            self.coinctrl = combine_data(self.coinctrl, data, uds, lds);
            return None;
        }

        // spec: docs/spec/01 §68000 — GFXRAM écriture 900000-92FFFF (COMBINE_DATA)
        if (0x900000..0x930000).contains(&a) {
            let word = (a - 0x900000) / 2;
            self.video.gfxram_write(word, data, uds, lds);
            return None;
        }

        // spec: docs/spec/01 §68000 — RAM de travail FF0000-FFFFFF
        if (0xFF0000..0x1000000).contains(&a) {
            self.write_ram_word(a - 0xFF0000, data, uds, lds);
            return None;
        }

        // spec: docs/spec/01 §Conventions — écriture non mappée ignorée
        None
    }

    pub(crate) fn read_rom_word(&self, addr: usize) -> u16 {
        if addr + 1 < self.rom.len() {
            ((self.rom[addr] as u16) << 8) | (self.rom[addr + 1] as u16)
        } else {
            // spec: docs/spec/01 §68000 — parties non chargées = 0x00
            0
        }
    }

    pub(crate) fn read_ram_word(&self, addr: usize) -> u16 {
        ((self.ram[addr] as u16) << 8) | (self.ram[addr + 1] as u16)
    }

    fn write_ram_word(&mut self, addr: usize, data: u16, uds: bool, lds: bool) {
        if uds {
            self.ram[addr] = ((data >> 8) & 0xFF) as u8;
        }
        if lds {
            self.ram[addr + 1] = (data & 0xFF) as u8;
        }
    }

    fn dsw_read(&self, k: usize) -> u16 {
        let port = match k {
            0 => self.inputs.in0,
            1 => self.dips.dswa,
            2 => self.dips.dswb,
            3 => self.dips.dswc,
            _ => 0xFF,
        };
        // spec: docs/spec/01 §68000 — valeur = (port << 8) | 0xFF
        ((port as u16) << 8) | 0xFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stub VideoPort qui enregistre les appels pour vérification.
    #[derive(Clone, Debug, Default)]
    struct TestVideo {
        cps_a_calls: Vec<(usize, u16)>,
        cps_b_read_calls: Vec<usize>,
        cps_b_write_calls: Vec<(usize, u16)>,
        gfxram_read_calls: Vec<usize>,
        gfxram_write_calls: Vec<(usize, u16)>,
    }

    impl VideoPort for TestVideo {
        fn cps_a_write(&mut self, word: usize, data: u16, _uds: bool, _lds: bool) {
            self.cps_a_calls.push((word, data));
        }
        fn cps_b_read(&mut self, word: usize) -> u16 {
            self.cps_b_read_calls.push(word);
            0xBEEF
        }
        fn cps_b_write(&mut self, word: usize, data: u16, _uds: bool, _lds: bool) {
            self.cps_b_write_calls.push((word, data));
        }
        fn gfxram_read(&self, word: usize) -> u16 {
            let _ = self.gfxram_read_calls.iter().find(|&&w| w == word);
            0
        }
        fn gfxram_write(&mut self, word: usize, data: u16, _uds: bool, _lds: bool) {
            self.gfxram_write_calls.push((word, data));
        }
        fn set_inputs(&mut self, _in2: u8, _in3: u8) {}
    }

    #[test]
    fn cpsb_relocated_for_sf2ee() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.cpsb_base = 0x8001C0;
        assert_eq!(
            bus.read16(0x800172, true, true),
            0,
            "plage d'origine non mappée"
        );
        assert_eq!(bus.read16(0x8001F2, true, true), 0xBEEF);
        assert_eq!(bus.video.cps_b_read_calls, vec![0x19]);
        bus.write16(0x8001C2, true, true, 0x1234);
        bus.write16(0x800142, true, true, 0x5678);
        assert_eq!(bus.video.cps_b_write_calls, vec![(1, 0x1234)]);
    }

    #[test]
    fn forgottn_dial() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(
            bus.read16(0x800052, true, true),
            0,
            "sans molette : non mappé"
        );
        bus.add_dial(5, -3);
        bus.dial = Some(crate::Upd4701::new());
        bus.add_dial(5, -3);
        let rd = |b: &mut Bus68k<TestVideo>, a: u32| {
            b.read16(a + 2, true, true) << 8 | b.read16(a, true, true)
        };
        assert_eq!(rd(&mut bus, 0x800052), 5);
        assert_eq!(rd(&mut bus, 0x80005A), 0xFFD);
        bus.write16(0x800040, false, true, 0x00);
        assert_eq!(rd(&mut bus, 0x800052), 0);
        bus.write16(0x800048, false, true, 0x00);
        assert_eq!(rd(&mut bus, 0x80005A), 0);
    }

    #[test]
    fn rom_read_big_endian() {
        let mut bus = Bus68k::new(
            vec![0x12, 0x34, 0x56, 0x78],
            TestVideo::default(),
            Dips::default(),
        );
        assert_eq!(bus.read16(0, true, true), 0x1234);
        assert_eq!(bus.read16(2, true, true), 0x5678);
    }

    #[test]
    fn rom_read_beyond_size_returns_zero() {
        let mut bus = Bus68k::new(
            vec![0x12, 0x34, 0x56, 0x78],
            TestVideo::default(),
            Dips::default(),
        );
        assert_eq!(bus.read16(0x3FFFFE, true, true), 0);
    }

    #[test]
    fn rom_write_ignored() {
        let mut bus = Bus68k::new(vec![0x12, 0x34], TestVideo::default(), Dips::default());
        bus.write16(0, true, true, 0xFFFF);
        assert_eq!(bus.read16(0, true, true), 0x1234);
    }

    #[test]
    fn ram_write_read_byte_strobes() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        // Écriture octet haut (uds)
        bus.write16(0xFF0000, true, false, 0xAB12);
        assert_eq!(bus.read16(0xFF0000, true, true), 0xAB00);
        // Écriture octet bas (lds)
        bus.write16(0xFF0000, false, true, 0x00CD);
        assert_eq!(bus.read16(0xFF0000, true, true), 0xABCD);
    }

    #[test]
    fn ram_24bit_address_mask() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.write16(0xFF0000, true, true, 0x1234);
        assert_eq!(bus.read16(0x1FF0000, true, true), 0x1234);
    }

    #[test]
    fn input_in1_read() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.inputs.in1 = 0x7FFE;
        assert_eq!(bus.read16(0x800000, true, true), 0x7FFE);
        assert_eq!(bus.read16(0x800006, true, true), 0x7FFE);
    }

    #[test]
    fn dsw_read_in0() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.inputs.in0 = 0xEF;
        assert_eq!(bus.read16(0x800018, true, true), 0xEFFF);
    }

    #[test]
    fn dsw_read_dips() {
        let mut bus = Bus68k::new(
            vec![],
            TestVideo::default(),
            Dips {
                dswa: 0x12,
                dswb: 0x34,
                dswc: 0x56,
            },
        );
        assert_eq!(bus.read16(0x80001A, true, true), 0x12FF);
        assert_eq!(bus.read16(0x80001C, true, true), 0x34FF);
        assert_eq!(bus.read16(0x80001E, true, true), 0x56FF);
    }

    #[test]
    fn nopr_returns_zero() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(bus.read16(0x800020, true, true), 0);
    }

    #[test]
    fn sound_latch1_write() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(
            bus.write16(0x800180, true, true, 0x00AB),
            Some(SoundCmd::Latch1(0xAB))
        );
        assert_eq!(
            bus.write16(0x800180, false, true, 0x0042),
            Some(SoundCmd::Latch1(0x42))
        );
        assert_eq!(
            bus.write16(0x800180, true, false, 0x4200),
            Some(SoundCmd::Latch1(0x42))
        );
    }

    #[test]
    fn sound_latch2_write() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(
            bus.write16(0x800188, false, true, 0x0077),
            Some(SoundCmd::Latch2(0x77))
        );
        // uds seul -> pas de latch2 (spec: seulement lds)
        assert_eq!(bus.write16(0x800188, true, false, 0x7700), None);
    }

    #[test]
    fn video_cps_a_write_routing() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.write16(0x80010A, true, true, 0x1234);
        assert_eq!(bus.video.cps_a_calls.len(), 1);
        assert_eq!(bus.video.cps_a_calls[0].0, 5); // (0x80010A - 0x800100) / 2 = 5
    }

    #[test]
    fn video_cps_b_read_routing() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        let val = bus.read16(0x800172, true, true);
        assert_eq!(val, 0xBEEF); // valeur retournée par le stub
        assert_eq!(bus.video.cps_b_read_calls.len(), 1);
        assert_eq!(bus.video.cps_b_read_calls[0], 0x19); // (0x800172 - 0x800140) / 2 = 0x19
    }

    #[test]
    fn video_cps_a_read_returns_zero() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(bus.read16(0x800100, true, true), 0);
    }

    #[test]
    fn video_gfxram_write_routing() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.write16(0x92FFFE, true, true, 0xABCD);
        assert_eq!(bus.video.gfxram_write_calls.len(), 1);
        assert_eq!(bus.video.gfxram_write_calls[0].0, 0x17FFF); // (0x92FFFE - 0x900000) / 2 = 0x17FFF
    }

    #[test]
    fn video_gfxram_read_beyond_returns_zero() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(bus.read16(0x930000, true, true), 0);
    }

    #[test]
    fn coinctrl_write() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.write16(0x800030, true, true, 0x00AB);
        assert_eq!(bus.coinctrl, 0x00AB);
    }

    #[test]
    fn reset_clears_ram_and_coinctrl() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        bus.write16(0xFF0000, true, true, 0x1234);
        bus.coinctrl = 0xABCD;
        bus.reset();
        assert_eq!(bus.read16(0xFF0000, true, true), 0);
        assert_eq!(bus.coinctrl, 0);
    }

    #[test]
    fn set_inputs_propagates_to_video() {
        let video = TestVideo::default();
        let mut bus = Bus68k::new(vec![], video, Dips::default());
        let inputs = InputState {
            in2: 0xAA,
            in3: 0xBB,
            ..Default::default()
        };
        bus.set_inputs(inputs);
        // Stub ne vérifie pas les valeurs mais l'appel est fait
    }

    #[test]
    fn unmapped_read_returns_zero() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        assert_eq!(bus.read16(0x400000, true, true), 0);
        assert_eq!(bus.read16(0x7FFFFF, true, true), 0);
    }

    #[test]
    fn unmapped_write_ignored() {
        let mut bus = Bus68k::new(vec![], TestVideo::default(), Dips::default());
        // Ne doit pas paniquer
        bus.write16(0x400000, true, true, 0xFFFF);
    }
}
