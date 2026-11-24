//! Voies de l'oscilloscope sur de vrais jeux (ROM requises) : chaque jeu tourne avec
//! l'enregistrement actif, puis les voies sont écrites dans `target/voices/<jeu>.txt`
//! (une ligne d'en-tête « nom|puce|Hz|active|détail » puis les échantillons séparés par « , »).
//! Vérifie aussi que l'enregistrement ne change pas le son.
//! `cargo test --release -p cps-gui --test voices_dump -- --ignored --nocapture`

use cps_cart::Config;
use cps_gui::driver::Cps1Driver;
use puce_jamma::board::JammaBoard;
use puce_jamma::driver::BoardDriver;
use puce_jamma::inputs::JammaInputs;
use std::fmt::Write as _;
use std::path::Path;

/// Pièce à `c`, départ à `c + 80` (8 trames chacun), puis bouton 1 toutes les 8 trames (beaucoup de jeux CPS1
/// sont muets pendant la démonstration).
fn inputs(t: u64, c: u64) -> JammaInputs {
    let mut j = JammaInputs::default();
    j.coins[0] = (c..c + 8).contains(&t);
    j.players[0].start = (c + 80..c + 88).contains(&t);
    j.players[0].buttons[0] = t > c + 200 && t % 8 < 2;
    j.players[0].right = t > c + 200 && (t / 60).is_multiple_of(2);
    j
}

/// Son de `frames` trames, avec ou sans enregistrement des voies.
fn run(b: &mut dyn JammaBoard, frames: u64, scope: bool) -> Vec<f32> {
    b.debug().unwrap().set_voice_scope(scope);
    let (mut all, mut out) = (Vec::new(), Vec::new());
    for t in 1..=frames {
        b.set_inputs(&inputs(t, frames - 560));
        b.run_frame();
        b.take_audio(&mut out);
        all.append(&mut out);
    }
    all
}

#[test]
#[ignore] // ROM requises
fn voices_dump() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(mut cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    let dir = root.join("target/voices");
    std::fs::create_dir_all(&dir).unwrap();
    // DIP et EEPROM par défaut : pas ceux du joueur
    cfg.saves_dir = dir.join("saves");
    let d = Cps1Driver { cfg };
    for (game, frames) in [
        ("ghouls", 1960),
        ("sf2", 2000),
        ("dino", 1300),
        ("xmcota", 1500),
    ] {
        let Ok(mut a) = d.load(game) else {
            println!("{game} : absent");
            continue;
        };
        let mut b = d.load(game).unwrap();
        let sa = run(a.as_mut(), frames, false);
        let sb = run(b.as_mut(), frames, true);
        assert!(sa.iter().any(|x| *x != 0.0), "{game} : silencieux");
        assert_eq!(sa, sb, "{game} : l'enregistrement change le son");
        let v = b.debug().unwrap().voices();
        assert!(!v.is_empty(), "{game} : aucune voie");
        let mut txt = String::new();
        for x in &v {
            let _ = writeln!(
                txt,
                "{}|{}|{}|{}|{}",
                x.name, x.chip, x.sample_rate, x.active, x.detail
            );
            let s: Vec<String> = x.samples.iter().map(|f| format!("{f:.4}")).collect();
            let _ = writeln!(txt, "{}", s.join(","));
        }
        std::fs::write(dir.join(format!("{game}.txt")), txt).unwrap();
        let sounding = v
            .iter()
            .filter(|x| x.samples.iter().any(|s| *s != 0.0))
            .count();
        println!("{game} : {} voies, {sounding} non muettes", v.len());

        // une voie coupée change le son ; masque 0 : son d'origine
        let first = v.iter().position(|x| x.samples.iter().any(|s| *s != 0.0));
        if let Some(i) = first {
            let mut c = d.load(game).unwrap();
            c.debug().unwrap().set_voice_mute(1 << i);
            let sc = run(c.as_mut(), frames, false);
            assert_ne!(sa, sc, "{game} : couper {} ne change rien", v[i].name);
        }
    }
}
