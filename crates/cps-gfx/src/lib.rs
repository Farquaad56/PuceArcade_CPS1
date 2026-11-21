//! cps-gfx — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
#[rustfmt::skip]
pub mod config_data;
pub mod mapper;

#[cfg(test)]
mod tests;
