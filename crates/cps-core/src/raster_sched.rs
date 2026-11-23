//! Ordonnancement des compteurs raster du CPS-B (ADR-003 étape b) : une fois par ligne,
//! raster_scanline ; IRQ4 (IPL2) après le délai rendu, en pixels. Sans raster : aucun effet.
//! spec: docs/spec/05 §Raster et interruptions ; source: ref/mame-cps1/cps1.cpp l.372-417

use crate::host68k::Host68k;
use cps_common::clock::{TICKS_PER_FRAME, TICKS_PER_LINE, TICKS_PER_PIXEL};

/// Traite les débuts de ligne atteints et déclenche l'IRQ4 programmée.
pub fn raster_events(host: &mut Host68k<'_>) {
    if host.bus.video.raster.is_none() {
        return;
    }
    let now = *host.now;
    while now >= *host.next_line {
        let line = (*host.next_line % TICKS_PER_FRAME) / TICKS_PER_LINE;
        if line == 0 {
            // début de trame : rendu partiel (ADR-003 c)
            host.bus.video.begin_frame();
        }
        let raster = host.bus.video.raster.as_mut().expect("raster");
        if let Some(d) = raster.raster_scanline(line as u32) {
            *host.raster_irq_at = Some(*host.next_line + u64::from(d) * TICKS_PER_PIXEL);
        }
        *host.next_line += TICKS_PER_LINE;
    }
    if host.raster_irq_at.is_some_and(|t| now >= t) {
        // IRQ4 : update_partial(vpos) puis IPL2 (ADR-003 c)
        let vpos = ((now % TICKS_PER_FRAME) / TICKS_PER_LINE) as usize;
        host.bus
            .video
            .render_lines(host.gfx, host.stars, *host.frame, vpos);
        *host.ipl2 = true;
        *host.raster_irq_at = None;
    }
}

