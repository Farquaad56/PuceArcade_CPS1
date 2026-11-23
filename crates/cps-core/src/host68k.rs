//! Hôte du 68000 : bus cadencé à l'horloge et rattrapage (catch-up) de la carte son.
//! spec: docs/ARCHITECTURE.md §Ordonnanceur, docs/spec/00-clock-timing.md §Trame

use cps_common::clock::Tick;
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
    /// ADR-007 : `now` avance de ticks_68k / ticks_68k_den par cycle, reste dans `tick_acc`.
    pub ticks_68k_den: u64,
    pub tick_acc: &'a mut u64,
    pub frame_done: &'a mut bool,
    pub gfx: &'a [u8],
    pub stars: Option<&'a [u8]>,
    pub fb: &'a mut Framebuffer,
    pub frame_start: Tick,
    /// CPS2 : opcodes déchiffrés (ADR-006).
    pub opcodes: Option<&'a [u8]>,
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
                ticks_68k_den: self.ticks_68k_den,
                tick_acc: &mut self.tick_acc,
                frame_done: &mut self.frame_done,
                gfx: &self.gfx,
                stars: self.stars.as_deref(),
                fb: &mut self.fb,
                frame_start: self.frame_start,
                opcodes: self.opcodes.as_deref(),
            },
        )
    }
}

impl Host68k<'_> {
    /// Fait tourner le Z80 T-state par T-state jusqu'à `t` (jamais au-delà).
    pub(crate) fn z80_run_until(&mut self, t: Tick) {
        while self.sound.time + self.sound.z80_ticks <= t {
            self.sound.tick(self.z80);
        }
    }

    /// qsound_main_map : RAM partagées (F18000-F19FFF -> Z80 C000, F1E000-F1FFFF -> Z80 F000)
    /// et port EEPROMIN (F1C006). Lecture : octet | 0xFF00. None hors de ces plages.
    fn qsound_read(&mut self, a: u32) -> Option<u16> {
        let q = self.sound.qsound.as_ref()?;
        let a = a as usize;
        match a {
            0xF18000..=0xF19FFF => Some(0xff00 | u16::from(self.sound.ram[(a - 0xF18000) / 2])),
            0xF1E000..=0xF1FFFF => Some(0xff00 | u16::from(q.ram2[(a - 0xF1E000) / 2])),
            0xF1C006 | 0xF1C007 => {
                let e = self.bus.eeprom.as_ref()?;
                Some(u16::from(e.do_read(*self.now)))
            }
            _ => None,
        }
    }

    /// Écritures QSound (octet bas seulement, ACCESSING_BITS_0_7). true si l'adresse est servie.
    fn qsound_write(&mut self, a: u32, lds: bool, data: u16) -> bool {
        if self.sound.qsound.is_none() {
            return false;
        }
        let a = a as usize;
        let v = data as u8;
        match a {
            0xF18000..=0xF19FFF => {
                if lds {
                    self.sound.ram[(a - 0xF18000) / 2] = v;
                }
                true
            }
            0xF1E000..=0xF1FFFF => {
                if lds {
                    if let Some(q) = self.sound.qsound.as_mut() {
                        q.ram2[(a - 0xF1E000) / 2] = v;
                    }
                }
                true
            }
            0xF1C006 | 0xF1C007 => {
                if lds {
                    let now = *self.now;
                    if let Some(e) = self.bus.eeprom.as_mut() {
                        e.write_port(v, now);
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl Bus for Host68k<'_> {
    fn read(&mut self, addr: u32, fc: u8, uds: bool, lds: bool) -> u16 {
        // CPS2 : opcodes déchiffrés pour un accès programme (FC & 3 == 2) sous 0x400000
        // spec: docs/spec/05 §Chiffrement
        let a = (addr & 0xFF_FFFE) as usize;
        if let Some(ops) = self.opcodes {
            if fc & 3 == 2 && a < 0x400000 {
                return ops
                    .get(a..a + 2)
                    .map_or(0, |w| u16::from_be_bytes([w[0], w[1]]));
            }
        }
        if self.bus.cps2.is_some() {
            if let Some(v) = self.cps2_read(a as u32) {
                return v;
            }
            return self.bus.read16(addr, uds, lds);
        }
        if let Some(v) = self.qsound_read(addr & 0xFF_FFFE) {
            return v;
        }
        self.bus.read16(addr, uds, lds)
    }

    fn write(&mut self, addr: u32, _fc: u8, uds: bool, lds: bool, data: u16) {
        if self.bus.cps2.is_some() {
            if !self.cps2_write(addr & 0xFF_FFFE, uds, lds, data) {
                self.bus.write16(addr, uds, lds, data);
            }
            return;
        }
        if self.qsound_write(addr & 0xFF_FFFE, lds, data) {
            return;
        }
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
        // Avancer l'horloge du 68000 : accumulateur entier (ADR-007), exact avec den = 1
        *self.tick_acc += self.ticks_68k;
        *self.now += *self.tick_acc / self.ticks_68k_den;
        *self.tick_acc %= self.ticks_68k_den;

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
