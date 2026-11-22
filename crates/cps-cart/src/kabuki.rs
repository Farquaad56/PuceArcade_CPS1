//! Déchiffrement Kabuki de la ROM Z80 des jeux QSound.
// license:BSD-3-Clause
// copyright-holders:Nicola Salmoria
// source: MAME 0.287 src/mame/capcom/kabuki.cpp (bitswap1, bitswap2, bytedecode, kabuki_decode,
//         cps1_decode, wof/dino/punisher/slammast_decode) et cps1.cpp kabuki_setup (l.14724-14730)

fn bitswap1(mut src: i32, key: i32, select: i32) -> i32 {
    if select & (1 << (key & 7)) != 0 {
        src = (src & 0xfc) | ((src & 0x01) << 1) | ((src & 0x02) >> 1);
    }
    if select & (1 << ((key >> 4) & 7)) != 0 {
        src = (src & 0xf3) | ((src & 0x04) << 1) | ((src & 0x08) >> 1);
    }
    if select & (1 << ((key >> 8) & 7)) != 0 {
        src = (src & 0xcf) | ((src & 0x10) << 1) | ((src & 0x20) >> 1);
    }
    if select & (1 << ((key >> 12) & 7)) != 0 {
        src = (src & 0x3f) | ((src & 0x40) << 1) | ((src & 0x80) >> 1);
    }
    src
}

fn bitswap2(mut src: i32, key: i32, select: i32) -> i32 {
    if select & (1 << ((key >> 12) & 7)) != 0 {
        src = (src & 0xfc) | ((src & 0x01) << 1) | ((src & 0x02) >> 1);
    }
    if select & (1 << ((key >> 8) & 7)) != 0 {
        src = (src & 0xf3) | ((src & 0x04) << 1) | ((src & 0x08) >> 1);
    }
    if select & (1 << ((key >> 4) & 7)) != 0 {
        src = (src & 0xcf) | ((src & 0x10) << 1) | ((src & 0x20) >> 1);
    }
    if select & (1 << (key & 7)) != 0 {
        src = (src & 0x3f) | ((src & 0x40) << 1) | ((src & 0x80) >> 1);
    }
    src
}

fn bytedecode(src: i32, swap_key1: u32, swap_key2: u32, xor_key: i32, select: i32) -> i32 {
    let rot = |s: i32| ((s & 0x7f) << 1) | ((s & 0x80) >> 7);
    let mut s = bitswap1(src, (swap_key1 & 0xffff) as i32, select & 0xff);
    s = rot(s);
    s = bitswap2(s, (swap_key1 >> 16) as i32, select & 0xff);
    s ^= xor_key;
    s = rot(s);
    s = bitswap2(s, (swap_key2 & 0xffff) as i32, select >> 8);
    s = rot(s);
    bitswap1(s, (swap_key2 >> 16) as i32, select >> 8)
}

/// Clés (swap_key1, swap_key2, addr_key, xor_key) de cps1_decode.
fn keys(init: &str) -> Option<(u32, u32, i32, i32)> {
    match init {
        "init_wof" => Some((0x0123_4567, 0x5416_3072, 0x5151, 0x51)),
        "init_dino" => Some((0x7654_3210, 0x2460_1357, 0x4343, 0x43)),
        "init_punisher" => Some((0x6745_2103, 0x7531_6024, 0x2222, 0x22)),
        "init_slammast" => Some((0x5432_1076, 0x6543_2107, 0x3131, 0x19)),
        _ => None,
    }
}

/// kabuki_setup : déchiffre les 0x8000 premiers octets de la ROM Z80. Retourne les opcodes
/// déchiffrés ; les données déchiffrées remplacent la ROM en place (kabuki_decode écrit
/// `dest_data` dans `src`). `None` si l'initialisation du jeu n'utilise pas Kabuki.
pub fn decode(init: &str, audiocpu: &mut [u8]) -> Option<Vec<u8>> {
    let (k1, k2, addr_key, xor_key) = keys(init)?;
    let len = 0x8000.min(audiocpu.len());
    let mut ops = vec![0u8; 0x8000];
    for a in 0..len {
        let src = i32::from(audiocpu[a]);
        // decode opcodes
        let select = a as i32 + addr_key;
        ops[a] = bytedecode(src, k1, k2, xor_key, select) as u8;
        // decode data
        let select = ((a as i32) ^ 0x1fc0) + addr_key + 1;
        audiocpu[a] = bytedecode(src, k1, k2, xor_key, select) as u8;
    }
    Some(ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_init_is_untouched() {
        let mut rom = vec![0x12u8; 0x10];
        assert!(decode("empty_init", &mut rom).is_none());
        assert_eq!(rom, vec![0x12u8; 0x10]);
    }

    #[test]
    fn decode_is_a_permutation_per_address() {
        // pour une adresse donnée, les 256 octets chiffrés donnent 256 octets distincts
        for init in ["init_wof", "init_dino", "init_punisher", "init_slammast"] {
            let (k1, k2, ak, xk) = keys(init).unwrap();
            for a in [0, 1, 0x1234, 0x7fff] {
                let mut seen = [false; 256];
                for b in 0..256 {
                    let v = bytedecode(b, k1, k2, xk, a + ak) as usize;
                    assert!(!seen[v], "{init} {a:x}");
                    seen[v] = true;
                }
            }
        }
    }

    #[test]
    fn opcodes_and_data_differ_and_banks_untouched() {
        let mut rom: Vec<u8> = (0..0x10000u32).map(|i| (i * 7) as u8).collect();
        let orig = rom.clone();
        let ops = decode("init_wof", &mut rom).unwrap();
        assert_eq!(ops.len(), 0x8000);
        assert_ne!(&ops[..], &rom[..0x8000]);
        assert_eq!(
            &rom[0x8000..],
            &orig[0x8000..],
            "au-delà de 0x8000 : non chiffré"
        );
    }
}
