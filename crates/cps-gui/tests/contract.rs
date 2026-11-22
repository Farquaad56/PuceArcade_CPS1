//! T505 — bout en bout par le contrat JAMMA uniquement (`Box<dyn JammaBoard>`), comme le frontal.

use cps_core::machine::testutil;
use cps_gui::Cps1Board;
use puce_jamma::board::JammaBoard;
use puce_jamma::inputs::JammaInputs;

/// MOVE.W $800000,$FF0000 ; BRA.S 0x400 — recopie IN1 en RAM à chaque tour.
fn copy_in1_board() -> Box<dyn JammaBoard> {
    let program: Vec<(u32, &[u8])> = vec![
        (0x000000, &[0x00, 0xFF, 0x80, 0x00]),
        (0x000004, &[0x00, 0x00, 0x04, 0x00]),
        (
            0x400,
            &[
                0x33, 0xF9, 0x00, 0x80, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x60, 0xF4,
            ],
        ),
    ];
    Box::new(Cps1Board::new(testutil::synthetic_game(&program)))
}

/// Même crc que `Framebuffer::crc32` (pixels en petit-boutiste).
fn crc(b: &dyn JammaBoard) -> u32 {
    let bytes: Vec<u8> = b
        .video()
        .pixels
        .iter()
        .flat_map(|p| p.to_le_bytes())
        .collect();
    cps_common::crc::crc32_bytes(&bytes)
}

#[test]
fn inputs_reach_the_68000() {
    let mut b = copy_in1_board();
    let mut j = JammaInputs::default();
    j.players[0].right = true;
    j.players[0].buttons[0] = true;
    b.set_inputs(&j);
    b.run_frame();
    let d = b.debug().unwrap();
    let word = u16::from(d.peek(0, 0xFF0000)) << 8 | u16::from(d.peek(0, 0xFF0001));
    assert_eq!(word, 0xFFEE);
}

#[test]
fn sixty_frames_cadence() {
    let mut b = copy_in1_board();
    let mut audio = Vec::new();
    for _ in 0..60 {
        b.run_frame();
        b.take_audio(&mut audio);
    }
    assert_eq!(b.frame_count(), 60);
    // 60 × 84 510 720 / 90 112 = 56 268,9, plus la fin de la dernière instruction (la carte disait 56 191-56 192 : docs/QUESTIONS.md)
    assert!((56_268..=56_270).contains(&audio.len()), "{}", audio.len());
    assert_eq!(b.info().audio.channels, 1);
}

#[test]
fn savestate_through_the_contract() {
    let mut b = copy_in1_board();
    b.run_frame();
    let s = b.save_state();
    for _ in 0..20 {
        b.run_frame();
    }
    let expected = crc(b.as_ref());
    b.load_state(&s).unwrap();
    for _ in 0..20 {
        b.run_frame();
    }
    assert_eq!(crc(b.as_ref()), expected);
}

/// ROM réelles : l'adaptateur ne change rien aux images (600 trames).
#[test]
#[ignore = "ROM commerciales (cps1.toml)"]
fn real_games_match_the_machine() {
    let cfg = cps_cart::Config::load(std::path::Path::new("../../cps1.toml")).expect("cps1.toml");
    for name in ["ghouls", "sf2"] {
        let mut m = cps_core::Machine::new(cps_cart::load_game(&cfg, name).unwrap());
        let mut b: Box<dyn JammaBoard> =
            Box::new(Cps1Board::new(cps_cart::load_game(&cfg, name).unwrap()));
        for _ in 0..600 {
            m.run_frame();
            b.run_frame();
        }
        assert_eq!(crc(b.as_ref()), m.framebuffer().crc32(), "{name}");
    }
}
