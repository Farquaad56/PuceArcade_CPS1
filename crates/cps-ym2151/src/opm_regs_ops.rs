// license:BSD-3-Clause
// copyright-holders:Aaron Giles
//! Opérations sur les registres OPM : write, clock_noise_and_lfo, lfo_am_offset,
//! cache_operator_data, compute_phase_step. Port 1:1 de ymfm_opm.cpp lignes 145-344.

use crate::common::{bitfield, EG_ATTACK, EG_DECAY, EG_RELEASE, EG_SUSTAIN};
use crate::fm_tables::{detune_adjustment, opm_key_code_to_phase_step};
use crate::opm_regs::{OpdataCache, OpmRegisters};

/// KSR appliqué au taux brut : ignoré si le taux brut est 0, plafonné à 63.
fn effective_rate(rawrate: u32, ksr: u32) -> u8 {
    if rawrate == 0 {
        0
    } else {
        (rawrate + ksr).min(63) as u8
    }
}

impl OpmRegisters {
    /// Écriture dans le tableau de registres ; retourne true pour un key on/off
    /// (`channel` et `opmask` sont alors renseignés).
    pub fn write(&mut self, index: u16, data: u8) -> (bool, u32, u32) {
        assert!((index as usize) < Self::REGISTERS);

        // LFO AM/PM depth are written to the same register (0x19);
        // redirect the PM depth to an unused neighbor (0x1a)
        if index == 0x19 {
            let target = (index + bitfield(data as u32, 7, 1) as u16) as usize;
            self.regdata[target] = data;
        } else if index != 0x1a {
            self.regdata[index as usize] = data;
        }

        // handle writes to the key on index
        if index == 0x08 {
            let channel = bitfield(data as u32, 0, 3);
            let opmask = bitfield(data as u32, 3, 4);
            return (true, channel, opmask);
        }
        (false, 0, 0)
    }

    /// Cadence le bruit et le LFO ; retourne la valeur PM du LFO.
    pub fn clock_noise_and_lfo(&mut self) -> i32 {
        // base noise frequency is measured at 2x 1/2 FM frequency; this
        // means each tick counts as two steps against the noise counter
        let freq = self.noise_frequency();
        for _ in 0..2 {
            // evidence seems to suggest the LFSR is clocked continually and just
            // sampled at the noise frequency for output purposes; note that the
            // low 8 bits are the most recent 8 bits of history while bits 8-24
            // contain the 17 bit LFSR state
            self.noise_lfsr <<= 1;
            self.noise_lfsr |=
                bitfield(self.noise_lfsr, 17, 1) ^ bitfield(self.noise_lfsr, 14, 1) ^ 1;

            // compare against the frequency and latch when we exceed it
            if (self.noise_counter as u32) >= freq {
                self.noise_counter = 0;
                self.noise_state = bitfield(self.noise_lfsr, 17, 1) as u8;
            } else {
                self.noise_counter += 1;
            }
        }

        // treat the rate as a 4.4 floating-point step value with implied
        // leading 1; this matches exactly the frequencies in the application
        // manual, though it might not be implemented exactly this way on chip
        let rate = self.lfo_rate();
        let step = (0x10 | bitfield(rate, 0, 4)) << bitfield(rate, 4, 4);
        self.lfo_counter = self.lfo_counter.wrapping_add(step);

        // bit 1 of the test register is officially undocumented but has been
        // discovered to hold the LFO in reset while active
        if self.lfo_reset() != 0 {
            self.lfo_counter = 0;
        }

        // now pull out the non-fractional LFO value
        let lfo = bitfield(self.lfo_counter, 22, 8) as usize;

        // fill in the noise entry 1 ahead of our current position; this
        // ensures the current value remains stable for a full LFO clock
        // and effectively latches the running value when the LFO advances
        let lfo_noise = bitfield(self.noise_lfsr, 17, 8) as u16;
        self.lfo_waveform[3][(lfo + 1) & 0xff] = (lfo_noise | (lfo_noise << 8)) as i16;

        // fetch the AM/PM values based on the waveform; AM is unsigned and
        // encoded in the low 8 bits, while PM signed and encoded in the upper
        // 8 bits
        let ampm = self.lfo_waveform[self.lfo_waveform_select() as usize][lfo];

        // apply depth to the AM value and store for later
        self.lfo_am = ((ampm & 0xff) * (self.lfo_am_depth() as i16)) >> 7;

        // apply depth to the PM value and return it
        let pm_result = ((ampm >> 8) * (self.lfo_pm_depth() as i16)) >> 7;
        pm_result as i32
    }

    /// Décalage AM du LFO pour le canal donné.
    pub fn lfo_am_offset(&self, choffs: u32) -> u32 {
        // OPM maps AM quite differently from OPN

        // shift value for AM sensitivity is [*, 0, 1, 2],
        // mapping to values of [0, 23.9, 47.8, and 95.6dB]
        let am_sensitivity = self.ch_lfo_am_sens(choffs);
        if am_sensitivity == 0 {
            return 0;
        }

        // raw LFO AM value on OPM is 0-FF, which is already a factor of 2
        // larger than the OPN below, putting our staring point at 2x theirs;
        // this works out since our minimum is 2x their maximum
        (self.lfo_am as u32) << (am_sensitivity - 1)
    }

