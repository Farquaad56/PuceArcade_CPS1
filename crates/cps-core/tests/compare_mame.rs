//! Comparaison rapide de jeux à une trame de MAME (golden dans target/golden_probe/<jeu>/fNNNNNN.png,
//! produit par tools/mame_golden.ps1). `CMP_GAMES="sf2rb,sf2m2" CMP_FRAME=1200 cargo test -p cps-core
//! --release --test compare_mame -- --ignored --nocapture`.

use cps_core::{load_game, Config, Machine};
use cps_testkit::golden::{diff_frame, load_png_rgb};
use std::path::Path;

#[test]
#[ignore]
fn compare_with_mame() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    let frame: u64 = std::env::var("CMP_FRAME")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1200);
    for game in std::env::var("CMP_GAMES")
        .unwrap_or_default()
        .split(',')
        .filter(|g| !g.is_empty())
    {
        let golden = root.join(format!("target/golden_probe/{game}/f{frame:06}.png"));
        let g = match load_game(&cfg, game) {
            Ok(g) => g,
            Err(e) => {
                println!("comparaison {game} : chargement impossible : {e}");
                continue;
            }
        };
        let mut m = Machine::new(g);
        let mut halted = false;
        for _ in 0..frame {
            m.run_frame();
            halted |= m.cpu.is_halted();
        }
        let res = match load_png_rgb(&golden) {
            Ok((w, h, px)) => match diff_frame(m.framebuffer(), &px, w, h) {
                Ok(d) => format!("{:.2} % de pixels différents", d.ratio() * 100.0),
                Err(e) => format!("taille : {e:?}"),
            },
            Err(_) => "pas de golden".to_string(),
        };
        println!(
            "comparaison {game} trame {frame} : {res}{}",
            if halted { " (CPU arrêté)" } else { "" }
        );
    }
}
