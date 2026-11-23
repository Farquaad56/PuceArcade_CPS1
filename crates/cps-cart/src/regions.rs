//! Construction des régions mémoire à partir d'un romset.
//! spec: docs/spec/04 §Sémantique des opérations

use crate::error::CartError;
use crate::game::Regions;
use crate::romset::{Load, Op, RegionDef, RomSetDef};
use crate::source::FileSource;
use cps_common::crc32_bytes;

/// Fichier « courant » d'une région (pour Continue / Reload / Ignore).
struct Current {
    name: String,
    data: Vec<u8>,
    pos: usize,
    kind: Load,
}

/// Construit les régions mémoire d'un romset (spec 04 §Sémantique), dans l'ordre du source.
pub fn build_regions(
    def: &RomSetDef,
    src: &dyn FileSource,
    allow_bad_crc: bool,
) -> Result<Regions, CartError> {
    let mut regions = Regions::default();
    for rdef in def.regions {
        let data = build_region(rdef, &regions, src, allow_bad_crc)?;
        match rdef.name {
            "maincpu" => regions.maincpu = data,
            "gfx" => regions.gfx = data,
            "audiocpu" => regions.audiocpu = data,
            "oki" => regions.oki = data,
            "stars" => regions.stars = Some(data),
            "qsound" => regions.qsound = data,
            // « user1 » dans MAME 0.287, « audiorom_raw » dans la référence : ROM Z80 encore
            // chiffrée (ROM_COPY avant kabuki_setup), lue par le 68000 en F00000
            "user1" | "audiorom_raw" => regions.user1 = Some(data),
            "key" => regions.key = Some(data),
            _ => {}
        }
    }
    Ok(regions)
}

fn build_region(
    rdef: &RegionDef,
    built: &Regions,
    src: &dyn FileSource,
    allow_bad_crc: bool,
) -> Result<Vec<u8>, CartError> {
    let mut region = vec![rdef.fill; rdef.size as usize];
    let mut cur: Option<Current> = None;
    for op in rdef.ops {
        match *op {
            Op::Load {
                kind,
                file,
                offset,
                size,
                crc,
            } => {
                let data = src.fetch(file, crc)?;
                let got = crc32_bytes(&data);
                if got != crc && !allow_bad_crc {
                    return Err(CartError::BadCrc {
                        file: file.to_string(),
                        expected: crc,
                        got,
                    });
                }
                let mut c = Current {
                    name: file.to_string(),
                    data,
                    pos: 0,
                    kind,
                };
                read_into(
                    &mut region,
                    rdef.name,
                    &mut c,
                    offset as usize,
                    size as usize,
                )?;
                cur = Some(c);
            }
            Op::Continue { offset, size } => {
                let c = current(&mut cur, rdef.name, offset)?;
                read_into(&mut region, rdef.name, c, offset as usize, size as usize)?;
            }
            Op::Reload { offset, size } => {
                let c = current(&mut cur, rdef.name, offset)?;
                c.pos = 0;
                read_into(&mut region, rdef.name, c, offset as usize, size as usize)?;
            }
            Op::Ignore { size } => {
                if let Some(c) = cur.as_mut() {
                    c.pos += size as usize;
                }
            }
            Op::Copy {
                src: from,
                src_offset,
                offset,
                size,
            } => {
                let s = region_by_name(built, from)?;
                let (so, o, n) = (src_offset as usize, offset as usize, size as usize);
                if so + n > s.len() {
                    return Err(CartError::OutOfRegion {
                        region: from.to_string(),
                        offset: so + n,
                    });
                }
                check(&region, rdef.name, o + n)?;
                region[o..o + n].copy_from_slice(&s[so..so + n]);
            }
            Op::Fill {
                offset,
                size,
                value,
            } => {
                let (o, n) = (offset as usize, size as usize);
                check(&region, rdef.name, o + n)?;
                region[o..o + n].fill(value as u8);
            }
            Op::NoDump => {}
        }
    }
    Ok(region)
}

fn current<'a>(
    cur: &'a mut Option<Current>,
    region: &str,
    offset: u32,
) -> Result<&'a mut Current, CartError> {
    cur.as_mut().ok_or_else(|| CartError::OutOfRegion {
        region: region.to_string(),
        offset: offset as usize,
    })
}

