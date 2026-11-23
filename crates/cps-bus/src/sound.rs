//! CPS1 Z80 SoundBoard: memory map, banking, ports, latches, pin server.
//! spec: docs/spec/01 §Z80

use cps_common::clock::{Tick, TICKS_PER_QSOUND_IRQ, TICKS_PER_QSOUND_SAMPLE};
use puce_z80::{Cpu, Pins};

/// Partie propre aux cartes QSound (cps1.cpp qsound_sub_map, l.683-697) : opcodes déchiffrés,
/// seconde RAM partagée, DSP, IRQ périodique à 250 Hz maintenue jusqu'à l'acquittement.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QSoundSide {
    /// Opcodes Z80 déchiffrés (Kabuki), lus aux cycles M1 en 0000-7FFF ; non sérialisés.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub opcodes: Vec<u8>,
    /// RAM partagée n° 2 (Z80 F000-FFFF, 68000 F1E000-F1FFFF) ; la n° 1 est `SoundBoard::ram`.
    pub ram2: Vec<u8>,
    pub dsp: cps_qsound::QSound,
    /// irq0_line_hold : ligne INT maintenue jusqu'à l'acquittement.
    pub irq: bool,
    pub next_irq: Tick,
    pub next_sample: Tick,
    /// Échantillons stéréo produits depuis le dernier `take_audio`.
    pub out: Vec<[f32; 2]>,
}

impl QSoundSide {
    pub fn new(opcodes: Vec<u8>, dsp: cps_qsound::QSound) -> Self {
        QSoundSide {
            opcodes,
            ram2: vec![0; 0x1000],
            dsp,
            irq: false,
            next_irq: TICKS_PER_QSOUND_IRQ,
            next_sample: 0,
            out: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.ram2.fill(0);
        self.dsp.reset();
        self.irq = false;
        self.next_irq = TICKS_PER_QSOUND_IRQ;
        self.next_sample = 0;
        self.out.clear();
    }

    /// IRQ périodique et échantillons du DSP jusqu'à `t` (exclu), avant tout accès du Z80 à
    /// l'instant `t` (qsound_w / qsound_r font stream->update() d'abord).
    pub fn advance(&mut self, t: Tick) {
        while self.next_irq <= t {
            self.irq = true;
            self.next_irq += TICKS_PER_QSOUND_IRQ;
        }
        while self.next_sample < t {
            self.dsp.update_sample();
            let [l, r] = self.dsp.output();
            self.out
                .push([f32::from(l) / 32768.0, f32::from(r) / 32768.0]);
            self.next_sample += TICKS_PER_QSOUND_SAMPLE;
        }
    }
}

/// Sound chips interface (YM2151 + OKI MSM6295).
pub trait SoundChips {
    fn ym_read(&mut self, offset: u8) -> u8; // offset 0/1
    fn ym_write(&mut self, offset: u8, data: u8);
    fn oki_read(&mut self) -> u8;
    fn oki_write(&mut self, data: u8);
    fn set_oki_pin7(&mut self, high: bool);
    fn irq(&self) -> bool; // sortie IRQ du YM2151
    fn advance(&mut self, t: cps_common::clock::Tick);
    fn reset(&mut self);
}

/// Z80 SoundBoard with memory map, banking, ports, and pin server.
#[derive(Clone, Debug)]
pub struct SoundBoard<C: SoundChips> {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub bank: u8,
    pub latch: [u8; 2],
    pub chips: C,
    pub time: cps_common::clock::Tick,
    /// Ticks par T-state du Z80 (3,58 MHz sur CPS1, 8 MHz sur QSound).
    pub z80_ticks: Tick,
    /// Carte QSound (None : carte CPS1 YM2151 + OKI).
    pub qsound: Option<Box<QSoundSide>>,
    /// CPS2 : Z80 tenu en reset par le 68000 (port 804040 bit 3). spec: docs/spec/05 §Son
    pub z80_reset_held: bool,
    /// Pin-server state (last pins, read data, write done).
    pub(crate) last_pins: Pins,
    pub(crate) read_data: Option<u8>,
    pub(crate) write_done: bool,
}

impl<C: SoundChips> SoundBoard<C> {
    /// État interne du serveur de broches (dernières broches, donnée lue, écriture faite), pour les savestates.
    pub fn port_state(&self) -> (u64, Option<u8>, bool) {
        (self.last_pins.0, self.read_data, self.write_done)
    }

