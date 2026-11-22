//! EEPROM série 93C46 en organisation 8 bits (128 × 8), des cartes QSound.
// license:BSD-3-Clause
// copyright-holders:Aaron Giles
// source: MAME 0.287 src/devices/machine/eepromser.cpp (eeprom_serial_base_device,
//         eeprom_serial_93cxx_device) et eeprom.cpp (temps d'opération, valeur par défaut ~0)

use cps_common::clock::{Tick, TICKS_PER_SECOND};

const ADDRESS_BITS: u32 = 7;
const DATA_BITS: u32 = 8;
const COMMAND_ADDRESS_BITS: u32 = 7;

/// eeprom.cpp l.42-45 (µs -> ticks)
const fn us(n: u64) -> Tick {
    n * TICKS_PER_SECOND / 1_000_000
}
const WRITE_TIME: Tick = us(1750);
const WRITE_ALL_TIME: Tick = us(8000);
const ERASE_TIME: Tick = us(1000);
const ERASE_ALL_TIME: Tick = us(8000);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum State {
    InReset,
    WaitForStartBit,
    WaitForCommand,
    ReadingData,
    WaitForData,
    WaitForCompletion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum Command {
    Invalid,
    Read,
    Write,
    Erase,
    Lock,
    Unlock,
    WriteAll,
    EraseAll,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eeprom93c46 {
    pub data: Vec<u8>,
    state: State,
    locked: bool,
    bits_accum: u32,
    command_address_accum: u32,
    command: Command,
    address: u32,
    shift_register: u32,
    cs: bool,
    clk: bool,
    di: bool,
    last_cs_rising_edge_time: Tick,
    completion_time: Tick,
}

impl Default for Eeprom93c46 {
    fn default() -> Self {
        Self::new()
    }
}

impl Eeprom93c46 {
    /// Contenu par défaut : tout à 0xFF (eeprom.cpp nvram_default, ~0).
    pub fn new() -> Self {
        Eeprom93c46 {
            data: vec![0xFF; 1 << ADDRESS_BITS],
            state: State::InReset,
            locked: true,
            bits_accum: 0,
            command_address_accum: 0,
            command: Command::Invalid,
            address: 0,
            shift_register: 0,
            cs: false,
            clk: false,
            di: false,
            last_cs_rising_edge_time: 0,
            completion_time: 0,
        }
    }

    fn ready(&self, now: Tick) -> bool {
        now >= self.completion_time
    }

    /// do_read (93cxx) : prêt/occupé en attente du bit de départ, sinon bit de donnée.
    pub fn do_read(&self, now: Tick) -> bool {
        match self.state {
            State::WaitForStartBit => self.ready(now),
            State::ReadingData => self.shift_register & 0x8000_0000 != 0,
            _ => true, // m_do_tristate = ASSERT_LINE
        }
    }

    /// Écriture du port EEPROMOUT : bit 0 = DI, bit 6 = CLK, bit 7 = CS (dans cet ordre, comme
    /// les champs du port de MAME).
    pub fn write_port(&mut self, data: u8, now: Tick) {
        self.di_write(data & 0x01 != 0);
        self.clk_write(data & 0x40 != 0, now);
        self.cs_write(data & 0x80 != 0, now);
    }

    pub fn di_write(&mut self, state: bool) {
        self.di = state;
    }

    pub fn cs_write(&mut self, state: bool, now: Tick) {
        if state == self.cs {
            return;
        }
        self.cs = state;
        if state {
            self.last_cs_rising_edge_time = now;
            self.cs_rising();
        } else {
            self.cs_falling();
        }
    }

    pub fn clk_write(&mut self, state: bool, now: Tick) {
        if state == self.clk {
            return;
        }
        self.clk = state;
        if state {
            self.clk_rising(now);
        }
        // front descendant : sans effet (output_on_falling_clock désactivé)
    }

    fn cs_rising(&mut self) {
        if self.state == State::InReset {
            self.state = State::WaitForStartBit;
        }
    }

    fn cs_falling(&mut self) {
        // tous les états sauf IN_RESET repassent en IN_RESET
        self.state = State::InReset;
    }

    fn clk_rising(&mut self, now: Tick) {
        match self.state {
            State::WaitForStartBit => {
                if self.di && self.ready(now) && now > self.last_cs_rising_edge_time {
                    self.command_address_accum = 0;
                    self.bits_accum = 0;
                    self.state = State::WaitForCommand;
                }
            }
            State::WaitForCommand => {
                self.command_address_accum = (self.command_address_accum << 1) | u32::from(self.di);
                self.bits_accum += 1;
                if self.bits_accum == 2 + COMMAND_ADDRESS_BITS {
                    self.execute_command(now);
                }
            }
            State::ReadingData => {
                let bit_index = self.bits_accum;
                self.bits_accum += 1;
                // streaming désactivé : seul le premier mot est chargé
                if bit_index == 0 {
                    let a =
                        (self.address + self.bits_accum / DATA_BITS) & ((1 << ADDRESS_BITS) - 1);
                    self.shift_register = u32::from(self.data[a as usize]) << (32 - DATA_BITS);
                } else {
                    self.shift_register = (self.shift_register << 1) | 1;
                }
            }
            State::WaitForData => {
                self.shift_register = (self.shift_register << 1) | u32::from(self.di);
                self.bits_accum += 1;
                if self.bits_accum == DATA_BITS {
                    self.execute_write_command(now);
                }
            }
            State::InReset | State::WaitForCompletion => {}
        }
    }

    /// parse_command_and_address (93cxx)
    fn parse_command_and_address(&mut self) {
        self.command = Command::Invalid;
        self.address = self.command_address_accum & ((1 << COMMAND_ADDRESS_BITS) - 1);
        match self.command_address_accum >> COMMAND_ADDRESS_BITS {
            0 => {
                self.command = match self.address >> (COMMAND_ADDRESS_BITS - 2) {
                    0 => Command::Lock,
                    1 => Command::WriteAll,
                    2 => Command::EraseAll,
                    _ => Command::Unlock,
                };
                self.address = 0;
            }
            1 => self.command = Command::Write,
            2 => self.command = Command::Read,
            _ => self.command = Command::Erase,
        }
    }

    fn execute_command(&mut self, now: Tick) {
        self.parse_command_and_address();
        self.bits_accum = 0;
        match self.command {
            Command::Read => {
                self.shift_register = 0;
                self.state = State::ReadingData;
            }
            Command::Write | Command::WriteAll => {
                self.shift_register = 0;
                self.state = State::WaitForData;
            }
            Command::Erase => {
                if self.locked {
                    self.state = State::InReset;
                } else {
                    self.data[self.address as usize] = 0xFF;
                    self.completion_time = now + ERASE_TIME;
                    self.state = State::WaitForCompletion;
                }
            }
            Command::Lock => {
                self.locked = true;
                self.state = State::InReset;
            }
            Command::Unlock => {
                self.locked = false;
                self.state = State::InReset;
            }
            Command::EraseAll => {
                if self.locked {
                    self.state = State::InReset;
                } else {
                    self.data.fill(0xFF);
                    self.completion_time = now + ERASE_ALL_TIME;
                    self.state = State::WaitForCompletion;
                }
            }
            // PORT-NOTE: MAME lève une erreur fatale ; impossible avec 2 + 7 bits complets.
            Command::Invalid => self.state = State::InReset,
        }
    }

    fn execute_write_command(&mut self, now: Tick) {
        if self.locked {
            self.state = State::InReset;
            return;
        }
        let v = self.shift_register as u8;
        match self.command {
            Command::Write => {
                self.data[self.address as usize] = v;
                self.completion_time = now + WRITE_TIME;
            }
            _ => {
                // WRITEALL : internal_write(a, internal_read(a) & data)
                self.data.iter_mut().for_each(|d| *d &= v);
                self.completion_time = now + WRITE_ALL_TIME;
            }
        }
        self.state = State::WaitForCompletion;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pilote un mot de commande bit à bit (CS haut), comme le 68000.
    struct Host {
        e: Eeprom93c46,
        t: Tick,
    }

    impl Host {
        fn new() -> Self {
            Host {
                e: Eeprom93c46::new(),
                t: 1000,
            }
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
}
