//! Chemin du frontal pour un jeu CPS2 (driver.load + set_inputs à chaque trame), ROM requises :
//! `cargo test -p cps-gui --release --test cps2_gui_path -- --ignored --nocapture`.

use cps_cart::Config;
use cps_gui::driver::Cps1Driver;
use puce_jamma::driver::BoardDriver;
use puce_jamma::inputs::JammaInputs;
use std::path::Path;

#[test]
#[ignore] // ROM requises
fn cps2_through_frontend() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    let mut cfg = cfg;
    cfg.saves_dir = root.join("target/test_saves"); // pas de .nv de l'utilisateur
    let d = Cps1Driver { cfg };
    for game in ["ssf2ud", "xmcota", "sfa3", "ddtod"] {
        let mut b = match d.load(game) {
            Ok(b) => b,
            Err(e) => {
                println!("{game} : chargement impossible : {e}");
                continue;
            }
        };
        let inputs = JammaInputs::default();
        let mut lit = Vec::new();
        for f in 1..=1600u64 {
            b.set_inputs(&inputs);
            b.run_frame();
            let mut a = Vec::new();
            b.take_audio(&mut a);
            if f % 100 == 0 {
                let n = b
                    .video()
                    .pixels
                    .iter()
                    .filter(|&&p| p & 0xFF_FFFF != 0)
                    .count();
                lit.push((f, n));
            }
        }
        println!("{game} : pixels non noirs par 100 trames : {lit:?}");
    }
}
