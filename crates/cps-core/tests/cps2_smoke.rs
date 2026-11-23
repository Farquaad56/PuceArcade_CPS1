//! Démarrage des jeux CPS2 : temps et PC par trame sur les premières trames (diagnostic).
//! `cargo test -p cps-core --release --test cps2_smoke <jeu> -- --ignored --nocapture`.

use cps_core::{load_game, Config, Machine};
use std::path::Path;
use std::time::Instant;

fn smoke(game: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    let g = match load_game(&cfg, game) {
        Ok(g) => g,
        Err(e) => return println!("{game} : {e}"),
    };
    let mut m = Machine::new(g);
    let t0 = Instant::now();
    for f in 1..=40u64 {
        let t = Instant::now();
        m.run_frame();
        let s = m.cpu.state();
        println!(
            "{game} trame {f} : {:?} pc {:06X} sr {:04X} arrêt {}",
            t.elapsed(),
            s.pc,
            s.sr,
            m.cpu.is_halted()
        );
        if t0.elapsed().as_secs() > 20 {
            println!("{game} : trop lent, arrêt");
            break;
        }
    }
}

#[test]
#[ignore]
fn smoke_xmcota() {
    smoke("xmcota");
}

#[test]
#[ignore]
fn smoke_sfa3() {
    smoke("sfa3");
}

#[test]
#[ignore]
fn smoke_ddtod() {
    smoke("ddtod");
}

#[test]
#[ignore]
fn smoke_xmvsf() {
    smoke("xmvsf");
}