fn check(region: &[u8], name: &str, end: usize) -> Result<(), CartError> {
    if end > region.len() {
        return Err(CartError::OutOfRegion {
            region: name.to_string(),
            offset: end - 1,
        });
    }
    Ok(())
}

/// Lit `size` octets du fichier courant (à partir de sa position) et les place dans la région selon son type.
fn read_into(
    region: &mut [u8],
    name: &str,
    c: &mut Current,
    offset: usize,
    size: usize,
) -> Result<(), CartError> {
    let avail = c.data.len().saturating_sub(c.pos);
    if avail < size {
        return Err(CartError::BadSize {
            file: c.name.clone(),
            expected: c.pos + size,
            got: c.data.len(),
        });
    }
    let bytes = &c.data[c.pos..c.pos + size];
    for (i, &b) in bytes.iter().enumerate() {
        let dst = offset + dest_index(c.kind, i);
        if dst >= region.len() {
            return Err(CartError::OutOfRegion {
                region: name.to_string(),
                offset: dst,
            });
        }
        region[dst] = b;
    }
    c.pos += size;
    Ok(())
}

/// Position dans la région (relative à l'offset) de l'octet i du fichier. spec: docs/spec/04 §Sémantique
fn dest_index(kind: Load, i: usize) -> usize {
    match kind {
        Load::Byte | Load::Word16 => i,
        Load::Byte16 => 2 * i,
        Load::Word16Swap => i ^ 1,
        Load::Word64 => 8 * (i / 2) + (i % 2),
        Load::Byte64 => 8 * i,
        Load::Byte32 => 4 * i,
    }
}

