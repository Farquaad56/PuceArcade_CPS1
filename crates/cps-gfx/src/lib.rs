//! cps-gfx — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
#[rustfmt::skip]
pub mod config_data;
pub mod decode;
pub mod mapper;
pub mod palette;
pub mod regs;
pub mod video_base;
pub mod video_io;

#[cfg(test)]
mod tests;
