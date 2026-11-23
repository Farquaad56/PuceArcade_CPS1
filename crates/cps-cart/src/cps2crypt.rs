// license:BSD-3-Clause
// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald
//! Chiffrement des opcodes du 68000 du CPS2 : deux réseaux de Feistel à 4 tours (adresse puis
//! opcode), et décodage du fichier .key. spec: docs/spec/05 §Chiffrement
//! source: ref/mame-cps2/cps2crypt.cpp l.139-192 (sbox, optimised_sbox), l.488-713 (fn, expand_*,
//!         feistel, optimise_sboxes, cps2_decrypt) ; ref/mame-cps2/cps2.cpp l.10955-10995 (init_cps2crypt)

use crate::cps2crypt_tables::{
    SBox, FN1_GROUP_A, FN1_GROUP_B, FN1_R1_BOXES, FN1_R2_BOXES, FN1_R3_BOXES, FN1_R4_BOXES,
    FN2_GROUP_A, FN2_GROUP_B, FN2_R1_BOXES, FN2_R2_BOXES, FN2_R3_BOXES, FN2_R4_BOXES,
};

fn bit(v: u32, n: u32) -> u32 {
    (v >> n) & 1
}

/// optimised_sbox (l.162-192) : entrées et sorties précalculées.
#[derive(Clone, Copy)]
struct OptimisedSbox {
    input_lookup: [u8; 256],
    output: [u8; 64],
}

impl OptimisedSbox {
    /// optimise (l.166-183), avec sbox::extract_inputs (l.141-152).
    fn optimise(sbox: &SBox) -> Self {
        let mut input_lookup = [0u8; 256];
        for (i, slot) in input_lookup.iter_mut().enumerate() {
            let mut res = 0u8;
            for (k, &inp) in sbox.inputs.iter().enumerate() {
                if inp >= 0 {
                    res |= (bit(i as u32, inp as u32) as u8) << k;
                }
            }
            *slot = res;
        }
        let mut output = [0u8; 64];
        for (i, slot) in output.iter_mut().enumerate() {
            let o = sbox.table[i];
            if o & 1 != 0 {
                *slot |= 1 << sbox.outputs[0];
            }
            if o & 2 != 0 {
                *slot |= 1 << sbox.outputs[1];
            }
        }
        OptimisedSbox {
            input_lookup,
            output,
        }
    }

    /// fn (l.185-188). PORT-NOTE : `fn` est un mot réservé en Rust.
    fn apply(&self, input: u8, key: u32) -> u8 {
        self.output[usize::from(self.input_lookup[usize::from(input)] ^ (key & 0x3f) as u8)]
    }
}

/// fn (l.488-495) : les 4 boîtes d'un tour. PORT-NOTE : renommée (mot réservé).
fn sbox_fn(input: u8, sboxes: &[OptimisedSbox], key: u32) -> u8 {
    sboxes[0].apply(input, key)
        | sboxes[1].apply(input, key >> 6)
        | sboxes[2].apply(input, key >> 12)
        | sboxes[3].apply(input, key >> 18)
}

/// Dépose les bits `bits[i]` de `src` (64 bits en 2 × 32) dans `dst` par groupes de `width`.
fn spread<const N: usize>(dst: &mut [u32], src: &[u32; 2], bits: &[u8; N], width: usize) {
    for (i, &b) in bits.iter().enumerate() {
        let b = u32::from(b);
        dst[i / width] |= bit(src[(b / 32) as usize], b % 32) << (i % width);
    }
}

/// expand_1st_key (l.501-530) : clé maître 64 bits -> clé 96 bits du 1er réseau (4 × 24 bits).
fn expand_1st_key(src: &[u32; 2]) -> [u32; 4] {
    const BITS: [u8; 96] = [
        33, 58, 49, 36, 0, 31, 22, 30, 3, 16, 5, 53, 10, 41, 23, 19, 27, 39, 43, 6, 34, 12, 61, 21,
        48, 13, 32, 35, 6, 42, 43, 14, 21, 41, 52, 25, 18, 47, 46, 37, 57, 53, 20, 8, 55, 54, 59,
        60, 27, 33, 35, 18, 8, 15, 63, 1, 50, 44, 16, 46, 5, 4, 45, 51, 38, 25, 13, 11, 62, 29, 48,
        2, 59, 61, 62, 56, 51, 57, 54, 9, 24, 63, 22, 7, 26, 42, 45, 40, 23, 14, 2, 31, 52, 28, 44,
        17,
    ];
    let mut dst = [0u32; 4];
    spread(&mut dst, src, &BITS, 24);
    dst
}

