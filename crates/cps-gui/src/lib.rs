//! cps-gui — carte CPS1 branchée sur le frontal JAMMA (puce-jamma-gui, ADR-002).
#![forbid(unsafe_code)]

pub mod board;
pub mod debug;
pub mod debug_audio;
pub mod debug_video;
pub mod driver;
pub mod inputs;
pub mod inputs_cps2;

pub use board::Cps1Board;
pub use driver::Cps1Driver;
