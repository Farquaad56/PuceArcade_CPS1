// license:BSD-3-Clause
// copyright-holders:Aaron Giles
//! Puce YM2151 (OPM) — port de ref/ymfm/src/ymfm_opm.cpp lignes 395-539.
//!
//! PORT-NOTE : la variante `opm_variant` est fixée à YM2151 (pas de YM2164).
//! `save_restore` n'est pas porté (sérialisation traitée séparément dans cps-common).

use crate::common::{AccessClass, Output, YmfmInterface};
use crate::fm_engine::FmEngine;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ym2151 {
    /// Registre d'adresse courant.
    address: u8,
    /// Moteur FM interne.
    fm: FmEngine,
}

impl Default for Ym2151 {
    fn default() -> Self {
        Self::new()
    }
}

impl Ym2151 {
    pub const OUTPUTS: u32 = FmEngine::OUTPUTS;

    /// Constructeur.
    pub fn new() -> Self {
        Ym2151 {
            address: 0,
            fm: FmEngine::new(),
        }
    }

    /// Remet l'état global à zéro.
    pub fn reset(&mut self, intf: &mut dyn YmfmInterface) {
        // remet à zéro le moteur FM
        self.fm.reset(intf);
    }

    /// Calcule la fréquence d'échantillonnage.
    pub fn sample_rate(&self, input_clock: u32) -> u32 {
        self.fm.sample_rate(input_clock)
    }

    /// Invalide les caches.
    pub fn invalidate_caches(&mut self) {
        self.fm.invalidate_caches();
    }

    /// Lit le registre de statut.
    pub fn read_status(&self, intf: &dyn YmfmInterface) -> u8 {
        let result = self.fm.status();
        if intf.is_busy() {
            return result | FmEngine::STATUS_BUSY;
        }
        result
    }

    /// Gère une lecture depuis la puce.
    pub fn read(&self, offset: u32, intf: &dyn YmfmInterface) -> u8 {
        let mut result = 0xff;
        match offset & 1 {
            // port de données (non utilisé)
            0 => {}
            // port de statut, compatible YM2203
            1 => {
                result = self.read_status(intf);
            }
            _ => unreachable!(),
        }
        result
    }

    /// Gère une écriture dans le registre d'adresse.
    pub fn write_address(&mut self, data: u8) {
        // fixe simplement l'adresse
        self.address = data;
    }

    /// Gère une écriture dans le port de données.
    pub fn write_data(&mut self, data: u8, intf: &mut dyn YmfmInterface) {
        // écrit le registre FM
        self.fm.write(self.address as u16, data, intf);

        // cas spéciaux
        if self.address == 0x1b {
            // les écritures dans le registre 0x1B envoient les 2 bits supérieurs sur les lignes de sortie
            intf.external_write(AccessClass::IoPort, 0, data >> 6);
        }

        // marque busy pour un moment
        let prescale = self.fm.clock_prescale();
        intf.set_busy_end(32 * prescale);
    }

    /// Gère une écriture vers l'interface de registre.
    pub fn write(&mut self, offset: u32, data: u8, intf: &mut dyn YmfmInterface) {
        match offset & 1 {
            // port d'adresse
            0 => {
                self.write_address(data);
            }
            // port de données
            1 => {
                self.write_data(data, intf);
            }
            _ => unreachable!(),
        }
    }

    /// Génère un échantillon de son.
    pub fn generate(&mut self, output: &mut Output<2>, numsamples: u32) {
        for _ in 0..numsamples {
            // cadence le système
            self.fm.clock(FmEngine::ALL_CHANNELS);

            // met à jour le contenu FM ; l'OPM est en 14 bits complets sans clipping intermédiaire
            output.clear();
            self.fm.output(output, 0, 32767, FmEngine::ALL_CHANNELS);

            // le YM2151 utilise un DAC externe (YM3012) au format mantisse/exposant
            // convertit en valeur flottante 10.3 et retour pour simuler la troncature
            output.roundtrip_fp();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestIntf {
        timers: Vec<(u32, i32)>,
        busy_end: u32,
        external_writes: Vec<(AccessClass, u32, u8)>,
    }

    impl YmfmInterface for TestIntf {
        fn sync_mode_write(&mut self, _data: u8) {}
        fn sync_check_interrupts(&mut self) {}
        fn set_timer(&mut self, tnum: u32, duration: i32) {
            self.timers.push((tnum, duration));
        }
        fn set_busy_end(&mut self, clocks: u32) {
            self.busy_end = clocks;
        }
        fn is_busy(&self) -> bool {
            false
        }
        fn external_write(&mut self, type_: AccessClass, address: u32, data: u8) {
            self.external_writes.push((type_, address, data));
        }
    }

    #[test]
    fn timer_a_load_sets_timer() {
        let mut chip = Ym2151::new();
        let mut intf = TestIntf::default();
        chip.reset(&mut intf);
        intf.timers.clear();

        // Timer A value: write low byte then high byte (0xFF, 0x03 -> 1023)
        chip.write(0, 0x10, &mut intf);
        chip.write(1, 0xFF, &mut intf);
        chip.write(0, 0x11, &mut intf);
        chip.write(1, 0x03, &mut intf);

        // Load and start timer A (bit 0 = load A, bit 2 = enable A)
        chip.write(0, 0x14, &mut intf);
        chip.write(1, 0x05, &mut intf);

        // period = 1024 - 1023 = 1; duration = 1 * 32 ops * 2 prescale = 64
        assert_eq!(intf.timers, vec![(1, -1), (0, 64)]);
    }

    #[test]
    fn read_returns_ff_on_data_port() {
        let chip = Ym2151::new();
        let intf = TestIntf::default();
        assert_eq!(chip.read(0, &intf), 0xff);
    }

    #[test]
    fn generate_with_null_regs_produces_zeros() {
        let mut chip = Ym2151::new();
        let mut intf = TestIntf::default();
        chip.reset(&mut intf);

        // All registers are zero after reset, so output should be [0, 0]
        let mut out = Output::<2> { data: [0, 0] };
        chip.generate(&mut out, 1);
        assert_eq!(out.data[0], 0);
        assert_eq!(out.data[1], 0);
    }

    #[test]
    fn write_data_sets_busy() {
        let mut chip = Ym2151::new();
        let mut intf = TestIntf::default();
        chip.reset(&mut intf);

        // Write any register to trigger busy
        chip.write(0, 0x00, &mut intf);
        chip.write(1, 0x00, &mut intf);

        // Busy should be set for 32 * prescale clocks (prescale=2 by default)
        assert_eq!(intf.busy_end, 64);
    }
}