fn region_by_name<'a>(r: &'a Regions, name: &str) -> Result<&'a [u8], CartError> {
    match name {
        "maincpu" => Ok(&r.maincpu),
        "gfx" => Ok(&r.gfx),
        "audiocpu" => Ok(&r.audiocpu),
        "oki" => Ok(&r.oki),
        "stars" => r.stars.as_deref().ok_or_else(|| CartError::OutOfRegion {
            region: name.to_string(),
            offset: 0,
        }),
        _ => Err(CartError::OutOfRegion {
            region: name.to_string(),
            offset: 0,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug)]
    struct MemSource(BTreeMap<&'static str, Vec<u8>>);
    impl FileSource for MemSource {
        fn fetch(&self, file: &str, _crc: u32) -> Result<Vec<u8>, CartError> {
            self.0
                .get(file)
                .cloned()
                .ok_or_else(|| CartError::MissingFile {
                    file: file.to_string(),
                })
        }
    }

    fn src(files: &[(&'static str, &[u8])]) -> MemSource {
        MemSource(files.iter().map(|(n, d)| (*n, d.to_vec())).collect())
    }

    /// RomSetDef d'une seule région « maincpu » (définitions « leakées » : 'static exigé par les types générés).
    fn set(size: u32, fill: u8, ops: Vec<Op>) -> &'static RomSetDef {
        let ops: &'static [Op] = Box::leak(ops.into_boxed_slice());
        let regions: &'static [RegionDef] = Box::leak(
            vec![RegionDef {
                name: "maincpu",
                size,
                fill,
                ops,
            }]
            .into_boxed_slice(),
        );
        Box::leak(Box::new(RomSetDef {
            name: "test",
            regions,
        }))
    }

    fn load(kind: Load, offset: u32, size: u32) -> Op {
        Op::Load {
            kind,
            file: "f",
            offset,
            size,
            crc: 0,
        }
    }

    fn build(def: &RomSetDef, data: &[u8]) -> Result<Vec<u8>, CartError> {
        build_regions(def, &src(&[("f", data)]), true).map(|r| r.maincpu)
    }

    #[test]
    fn load_byte16() {
        let r = build(set(16, 0, vec![load(Load::Byte16, 1, 3)]), &[1, 2, 3]).unwrap();
        assert_eq!((r[1], r[3], r[5]), (1, 2, 3));
        assert_eq!(r[0], 0);
    }

    #[test]
    fn load_word16_swap() {
        let r = build(
            set(4, 0, vec![load(Load::Word16Swap, 0, 4)]),
            &[0x34, 0x12, 0x78, 0x56],
        )
        .unwrap();
        assert_eq!(r, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn load_word16_direct() {
        let r = build(set(4, 0, vec![load(Load::Word16, 0, 4)]), &[1, 2, 3, 4]).unwrap();
        assert_eq!(r, vec![1, 2, 3, 4]);
    }

    #[test]
    fn load_word64() {
        let r = build(set(16, 0, vec![load(Load::Word64, 2, 4)]), &[1, 2, 3, 4]).unwrap();
        assert_eq!((r[2], r[3], r[10], r[11]), (1, 2, 3, 4));
    }

    #[test]
    fn load_byte64_and_byte32() {
        let r = build(set(16, 0, vec![load(Load::Byte64, 1, 2)]), &[9, 8]).unwrap();
        assert_eq!((r[1], r[9]), (9, 8));
        let r = build(set(16, 0, vec![load(Load::Byte32, 0, 3)]), &[7, 6, 5]).unwrap();
        assert_eq!((r[0], r[4], r[8]), (7, 6, 5));
    }

    #[test]
    fn load_reads_only_size_bytes() {
        let r = build(set(8, 0xEE, vec![load(Load::Byte, 0, 2)]), &[1, 2, 3, 4]).unwrap();
        assert_eq!(&r[..4], &[1, 2, 0xEE, 0xEE]);
    }

    #[test]
    fn continue_and_reload() {
        let ops = vec![
            load(Load::Byte, 0, 2),
            Op::Continue {
                offset: 0x10,
                size: 2,
            },
            Op::Reload {
                offset: 0x20,
                size: 1,
            },
        ];
        let r = build(set(0x40, 0, ops), &[1, 2, 3, 4]).unwrap();
        assert_eq!((r[0], r[1], r[0x10], r[0x11], r[0x20]), (1, 2, 3, 4, 1));
    }

    #[test]
    fn ignore_skips_bytes() {
        let ops = vec![
            load(Load::Byte, 0, 1),
            Op::Ignore { size: 2 },
            Op::Continue { offset: 4, size: 1 },
        ];
        let r = build(set(8, 0, ops), &[1, 2, 3, 4]).unwrap();
        assert_eq!(r[4], 4);
    }

    #[test]
    fn fill_and_erase_ff() {
        let r = build(
            set(
                8,
                0xFF,
                vec![Op::Fill {
                    offset: 2,
                    size: 3,
                    value: 0x5A,
                }],
            ),
            &[],
        )
        .unwrap();
        assert_eq!(r, vec![0xFF, 0xFF, 0x5A, 0x5A, 0x5A, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn copy_from_built_region() {
        let ops_a: &'static [Op] = Box::leak(vec![load(Load::Byte, 0, 4)].into_boxed_slice());
        let ops_b: &'static [Op] = Box::leak(
            vec![Op::Copy {
                src: "gfx",
                src_offset: 1,
                offset: 0,
                size: 2,
            }]
            .into_boxed_slice(),
        );
        let regions: &'static [RegionDef] = Box::leak(
            vec![
                RegionDef {
                    name: "gfx",
                    size: 4,
                    fill: 0,
                    ops: ops_a,
                },
                RegionDef {
                    name: "stars",
                    size: 4,
                    fill: 0,
                    ops: ops_b,
                },
            ]
            .into_boxed_slice(),
        );
        let def = RomSetDef { name: "t", regions };
        let r = build_regions(&def, &src(&[("f", &[1, 2, 3, 4])]), true).unwrap();
        assert_eq!(r.stars.unwrap(), vec![2, 3, 0, 0]);
    }

    #[test]
    fn errors() {
        let def = set(
            4,
            0,
            vec![Op::Load {
                kind: Load::Byte,
                file: "f",
                offset: 0,
                size: 2,
                crc: 0x1234,
            }],
        );
        let e = build_regions(def, &src(&[("f", &[1, 2])]), false).unwrap_err();
        assert_eq!(
            e,
            CartError::BadCrc {
                file: "f".into(),
                expected: 0x1234,
                got: crc32_bytes(&[1, 2])
            }
        );
        assert!(matches!(
            build(set(4, 0, vec![load(Load::Byte, 0, 4)]), &[1, 2]),
            Err(CartError::BadSize { .. })
        ));
        assert!(matches!(
            build(set(4, 0, vec![load(Load::Byte, 3, 2)]), &[1, 2]),
            Err(CartError::OutOfRegion { .. })
        ));
        assert!(matches!(
            build_regions(def, &src(&[]), true),
            Err(CartError::MissingFile { .. })
        ));
    }
}
