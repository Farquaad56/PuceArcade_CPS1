//! `BoardDebug` de la carte CPS1 : processeurs, mémoire, pas à pas (T503b) ;
//! registres, images et couches dans `debug_video` (T504).

use puce_jamma::debug::{BoardDebug, CpuView, DebugImage};

use crate::board::Cps1Board;
use crate::debug_video;

/// Garde-fou du pas à pas Z80 : nombre maximal d'instructions 68000.
const Z80_STEP_LIMIT: usize = 1000;

impl Cps1Board {
    fn view_68k(&self) -> CpuView {
        let s = self.machine.cpu.state();
        let mut registers: Vec<(String, u32)> = (0..8).map(|i| (format!("D{i}"), s.d[i])).collect();
        // A7 = pile active (SSP en mode superviseur, USP sinon)
        let a7 = if s.sr & 0x2000 != 0 { s.ssp } else { s.usp };
        registers.extend((0..7).map(|i| (format!("A{i}"), s.a[i])));
        registers.push(("A7".into(), a7));
        registers.push(("SR".into(), u32::from(s.sr)));
        registers.push(("SSP".into(), s.ssp));
        registers.push(("USP".into(), s.usp));
        CpuView {
            name: "68000".into(),
            pc: s.pc,
            registers,
            address_bits: 24,
        }
    }

    fn view_z80(&self) -> CpuView {
        let r = &self.machine.z80.regs;
        let pair = |h: u8, l: u8| u32::from(h) << 8 | u32::from(l);
        let registers = vec![
            ("AF".into(), pair(r.a, r.f)),
            ("BC".into(), pair(r.b, r.c)),
            ("DE".into(), pair(r.d, r.e)),
            ("HL".into(), pair(r.h, r.l)),
            ("IX".into(), u32::from(r.ix)),
            ("IY".into(), u32::from(r.iy)),
            ("SP".into(), u32::from(r.sp)),
            ("AF'".into(), pair(r.a2, r.f2)),
            ("BC'".into(), pair(r.b2, r.c2)),
            ("DE'".into(), pair(r.d2, r.e2)),
            ("HL'".into(), pair(r.h2, r.l2)),
            ("I".into(), u32::from(r.i)),
            ("R".into(), u32::from(r.r)),
        ];
        CpuView {
            name: "Z80".into(),
            pc: u32::from(r.pc),
            registers,
            address_bits: 16,
        }
    }
}

impl BoardDebug for Cps1Board {
    fn cpus(&self) -> Vec<CpuView> {
        vec![self.view_68k(), self.view_z80()]
    }

    fn peek(&self, cpu: usize, addr: u32) -> u8 {
        match cpu {
            0 => self.machine.peek68k(addr),
            1 => self.machine.peek_z80(addr as u16),
            _ => 0xFF,
        }
    }

    fn step_instruction(&mut self, cpu: usize) {
        match cpu {
            0 => self.machine.step_instruction(),
            1 => {
                let pc = self.machine.z80.regs.pc;
                for _ in 0..Z80_STEP_LIMIT {
                    self.machine.step_instruction();
                    if self.machine.z80.regs.pc != pc {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    fn registers(&self) -> Vec<(String, Vec<(String, String)>)> {
        debug_video::registers(&self.machine)
    }

    fn images(&self) -> Vec<DebugImage> {
        debug_video::images(&self.machine)
    }

    fn layer_names(&self) -> Vec<String> {
        debug_video::LAYER_NAMES
            .iter()
            .map(|s| (*s).into())
            .collect()
    }

    fn set_layer_mask(&mut self, mask: u32) {
        self.machine.bus.video.debug_layer_mask = mask;
    }
}

#[cfg(test)]
mod tests {
    use crate::board::tests::synthetic_board;
    use puce_jamma::board::JammaBoard;
    use puce_jamma::debug::BoardDebug;

    #[test]
    fn debug_cpus_and_memory() {
        let mut b = synthetic_board();
        b.run_frame();
        let c = b.cpus();
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].name, "68000");
        assert!(c[0].registers.iter().any(|(n, _)| n == "D0"));
        assert!(c[0].registers.iter().any(|(n, _)| n == "SR"));
        assert_eq!(c[1].name, "Z80");
        assert_eq!((c[0].address_bits, c[1].address_bits), (24, 16));
        assert_eq!(b.peek(0, 0x400), 0x60);
        assert_eq!(b.peek(1, 0), b.machine.peek_z80(0));
        assert_eq!(b.peek(7, 0), 0xFF);
    }

    #[test]
    fn debug_step_68000() {
        let mut b = synthetic_board();
        b.run_frame();
        b.step_instruction(0);
        let pc = b.cpus()[0].pc;
        b.step_instruction(0);
        // BRA.S * : le PC (prélecture comprise) reste identique d'un pas à l'autre
        assert_eq!(b.cpus()[0].pc, pc);
        assert!((0x400..=0x404).contains(&pc), "{pc:X}");
        b.step_instruction(1);
        b.debug().unwrap();
    }
}
