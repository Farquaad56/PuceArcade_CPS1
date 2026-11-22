//! VBLANK event: frame rendering, IRQ2 assertion, video bases update, sprite latch (MAME order).
//! spec: docs/spec/01 §Interruptions, docs/spec/02 §Instant du rendu

use crate::host68k::Host68k;
use cps_common::clock::TICKS_PER_FRAME;

/// Execute the VBLANK event sequence (Q-VBL).
/// Called by on_tick when now >= next_vblank.
pub fn vblank_event(host: &mut Host68k<'_>) {
    // Ordre de MAME (screen.cpp vblank_begin) : la mise à jour d'écran (update_if_primary) passe
    // AVANT les callbacks de VBLANK (IRQ2 de set_vblank_int, puis screen_vblank_cps1 et
    // cps1_objram_latch). Les sprites affichés sont donc ceux copiés au VBLANK précédent :
    // une trame de retard sur les tilemaps, comme sur la borne (Q-VBL, ADR-004).

    // 1. Rendu de la trame (render_frame recalcule lui-même les bases, comme screen_update_cps1)
    //    frame_number = trames déjà rendues (avant incrément)
    host.bus
        .video
        .render_frame(host.gfx, host.stars, *host.frame, host.fb);

    // 2. IRQ2 (IPL1) au début du VBLANK
    *host.ipl1 = true;

    // 3. screen_vblank_cps1 (bases vidéo) puis cps1_objram_latch (copie des sprites)
    host.bus.video.vblank();

    // 4. Compteur de trames et VBLANK suivant
    *host.frame += 1;
    *host.next_vblank += TICKS_PER_FRAME;
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};

    /// Test 1: IRQ handling - program increments counter on each VBLANK interrupt.
    #[test]
    fn irq_vblank_counter() {
        // Program: SSP=0x00FF8000, PC=0x400
        // Vector 26 (IRQ level 2) at 0x68 = 0x00000500
        // At 0x400: MOVE #$2000,SR (enable IRQs), BRA.S *
        // At 0x500: ADDQ.W #1,$FF0000, RTE
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]), // SSP vector
            (0x000004, &[0x00, 0x00, 0x04, 0x00]), // PC vector = 0x400
            (0x68, &[0x00, 0x00, 0x05, 0x00]),     // Vector 26 = 0x500
            // At 0x400: MOVE #$2000,SR then BRA.S *
            (0x400, &[0x46, 0xFC, 0x20, 0x00, 0x60, 0xFE]),
            // At 0x500: ADDQ.W #1,$FF0000 then RTE
            (0x500, &[0x52, 0x79, 0x00, 0xFF, 0x00, 0x00, 0x4E, 0x73]),
        ];

        let game = testutil::synthetic_game(&program);
        let mut m = Machine::new(game);

        // Run 3 frames
        for _ in 0..3 {
            m.run_frame();
        }

        // Counter at 0xFF0000 should be 3 (one increment per frame)
        let counter = m.bus.read16(0xFF0000, true, true);
        assert_eq!(counter, 3, "IRQ counter should be 3 after 3 frames");

        // IPL1 should be false (interrupt acknowledged via RTE)
        assert!(
            !m.ipl1,
            "IPL1 should be cleared after interrupt acknowledgment"
        );
    }

    /// Test 2: IRQs masked - counter stays at 0, IPL1 remains asserted.
    #[test]
    fn irq_masked_no_interrupt() {
        // Same as test 1 but with MOVE #$2700,SR (IRQs masked)
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]), // SSP vector
            (0x000004, &[0x00, 0x00, 0x04, 0x00]), // PC vector = 0x400
            (0x68, &[0x00, 0x00, 0x05, 0x00]),     // Vector 26 = 0x500
            // At 0x400: MOVE #$2700,SR (mask IRQs) then BRA.S *
            (0x400, &[0x46, 0xFC, 0x27, 0x00, 0x60, 0xFE]),
            // At 0x500: ADDQ.W #1,$FF0000 then RTE
            (0x500, &[0x52, 0x79, 0x00, 0xFF, 0x00, 0x00, 0x4E, 0x73]),
        ];

        let game = testutil::synthetic_game(&program);
        let mut m = Machine::new(game);

        // Run 2 frames
        for _ in 0..2 {
            m.run_frame();
        }

        // Counter should still be 0 (IRQs masked)
        let counter = m.bus.read16(0xFF0000, true, true);
        assert_eq!(counter, 0, "Counter should be 0 with IRQs masked");

        // IPL1 should remain asserted (never acknowledged)
        assert!(m.ipl1, "IPL1 should remain asserted when IRQs are masked");
    }

    /// Test 3: Frame rendering - palette write and pixel color check.
    #[test]
    fn frame_rendering_palette() {
        // Program writes to GFXRAM, sets palette control, triggers palette copy
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]), // SSP vector
            (0x000004, &[0x00, 0x00, 0x04, 0x00]), // PC vector = 0x400
            // At 0x400: Write to GFXRAM, set palette control, trigger copy, then loop
            (
                0x400,
                &[
                    // MOVE.W #$F800,$9157FE (GFXRAM write)
                    0x33, 0xFC, 0xF8, 0x00, 0x00, 0x91, 0x57, 0xFE,
                    // MOVE.W #$003F,$00800170 (CPS-B palette_control)
                    0x33, 0xFC, 0x00, 0x3F, 0x00, 0x80, 0x01, 0x70,
                    // MOVE.W #$9140,$0080010A (CPS-A PALETTE_BASE - triggers copy)
                    0x33, 0xFC, 0x91, 0x40, 0x00, 0x80, 0x01, 0x0A,
                    // BRA.S * (loop forever)
                    0x60, 0xFE,
                ],
            ),
        ];

        let game = testutil::synthetic_game(&program);
        let mut m = Machine::new(game);

        // Run one frame to execute the program and render
        m.run_frame();

        // Check pixel at (0,0) - should be red from palette entry 0xFFFF -> (136,0,0)
        // In framebuffer format 0x00RRGGBB, this is 0x880000
        let fb = m.framebuffer();
        assert_eq!(
            fb.pixels[0], 0x880000,
            "Pixel (0,0) should be red from palette"
        );
    }
}
