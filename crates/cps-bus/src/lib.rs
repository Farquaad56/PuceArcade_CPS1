//! cps-bus — carte mémoire 68000 et E/S CPS1.
//! spec: docs/spec/01 §68000

#![forbid(unsafe_code)]

pub mod map68k;
pub use map68k::{Bus68k, SoundCmd, VideoPort};

pub mod video_port;
pub use video_port::{new_cps_bus, CpsBus};

pub mod sound;
pub mod sound_reset;
pub use sound::SoundBoard;

pub mod chips_stub;
pub use chips_stub::NullChips;

pub mod chips;
pub mod cps2_io;
pub use cps2_io::{Cps2Inputs, Cps2Io};
pub mod eeprom;
pub use eeprom::Eeprom93c46;
pub use sound::QSoundSide;
pub mod upd4701;
pub use chips::Chips;
pub use upd4701::Upd4701;

#[cfg(test)]
mod quirk_tests;
