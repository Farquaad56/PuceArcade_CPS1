//! `Cps1Board` : la machine CPS1 derrière le connecteur JAMMA (puce-jamma).
//! T500b (BoardInfo, trame, vidéo, son, entrées) ; T501 (DIP, savestates).

use cps_cart::{dip_fields, input_layout, Game, GameInfo};
use cps_common::Dips;
use cps_core::Machine;
use puce_jamma::audio::AudioInfo;
use puce_jamma::board::{BoardInfo, JammaBoard};
use puce_jamma::debug::BoardDebug;
use puce_jamma::dips::{DipBank, DipSetting, DipSwitch};
use puce_jamma::inputs::JammaInputs;
use puce_jamma::video::{Rotation, VideoInfo, VideoOut};

use crate::inputs::to_cps_inputs_for;

/// Image visible du CPS1. spec: docs/spec/02 §Écran — 384 × 224 (x 64..447, y 16..239)
pub const SCREEN_W: u32 = 384;
pub const SCREEN_H: u32 = 224;

pub struct Cps1Board {
    pub machine: Machine,
    info: BoardInfo,
    audio_mono: Vec<f32>,
}

/// Description du jeu pour le frontal.
pub fn board_info(info: &GameInfo, dips: Dips) -> BoardInfo {
    let (players, buttons) = input_layout(info.input);
    let rotation = if info.rotation == 270 {
        Rotation::Rot270
    } else {
        Rotation::Rot0
    };
    let aspect = if rotation == Rotation::Rot0 {
        (4, 3)
    } else {
        (3, 4)
    };
    let banks = [
        ("SW(A)", dips.dswa),
        ("SW(B)", dips.dswb),
        ("SW(C)", dips.dswc),
    ];
    BoardInfo {
        system: "Capcom CPS1".into(),
        name: info.name.into(),
        title: info.title.into(),
        players,
        buttons,
        video: VideoInfo {
            width: SCREEN_W,
            height: SCREEN_H,
            refresh_hz: cps_core::FRAME_HZ,
            aspect,
            rotation,
        },
        // QSound : stéréo à 24 038 Hz ; CPS1 : haut-parleur mono à 55 930 Hz
        audio: if info.machine == "qsound" {
            AudioInfo {
                sample_rate: cps_core::QSOUND_HZ,
                channels: 2,
            }
        } else {
            AudioInfo {
                sample_rate: cps_core::AUDIO_HZ,
                channels: 1,
            }
        },
        dip_banks: banks
            .iter()
            .map(|(n, d)| DipBank {
                name: (*n).into(),
                default: *d,
            })
            .collect(),
        dips: dip_fields(info.input)
            .iter()
            .map(|f| DipSwitch {
                bank: usize::from(f.port),
                mask: f.mask,
                label: f.label.into(),
                default: f.default,
                settings: f
                    .settings
                    .iter()
                    .map(|(l, v)| DipSetting {
                        label: (*l).into(),
                        value: *v,
                    })
                    .collect(),
            })
            .collect(),
        has_analog: info.machine == "forgottn",
    }
}

impl Cps1Board {
    pub fn new(game: Game) -> Cps1Board {
        let info = board_info(game.info, game.dips);
        Cps1Board {
            machine: Machine::new(game),
            info,
            audio_mono: Vec::new(),
        }
    }
}

impl JammaBoard for Cps1Board {
    fn info(&self) -> &BoardInfo {
        &self.info
    }

    /// Reset de la machine ; les DIP en cours sont conservés.
    fn reset(&mut self) {
        let dips = self.machine.bus.dips;
        self.machine.reset();
        self.machine.set_dips(dips);
    }

    fn run_frame(&mut self) {
        self.machine.run_frame();
    }

    fn frame_count(&self) -> u64 {
        self.machine.frame_count()
    }

