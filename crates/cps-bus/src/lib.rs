//! cps-bus — carte mémoire 68000 et E/S CPS1.
//! spec: docs/spec/01 §68000

#![forbid(unsafe_code)]

pub mod map68k;
pub use map68k::{Bus68k, SoundCmd, VideoPort};

pub mod sound;
pub use sound::SoundBoard;

pub mod chips_stub;
pub use chips_stub::NullChips;
