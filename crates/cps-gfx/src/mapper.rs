// license:BSD-3-Clause (port de ref/mame-cps1/cps1_v.cpp l.2385-2430)
// copyright-holders: Paul Leaman
//! Mapper de banques ROM graphique CPS1 (spec: docs/spec/02 §ROM graphique).

use crate::config::Mapper;

/// Types de tuiles pour le mapper.
pub const GFXTYPE_SPRITES: u32 = 1;
pub const GFXTYPE_SCROLL1: u32 = 2;
pub const GFXTYPE_SCROLL2: u32 = 4;
pub const GFXTYPE_SCROLL3: u32 = 8;
pub const GFXTYPE_STARS: u32 = 16;

/// Mappe un code de tuile vers l'adresse dans la ROM graphique (port de gfxrom_bank_mapper).
/// Retourne -1 si aucune plage ne correspond.
pub fn gfxrom_bank_mapper(mapper: &Mapper, kind: u32, code: i32) -> i32 {
    let shift = match kind {
        GFXTYPE_SPRITES => 1,
        GFXTYPE_SCROLL1 => 0,
        GFXTYPE_SCROLL2 => 1,
        GFXTYPE_SCROLL3 => 3,
        _ => 0, // STARS et autres : pas de décalage défini
    };

    let code = code << shift;

    for range in mapper.ranges.iter() {
        if code >= range.start && code <= range.end && (range.kind & kind) != 0 {
            let mut base = 0;
            for i in 0..range.bank {
                base += mapper.bank_sizes[i];
            }
            return (base + (code & (mapper.bank_sizes[range.bank] - 1))) >> shift;
        }
    }

    // Aucune plage ne correspond : tuile vide
    -1
}
