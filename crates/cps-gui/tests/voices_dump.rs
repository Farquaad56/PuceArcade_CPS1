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

/// Pièce à 600, départ à 700, puis bouton 1 toutes les 8 trames.
fn inputs(t: u64) -> JammaInputs {
    let mut j = JammaInputs::default();
    j.coins[0] = (600..606).contains(&t);
    j.players[0].start = (700..706).contains(&t);
    j.players[0].buttons[0] = t > 800 && t % 8 < 2;
    j.players[0].right = t > 800 && (t / 60).is_multiple_of(2);
    j
}

/// Son de `frames` trames, avec ou sans enregistrement des voies.
fn run(b: &mut dyn JammaBoard, frames: u64, scope: bool) -> Vec<f32> {
    b.debug().unwrap().set_voice_scope(scope);
    let (mut all, mut out) = (Vec::new(), Vec::new());
    for t in 1..=frames {
        b.set_inputs(&inputs(t));
        b.run_frame();
        b.take_audio(&mut out);
        all.extend_from_slice(&out);
    }
    all
}

#[test]
#[ignore] // ROM requises
fn voices_dump() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    let dir = root.join("target/voices");
    std::fs::create_dir_all(&dir).unwrap();
    let d = Cps1Driver { cfg };
    for (game, frames) in [
        ("ghouls", 1200),
        ("sf2", 1500),
        ("dino", 1200),
        ("xmcota", 1500),
    ] {
        let Ok(mut a) = d.load(game) else {
            println!("{game} : absent");
            continue;
        };
        let mut b = d.load(game).unwrap();
        let sa = run(a.as_mut(), frames, false);
        let sb = run(b.as_mut(), frames, true);
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
