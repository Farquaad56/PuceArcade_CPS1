//! Chargement d'un jeu : construction des régions, DIP par défaut, ticks CPU.
//! spec: docs/spec/00 §Horloges du matériel, docs/spec/01 §DIP, docs/spec/04 §Jeux cibles

use crate::config::Config;
use crate::cps2crypt::{decode_key, decrypt_program};
use crate::cps2load::{cps2_gfx_decode, Cps2Rom};
use crate::dips::default_dips as dips_default;
use crate::error::CartError;
use crate::regions::build_regions;
use crate::romset::{find_game, system_of, GameInfo, System};
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
    /// Particularités de carte des bootlegs (entrées, miroir du CPS-B).
    pub quirks: crate::quirks::Quirks,
    /// QSound : opcodes Z80 déchiffrés (Kabuki, 0x8000 octets) ; la région audiocpu contient
    /// alors les données déchiffrées.
    pub audio_opcodes: Option<Vec<u8>>,
    /// QSound : ROM interne du DSP (`dl-1425.bin`, cherchée aussi dans `qsound.zip`).
    pub qsound_dsp: Option<Vec<u8>>,
    /// Dénominateur des ticks par cycle 68000 (ADR-007) : 1 pour le CPS1, 59 pour le CPS2.
    pub ticks_68k_den: u64,
    /// CPS2 : opcodes déchiffrés et variantes de carte (None pour le CPS1).
    pub cps2: Option<Cps2Rom>,
}

/// CRC de `dl-1425.bin` (qsoundhle.cpp, ROM_START( qsound_hle )).
pub const DL1425_CRC: u32 = 0xD6CF_5EF5;

/// Nombre de ticks (5,04 GHz) par cycle 68000 selon la machine.
pub fn ticks_per_68k(machine: &str) -> Result<u64, CartError> {
    match machine {
        "cps1_10MHz" => Ok(504),
        "cps1_12MHz" => Ok(420),
        // wofhfh = cps1_12MHz + une EEPROM que la carte mémoire ne relie pas (cps1.cpp l.4018-4024)
        "wofhfh" => Ok(420),
        // forgottn = cps1_10MHz + uPD4701 (cps1.cpp l.3949-3957)
        "forgottn" => Ok(504),
        // qsound = cps1_12MHz + Z80 8 MHz et DSP QSound (cps1.cpp l.4013-4044)
        "qsound" => Ok(420),
        // CPS2 : 16 MHz × 0,7375 = 11,8 MHz -> 25 200 / 59 ticks (ADR-007, spec: docs/spec/05 §Horloges)
        "cps2" | "dead_cps2" => Ok(25200),
        other => Err(CartError::UnsupportedMachine(other.to_string())),
    }
}

/// Dénominateur des ticks par cycle 68000 (ADR-007) : 59 pour le CPS2, 1 sinon.
pub fn ticks_68k_den(machine: &str) -> u64 {
    match machine {
        "cps2" | "dead_cps2" => 59,
        _ => 1,
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
    if system_of(info) == System::Cps2 {
        return load_cps2(info, regions, src, ticks, dips);
    }
    crate::quirks::descramble_gfx(info.init, &mut regions.gfx);
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
        quirks: crate::quirks::quirks_of(info),
        audio_opcodes,
        qsound_dsp,
        ticks_68k_den: 1,
        cps2: None,
    })
}

/// Fin du chargement d'un jeu CPS2 : graphismes, clé, opcodes, carte QSound (Z80 en clair).
/// spec: docs/spec/05 §Romsets, §Chiffrement, §Son
fn load_cps2(
    info: &'static GameInfo,
    mut regions: Regions,
    src: &dyn FileSource,
    ticks: u64,
    dips: Option<cps_common::Dips>,
) -> Result<Game, CartError> {
    cps2_gfx_decode(&mut regions.gfx);
    let key = regions
        .key
        .as_ref()
        .ok_or_else(|| CartError::MissingKey(info.name.to_string()))?;
    let opcodes = decrypt_program(&regions.maincpu, &decode_key(key));
    let audio_opcodes = Some(regions.audiocpu[..0x8000].to_vec());
    let qsound_dsp = Some(src.fetch("dl-1425.bin", DL1425_CRC)?);
    Ok(Game {
        info,
        regions,
        ticks_per_68k: ticks,
        dips: dips.unwrap_or_else(|| dips_default(info.input)),
        cpsb_at_1c0: false,
        quirks: crate::quirks::Quirks::default(),
        audio_opcodes,
        qsound_dsp,
        ticks_68k_den: ticks_68k_den(info.machine),
        cps2: Some(Cps2Rom {
            opcodes,
            dead: info.machine == "dead_cps2",
            singbrd: info.init == "init_singbrd",
        }),
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
    /// CPS2 : fichier .key (région « key »).
    pub key: Option<Vec<u8>>,
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
    fn ticks_cps2() {
        assert_eq!(ticks_per_68k("cps2").unwrap(), 25200);
        assert_eq!(ticks_68k_den("cps2"), 59);
        assert_eq!(ticks_68k_den("dead_cps2"), 59);
        assert_eq!(ticks_68k_den("cps1_10MHz"), 1);
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

    /// ROM réelles : ssf2.zip (fichiers de ssf2ud et phoenix.key, MAME 0.287) et qsound.zip.
    #[test]
    #[ignore]
    fn real_ssf2ud() {
        let g = load_game(&cfg(), "ssf2ud").expect("ssf2ud");
        assert_eq!(g.regions.maincpu.len(), 0x400000);
        assert_eq!(g.regions.gfx.len(), 0xC00000);
        assert_eq!(g.regions.audiocpu.len(), 0x50000);
        assert_eq!(g.regions.qsound.len(), 0x400000);
        let c = g.cps2.as_ref().expect("jeu CPS2");
        assert!(c.dead);
        assert_eq!(
            c.opcodes, g.regions.maincpu,
            "clé Phoenix : rien n'est déchiffré"
        );
        assert_eq!((g.ticks_per_68k, g.ticks_68k_den), (25200, 59));
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
