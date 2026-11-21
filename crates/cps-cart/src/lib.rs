//! cps-cart — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
pub mod dips;
#[rustfmt::skip]
pub mod dips_data;
pub mod error;
pub mod game;
pub mod regions;
#[rustfmt::skip]
pub mod romsets_data;
pub mod romset;
pub mod source;

pub use config::Config;
pub use dips::{dip_fields, input_layout};
pub use error::CartError;
pub use game::{load_game, load_game_from, ticks_per_68k, Game};
pub use regions::build_regions;
pub use romset::{find_game, GameInfo};