/// expand_2nd_key (l.535-565) : clé maître XOR sous-clé -> clé 96 bits du 2e réseau.
fn expand_2nd_key(src: &[u32; 2]) -> [u32; 4] {
    const BITS: [u8; 96] = [
        34, 9, 32, 24, 44, 54, 38, 61, 47, 13, 28, 7, 29, 58, 18, 1, 20, 60, 15, 6, 11, 43, 39, 19,
        63, 23, 16, 62, 54, 40, 31, 3, 56, 61, 17, 25, 47, 38, 55, 57, 5, 4, 15, 42, 22, 7, 2, 19,
        46, 37, 29, 39, 12, 30, 49, 57, 31, 41, 26, 27, 24, 36, 11, 63, 33, 16, 56, 62, 48, 60, 59,
        32, 12, 30, 53, 48, 10, 0, 50, 35, 3, 59, 14, 49, 51, 45, 44, 2, 21, 33, 55, 52, 23, 28, 8,
        26,
    ];
    let mut dst = [0u32; 4];
    spread(&mut dst, src, &BITS, 24);
    dst
}

/// expand_subkey (l.572-589) : graine 16 bits du 1er réseau -> sous-clé 64 bits (2 × 32).
fn expand_subkey(seed: u16) -> [u32; 2] {
    const BITS: [u8; 64] = [
        5, 10, 14, 9, 4, 0, 15, 6, 1, 8, 3, 2, 12, 7, 13, 11, 5, 12, 7, 2, 13, 11, 9, 14, 4, 1, 6,
        10, 8, 0, 15, 3, 4, 10, 2, 0, 6, 9, 12, 1, 11, 7, 15, 8, 13, 5, 14, 3, 14, 11, 12, 7, 4, 5,
        2, 10, 1, 15, 0, 9, 8, 6, 13, 3,
    ];
    let mut subkey = [0u32; 2];
    for (i, &b) in BITS.iter().enumerate() {
        subkey[i / 32] |= bit(u32::from(seed), u32::from(b)) << (i % 32);
    }
    subkey
}

/// bitswap<8>(val, b[7], ..., b[0]) : le bit k du résultat vaut le bit b[k] de `val`.
fn gather(val: u16, bits: &[u8; 8]) -> u8 {
    let mut r = 0u8;
    for (k, &b) in bits.iter().enumerate() {
        r |= (bit(u32::from(val), u32::from(b)) as u8) << k;
    }
    r
}

/// feistel (l.593-622) : 4 demi-tours sur l'octet `l` (groupe B) et l'octet `r` (groupe A).
fn feistel(
    val: u16,
    bits_a: &[u8; 8],
    bits_b: &[u8; 8],
    boxes: [&[OptimisedSbox]; 4],
    key: [u32; 4],
) -> u16 {
    let mut l = gather(val, bits_b);
    let mut r = gather(val, bits_a);

    l ^= sbox_fn(r, boxes[0], key[0]);
    r ^= sbox_fn(l, boxes[1], key[1]);
    l ^= sbox_fn(r, boxes[2], key[2]);
    r ^= sbox_fn(l, boxes[3], key[3]);

    let mut out = 0u16;
    for k in 0..8 {
        out |= ((u16::from(l) >> k) & 1) << bits_a[k];
        out |= ((u16::from(r) >> k) & 1) << bits_b[k];
    }
    out
}

/// optimise_sboxes (l.626-630) : les 4 boîtes d'un tour.
fn optimise_sboxes(boxes: &[SBox; 4]) -> [OptimisedSbox; 4] {
    std::array::from_fn(|i| OptimisedSbox::optimise(&boxes[i]))
}

