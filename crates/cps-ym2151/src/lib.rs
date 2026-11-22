//! cps-ym2151 — YM2151 FM sound chip emulation (CPS1).
//! Ported from ymfm by Aaron Giles (BSD 3-Clause License).
#![forbid(unsafe_code)]

pub mod common;

#[cfg(feature = "serde")]
mod serde_arr;
