// license:BSD-3-Clause
// copyright-holders:Barry Rodewald, Nicola Salmoria
// source: ref/mame-cps1/cps1_v.cpp lines 2604-2646

pub const PALETTE_ENTRIES: usize = 0xC00;
pub const BACKGROUND_PEN: usize = 0xBFF;

/// Convert a 16-bit palette word to an RGB color (0x00RRGGBB).
/// Formula from docs/spec/02-video-cps-a-b.md §Palette.
pub fn color_from_word(p: u16) -> u32 {
    let bright = 0x0F + ((p >> 12) << 1);
    let r = (((p >> 8) & 0xF) * 0x11 * bright / 0x2D) as u32;
    let g = (((p >> 4) & 0xF) * 0x11 * bright / 0x2D) as u32;
    let b = ((p & 0xF) * 0x11 * bright / 0x2D) as u32;
    (r << 16) | (g << 8) | b
}

/// Build the palette from gfxram. Port of cps1_build_palette (l.2612-2646).
/// `base_word` is the starting word index in gfxram.
/// `ctrl` determines which pages are copied (bit N = page N).
pub fn cps1_build_palette(palette: &mut [u32], gfxram: &[u16], base_word: usize, ctrl: u16) {
    let mut src = base_word;
    for page in 0..6 {
        if (ctrl >> page) & 1 != 0 {
            // Copy this page
            for i in 0..0x200 {
                let idx = (src + i) % gfxram.len();
                palette[0x200 * page + i] = color_from_word(gfxram[idx]);
            }
            src += 0x200;
        } else if src != base_word {
            // Skip this page in source, but only if we've already copied at least one page
            src += 0x200;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_word() {
        assert_eq!(color_from_word(0xFFFF), 0xFFFFFF);
        assert_eq!(color_from_word(0x0FFF), 0x555555);
        assert_eq!(color_from_word(0xF800), 0x880000);
        assert_eq!(color_from_word(0x1F00), 0x600000);
        assert_eq!(color_from_word(0x0000), 0);
        assert_eq!(color_from_word(0xF0F0), 0x00FF00);
    }

    #[test]
    fn test_build_palette_basic() {
        let mut palette = [0u32; PALETTE_ENTRIES];
        // gfxram with page markers: each word encodes its source page in bits 12-15
        let mut gfxram = vec![0u16; 0x1000];
        for (i, w) in gfxram.iter_mut().enumerate() {
            *w = 0xF000 | ((i >> 9) as u16); // page source = i/0x200
        }

        // Copy pages 0 and 2 only (ctrl bits 0 and 1 set)
        cps1_build_palette(&mut palette, &gfxram, 0, 0b000101);

        // Page 0 copied from gfxram[0..0x200]
        assert_eq!(palette[0], color_from_word(0xF000));
        // Page 1 skipped (not in ctrl) - should be unchanged (0)
        assert_eq!(palette[0x200], 0);
        // Page 2 copied from gfxram[0x400..0x600] (after skipping page 1's source)
        assert_eq!(palette[0x400], color_from_word(0xF002));
    }

    #[test]
    fn test_build_palette_first_page_skipped() {
        let mut palette = [0u32; PALETTE_ENTRIES];
        let mut gfxram = vec![0u16; 0x1000];
        for (i, w) in gfxram.iter_mut().enumerate() {
            *w = 0xF000 | ((i >> 9) as u16);
        }

        // Only page 2 enabled - no pages copied before it, so no source skip
        cps1_build_palette(&mut palette, &gfxram, 0, 0b000100);

        assert_eq!(palette[0x400], color_from_word(0xF000));
    }

    #[test]
    fn test_build_palette_wraparound() {
        let mut palette = [0u32; PALETTE_ENTRIES];
        // Small gfxram to force wraparound
        let mut gfxram = vec![0u16; 0x1000];
        for (i, w) in gfxram.iter_mut().enumerate() {
            *w = 0xF000 | ((i >> 9) as u16);
        }

        // Start near the end, copy one page - should wrap around
        cps1_build_palette(&mut palette, &gfxram, 0xF00, 1);

        // First word of copied page comes from gfxram[0xF00] (page source = 7)
        assert_eq!(palette[0], color_from_word(0xF007));
    }
}