    /// Restaure l'état produit par `port_state`.
    pub fn set_port_state(&mut self, st: (u64, Option<u8>, bool)) {
        self.last_pins = Pins(st.0);
        self.read_data = st.1;
        self.write_done = st.2;
    }

    pub fn new(rom: Vec<u8>, chips: C) -> Self {
        Self {
            rom,
            ram: vec![0; 0x800],
            bank: 0,
            latch: [0; 2],
            chips,
            time: 0,
            z80_ticks: cps_common::clock::TICKS_PER_Z80,
            qsound: None,
            z80_reset_held: false,
            last_pins: Pins(0),
            read_data: None,
            write_done: false,
        }
    }

    pub fn reset(&mut self) {
        self.bank = 0;
        self.latch = [0; 2];
        self.ram.fill(0);
        self.chips.reset();
        self.chips.set_oki_pin7(true);
        self.last_pins = Pins(0);
        self.read_data = None;
        self.write_done = false;
        self.z80_reset_held = false;
        if let Some(q) = self.qsound.as_mut() {
            q.reset();
        }
    }

    /// Carte QSound : Z80 à 8 MHz, 4 Kio de RAM partagée en C000, opcodes déchiffrés.
    pub fn with_qsound(mut self, side: QSoundSide) -> Self {
        self.ram = vec![0; 0x1000];
        self.z80_ticks = cps_common::clock::TICKS_PER_Z80_QSOUND;
        self.qsound = Some(Box::new(side));
        self
    }

    /// Échantillons stéréo QSound produits depuis le dernier appel (vide sur CPS1).
    pub fn take_qsound_samples(&mut self) -> Vec<[f32; 2]> {
        self.qsound
            .as_mut()
            .map_or_else(Vec::new, |q| std::mem::take(&mut q.out))
    }

    /// qsound_sub_map, lecture (`m1` : cycle de lecture d'opcode).
    fn qsound_read(&mut self, addr: u16, m1: bool) -> u8 {
        let q = self.qsound.as_ref().expect("carte QSound");
        match addr {
            0x0000..=0x7FFF => {
                let src = if m1 { &q.opcodes } else { &self.rom };
                src.get(usize::from(addr)).copied().unwrap_or(0xFF)
            }
            0x8000..=0xBFFF => {
                let off = 0x10000 + usize::from(self.bank) * 0x4000 + usize::from(addr - 0x8000);
                self.rom.get(off).copied().unwrap_or(0xFF)
            }
            0xC000..=0xCFFF => self.ram[usize::from(addr - 0xC000)],
            0xD007 => q.dsp.read(),
            0xF000..=0xFFFF => q.ram2[usize::from(addr - 0xF000)],
            _ => 0x00,
        }
    }

    /// qsound_sub_map, écriture.
    fn qsound_write(&mut self, addr: u16, v: u8) {
        let rom_len = self.rom.len();
        let q = self.qsound.as_mut().expect("carte QSound");
        match addr {
            0xC000..=0xCFFF => self.ram[usize::from(addr - 0xC000)] = v,
            0xD000..=0xD002 => q.dsp.write((addr - 0xD000) as u8, v),
            0xD003 => {
                // qsound_banksw_w (l.466-476)
                let mut bank = v & 0x0f;
                if 0x10000 + usize::from(bank) * 0x4000 >= rom_len {
                    bank = 0;
                }
                self.bank = bank;
            }
            0xF000..=0xFFFF => q.ram2[usize::from(addr - 0xF000)] = v,
            _ => {}
        }
    }

    pub fn set_latch(&mut self, n: usize, v: u8) {
        if n < 2 {
            self.latch[n] = v;
        }
    }

