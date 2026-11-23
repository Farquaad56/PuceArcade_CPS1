//! CPS-A writes (palette copy), CPS-B read/write, sprite latch at VBLANK.
// license:BSD-3-Clause
// copyright-holders:Barry Rodowick, Nicola Salmoria, Aaron Giles
// source: ref/mame-cps1/cps1_v.cpp lines 2115-2244, 3054-3070

use crate::palette::{cps1_build_palette, PALETTE_ENTRIES};
use crate::regs::{Video, GFXRAM_WORDS, OBJ_BASE, OBJ_SIZE, PALETTE_ALIGN, PALETTE_BASE};

impl Video {
    /// CPS-A register write with COMBINE_DATA (l.2115-2134).
    /// Writing to PALETTE_BASE triggers immediate palette copy.
    pub fn cps_a_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        // COMBINE_DATA logic
        if uds && lds {
            self.regs_a[word] = data;
        } else if uds {
            let old = self.regs_a[word];
            self.regs_a[word] = (old & 0x00FF) | ((data >> 8) << 8);
        } else if lds {
            let old = self.regs_a[word];
            self.regs_a[word] = (old & 0xFF00) | (data & 0xFF);
        }

        // Writing PALETTE_BASE triggers immediate palette copy (l.2125-2126)
        if word == PALETTE_BASE {
            let base_word = self.cps1_base(PALETTE_BASE, PALETTE_ALIGN);
            let ctrl = self.regs_b[(self.cfg.cpsb.palette_control / 2) as usize];
            cps1_build_palette(
                &mut self.palette[..PALETTE_ENTRIES],
                &self.gfxram,
                base_word,
                ctrl,
            );
        }
    }

    /// CPS-B register read (l.2136-2181).
    /// Order: cpsb_addr check, multiplication result, IN2/IN3, default 0xFFFF.
    pub fn cps_b_read(&self, word: usize) -> u16 {
        let cfg = self.cfg;

        // 1. CPS-B specific address (l.2140-2141)
        if word as i32 == cfg.cpsb.cpsb_addr / 2 {
            return cfg.cpsb.cpsb_value as u16;
        }

        // 2. Multiplication result low/high (l.2146-2152)
        if cfg.cpsb.mult_factor1 >= 0 && cfg.cpsb.mult_factor2 >= 0 {
            let f1 = (cfg.cpsb.mult_factor1 / 2) as usize;
            let f2 = (cfg.cpsb.mult_factor2 / 2) as usize;

            if word as i32 == cfg.cpsb.mult_result_lo / 2 {
                let prod = (self.regs_b[f1] as u32) * (self.regs_b[f2] as u32);
                return prod as u16;
            }

            if word as i32 == cfg.cpsb.mult_result_hi / 2 {
                let prod = (self.regs_b[f1] as u32) * (self.regs_b[f2] as u32);
                return ((prod >> 16) & 0xFFFF) as u16;
            }
        }

        // 3. IN2/IN3 inputs (l.2155-2160)
        if cfg.in2_addr != 0 && word as i32 == cfg.in2_addr / 2 {
            return self.in2;
        }

        if cfg.in3_addr != 0 && word as i32 == cfg.in3_addr / 2 {
            return self.in3;
        }

        // 5. Default: 0xFFFF (l.2179)
        0xFFFF
    }

    /// CPS-B register write with COMBINE_DATA (l.2183-2244).
    pub fn cps_b_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        // COMBINE_DATA logic
        if uds && lds {
            self.regs_b[word] = data;
        } else if uds {
            let old = self.regs_b[word];
            self.regs_b[word] = (old & 0x00FF) | ((data >> 8) << 8);
        } else if lds {
            let old = self.regs_b[word];
            self.regs_b[word] = (old & 0xFF00) | (data & 0xFF);
        }
    }

    /// VBLANK: get video base and latch sprites (l.3054-3070).
    pub fn vblank(&mut self) {
        // screen_vblank_cps1: call get_video_base() on entry
        self.get_video_base();

        // CPS2 : seul cps2_objram_latch est branché (cps2.cpp l.1824-1825)
        if let Some(c2) = self.cps2.as_mut() {
            c2.objram_latch();
            return;
        }

        // cps1_objram_latch: copy obj RAM to buffered_obj
        let obj_base = self.cps1_base(OBJ_BASE, OBJ_SIZE);
        for i in 0..0x400 {
            let src = (obj_base + i) % GFXRAM_WORDS;
            self.buffered_obj[i] = self.gfxram[src];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cps_a_write_palette_copy() {
        let mut v = Video::new("ghouls");
        // Set palette control register (regs_b[0x18] for ghouls)
        v.regs_b[0x18] = 0x003F; // All pages enabled

        // Write a white color word to gfxram at the palette base location
        // After writing PALETTE_BASE=0x9140, cps1_base(5, 0x400) = 0xA000
        v.gfxram[0xA000] = 0xFFFF; // White

        // Trigger palette copy by writing to PALETTE_BASE
        v.cps_a_write(5, 0x9140, true, true);

        // Palette entry 0 should now be white (0xFFFFFF)
        assert_eq!(v.palette[0], 0xFFFFFF);
    }

    #[test]
    fn test_cps_b_read_ghouls() {
        let v = Video::new("ghouls");
        // ghouls: cpsb_addr=-1, -1/2=0 in C++ integer division
        assert_eq!(v.cps_b_read(0), 0x0000);

        // Non-matching address returns 0xFFFF
        assert_eq!(v.cps_b_read(5), 0xFFFF);
    }

    #[test]
    fn test_cps_b_read_sf2() {
        let mut v = Video::new("sf2");
        // sf2: cpsb_addr=0x32, 0x32/2=0x19
        assert_eq!(v.cps_b_read(0x19), 0x0401);

        // IN2 at address 0x36 (word 0x1B)
        v.set_inputs(0xFF, 0xFF);
        assert_eq!(v.cps_b_read(0x1B), 0x00FF);

        v.set_inputs(0xFE, 0xFF);
        assert_eq!(v.cps_b_read(0x1B), 0x00FE);
    }

    #[test]
    fn test_cps_b_multiplication_sf2ce() {
        let mut v = Video::new("sf2ce");
        // sf2ce: mult_factor1=0x00, mult_factor2=0x02, mult_result_lo=0x04, mult_result_hi=0x06

        // Write factors to regs_b[0] and regs_b[1]
        v.cps_b_write(0, 0x1234, true, true);
        v.cps_b_write(1, 0x0100, true, true);

        // Read multiplication result: 0x1234 * 0x0100 = 0x123400
        assert_eq!(v.cps_b_read(2), 0x3400); // Low 16 bits
        assert_eq!(v.cps_b_read(3), 0x0012); // High 16 bits

        // cpsb_addr=0x32, so word 0x19 should return -1 as u16 = 0xFFFF
        assert_eq!(v.cps_b_read(0x19), 0xFFFF);
    }

    #[test]
    fn test_vblank_sprite_latch() {
        let mut v = Video::new("ghouls");
        // Write sprite data to gfxram at obj base location
        let obj_base = v.cps1_base(OBJ_BASE, OBJ_SIZE);
        v.gfxram[obj_base] = 0x1111;
        v.gfxram[(obj_base + 0x3FF) % GFXRAM_WORDS] = 0x2222;

        // Trigger VBLANK to latch sprites
        v.vblank();

        // Buffered obj should now contain the latched data
        assert_eq!(v.buffered_obj[0], 0x1111);
        assert_eq!(v.buffered_obj[0x3FF], 0x2222);
    }
}
