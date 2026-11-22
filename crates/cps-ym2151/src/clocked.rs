// license:BSD-3-Clause
// copyright-holders:Aaron Giles
//! Adaptateur cadencé pour la puce YM2151.
//!
//! Gère le temps en HORLOGES YM (pas mclk), les timers, l'état busy,
//! l'IRQ et la génération d'échantillons à intervalles réguliers.
//!
//! PORT-NOTE : inspiré de ref/puce16-neogeo/ng-core/ym_adapter.rs mais simplifié
//! pour le CPS1 (pas de ROM ADPCM, pas de Rc<RefCell>, temps en horloges YM direct).

use crate::common::{AccessClass, Output, YmfmInterface};
use crate::ym2151::Ym2151;

/// Période d'échantillonnage en horloges YM (64 = 32 opérateurs × prescale 2).
const SAMPLE_PERIOD: u64 = 64;

/// État de l'interface hôte de la puce.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct YmIntf {
    /// Temps courant en horloges YM.
    time: u64,
    /// Échéances des timers A et B (None = arrêté).
    deadlines: [Option<u64>; 2],
    /// Fin de l'état busy en horloges YM.
    busy_end: u64,
    /// Niveau de la ligne IRQ.
    irq: bool,
}

impl YmfmInterface for YmIntf {
    fn sync_mode_write(&mut self, _data: u8) {}

    fn sync_check_interrupts(&mut self) {}

    fn set_timer(&mut self, tnum: u32, duration_in_clocks: i32) {
        if duration_in_clocks < 0 {
            self.deadlines[tnum as usize] = None;
        } else {
            self.deadlines[tnum as usize] = Some(self.time + duration_in_clocks as u64);
        }
    }

    fn set_busy_end(&mut self, clocks: u32) {
        self.busy_end = self.time + clocks as u64;
    }

    fn is_busy(&self) -> bool {
        self.time < self.busy_end
    }

    fn update_irq(&mut self, asserted: bool) {
        self.irq = asserted;
    }

