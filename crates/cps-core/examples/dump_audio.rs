//! Enregistre le son d'un jeu en WAV 16 bits stéréo à sa fréquence native :
//! `cargo run -p cps-core --release --example dump_audio -- dino 1800 target/dino.wav [trame_pièce]`
//! Avec `trame_pièce` : pièce 1 pendant 10 trames à partir de cette trame, puis Start 1 pendant
//! 10 trames 120 trames plus tard (même séquence que tools/mame_coin.lua).
use std::io::Write;
use std::path::Path;

use cps_core::{load_game, Config, Machine};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let game = a.first().cloned().unwrap_or_else(|| "dino".into());
    let frames: u64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(1800);
    let out = a
        .get(2)
        .cloned()
        .unwrap_or_else(|| format!("target/{game}.wav"));
    let cfg = Config::load(Path::new(Config::DEFAULT_PATH)).expect("cps1.toml");
    let mut m = Machine::new(load_game(&cfg, &game).expect("jeu"));
    let coin: Option<u64> = a.get(3).and_then(|v| v.parse().ok());
    let mut pcm: Vec<i16> = Vec::new();
    for k in 1..=frames {
        if let Some(c) = coin {
            let mut i = cps_core::InputState::default();
            if (c..c + 10).contains(&k) {
                i.in0 &= !0x01; // pièce 1
            }
            if (c + 120..c + 130).contains(&k) {
                i.in0 &= !0x10; // start 1
            }
            m.set_inputs(i);
        }
        m.run_frame();
        for [l, r] in m.take_audio() {
            pcm.push((l * 32768.0).clamp(-32768.0, 32767.0) as i16);
            pcm.push((r * 32768.0).clamp(-32768.0, 32767.0) as i16);
        }
    }
    let rate = m.audio_rate().round() as u32;
    let data_len = (pcm.len() * 2) as u32;
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out).unwrap());
    f.write_all(b"RIFF").unwrap();
    f.write_all(&(36 + data_len).to_le_bytes()).unwrap();
    f.write_all(b"WAVEfmt ").unwrap();
    f.write_all(&16u32.to_le_bytes()).unwrap();
    f.write_all(&1u16.to_le_bytes()).unwrap(); // PCM
    f.write_all(&2u16.to_le_bytes()).unwrap(); // stéréo
    f.write_all(&rate.to_le_bytes()).unwrap();
    f.write_all(&(rate * 4).to_le_bytes()).unwrap();
    f.write_all(&4u16.to_le_bytes()).unwrap();
    f.write_all(&16u16.to_le_bytes()).unwrap();
    f.write_all(b"data").unwrap();
    f.write_all(&data_len.to_le_bytes()).unwrap();
    for s in pcm {
        f.write_all(&s.to_le_bytes()).unwrap();
    }
    println!("{} échantillons à {rate} Hz -> {out}", data_len / 4);
}
