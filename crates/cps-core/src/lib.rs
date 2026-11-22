//! cps-core — Machine, ordonnanceur, interruptions, trame, audio.
//! Voir docs/ARCHITECTURE.md.
#![forbid(unsafe_code)]

pub mod host68k;
pub mod machine;
pub mod sched;

pub use machine::Machine;