    /// Remplit le cache de l'opérateur avec des données précalculées.
    pub fn cache_operator_data(&mut self, choffs: u32, opoffs: u32, cache: &mut OpdataCache) {
        // get frequency from the channel
        let block_freq = self.ch_block_freq(choffs);
        cache.block_freq = block_freq;

        // compute the keycode: block_freq is:
        //     BBBCCCCFFFFFF
        //     ^^^^^
        // the 5-bit keycode is just the top 5 bits (block + top 2 bits of key code)
        let keycode = bitfield(block_freq, 8, 5);

        // detune adjustment
        cache.detune = detune_adjustment(self.op_detune(opoffs), keycode);

        // multiple value, as an x.1 value (0 means 0.5)
        cache.multiple = self.op_multiple(opoffs) * 2;
        if cache.multiple == 0 {
            cache.multiple = 1;
        }

        // phase step, or PHASE_STEP_DYNAMIC if PM is active; this depends on
        // block_freq, detune, and multiple, so compute it after we've done those
        if self.lfo_pm_depth() == 0 || self.ch_lfo_pm_sens(choffs) == 0 {
            cache.phase_step = self.compute_phase_step(choffs, opoffs, cache, 0);
        } else {
            cache.phase_step = OpdataCache::PHASE_STEP_DYNAMIC;
        }

        // total level, scaled by 8
        cache.total_level = self.op_total_level(opoffs) << 3;

        // 4-bit sustain level, but 15 means 31 so effectively 5 bits
        let mut eg_sustain = self.op_sustain_level(opoffs);
        eg_sustain |= (eg_sustain + 1) & 0x10;
        cache.eg_sustain = eg_sustain << 5;

        // determine KSR adjustment for envelope rates
        let ksrval = keycode >> (self.op_ksr(opoffs) ^ 3);
        cache.eg_rate[EG_ATTACK] = effective_rate(self.op_attack_rate(opoffs) * 2, ksrval);
        cache.eg_rate[EG_DECAY] = effective_rate(self.op_decay_rate(opoffs) * 2, ksrval);
        cache.eg_rate[EG_SUSTAIN] = effective_rate(self.op_sustain_rate(opoffs) * 2, ksrval);
        cache.eg_rate[EG_RELEASE] = effective_rate(self.op_release_rate(opoffs) * 4 + 2, ksrval);
    }

    /// Calcule le pas de phase pour une valeur PM donnée.
    pub fn compute_phase_step(
        &self,
        choffs: u32,
        opoffs: u32,
        cache: &OpdataCache,
        lfo_raw_pm: i32,
    ) -> u32 {
        // OPM logic is rather unique here, due to extra detune
        // and the use of key codes (not to be confused with keycode)

        // start with coarse detune delta; table uses cents value from
        // manual, converted into 1/64ths
        let s_detune2_delta: [i32; 4] = [0, 384, 500, 608];
        let mut delta = s_detune2_delta[self.op_detune2(opoffs) as usize];

        // add in the PM delta
        let pm_sensitivity = self.ch_lfo_pm_sens(choffs);
        if pm_sensitivity != 0 {
            // raw PM value is -127..128 which is +/- 200 cents
            // manual gives these magnitudes in cents:
            //    0, +/-5, +/-10, +/-20, +/-50, +/-100, +/-400, +/-700
            // this roughly corresponds to shifting the 200-cent value:
            //    0  >> 5,  >> 4,  >> 3,  >> 2,  >> 1,   << 1,   << 2
            if pm_sensitivity < 6 {
                delta += lfo_raw_pm >> (6 - pm_sensitivity);
            } else {
                delta += lfo_raw_pm << (pm_sensitivity - 5);
            }
        }

        // apply delta and convert to a frequency number
        let phase_step = opm_key_code_to_phase_step(cache.block_freq, delta);

        // apply detune based on the keycode
        let mut result = phase_step.wrapping_add(cache.detune as u32);

        // apply frequency multiplier (which is cached as an x.1 value)
        result = (result * cache.multiple) >> 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regs() -> OpmRegisters {
        let mut r = OpmRegisters::new();
        r.reset();
        r
    }

    #[test]
    fn write_key_on_channel_0_all_ops() {
        let mut r = regs();
        let (keyon, channel, opmask) = r.write(0x08, 0x78);
        assert!(keyon);
        assert_eq!(channel, 0);
        assert_eq!(opmask, 0xF);
    }

    #[test]
    fn write_key_on_channel_3_op0() {
        let mut r = regs();
        let (keyon, channel, opmask) = r.write(0x08, 0x0B);
        assert!(keyon);
        assert_eq!(channel, 3);
        assert_eq!(opmask, 0x1);
    }

    #[test]
    fn write_non_key_on() {
        let mut r = regs();
        let (keyon, _channel, _opmask) = r.write(0x20, 0xC7);
        assert!(!keyon);
    }

    #[test]
    fn clock_noise_lfo_zeroed_registers() {
        let mut r = regs();
        for _ in 0..10 {
            let pm = r.clock_noise_and_lfo();
            assert_eq!(pm, 0);
        }
        assert_eq!(r.noise_state_get(), 0);
    }

    #[test]
    fn cache_operator_data_phase_step() {
        let mut r = regs();
        // Set up registers as in the test spec
        r.write(0x20, 0xC7); // channel config
        r.write(0x28, 0x4A); // block/freq low byte (key code)
        r.write(0x40, 0x01); // operator multiple = 1

        let mut cache = OpdataCache::default();
        r.cache_operator_data(0, 0, &mut cache);

        assert_eq!(cache.block_freq, 0x1280);
        assert_eq!(cache.multiple, 2);
        let step = r.compute_phase_step(0, 0, &cache, 0);
        assert_eq!(step, 8248);
    }
}
