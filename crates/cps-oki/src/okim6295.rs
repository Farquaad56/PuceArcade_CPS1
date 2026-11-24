// license:BSD-3-Clause
// copyright-holders: Mirko Buffoni, Aaron Giles
// Ported from ref/mame-sound/okim6295.cpp (lines 55-80, 136-361)

//! OKI MSM6295 ADPCM sound chip emulation. spec: docs/spec/03 §OKI

use crate::adpcm::OkiAdpcmState;

/// Number of voices on the MSM6295.
pub const OKIM6295_VOICES: usize = 4;

/// Volume lookup table numerators (× 0x20). spec: docs/spec/03 §OKI l.59-77
pub const VOLUME_NUM: [i32; 16] = [
    0x20, 0x16, 0x10, 0x0B, 0x08, 0x06, 0x04, 0x03, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// A single voice channel on the MSM6295.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Okim6295 {
    pub voices: [OkiVoice; 4],
    pub command: i32,
    pub pin7: bool,
    /// ROM ADPCM : non sérialisée (load_state la conserve).
    #[cfg_attr(feature = "serde", serde(skip))]
    pub rom: Vec<u8>,
    /// Débogueur : sortie de chaque voix au dernier échantillon (0 si muette).
    #[cfg_attr(feature = "serde", serde(skip))]
    pub voice_out: [i32; OKIM6295_VOICES],
    /// Débogueur : voix exclues de la somme (bit n = voix n ; 0 = son normal).
    #[cfg_attr(feature = "serde", serde(skip))]
    pub mute: u8,
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
            voice_out: [0; OKIM6295_VOICES],
            mute: 0,
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
        if self.pin7 {
            132
        } else {
            165
        }
    }

    /// Write to the command register. spec: docs/spec/03 §OKI l.235-313
    pub fn write(&mut self, command: u8) {
        // If a command is pending, process the second half
        if self.command != -1 {
            let mut voicemask = (command >> 4) as i32;

            for voicenum in 0..OKIM6295_VOICES {
                if (voicemask & 1) == 1 {
                    // Read ROM data before borrowing voice mutably
                    let base = (self.command * 8) as u32;

                    let start = ((self.read_byte(base) as u32) << 16)
                        | ((self.read_byte(base + 1) as u32) << 8)
                        | (self.read_byte(base + 2) as u32);
                    let start = start & 0x3FFFF;

                    let stop = ((self.read_byte(base + 3) as u32) << 16)
                        | ((self.read_byte(base + 4) as u32) << 8)
                        | (self.read_byte(base + 5) as u32);
                    let stop = stop & 0x3FFFF;

                    // Now borrow voice mutably
                    if !self.voices[voicenum].playing && start < stop {
                        self.voices[voicenum].playing = true;
                        self.voices[voicenum].base_offset = start;
                        self.voices[voicenum].sample = 0;
                        self.voices[voicenum].count = 2 * (stop - start + 1);
                        self.voices[voicenum].adpcm.reset();
                        let vol_idx = (command & 0x0F) as usize;
                        self.voices[voicenum].volume = VOLUME_NUM[vol_idx];
                    }
                }
                voicemask >>= 1;
            }

            // Reset the command
            self.command = -1;
        } else if (command & 0x80) == 0x80 {
            // Start of a command: remember sample number
            self.command = (command & 0x7F) as i32;
        } else {
            // Silence command: stop voices specified in bits 3-6
            let mut voicemask = (command >> 3) as i32;

            for voicenum in 0..OKIM6295_VOICES {
                if (voicemask & 1) == 1 {
                    self.voices[voicenum].playing = false;
                }
                voicemask >>= 1;
            }
        }
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

    /// Generate one sample: sum of all active voices. spec: docs/spec/03 §OKI l.178-183, l.338-361
    pub fn generate(&mut self) -> i32 {
        let mut total = 0;

        for voicenum in 0..OKIM6295_VOICES {
            self.voice_out[voicenum] = 0;
            if !self.voices[voicenum].playing {
                continue;
            }

            // Fetch the next sample byte, extract nibble (high first)
            let addr = self.voices[voicenum].base_offset + self.voices[voicenum].sample / 2;
            let byte = self.read_byte(addr);
            let shift = ((self.voices[voicenum].sample & 1) << 2) ^ 4;
            let nibble = (byte >> shift) & 0x0F;

            // Decode ADPCM and scale by volume
            let decoded = self.voices[voicenum].adpcm.clock(nibble);
            let out = decoded as i32 * self.voices[voicenum].volume;
            self.voice_out[voicenum] = out;
            if self.mute & (1 << voicenum) == 0 {
                total += out;
            }

            // Advance sample counter, stop when done
            if self.voices[voicenum].sample + 1 >= self.voices[voicenum].count {
                self.voices[voicenum].playing = false;
            } else {
                self.voices[voicenum].sample += 1;
            }
        }

        total
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
    fn test_write_start_voice() {
        // ROM with sample entry at index 1 (offset 8): start=0x400, stop=0x480
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00; // start high byte
        rom[9] = 0x04; // start mid byte (0x400)
        rom[10] = 0x00; // start low byte
        rom[11] = 0x00; // stop high byte
        rom[12] = 0x04; // stop mid byte (0x480)
        rom[13] = 0x80; // stop low byte

        let mut chip = Okim6295::new(rom);

        // Send start command: sample number 1, voice 0, volume 0
        chip.write(0x81); // bit7=1, sample num = 1
        chip.write(0x10); // voice mask = 1 (voice 0), volume = 0

        assert_eq!(chip.read(), 0xF1); // Voice 0 playing
        assert!(chip.voices[0].playing);
        assert_eq!(chip.voices[0].base_offset, 0x400);
        assert_eq!(chip.voices[0].count, 2 * (0x480 - 0x400 + 1)); // 258
    }

    #[test]
    fn test_write_volume() {
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00;
        rom[9] = 0x04;
        rom[10] = 0x00;
        rom[11] = 0x00;
        rom[12] = 0x04;
        rom[13] = 0x80;

        let mut chip = Okim6295::new(rom);

        // Volume index 8 -> VOLUME_NUM[8] = 0x02
        chip.write(0x81);
        chip.write(0x18); // voice 0, volume index 8

        assert_eq!(chip.voices[0].volume, 0x02);
    }

    #[test]
    fn test_write_stop_voice() {
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00;
        rom[9] = 0x04;
        rom[10] = 0x00;
        rom[11] = 0x00;
        rom[12] = 0x04;
        rom[13] = 0x80;

        let mut chip = Okim6295::new(rom);

        // Start voice 0
        chip.write(0x81);
        chip.write(0x10);
        assert!(chip.voices[0].playing);

        // Stop voice 0 (silence command, bit 3 = voice 0)
        chip.write(0x08);
        assert!(!chip.voices[0].playing);
    }

    #[test]
    fn test_write_ignored_on_busy_voice() {
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00;
        rom[9] = 0x04;
        rom[10] = 0x00;
        rom[11] = 0x00;
        rom[12] = 0x04;
        rom[13] = 0x80;

        let mut chip = Okim6295::new(rom);

        // Start voice 0
        chip.write(0x81);
        chip.write(0x10);
        assert!(chip.voices[0].playing);

        // Try to start again - should be ignored (voice busy)
        let original_sample = chip.voices[0].sample;
        chip.write(0x81);
        chip.write(0x10);
        assert_eq!(chip.voices[0].sample, original_sample); // Still at same position
    }

    #[test]
    fn test_write_ignored_start_ge_stop() {
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00;
        rom[9] = 0x05; // start = 0x500
        rom[10] = 0x00;
        rom[11] = 0x00;
        rom[12] = 0x04; // stop = 0x480 (start > stop)
        rom[13] = 0x80;

        let mut chip = Okim6295::new(rom);

        chip.write(0x81);
        chip.write(0x10);
        assert!(!chip.voices[0].playing); // Ignored because start >= stop
    }

    #[test]
    fn test_generate_stops_after_count() {
        let mut rom = vec![0u8; 0x40000];
        rom[8] = 0x00; // start high byte
        rom[9] = 0x04; // start mid byte (0x400)
        rom[10] = 0x00; // start low byte
        rom[11] = 0x00; // stop high byte
        rom[12] = 0x04; // stop mid byte (0x480)
        rom[13] = 0x80; // stop low byte

        let mut chip = Okim6295::new(rom);

        // Start voice 0 with count = 2 * (0x480 - 0x400 + 1) = 258
        chip.write(0x81);
        chip.write(0x10);

        assert_eq!(chip.read(), 0xF1); // Voice playing
        assert_eq!(chip.voices[0].count, 258);

        // Generate all samples - voice should stop after count is reached
        for _ in 0..258 {
            chip.generate();
        }

        assert!(!chip.voices[0].playing);
        assert_eq!(chip.read(), 0xF0); // Voice stopped
    }

    #[test]
    fn test_generate_no_voices() {
        let mut chip = Okim6295::new(vec![0u8; 0x40000]);
        assert_eq!(chip.generate(), 0);
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
