//! cps-ym2151 — YM2151 FM sound chip emulation (CPS1).
//! Ported from ymfm by Aaron Giles (BSD 3-Clause License).
#![forbid(unsafe_code)]

pub mod common;
pub mod fm_operator;
pub mod fm_tables;
pub mod opm_regs;
mod opm_regs_ops;

#[cfg(feature = "serde")]
mod serde_arr;
