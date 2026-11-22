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

fn present_games(cfg: &Config) -> Vec<&'static str> {
    cps_cart::supported_games()
        .into_iter()
        .map(|g| g.name)
        .filter(|n| present(cfg, n))
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
    NoGolden,
    Crash(String),
}

fn run_game(cfg: &Config, name: &str) -> Outcome {
    if FRAMES
        .iter()
        .any(|f| !golden_dir(name).join(format!("f{f:06}.png")).is_file())
    {
        return Outcome::NoGolden;
    }
    let game = match load_game(cfg, name) {
        Ok(g) => g,
        Err(e) => return Outcome::Crash(format!("chargement : {e}")),
    };
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
    let (mut ok, mut failed) = (0, Vec::new());
    for name in &games {
        let title = cps_cart::find_game(name).map_or("", |g| g.title);
        let outcome = run_game(&cfg, name);
        let cell = match &outcome {
            Outcome::Ok(r) => format!("OK (écart max {:.3} %)", r * 100.0),
            Outcome::Diff(f, r) => format!("écart {:.2} % à la trame {f}", r * 100.0),
            Outcome::NoGolden => "pas de golden".into(),
            Outcome::Crash(e) => format!("crash : {e}"),
        };
        println!("{name}: {cell}");
        let _ = writeln!(md, "| {name} | {title} | {cell} |");
        match outcome {
            Outcome::Ok(_) => {
                ok += 1;
                common::update_game_metrics(name, true);
            }
            Outcome::NoGolden => {}
            _ => {
                failed.push(*name);
                common::update_game_metrics(name, false);
            }
        }
    }
    let _ = writeln!(
        md,
        "\n**{ok} OK** sur {} jeux présents ; en échec : {}.",
        games.len(),
        failed.len()
    );
    std::fs::write(root().join("docs/CATALOGUE.md"), md).unwrap();
    for g in MUST_PASS {
        assert!(!failed.contains(&g), "non-régression : {g} en échec");
    }
}
