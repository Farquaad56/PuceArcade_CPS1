// Tests de fumée des crates CPU (puce-m68k, puce-z80) — contrat d'utilisation.
// Valeurs mesurées sur les révisions épinglées dans Cargo.toml (identiques à Puce16 Neo Geo T005).

use puce_m68k::bus::Bus;
use puce_m68k::cpu::Cpu as M68K;
use puce_z80::{Bus as Z80Bus, Cpu as Z80};

// ──────────────────────────── 68000 : machine test ────────────────────────────

struct TestMachine {
    ram: [u8; 0x10000], // 64 KiB miroir
    ticks: u32,
}

impl TestMachine {
    fn new() -> Self {
        let mut m = Self {
            ram: [0; 0x10000],
            ticks: 0,
        };
        // Vecteurs : SSP=0x1000 (octets 00 00 10 00), PC=0x100 (00 00 01 00)
        m.ram[0] = 0;
        m.ram[1] = 0;
        m.ram[2] = 0x10;
        m.ram[3] = 0; // SSP high byte at offset 2, low at 3 (big-endian)
        m.ram[4] = 0;
        m.ram[5] = 0;
        m.ram[6] = 0x01;
        m.ram[7] = 0; // PC high byte at offset 6, low at 7 (big-endian)

        // Programme à 0x100 : MOVE.W #$1234,D0 ; STOP #$2700
        let prog: [u8; 6] = [0x30, 0x3C, 0x12, 0x34, 0x4E, 0x72];
        m.ram[0x100..0x106].copy_from_slice(&prog);
        // STOP #$2700 : le mot suivant est l'immédiat
        m.ram[0x106] = 0x27;
        m.ram[0x107] = 0x00;

        m
    }
}

impl Bus for TestMachine {
    fn read(&mut self, addr: u32, _fc: u8, _uds: bool, _lds: bool) -> u16 {
        let a = (addr as usize) & 0xFFFE;
        u16::from_be_bytes([self.ram[a], self.ram[a + 1]])
    }

    fn write(&mut self, addr: u32, _fc: u8, uds: bool, lds: bool, data: u16) {
        let a = (addr as usize) & 0xFFFE;
        if uds {
            self.ram[a] = (data >> 8) as u8;
        }
        if lds {
            self.ram[a + 1] = data as u8;
        }
    }

    fn on_tick(&mut self, _pins: puce_m68k::pins::Pins) {
        self.ticks += 1;
    }
}

// ──────────────────────────── Test 1 : reset + MOVE.W + STOP ─────────────────

