//! T700 — banc de performance : `cargo run -p cps-core --release --example bench -- ghouls 1800`
//! Exécute N trames, affiche trames/s, temps par trame, part du rendu et le crc de la dernière trame.

use std::path::Path;
use std::time::{Duration, Instant};

use cps_common::video::Framebuffer;
use cps_core::{load_game, Config, Machine};

fn main() {
    let mut args = std::env::args().skip(1);
    let game = args.next().unwrap_or_else(|| "ghouls".into());
    let frames: u64 = args.next().and_then(|n| n.parse().ok()).unwrap_or(1800);
    let cfg = Config::load(Path::new(Config::DEFAULT_PATH)).expect("cps1.toml introuvable");
    let mut m = Machine::new(load_game(&cfg, &game).expect("chargement du jeu"));

    let start = Instant::now();
    for _ in 0..frames {
        m.run_frame();
        let _ = m.take_audio();
    }
    let total = start.elapsed();

    // Part du rendu : rendu d'une copie de l'état vidéo sur l'état final, répété (exemple seulement).
    let mut video = m.bus.video.clone();
    let mut fb = Framebuffer::new();
    let reps = 200;
    let t = Instant::now();
    for _ in 0..reps {
        video.render_frame(&m.gfx, m.stars.as_deref(), m.frame, &mut fb);
    }
    let render = t.elapsed() / reps;

    let per_frame = total / frames as u32;
    let fps = frames as f64 / total.as_secs_f64();
    println!("jeu            : {game}");
    println!("trames         : {frames}");
    println!("durée          : {:.2} s", total.as_secs_f64());
    println!(
        "trames/s       : {fps:.1} ({:.2}× le temps réel)",
        fps / cps_core::FRAME_HZ
    );
    println!("temps / trame  : {:.3} ms", ms(per_frame));
    println!(
        "rendu / trame  : {:.3} ms ({:.0} %)",
        ms(render),
        100.0 * render.as_secs_f64() / per_frame.as_secs_f64()
    );
    println!("crc final      : {:08X}", m.framebuffer().crc32());
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}
