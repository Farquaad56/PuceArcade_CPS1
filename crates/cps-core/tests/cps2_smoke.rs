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

/// Vecteurs de reset et premiers mots programme, bruts et déchiffrés.
#[test]
#[ignore]
fn opcodes_at_reset() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    for game in ["ssf2ud", "ssf2", "xmcota", "sfa3"] {
        let Ok(g) = load_game(&cfg, game) else {
            continue;
        };
        let raw = &g.regions.maincpu;
        let ops = &g.cps2.as_ref().unwrap().opcodes;
        let w = |b: &[u8], a: usize| u16::from_be_bytes([b[a], b[a + 1]]);
        let pc = (u32::from(w(raw, 4)) << 16 | u32::from(w(raw, 6))) as usize;
        let key = g
            .regions
            .key
            .as_ref()
            .map(|k| cps_cart::cps2crypt::decode_key(k));
        println!(
            "{game} : maincpu {:#X} octets, clé {key:X?}, SSP {:04X}{:04X} PC {pc:06X}",
            raw.len(),
            w(raw, 0),
            w(raw, 2)
        );
        let raw_s: Vec<String> = (0..8)
            .map(|i| format!("{:04X}", w(raw, pc + 2 * i)))
            .collect();
        let dec_s: Vec<String> = (0..8)
            .map(|i| format!("{:04X}", w(ops, pc + 2 * i)))
            .collect();
        println!("  brut     : {}", raw_s.join(" "));
        println!("  déchiffré: {}", dec_s.join(" "));
    }
}
