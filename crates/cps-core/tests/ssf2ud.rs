//! Super Street Fighter II (ssf2ud, CPS2, clé Phoenix) contre MAME 0.287 (T834).
//! ROM (ssf2.zip avec phoenix.key, qsound.zip) et golden requis (hors dépôt) : `--include-ignored`.
//! Golden : `powershell tools/mame_golden.ps1 -Game ssf2ud -Frames 600,1500,3000 -Out tests-data/golden/ssf2ud`.

mod common;

use cps_core::{load_game, Config, Machine};
use cps_testkit::golden::{diff_frame, golden_dir, load_png_rgb, write_diff_ppm};
use std::path::Path;

const GAME: &str = "ssf2ud";
const GOLDEN_FRAMES: [u64; 3] = [600, 1500, 3000];
const MAX_DIFF_RATIO: f64 = 0.02;

fn machine() -> Option<Machine> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config = Config::load(&manifest_dir.join("../../cps1.toml")).ok()?;
    match load_game(&config, GAME) {
        Ok(g) => Some(Machine::new(g)),
        Err(e) => {
            println!("{GAME} ignoré : {e}");
            None
        }
    }
}

#[test]
#[ignore] // ROM et golden requis
fn ssf2ud_golden_comparison() {
    let Some(mut machine) = machine() else { return };
    if !golden_dir(GAME).exists() {
        println!("{GAME} ignoré : pas de golden");
        return;
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut failures = Vec::new();
    for frame in 1..=3000 {
        machine.run_frame();
        assert!(!machine.cpu.is_halted(), "double faute à la trame {frame}");
        if !GOLDEN_FRAMES.contains(&frame) {
            continue;
        }
        let out = manifest_dir.join(format!("../../target/frames/{GAME}_f{frame:06}.png"));
        common::write_png(machine.framebuffer(), &out);
        let golden = golden_dir(GAME).join(format!("f{frame:06}.png"));
        let (w, h, pixels) = load_png_rgb(&golden).expect("golden illisible");
        let diff = diff_frame(machine.framebuffer(), &pixels, w, h).expect("taille du golden");
        println!(
            "trame {frame} : {:.4} % de pixels différents",
            diff.ratio() * 100.0
        );
        if diff.ratio() > MAX_DIFF_RATIO {
            let p = manifest_dir.join(format!("../../target/diff/{GAME}_f{frame:06}.ppm"));
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            write_diff_ppm(machine.framebuffer(), &pixels, &p).expect("diff");
            failures.push((frame, diff.ratio()));
        }
    }
    common::update_game_metrics(GAME, failures.is_empty());
    assert!(failures.is_empty(), "écarts au-delà de 2 % : {failures:?}");
}

#[test]
#[ignore] // ROM requises
fn ssf2ud_determinism() {
    let (Some(mut a), Some(mut b)) = (machine(), machine()) else {
        return;
    };
    for _ in 0..600 {
        a.run_frame();
        b.run_frame();
    }
    assert_eq!(a.framebuffer().crc32(), b.framebuffer().crc32());
    assert_eq!(a.cpu.state().pc, b.cpu.state().pc);
}

/// Image golden : largeur, hauteur, pixels.
type Golden = (usize, usize, Vec<u32>);

/// Sonde de diagnostic : pour chaque golden `target/golden_probe/ssf2ud/fNNNNNN.png`, écart de nos
/// trames N-3..N+3 (`--ignored --nocapture`).
#[test]
#[ignore]
fn ssf2ud_probe() {
    let Some(mut m) = machine() else { return };
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/golden_probe/ssf2ud");
    let mut goldens: Vec<(u64, Golden)> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            let f: u64 = n.strip_prefix('f')?.strip_suffix(".png")?.parse().ok()?;
            Some((f, load_png_rgb(&e.path()).ok()?))
        })
        .collect();
    goldens.sort_by_key(|g| g.0);
    let last = goldens.last().map_or(0, |g| g.0) + 3;
    for frame in 1..=last {
        m.run_frame();
        for (g, (w, h, px)) in &goldens {
            if frame + 3 >= *g && frame <= g + 3 {
                let d = diff_frame(m.framebuffer(), px, *w, *h).expect("taille");
                println!(
                    "sonde : golden {g}, notre trame {frame} : {:.4} %",
                    d.ratio() * 100.0
                );
            }
        }
    }
}
