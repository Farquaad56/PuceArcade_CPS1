//! EEPROM série 93C46 en organisation 8 bits (128 × 8, cartes QSound) ou 16 bits (64 × 16, CPS2).
// license:BSD-3-Clause
// copyright-holders:Aaron Giles
// source: MAME 0.287 src/devices/machine/eepromser.cpp (eeprom_serial_base_device,
//         eeprom_serial_93cxx_device) et eeprom.cpp (temps d'opération, valeur par défaut ~0,
//         internal_read / internal_write l.283-303). spec: docs/spec/05 §Port 804040

use cps_common::clock::{Tick, TICKS_PER_SECOND};

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
    /// Bits d'adresse (7 en 8 bits, 6 en 16 bits) ; command_address_bits = address_bits
    /// (eepromser.cpp l.186-187).
    address_bits: u32,
    /// Largeur d'un mot (8 ou 16).
    data_bits: u32,
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
    /// Organisation 8 bits (cartes QSound). Contenu par défaut : tout à 0xFF (eeprom.cpp
    /// nvram_default, ~0).
    pub fn new() -> Self {
        Self::with_organisation(7, 8)
    }

    /// EEPROM_93C46_16BIT (eepromser.cpp l.1100) : 64 mots de 16 bits, 6 bits d'adresse (CPS2).
    pub fn new_16bit() -> Self {
        Self::with_organisation(6, 16)
    }

    fn with_organisation(address_bits: u32, data_bits: u32) -> Self {
        Eeprom93c46 {
            data: vec![0xFF; (1 << address_bits) * (data_bits as usize / 8)],
            address_bits,
            data_bits,
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

    /// internal_read (eeprom.cpp l.283-290) : mot petit-boutiste en 16 bits.
    fn internal_read(&self, address: u32) -> u32 {
        let a = address as usize;
        if self.data_bits == 16 {
            u32::from(self.data[2 * a]) | u32::from(self.data[2 * a + 1]) << 8
        } else {
            u32::from(self.data[a])
        }
    }

    /// internal_write (eeprom.cpp l.295-303).
    fn internal_write(&mut self, address: u32, value: u32) {
        let a = address as usize;
        if self.data_bits == 16 {
            self.data[2 * a] = value as u8;
            self.data[2 * a + 1] = (value >> 8) as u8;
        } else {
            self.data[a] = value as u8;
        }
    }

    /// Masque d'un mot (valeur d'un mot effacé).
    fn word_mask(&self) -> u32 {
        (1 << self.data_bits) - 1
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

    /// Écriture du port 804040 du CPS2 : DI = bit 12, CLK = bit 13, CS = bit 14, dans cet
    /// ordre (cps2.cpp l.1458-1461).
    pub fn write_word_port(&mut self, data: u16, now: Tick) {
        self.di_write(data & 0x1000 != 0);
        self.clk_write(data & 0x2000 != 0, now);
        self.cs_write(data & 0x4000 != 0, now);
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
                if self.bits_accum == 2 + self.address_bits {
                    self.execute_command(now);
                }
            }
            State::ReadingData => {
                let bit_index = self.bits_accum;
                self.bits_accum += 1;
                // streaming désactivé : seul le premier mot est chargé
                if bit_index == 0 {
                    let a = (self.address + self.bits_accum / self.data_bits)
                        & ((1 << self.address_bits) - 1);
                    self.shift_register = self.internal_read(a) << (32 - self.data_bits);
                } else {
                    self.shift_register = (self.shift_register << 1) | 1;
                }
            }
            State::WaitForData => {
                self.shift_register = (self.shift_register << 1) | u32::from(self.di);
                self.bits_accum += 1;
                if self.bits_accum == self.data_bits {
                    self.execute_write_command(now);
                }
            }
            State::InReset | State::WaitForCompletion => {}
        }
    }

    /// parse_command_and_address (93cxx)
    fn parse_command_and_address(&mut self) {
        self.command = Command::Invalid;
        let bits = self.address_bits; // command_address_bits
        self.address = self.command_address_accum & ((1 << bits) - 1);
        match self.command_address_accum >> bits {
            0 => {
                self.command = match self.address >> (bits - 2) {
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
                    self.internal_write(self.address, self.word_mask());
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
                    for a in 0..1 << self.address_bits {
                        self.internal_write(a, self.word_mask());
                    }
                    self.completion_time = now + ERASE_ALL_TIME;
                    self.state = State::WaitForCompletion;
                }
            }
            // PORT-NOTE: MAME lève une erreur fatale ; impossible avec 2 + n bits complets.
            Command::Invalid => self.state = State::InReset,
        }
    }

    fn execute_write_command(&mut self, now: Tick) {
        if self.locked {
            self.state = State::InReset;
            return;
        }
        let v = self.shift_register & self.word_mask();
        match self.command {
            Command::Write => {
                self.internal_write(self.address, v);
                self.completion_time = now + WRITE_TIME;
            }
            _ => {
                // WRITEALL : internal_write(a, internal_read(a) & data)
                for a in 0..1 << self.address_bits {
                    let old = self.internal_read(a);
                    self.internal_write(a, old & v);
                }
                self.completion_time = now + WRITE_ALL_TIME;
            }
        }
        self.state = State::WaitForCompletion;
    }
}

#[cfg(test)]
#[path = "eeprom_tests.rs"]
mod tests;
