//! Accès du 68000 CPS2 servis par l'hôte (il possède la carte son) : RAM partagée QSound.
//! spec: docs/spec/05 §Carte mémoire 68000, §Son ; source: ref/mame-cps2/cps2.cpp l.1324-1347
//! (618000-619FFF : qsound_sharedram1_r / _w, umask 0x00FF)

use crate::host68k::Host68k;

impl Host68k<'_> {
    /// 618000-619FFF : RAM partagée 1 (Z80 C000-CFFF), `octet | 0xFF00`. None ailleurs.
    pub(crate) fn cps2_read(&mut self, a: u32) -> Option<u16> {
        match a as usize {
            a @ 0x618000..=0x619FFF => Some(0xff00 | u16::from(self.sound.ram[(a - 0x618000) / 2])),
            _ => None,
        }
    }

    /// Écriture de l'octet bas (lds) de la RAM partagée. true si l'adresse est servie.
    pub(crate) fn cps2_write(&mut self, a: u32, lds: bool, data: u16) -> bool {
        match a as usize {
            a @ 0x618000..=0x619FFF => {
                if lds {
                    self.sound.ram[(a - 0x618000) / 2] = data as u8;
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};
    use puce_m68k::bus::Bus;

    fn machine() -> Machine {
        Machine::new(testutil::synthetic_cps2_game())
    }

    #[test]
    fn construction() {
        let m = machine();
        assert!(m.bus.cps2.is_some());
        assert!(m.bus.video.cps2.is_some());
        assert!(!m.bus.qsound);
        assert!(m.bus.eeprom.is_some());
        assert!(m.sound.qsound.is_some());
        assert_eq!(m.ticks_68k_den, 59);
    }

    #[test]
    fn opcodes_by_function_code() {
        let mut m = machine();
        let (_, mut h) = m.split();
        assert_eq!(h.read(0x400, 6, true, true), 0x2222);
        assert_eq!(h.read(0x400, 5, true, true), 0x1111);
        assert_eq!(h.read(0x400, 2, true, true), 0x2222);
        assert_eq!(h.read(0x500000, 6, true, true), 0);
    }

    #[test]
    fn shared_ram_and_inputs() {
        let mut m = machine();
        {
            let (_, mut h) = m.split();
            h.write(0x618002, 5, false, true, 0x00AB);
            assert_eq!(h.read(0x618002, 5, true, true), 0xFFAB);
        }
        assert_eq!(m.sound.ram[1], 0xAB);
        m.set_cps2_inputs(cps_bus::Cps2Inputs {
            in0: 0x1234,
            ..Default::default()
        });
        let (_, mut h) = m.split();
        assert_eq!(h.read(0x804000, 5, true, true), 0x1234);
    }
}