    /// Read a byte from the Z80 memory map.
    pub fn read(&mut self, addr: u16) -> u8 {
        if self.qsound.is_some() {
            return self.qsound_read(addr, false);
        }
        match addr {
            0x0000..=0x7FFF => {
                if (addr as usize) < self.rom.len() {
                    self.rom[addr as usize]
                } else {
                    0xFF
                }
            }
            0x8000..=0xBFFF => {
                let offset = 0x10000 + (self.bank as usize * 0x4000) + ((addr - 0x8000) as usize);
                if offset < self.rom.len() {
                    self.rom[offset]
                } else {
                    0xFF
                }
            }
            0xD000..=0xD7FF => self.ram[(addr - 0xD000) as usize],
            0xF000 | 0xF001 => self.chips.ym_read((addr & 1) as u8),
            0xF002 => self.chips.oki_read(),
            0xF008 => self.latch[0],
            0xF00A => self.latch[1],
            _ => 0x00,
        }
    }

    /// Write a byte to the Z80 memory map.
    pub fn write(&mut self, addr: u16, v: u8) {
        if self.qsound.is_some() {
            return self.qsound_write(addr, v);
        }
        match addr {
            0xD000..=0xD7FF => self.ram[(addr - 0xD000) as usize] = v,
            0xF000 | 0xF001 => self.chips.ym_write((addr & 1) as u8, v),
            0xF002 => self.chips.oki_write(v),
            0xF004 => {
                self.bank = v & 1;
            }
            0xF006 => {
                self.chips.set_oki_pin7((v & 1) == 1);
            }
            _ => {} // ignored
        }
    }

    /// Read from a Z80 I/O port (not connected on CPS1).
    pub fn input(&mut self, _port: u16) -> u8 {
        0x00
    }

    /// Write to a Z80 I/O port (not connected on CPS1).
    pub fn output(&mut self, _port: u16, _v: u8) {}

    /// IRQ level: from YM2151.
    pub fn irq_level(&self) -> bool {
        match self.qsound.as_ref() {
            Some(q) => q.irq,
            None => self.chips.irq(),
        }
    }

