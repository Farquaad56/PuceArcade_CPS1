//! cps-oki — OKI MSM6295 ADPCM emulation (4 voices). spec: docs/spec/03 §OKI
#![forbid(unsafe_code)]

#[rustfmt::skip]
pub mod tables;
pub mod adpcm;
pub mod okim6295;

pub use adpcm::OkiAdpcmState;
pub use okim6295::{OkiVoice, Okim6295};
