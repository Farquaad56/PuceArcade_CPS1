//! Stub sound chips for testing (no actual audio).
//! spec: docs/spec/01 §Z80

use crate::sound::SoundChips;
use cps_common::clock::Tick;

/// Null sound chips that record writes and provide configurable IRQ.
#[derive(Clone, Debug)]
pub struct NullChips {
    pub irq: bool,
    pub log: Vec<(u16, u8)>,
}

impl NullChips {
    pub fn new() -> Self {
        Self {
            irq: false,
            log: Vec::new(),
        }
    }
}

impl Default for NullChips {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundChips for NullChips {
    fn ym_read(&mut self, offset: u8) -> u8 {
        if offset == 0 {
            0xFF // YM2151 address port read returns 0xFF
        } else {
            0x00 // status
        }
    }

    fn ym_write(&mut self, offset: u8, data: u8) {
        let addr = if offset == 0 { 0xF000 } else { 0xF001 };
        self.log.push((addr, data));
    }

    fn oki_read(&mut self) -> u8 {
        0xF0 // OKI status (busy bit clear)
    }

    fn oki_write(&mut self, data: u8) {
        self.log.push((0xF002, data));
    }

    fn set_oki_pin7(&mut self, high: bool) {
        let val = if high { 1 } else { 0 };
        self.log.push((0xF006, val as u8));
    }

    fn irq(&self) -> bool {
        self.irq
    }

    fn advance(&mut self, _t: Tick) {}

    fn reset(&mut self) {
        self.irq = false;
        self.log.clear();
    }
}
