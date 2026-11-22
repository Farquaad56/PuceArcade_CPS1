//! T604 — validation du catalogue : chaque jeu jouable présent contre ses golden MAME (600, 1500).
//! Nécessite les ROM (cps1.toml) et les golden générés par tools/mame_golden_batch.ps1.
//!
//! `list_games` écrit target/games.txt (jeux jouables dont les ROM sont présentes) ;
//! `catalogue` exécute 1500 trames par jeu, compare à ≤ 2 % et écrit docs/CATALOGUE.md et
//! metrics/games.json. Un jeu en échec est listé, jamais exclu ; le seuil n'est jamais relevé.

mod common;

use cps_core::{load_game, Config, Machine};
use cps_testkit::golden::{diff_frame, golden_dir, load_png_rgb};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const FRAMES: [u64; 2] = [600, 1500];
const MAX_DIFF_RATIO: f64 = 0.02;
/// Non-régression : ces jeux doivent toujours être OK.
const MUST_PASS: [&str; 3] = ["ghouls", "sf2", "sf2ceea"];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn config() -> Config {
    Config::load(&root().join("cps1.toml")).expect("cps1.toml")
}

/// ROM présentes : zip ou dossier du jeu (les clones complètent avec le parent au chargement).
fn present(cfg: &Config, name: &str) -> bool {
    cfg.roms_dir.join(format!("{name}.zip")).is_file() || cfg.roms_dir.join(name).is_dir()
}

/// Jeux jouables présents : zip ou dossier du jeu, ou clone entièrement chargeable depuis le zip
/// de son parent (willowu, wofr1…).
fn present_games(cfg: &Config) -> Vec<&'static str> {
    cps_cart::supported_games()
        .into_iter()
        .filter(|g| {
            present(cfg, g.name)
                || (!g.parent.is_empty()
                    && present(cfg, g.parent)
                    && load_game(cfg, g.name).is_ok())
        })
        .map(|g| g.name)
        .collect()
}

#[test]
#[ignore = "ROM commerciales (cps1.toml)"]
fn list_games() {
    let games = present_games(&config());
    let path = root().join("target/games.txt");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, games.join("\n") + "\n").unwrap();
    println!("{} jeux -> {}", games.len(), path.display());
}

enum Outcome {
    Ok(f64),
    Diff(u64, f64),
    /// ROM refusées (CRC, fichier absent) : set d'une autre version de MAME.
    BadRoms(String),
    NoGolden,
    Crash(String),
}

/// MAME capture les jeux ROT270 tournés (224 × 384) : on ramène le golden à 384 × 224.
/// Rot270 de puce-jamma : écran (x, y) -> capture (y, 383 - x).
fn unrotate_270(px: &[u32], w: usize, h: usize) -> (usize, usize, Vec<u32>) {
    let (ow, oh) = (h, w);
    let mut out = vec![0u32; ow * oh];
    for y in 0..oh {
        for x in 0..ow {
            out[y * ow + x] = px[(ow - 1 - x) * w + y];
        }
    }
    (ow, oh, out)
}

fn run_game(cfg: &Config, name: &str) -> Outcome {
    let game = match load_game(cfg, name) {
        Ok(g) => g,
        Err(e) => return Outcome::BadRoms(e.to_string()),
    };
    if FRAMES
        .iter()
        .any(|f| !golden_dir(name).join(format!("f{f:06}.png")).is_file())
    {
        return Outcome::NoGolden;
    }
    let vertical = game.info.rotation == 270;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut m = Machine::new(game);
        let mut worst = 0.0f64;
        for frame in 1..=*FRAMES.last().unwrap() {
            m.run_frame();
            if m.cpu.is_halted() {
                return Outcome::Crash(format!("double faute trame {frame}"));
            }
            if FRAMES.contains(&frame) {
                let path = golden_dir(name).join(format!("f{frame:06}.png"));
                let (w, h, px) = match load_png_rgb(&path) {
                    Ok((w, h, px)) if vertical && w < h => unrotate_270(&px, w, h),
                    Ok(g) => g,
                    Err(e) => return Outcome::Crash(format!("golden : {e}")),
                };
                let ratio = match diff_frame(m.framebuffer(), &px, w, h) {
                    Ok(d) => d.ratio(),
                    Err(e) => return Outcome::Crash(format!("golden : {e}")),
                };
                let out = root().join(format!("target/frames/catalogue/{name}_f{frame:06}.png"));
                common::write_png(m.framebuffer(), &out);
                if ratio > MAX_DIFF_RATIO {
                    return Outcome::Diff(frame, ratio);
                }
                worst = worst.max(ratio);
            }
        }
        Outcome::Ok(worst)
    }));
    result.unwrap_or_else(|_| Outcome::Crash("panique".into()))
}

#[test]
#[ignore = "ROM commerciales et golden MAME"]
fn catalogue() {
    let cfg = config();
    let games = present_games(&cfg);
    let mut md = String::from(
        "# Catalogue CPS1 — validation contre MAME\n\n\
         Généré par `cargo test -p cps-core --release --test catalogue -- --ignored catalogue`.\n\
         Trames 600 et 1500 comparées aux golden MAME (seuil 2 % de pixels différents).\n\n\
         | Jeu | Titre | Résultat |\n|---|---|---|\n",
    );
    let (mut ok, mut skipped, mut failed) = (0, 0, Vec::new());
    for name in &games {
        let title = cps_cart::find_game(name).map_or("", |g| g.title);
        let outcome = run_game(&cfg, name);
        let cell = match &outcome {
            Outcome::Ok(r) => format!("OK (écart max {:.3} %)", r * 100.0),
            Outcome::Diff(f, r) => format!("écart {:.2} % à la trame {f}", r * 100.0),
            Outcome::BadRoms(e) => format!("ROM refusées ({e})"),
            Outcome::NoGolden => "pas de golden (MAME 0.287 refuse le set)".into(),
            Outcome::Crash(e) => format!("crash : {e}"),
        };
        println!("{name}: {cell}");
        let _ = writeln!(md, "| {name} | {title} | {cell} |");
        match outcome {
            Outcome::Ok(_) => {
                ok += 1;
                common::update_game_metrics(name, true);
            }
            Outcome::NoGolden | Outcome::BadRoms(_) => skipped += 1,
            _ => {
                failed.push(*name);
                common::update_game_metrics(name, false);
            }
        }
    }
    let _ = writeln!(
        md,
        "\n**{ok} OK**, {} en échec, {skipped} non comparés (ROM ou golden indisponibles) sur {} jeux présents.",
        failed.len(),
        games.len()
    );
    std::fs::write(root().join("docs/CATALOGUE.md"), md).unwrap();
    for g in MUST_PASS {
        assert!(!failed.contains(&g), "non-régression : {g} en échec");
    }
}