    /// Serve one T-state of the Z80 CPU via pin-level interaction.
    pub fn tick(&mut self, cpu: &mut Cpu) {
        // QSound : DSP et IRQ à jour avant tout accès de cet instant
        if let Some(q) = self.qsound.as_mut() {
            q.advance(self.time);
        }

        // CPS2 : Z80 tenu en reset, le temps avance sans instruction
        if self.z80_reset_held {
            self.time += self.z80_ticks;
            return;
        }

        // Build input pins for this tick based on previous outputs
        let mut pins_in = Pins(0);

        // Handle read requests from previous tick's outputs
        let last = self.last_pins;
        let mem_rd = last.has(Pins::MREQ) && last.has(Pins::RD);
        let io_rd = last.has(Pins::IORQ) && last.has(Pins::RD) && !last.has(Pins::M1);
        let ack = last.is_int_ack();

        if mem_rd || io_rd || ack {
            // Provide data requested by previous outputs
            let data = match self.read_data.take() {
                Some(d) => d,
                None => {
                    let d = if mem_rd && self.qsound.is_some() {
                        self.qsound_read(last.addr(), last.has(Pins::M1))
                    } else if mem_rd {
                        self.read(last.addr())
                    } else if io_rd {
                        self.input(last.addr())
                    } else {
                        0xFF // INT ACK returns opcode (not used in CPS1)
                    };
                    self.read_data = Some(d);
                    d
                }
            };
            pins_in = pins_in.set_data(data);
        } else {
            self.read_data = None;
        }

        // Handle write requests from previous tick's outputs
        let mem_wr = last.has(Pins::MREQ) && last.has(Pins::WR);
        let io_wr = last.has(Pins::IORQ) && last.has(Pins::WR);
        if mem_wr || io_wr {
            if !self.write_done {
                if mem_wr {
                    self.write(last.addr(), last.data());
                } else {
                    self.output(last.addr(), last.data());
                }
                self.write_done = true;
            }
        } else {
            self.write_done = false;
        }

        // Advance chips to current time before sampling INT
        match self.qsound.as_mut() {
            // irq0_line_hold : acquittement = ligne relâchée
            Some(q) if ack => q.irq = false,
            Some(_) => {}
            None => self.chips.advance(self.time),
        }

        // Forward interrupt lines (no WAIT, no BUSRQ on CPS1)
        if self.irq_level() {
            pins_in = pins_in.with(Pins::INT);
        }

        // Execute one T-state
        let out = cpu.tick(pins_in);
        self.last_pins = out;
        self.time += self.z80_ticks;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chips_stub::NullChips;

    fn make_board() -> SoundBoard<NullChips> {
        let rom = vec![0x00; 0x18000];
        let chips = NullChips::new();
        SoundBoard::new(rom, chips)
    }

    #[test]
    fn test_rom_read() {
        let mut board = make_board();
        board.rom[0x0123] = 0xAB;
        assert_eq!(board.read(0x0123), 0xAB);
    }

    #[test]
    fn test_banked_read() {
        let mut board = make_board();
        board.rom[0x10010] = 0xCD;
        board.rom[0x14010] = 0xEF;

        // Bank 0: read(0x8010) -> rom[0x10010]
        assert_eq!(board.read(0x8010), 0xCD);

        // Switch to bank 1
        board.write(0xF004, 1);
        assert_eq!(board.read(0x8010), 0xEF);
    }

    #[test]
    fn test_ram_rw() {
        let mut board = make_board();
        board.write(0xD7FF, 9);
        assert_eq!(board.read(0xD7FF), 9);
    }

    #[test]
    fn test_unmapped_read() {
        let mut board = make_board();
        assert_eq!(board.read(0xC000), 0x00);
    }

    #[test]
    fn test_latches() {
        let mut board = make_board();
        board.set_latch(1, 0x33);
        assert_eq!(board.read(0xF00A), 0x33);
    }

    #[test]
    fn test_z80_program_read_latch() {
        let mut board = make_board();

        // Program: LD A,(F008) ; LD (D000),A ; HALT
        let prog = [0x3A, 0x08, 0xF0, 0x32, 0x00, 0xD0, 0x76];
        board.rom[0..prog.len()].copy_from_slice(&prog);

        board.set_latch(0, 0x5A);

        let mut cpu = Cpu::new();
        for _ in 0..40 {
            board.tick(&mut cpu);
        }

        assert_eq!(board.ram[0], 0x5A);
        assert_eq!(board.time, 40 * cps_common::clock::TICKS_PER_Z80);
    }

    #[test]
    fn test_z80_program_write_ym() {
        let mut board = make_board();

        // Program: LD A,0x2B ; LD (F001),A ; HALT
        let prog = [0x3E, 0x2B, 0x32, 0x01, 0xF0, 0x76];
        board.rom[0..prog.len()].copy_from_slice(&prog);

        let mut cpu = Cpu::new();
        for _ in 0..40 {
            board.tick(&mut cpu);
        }

        assert!(board
            .chips
            .log
            .iter()
            .any(|(addr, val)| *addr == 0xF001 && *val == 0x2B));
    }

    #[test]
    fn test_z80_program_oki_pin7() {
        let mut board = make_board();

        // Program: LD A,0 ; LD (F006),A ; HALT
        let prog = [0x3E, 0x00, 0x32, 0x06, 0xF0, 0x76];
        board.rom[0..prog.len()].copy_from_slice(&prog);

        let mut cpu = Cpu::new();
        for _ in 0..40 {
            board.tick(&mut cpu);
        }

        assert!(board
            .chips
            .log
            .iter()
            .any(|(addr, val)| *addr == 0xF006 && *val == 0));
    }

    #[test]
    fn test_z80_interrupt() {
        let mut board = make_board();

        // Program: IM 1 ; EI ; HALT
        let prog = [0xED, 0x56, 0xFB, 0x76];
        board.rom[0..prog.len()].copy_from_slice(&prog);

        // ISR at 0x38: LD A,0x77 ; LD (D000),A ; HALT
        let isr = [0x3E, 0x77, 0x32, 0x00, 0xD0, 0x76];
        board.rom[0x38..0x38 + isr.len()].copy_from_slice(&isr);

        board.chips.irq = true;

        let mut cpu = Cpu::new();
        for _ in 0..200 {
            board.tick(&mut cpu);
        }

        assert_eq!(board.ram[0], 0x77);
    }
}
