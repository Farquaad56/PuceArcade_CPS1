//! Capcom QSound DL-1425, modèle haut niveau (HLE).
// license:BSD-3-Clause
// copyright-holders:superctr, Valley Bell
// source: MAME 0.287 src/devices/sound/qsoundhle.cpp / qsoundhle.h (port 1:1)
//
// Le DSP tourne à 60 MHz ; son programme prend 1248 cycles machine (2 horloges) par échantillon :
// fréquence de sortie 60 MHz / 2 / 1248 = 24 038,46 Hz, stéréo.
#![forbid(unsafe_code)]

/// Fréquence d'échantillonnage (Hz).
pub const SAMPLE_HZ: f64 = 60_000_000.0 / 2.0 / 1248.0;

const DATA_PAN_TAB: u16 = 0x110;
const DATA_ADPCM_TAB: u16 = 0x9dc;
const DATA_FILTER_TAB: u16 = 0xd53; // dual filter mode, 5 tables * 95 taps each

const STATE_BOOT: u16 = 0x000;
const STATE_INIT1: u16 = 0x288;
const STATE_INIT2: u16 = 0x61a;
const STATE_REFRESH1: u16 = 0x039;
const STATE_REFRESH2: u16 = 0x04f;
const STATE_NORMAL1: u16 = 0x314;
const STATE_NORMAL2: u16 = 0x6b2;

const PAN_TABLE_DRY: u16 = 0;
const PAN_TABLE_WET: u16 = 98;
const PAN_TABLE_CH_OFFSET: u16 = 196;
const FILTER_ENTRY_SIZE: u16 = 95;
const DELAY_BASE_OFFSET: u16 = 0x554;
const DELAY_BASE_OFFSET2: u16 = 0x53c;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Voice {
    bank: u16,
    addr: i16, // top word is the sample address
    phase: u16,
    rate: u16,
    loop_len: i16,
    end_addr: i16,
    volume: i16,
    echo: i16,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Adpcm {
    start_addr: u16,
    end_addr: u16,
    bank: u16,
    volume: i16,
    flag: u16,
    cur_vol: i16,
    step_size: i16,
    cur_addr: u16,
}

/// Filtre FIR (fonction de transfert Q1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Fir {
    tap_count: i32, // usually 95
    delay_pos: i32,
    table_pos: u16,
    taps: Vec<i16>,
    delay_line: Vec<i16>,
}

impl Default for Fir {
    fn default() -> Self {
        Fir {
            tap_count: 0,
            delay_pos: 0,
            table_pos: 0,
            taps: vec![0; 95],
            delay_line: vec![0; 95],
        }
    }
}

/// Ligne à retard.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Delay {
    delay: i16,
    volume: i16,
    write_pos: i16,
    read_pos: i16,
    delay_line: Vec<i16>,
}

