//! Diagnostic du son QSound : WAV brut du cœur (24 038 Hz, stéréo) et mesures (largeur stéréo,
//! saturation, sauts). `cargo test -p cps-core --release --test qsound_audio -- --ignored --nocapture`.
//! Variables : QS_GAME (défaut xmcota), QS_FRAMES (défaut 1800). WAV dans target/audio/.

use cps_core::{load_game, Config, Machine};
use std::io::Write;
use std::path::Path;

fn write_wav(path: &Path, rate: u32, s: &[[f32; 2]]) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut f = std::fs::File::create(path).unwrap();
    let data = (s.len() * 4) as u32;
    f.write_all(b"RIFF").unwrap();
    f.write_all(&(36 + data).to_le_bytes()).unwrap();
    f.write_all(b"WAVEfmt ").unwrap();
    f.write_all(&16u32.to_le_bytes()).unwrap();
    f.write_all(&1u16.to_le_bytes()).unwrap();
    f.write_all(&2u16.to_le_bytes()).unwrap();
    f.write_all(&rate.to_le_bytes()).unwrap();
    f.write_all(&(rate * 4).to_le_bytes()).unwrap();
    f.write_all(&4u16.to_le_bytes()).unwrap();
    f.write_all(&16u16.to_le_bytes()).unwrap();
    f.write_all(b"data").unwrap();
    f.write_all(&data.to_le_bytes()).unwrap();
    for [l, r] in s {
        for v in [l, r] {
            let i = (v * 32768.0).round().clamp(-32768.0, 32767.0) as i16;
            f.write_all(&i.to_le_bytes()).unwrap();
        }
    }
}

#[test]
#[ignore]
fn qsound_dump() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        return;
    };
    let game = std::env::var("QS_GAME").unwrap_or_else(|_| "xmcota".into());
    let frames: u64 = std::env::var("QS_FRAMES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1800);
    let Ok(g) = load_game(&cfg, &game) else {
        return println!("{game} : ROM absentes");
    };
    let mut m = Machine::new(g);
    let mut s = Vec::new();
    for _ in 0..frames {
        m.run_frame();
        s.extend(m.take_audio());
    }
    let rate = m.audio_rate().round() as u32;
    let n = s.len() as f64;
    let rms = |f: &dyn Fn(&[f32; 2]) -> f32| {
        (s.iter().map(|x| f64::from(f(x)).powi(2)).sum::<f64>() / n).sqrt()
    };
    let (l, r) = (rms(&|x| x[0]), rms(&|x| x[1]));
    let mid = rms(&|x| (x[0] + x[1]) / 2.0);
    let side = rms(&|x| (x[0] - x[1]) / 2.0);
    let clipped = s
        .iter()
        .filter(|x| x[0].abs() >= 0.999 || x[1].abs() >= 0.999)
        .count();
    let jumps = s
        .windows(2)
        .filter(|w| (w[1][0] - w[0][0]).abs() > 0.5 || (w[1][1] - w[0][1]).abs() > 0.5)
        .count();
    let identical = s.iter().filter(|x| x[0] == x[1]).count();
    println!(
        "{game} : {} échantillons à {rate} Hz ; RMS G {l:.4} D {r:.4} ; milieu {mid:.4} côté {side:.4} \
         (largeur {:.1} %) ; G == D {:.1} % ; saturés {clipped} ; sauts > 0,5 : {jumps}",
        s.len(),
        100.0 * side / mid.max(1e-9),
        100.0 * identical as f64 / n
    );
    write_wav(
        &root.join(format!("target/audio/{game}_coeur.wav")),
        rate,
        &s,
    );
}
