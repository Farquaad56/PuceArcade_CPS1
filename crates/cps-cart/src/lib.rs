//! cps-cart — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
pub mod error;
#[rustfmt::skip]
pub mod romsets_data;
pub mod romset;
pub mod source;

pub use config::Config;
pub use error::CartError;
pub use romset::{find_game, GameInfo};