#[test]
fn m68k_reset_move_stop() {
    let mut machine = TestMachine::new();
    let mut cpu = M68K::new();

    // Démarrer la séquence de reset (40 horloges)
    cpu.reset();

    // Boucle step_clock : première frontière d'instruction après 40 ticks
    loop {
        let _ = cpu.step_clock(&mut machine);
        if cpu.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(machine.ticks, 40, "reset doit prendre exactement 40 ticks");
    assert_eq!(
        cpu.state().pc,
        0x104,
        "PC après reset doit être 0x104 (après fetch du vecteur PC)"
    );
    assert_eq!(cpu.state().ssp, 0x1000, "SSP doit être initialisé à 0x1000");

    // Exécuter MOVE.W #$1234,D0 : 8 ticks supplémentaires (total 48)
    loop {
        let _ = cpu.step_clock(&mut machine);
        if cpu.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(
        machine.ticks, 48,
        "MOVE.W #$1234,D0 doit prendre 8 ticks (total 48)"
    );
    assert_eq!(
        cpu.state().d[0],
        0x1234,
        "D0 doit contenir 0x1234 après MOVE.W"
    );

    // Exécuter STOP #$2700 : atteint la frontière suivante puis is_stopped()
    loop {
        let _ = cpu.step_clock(&mut machine);
        if cpu.at_instruction_boundary() {
            break;
        }
    }
    assert!(cpu.is_stopped(), "CPU doit être arrêté après STOP");

    // Après STOP, chaque step_clock prend 4 ticks (cycle de bus vide)
    let ticks_before = machine.ticks;
    loop {
        let _ = cpu.step_clock(&mut machine);
        if cpu.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(
        machine.ticks - ticks_before,
        4,
        "cycle après STOP doit prendre 4 ticks"
    );
}

// ──────────────────────────── Test 2 : attente (wait states) ─────────────────

struct WaitMachine {
    ram: [u8; 0x10000],
    ticks: u32,
    wait_addr: u32,
    wait_cycles: u8,
}

impl WaitMachine {
    fn new() -> Self {
        let mut m = Self {
            ram: [0; 0x10000],
            ticks: 0,
            wait_addr: 0x200,
            wait_cycles: 0,
        };
        // Vecteurs SSP=0x1000, PC=0x100 (même layout que TestMachine)
        m.ram[0] = 0;
        m.ram[1] = 0;
        m.ram[2] = 0x10;
        m.ram[3] = 0; // SSP low byte
        m.ram[4] = 0;
        m.ram[5] = 0;
        m.ram[6] = 0x01;
        m.ram[7] = 0; // PC low byte

        // Programme : MOVE.W $200,D1 (lit l'adresse qui déclenche l'attente)
        let prog: [u8; 6] = [0x32, 0x39, 0x00, 0x00, 0x02, 0x00];
        m.ram[0x100..0x106].copy_from_slice(&prog);

        // STOP après pour arrêter proprement
        let stop: [u8; 4] = [0x4E, 0x72, 0x27, 0x00];
        m.ram[0x106..0x10A].copy_from_slice(&stop);

        m
    }
}

impl Bus for WaitMachine {
    fn read(&mut self, addr: u32, _fc: u8, _uds: bool, _lds: bool) -> u16 {
        let a = (addr as usize) & 0xFFFE;
        u16::from_be_bytes([self.ram[a], self.ram[a + 1]])
    }

    fn write(&mut self, addr: u32, _fc: u8, uds: bool, lds: bool, data: u16) {
        let a = (addr as usize) & 0xFFFE;
        if uds {
            self.ram[a] = (data >> 8) as u8;
        }
        if lds {
            self.ram[a + 1] = data as u8;
        }
    }

    fn wait_clocks(&mut self, addr: u32, _write: bool) -> u8 {
        if addr == self.wait_addr {
            return self.wait_cycles;
        }
        0
    }

    fn on_tick(&mut self, _pins: puce_m68k::pins::Pins) {
        self.ticks += 1;
    }
}

#[test]
fn m68k_wait_states() {
    // Sans attente : MOVE.W $200,D1 prend 16 ticks (4 cycles de bus × 4 ticks)
    let mut machine = WaitMachine::new();
    machine.wait_cycles = 0;
    let mut cpu = M68K::new();
    cpu.reset();

    // Attendre la fin du reset (première frontière après 40 ticks)
    loop {
        let _ = cpu.step_clock(&mut machine);
        if cpu.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(machine.ticks, 40, "reset doit prendre 40 ticks");

    // Exécuter MOVE.W $200,D1 (deuxième frontière)
    loop {
        let _ = cpu.step_clock(&mut machine);
        if cpu.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(
        machine.ticks, 56,
        "MOVE.W sans attente : +16 ticks (total 56)"
    );

    // Avec attente de 2 horloges sur l'adresse 0x200 : +2 ticks exactement
    let mut machine_wait = WaitMachine::new();
    machine_wait.wait_cycles = 2;
    let mut cpu_wait = M68K::new();
    cpu_wait.reset();

    // Reset (40 ticks)
    loop {
        let _ = cpu_wait.step_clock(&mut machine_wait);
        if cpu_wait.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(machine_wait.ticks, 40, "reset doit prendre 40 ticks");

    // MOVE.W avec attente (18 ticks au lieu de 16)
    loop {
        let _ = cpu_wait.step_clock(&mut machine_wait);
        if cpu_wait.at_instruction_boundary() {
            break;
        }
    }
    assert_eq!(
        machine_wait.ticks, 58,
        "MOVE.W avec attente de 2 : +18 ticks (total 58)"
    );
}

// ──────────────────────────── Test 3 : Z80 step API ──────────────────────────

struct Z80Machine {
    ram: [u8; 65536],
}

impl Z80Machine {
    fn new() -> Self {
        let mut m = Self { ram: [0; 65536] };
        // Programme : LD A,1 ; ADD A,A ; HALT
        let prog: [u8; 4] = [0x3E, 0x01, 0x87, 0x76];
        m.ram[..4].copy_from_slice(&prog);
        m
    }
}

impl Z80Bus for Z80Machine {
    fn read(&mut self, addr: u16) -> u8 {
        self.ram[usize::from(addr)]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.ram[usize::from(addr)] = data;
    }
}

#[test]
fn z80_step_api() {
    let mut machine = Z80Machine::new();
    let mut cpu = Z80::new();
    cpu.reset();

    // 3 instructions : LD A,1 (4 T-states) + ADD A,A (4 T-states) + HALT (7 T-states) = 15 total
    let mut tstates: u32 = 0;
    for _ in 0..3 {
        if cpu.at_instruction_boundary() || tstates == 0 {
            tstates += cpu.step(&mut machine);
        } else {
            break;
        }
    }

    assert_eq!(cpu.regs.a, 2, "A doit être 2 après LD A,1 ; ADD A,A");
    assert_eq!(tstates, 15, "Total T-states doit être 15 (4+4+7)");
}
