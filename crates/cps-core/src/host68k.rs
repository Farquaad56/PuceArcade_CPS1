//! Hôte du 68000 : bus cadencé à l'horloge et rattrapage (catch-up) de la carte son.
//! spec: docs/ARCHITECTURE.md §Ordonnanceur, docs/spec/00-clock-timing.md §Trame

use cps_common::clock::{Tick, TICKS_PER_Z80};
use cps_common::video::Framebuffer;
use puce_m68k::bus::Bus;
use puce_m68k::cpu::Cpu as Cpu68k;
use puce_m68k::pins::Pins;

/// Références disjointes vers les champs de `Machine` autres que `cpu`.
pub struct Host68k<'a> {
    pub bus: &'a mut cps_bus::CpsBus,
    pub sound: &'a mut cps_bus::SoundBoard<cps_bus::Chips>,
    pub z80: &'a mut puce_z80::Cpu,
    pub now: &'a mut Tick,
    pub next_vblank: &'a mut Tick,
    pub frame: &'a mut u64,
    pub ipl1: &'a mut bool,
    pub ipl2: &'a mut bool,
    pub ticks_68k: u64,
    pub frame_done: &'a mut bool,
    pub gfx: &'a [u8],
    pub fb: &'a mut Framebuffer,
    pub frame_start: Tick,
}

impl super::machine::Machine {
    /// Sépare le CPU de l'hôte (qui emprunte tout le reste).
    pub fn split(&mut self) -> (&mut Cpu68k, Host68k<'_>) {
        (
            &mut self.cpu,
            Host68k {
                bus: &mut self.bus,
                sound: &mut self.sound,
                z80: &mut self.z80,
                now: &mut self.now,
                next_vblank: &mut self.next_vblank,
                frame: &mut self.frame,
                ipl1: &mut self.ipl1,
                ipl2: &mut self.ipl2,
                ticks_68k: self.ticks_68k,
                frame_done: &mut self.frame_done,
                gfx: &self.gfx,
                fb: &mut self.fb,
                frame_start: self.frame_start,
            },
        )
    }
}

impl Host68k<'_> {
    /// Fait tourner le Z80 T-state par T-state jusqu'à `t` (jamais au-delà).
    fn z80_run_until(&mut self, t: Tick) {
        while self.sound.time + TICKS_PER_Z80 <= t {
            self.sound.tick(self.z80);
        }
    }
}

impl Bus for Host68k<'_> {
    fn read(&mut self, addr: u32, _fc: u8, uds: bool, lds: bool) -> u16 {
        self.bus.read16(addr, uds, lds)
    }

    fn write(&mut self, addr: u32, _fc: u8, uds: bool, lds: bool, data: u16) {
        // Écriture bus normale - capture le SoundCmd si latch écrit
        let cmd = self.bus.write16(addr, uds, lds, data);

        // Traiter la commande son retournée par le bus
        match cmd {
            Some(cps_bus::SoundCmd::Latch1(v)) => self.sound.set_latch(0, v),
            Some(cps_bus::SoundCmd::Latch2(v)) => self.sound.set_latch(1, v),
            _ => {}
        }
    }

    fn wait_clocks(&mut self, _addr: u32, _write: bool) -> u8 {
        // Q-WAIT : 0 partout (ADR-001)
        0
    }

    fn ipl(&mut self) -> u8 {
        match (*self.ipl1, *self.ipl2) {
            (false, false) => 0,
            (true, false) => 2,
            (false, true) => 4,
            (true, true) => 6,
        }
    }

    fn int_ack(&mut self, _level: u8) -> Option<u8> {
        // Autovecteur : effacer les drapeaux IPL, pas de vecteur retourné
        *self.ipl1 = false;
        *self.ipl2 = false;
        None
    }

    fn on_tick(&mut self, _pins: Pins) {
        // Avancer l'horloge du 68000
        *self.now += self.ticks_68k;

        // Catch-up de la carte son (Z80 + YM2151 + OKI)
        self.z80_run_until(*self.now);

        // Événement VBLANK si atteint (ligne 240)
        if *self.now >= *self.next_vblank {
            crate::vblank::vblank_event(self);
        }

        // Fin de trame : TICKS_PER_FRAME ticks après le début de cette trame
        let frame_end = self.frame_start + cps_common::clock::TICKS_PER_FRAME;
        if *self.now >= frame_end {
            *self.frame_done = true;
        }
    }
}
