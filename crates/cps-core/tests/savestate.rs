//! Savestates sur de vrais jeux (T408) : save à la trame 1000, 300 trames, puis load et 300 trames
//! -> même image, même son et même état complet. ROM requises (cps1.toml) : `--include-ignored`.

use cps_core::{load_game, Config, Machine};
use std::path::Path;

fn machine(name: &str) -> Option<Machine> {
    let cfg_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../cps1.toml");
    let cfg = Config::load(&cfg_path).ok()?;
    match load_game(&cfg, name) {
        Ok(g) => Some(Machine::new(g)),
        Err(e) => {
            println!("{name} ignoré : {e}");
            None
        }
    }
}

/// Rejoue 300 trames et renvoie (crc de l'image, son produit, état final).
fn run_300(m: &mut Machine) -> (u32, Vec<[f32; 2]>, Vec<u8>) {
    let mut audio = Vec::new();
    for _ in 0..300 {
        m.run_frame();
        audio.extend(m.take_audio());
    }
    (m.framebuffer().crc32(), audio, m.save_state())
}

/// `expect_sound` : le jeu produit du son sur les trames 1000-1300 (vérifié avec MAME 0.287 :
/// la démo de ghouls est muette, celle de sf2ceea ne l'est pas).
fn check(name: &str, expect_sound: bool) {
    let Some(mut m) = machine(name) else { return };
    for _ in 0..1000 {
        m.run_frame();
        m.take_audio();
    }
    let saved = m.save_state();
    let (crc_a, audio_a, end_a) = run_300(&mut m);

    m.load_state(&saved).unwrap();
    let (crc_b, audio_b, end_b) = run_300(&mut m);
    assert_eq!(crc_a, crc_b, "{name} : image différente après reprise");
    assert!(audio_a == audio_b, "{name} : son différent après reprise");
    assert!(
        end_a == end_b,
        "{name} : état final différent après reprise"
    );

    // Même résultat en chargeant l'état dans une machine neuve.
    let Some(mut fresh) = machine(name) else {
        return;
    };
    fresh.load_state(&saved).unwrap();
    let (crc_c, audio_c, end_c) = run_300(&mut fresh);
    assert_eq!(crc_a, crc_c, "{name} : image différente (machine neuve)");
    assert!(audio_a == audio_c, "{name} : son différent (machine neuve)");
    assert!(
        end_a == end_c,
        "{name} : état final différent (machine neuve)"
    );
    if expect_sound {
        assert!(
            audio_a.iter().any(|s| s[0] != 0.0),
            "{name} : aucun son sur 300 trames (le test ne vérifierait pas le son)"
        );
    }
}

#[test]
#[ignore] // ROM requises
fn ghouls_savestate() {
    check("ghouls", false);
}

#[test]
#[ignore] // ROM requises
fn sf2ceea_savestate() {
    check("sf2ceea", true);
}
