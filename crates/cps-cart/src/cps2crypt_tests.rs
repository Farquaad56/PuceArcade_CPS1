//! Tests du chiffrement CPS2 contre l'oracle tests-data/cps2crypt/vectors.txt (MAME 0.287 compilé tel quel).

use super::*;

struct Vector {
    key: Vec<u8>,
    master: [u32; 2],
    lower: u32,
    upper: u32,
    crc: u32,
    samples: Vec<(usize, u16)>,
}

fn hex(s: &str) -> u32 {
    u32::from_str_radix(s, 16).expect("hexadécimal")
}

fn vectors() -> Vec<Vector> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests-data/cps2crypt/vectors.txt"
    );
    let text = std::fs::read_to_string(path).expect("vectors.txt");
    let mut out: Vec<Vector> = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f[0] == "key" {
            let key = (0..20)
                .map(|i| hex(&f[1][2 * i..2 * i + 2]) as u8)
                .collect();
            out.push(Vector {
                key,
                master: [hex(f[2]), hex(f[3])],
                lower: hex(f[4]),
                upper: hex(f[5]),
                crc: hex(f[6]),
                samples: Vec::new(),
            });
        } else {
            let v = out.last_mut().expect("ligne key avant les mots témoins");
            v.samples.push((hex(f[0]) as usize, hex(f[1]) as u16));
        }
    }
    out
}

/// ROM synthétique de l'oracle : rom[a] = (a × 0x9E37 + 0x1234) & 0xFFFF, 0x200000 mots.
fn rom() -> Vec<u16> {
    (0..0x200000u32)
        .map(|a| a.wrapping_mul(0x9e37).wrapping_add(0x1234) as u16)
        .collect()
}

#[test]
fn decode_key_matches_oracle() {
    let vs = vectors();
    assert_eq!(vs.len(), 5);
    for v in &vs {
        let k = decode_key(&v.key);
        assert_eq!((k.master, k.lower, k.upper), (v.master, v.lower, v.upper));
    }
}

#[test]
fn decrypt_matches_oracle() {
    let rom = rom();
    for (n, v) in vectors().iter().enumerate() {
        let dec = cps2_decrypt(&rom, v.master, v.lower / 2, v.upper / 2);
        let bytes: Vec<u8> = dec.iter().flat_map(|w| w.to_be_bytes()).collect();
        assert_eq!(cps_common::crc::crc32_bytes(&bytes), v.crc, "clé {n}");
        assert_eq!(v.samples.len(), 16);
        for &(a, w) in &v.samples {
            assert_eq!(dec[a], w, "clé {n}, mot {a:#x}");
        }
        if n == 4 {
            // borne haute incluse : le mot 0x80000 est déchiffré, 0x100000 ne l'est pas
            assert_eq!(dec[0x80000], 0xd60a);
            assert_eq!(dec[0x100000], 0x1234);
        }
    }
}

#[test]
fn dead_key_leaves_program_unchanged() {
    let k = decode_key(&[0xff; 20]);
    let prog: Vec<u8> = (0..0x40000u32).map(|i| (i * 7) as u8).collect();
    assert_eq!(decrypt_program(&prog, &k), prog);
}