impl Default for Delay {
    fn default() -> Self {
        Delay {
            delay: 0,
            volume: 0,
            write_pos: 0,
            read_pos: 0,
            delay_line: vec![0; 51],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Echo {
    end_pos: u16,
    feedback: i16,
    length: i16,
    last_sample: i16,
    delay_line: Vec<i16>,
    delay_pos: i16,
}

impl Default for Echo {
    fn default() -> Self {
        Echo {
            end_pos: 0,
            feedback: 0,
            length: 0,
            last_sample: 0,
            delay_line: vec![0; 1024],
            delay_pos: 0,
        }
    }
}

/// Puce QSound. La ROM interne du DSP (`dl-1425.bin`, 4096 mots petit-boutistes) et la ROM des
/// échantillons (région « qsound ») ne sont pas sérialisées : `set_roms` après un chargement d'état.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QSound {
    #[cfg_attr(feature = "serde", serde(skip))]
    dsp_rom: Vec<u16>,
    #[cfg_attr(feature = "serde", serde(skip))]
    sample_rom: Vec<u8>,
    data_latch: u16,
    out: [i16; 2],
    voice: Vec<Voice>,
    adpcm: Vec<Adpcm>,
    voice_pan: Vec<u16>,
    voice_output: Vec<i16>,
    echo: Echo,
    filter: Vec<Fir>,
    alt_filter: Vec<Fir>,
    wet: Vec<Delay>,
    dry: Vec<Delay>,
    state: u16,
    next_state: u16,
    delay_update: u16,
    state_counter: i32,
    ready_flag: i32,
}

impl QSound {
    /// `dsp_rom` : contenu de `dl-1425.bin` (au moins 0x2000 octets ; seuls les 0x2000 premiers sont
    /// lus, mots de 16 bits petit-boutistes, comme ROM_LOAD16_WORD dans une région LE) ;
    /// `sample_rom` : région « qsound ».
    pub fn new(dsp_rom: &[u8], sample_rom: Vec<u8>) -> QSound {
        let mut q = QSound {
            dsp_rom: Vec::new(),
            sample_rom: Vec::new(),
            data_latch: 0,
            out: [0; 2],
            voice: vec![Voice::default(); 16],
            adpcm: vec![Adpcm::default(); 3],
            voice_pan: vec![0; 19],
            voice_output: vec![0; 19],
            echo: Echo::default(),
            filter: vec![Fir::default(); 2],
            alt_filter: vec![Fir::default(); 2],
            wet: vec![Delay::default(); 2],
            dry: vec![Delay::default(); 2],
            state: STATE_BOOT,
            next_state: 0,
            delay_update: 0,
            state_counter: 0,
            ready_flag: 0,
        };
        q.set_roms(dsp_rom, sample_rom);
        q.reset();
        q
    }

    /// Recharge les ROM (après désérialisation).
    pub fn set_roms(&mut self, dsp_rom: &[u8], sample_rom: Vec<u8>) {
        self.dsp_rom = (0..0x1000)
            .map(|i| {
                let lo = dsp_rom.get(2 * i).copied().unwrap_or(0);
                let hi = dsp_rom.get(2 * i + 1).copied().unwrap_or(0);
                u16::from_le_bytes([lo, hi])
            })
            .collect();
        self.sample_rom = sample_rom;
    }

    /// device_reset
    pub fn reset(&mut self) {
        self.ready_flag = 0;
        self.out = [0, 0];
        self.state = STATE_BOOT;
        self.state_counter = 0;
    }

    /// Dernier échantillon produit (gauche, droite).
    pub fn output(&self) -> [i16; 2] {
        self.out
    }

    /// qsound_w (Z80 D000-D002)
    pub fn write(&mut self, offset: u8, data: u8) {
        match offset {
            0 => self.data_latch = (self.data_latch & 0x00ff) | (u16::from(data) << 8),
            1 => self.data_latch = (self.data_latch & 0xff00) | u16::from(data),
            2 => self.write_data(data, self.data_latch),
            _ => {}
        }
    }

    /// qsound_r (Z80 D007) : 0x80 = prêt, 0x00 = occupé.
    pub fn read(&self) -> u8 {
        self.ready_flag as u8
    }

    fn read_dsp_rom(&self, addr: u16) -> u16 {
        self.dsp_rom[usize::from(addr & 0xfff)]
    }

    fn write_data(&mut self, address: u8, data: u16) {
        let a = usize::from(address);
        let s = data as i16;
        match a {
            0x00..=0x7f => {
                let i = a >> 3;
                match a & 7 {
                    0 => self.voice[(i + 1) % 16].bank = data, // Bank applies to the next channel
                    1 => self.voice[i].addr = s,
                    2 => self.voice[i].rate = data,
                    3 => self.voice[i].phase = data,
                    4 => self.voice[i].loop_len = s,
                    5 => self.voice[i].end_addr = s,
                    6 => self.voice[i].volume = s,
                    _ => {}
                }
            }
            0x80..=0x8f => self.voice_pan[a - 0x80] = data,
            0x90..=0x92 => self.voice_pan[16 + a - 0x90] = data,
            0x93 => self.echo.feedback = s,
            0xba..=0xc9 => self.voice[a - 0xba].echo = s,
            0xca..=0xd5 => {
                let ad = &mut self.adpcm[(a - 0xca) >> 2];
                match (a - 0xca) & 3 {
                    0 => ad.start_addr = data,
                    1 => ad.end_addr = data,
                    2 => ad.bank = data,
                    _ => ad.volume = s,
                }
            }
            0xd6..=0xd8 => self.adpcm[a - 0xd6].flag = data,
            0xd9 => self.echo.end_pos = data,
            0xda => self.filter[0].table_pos = data,
            0xdb => self.alt_filter[0].table_pos = data,
            0xdc => self.filter[1].table_pos = data,
            0xdd => self.alt_filter[1].table_pos = data,
            0xde => self.wet[0].delay = s,
            0xdf => self.dry[0].delay = s,
            0xe0 => self.wet[1].delay = s,
            0xe1 => self.dry[1].delay = s,
            0xe2 => self.delay_update = data,
            0xe3 => self.next_state = data,
            0xe4 => self.wet[0].volume = s,
            0xe5 => self.dry[0].volume = s,
            0xe6 => self.wet[1].volume = s,
            0xe7 => self.dry[1].volume = s,
            _ => {}
        }
        self.ready_flag = 0;
    }

    fn read_sample(&self, bank: u16, address: u16) -> i16 {
        let bank = bank & 0x7fff;
        let rom_addr = (u32::from(bank) << 16) | u32::from(address);
        // PORT-NOTE: device_rom_interface<24> : la région est vue en miroir dans l'espace de 24 bits.
        let len = self.sample_rom.len();
        let b = if len == 0 {
            0
        } else {
            self.sample_rom[(rom_addr & 0xff_ffff) as usize % len]
        };
        (u16::from(b) << 8) as i16 // bit0-7 is tied to ground
    }

    /// Un échantillon du DSP (update_sample).
    pub fn update_sample(&mut self) {
        match self.state {
            STATE_REFRESH1 => self.state_refresh_filter_1(),
            STATE_REFRESH2 => self.state_refresh_filter_2(),
            STATE_NORMAL1 | STATE_NORMAL2 => self.state_normal_update(),
            STATE_BOOT | STATE_INIT1 | STATE_INIT2 => self.state_init(),
            _ => self.state_init(), // default:
        }
    }

    fn state_init(&mut self) {
        let mode = if self.state == STATE_INIT2 { 1 } else { 0 };
        // we're busy for 4 samples, including the filter refresh.
        if self.state_counter >= 2 {
            self.state_counter = 0;
            self.state = self.next_state;
            return;
        } else if self.state_counter == 1 {
            self.state_counter += 1;
            return;
        }
        self.voice.iter_mut().for_each(|v| *v = Voice::default());
        self.adpcm.iter_mut().for_each(|v| *v = Adpcm::default());
        self.filter.iter_mut().for_each(|v| *v = Fir::default());
        self.alt_filter.iter_mut().for_each(|v| *v = Fir::default());
        self.wet.iter_mut().for_each(|v| *v = Delay::default());
        self.dry.iter_mut().for_each(|v| *v = Delay::default());
        self.echo = Echo::default();
        for i in 0..19 {
            self.voice_pan[i] = DATA_PAN_TAB + 0x10;
            self.voice_output[i] = 0;
        }
        for v in &mut self.voice {
            v.bank = 0x8000;
        }
        for a in &mut self.adpcm {
            a.bank = 0x8000;
        }
        if mode == 0 {
            // mode 1
            self.wet[0].delay = 0;
            self.dry[0].delay = 46;
            self.wet[1].delay = 0;
            self.dry[1].delay = 48;
            self.filter[0].table_pos = DATA_FILTER_TAB + FILTER_ENTRY_SIZE;
            self.filter[1].table_pos = DATA_FILTER_TAB + FILTER_ENTRY_SIZE * 2;
            self.echo.end_pos = DELAY_BASE_OFFSET + 6;
            self.next_state = STATE_REFRESH1;
        } else {
            // mode 2
            self.wet[0].delay = 1;
            self.dry[0].delay = 0;
            self.wet[1].delay = 0;
            self.dry[1].delay = 0;
            self.filter[0].table_pos = 0xf73;
            self.filter[1].table_pos = 0xfa4;
            self.alt_filter[0].table_pos = 0xf73;
            self.alt_filter[1].table_pos = 0xfa4;
            self.echo.end_pos = DELAY_BASE_OFFSET2 + 6;
            self.next_state = STATE_REFRESH2;
        }
        self.wet[0].volume = 0x3fff;
        self.dry[0].volume = 0x3fff;
        self.wet[1].volume = 0x3fff;
        self.dry[1].volume = 0x3fff;
        self.delay_update = 1;
        self.ready_flag = 0;
        self.state_counter = 1;
    }

    fn state_refresh_filter_1(&mut self) {
        for ch in 0..2 {
            self.filter[ch].delay_pos = 0;
            self.filter[ch].tap_count = 95;
            for i in 0..95u16 {
                let v = self.read_dsp_rom(self.filter[ch].table_pos.wrapping_add(i)) as i16;
                self.filter[ch].taps[usize::from(i)] = v;
            }
        }
        self.state = STATE_NORMAL1;
        self.next_state = STATE_NORMAL1;
    }

    fn state_refresh_filter_2(&mut self) {
        for ch in 0..2 {
            self.filter[ch].delay_pos = 0;
            self.filter[ch].tap_count = 45;
            for i in 0..45u16 {
                let v = self.read_dsp_rom(self.filter[ch].table_pos.wrapping_add(i)) as i16;
                self.filter[ch].taps[usize::from(i)] = v;
            }
            self.alt_filter[ch].delay_pos = 0;
            self.alt_filter[ch].tap_count = 44;
            for i in 0..44u16 {
                let v = self.read_dsp_rom(self.alt_filter[ch].table_pos.wrapping_add(i)) as i16;
                self.alt_filter[ch].taps[usize::from(i)] = v;
            }
        }
        self.state = STATE_NORMAL2;
        self.next_state = STATE_NORMAL2;
    }

    /// qsound_voice::update
    fn voice_update(&mut self, i: usize, echo_out: &mut i32) -> i16 {
        let v = &self.voice[i];
        let sample = self.read_sample(v.bank, v.addr as u16);
        let output = ((i32::from(v.volume) * i32::from(sample)) >> 14) as i16;
        *echo_out = echo_out.wrapping_add((i32::from(output) * i32::from(v.echo)) << 2);
        let v = &mut self.voice[i];
        let mut new_phase =
            i32::from(v.rate).wrapping_add((i32::from(v.addr) << 12) | i32::from(v.phase >> 4));
        if (new_phase >> 12) >= i32::from(v.end_addr) {
            new_phase = new_phase.wrapping_sub(i32::from(v.loop_len) << 12);
        }
        new_phase = new_phase.clamp(-0x800_0000, 0x7FF_FFFF);
        v.addr = (new_phase >> 12) as i16;
        v.phase = ((new_phase << 4) & 0xffff) as u16;
        output
    }

    /// qsound_adpcm::update
    fn adpcm_update(&mut self, i: usize, mut curr_sample: i16, nibble: i32) -> i16 {
        let step: i8;
        if nibble == 0 {
            let a = &mut self.adpcm[i];
            // Mute voice when it reaches the end address.
            if a.cur_addr == a.end_addr {
                a.cur_vol = 0;
            }
            // Playback start flag
            if a.flag != 0 {
                curr_sample = 0;
                a.flag = 0;
                a.step_size = 10;
                a.cur_vol = a.volume;
                a.cur_addr = a.start_addr;
            }
            // get top nibble
            let (bank, addr) = (a.bank, a.cur_addr);
            step = (self.read_sample(bank, addr) >> 8) as i8;
        } else {
            // get bottom nibble
            let (bank, addr) = (self.adpcm[i].bank, self.adpcm[i].cur_addr);
            self.adpcm[i].cur_addr = addr.wrapping_add(1);
            step = (self.read_sample(bank, addr) >> 4) as i8;
        }
        // shift with sign extend
        let step = step >> 4;
        let a_step = i32::from(self.adpcm[i].step_size);
        // delta = (0.5 + abs(step)) * m_step_size
        let mut delta = ((1 + (i32::from(step) << 1).abs()) * a_step) >> 1;
        if step <= 0 {
            delta = -delta;
        }
        delta += i32::from(curr_sample);
        let delta = delta.clamp(-32768, 32767);
        let rom =
            i32::from(self.read_dsp_rom((DATA_ADPCM_TAB as i32 + 8 + i32::from(step)) as u16));
        let a = &mut self.adpcm[i];
        a.step_size = ((rom * a_step) >> 6) as i16;
        a.step_size = a.step_size.clamp(1, 2000);
        ((delta * i32::from(a.cur_vol)) >> 16) as i16
    }

    fn state_normal_update(&mut self) {
        self.ready_flag = 0x80;
        // recalculate echo length
        let base = if self.state == STATE_NORMAL2 {
            DELAY_BASE_OFFSET2
        } else {
            DELAY_BASE_OFFSET
        };
        self.echo.length = (self.echo.end_pos.wrapping_sub(base) as i16).clamp(0, 1024);

        // update PCM voices
        let mut echo_input = 0i32;
        for i in 0..16 {
            self.voice_output[i] = self.voice_update(i, &mut echo_input);
        }
        // update ADPCM voices (one every third sample)
        let av = (self.state_counter % 3) as usize;
        let prev = self.voice_output[16 + av];
        self.voice_output[16 + av] = self.adpcm_update(av, prev, self.state_counter / 3);

        let echo_output = self.echo.apply(echo_input);

        // now, we do the magic stuff
        for ch in 0..2 {
            // Echo is output on the unfiltered component of the left channel and
            // the filtered component of the right channel.
            let mut wet: i32 = if ch == 1 {
                i32::from(echo_output) << 14
            } else {
                0
            };
            let mut dry: i32 = if ch == 0 {
                i32::from(echo_output) << 14
            } else {
                0
            };
            for i in 0..19 {
                let pan_index = self.voice_pan[i].wrapping_add(ch as u16 * PAN_TABLE_CH_OFFSET);
                let vo = i32::from(self.voice_output[i]);
                // Apply different volume tables on the dry and wet inputs.
                dry = dry.wrapping_sub(
                    vo * i32::from(self.read_dsp_rom(pan_index.wrapping_add(PAN_TABLE_DRY)) as i16),
                );
                wet = wet.wrapping_sub(
                    vo * i32::from(self.read_dsp_rom(pan_index.wrapping_add(PAN_TABLE_WET)) as i16),
                );
            }
            // Saturate accumulated voices
            dry = dry.clamp(-0x1fff_ffff, 0x1fff_ffff) << 2;
            wet = wet.clamp(-0x1fff_ffff, 0x1fff_ffff) << 2;
            // Apply FIR filter on 'wet' input
            wet = self.filter[ch].apply((wet >> 16) as i16);
            // in mode 2, we do this on the 'dry' input too
            if self.state == STATE_NORMAL2 {
                dry = self.alt_filter[ch].apply((dry >> 16) as i16);
            }
            // output goes through a delay line and attenuation
            let mut output = self.wet[ch]
                .apply(wet)
                .wrapping_add(self.dry[ch].apply(dry));
            // DSP round function
            output = output.wrapping_add(0x2000) & !0x3fff;
            self.out[ch] = (output >> 14).clamp(-0x7fff, 0x7fff) as i16;
            if self.delay_update != 0 {
                self.wet[ch].update();
                self.dry[ch].update();
            }
        }
        self.delay_update = 0;
        // after 6 samples, the next state is executed.
        self.state_counter += 1;
        if self.state_counter > 5 {
            self.state_counter = 0;
            self.state = self.next_state;
        }
    }
}

impl Echo {
    /// The echo effect is pretty simple. A moving average filter is used on
    /// the output from the delay line to smooth samples over time.
    fn apply(&mut self, input: i32) -> i16 {
        // get average of last 2 samples from the delay line
        let mut old_sample = i32::from(self.delay_line[self.delay_pos as usize]);
        let last_sample = i32::from(self.last_sample);
        self.last_sample = old_sample as i16;
        old_sample = (old_sample + last_sample) >> 1;
        // add current sample to the delay line
        let new_sample = input.wrapping_add((old_sample * i32::from(self.feedback)) << 2);
        self.delay_line[self.delay_pos as usize] = (new_sample >> 16) as i16;
        self.delay_pos += 1;
        if self.delay_pos >= self.length {
            self.delay_pos = 0;
        }
        old_sample as i16
    }
}

impl Fir {
    /// Apply the FIR filter used as the Q1 transfer function
    fn apply(&mut self, input: i16) -> i32 {
        let mut output = 0i32;
        let mut tap = 0usize;
        while (tap as i32) < self.tap_count - 1 {
            let d = i32::from(self.delay_line[self.delay_pos as usize]);
            output = output.wrapping_sub((i32::from(self.taps[tap]) * d) << 2);
            self.delay_pos += 1;
            if self.delay_pos >= self.tap_count - 1 {
                self.delay_pos = 0;
            }
            tap += 1;
        }
        // PORT-NOTE: avec tap_count = 0 (état initial), MAME lit m_taps[0] et écrit m_delay_line[0].
        let t = i32::from(self.taps.get(tap).copied().unwrap_or(0));
        output = output.wrapping_sub((t * i32::from(input)) << 2);
        self.delay_line[self.delay_pos as usize] = input;
        self.delay_pos += 1;
        if self.delay_pos >= self.tap_count - 1 {
            self.delay_pos = 0;
        }
        output
    }
}

impl Delay {
    /// Apply delay line and component volume
    fn apply(&mut self, input: i32) -> i32 {
        self.delay_line[self.write_pos as usize] = (input >> 16) as i16;
        self.write_pos += 1;
        if self.write_pos >= 51 {
            self.write_pos = 0;
        }
        let output = i32::from(self.delay_line[self.read_pos as usize]) * i32::from(self.volume);
        self.read_pos += 1;
        if self.read_pos >= 51 {
            self.read_pos = 0;
        }
        output
    }

    /// Update the delay read position to match new delay length
    fn update(&mut self) {
        let new_read_pos = (i32::from(self.write_pos) - i32::from(self.delay)) % 51;
        self.read_pos = if new_read_pos < 0 {
            new_read_pos + 51
        } else {
            new_read_pos
        } as i16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ROM DSP synthétique : tables de panoramique à 0x4000 (gain unitaire), filtres nuls sauf un
    /// coefficient, table ADPCM à 0.
    fn dsp_rom() -> Vec<u8> {
        let mut w = vec![0u16; 0x1000];
        for i in 0..0x200 {
            w[usize::from(DATA_PAN_TAB) + i] = 0xC000; // -0x4000 : les voix sont soustraites
        }
        let mut b = Vec::new();
        for x in w {
            b.extend_from_slice(&x.to_le_bytes());
        }
        b.resize(0x6000, 0);
        b
    }

    fn booted() -> QSound {
        let mut q = QSound::new(&dsp_rom(), vec![0x40; 0x10000]);
        for _ in 0..8 {
            q.update_sample();
        }
        q
    }

    #[test]
    fn boot_sequence_reaches_normal_mode() {
        let mut q = QSound::new(&dsp_rom(), vec![]);
        assert_eq!(q.read(), 0);
        q.update_sample(); // init
        assert_eq!(q.state_counter, 1);
        q.update_sample();
        q.update_sample(); // -> REFRESH1
        assert_eq!(q.state, STATE_REFRESH1);
        q.update_sample(); // -> NORMAL1
        assert_eq!(q.state, STATE_NORMAL1);
        q.update_sample();
        assert_eq!(q.read(), 0x80, "prêt en mode normal");
        assert_eq!(q.echo.length, 6);
    }

    #[test]
    fn register_writes_and_busy_flag() {
        let mut q = booted();
        q.write(0, 0x12);
        q.write(1, 0x34);
        q.write(2, 0x08 + 6); // voix 1, volume
        assert_eq!(q.voice[1].volume, 0x1234);
        assert_eq!(q.read(), 0, "occupé après une écriture");
        q.write(2, 0x00); // banque : s'applique à la voix suivante
        assert_eq!(q.voice[1].bank, 0x1234);
        q.write(2, 0xe3);
        assert_eq!(q.next_state, 0x1234);
    }

    #[test]
    fn voice_phase_and_loop() {
        let mut q = booted();
        let v = &mut q.voice[0];
        v.addr = 10;
        v.rate = 0x2000; // 2 échantillons par sortie
        v.end_addr = 14;
        v.loop_len = 4;
        v.volume = 0x4000;
        let mut e = 0;
        let out = q.voice_update(0, &mut e);
        assert_eq!(out, 0x4000, "octet 0x40 << 8, volume unité");
        assert_eq!(q.voice[0].addr, 12);
        q.voice_update(0, &mut e);
        // 14 >= fin : retour de loop_len
        assert_eq!(q.voice[0].addr, 10);
    }

    #[test]
    fn delay_read_position_wraps() {
        let mut d = Delay {
            write_pos: 2,
            delay: 5,
            ..Delay::default()
        };
        d.update();
        assert_eq!(d.read_pos, 48);
    }

    #[test]
    fn voice_reaches_output() {
        let mut q = booted();
        q.voice[0] = Voice {
            bank: 0x8000,
            addr: 0,
            rate: 0x1000,
            end_addr: 0x7fff,
            volume: 0x4000,
            ..Voice::default()
        };
        let mut any = false;
        for _ in 0..200 {
            q.update_sample();
            any |= q.output() != [0, 0];
        }
        assert!(any, "une voix active produit du son");
    }
}
