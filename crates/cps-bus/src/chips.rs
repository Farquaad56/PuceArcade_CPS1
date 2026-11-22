//! CPS1 sound chips: YM2151 (FM) + OKI MSM6295 (ADPCM speech).
//! spec: docs/spec/03 §Chaîne son §Mixage hôte

use cps_common::clock::{OKI_DIVISOR_PIN7_HIGH, TICKS_PER_OKI, TICKS_PER_YM};
use cps_oki::okim6295::Okim6295;
use cps_ym2151::clocked::ClockedYm2151;

/// Combined YM2151 + OKI MSM6295 sound chips with mixing.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Chips {
    pub ym: ClockedYm2151,
    pub oki: Okim6295,
    /// Next tick at which an OKI sample is due.
    pub oki_next: u64,
    /// Last generated OKI sample value (held for mixing).
    pub oki_current: i32,
    /// Mixed output samples accumulated since last take_samples().
    pub out: Vec<f32>,
}

impl Chips {
    /// Create new sound chips with the given OKI ROM.
    pub fn new(oki_rom: Vec<u8>) -> Self {
        let ym = ClockedYm2151::new();
        let oki = Okim6295::new(oki_rom);
        // First OKI sample due at one period (pin7 high by default).
        let oki_next = OKI_DIVISOR_PIN7_HIGH * TICKS_PER_OKI;
        Self {
            ym,
            oki,
            oki_next,
            oki_current: 0,
            out: Vec::new(),
        }
    }

    /// Take all mixed samples generated since the last call.
    pub fn take_samples(&mut self) -> Vec<f32> {
        std::mem::take(&mut self.out)
    }
}

impl crate::sound::SoundChips for Chips {
    fn ym_read(&mut self, offset: u8) -> u8 {
        self.ym.read(offset & 1)
    }

    fn ym_write(&mut self, offset: u8, data: u8) {
        self.ym.write(offset & 1, data);
    }

    fn oki_read(&mut self) -> u8 {
        self.oki.read()
    }

    fn oki_write(&mut self, data: u8) {
        self.oki.write(data);
    }

    fn set_oki_pin7(&mut self, high: bool) {
        // Period changes for future samples; oki_next unchanged.
        self.oki.set_pin7(high);
    }

    fn irq(&self) -> bool {
        self.ym.irq()
    }

    fn advance(&mut self, t: u64) {
        // Chronological merge of YM and OKI sample generation.
        loop {
            let ty = self.ym.next_sample_clock() * TICKS_PER_YM;
            let to = self.oki_next;

            if std::cmp::min(ty, to) > t {
                break;
            }

            if to <= ty {
                // OKI sample due first (or simultaneous).
                self.oki_current = self.oki.generate();
                self.oki_next += self.oki.divisor() * TICKS_PER_OKI;
            } else {
                // YM sample due first.
                let ym_clocks = ty / TICKS_PER_YM;
                self.ym.advance_to(ym_clocks);
                let samples = self.ym.take_samples();
                for s in samples {
                    let mixed = (s[0] + s[1]) as f32 / 32768.0 * 0.35
                        + self.oki_current as f32 / 65536.0 * 0.30;
                    self.out.push(mixed);
                }
            }
        }

        // Advance YM timers only (no more samples within this interval).
        let ym_clocks = t / TICKS_PER_YM;
        if ym_clocks > self.ym.time() {
            self.ym.advance_to(ym_clocks);
        }
    }

