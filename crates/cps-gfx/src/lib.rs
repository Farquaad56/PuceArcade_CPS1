//! cps-gfx — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
pub mod cps2;
pub mod cps2_render;
pub mod cps2_sprites;
#[rustfmt::skip]
pub mod config_data;
pub mod decode;
pub mod mapper;
pub mod palette;
pub mod raster;
pub mod regs;
pub mod render;
pub mod sprites;
pub mod sprites_render;
pub mod stars;
pub mod tilemap;
pub mod video_base;
pub mod video_io;

#[cfg(test)]
mod tests;
