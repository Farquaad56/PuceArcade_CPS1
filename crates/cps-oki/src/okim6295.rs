// license:BSD-3-Clause
// copyright-holders: Mirko Buffoni, Aaron Giles
// Ported from ref/mame-sound/okim6295.cpp (lines 55-80, 136-361)

//! OKI MSM6295 ADPCM sound chip emulation. spec: docs/spec/03 §OKI

use crate::adpcm::OkiAdpcmState;

/// Number of voices on the MSM6295.
pub const OKIM6295_VOICES: usize = 4;

/// Volume lookup table numerators (× 0x20). spec: docs/spec/03 §OKI l.59-77
pub const VOLUME_NUM: [i32; 16] = [
    0x20, 0x16, 0x10, 0x0B, 0x08, 0x06, 0x04, 0x03,
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// A single voice channel on the MSM6295.
#[derive(Clone, Debug)]
pub struct OkiVoice {
    pub playing: bool,
    pub base_offset: u32,
    pub sample: u32,
    pub count: u32,
    pub volume: i32,
    pub adpcm: OkiAdpcmState,
}

impl OkiVoice {
    fn new() -> Self {
        Self {
            playing: false,
            base_offset: 0,
            sample: 0,
            count: 0,
            volume: 0,
            adpcm: OkiAdpcmState::new(),
        }
    }
}

/// OKI MSM6295 ADPCM sound chip.
#[derive(Clone, Debug)]
pub struct Okim6295 {
    pub voices: [OkiVoice; 4],
    pub command: i32,
    pub pin7: bool,
    pub rom: Vec<u8>,
}

impl Okim6295 {
    /// Create a new MSM6295 chip with the given ROM.
    pub fn new(rom: Vec<u8>) -> Self {
        let voices = core::array::from_fn(|_| OkiVoice::new());
        Self {
            voices,
            command: -1,
            pin7: true,
            rom,
        }
    }

    /// Reset the chip (stop all voices). spec: docs/spec/03 §OKI l.143-148
    pub fn reset(&mut self) {
        for v in self.voices.iter_mut() {
            v.playing = false;
        }
    }

    /// Read the status register. Returns 0xF0 | bits of active voices. spec: docs/spec/03 §OKI l.214-225
    pub fn read(&self) -> u8 {
        let mut result: u8 = 0xF0;
        for (i, v) in self.voices.iter().enumerate() {
            if v.playing {
                result |= 1 << i;
            }
        }
        result
    }

    /// Set pin 7 state (affects clock divisor). spec: docs/spec/03 §OKI l.202-207
    pub fn set_pin7(&mut self, high: bool) {
        self.pin7 = high;
    }

    /// Get the clock divisor based on pin 7 state. spec: docs/spec/03 §OKI l.120, l.168
    pub fn divisor(&self) -> u64 {
        if self.pin7 { 132 } else { 165 }
    }

    /// Read a byte from ROM with address masking. spec: docs/spec/03 §OKI l.256-264
    fn read_byte(&self, addr: u32) -> u8 {
        let masked = (addr & 0x3FFFF) as usize;
        if masked < self.rom.len() {
            self.rom[masked]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_defaults() {
        let chip = Okim6295::new(vec![0u8; 0x40000]);
        assert_eq!(chip.command, -1);
        assert!(chip.pin7);
        for v in chip.voices.iter() {
            assert!(!v.playing);
        }
    }

    #[test]
    fn test_reset_stops_voices() {
        let mut chip = Okim6295::new(vec![0u8; 0x40000]);
        chip.voices[0].playing = true;
        chip.voices[1].playing = true;
        chip.reset();
        for v in chip.voices.iter() {
            assert!(!v.playing);
        }
    }

    #[test]
    fn test_read_status_no_voices() {
        let chip = Okim6295::new(vec![0u8; 0x40000]);
        assert_eq!(chip.read(), 0xF0);
    }

    #[test]
    fn test_read_status_with_voices() {
        let mut chip = Okim6295::new(vec![0u8; 0x40000]);
        chip.voices[0].playing = true;
        assert_eq!(chip.read(), 0xF1);
        chip.voices[2].playing = true;
        assert_eq!(chip.read(), 0xF5);
    }

    #[test]
    fn test_divisor_pin7_high() {
        let mut chip = Okim6295::new(vec![0u8; 0x40000]);
        chip.set_pin7(true);
        assert_eq!(chip.divisor(), 132);
    }

    #[test]
    fn test_divisor_pin7_low() {
        let mut chip = Okim6295::new(vec![0u8; 0x40000]);
        chip.set_pin7(false);
        assert_eq!(chip.divisor(), 165);
    }

    #[test]
    fn test_read_byte_masking() {
        let mut rom = vec![0u8; 0x40000];
        rom[0x1234] = 0xAB;
        let chip = Okim6295::new(rom);
        assert_eq!(chip.read_byte(0x1234), 0xAB);
        // Test masking beyond ROM size
        assert_eq!(chip.read_byte(0x401234), 0xAB);
    }
}
