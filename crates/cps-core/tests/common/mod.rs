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

/// Update metrics/games.json with test results.
pub fn update_game_metrics(pass: bool) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let metrics_path = manifest_dir.join("../../metrics/games.json");

    // Read existing or create new
    let (mut pass_count, mut fail_count, mut total_count) = if metrics_path.exists() {
        let content = std::fs::read_to_string(&metrics_path).unwrap();
        parse_metrics_json(&content)
    } else {
        (0, 0, 0)
    };

    total_count += 1;
    if pass {
        pass_count += 1;
    } else {
        fail_count += 1;
    }

    std::fs::create_dir_all(metrics_path.parent().unwrap()).unwrap();
    let json_str = format!(
        "{{\n  \"pass\": {},\n  \"fail\": {},\n  \"total\": {}\n}}",
        pass_count, fail_count, total_count
    );
    std::fs::write(&metrics_path, &json_str).unwrap();
}

fn parse_metrics_json(content: &str) -> (i64, i64, i64) {
    let pass = extract_int(content, "pass").unwrap_or(0);
    let fail = extract_int(content, "fail").unwrap_or(0);
    let total = extract_int(content, "total").unwrap_or(0);
    (pass, fail, total)
}

fn extract_int(json: &str, key: &str) -> Option<i64> {
    let search_key = format!("\"{}\"", key);
    if let Some(pos) = json.find(&search_key) {
        let after_key = &json[pos + search_key.len()..];
        let trimmed = after_key.trim_start();
        if let Some(colon_pos) = trimmed.find(':') {
            let value_str = trimmed[colon_pos + 1..].trim_start();
            let num_chars: String = value_str
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            return num_chars.parse::<i64>().ok();
        }
    }
    None
}
