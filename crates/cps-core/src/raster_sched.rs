//! Ordonnancement des compteurs raster du CPS-B (ADR-003 étape b) : une fois par ligne,
//! raster_scanline ; IRQ4 (IPL2) après le délai rendu, en pixels. Sans raster : aucun effet.
//! spec: docs/spec/05 §Raster et interruptions ; source: ref/mame-cps1/cps1.cpp l.372-417

use crate::host68k::Host68k;
use cps_common::clock::{TICKS_PER_FRAME, TICKS_PER_LINE, TICKS_PER_PIXEL};

/// Traite les débuts de ligne atteints et déclenche l'IRQ4 programmée.
pub fn raster_events(host: &mut Host68k<'_>) {
    let Some(raster) = host.bus.video.raster.as_mut() else {
        return;
    };
    let now = *host.now;
    while now >= *host.next_line {
        let line = (*host.next_line % TICKS_PER_FRAME) / TICKS_PER_LINE;
        if let Some(d) = raster.raster_scanline(line as u32) {
            *host.raster_irq_at = Some(*host.next_line + u64::from(d) * TICKS_PER_PIXEL);
        }
        *host.next_line += TICKS_PER_LINE;
    }
    if host.raster_irq_at.is_some_and(|t| now >= t) {
        // IRQ4 ; le rendu partiel viendra en T851
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
}
