//! Compteurs raster du CPS-B-21 (IRQ4) — état seul, non intégré (ADR-003).
// license:BSD-3-Clause
// copyright-holders:Paul Leaman
// source: ref/mame-cps1/cps1.cpp l.372-405 (raster_scanline, raster_irq),
//         ref/mame-cps1/cps1_v.cpp l.2162-2203 (cps1_cps_b_r / cps1_cps_b_w, partie raster),
//         ref/mame-cps1/cps1.cpp l.14861-14869 (init_rasters)

/// État des compteurs : présent seulement pour les jeux qui appellent `init_rasters` (ganbare, CPS2).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Raster {
    pub counter: [u16; 3],
    pub reload: [u16; 3],
}

impl Default for Raster {
    fn default() -> Self {
        Self::new()
    }
}

impl Raster {
    /// init_rasters (l.14861) : rechargements 0 et 1 à 0x1FF.
    pub fn new() -> Self {
        Raster {
            counter: [0; 3],
            reload: [0x1ff, 0x1ff, 0],
        }
    }

    /// raster_scanline (l.372) : à appeler au début de chaque ligne.
    /// Retourne le délai de l'IRQ4 en pixels (`counter[2] * 2`) si un compteur atteint 0 ;
    /// l'IRQ1 de VBLANK (ligne 240) reste gérée par l'ordonnanceur.
    pub fn raster_scanline(&mut self, scanline: u32) -> Option<u32> {
        let mut raster_irq_pending = 0;
        for i in 0..2 {
            if scanline == 0 {
                self.counter[i] = self.reload[i];
            } else {
                self.counter[i] = self.counter[i].wrapping_sub(1) & 0x1ff;
            }
            if self.counter[i] == 0 {
                raster_irq_pending |= 1 << i;
            }
        }
        self.counter[2] = self.reload[2];
        // PORT-NOTE: m_raster_irq->adjust(counter[2] * 2 * pixel_period) -> délai rendu à l'appelant.
        (raster_irq_pending != 0).then(|| u32::from(self.counter[2]) * 2)
    }

    /// Lecture CPS-B (l.2165-2174) : Some pour les registres raster 0x0E, 0x10, 0x12.
    /// `hpos` = position horizontale de l'écran ; `reg_0e` = m_cps_b_regs[0x0e / 2].
    pub fn read(&self, offset: usize, hpos: u32, reg_0e: u16) -> Option<u16> {
        if offset == 0x0e / 2 {
            // 2-pixel hpos relative to raster counter #3
            let v = (self.counter[2].wrapping_sub((hpos / 2) as u16) << 1) & 0x1fe;
            return Some(v | (reg_0e & 1));
        }
        if offset == 0x10 / 2 || offset == 0x12 / 2 {
            // scanline relative to raster counter #1, #2
            return Some(self.counter[offset & 1] & 0x1ff);
        }
        None
    }

    /// Écriture CPS-B (l.2190-2202), `data` après COMBINE_DATA. Retourne true si traitée.
    pub fn write(&mut self, offset: usize, data: u16) -> bool {
        if offset == 0x0e / 2 {
            self.reload[2] = (data >> 1) & 0xff;
            return true;
        }
        if offset == 0x10 / 2 || offset == 0x12 / 2 {
            self.reload[offset & 1] = data & 0x1ff;
            // manually reload counter
            if data & 0x8000 != 0 {
                self.counter[offset & 1] = self.reload[offset & 1];
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irq_at_line_10() {
        let mut r = Raster::new();
        assert!(r.write(0x10 / 2, 10));
        let lines: Vec<u32> = (0..262)
            .filter(|&l| r.raster_scanline(l).is_some())
            .collect();
        assert_eq!(lines, vec![10]);
        // le compteur 1 (0x1FF) n'atteint jamais 0 dans une trame de 262 lignes
    }

    #[test]
    fn manual_reload_and_reads() {
        let mut r = Raster::new();
        r.write(0x12 / 2, 0x0050);
        assert_eq!(
            r.counter[1], 0,
            "sans bit 15 : pas de rechargement immédiat"
        );
        r.write(0x12 / 2, 0x8050);
        assert_eq!(r.counter[1], 0x50);
        assert_eq!(r.read(0x12 / 2, 0, 0), Some(0x50));
        r.write(0x0e / 2, 0x0021);
        r.raster_scanline(1);
        assert_eq!(r.counter[2], 0x10);
        // (0x10 - 8/2) << 1 & 0x1FE = 0x18, bit 0 de m_cps_b_regs[7]
        assert_eq!(r.read(0x0e / 2, 8, 1), Some(0x19));
        assert_eq!(r.read(0x14 / 2, 0, 0), None);
        assert!(!r.write(0x14 / 2, 0));
    }

    #[test]
    fn delay_in_pixels() {
        let mut r = Raster::new();
        r.write(0x0e / 2, 0x0040); // reload[2] = 0x20
        r.write(0x10 / 2, 1);
        assert_eq!(r.raster_scanline(0), None);
        assert_eq!(r.raster_scanline(1), Some(0x40));
    }
}
