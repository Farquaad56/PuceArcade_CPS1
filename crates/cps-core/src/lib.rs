//! cps-core — Machine, ordonnanceur, interruptions, trame, audio.
//! Voir docs/ARCHITECTURE.md.
#![forbid(unsafe_code)]

pub mod host68k;
pub mod machine;
pub mod sched;
pub mod state;
pub mod vblank;

pub use machine::Machine;

// Re-exports for GUI convenience
pub use cps_cart::{load_game, Config, Game};
pub use cps_common::{Dips, Framebuffer, InputState};

/// Frame rate: 59.637 Hz (derived from 5.04 GHz / TICKS_PER_FRAME)
pub const FRAME_HZ: f64 = 5_040_000_000.0 / 84_510_720.0;

/// Audio sample rate: 55930.4 Hz (derived from 5.04 GHz / TICKS_PER_YM_SAMPLE)
pub const AUDIO_HZ: f64 = 5_040_000_000.0 / 90_112.0;

#[cfg(test)]
mod api_tests {
    use super::*;
    use crate::machine::testutil;

    fn make_machine() -> Machine {
        let game = testutil::synthetic_game(&[(0, &[0, 0, 0x80, 0, 0, 0, 4, 0])]);
        Machine::new(game)
    }

    #[test]
    fn frame_hz_constant() {
        assert!((FRAME_HZ - 59.637).abs() < 0.001);
    }

    #[test]
    fn audio_hz_constant() {
        assert!((AUDIO_HZ - 55930.4).abs() < 0.1);
    }

    #[test]
    fn audio_samples_after_one_frame() {
        let mut m = make_machine();
        m.run_frame();
        let audio = m.take_audio();
        // Expected: ~938 samples (TICKS_PER_FRAME / TICKS_PER_YM_SAMPLE)
        // 84510720 / 90112 = 937.8 -> rounds to 938
        assert!(
            audio.len() == 937 || audio.len() == 938,
            "got {} samples",
            audio.len()
        );
        // All values should be 0.0 (Z80 in HALT, YM silent)
        for sample in &audio {
            assert_eq!(sample[0], 0.0);
            assert_eq!(sample[1], 0.0);
        }
    }

    #[test]
    fn audio_samples_after_two_frames() {
        let mut m = make_machine();
        m.run_frame();
        let first = m.take_audio().len();
        m.run_frame();
        let second = m.take_audio().len();
        // Expected: ~1876 total (2 * TICKS_PER_FRAME / TICKS_PER_YM_SAMPLE)
        assert!(
            first + second == 1875 || first + second == 1876,
            "got {}+{}={}",
            first,
            second,
            first + second
        );
    }
}
