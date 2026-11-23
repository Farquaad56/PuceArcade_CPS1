//! Débogage vidéo et son (T504) : registres CPS-A/B, sprites, son, palette, tuiles.
//! spec: docs/spec/02-video-cps-a-b.md §Registres CPS-A §CPS-B §Palette

use cps_core::Machine;
use cps_gfx::decode::{tile_pixel, Layout};
use cps_gfx::mapper::{gfxrom_bank_mapper, GFXTYPE_SPRITES};
use puce_jamma::debug::DebugImage;

pub const LAYER_NAMES: [&str; 5] = ["Sprites", "Scroll 1", "Scroll 2", "Scroll 3", "Étoiles"];

/// Noms des 18 registres CPS-A (index des mots, cf. cps-gfx regs.rs).
const CPS_A_NAMES: [&str; 18] = [
    "OBJ_BASE",
    "SCROLL1_BASE",
    "SCROLL2_BASE",
    "SCROLL3_BASE",
    "OTHER_BASE",
    "PALETTE_BASE",
    "SCROLL1_X",
    "SCROLL1_Y",
    "SCROLL2_X",
    "SCROLL2_Y",
    "SCROLL3_X",
    "SCROLL3_Y",
    "STARS1_X",
    "STARS1_Y",
    "STARS2_X",
    "STARS2_Y",
    "ROWSCROLL_OFFS",
    "VIDEOCONTROL",
];

/// Ordre de tracé des couches (du fond vers l'avant) : bits 6-7, 8-9, 10-11, 12-13.
/// spec: docs/spec/02 §CPS-B — layer_control
pub fn decode_layer_order(lc: u16) -> [u8; 4] {
    [6, 8, 10, 12].map(|s| ((lc >> s) & 3) as u8)
}

const ORDER_NAMES: [&str; 4] = ["Sprites", "Scroll1", "Scroll2", "Scroll3"];

fn hex4(v: u16) -> String {
    format!("{v:04X}")
}

/// Mot CPS-B à une adresse d'octet de la configuration (-1 = absent).
fn cps_b(m: &Machine, addr: i32) -> Option<u16> {
    (addr >= 0).then(|| m.bus.video.regs_b[(addr / 2) as usize & 31])
}

pub fn registers(m: &Machine) -> Vec<(String, Vec<(String, String)>)> {
    let v = &m.bus.video;
    let cps_a = CPS_A_NAMES
        .iter()
        .enumerate()
        .map(|(i, n)| ((*n).into(), hex4(v.regs_a[i])))
        .collect();

    let cfg = &v.cfg.cpsb;
    let lc = cps_b(m, cfg.layer_control).unwrap_or(0);
    let order: Vec<&str> = decode_layer_order(lc)
        .iter()
        .map(|l| ORDER_NAMES[usize::from(*l)])
        .collect();
    let mut b = vec![
        ("layer_control".into(), hex4(lc)),
        ("ordre".into(), order.join(" ")),
    ];
    let enables: Vec<String> = cfg
        .layer_enable_mask
        .iter()
        .map(|e| format!("{:02X}", e))
        .collect();
    b.push(("enables".into(), enables.join(" ")));
    for (i, p) in cfg.priority.iter().enumerate() {
        b.push((
            format!("priorité {i}"),
            cps_b(m, *p).map_or("-".into(), hex4),
        ));
    }
    b.push((
        "palette_control".into(),
        cps_b(m, cfg.palette_control).map_or("-".into(), hex4),
    ));
    let id = if cfg.cpsb_addr >= 0 {
        format!("{:04X} @ {:02X}", cfg.cpsb_value, cfg.cpsb_addr)
    } else {
        "-".into()
    };
    b.push(("id CPS-B".into(), id));

    vec![
        ("CPS-A".into(), cps_a),
        ("CPS-B".into(), b),
        ("Sprites".into(), sprites(m)),
        ("Son".into(), sound(m)),
    ]
}

/// Entrées de la table de sprites tamponnée, jusqu'au marqueur de fin (64 au plus).
fn sprites(m: &Machine) -> Vec<(String, String)> {
    let v = &m.bus.video;
    let last = cps_gfx::sprites::find_last_sprite(&v.buffered_obj, v.cfg.bootleg_kludge);
    (0..=last.max(-4) / 4)
        .take(64)
        .filter(|_| last >= 0)
        .map(|i| {
            let o = i as usize * 4;
            let [x, y, code, attr] = [0, 1, 2, 3].map(|k| v.buffered_obj[o + k]);
            let mapped = gfxrom_bank_mapper(&v.cfg.mapper, GFXTYPE_SPRITES, i32::from(code));
            (
                format!("{i:02}"),
                format!("{x:03X} {y:03X} {code:04X} {attr:04X} -> {mapped:X}"),
            )
        })
        .collect()
}

