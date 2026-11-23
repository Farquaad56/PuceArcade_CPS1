//! Tests de l'EEPROM 93C46 (organisations 8 et 16 bits).

use super::*;

/// Pilote un mot de commande bit à bit (CS haut), comme le 68000.
struct Host {
    e: Eeprom93c46,
    t: Tick,
}

impl Host {
    fn new() -> Self {
        Host::with(Eeprom93c46::new())
    }
    fn with(e: Eeprom93c46) -> Self {
        Host { e, t: 1000 }
    }
    fn port(&mut self, di: bool, clk: bool, cs: bool) {
        self.t += 1000;
        let v = u8::from(di) | if clk { 0x40 } else { 0 } | if cs { 0x80 } else { 0 };
        self.e.write_port(v, self.t);
    }
    fn bit(&mut self, b: bool) {
        self.port(b, false, true);
        self.port(b, true, true);
    }
    fn send(&mut self, bits: u32, n: u32) {
        for i in (0..n).rev() {
            self.bit(bits >> i & 1 != 0);
        }
    }
    fn select(&mut self) {
        self.port(false, false, false);
        self.port(false, false, true);
    }
    fn deselect(&mut self) {
        self.port(false, false, false);
    }
    fn read(&mut self, addr: u32) -> u8 {
        self.select();
        self.send(0b110 << 7 | addr, 10); // start + READ + adresse
        let mut v = 0;
        for _ in 0..8 {
            self.bit(false);
            v = v << 1 | u8::from(self.e.do_read(self.t));
        }
        self.deselect();
        v
    }
}

#[test]
fn default_contents_and_read() {
    let mut h = Host::new();
    assert_eq!(h.read(0x05), 0xFF);
}

#[test]
fn locked_write_is_ignored_then_unlock_write_read() {
    let mut h = Host::new();
    h.select();
    h.send(0b101 << 7 | 0x05, 10); // WRITE 5
    h.send(0x5A, 8);
    h.deselect();
    assert_eq!(h.e.data[5], 0xFF, "verrouillée au démarrage");
    h.select();
    h.send(0b100 << 7 | 0b11 << 5, 10); // EWEN (unlock)
    h.deselect();
    h.select();
    h.send(0b101 << 7 | 0x05, 10);
    h.send(0x5A, 8);
    h.deselect();
    assert_eq!(h.e.data[5], 0x5A);
    // occupé 1,75 ms : DO = 0 en attente du bit de départ
    h.select();
    assert!(!h.e.do_read(h.t));
    h.t += WRITE_TIME;
    assert!(h.e.do_read(h.t), "prêt après le temps d'écriture");
    h.deselect();
    assert_eq!(h.read(0x05), 0x5A);
}

#[test]
fn word16_write_little_endian() {
    let mut h = Host::with(Eeprom93c46::new_16bit());
    assert_eq!(h.e.data.len(), 128);
    h.select();
    h.send(0b100 << 6 | 0b11 << 4, 9); // EWEN (unlock)
    h.deselect();
    h.select();
    h.send(0b101 << 6 | 5, 9); // start + WRITE + adresse 5
    h.send(0x1234, 16);
    h.deselect();
    h.t += WRITE_TIME;
    assert_eq!((h.e.data[10], h.e.data[11]), (0x34, 0x12));

    // lecture : bit factice 0, puis 0x1234 du bit 15 au bit 0
    h.select();
    h.send(0b110 << 6 | 5, 9);
    assert!(!h.e.do_read(h.t), "bit factice");
    let mut v = 0u16;
    for _ in 0..16 {
        h.bit(false);
        v = v << 1 | u16::from(h.e.do_read(h.t));
    }
    h.deselect();
    assert_eq!(v, 0x1234);
}

#[test]
fn word_port_bits() {
    let mut e = Eeprom93c46::new_16bit();
    e.write_word_port(0x7000, 10);
    assert!(e.di && e.clk && e.cs);
    let mut e = Eeprom93c46::new_16bit();
    e.write_word_port(0x5000, 10);
    assert!(e.di && !e.clk && e.cs);
}
