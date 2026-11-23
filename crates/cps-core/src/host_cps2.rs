//! Accès du 68000 CPS2 servis par l'hôte (il possède la carte son) : RAM partagée QSound.
//! spec: docs/spec/05 §Carte mémoire 68000, §Son ; source: ref/mame-cps2/cps2.cpp l.1324-1347
//! (618000-619FFF : qsound_sharedram1_r / _w, umask 0x00FF) ; l.1121-1160 (cps2_eeprom_port_w :
//! EEPROM sur l'octet fort, reset du Z80 sur l'octet faible) ; IN2 bit 0 = DO de l'EEPROM.

use crate::host68k::Host68k;

impl Host68k<'_> {
    /// 618000-619FFF : RAM partagée 1 (Z80 C000-CFFF), `octet | 0xFF00`. None ailleurs.
    pub(crate) fn cps2_read(&mut self, a: u32) -> Option<u16> {
        match a as usize {
            a @ 0x618000..=0x619FFF => Some(0xff00 | u16::from(self.sound.ram[(a - 0x618000) / 2])),
            0x804020 | 0x804021 => {
                let in2 = self.bus.cps2.as_ref()?.inputs.in2;
                let dout = self
                    .bus
                    .eeprom
                    .as_ref()
                    .is_some_and(|e| e.do_read(*self.now));
                Some((in2 & 0xFFFE) | u16::from(dout))
            }
            _ => None,
        }
    }

    /// Écriture de l'octet bas (lds) de la RAM partagée. true si l'adresse est servie.
    pub(crate) fn cps2_write(&mut self, a: u32, uds: bool, lds: bool, data: u16) -> bool {
        match a as usize {
            a @ 0x618000..=0x619FFF => {
                if lds {
                    self.sound.ram[(a - 0x618000) / 2] = data as u8;
                }
                true
            }
            0x804040 | 0x804041 => {
                let now = *self.now;
                if uds {
                    // octet fort : EEPROM (DI bit 12, CLK bit 13, CS bit 14)
                    if let Some(e) = self.bus.eeprom.as_mut() {
                        e.write_word_port(data, now);
                    }
                }
                if lds {
                    // octet faible : bit 3 = 0 tient le Z80 en reset ; rattrapage d'abord
                    self.z80_run_until(now);
                    self.sound.set_z80_reset(data & 0x0008 == 0, self.z80);
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

    /// Pilote l'EEPROM par le port 804040 (uds seul), un pas de 1000 ticks par écriture.
    struct Port<'m> {
        m: &'m mut Machine,
    }

    impl Port<'_> {
        fn set(&mut self, di: bool, clk: bool, cs: bool) {
            let (_, mut h) = self.m.split();
            *h.now += 1000;
            let v = u16::from(di) << 12 | u16::from(clk) << 13 | u16::from(cs) << 14;
            h.write(0x804040, 5, true, false, v);
        }
        fn bit(&mut self, b: bool) {
            self.set(b, false, true);
            self.set(b, true, true);
        }
        fn send(&mut self, bits: u32, n: u32) {
            for i in (0..n).rev() {
                self.bit(bits >> i & 1 != 0);
            }
        }
        fn select(&mut self) {
            self.set(false, false, false);
            self.set(false, false, true);
        }
        fn do_bit(&mut self) -> u16 {
            let (_, mut h) = self.m.split();
            h.read(0x804020, 5, true, true) & 1
        }
    }

    #[test]
    fn eeprom_through_port_and_in2() {
        let mut m = machine();
        m.set_cps2_inputs(cps_bus::Cps2Inputs {
            in2: 0xFFFE,
            ..Default::default()
        });
        let mut p = Port { m: &mut m };
        p.select();
        p.send(0b100 << 6 | 0b11 << 4, 9); // EWEN
        p.select();
        p.send(0b101 << 6 | 5, 9); // WRITE 5
        p.send(0x1234, 16);
        p.set(false, false, false);
        {
            let (_, h) = p.m.split();
            *h.now += 5_040_000_000 / 100; // 10 ms > WRITE_TIME
        }
        p.select();
        p.send(0b110 << 6 | 5, 9); // READ 5
        assert_eq!(p.do_bit(), 0, "bit factice");
        let mut v = 0u16;
        for _ in 0..16 {
            p.bit(false);
            v = v << 1 | p.do_bit();
        }
        assert_eq!(v, 0x1234);
        assert!(!p.m.sound.z80_reset_held, "uds seul : reset du Z80 intact");
    }

    #[test]
    fn z80_reset_line() {
        let mut m = machine();
        let eeprom_before = m.bus.eeprom.clone();
        {
            let (_, mut h) = m.split();
            h.write(0x804040, 5, false, true, 0x7000);
        }
        assert!(m.sound.z80_reset_held);
        assert_eq!(m.bus.eeprom, eeprom_before, "lds seul : EEPROM intacte");
        {
            let (_, mut h) = m.split();
            h.write(0x804040, 5, false, true, 0x0008);
        }
        assert!(!m.sound.z80_reset_held);
    }
}
