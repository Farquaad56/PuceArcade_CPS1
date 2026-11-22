//! CPS1 Z80 SoundBoard: memory map, banking, ports, latches, pin server.
//! spec: docs/spec/01 §Z80

use puce_z80::{Cpu, Pins};

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
    /// Pin-server state (last pins, read data, write done).
    last_pins: Pins,
    read_data: Option<u8>,
    write_done: bool,
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
    }

    pub fn set_latch(&mut self, n: usize, v: u8) {
        if n < 2 {
            self.latch[n] = v;
        }
    }

    /// Read a byte from the Z80 memory map.
    pub fn read(&mut self, addr: u16) -> u8 {
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
        self.chips.irq()
    }

    /// Serve one T-state of the Z80 CPU via pin-level interaction.
    pub fn tick(&mut self, cpu: &mut Cpu) {
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
                    let d = if mem_rd {
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
        self.chips.advance(self.time);

        // Forward interrupt lines (no WAIT, no BUSRQ on CPS1)
        if self.irq_level() {
            pins_in = pins_in.with(Pins::INT);
        }

        // Execute one T-state
        let out = cpu.tick(pins_in);
        self.last_pins = out;
        self.time += cps_common::clock::TICKS_PER_Z80;
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
