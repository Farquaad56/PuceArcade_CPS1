//! OKI ADPCM state machine tests. spec: docs/spec/03 §OKI

use cps_oki::tables::{DIFF_LOOKUP, INDEX_SHIFT};
use cps_oki::OkiAdpcmState;

#[test]
fn diff_lookup_first_row() {
    // First row (step 0) should be [2,6,10,14,18,22,26,30,-2,-6,...]
    assert_eq!(&DIFF_LOOKUP[0..8], &[2, 6, 10, 14, 18, 22, 26, 30]);
}

#[test]
fn diff_lookup_last_entry() {
    // Last entry (step 48, nibble 7) should be 2910
    assert_eq!(DIFF_LOOKUP[48 * 16 + 7], 2910);
}

#[test]
fn index_shift_values() {
    assert_eq!(&INDEX_SHIFT[..], &[-1, -1, -1, -1, 2, 4, 6, 8]);
}

#[test]
fn adpcm_sequence_mame_measured() {
    // Sequence of nibbles [7,7,7,0,8,15,3,12] from reset -> outputs [30,93,229,248,231,-12,230,-54]
    let mut state = OkiAdpcmState::new();
    let nibbles = [7u8, 7, 7, 0, 8, 15, 3, 12];
    let expected = [30i16, 93, 229, 248, 231, -12, 230, -54];

    for (nib, exp) in nibbles.iter().zip(expected.iter()) {
        let out = state.clock(*nib);
        assert_eq!(out, *exp, "nibble {} -> expected {}, got {}", nib, exp, out);
    }
}

#[test]
fn adpcm_clamp_negative() {
    // 20 × nibble 15 should clamp to -2048
    let mut state = OkiAdpcmState::new();
    for _ in 0..20 {
        state.clock(15);
    }
    assert_eq!(state.output(), -2048);
}

#[test]
fn adpcm_clamp_positive() {
    // 10 × nibble 7 should clamp to 2047
    let mut state = OkiAdpcmState::new();
    for _ in 0..10 {
        state.clock(7);
    }
    assert_eq!(state.output(), 2047);
}

#[test]
fn adpcm_save_restore() {
    // save/restore should return signal and step
    let mut state = OkiAdpcmState::new();

    // Advance some
    for nib in [7u8, 7, 7, 0] {
        state.clock(nib);
    }

    let saved_signal = state.signal;
    let saved_step = state.step;

    state.save();

    // Advance more
    for nib in [8u8, 15, 3] {
        state.clock(nib);
    }

    assert_ne!(state.signal, saved_signal);

    // Restore
    state.restore();
    assert_eq!(state.signal, saved_signal);
    assert_eq!(state.step, saved_step);
}

#[test]
fn adpcm_reset() {
    let mut state = OkiAdpcmState::new();
    for nib in [7u8, 7, 7] {
        state.clock(nib);
    }
    assert_ne!(state.signal, 0);

    state.reset();
    assert_eq!(state.signal, 0);
    assert_eq!(state.step, 0);
    assert!(!state.saved);
}
