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
    /// Fichier de l'EEPROM (`<saves>/<jeu>.nv`), relu au chargement et écrit à la fermeture,
    /// comme la nvram de MAME (cartes QSound).
    nv_path: Option<std::path::PathBuf>,
    /// Disposition des entrées du jeu (`GameInfo::input`), pour les ports CPS2.
    input: &'static str,
    /// Débogueur : voies enregistrées (onglet Son ouvert) et voies coupées.
    pub(crate) voice_scope: bool,
    pub(crate) voice_mute: u64,
}

impl Cps1Board {
    /// Relie l'EEPROM à un fichier : contenu relu s'il existe (128 octets), écrit à la fermeture.
    pub fn with_nvram(mut self, path: std::path::PathBuf) -> Self {
        if let Some(e) = self.machine.bus.eeprom.as_mut() {
            if let Ok(d) = std::fs::read(&path) {
                if d.len() == e.data.len() {
                    e.data = d;
                }
            }
            self.nv_path = Some(path);
        }
        self
    }
}

impl Drop for Cps1Board {
    fn drop(&mut self) {
        if let (Some(p), Some(e)) = (&self.nv_path, &self.machine.bus.eeprom) {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = std::fs::write(p, &e.data);
        }
    }
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
        // QSound : cartes QSound du CPS1 et toutes les cartes CPS2 (stéréo)
        audio: if info.machine == "qsound" || cps_cart::system_of(info) == cps_cart::System::Cps2 {
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
        let input = game.info.input;
        Cps1Board {
            machine: Machine::new(game),
            info,
            audio_mono: Vec::new(),
            nv_path: None,
            input,
            voice_scope: false,
            voice_mute: 0,
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
        crate::debug_audio::apply(&mut self.machine, self.voice_scope, self.voice_mute);
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
        if self.machine.bus.cps2.is_some() {
            // CPS2 : ports IN0-IN2 (spec: docs/spec/05 §Ports d'entrée)
            let i = crate::inputs_cps2::to_cps2_inputs(inputs, self.input);
            self.machine.set_cps2_inputs(i);
            return;
        }
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
        let r = self.machine.load_state(data);
        // l'état chargé remplace les puces : enregistrement et muets du débogueur réappliqués
        crate::debug_audio::apply(&mut self.machine, self.voice_scope, self.voice_mute);
        r
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
    fn cps2_board_inputs_and_audio() {
        let mut b = Cps1Board::new(testutil::synthetic_cps2_game());
        assert_eq!(b.info().audio.channels, 2);
        let mut j = JammaInputs::default();
        j.players[0].buttons[0] = true;
        b.set_inputs(&j);
        assert_eq!(b.machine.bus.cps2.as_ref().unwrap().inputs.in0, 0xFFEF);
        assert_eq!(b.machine.bus.eeprom.as_ref().unwrap().data.len(), 128);
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
    fn eeprom_is_saved_and_restored() {
        let dir = std::env::temp_dir().join(format!("cps1_nv_{}", std::process::id()));
        let path = dir.join("jeu.nv");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&path, [0x5Au8; 128]).unwrap();
        let mut b = synthetic_board();
        b.machine.bus.eeprom = Some(cps_bus::Eeprom93c46::new());
        let mut b = b.with_nvram(path.clone());
        assert_eq!(b.machine.bus.eeprom.as_ref().unwrap().data, vec![0x5A; 128]);
        b.machine.bus.eeprom.as_mut().unwrap().data[3] = 0x12;
        drop(b);
        let d = std::fs::read(&path).unwrap();
        assert_eq!((d.len(), d[3], d[4]), (128, 0x12, 0x5A));
        // sans EEPROM (carte CPS1) : aucun fichier
        let p2 = dir.join("cps1.nv");
        drop(synthetic_board().with_nvram(p2.clone()));
        assert!(!p2.exists());
        let _ = std::fs::remove_dir_all(&dir);
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