    fn video(&self) -> VideoOut<'_> {
        VideoOut {
            info: self.info.video,
            pixels: &self.machine.framebuffer().pixels,
        }
    }

    /// CPS1 : haut-parleur mono (voie gauche du mélange, les deux voies sont identiques) ;
    /// QSound : stéréo entrelacé gauche, droite.
    fn take_audio(&mut self, out: &mut Vec<f32>) {
        let samples = self.machine.take_audio();
        if self.info.audio.channels == 2 {
            out.extend(samples.iter().flatten());
            return;
        }
        self.audio_mono.clear();
        self.audio_mono.extend(samples.iter().map(|s| s[0]));
        out.extend_from_slice(&self.audio_mono);
    }

    fn set_inputs(&mut self, inputs: &JammaInputs) {
        self.machine
            .set_inputs(to_cps_inputs_for(inputs, self.info.players));
    }

    fn dips(&self) -> Vec<u8> {
        let d = self.machine.bus.dips;
        vec![d.dswa, d.dswb, d.dswc]
    }

    fn set_dips(&mut self, banks: &[u8]) {
        if let [dswa, dswb, dswc] = *banks {
            self.machine.set_dips(Dips { dswa, dswb, dswc });
        }
    }

    fn save_state(&mut self) -> Vec<u8> {
        self.machine.save_state()
    }

    fn load_state(&mut self, data: &[u8]) -> Result<(), String> {
        self.machine.load_state(data)
    }

    fn debug(&mut self) -> Option<&mut dyn BoardDebug> {
        Some(self)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use cps_core::machine::testutil;

    /// Programme minimal : vecteurs de reset, IRQ2 activée, boucle.
    pub(crate) fn synthetic_board() -> Cps1Board {
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]),
            (0x000004, &[0x00, 0x00, 0x04, 0x00]),
            (0x400, &[0x60, 0xFE]),
        ];
        Cps1Board::new(testutil::synthetic_game(&program))
    }

    #[test]
    fn board_info_of_ghouls() {
        let b = synthetic_board();
        let i = b.info();
        assert_eq!(
            i.video,
            VideoInfo {
                width: 384,
                height: 224,
                refresh_hz: cps_core::FRAME_HZ,
                aspect: (4, 3),
                rotation: Rotation::Rot0
            }
        );
        assert_eq!((i.players, i.buttons), (2, 2));
        assert_eq!(i.system, "Capcom CPS1");
        assert_eq!(i.dip_banks.len(), 3);
        assert!(!i.dips.is_empty());
        assert!(i.dips.iter().all(|d| d.bank < 3 && !d.settings.is_empty()));
    }

    #[test]
    fn frames_video_and_audio() {
        let mut b = synthetic_board();
        b.run_frame();
        let mut a = Vec::new();
        b.take_audio(&mut a);
        // Une trame du cœur = TICKS_PER_FRAME (84 510 720) / 90 112 = 937,8 échantillons
        // (la carte T500b disait 859-860 : voir docs/QUESTIONS.md)
        assert!((937..=938).contains(&a.len()), "{}", a.len());
        b.run_frame();
        b.run_frame();
        assert_eq!(b.frame_count(), 3);
        assert_eq!(b.video().pixels.len(), 86_016);
    }

    #[test]
    fn vertical_game_is_rotated() {
        let info = cps_cart::find_game("1941").unwrap();
        let i = board_info(info, Dips::default());
        assert_eq!(i.video.rotation, Rotation::Rot270);
        assert_eq!(i.video.aspect, (3, 4));
    }

    #[test]
    fn dips_round_trip() {
        let mut b = synthetic_board();
        b.set_dips(&[0xFE, 0xFD, 0x7F]);
        assert_eq!(b.dips(), vec![0xFE, 0xFD, 0x7F]);
        // spec: docs/spec/01 §68000 — 0x80001E : DSWC dans l'octet haut
        assert_eq!(b.machine.bus.read16(0x80001E, true, true), 0x7FFF);
        b.set_dips(&[1, 2]);
        assert_eq!(
            b.dips(),
            vec![0xFE, 0xFD, 0x7F],
            "taille incorrecte ignorée"
        );
        b.reset();
        assert_eq!(b.dips(), vec![0xFE, 0xFD, 0x7F], "reset conserve les DIP");
    }

    #[test]
    fn savestate_through_the_board() {
        let mut b = synthetic_board();
        b.run_frame();
        let s = b.save_state();
        for _ in 0..10 {
            b.run_frame();
        }
        let crc = b.machine.framebuffer().crc32();
        b.load_state(&s).unwrap();
        for _ in 0..10 {
            b.run_frame();
        }
        assert_eq!(b.machine.framebuffer().crc32(), crc);
        assert!(b.load_state(b"bad").is_err());
    }
}
