//! Exporte des trames en PNG : `cargo run -p cps-core --release --example dump_frames -- sf2hf 2990 3010`
//! (trames première..dernière incluses, dans target/frames/<jeu>/fNNNNNN.png).

use std::path::Path;

use cps_common::video::{SCREEN_H, SCREEN_W};
use cps_core::{load_game, Config, Machine};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let game = a.first().cloned().unwrap_or_else(|| "sf2hf".into());
    let first: u64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(2990);
    let last: u64 = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(first);
    let cfg = Config::load(Path::new(Config::DEFAULT_PATH)).expect("cps1.toml");
    let mut m = Machine::new(load_game(&cfg, &game).expect("jeu"));
    let dir = Path::new("target/frames").join(&game);
    std::fs::create_dir_all(&dir).unwrap();
    for frame in 1..=last {
        m.run_frame();
        if frame >= first {
            let file = std::fs::File::create(dir.join(format!("f{frame:06}.png"))).unwrap();
            let mut enc = png::Encoder::new(
                std::io::BufWriter::new(file),
                SCREEN_W as u32,
                SCREEN_H as u32,
            );
            enc.set_color(png::ColorType::Rgb);
            enc.set_depth(png::BitDepth::Eight);
            let data: Vec<u8> = m
                .framebuffer()
                .pixels
                .iter()
                .flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8])
                .collect();
            enc.write_header().unwrap().write_image_data(&data).unwrap();
        }
    }
    println!("{} trames -> {}", last - first + 1, dir.display());
}
