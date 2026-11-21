//! CPS-B configurations et recherche de configuration par nom de jeu (spec: docs/spec/02 §Config).
#![allow(clippy::unreadable_literal)]

use crate::config_data;

/// Configuration CPS-B pour un jeu donné.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpsBConfig {
    pub cpsb_addr: i32,
    pub cpsb_value: i32,
    pub mult_factor1: i32,
    pub mult_factor2: i32,
    pub mult_result_lo: i32,
    pub mult_result_hi: i32,
    pub unknown1: i32,
    pub unknown2: i32,
    pub unknown3: i32,
    pub layer_control: i32,
    pub priority: [i32; 4],
    pub palette_control: i32,
    pub layer_enable_mask: [i32; 5],
}

/// Plage de codes tuile pour le mapper de banques ROM graphique.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GfxRange {
    pub kind: u32,
    pub start: i32,
    pub end: i32,
    pub bank: usize,
}

/// Mapper de banques pour un jeu donné.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mapper {
    pub bank_sizes: [i32; 4],
    pub ranges: &'static [GfxRange],
}

/// Configuration complète d'un jeu CPS1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameConfig {
    pub name: &'static str,
    pub cpsb: CpsBConfig,
    pub mapper: Mapper,
    pub in2_addr: i32,
    pub in3_addr: i32,
    pub out2_addr: i32,
    pub bootleg_kludge: i32,
}

/// Recherche la configuration pour un jeu donné (MACHINE_RESET cps, l.2046-2075).
/// Si le nom n'est pas trouvé, retourne la configuration "cps2".
pub fn find_config(name: &str) -> &'static GameConfig {
    for cfg in config_data::GAMES.iter() {
        if cfg.name == name {
            return cfg;
        }
    }
    // Fallback to cps2 config (last entry in the table)
    let cps2 = config_data::GAMES
        .iter()
        .find(|c| c.name == "cps2")
        .unwrap();
    cps2
}
