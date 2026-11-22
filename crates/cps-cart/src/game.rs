//! Chargement d'un jeu : construction des régions, DIP par défaut, ticks CPU.
//! spec: docs/spec/00 §Horloges du matériel, docs/spec/01 §DIP, docs/spec/04 §Jeux cibles

use crate::config::Config;
use crate::dips::default_dips as dips_default;
use crate::error::CartError;
use crate::regions::build_regions;
use crate::romset::{find_game, GameInfo};
use crate::source::{sources_for, FileSource};

/// Structure représentant un jeu chargé.
#[derive(Clone, Debug)]
pub struct Game {
    pub info: &'static GameInfo,
    pub regions: Regions,
    pub ticks_per_68k: u64,
    pub dips: cps_common::Dips,
    /// init_sf2ee (cps1.cpp l.14594-14601) : CPS-B en 8001C0 au lieu de 800140.
    pub cpsb_at_1c0: bool,
    /// QSound : opcodes Z80 déchiffrés (Kabuki, 0x8000 octets) ; la région audiocpu contient
    /// alors les données déchiffrées.
    pub audio_opcodes: Option<Vec<u8>>,
    /// QSound : ROM interne du DSP (`dl-1425.bin`, cherchée aussi dans `qsound.zip`).
    pub qsound_dsp: Option<Vec<u8>>,
}

/// CRC de `dl-1425.bin` (qsoundhle.cpp, ROM_START( qsound_hle )).
pub const DL1425_CRC: u32 = 0xD6CF_5EF5;

/// Nombre de ticks (5,04 GHz) par cycle 68000 selon la machine.
pub fn ticks_per_68k(machine: &str) -> Result<u64, CartError> {
    match machine {
        "cps1_10MHz" => Ok(504),
        "cps1_12MHz" => Ok(420),
        // forgottn = cps1_10MHz + uPD4701 (cps1.cpp l.3949-3957)
        "forgottn" => Ok(504),
        // qsound = cps1_12MHz + Z80 8 MHz et DSP QSound (cps1.cpp l.4013-4044)
        "qsound" => Ok(420),
        other => Err(CartError::UnsupportedMachine(other.to_string())),
    }
}

/// Charge un jeu depuis la configuration.
pub fn load_game(cfg: &Config, name: &str) -> Result<Game, CartError> {
    let info = find_game(name).ok_or_else(|| CartError::UnknownGame(name.to_string()))?;

    // DIP : ceux de la config si présents, sinon les valeurs par défaut.
    let dips = cfg
        .dips
        .get(name)
        .copied()
        .unwrap_or_else(|| dips_default(info.input));

    load_game_from(info, &sources_for(cfg, info), cfg.allow_bad_crc, Some(dips))
}

/// Charge un jeu depuis une source de fichiers donnée.
pub fn load_game_from(
    info: &'static GameInfo,
    src: &dyn FileSource,
    allow_bad_crc: bool,
    dips: Option<cps_common::Dips>,
) -> Result<Game, CartError> {
    let ticks = ticks_per_68k(info.machine)?;

    // Construit les régions.
    let mut regions = build_regions(info.romset, src, allow_bad_crc)?;
    let audio_opcodes = crate::kabuki::decode(info.init, &mut regions.audiocpu);
    let qsound_dsp = if info.machine == "qsound" {
        Some(src.fetch("dl-1425.bin", DL1425_CRC)?)
    } else {
        None
    };

    Ok(Game {
        info,
        regions,
        ticks_per_68k: ticks,
        dips: dips.unwrap_or_else(|| dips_default(info.input)),
        cpsb_at_1c0: info.init == "init_sf2ee",
        audio_opcodes,
        qsound_dsp,
    })
}

/// Types de régions construites pour un jeu CPS1.
#[derive(Clone, Debug, Default)]
pub struct Regions {
    pub maincpu: Vec<u8>,
    pub gfx: Vec<u8>,
    pub audiocpu: Vec<u8>,
    pub oki: Vec<u8>,
    pub stars: Option<Vec<u8>>,
    /// Échantillons QSound (vide hors QSound).
    pub qsound: Vec<u8>,
    /// Données lues par le 68000 en F00000 (protection de Slam Masters ; qsound_rom_r).
    pub user1: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dips::input_layout;

    #[test]
    fn ticks_per_68k_10mhz() {
        assert_eq!(ticks_per_68k("cps1_10MHz").unwrap(), 504);
    }

    #[test]
    fn ticks_per_68k_12mhz() {
        assert_eq!(ticks_per_68k("cps1_12MHz").unwrap(), 420);
    }

    #[test]
    fn ticks_per_68k_unknown() {
        let result = ticks_per_68k("pang3");
        assert!(matches!(result, Err(CartError::UnsupportedMachine(_))));
    }

    #[test]
    fn default_dips_via_game_module() {
        assert_eq!(
            dips_default("ghouls"),
            cps_common::Dips {
                dswa: 0xFF,
                dswb: 0xFD,
                dswc: 0xFF
            }
        );
        assert_eq!(input_layout("sf2"), (2, 6));
    }

    /// ROM réelles : nécessite cps1.toml (roms_dir). Lancer avec `cargo test -p cps-cart -- --include-ignored`.
    fn cfg() -> Config {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../cps1.toml");
        Config::load(&p).expect("cps1.toml")
    }

    #[test]
    #[ignore]
    fn real_ghouls() {
        let g = load_game(&cfg(), "ghouls").expect("ghouls");
        assert_eq!(g.regions.maincpu.len(), 0x400000);
        assert_eq!(g.regions.gfx.len(), 0x300000);
        assert_eq!(g.regions.audiocpu.len(), 0x18000);
        assert_eq!(g.regions.oki, vec![0xFF; 0x40000]);
        assert_eq!(g.ticks_per_68k, 504);
    }

    #[test]
    #[ignore]
    fn real_sf2_and_sf2ceea() {
        let g = load_game(&cfg(), "sf2").expect("sf2");
        assert_eq!(g.regions.gfx.len(), 0x600000);
        assert_eq!(
            g.dips,
            cps_common::Dips {
                dswa: 0xFF,
                dswb: 0xFC,
                dswc: 0x9F
            }
        );
        let g = load_game(&cfg(), "sf2ceea")
            .expect("sf2ceea (fichiers trouvés par CRC dans sf2ce.zip)");
        assert_eq!(g.ticks_per_68k, 420);
    }
}
