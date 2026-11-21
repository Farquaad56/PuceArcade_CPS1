//! Configuration locale (cps1.toml).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use cps_common::Dips;

use crate::error::CartError;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub roms_dir: PathBuf,
    pub saves_dir: PathBuf,
    pub mame: Option<PathBuf>,
    pub game: String,
    pub allow_bad_crc: bool,
    pub dips: BTreeMap<String, Dips>,
}

impl Config {
    pub const DEFAULT_PATH: &'static str = "cps1.toml";

    /// Parse un fichier de configuration TOML.
    pub fn parse(text: &str) -> Result<Config, CartError> {
        let table: toml::Table = text.parse().map_err(|e: toml::de::Error| CartError::Config(e.to_string()))?;

        // [paths] — obligatoire
        let paths_table = table.get("paths")
            .and_then(|v| v.as_table())
            .ok_or_else(|| CartError::Config("[paths] manquant ou invalide".to_string()))?;

        let roms_dir = paths_table.get("roms_dir")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .ok_or_else(|| CartError::Config("paths.roms_dir manquant ou invalide".to_string()))?;

        let saves_dir = paths_table.get("saves_dir")
            .and_then(|v| v.as_str())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("saves"));

        let mame = paths_table.get("mame")
            .and_then(|v| v.as_str())
            .map(PathBuf::from);

        // [system] — optionnel
        let system_table = table.get("system").and_then(|v| v.as_table());

        let game = system_table
            .and_then(|t| t.get("game"))
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| "ghouls".to_string());

        let allow_bad_crc = system_table
            .and_then(|t| t.get("allow_bad_crc"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // [dips] — optionnel
        let dips = Self::parse_dips(&table)?;

        Ok(Config {
            roms_dir,
            saves_dir,
            mame,
            game,
            allow_bad_crc,
            dips,
        })
    }

    /// Charger la configuration depuis un fichier.
    pub fn load(path: &Path) -> Result<Config, CartError> {
        let text = std::fs::read_to_string(path).map_err(|e| CartError::Io(e.to_string()))?;
        Self::parse(&text)
    }

    fn parse_dips(table: &toml::Table) -> Result<BTreeMap<String, Dips>, CartError> {
        let mut dips = BTreeMap::new();

        if let Some(dips_table) = table.get("dips").and_then(|v| v.as_table()) {
            for (game_name, value) in dips_table {
                let dip_table = value.as_table()
                    .ok_or_else(|| CartError::Config(format!("dips.{} invalide", game_name)))?;

                let dswa = Self::parse_dip_byte(dip_table, "dswa", game_name)?;
                let dswb = Self::parse_dip_byte(dip_table, "dswb", game_name)?;
                let dswc = Self::parse_dip_byte(dip_table, "dswc", game_name)?;

                dips.insert(game_name.clone(), Dips { dswa, dswb, dswc });
            }
        }

        Ok(dips)
    }

    fn parse_dip_byte(table: &toml::Table, key: &str, game_name: &str) -> Result<u8, CartError> {
        let value = table.get(key)
            .and_then(|v| v.as_integer())
            .ok_or_else(|| CartError::Config(format!("dips.{}.{} manquant", game_name, key)))?;

        if !(0..=255).contains(&value) {
            return Err(CartError::Config(
                format!("dips.{}.{} hors plage (0-255) : {}", game_name, key, value),
            ));
        }

        Ok(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_config() {
        let text = include_str!("../../../cps1.example.toml");
        let config = Config::parse(text).expect("exemplaire doit être valide");

        assert_eq!(config.roms_dir, PathBuf::from("H:/__Emulator__/PuceArcade_CPS1/roms"));
        assert_eq!(config.saves_dir, PathBuf::from("saves"));
        assert_eq!(config.mame, Some(PathBuf::from("X:/Mame_0287/mame.exe")));
        assert_eq!(config.game, "ghouls");
        assert!(!config.allow_bad_crc);
        assert!(config.dips.is_empty());
    }

    #[test]
    fn parse_dips() {
        let text = r#"
[paths]
roms_dir = "roms"

[dips]
ghouls = { dswa = 0xFE, dswb = 0xFD, dswc = 0x7F }
"#;
        let config = Config::parse(text).expect("dips doivent être valides");

        assert_eq!(config.dips.len(), 1);
        let ghouls_dips = &config.dips["ghouls"];
        assert_eq!(ghouls_dips.dswa, 0xFE);
        assert_eq!(ghouls_dips.dswb, 0xFD);
        assert_eq!(ghouls_dips.dswc, 0x7F);
    }

    #[test]
    fn missing_paths_section() {
        let text = r#"
[system]
game = "ghouls"
"#;
        let result = Config::parse(text);
        assert!(matches!(result, Err(CartError::Config(_))));
    }

    #[test]
    fn dip_value_out_of_range() {
        let text = r#"
[paths]
roms_dir = "roms"

[dips]
ghouls = { dswa = 300, dswb = 0xFF, dswc = 0xFF }
"#;
        let result = Config::parse(text);
        assert!(matches!(result, Err(CartError::Config(_))));
    }

    #[test]
    fn load_nonexistent_file() {
        let path = PathBuf::from("nonexistent_config.toml");
        let result = Config::load(&path);
        assert!(matches!(result, Err(CartError::Io(_))));
    }
}