impl Host68k<'_> {
    /// m_screen->hpos() avant un accès CPS-B (seulement avec raster).
    pub(crate) fn update_hpos(&mut self) {
        if self.bus.video.raster.is_some() {
            self.bus.video.hpos = ((*self.now % TICKS_PER_LINE) / TICKS_PER_PIXEL) as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};
    use cps_common::clock::{TICKS_PER_LINE, TICKS_PER_PIXEL};
    use puce_m68k::bus::Bus;

    fn machine(raster: bool) -> Machine {
        let g = testutil::synthetic_game(&[(0, &[0, 0, 0x80, 0, 0, 0, 4, 0])]);
        let mut m = Machine::new(g);
        if raster {
            let mut r = cps_gfx::raster::Raster::new();
            r.reload[0] = 10;
            m.bus.video.raster = Some(r);
        }
        m
    }

    /// Un appel de on_tick ; rend (now, ipl2, counter[0]).
    fn step(m: &mut Machine) -> (u64, bool, u16) {
        let (_, mut h) = m.split();
        h.on_tick(puce_m68k::pins::Pins::default());
        let c = h.bus.video.raster.as_ref().map_or(0, |r| r.counter[0]);
        (*h.now, *h.ipl2, c)
    }

    #[test]
    fn irq4_at_line_10() {
        let mut m = machine(true);
        let target = 10 * TICKS_PER_LINE;
        let mut seen_reload = false;
        loop {
            let (now, ipl2, c) = step(&mut m);
            if now < TICKS_PER_LINE {
                seen_reload |= c == 10;
            }
            if now < target {
                assert!(!ipl2, "pas avant la ligne 10 (now = {now})");
                if (5 * TICKS_PER_LINE..6 * TICKS_PER_LINE).contains(&now) {
                    assert_eq!(c, 5, "lignes traitées dans l'ordre");
                }
            } else {
                assert!(ipl2);
                assert_eq!(c, 0);
                assert!(now - target < 504);
                break;
            }
        }
        assert!(seen_reload, "compteur 0 rechargé à 10 en ligne 0");
    }

    #[test]
    fn hpos_before_cps_b_read() {
        let mut m = machine(true);
        let (_, mut h) = m.split();
        *h.now = 3 * TICKS_PER_LINE + 123_456;
        h.read(0x800150, 5, true, true);
        assert_eq!(h.bus.video.hpos, (123_456 / TICKS_PER_PIXEL) as u32);
    }

    #[test]
    fn cps1_without_raster() {
        let mut m = machine(false);
        for _ in 0..20_000 {
            assert!(!step(&mut m).1);
        }
        assert_eq!(m.next_line, 0);
    }

    /// Machine CPS1 synthétique avec raster : l'IRQ4 de la ligne 100 passe le scroll X du
    /// scroll1 à 0x37, l'IRQ2 du VBLANK le remet à 0x10.
    fn split_scroll_machine() -> Machine {
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]),
            (0x000004, &[0x00, 0x00, 0x04, 0x00]),
            (0x68, &[0x00, 0x00, 0x06, 0x00]), // IRQ2 -> 0x600
            (0x70, &[0x00, 0x00, 0x05, 0x00]), // IRQ4 -> 0x500
            (0x400, &[0x46, 0xFC, 0x20, 0x00, 0x60, 0xFE]), // MOVE #$2000,SR ; BRA.S *
            // MOVE.W #$0037,$80010C ; RTE
            (
                0x500,
                &[0x33, 0xFC, 0x00, 0x37, 0x00, 0x80, 0x01, 0x0C, 0x4E, 0x73],
            ),
            // MOVE.W #$0010,$80010C ; RTE
            (
                0x600,
                &[0x33, 0xFC, 0x00, 0x10, 0x00, 0x80, 0x01, 0x0C, 0x4E, 0x73],
            ),
        ];
        let mut m = Machine::new(testutil::synthetic_game(&program));
        let mut seed = 0x9E37_79B9u32;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            seed
        };
        m.gfx = (0..0x40000).map(|_| rnd() as u8).collect();
        let v = &mut m.bus.video;
        for w in v.gfxram.iter_mut() {
            *w = rnd() as u16;
        }
        for (i, p) in v.palette.iter_mut().enumerate() {
            *p = i as u32;
        }
        let lc = (v.cfg.cpsb.layer_control / 2) as usize;
        // scroll1 dans les 4 emplacements : ni sprites (couche 0) ni autre tilemap
        v.regs_b[lc] = 0x1540 | v.cfg.cpsb.layer_enable_mask[0] as u16;
        v.regs_a[cps_gfx::regs::SCROLL1_X] = 0x10;
        let mut r = cps_gfx::raster::Raster::new();
        r.reload[0] = 100;
        v.raster = Some(r);
        m
    }

    #[test]
    fn raster_irq_renders_partial_frame() {
        use cps_common::video::{Framebuffer, SCREEN_W};
        let mut m = split_scroll_machine();
        for _ in 0..3 {
            m.run_frame();
        }
        // références : trame entière avec chacun des deux scrolls
        let reference = |scroll: u16| {
            let mut v = m.bus.video.clone();
            v.regs_a[cps_gfx::regs::SCROLL1_X] = scroll;
            let mut fb = Framebuffer::new();
            v.render_frame(&m.gfx, None, 0, &mut fb);
            fb.pixels
        };
        let (a, b) = (reference(0x10), reference(0x37));
        let fb = &m.framebuffer().pixels;
        let split = (100 - cps_gfx::sprites::CLIP.2) * SCREEN_W;
        assert_ne!(a[..split], b[..split]);
        assert_eq!(
            fb[..split],
            a[..split],
            "lignes < 100 : scroll de la trame (0x10)"
        );
        assert_eq!(
            fb[split..],
            b[split..],
            "lignes >= 100 : scroll de l'IRQ4 (0x37)"
        );
    }
}
