//! Common helpers for CPS1 integration tests (golden comparison, PNG writing).

use cps_common::video::{Framebuffer, SCREEN_H, SCREEN_W};
use std::path::Path;

/// Write a framebuffer as an RGB8 PNG file.
pub fn write_png(fb: &Framebuffer, path: &Path) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let mut enc = png::Encoder::new(file, SCREEN_W as u32, SCREEN_H as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().unwrap();
    let mut data = Vec::with_capacity(SCREEN_W * SCREEN_H * 3);
    for &p in &fb.pixels {
        data.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
    }
    writer.write_image_data(&data).unwrap();
}

/// Enregistre le résultat d'un jeu dans metrics/games.json (un résultat par jeu : relancer un test
/// remplace la valeur du jeu au lieu de l'ajouter ; pass/fail/total sont recalculés à partir de la liste).
pub fn update_game_metrics(game: &str, pass: bool) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let metrics_path = manifest_dir.join("../../metrics/games.json");
    let mut games = std::fs::read_to_string(&metrics_path)
        .map(|c| parse_games(&c))
        .unwrap_or_default();
    games.insert(game.to_string(), pass);
    let pass_count = games.values().filter(|&&p| p).count();
    let fail_count = games.len() - pass_count;
    let list: Vec<String> = games
        .iter()
        .map(|(g, p)| format!("    \"{g}\": {p}"))
        .collect();
    let json = format!(
        "{{\n  \"pass\": {pass_count},\n  \"fail\": {fail_count},\n  \"total\": {},\n  \"games\": {{\n{}\n  }}\n}}\n",
        games.len(),
        list.join(",\n")
    );
    std::fs::create_dir_all(metrics_path.parent().unwrap()).unwrap();
    std::fs::write(&metrics_path, json).unwrap();
}

/// Lit la liste `"games": { "nom": true|false, ... }` (format écrit par update_game_metrics).
fn parse_games(content: &str) -> std::collections::BTreeMap<String, bool> {
    let mut games = std::collections::BTreeMap::new();
    let Some(pos) = content.find("\"games\"") else {
        return games;
    };
    let body = &content[pos..];
    let body = &body[body.find('{').map_or(0, |i| i + 1)..body.find('}').unwrap_or(body.len())];
    for item in body.split(',') {
        let mut kv = item.splitn(2, ':');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            let k = k.trim().trim_matches('"');
            if !k.is_empty() {
                games.insert(k.to_string(), v.trim() == "true");
            }
        }
    }
    games
}
