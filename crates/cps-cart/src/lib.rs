//! cps-cart — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
pub mod cps2crypt;
pub mod cps2load;
#[rustfmt::skip]
pub mod cps2crypt_tables;
pub mod dips;
#[rustfmt::skip]
pub mod dips_data;
pub mod error;
pub mod game;
pub mod kabuki;
pub mod regions;
#[rustfmt::skip]
pub mod romsets_data;
#[rustfmt::skip]
pub mod romsets2_data;
pub mod romset;
pub mod source;
pub mod support;

pub use config::Config;
pub use cps2load::Cps2Rom;
pub use dips::{dip_fields, input_layout};
pub use error::CartError;
pub use game::{load_game, load_game_from, ticks_68k_den, ticks_per_68k, Game};
pub use regions::build_regions;
pub use romset::{find_game, system_of, GameInfo, System};
pub use support::{support_of, supported_games, supported_games2, Support};
