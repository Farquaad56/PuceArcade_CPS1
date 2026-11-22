//! CPS1 Machine: construction, reset, accessors.
//! spec: docs/spec/01 §Reset, docs/ARCHITECTURE.md §Ordonnanceur

use cps_common::clock::{Tick, TICKS_VBLANK_START};
use cps_common::input::{Dips, InputState};
use cps_common::video::Framebuffer;
use puce_m68k::cpu::Cpu as Cpu68k;
use puce_z80::Cpu as CpuZ80;

/// CPS1 machine state.
pub struct Machine {
    pub cpu: Cpu68k,
    pub bus: cps_bus::CpsBus,
    pub z80: CpuZ80,
    pub sound: cps_bus::SoundBoard<cps_bus::Chips>,
    pub gfx: Vec<u8>,
    pub fb: Framebuffer,
    pub now: Tick,
    pub next_vblank: Tick,
    pub frame: u64,
    pub ipl1: bool,
    pub ipl2: bool,
    pub ticks_68k: u64,
    pub frame_done: bool,
    pub game: String,
    pub frame_start: Tick,
}

impl Machine {
    /// Construct machine from loaded game; calls reset().
    pub fn new(game: cps_cart::Game) -> Self {
        let bus = cps_bus::new_cps_bus(game.regions.maincpu.clone(), game.info.name, game.dips);
        let sound = cps_bus::SoundBoard::new(
            game.regions.audiocpu.clone(),
            cps_bus::Chips::new(game.regions.oki),
        );
        let gfx = game.regions.gfx;

        let mut machine = Machine {
            cpu: Cpu68k::default(),
            bus,
            z80: CpuZ80::new(),
            sound,
            gfx,
            fb: Framebuffer::new(),
            now: 0,
            next_vblank: TICKS_VBLANK_START,
            frame: 0,
            ipl1: false,
            ipl2: false,
            ticks_68k: game.ticks_per_68k,
            frame_done: false,
            game: game.info.name.to_string(),
            frame_start: 0,
        };
        machine.reset();
        machine
    }

    /// Full reset: bus, video, sound, CPUs, timing.
    pub fn reset(&mut self) {
        self.bus.reset();
        self.bus.video.reset();
        self.sound.reset();
        self.sound.time = 0;
        self.cpu = Cpu68k::default();
        self.cpu.reset();
        self.z80 = CpuZ80::new();
        self.z80.reset();
        self.now = 0;
        self.next_vblank = TICKS_VBLANK_START;
        self.frame = 0;
        self.ipl1 = false;
        self.ipl2 = false;
        self.frame_done = false;
        self.frame_start = 0;
    }

    /// Borrow the framebuffer.
    pub fn framebuffer(&self) -> &Framebuffer {
        &self.fb
    }

    /// Frames executed since reset.
    pub fn frame_count(&self) -> u64 {
        self.frame
    }

    /// Set input state and propagate to bus/video.
    pub fn set_inputs(&mut self, i: InputState) {
        self.bus.set_inputs(i);
    }

    /// Set DIP switches.
    pub fn set_dips(&mut self, d: Dips) {
        self.bus.dips = d;
    }

    /// Take mixed audio samples (duplicated to stereo).
    pub fn take_audio(&mut self) -> Vec<[f32; 2]> {
        let mono = self.sound.chips.take_samples();
        mono.into_iter().map(|m| [m, m]).collect()
    }
}

/// Test utilities for constructing synthetic games.
pub mod testutil {
    use cps_cart::dips::default_dips;
    use cps_cart::romset::find_game;

    /// Build a synthetic game with program blocks at given addresses.
    pub fn synthetic_game(program: &[(u32, &[u8])]) -> cps_cart::Game {
        let info = find_game("ghouls").expect("ghouls definition");

        // maincpu ROM: 0x400000 bytes, zeroed, with program blocks copied in.
        let mut maincpu = vec![0u8; 0x400000];
        for (addr, data) in program {
            let start = *addr as usize;
            maincpu[start..start + data.len()].copy_from_slice(data);
        }

        // gfx: minimal 0x8000 bytes of 0xFF.
        let gfx = vec![0xFFu8; 0x8000];

        // audiocpu ROM: 0x18000 bytes with HALT at address 0.
        let mut audiocpu = vec![0u8; 0x18000];
        audiocpu[0] = 0x76; // Z80 HALT

        // OKI ROM: 0x40000 bytes of 0xFF.
        let oki = vec![0xFFu8; 0x40000];

        cps_cart::Game {
            info,
            regions: cps_cart::game::Regions {
                maincpu,
                gfx,
                audiocpu,
                oki,
                stars: None,
            },
            ticks_per_68k: 504,
            dips: default_dips("ghouls"),
            cpsb_at_1c0: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};
    use cps_common::clock::TICKS_VBLANK_START;
    use cps_common::input::{Dips, InputState};

    fn make_machine() -> Machine {
        let game = testutil::synthetic_game(&[(0, &[0, 0, 0x80, 0, 0, 0, 4, 0])]);
        Machine::new(game)
    }

    #[test]
    fn construction_resets_state() {
        let m = make_machine();
        assert_eq!(m.now, 0);
        assert_eq!(m.next_vblank, TICKS_VBLANK_START);
        assert_eq!(m.frame, 0);
        assert!(!m.ipl1);
        assert!(!m.ipl2);
        assert!(!m.frame_done);
    }

    #[test]
    fn reset_clears_timing() {
        let mut m = make_machine();
        m.now = 12345;
        m.frame = 99;
        m.ipl1 = true;
        m.reset();
        assert_eq!(m.now, 0);
        assert_eq!(m.next_vblank, TICKS_VBLANK_START);
        assert_eq!(m.frame, 0);
        assert!(!m.ipl1);
    }

    #[test]
    fn set_inputs_propagates_to_bus() {
        let mut m = make_machine();
        m.set_inputs(InputState {
            in1: 0x1234,
            ..Default::default()
        });
        assert_eq!(m.bus.read16(0x800000, true, true), 0x1234);
    }

    #[test]
    fn set_dips_propagates_to_bus() {
        let mut m = make_machine();
        m.set_dips(Dips {
            dswa: 0x55,
            ..Default::default()
        });
        assert_eq!(m.bus.read16(0x80001A, true, true), 0x55FF);
    }

    #[test]
    fn take_audio_empty_on_fresh_machine() {
        let mut m = make_machine();
        let audio = m.take_audio();
        assert!(audio.is_empty());
    }

    #[test]
    fn framebuffer_and_frame_count() {
        let m = make_machine();
        assert_eq!(m.framebuffer().pixels.len(), 384 * 224);
        assert_eq!(m.frame_count(), 0);
    }
}