/// cps2_decrypt (l.635-713) : `rom` = mots du programme (length / 2 = rom.len()), bornes en mots
/// **incluses**. Les mots hors bornes sont recopiés tels quels.
pub fn cps2_decrypt(
    rom: &[u16],
    master_key: [u32; 2],
    lower_limit: u32,
    upper_limit: u32,
) -> Vec<u16> {
    let s1 = [&FN1_R1_BOXES, &FN1_R2_BOXES, &FN1_R3_BOXES, &FN1_R4_BOXES].map(optimise_sboxes);
    let s2 = [&FN2_R1_BOXES, &FN2_R2_BOXES, &FN2_R3_BOXES, &FN2_R4_BOXES].map(optimise_sboxes);
    let b1: [&[OptimisedSbox]; 4] = [&s1[0], &s1[1], &s1[2], &s1[3]];
    let b2: [&[OptimisedSbox]; 4] = [&s2[0], &s2[1], &s2[2], &s2[3]];

    // expand master key to 1st FN 96-bit key
    let mut key1 = expand_1st_key(&master_key);
    // add extra bits for s-boxes with less than 6 inputs (l.655-661)
    key1[0] ^= bit(key1[0], 1) << 4;
    key1[0] ^= bit(key1[0], 2) << 5;
    key1[0] ^= bit(key1[0], 8) << 11;
    key1[1] ^= bit(key1[1], 0) << 5;
    key1[1] ^= bit(key1[1], 8) << 11;
    key1[2] ^= bit(key1[2], 1) << 5;
    key1[2] ^= bit(key1[2], 8) << 11;

    let mut dec = vec![0u16; rom.len()];
    for i in 0..0x10000usize {
        // pass the address through FN1
        let seed = feistel(i as u16, &FN1_GROUP_A, &FN1_GROUP_B, b1, key1);
        // expand the result to 64-bit, XOR with the master key
        let mut subkey = expand_subkey(seed);
        subkey[0] ^= master_key[0];
        subkey[1] ^= master_key[1];
        // expand key to 2nd FN 96-bit key, extra bits (l.689-696)
        let mut key2 = expand_2nd_key(&subkey);
        key2[0] ^= bit(key2[0], 0) << 5;
        key2[0] ^= bit(key2[0], 6) << 11;
        key2[1] ^= bit(key2[1], 0) << 5;
        key2[1] ^= bit(key2[1], 1) << 4;
        key2[2] ^= bit(key2[2], 2) << 5;
        key2[2] ^= bit(key2[2], 3) << 4;
        key2[2] ^= bit(key2[2], 7) << 11;
        key2[3] ^= bit(key2[3], 1) << 5;

        // decrypt the opcodes
        let mut a = i;
        while a < rom.len() {
            dec[a] = if (lower_limit as usize..=upper_limit as usize).contains(&a) {
                feistel(rom[a], &FN2_GROUP_A, &FN2_GROUP_B, b2, key2)
            } else {
                rom[a]
            };
            a += 0x10000;
        }
    }
    dec
}

/// Clé décodée d'un fichier .key (init_cps2crypt) : clé maître et bornes en octets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cps2Key {
    pub master: [u32; 2],
    pub lower: u32,
    pub upper: u32,
}

/// init_cps2crypt (cps2.cpp l.10959-10986) : 160 bits du .key -> clé maître et bornes.
/// `key` doit contenir au moins 20 octets.
pub fn decode_key(key: &[u8]) -> Cps2Key {
    let mut decoded = [0u16; 10];
    for b in 0..10 * 16 {
        let bit_pos = (317 - b) % 160;
        if (key[bit_pos / 8] >> ((bit_pos ^ 7) % 8)) & 1 != 0 {
            decoded[b / 16] |= 0x8000 >> (b % 16);
        }
    }
    let master = [
        u32::from(decoded[0]) << 16 | u32::from(decoded[1]),
        u32::from(decoded[2]) << 16 | u32::from(decoded[3]),
    ];
    let (lower, upper) = if decoded[9] == 0xffff {
        // carte morte : seule la plage FF0000-FFFFFF est chiffrée
        (0xff0000, 0xffffff)
    } else {
        (0, (((u32::from(!decoded[9]) & 0x3ff) << 14) | 0x3fff) + 1)
    };
    Cps2Key {
        master,
        lower,
        upper,
    }
}

/// Déchiffre la région programme (mots gros-boutistes) : opcodes, même taille et même ordre d'octets.
pub fn decrypt_program(rom: &[u8], key: &Cps2Key) -> Vec<u8> {
    let words: Vec<u16> = rom
        .as_chunks::<2>()
        .0
        .iter()
        .map(|w| u16::from_be_bytes(*w))
        .collect();
    cps2_decrypt(&words, key.master, key.lower / 2, key.upper / 2)
        .iter()
        .flat_map(|w| w.to_be_bytes())
        .collect()
}

#[cfg(test)]
#[path = "cps2crypt_tests.rs"]
mod tests;
