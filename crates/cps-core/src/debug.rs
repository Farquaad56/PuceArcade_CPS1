//! Débogage (T503) : pas à pas du 68000 et lectures mémoire SANS effet de bord.
//! spec: docs/spec/01 §68000 et §Z80 (cartes mémoire)

use crate::host68k::Host68k;
use crate::machine::Machine;
use puce_m68k::cpu::Cpu as Cpu68k;

impl Machine {
    /// Exécute une instruction du 68000 (le Z80 et la vidéo suivent par catch-up).
    pub fn step_instruction(&mut self) {
        self.finish_instruction();
        // Au moins une horloge, puis jusqu'à la frontière d'instruction suivante.
        loop {
            let (cpu, mut host): (&mut Cpu68k, Host68k<'_>) = self.split();
            cpu.step_clock(&mut host);
            if self.cpu.at_instruction_boundary() {
                break;
            }
        }
    }

    /// Octet de l'espace 68000 : ROM, GFXRAM et RAM ; E/S, CPS-A/B et zones non mappées -> 0xFF.
    pub fn peek68k(&self, addr: u32) -> u8 {
        let a = (addr & 0xFF_FFFF) as usize;
        let hi = a & 1 == 0;
        let byte = |w: u16| if hi { (w >> 8) as u8 } else { w as u8 };
        match a {
            // spec: docs/spec/01 §68000 — ROM programme 000000-3FFFFF
            0..=0x3F_FFFF => self.bus.rom.get(a).copied().unwrap_or(0),
            // spec: docs/spec/01 §68000 — GFXRAM 900000-92FFFF
            0x90_0000..=0x92_FFFF => self
                .bus
                .video
                .gfxram
                .get((a - 0x90_0000) / 2)
                .map_or(0xFF, |w| byte(*w)),
            // spec: docs/spec/01 §68000 — RAM de travail FF0000-FFFFFF
            0xFF_0000..=0xFF_FFFF => self.bus.ram[a - 0xFF_0000],
            _ => 0xFF,
        }
    }

    /// Octet de l'espace Z80 : ROM, banque et RAM ; ports F000-F00A et zones non mappées -> 0xFF.
    pub fn peek_z80(&self, addr: u16) -> u8 {
        let s = &self.sound;
        match addr {
            // spec: docs/spec/01 §Z80 — ROM 0000-7FFF, banque 8000-BFFF, RAM D000-D7FF
            0x0000..=0x7FFF => s.rom.get(usize::from(addr)).copied().unwrap_or(0xFF),
            0x8000..=0xBFFF => {
                let off = 0x10000 + usize::from(s.bank) * 0x4000 + usize::from(addr - 0x8000);
                s.rom.get(off).copied().unwrap_or(0xFF)
            }
            0xD000..=0xD7FF => s.ram[usize::from(addr - 0xD000)],
            _ => 0xFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};

    fn nop_machine() -> Machine {
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]),
            (0x000004, &[0x00, 0x00, 0x04, 0x00]),
            (0x400, &[0x4E, 0x71, 0x4E, 0x71, 0x4E, 0x71, 0x60, 0xF8]), // NOP ×3 ; BRA.S 0x400
        ];
        Machine::new(testutil::synthetic_game(&program))
    }

    #[test]
    fn step_advances_one_nop() {
        let mut m = nop_machine();
        m.step_instruction(); // premier pas : fin du reset et 1re instruction
        let pc = m.cpu.state().pc;
        m.step_instruction();
        assert_eq!(m.cpu.state().pc, pc + 2);
        assert!(m.cpu.at_instruction_boundary());
    }

    #[test]
    fn peeks_have_no_side_effect() {
        let mut m = nop_machine();
        m.run_frame();
        m.bus.ram[0] = 0x12;
        let before = m.save_state();
        assert_eq!(m.peek68k(0xFF0000), 0x12);
        assert_eq!(m.peek68k(0x800000), 0xFF);
        assert_eq!(m.peek68k(0x400), 0x4E);
        assert_eq!(m.peek68k(0x401), 0x71);
        assert_eq!(m.peek_z80(0x0000), 0x76);
        assert_eq!(m.peek_z80(0xF008), 0xFF);
        assert_eq!(m.save_state(), before);
    }
}
