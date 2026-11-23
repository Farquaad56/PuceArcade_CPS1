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
    /// Région « stars » (Forgotten Worlds…), passée au rendu.
    pub stars: Option<Vec<u8>>,
    pub fb: Framebuffer,
    pub now: Tick,
    pub next_vblank: Tick,
    pub frame: u64,
    pub ipl1: bool,
    pub ipl2: bool,
    pub ticks_68k: u64,
    /// Dénominateur des ticks par cycle 68000 (ADR-007) : 1 (CPS1) ou 59 (CPS2).
    pub ticks_68k_den: u64,
    /// Reste fractionnaire des ticks du 68000 (< ticks_68k_den).
    pub tick_acc: u64,
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
        let mut bus = bus;
        // Carte QSound (cps1.cpp qsound, l.4013-4044) : Z80 8 MHz chiffré, DSP, EEPROM 93C46
        let sound = match (&game.qsound_dsp, game.audio_opcodes) {
            (Some(dsp), Some(ops)) => {
                bus.qsound = true;
                bus.user1 = game.regions.user1.clone();
                bus.eeprom = Some(cps_bus::Eeprom93c46::new());
                let dsp = cps_qsound::QSound::new(dsp, game.regions.qsound);
                sound.with_qsound(cps_bus::QSoundSide::new(ops, dsp))
            }
            _ => sound,
        };
        // forgottn : uPD4701 (cps1.cpp l.3949-3957) ; sf2ee : CPS-B en 8001C0 (l.14594-14601)
        if game.info.machine == "forgottn" {
            bus.dial = Some(cps_bus::Upd4701::new());
        }
        if game.cpsb_at_1c0 {
            bus.cpsb_base = 0x8001C0;
        }

        let mut machine = Machine {
            cpu: Cpu68k::default(),
            bus,
            z80: CpuZ80::new(),
            sound,
            gfx,
            stars: game.regions.stars,
            fb: Framebuffer::new(),
            now: 0,
            next_vblank: TICKS_VBLANK_START,
            frame: 0,
            ipl1: false,
            ipl2: false,
            ticks_68k: game.ticks_per_68k,
            ticks_68k_den: game.ticks_68k_den.max(1),
            tick_acc: 0,
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
        self.tick_acc = 0;
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

    /// Molettes (forgottn) : mouvement relatif du joueur 1 (X) et du joueur 2 (Y).
    pub fn add_dial(&mut self, dx: i32, dy: i32) {
        self.bus.add_dial(dx, dy);
    }

    /// Set DIP switches.
    pub fn set_dips(&mut self, d: Dips) {
        self.bus.dips = d;
    }

    /// Take mixed audio samples (duplicated to stereo).
    /// Fréquence des échantillons de `take_audio` : AUDIO_HZ (CPS1, mono dupliqué) ou
    /// cps_qsound::SAMPLE_HZ (QSound, stéréo).
    pub fn audio_rate(&self) -> f64 {
        if self.sound.qsound.is_some() {
            cps_qsound::SAMPLE_HZ
        } else {
            crate::AUDIO_HZ
        }
    }

    /// Son stéréo (QSound) ?
    pub fn audio_stereo(&self) -> bool {
        self.sound.qsound.is_some()
    }

    pub fn take_audio(&mut self) -> Vec<[f32; 2]> {
        if self.sound.qsound.is_some() {
            return self.sound.take_qsound_samples();
        }
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
                ..Default::default()
            },
            ticks_per_68k: 504,
            dips: default_dips("ghouls"),
            cpsb_at_1c0: false,
            audio_opcodes: None,
            qsound_dsp: None,
            ticks_68k_den: 1,
            cps2: None,
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

    /// ADR-007 : n appels de on_tick, `now` relevé après chacun.
    fn ticks(m: &mut Machine, n: usize) -> Vec<u64> {
        use puce_m68k::bus::Bus;
        (0..n)
            .map(|_| {
                let (_, mut h) = m.split();
                h.on_tick(puce_m68k::pins::Pins::default());
                *h.now
            })
            .collect()
    }

    #[test]
    fn fractional_68k_clock() {
        let mut g = testutil::synthetic_game(&[(0, &[0, 0, 0x80, 0, 0, 0, 4, 0])]);
        g.ticks_per_68k = 25200;
        g.ticks_68k_den = 59;
        let mut m = Machine::new(g);
        let t = ticks(&mut m, 59);
        assert_eq!(t[0], 427);
        assert_eq!(t[58], 25200);
        assert_eq!(m.tick_acc, 0);
    }

    #[test]
    fn cps1_clock_unchanged() {
        let mut m = make_machine();
        assert_eq!(ticks(&mut m, 10)[9], 5040);
        assert_eq!(m.tick_acc, 0);
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

    #[test]
    fn sf2ee_cpsb_at_1c0() {
        let mut g = testutil::synthetic_game(&[]);
        g.info = cps_cart::find_game("sf2ee").unwrap();
        g.cpsb_at_1c0 = true;
        let mut m = Machine::new(g);
        // spec: init_sf2ee — CPS-B-18 : id 0x0408 en offset 0x10 (cpsb_addr), donc 8001D0
        assert_eq!(m.bus.read16(0x8001D0, true, true), 0x0408);
        assert_eq!(
            m.bus.read16(0x800150, true, true),
            0,
            "800140-80017F non mappé"
        );
    }

    #[test]
    fn forgottn_has_dial_and_stars() {
        let mut g = testutil::synthetic_game(&[]);
        g.info = cps_cart::find_game("forgottn").unwrap();
        g.regions.stars = Some(vec![0x0F; 0x8000]);
        let mut m = Machine::new(g);
        assert!(m.bus.dial.is_some() && m.stars.is_some());
        m.add_dial(5, -3);
        assert_eq!(m.bus.read16(0x800052, true, true), 5);
        let s = m.save_state();
        m.add_dial(1, 1);
        m.load_state(&s).unwrap();
        assert_eq!(m.bus.read16(0x800052, true, true), 5);
        // savestate d'un jeu sans molette refusé
        let ghouls = Machine::new(testutil::synthetic_game(&[])).save_state();
        assert!(m.load_state(&ghouls).is_err());
    }
}