    fn reset(&mut self) {
        self.ym.reset();
        self.oki.reset();
        self.oki_next = OKI_DIVISOR_PIN7_HIGH * TICKS_PER_OKI;
        self.oki_current = 0;
        self.out.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sound::{SoundBoard, SoundChips};
    use cps_common::clock::OKI_DIVISOR_PIN7_LOW;
    use puce_z80::Cpu;

    fn make_chips() -> Chips {
        // ROM with one sample entry at index 1 (offset 8): start=0x400, stop=0x480.
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00; // start high byte
        rom[9] = 0x04; // start mid byte (0x400)
        rom[10] = 0x00; // start low byte
        rom[11] = 0x00; // stop high byte
        rom[12] = 0x04; // stop mid byte (0x480)
        rom[13] = 0x80; // stop low byte
        Chips::new(rom)
    }

    #[test]
    fn silence_produces_ten_samples() {
        let mut chips = make_chips();
        // 10 YM sample periods (90_112 ticks each). No OKI activity.
        chips.advance(cps_common::clock::TICKS_PER_YM_SAMPLE * 10);
        let samples = chips.take_samples();
        assert_eq!(samples.len(), 10);
        for s in &samples {
            assert_eq!(*s, 0.0);
        }
    }

    #[test]
    fn oki_only_produces_nonzero() {
        let mut chips = make_chips();
        // Start OKI voice 0 with sample 1, volume index 0 (max).
        chips.oki_write(0x81);
        chips.oki_write(0x10);

        // Advance past 3 OKI sample periods.
        let period = OKI_DIVISOR_PIN7_HIGH * TICKS_PER_OKI;
        chips.advance(period * 3);

        // oki_current should be the third generated value (nonzero).
        assert_ne!(chips.oki_current, 0);
        let samples = chips.take_samples();
        // YM produces no sound, but OKI mixing makes values nonzero.
        assert!(samples.iter().any(|s| *s != 0.0));
    }

    #[test]
    fn z80_program_triggers_irq() {
        let mut board = SoundBoard::new(vec![0x00; 0x18000], make_chips());

        // Program: set Timer A to period 1 (value 1023), enable it, wait for IRQ.
        // LD A,0x10 ; LD (F000),A   -> address reg 0x10 (timer A low)
        // LD A,0xFF ; LD (F001),A   -> data 0xFF
        // LD A,0x11 ; LD (F000),A   -> address reg 0x11 (timer A high)
        // LD A,0x03 ; LD (F001),A   -> data 0x03 (value = 1023)
        // LD A,0x14 ; LD (F000),A   -> address reg 0x14 (mode)
        // LD A,0x05 ; LD (F001),A   -> data: load+enable timer A
        // IM 1 ; EI ; HALT          -> wait for interrupt
        let prog = [
            0x3E, 0x10, 0x32, 0x00, 0xF0, 0x3E, 0xFF, 0x32, 0x01, 0xF0, 0x3E, 0x11, 0x32, 0x00,
            0xF0, 0x3E, 0x03, 0x32, 0x01, 0xF0, 0x3E, 0x14, 0x32, 0x00, 0xF0, 0x3E, 0x05, 0x32,
            0x01, 0xF0, 0xED, 0x56, 0xFB, 0x76,
        ];
        board.rom[0..prog.len()].copy_from_slice(&prog);

        // ISR at 0x38: LD A,0x77 ; LD (D000),A ; HALT
        let isr = [0x3E, 0x77, 0x32, 0x00, 0xD0, 0x76];
        board.rom[0x38..0x38 + isr.len()].copy_from_slice(&isr);

        let mut cpu = Cpu::new();
        // Run enough ticks for timer to fire and ISR to execute.
        for _ in 0..2000 {
            board.tick(&mut cpu);
        }

        assert_eq!(board.ram[0], 0x77);
    }

    #[test]
    fn z80_program_no_irq_too_early() {
        let mut board = SoundBoard::new(vec![0x00; 0x18000], make_chips());

        // Same program as above.
        let prog = [
            0x3E, 0x10, 0x32, 0x00, 0xF0, 0x3E, 0xFF, 0x32, 0x01, 0xF0, 0x3E, 0x11, 0x32, 0x00,
            0xF0, 0x3E, 0x03, 0x32, 0x01, 0xF0, 0x3E, 0x14, 0x32, 0x00, 0xF0, 0x3E, 0x05, 0x32,
            0x01, 0xF0, 0xED, 0x56, 0xFB, 0x76,
        ];
        board.rom[0..prog.len()].copy_from_slice(&prog);

        let isr = [0x3E, 0x77, 0x32, 0x00, 0xD0, 0x76];
        board.rom[0x38..0x38 + isr.len()].copy_from_slice(&isr);

        let mut cpu = Cpu::new();
        // Run only a few ticks — not enough for timer to fire.
        for _ in 0..100 {
            board.tick(&mut cpu);
        }

        assert_eq!(board.ram[0], 0);
    }

    #[test]
    fn oki_pin7_low_changes_period() {
        let mut chips = make_chips();
        // Start OKI voice.
        chips.oki_write(0x81);
        chips.oki_write(0x10);

        let period_high = OKI_DIVISOR_PIN7_HIGH * TICKS_PER_OKI;
        let period_low = OKI_DIVISOR_PIN7_LOW * TICKS_PER_OKI;

        // Advance past one high-period sample.
        chips.advance(period_high);
        // One OKI sample generated at t=period_high, oki_next now 2*period_high.

        // Switch to pin7 low (slower clock -> longer period).
        chips.set_oki_pin7(false);

        // Advance past one more sample at the new slower rate.
        // The next OKI sample is due at t=2*period_high, which is within this interval.
        chips.advance(period_low * 2);

        // After first advance: oki_next = 2*period_high (one sample generated)
        // After second advance: one more sample at t=2*period_high, oki_next becomes
        //   2*period_high + period_low. The next sample would be at 3*period_high+period_low,
        //   which is beyond the advance interval (665280 + 1663200 = 2328480).
        assert_eq!(chips.oki_next, 2 * period_high + period_low);
    }
}
