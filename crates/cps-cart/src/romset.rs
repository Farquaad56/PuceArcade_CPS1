//! Types pour les définitions de romsets CPS1 (générées par tools/extract_romsets.py).
//! spec: docs/spec/04 §Sémantique des opérations

use crate::romsets_data;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Load {
    Byte,
    Byte16,
    Word16,
    Word16Swap,
    Word64,
    Byte64,
    Byte32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Load {
        kind: Load,
        file: &'static str,
        offset: u32,
        size: u32,
        crc: u32,
    },
    Continue {
        offset: u32,
        size: u32,
    },
    Reload {
        offset: u32,
        size: u32,
    },
    Ignore {
        size: u32,
    },
    Copy {
        src: &'static str,
        src_offset: u32,
        offset: u32,
        size: u32,
    },
    Fill {
        offset: u32,
        size: u32,
        value: u32,
    },
    NoDump,
}

#[derive(Debug)]
pub struct RegionDef {
    pub name: &'static str,
    pub size: u32,
    pub fill: u8,
    pub ops: &'static [Op],
}

#[derive(Debug)]
pub struct RomSetDef {
    pub name: &'static str,
    pub regions: &'static [RegionDef],
}

#[derive(Debug)]
pub struct GameInfo {
    pub name: &'static str,
    pub parent: &'static str,
    pub machine: &'static str,
    pub input: &'static str,
    pub init: &'static str,
    pub rotation: u16, // 0 ou 270 (ROTxx de MAME)
    pub maker: &'static str,
    pub title: &'static str,
    pub not_working: bool,
    pub romset: &'static RomSetDef,
}

/// Recherche linéaire dans la table des jeux CPS1, puis dans celle du CPS2.
pub fn find_game(name: &str) -> Option<&'static GameInfo> {
    romsets_data::GAMES
        .iter()
        .chain(crate::romsets2_data::GAMES2.iter())
        .find(|g| g.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_count() {
        assert_eq!(romsets_data::GAMES.len(), 229);
    }

    #[test]
    fn find_ghouls_machine() {
        let g = find_game("ghouls").expect("ghouls doit exister");
        assert_eq!(g.machine, "cps1_10MHz");
    }

    #[test]
    fn find_sf2ce_machine() {
        let g = find_game("sf2ce").expect("sf2ce doit exister");
        assert_eq!(g.machine, "cps1_12MHz");
    }

    #[test]
    fn find_cps2_game() {
        let g = find_game("ssf2ud").expect("ssf2ud doit exister");
        assert_eq!(g.parent, "ssf2");
        assert_eq!(g.machine, "dead_cps2");
        assert_eq!(g.input, "cps2_2p6b");
        assert_eq!(g.init, "init_cps2");
    }

    #[test]
    fn find_unknown() {
        assert!(find_game("zzz").is_none());
    }

    #[test]
    fn rotation_1941() {
        let g = find_game("1941").expect("1941 doit exister");
        assert_eq!(g.rotation, 270);
    }

    #[test]
    fn rotation_ghouls() {
        let g = find_game("ghouls").expect("ghouls doit exister");
        assert_eq!(g.rotation, 0);
    }

    #[test]
    fn count_rotation_270() {
        let count = romsets_data::GAMES
            .iter()
            .filter(|g| g.rotation == 270)
            .count();
        assert_eq!(count, 15);
    }

    #[test]
    fn sf2ce_first_maincpu_op() {
        let g = find_game("sf2ce").expect("sf2ce doit exister");
        let maincpu = g
            .romset
            .regions
            .iter()
            .find(|r| r.name == "maincpu")
            .expect("région maincpu");
        assert!(!maincpu.ops.is_empty());
        match &maincpu.ops[0] {
            Op::Load {
                kind,
                file,
                offset,
                size,
                crc,
            } => {
                assert_eq!(*kind, Load::Word16Swap);
                assert_eq!(*file, "s92e_23b.8f");
                assert_eq!(*offset, 0);
                assert_eq!(*size, 0x80000);
                assert_eq!(*crc, 0x0AAA1A3A);
            }
            other => panic!("attendu Op::Load, obtenu {:?}", other),
        }
    }

    #[test]
    fn ghouls_region_order() {
        let g = find_game("ghouls").expect("ghouls doit exister");
        assert_eq!(g.romset.regions.len(), 4);
        assert_eq!(g.romset.regions[0].name, "maincpu");
        assert_eq!(g.romset.regions[1].name, "gfx");
        assert_eq!(g.romset.regions[2].name, "audiocpu");
        assert_eq!(g.romset.regions[3].name, "oki");
    }

    #[test]
    fn ghouls_oki_region() {
        let g = find_game("ghouls").expect("ghouls doit exister");
        let oki = g
            .romset
            .regions
            .iter()
            .find(|r| r.name == "oki")
            .expect("région oki");
        assert_eq!(oki.fill, 0xFF);
        assert!(oki.ops.is_empty());
    }

    #[test]
    fn all_load_ops_have_size() {
        for game in romsets_data::GAMES.iter() {
            for region in game.romset.regions {
                for op in region.ops {
                    if let Op::Load { size, .. } = op {
                        assert!(
                            *size > 0,
                            "Load sans taille dans {} / {}",
                            game.name,
                            region.name
                        );
                    }
                }
            }
        }
    }
}