    fn external_write(&mut self, _type_: AccessClass, _address: u32, _data: u8) {}
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClockedYm2151 {
    chip: Ym2151,
    intf: YmIntf,
    /// Nombre d'échantillons générés.
    samples_done: u64,
    /// Échantillons accumulés [gauche, droite].
    out: Vec<[i32; 2]>,
}

impl Default for ClockedYm2151 {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockedYm2151 {
    pub fn new() -> Self {
        let mut chip = Ym2151::new();
        let mut intf = YmIntf {
            time: 0,
            deadlines: [None, None],
            busy_end: 0,
            irq: false,
        };
        chip.reset(&mut intf);

        ClockedYm2151 {
            chip,
            intf,
            samples_done: 0,
            out: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.chip.reset(&mut self.intf);
        self.intf.time = 0;
        self.intf.deadlines = [None, None];
        self.intf.busy_end = 0;
        self.intf.irq = false;
        self.samples_done = 0;
        self.out.clear();
    }

    pub fn time(&self) -> u64 {
        self.intf.time
    }

    /// Fait avancer la puce jusqu'à `t` (horloges YM).
    /// Traite les timers et échantillons dans l'ordre chronologique.
    /// En cas d'égalité timer/échantillon : le timer est traité en premier.
    pub fn advance_to(&mut self, t: u64) {
        loop {
            let ns = self.next_sample_clock();

            // Trouver la plus petite échéance de timer
            let mut nt: Option<u64> = None;
            for &d in self.intf.deadlines.iter() {
                if let Some(d) = d {
                    if nt.is_none() || d < nt.unwrap() {
                        nt = Some(d);
                    }
                }
            }

            // Décider quel événement traiter en premier
            if let Some(nt_val) = nt {
                if nt_val <= ns && nt_val <= t {
                    // Timer expire avant ou à l'égalité de l'échantillon suivant
                    self.intf.time = nt_val;
                    // Trouver quel timer a expiré et le traiter
                    for i in 0..2usize {
                        if self.intf.deadlines[i] == Some(nt_val) {
                            self.intf.deadlines[i] = None;
                            self.chip.fm.engine_timer_expired(i as u32, &mut self.intf);
                        }
                    }
                    continue;
                }
            }

            // Échantillon suivant avant ou à l'égalité de t
            if ns <= t {
                self.intf.time = ns;
                let mut out = Output::<2> { data: [0, 0] };
                self.chip.generate(&mut out, 1);
                self.out.push([out.data[0], out.data[1]]);
                self.samples_done += 1;
            } else {
                break;
            }
        }

        // Avancer le temps au-delà du dernier événement traité
        if t > self.intf.time {
            self.intf.time = t;
        }
    }

    pub fn write(&mut self, offset: u8, data: u8) {
        self.chip.write(offset as u32, data, &mut self.intf);
    }

    pub fn read(&mut self, offset: u8) -> u8 {
        self.chip.read(offset as u32, &self.intf)
    }

    pub fn irq(&self) -> bool {
        self.intf.irq
    }

    /// Horloge YM du prochain échantillon à générer.
    pub fn next_sample_clock(&self) -> u64 {
        (self.samples_done + 1) * SAMPLE_PERIOD
    }

    /// Récupère tous les échantillons accumulés et vide le tampon.
    pub fn take_samples(&mut self) -> Vec<[i32; 2]> {
        std::mem::take(&mut self.out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_produces_zeros() {
        let mut clocked = ClockedYm2151::new();
        clocked.advance_to(64 * 1000);
        let samples = clocked.take_samples();
        assert_eq!(samples.len(), 1000);
        for s in &samples {
            assert_eq!(s[0], 0);
            assert_eq!(s[1], 0);
        }
    }

    #[test]
    fn timer_a_fires_at_correct_time() {
        let mut clocked = ClockedYm2151::new();

        // Load Timer A value: 1023 (period = 1)
        // YM2151 has address port at offset 0, data port at offset 1
        clocked.write(0x00, 0x10); // address = reg 0x10 (timer A low)
        clocked.write(0x01, 0xFF); // data = 0xFF
        clocked.write(0x00, 0x11); // address = reg 0x11 (timer A high)
        clocked.write(0x01, 0x03); // data = 0x03 -> value = 1023

        // Start Timer A (bit 0 = load A, bit 2 = enable A)
        clocked.write(0x00, 0x14); // address = reg 0x14 (mode)
        clocked.write(0x01, 0x05); // data = 0x05

        // At t=63, timer should not have fired yet
        clocked.advance_to(63);
        assert!(!clocked.irq());

        // At t=64, timer should fire (period = 1 * 32 ops * 2 prescale = 64)
        clocked.advance_to(64);
        assert!(clocked.irq());
        let status = clocked.read(0x01);
        assert_ne!(status & 0x01, 0); // STATUS_TIMERA bit set
    }

    #[test]
    fn busy_state_follows_writes() {
        let mut clocked = ClockedYm2151::new();

        // Write a register to trigger busy state
        clocked.write(0x00, 0x30);
        clocked.write(0x01, 0x00);

        // Busy should be set (busy_end = time + 64)
        let status = clocked.read(0x01);
        assert_ne!(status & 0x80, 0); // STATUS_BUSY bit set

        // After advancing past busy period, busy should clear
        clocked.advance_to(clocked.time() + 63);
        let status = clocked.read(0x01);
        assert_ne!(status & 0x80, 0); // Still busy

        clocked.advance_to(clocked.time() + 1);
        let status = clocked.read(0x01);
        assert_eq!(status & 0x80, 0); // No longer busy
    }

    #[test]
    fn next_sample_clock_advances() {
        let mut clocked = ClockedYm2151::new();
        assert_eq!(clocked.next_sample_clock(), 64);

        clocked.advance_to(64);
        assert_eq!(clocked.next_sample_clock(), 128);
    }
}
