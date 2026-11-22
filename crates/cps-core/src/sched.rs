//! Boucle de trame : le 68000 avance une horloge à la fois, le reste suit par catch-up.
//! spec: docs/ARCHITECTURE.md §Ordonnanceur

use crate::host68k::Host68k;
use puce_m68k::cpu::Cpu as Cpu68k;

impl super::machine::Machine {
    /// Exécute une trame complète (jusqu'au prochain VBLANK).
    pub fn run_frame(&mut self) {
        self.frame_done = false;

        loop {
            let (cpu, mut host): (&mut Cpu68k, Host68k<'_>) = self.split();
            cpu.step_clock(&mut host);
            if *host.frame_done {
                break;
            }
        }
        self.finish_instruction();
    }

    /// Termine l'instruction en cours (avant un savestate ou un instantané de débogueur).
    pub fn finish_instruction(&mut self) {
        while !self.cpu.at_instruction_boundary() {
            let (cpu, mut host): (&mut Cpu68k, Host68k<'_>) = self.split();
            cpu.step_clock(&mut host);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};
    use cps_common::clock::{TICKS_PER_FRAME, TICKS_VBLANK_START};

    /// Programme NOP infini : vecteurs SSP/PC puis BRA.S * (boucle sur place).
    fn nop_loop_machine() -> Machine {
        // Vecteurs de reset en 0x000000 (SSP = 0x00FF8000) et 0x000004 (PC = 0x400)
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]), // SSP
            (0x000004, &[0x00, 0x00, 0x04, 0x00]), // PC = 0x400
            (0x400, &[0x60, 0xFE]),                // BRA.S * - boucle infinie sur place
        ];

        let game = testutil::synthetic_game(&program);
        Machine::new(game)
    }

    #[test]
    fn nop_loop_runs_full_frame() {
        let mut m = nop_loop_machine();
        m.run_frame();

        // Une trame complète exécutée
        assert_eq!(m.frame_count(), 1);

        // now >= début du VBLANK de la première trame
        assert!(
            m.now >= TICKS_VBLANK_START,
            "now={} < {}",
            m.now,
            TICKS_VBLANK_START
        );

        // now < début du VBLANK + marge (30 cycles 68k) - pas trop au-delà
        let margin = 30 * 504;
        assert!(
            m.now < TICKS_VBLANK_START + margin,
            "now={} >= {}",
            m.now,
            TICKS_VBLANK_START + margin
        );

        // Carte son synchronisée : sound.time dans ]now-1408, now]
        assert!(
            m.sound.time <= m.now,
            "sound.time={} > now={}",
            m.sound.time,
            m.now
        );
        assert!(
            m.sound.time + 1408 > m.now,
            "sound.time+1408={} <= now={}",
            m.sound.time + 1408,
            m.now
        );
    }

    #[test]
    fn second_frame_advances_time() {
        let mut m = nop_loop_machine();
        m.run_frame();

        m.run_frame();

        // Deuxième trame : now >= début VBLANK + TICKS_PER_FRAME
        assert!(
            m.now >= TICKS_VBLANK_START + TICKS_PER_FRAME,
            "now={} < {}",
            m.now,
            TICKS_VBLANK_START + TICKS_PER_FRAME
        );
        assert_eq!(m.frame_count(), 2);
    }

    #[test]
    fn sound_latch_write_visible() {
        // Programme : écrit $AB dans le latch1 (0x800181) puis boucle
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]), // SSP
            (0x000004, &[0x00, 0x00, 0x04, 0x00]), // PC = 0x400
            // MOVE.B #$AB,($00800181).L : 13FC 00AB 0080 0181
            (0x400, &[0x13, 0xFC, 0x00, 0xAB, 0x00, 0x80, 0x01, 0x81]),
            // BRA.S * à 0x408
            (0x408, &[0x60, 0xFE]),
        ];

        let game = testutil::synthetic_game(&program);
        let mut m = Machine::new(game);
        m.run_frame();

        // Le latch doit avoir reçu la valeur
        assert_eq!(m.sound.latch[0], 0xAB, "latch[0] should be 0xAB");
    }

    #[test]
    fn deterministic_execution() {
        let mut m1 = nop_loop_machine();
        let mut m2 = nop_loop_machine();

        for _ in 0..3 {
            m1.run_frame();
            m2.run_frame();
        }

        // Même temps écoulé
        assert_eq!(m1.now, m2.now, "now differs");

        // Même état CPU complet
        assert_eq!(
            format!("{:?}", m1.cpu.state()),
            format!("{:?}", m2.cpu.state()),
            "CPU state differs"
        );
        // Boucle BRA.S en 0x400 : avec la prélecture de deux mots, le PC visible vaut 0x404.
        assert_eq!(
            m1.cpu.state().pc,
            0x404,
            "le programme doit boucler en 0x400"
        );

        // Même état Z80
        assert_eq!(m1.z80.regs.pc, m2.z80.regs.pc, "Z80 PC differs");
    }
}
