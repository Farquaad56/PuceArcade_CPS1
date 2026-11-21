//! cps-common — interfaces gelées pour l'émulateur CPS1 (ADR-001).
//! Voir docs/ARCHITECTURE.md.

#![forbid(unsafe_code)]

pub mod bits;
pub mod clock;
pub mod crc;
pub mod input;
pub mod video;

// Ré-exports publics (interfaces gelées)
pub use bits::combine_data;
pub use clock::*;
pub use crc::crc32_bytes;
pub use input::*;
pub use video::*;

#[cfg(test)]
mod tests {
    use super::*;

    // === Tests horloge (ADR-001) ===

    #[test]
    fn ticks_per_frame_is_line_times_lines() {
        assert_eq!(TICKS_PER_FRAME, TICKS_PER_LINE * LINES_PER_FRAME);
    }

    #[test]
    fn ticks_per_line_is_pixel_times_pixels() {
        assert_eq!(TICKS_PER_LINE, PIXELS_PER_LINE * TICKS_PER_PIXEL);
    }

    #[test]
    fn vblank_start_tick_is_correct() {
        assert_eq!(TICKS_VBLANK_START, VBLANK_START_LINE * TICKS_PER_LINE);
    }

    #[test]
    fn ym_sample_ticks_are_correct() {
        assert_eq!(TICKS_PER_YM_SAMPLE, YM_CLOCKS_PER_SAMPLE * TICKS_PER_YM);
    }

    #[test]
    fn ticks_per_68k_10mhz_gives_10_mhz() {
        assert_eq!(TICKS_PER_SECOND / TICKS_PER_68K_10MHZ, 10_000_000);
    }

    #[test]
    fn ticks_per_z80_remainder_is_correct() {
        // 3 579 545,45… Hz n'est pas entier → reste non nul
        assert_eq!(TICKS_PER_SECOND % TICKS_PER_Z80, 640);
    }

    // === Tests framebuffer ===

    #[test]
    fn framebuffer_size_is_correct() {
        let fb = Framebuffer::new();
        assert_eq!(fb.pixels.len(), SCREEN_W * SCREEN_H);
        assert_eq!(fb.pixels.len(), 86016);
    }

    #[test]
    fn framebuffer_pixels_are_zero() {
        let fb = Framebuffer::new();
        for &p in &fb.pixels {
            assert_eq!(p, 0);
        }
    }

    #[test]
    fn empty_framebuffer_crc_is_correct() {
        let fb = Framebuffer::new();
        assert_eq!(fb.crc32(), 0x61605322);
    }

    #[test]
    fn write_ppm_produces_correct_size() {
        use std::env;
        let tmp_dir = env::temp_dir();
        let path = tmp_dir.join("cps_common_test.ppm");
        let fb = Framebuffer::new();
        fb.write_ppm(&path).expect("write_ppm failed");

        let metadata = std::fs::metadata(&path).expect("metadata failed");
        // En-tête 15 octets + 384*224*3 pixels
        assert_eq!(
            metadata.len(),
            15 + (SCREEN_W as u64) * (SCREEN_H as u64) * 3
        );
        assert_eq!(metadata.len(), 258063);

        std::fs::remove_file(&path).ok();
    }

    // === Tests CRC-32 ===

    #[test]
    fn crc32_known_value() {
        assert_eq!(crc32_bytes(b"123456789"), 0xCBF43926);
    }

    // === Tests COMBINE_DATA ===

    #[test]
    fn combine_data_uds_only() {
        let result = combine_data(0x1234, 0xABCD, true, false);
        assert_eq!(result, 0xAB34);
    }

    #[test]
    fn combine_data_lds_only() {
        let result = combine_data(0x1234, 0xABCD, false, true);
        assert_eq!(result, 0x12CD);
    }

    #[test]
    fn combine_data_both() {
        let result = combine_data(0x1234, 0xABCD, true, true);
        assert_eq!(result, 0xABCD);
    }

    #[test]
    fn combine_data_neither() {
        let result = combine_data(0x1234, 0xABCD, false, false);
        assert_eq!(result, 0x1234);
    }

    // === Tests entrées ===

    #[test]
    fn input_state_default_is_all_ones() {
        let inp = InputState::default();
        assert_eq!(inp.in0, 0xFF);
        assert_eq!(inp.in1, 0xFFFF);
        assert_eq!(inp.in2, 0xFF);
        assert_eq!(inp.in3, 0xFF);
    }

    #[test]
    fn dips_default_is_all_ones() {
        let d = Dips::default();
        assert_eq!(d.dswa, 0xFF);
        assert_eq!(d.dswb, 0xFF);
        assert_eq!(d.dswc, 0xFF);
    }

    // === Tests constantes matérielles ===

    #[test]
    fn screen_dimensions_are_correct() {
        assert_eq!(SCREEN_W, 384);
        assert_eq!(SCREEN_H, 224);
    }

    #[test]
    fn visible_area_offsets_are_correct() {
        assert_eq!(VISIBLE_X0, 64);
        assert_eq!(VISIBLE_Y0, 16);
    }
}
