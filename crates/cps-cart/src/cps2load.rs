// license:BSD-3-Clause
// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald
//! Chargement CPS2 : remise en ordre des graphismes (unshuffle) et programme déchiffré.
//! spec: docs/spec/05 §Romsets, §Chiffrement
//! source: ref/mame-cps2/cps2.cpp l.768-796 (unshuffle, cps2_gfx_decode)

/// Données propres à un jeu CPS2 chargé.
#[derive(Clone, Debug)]
pub struct Cps2Rom {
    /// Opcodes du 68000 déchiffrés (même taille et même ordre d'octets que la région maincpu).
    pub opcodes: Vec<u8>,
    /// Machine `dead_cps2` (versions Phoenix) : registres output aussi en FFFFF0-FFFFFB.
    pub dead: bool,
    /// `init_singbrd` : pas de volume numérique (804030 = 0xD000).
    pub singbrd: bool,
}

/// Taille d'un `uint64_t` déplacé d'un bloc par unshuffle.
const BLOCK: usize = 8;

/// Échange les blocs de 8 octets `a` et `b` de `buf`.
fn swap_blocks(buf: &mut [u8], a: usize, b: usize) {
    for k in 0..BLOCK {
        buf.swap(a * BLOCK + k, b * BLOCK + k);
    }
}

/// unshuffle (l.768-786) sur `len` blocs de 8 octets.
pub fn unshuffle(buf: &mut [u8], len: usize) {
    if len == 2 {
        return;
    }
    // PORT-NOTE : assert(len % 4 == 0) de MAME, vérifié seulement en debug
    debug_assert!(len.is_multiple_of(4));
    let len = len / 2;
    unshuffle(buf, len);
    unshuffle(&mut buf[len * BLOCK..], len);
    for i in 0..len / 2 {
        swap_blocks(buf, len / 2 + i, len + i);
    }
}

/// cps2_gfx_decode (l.789-796) : unshuffle de chaque banque de 0x200000 octets.
pub fn cps2_gfx_decode(gfx: &mut [u8]) {
    const BANKSIZE: usize = 0x200000;
    for bank in gfx.chunks_mut(BANKSIZE) {
        let blocks = bank.len() / BLOCK;
        unshuffle(bank, blocks);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Blocs dont le premier octet vaut l'indice d'origine.
    fn order(n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n * BLOCK];
        for i in 0..n {
            buf[i * BLOCK] = i as u8;
        }
        unshuffle(&mut buf, n);
        (0..n).map(|i| buf[i * BLOCK]).collect()
    }

    #[test]
    fn unshuffle_small() {
        assert_eq!(order(4), [0, 2, 1, 3]);
        assert_eq!(order(8), [0, 2, 4, 6, 1, 3, 5, 7]);
        assert_eq!(
            order(16),
            [0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15]
        );
    }

    #[test]
    fn unshuffle_full_bank() {
        let n = 0x40000;
        let mut buf = vec![0u8; n * BLOCK];
        for i in 0..n {
            buf[i * BLOCK..i * BLOCK + 4].copy_from_slice(&(i as u32).to_be_bytes());
        }
        cps2_gfx_decode(&mut buf);
        let at = |p: usize| u32::from_be_bytes(buf[p * BLOCK..p * BLOCK + 4].try_into().unwrap());
        for (p, v) in [
            (0, 0),
            (1, 2),
            (2, 4),
            (5, 0xA),
            (0x20000, 1),
            (0x3FFFF, 0x3FFFF),
            (0x12345, 0x2468A),
        ] {
            assert_eq!(at(p), v, "position {p:#x}");
        }
    }
}
