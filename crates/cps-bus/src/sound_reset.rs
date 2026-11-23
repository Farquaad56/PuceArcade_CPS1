//! CPS2 : ligne de reset du Z80 pilotée par le 68000 (port 804040, bit 3).
//! spec: docs/spec/05 §Port 804040, §Son ; source: ref/mame-cps2/cps2.cpp l.1121-1160
//! (cps2_eeprom_port_w : m_audiocpu->set_input_line(INPUT_LINE_RESET, ...))

use crate::sound::{SoundBoard, SoundChips};
use puce_z80::{Cpu, Pins};

impl<C: SoundChips> SoundBoard<C> {
    /// Tient (`held`) ou relâche le Z80. Passage de relâché à tenu : reset du CPU et du serveur
    /// de broches ; tant qu'il est tenu, `tick` avance le temps sans exécuter.
    pub fn set_z80_reset(&mut self, held: bool, cpu: &mut Cpu) {
        if held && !self.z80_reset_held {
            cpu.reset();
            self.last_pins = Pins(0);
            self.read_data = None;
            self.write_done = false;
        }
        self.z80_reset_held = held;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NullChips;
    use cps_common::clock::TICKS_PER_Z80;

    /// LD HL,D000 ; INC (HL) ; JR -3 : compteur en RAM (D000 = ram[0]).
    fn board() -> SoundBoard<NullChips> {
        let mut rom = vec![0u8; 0x18000];
        rom[..6].copy_from_slice(&[0x21, 0x00, 0xD0, 0x34, 0x18, 0xFD]);
        SoundBoard::new(rom, NullChips::new())
    }

    #[test]
    fn held_then_released() {
        let mut b = board();
        let mut cpu = Cpu::new();
        assert!(!b.z80_reset_held);
        b.set_z80_reset(true, &mut cpu);
        for _ in 0..1000 {
            b.tick(&mut cpu);
        }
        assert_eq!(b.ram[0], 0);
        assert_eq!(b.time, 1000 * TICKS_PER_Z80);
        b.set_z80_reset(false, &mut cpu);
        for _ in 0..200 {
            b.tick(&mut cpu);
        }
        assert!(b.ram[0] > 0);
    }

    #[test]
    fn reset_only_on_edge() {
        let mut b = board();
        let mut cpu = Cpu::new();
        for _ in 0..100 {
            b.tick(&mut cpu);
        }
        assert_ne!(cpu.regs.pc, 0);
        b.set_z80_reset(true, &mut cpu);
        assert_eq!(cpu.regs.pc, 0);
        cpu.regs.pc = 0x1234;
        b.set_z80_reset(true, &mut cpu);
        assert_eq!(cpu.regs.pc, 0x1234, "déjà tenu : pas de second reset");
    }

    #[test]
    fn board_reset_releases() {
        let mut b = board();
        let mut cpu = Cpu::new();
        b.set_z80_reset(true, &mut cpu);
        b.reset();
        assert!(!b.z80_reset_held);
    }
}
