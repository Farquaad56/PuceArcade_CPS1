// license:BSD-3-Clause
// copyright-holders: Andrew Gardner, Aaron Giles
// Ported from ref/mame-sound/okiadpcm.cpp (lines 30-145)

//! OKI ADPCM state machine. spec: docs/spec/03 §OKI

use crate::tables::{DIFF_LOOKUP, INDEX_SHIFT};

/// Internal ADPCM state for OKI MSM6295-compatible chips.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OkiAdpcmState {
    pub signal: i32,
    pub step: i32,
    pub loop_signal: i32,
    pub loop_step: i32,
    pub saved: bool,
}

impl OkiAdpcmState {
    /// Create a new ADPCM state (tables are precomputed in tables.rs).
    pub fn new() -> Self {
        let mut s = Self {
            signal: 0,
            step: 0,
            loop_signal: 0,
            loop_step: 0,
            saved: false,
        };
        s.reset();
        s
    }

    /// Reset the ADPCM state.
    pub fn reset(&mut self) {
        // reset the signal/step
        self.signal = 0;
        self.loop_signal = 0;
        self.step = 0;
        self.loop_step = 0;
        self.saved = false;
    }

    /// Decode a single nibble and update ADPCM output.
    pub fn clock(&mut self, nibble: u8) -> i16 {
        // update the signal using precomputed table
        let idx = (self.step * 16 + (nibble & 0xF) as i32) as usize;
        self.signal += DIFF_LOOKUP[idx];

        // clamp to the maximum
        self.signal = self.signal.clamp(-2048, 2047);

        // adjust the step size and clamp
        let shift_idx = (nibble & 0x7) as usize;
        self.step += INDEX_SHIFT[shift_idx];
        self.step = self.step.clamp(0, 48);

        // return the signal
        self.signal as i16
    }

    /// Get current output.
    pub fn output(&self) -> i16 {
        self.signal as i16
    }

    /// Save current ADPCM state to buffer (for looping).
    pub fn save(&mut self) {
        if !self.saved {
            self.loop_signal = self.signal;
            self.loop_step = self.step;
            self.saved = true;
        }
    }

    /// Restore previous ADPCM state from buffer.
    pub fn restore(&mut self) {
        self.signal = self.loop_signal;
        self.step = self.loop_step;
    }
}

impl Default for OkiAdpcmState {
    fn default() -> Self {
        Self::new()
    }
}