fn sound(m: &Machine) -> Vec<(String, String)> {
    let s = &m.sound;
    let oki = &s.chips.oki;
    let voices: Vec<String> = oki
        .voices
        .iter()
        .enumerate()
        .filter(|(_, v)| v.playing)
        .map(|(i, _)| i.to_string())
        .collect();
    if s.qsound.is_some() {
        return vec![
            (
                "carte son".into(),
                "QSound (Z80 Kabuki, DSP HLE, 24 038 Hz stéréo)".into(),
            ),
            ("latch1".into(), format!("{:02X}", s.latch[0])),
            ("latch2".into(), format!("{:02X}", s.latch[1])),
        ];
    }
    vec![
        (
            "carte son".into(),
            "YM2151 + OKI M6295 (55 930 Hz mono)".into(),
        ),
        ("latch1".into(), format!("{:02X}", s.latch[0])),
        ("latch2".into(), format!("{:02X}", s.latch[1])),
        ("banque".into(), s.bank.to_string()),
        (
            "OKI broche 7".into(),
            if oki.pin7 { "haut" } else { "bas" }.into(),
        ),
        (
            "OKI voix".into(),
            if voices.is_empty() {
                "-".into()
            } else {
                voices.join(" ")
            },
        ),
    ]
}

/// Grille de 256 tuiles (16 × 16), pens bruts en niveaux de gris.
fn tiles(gfx: &[u8], name: &str, layout: Layout) -> DebugImage {
    let t = layout.size();
    let side = 16 * t;
    let mut pixels = vec![0u32; (side * side) as usize];
    for code in 0..256u32 {
        let (cx, cy) = ((code % 16) * t, (code / 16) * t);
        for y in 0..t {
            for x in 0..t {
                let v = u32::from(tile_pixel(gfx, layout, code, x, y));
                pixels[((cy + y) * side + cx + x) as usize] = v * 0x11_1111;
            }
        }
    }
    DebugImage {
        name: name.into(),
        width: side,
        height: side,
        pixels,
    }
}

pub fn images(m: &Machine) -> Vec<DebugImage> {
    let pal = &m.bus.video.palette;
    let palette = DebugImage {
        name: "Palette".into(),
        width: 64,
        height: 48,
        pixels: pal[..64 * 48].to_vec(),
    };
    vec![
        palette,
        tiles(&m.gfx, "Tuiles 16x16", Layout::L16x16),
        tiles(&m.gfx, "Tuiles 8x8", Layout::L8x8),
        tiles(&m.gfx, "Tuiles 32x32", Layout::L32x32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::tests::synthetic_board;
    use puce_jamma::board::JammaBoard;
    use puce_jamma::debug::BoardDebug;

    #[test]
    fn layer_order() {
        assert_eq!(decode_layer_order(0x3842), [1, 0, 2, 3]);
    }

    #[test]
    fn registers_groups() {
        let b = synthetic_board();
        let r = b.registers();
        let names: Vec<&str> = r.iter().map(|g| g.0.as_str()).collect();
        assert_eq!(names, ["CPS-A", "CPS-B", "Sprites", "Son"]);
        assert_eq!(r[0].1.len(), 18);
        assert!(r[0].1.iter().all(|(_, v)| v.len() == 4));
    }

    #[test]
    fn palette_and_tiles_images() {
        let mut b = synthetic_board();
        b.machine.bus.video.palette[0x18] = 0x00AB_CDEF;
        let i = b.images();
        assert_eq!((i[0].width, i[0].height), (64, 48));
        assert_eq!(i[0].pixels[0x18], 0x00AB_CDEF);
        assert_eq!((i[1].width, i[1].height), (256, 256));
        assert_eq!((i[2].width, i[3].width), (128, 512));
    }

    #[test]
    fn layer_mask() {
        let mut b = synthetic_board();
        assert_eq!(b.layer_names().len(), 5);
        b.set_layer_mask(0xFFFF_FFFE);
        b.run_frame();
        assert_eq!(b.machine.bus.video.debug_layer_mask, 0xFFFF_FFFE);
    }
}
