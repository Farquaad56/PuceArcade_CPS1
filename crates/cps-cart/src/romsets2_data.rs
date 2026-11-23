// Fichier GÉNÉRÉ par tools/extract_romsets.py --cps2 depuis ref/mame-cps2/cps2.cpp — NE PAS MODIFIER À LA MAIN.
// license:BSD-3-Clause (données MAME, copyright-holders: Paul Leaman, Andreas Naive, Nicola Salmoria, Charles MacDonald)
//! Romsets CPS2 (régions maincpu, gfx, audiocpu, qsound, key) et table des jeux. spec: docs/spec/05 §Romsets

use crate::romset::{GameInfo, Load, Op, RegionDef, RomSetDef};

/// Régions ignorées : (aucune)
pub const RS2_1944: RomSetDef = RomSetDef { name: "1944", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nffe.03", offset: 0x0, size: 0x80000, crc: 0x7544B926 },
            Op::Load { kind: Load::Word16Swap, file: "nffe.04", offset: 0x80000, size: 0x80000, crc: 0xDBA1C66E },
            Op::Load { kind: Load::Word16Swap, file: "nffe.05", offset: 0x100000, size: 0x80000, crc: 0xD78D31D3 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "nff.13m", offset: 0x0, size: 0x400000, crc: 0xC9FCA741 },
            Op::Load { kind: Load::Word64, file: "nff.15m", offset: 0x2, size: 0x400000, crc: 0xF809D898 },
            Op::Load { kind: Load::Word64, file: "nff.17m", offset: 0x4, size: 0x400000, crc: 0x15BA4507 },
            Op::Load { kind: Load::Word64, file: "nff.19m", offset: 0x6, size: 0x400000, crc: 0x3DD41B8C },
            Op::Load { kind: Load::Word64, file: "nff.14m", offset: 0x1000000, size: 0x100000, crc: 0x3FE3A54B },
            Op::Load { kind: Load::Word64, file: "nff.16m", offset: 0x1000002, size: 0x100000, crc: 0x565CD231 },
            Op::Load { kind: Load::Word64, file: "nff.18m", offset: 0x1000004, size: 0x100000, crc: 0x63CA5988 },
            Op::Load { kind: Load::Word64, file: "nff.20m", offset: 0x1000006, size: 0x100000, crc: 0x21EB8F3B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nff.01", offset: 0x0, size: 0x8000, crc: 0xD2E44318 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nff.11m", offset: 0x0, size: 0x400000, crc: 0x243E4E05 },
            Op::Load { kind: Load::Word16Swap, file: "nff.12m", offset: 0x400000, size: 0x400000, crc: 0x4FCF1600 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "1944.key", offset: 0x0, size: 0x14, crc: 0x5F22140E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_1944U: RomSetDef = RomSetDef { name: "1944u", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nffu.03", offset: 0x0, size: 0x80000, crc: 0x9693CF8F },
            Op::Load { kind: Load::Word16Swap, file: "nff.04", offset: 0x80000, size: 0x80000, crc: 0xDBA1C66E },
            Op::Load { kind: Load::Word16Swap, file: "nffu.05", offset: 0x100000, size: 0x80000, crc: 0xEA813EB7 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "nff.13m", offset: 0x0, size: 0x400000, crc: 0xC9FCA741 },
            Op::Load { kind: Load::Word64, file: "nff.15m", offset: 0x2, size: 0x400000, crc: 0xF809D898 },
            Op::Load { kind: Load::Word64, file: "nff.17m", offset: 0x4, size: 0x400000, crc: 0x15BA4507 },
            Op::Load { kind: Load::Word64, file: "nff.19m", offset: 0x6, size: 0x400000, crc: 0x3DD41B8C },
            Op::Load { kind: Load::Word64, file: "nff.14m", offset: 0x1000000, size: 0x100000, crc: 0x3FE3A54B },
            Op::Load { kind: Load::Word64, file: "nff.16m", offset: 0x1000002, size: 0x100000, crc: 0x565CD231 },
            Op::Load { kind: Load::Word64, file: "nff.18m", offset: 0x1000004, size: 0x100000, crc: 0x63CA5988 },
            Op::Load { kind: Load::Word64, file: "nff.20m", offset: 0x1000006, size: 0x100000, crc: 0x21EB8F3B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nff.01", offset: 0x0, size: 0x8000, crc: 0xD2E44318 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nff.11m", offset: 0x0, size: 0x400000, crc: 0x243E4E05 },
            Op::Load { kind: Load::Word16Swap, file: "nff.12m", offset: 0x400000, size: 0x400000, crc: 0x4FCF1600 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "1944u.key", offset: 0x0, size: 0x14, crc: 0x61734F5B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_1944J: RomSetDef = RomSetDef { name: "1944j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nffj.03", offset: 0x0, size: 0x80000, crc: 0x247521EF },
            Op::Load { kind: Load::Word16Swap, file: "nffj.04", offset: 0x80000, size: 0x80000, crc: 0xDBA1C66E },
            Op::Load { kind: Load::Word16Swap, file: "nffj.05", offset: 0x100000, size: 0x80000, crc: 0x7F20C2EF },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "nff.13m", offset: 0x0, size: 0x400000, crc: 0xC9FCA741 },
            Op::Load { kind: Load::Word64, file: "nff.15m", offset: 0x2, size: 0x400000, crc: 0xF809D898 },
            Op::Load { kind: Load::Word64, file: "nff.17m", offset: 0x4, size: 0x400000, crc: 0x15BA4507 },
            Op::Load { kind: Load::Word64, file: "nff.19m", offset: 0x6, size: 0x400000, crc: 0x3DD41B8C },
            Op::Load { kind: Load::Word64, file: "nff.14m", offset: 0x1000000, size: 0x100000, crc: 0x3FE3A54B },
            Op::Load { kind: Load::Word64, file: "nff.16m", offset: 0x1000002, size: 0x100000, crc: 0x565CD231 },
            Op::Load { kind: Load::Word64, file: "nff.18m", offset: 0x1000004, size: 0x100000, crc: 0x63CA5988 },
            Op::Load { kind: Load::Word64, file: "nff.20m", offset: 0x1000006, size: 0x100000, crc: 0x21EB8F3B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nff.01", offset: 0x0, size: 0x8000, crc: 0xD2E44318 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nff.11m", offset: 0x0, size: 0x400000, crc: 0x243E4E05 },
            Op::Load { kind: Load::Word16Swap, file: "nff.12m", offset: 0x400000, size: 0x400000, crc: 0x4FCF1600 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "1944j.key", offset: 0x0, size: 0x14, crc: 0x210202AA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XX: RomSetDef = RomSetDef { name: "19xx", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xe.03b", offset: 0x0, size: 0x80000, crc: 0x9BF9D9B1 },
            Op::Load { kind: Load::Word16Swap, file: "19xe.04b", offset: 0x80000, size: 0x80000, crc: 0x9AA46476 },
            Op::Load { kind: Load::Word16Swap, file: "19xe.05b", offset: 0x100000, size: 0x80000, crc: 0xA04A2C5E },
            Op::Load { kind: Load::Word16Swap, file: "19xe.06b", offset: 0x180000, size: 0x80000, crc: 0xF0A81C33 },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xx.key", offset: 0x0, size: 0x14, crc: 0x6F5D6406 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXU: RomSetDef = RomSetDef { name: "19xxu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xu.03", offset: 0x0, size: 0x80000, crc: 0x05955268 },
            Op::Load { kind: Load::Word16Swap, file: "19xu.04", offset: 0x80000, size: 0x80000, crc: 0x3111AB7F },
            Op::Load { kind: Load::Word16Swap, file: "19xu.05", offset: 0x100000, size: 0x80000, crc: 0x38DF4A63 },
            Op::Load { kind: Load::Word16Swap, file: "19xu.06", offset: 0x180000, size: 0x80000, crc: 0x5C7E60D3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxu.key", offset: 0x0, size: 0x14, crc: 0x77E67BA1 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXJ: RomSetDef = RomSetDef { name: "19xxj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xj-03b.6a", offset: 0x0, size: 0x80000, crc: 0xBCAD93DD },
            Op::Load { kind: Load::Word16Swap, file: "19xj-04b.7a", offset: 0x80000, size: 0x80000, crc: 0x931882A1 },
            Op::Load { kind: Load::Word16Swap, file: "19xj-05b.8a", offset: 0x100000, size: 0x80000, crc: 0xE7EEDDC4 },
            Op::Load { kind: Load::Word16Swap, file: "19xj-06b.9a", offset: 0x180000, size: 0x80000, crc: 0xF27CD6B8 },
            Op::Load { kind: Load::Word16Swap, file: "19xj-07.6d", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x-69.4j", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x-59.4d", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x-79.4m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x-89.4p", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x-73.8j", offset: 0x800000, size: 0x80000, crc: 0x8E81F595 },
            Op::Load { kind: Load::Word64, file: "19x-74.9j", offset: 0xa00000, size: 0x80000, crc: 0x6D7AD22E },
            Op::Load { kind: Load::Word64, file: "19x-75.10j", offset: 0xc00000, size: 0x80000, crc: 0xCB1A1B6A },
            Op::Load { kind: Load::Word64, file: "19x-76.11j", offset: 0xe00000, size: 0x80000, crc: 0x26FC2B08 },
            Op::Load { kind: Load::Word64, file: "19x-63.8d", offset: 0x800002, size: 0x80000, crc: 0x6F8B045E },
            Op::Load { kind: Load::Word64, file: "19x-64.9d", offset: 0xa00002, size: 0x80000, crc: 0xCCD5725A },
            Op::Load { kind: Load::Word64, file: "19x-65.10d", offset: 0xc00002, size: 0x80000, crc: 0x6CF6DB35 },
            Op::Load { kind: Load::Word64, file: "19x-66.11d", offset: 0xe00002, size: 0x80000, crc: 0x16115DD3 },
            Op::Load { kind: Load::Word64, file: "19x-83.8m", offset: 0x800004, size: 0x80000, crc: 0xC11F88C1 },
            Op::Load { kind: Load::Word64, file: "19x-84.9m", offset: 0xa00004, size: 0x80000, crc: 0x68CC9CD8 },
            Op::Load { kind: Load::Word64, file: "19x-85.10m", offset: 0xc00004, size: 0x80000, crc: 0xF213666B },
            Op::Load { kind: Load::Word64, file: "19x-86.11m", offset: 0xe00004, size: 0x80000, crc: 0x574E0473 },
            Op::Load { kind: Load::Word64, file: "19x-93.8p", offset: 0x800006, size: 0x80000, crc: 0x9FAD3C55 },
            Op::Load { kind: Load::Word64, file: "19x-94.9p", offset: 0xa00006, size: 0x80000, crc: 0xE10E252C },
            Op::Load { kind: Load::Word64, file: "19x-95.10p", offset: 0xc00006, size: 0x80000, crc: 0x2B86FA67 },
            Op::Load { kind: Load::Word64, file: "19x-96.11p", offset: 0xe00006, size: 0x80000, crc: 0xAE6EB692 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x-01.1a", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x-51.6a", offset: 0x0, size: 0x80000, crc: 0xE9CD7780 },
            Op::Load { kind: Load::Word16Swap, file: "19x-52.7a", offset: 0x80000, size: 0x80000, crc: 0xB27B91A8 },
            Op::Load { kind: Load::Word16Swap, file: "19x-53.8a", offset: 0x100000, size: 0x80000, crc: 0x2E563EE2 },
            Op::Load { kind: Load::Word16Swap, file: "19x-54.9a", offset: 0x180000, size: 0x80000, crc: 0xF47C1F24 },
            Op::Load { kind: Load::Word16Swap, file: "19x-55.10a", offset: 0x200000, size: 0x80000, crc: 0x0B1AF6E0 },
            Op::Load { kind: Load::Word16Swap, file: "19x-56.11a", offset: 0x280000, size: 0x80000, crc: 0xDFA8819F },
            Op::Load { kind: Load::Word16Swap, file: "19x-57.12a", offset: 0x300000, size: 0x80000, crc: 0x229BA777 },
            Op::Load { kind: Load::Word16Swap, file: "19x-58.13a", offset: 0x380000, size: 0x80000, crc: 0xC7DCEBA4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxj.key", offset: 0x0, size: 0x14, crc: 0x9AAFA71A },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXJR1: RomSetDef = RomSetDef { name: "19xxjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xj.03a", offset: 0x0, size: 0x80000, crc: 0xED08BDD1 },
            Op::Load { kind: Load::Word16Swap, file: "19xj.04a", offset: 0x80000, size: 0x80000, crc: 0xFB8E3F29 },
            Op::Load { kind: Load::Word16Swap, file: "19xj.05a", offset: 0x100000, size: 0x80000, crc: 0xAA508AC4 },
            Op::Load { kind: Load::Word16Swap, file: "19xj.06a", offset: 0x180000, size: 0x80000, crc: 0xFF2D785B },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxj.key", offset: 0x0, size: 0x14, crc: 0x9AAFA71A },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXJR2: RomSetDef = RomSetDef { name: "19xxjr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xj.03", offset: 0x0, size: 0x80000, crc: 0x26A381ED },
            Op::Load { kind: Load::Word16Swap, file: "19xj.04", offset: 0x80000, size: 0x80000, crc: 0x30100CCA },
            Op::Load { kind: Load::Word16Swap, file: "19xj.05", offset: 0x100000, size: 0x80000, crc: 0xDE67E938 },
            Op::Load { kind: Load::Word16Swap, file: "19xj.06", offset: 0x180000, size: 0x80000, crc: 0x39F9A409 },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxj.key", offset: 0x0, size: 0x14, crc: 0x9AAFA71A },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXA: RomSetDef = RomSetDef { name: "19xxa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "09xa.03b", offset: 0x0, size: 0x80000, crc: 0x2E994897 },
            Op::Load { kind: Load::Word16Swap, file: "09xa.04b", offset: 0x80000, size: 0x80000, crc: 0x6364D001 },
            Op::Load { kind: Load::Word16Swap, file: "09xa.05b", offset: 0x100000, size: 0x80000, crc: 0x00C1949B },
            Op::Load { kind: Load::Word16Swap, file: "09xa.06b", offset: 0x180000, size: 0x80000, crc: 0x363C1F6E },
            Op::Load { kind: Load::Word16Swap, file: "19xa.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxa.key", offset: 0x0, size: 0x14, crc: 0x2CD32EB9 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXAR1: RomSetDef = RomSetDef { name: "19xxar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xa.03", offset: 0x0, size: 0x80000, crc: 0x0C20FD50 },
            Op::Load { kind: Load::Word16Swap, file: "19xa.04", offset: 0x80000, size: 0x80000, crc: 0x1FC37508 },
            Op::Load { kind: Load::Word16Swap, file: "19xa.05", offset: 0x100000, size: 0x80000, crc: 0x6C9CC4ED },
            Op::Load { kind: Load::Word16Swap, file: "19xa.06", offset: 0x180000, size: 0x80000, crc: 0xCA5B9F76 },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxa.key", offset: 0x0, size: 0x14, crc: 0x2CD32EB9 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXH: RomSetDef = RomSetDef { name: "19xxh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xh.03a", offset: 0x0, size: 0x80000, crc: 0x357BE2AC },
            Op::Load { kind: Load::Word16Swap, file: "19xh.04a", offset: 0x80000, size: 0x80000, crc: 0xBB13EA3B },
            Op::Load { kind: Load::Word16Swap, file: "19xh.05a", offset: 0x100000, size: 0x80000, crc: 0xCBD76601 },
            Op::Load { kind: Load::Word16Swap, file: "19xh.06a", offset: 0x180000, size: 0x80000, crc: 0xB362DE8B },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxh.key", offset: 0x0, size: 0x14, crc: 0x215CF208 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_19XXB: RomSetDef = RomSetDef { name: "19xxb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xb.03a", offset: 0x0, size: 0x80000, crc: 0x341BDF4A },
            Op::Load { kind: Load::Word16Swap, file: "19xb.04a", offset: 0x80000, size: 0x80000, crc: 0xDFF8069E },
            Op::Load { kind: Load::Word16Swap, file: "19xb.05a", offset: 0x100000, size: 0x80000, crc: 0xA47A92A8 },
            Op::Load { kind: Load::Word16Swap, file: "19xb.06a", offset: 0x180000, size: 0x80000, crc: 0xC52DF10D },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19xxb.key", offset: 0x0, size: 0x14, crc: 0x4200E334 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWAR: RomSetDef = RomSetDef { name: "armwar", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwge.03c", offset: 0x0, size: 0x80000, crc: 0x31F74931 },
            Op::Load { kind: Load::Word16Swap, file: "pwge.04c", offset: 0x80000, size: 0x80000, crc: 0x16F34F5F },
            Op::Load { kind: Load::Word16Swap, file: "pwge.05b", offset: 0x100000, size: 0x80000, crc: 0x4403ED08 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09a", offset: 0x300000, size: 0x80000, crc: 0x4C26BAEE },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwar.key", offset: 0x0, size: 0x14, crc: 0xFE979382 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWARB: RomSetDef = RomSetDef { name: "armwarb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwgb.03b", offset: 0x0, size: 0x80000, crc: 0xD0161E14 },
            Op::Load { kind: Load::Word16Swap, file: "pwgb.04b", offset: 0x80000, size: 0x80000, crc: 0x7FB39864 },
            Op::Load { kind: Load::Word16Swap, file: "pwgb.05b", offset: 0x100000, size: 0x80000, crc: 0x52A1C12C },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09a", offset: 0x300000, size: 0x80000, crc: 0x4C26BAEE },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwarb.key", offset: 0x0, size: 0x14, crc: 0xEEB97BDB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWARR1: RomSetDef = RomSetDef { name: "armwarr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwge.03b", offset: 0x0, size: 0x80000, crc: 0xE822E3E9 },
            Op::Load { kind: Load::Word16Swap, file: "pwge.04b", offset: 0x80000, size: 0x80000, crc: 0x4F89DE39 },
            Op::Load { kind: Load::Word16Swap, file: "pwge.05a", offset: 0x100000, size: 0x80000, crc: 0x83DF24E5 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09", offset: 0x300000, size: 0x80000, crc: 0xDDC85CA6 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwar.key", offset: 0x0, size: 0x14, crc: 0xFE979382 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWARU: RomSetDef = RomSetDef { name: "armwaru", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwgu.03b", offset: 0x0, size: 0x80000, crc: 0x8B95497A },
            Op::Load { kind: Load::Word16Swap, file: "pwgu.04b", offset: 0x80000, size: 0x80000, crc: 0x29EB5661 },
            Op::Load { kind: Load::Word16Swap, file: "pwgu.05b", offset: 0x100000, size: 0x80000, crc: 0xA54E9E44 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09a", offset: 0x300000, size: 0x80000, crc: 0x4C26BAEE },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwaru.key", offset: 0x0, size: 0x14, crc: 0xFB9AADA5 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWARU1: RomSetDef = RomSetDef { name: "armwaru1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwgu.03a", offset: 0x0, size: 0x80000, crc: 0x73D397B1 },
            Op::Load { kind: Load::Word16Swap, file: "pwgu.04a", offset: 0x80000, size: 0x80000, crc: 0x1F1DE215 },
            Op::Load { kind: Load::Word16Swap, file: "pwgu.05a", offset: 0x100000, size: 0x80000, crc: 0x835FBE73 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09", offset: 0x300000, size: 0x80000, crc: 0xDDC85CA6 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwaru.key", offset: 0x0, size: 0x14, crc: 0xFB9AADA5 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PGEAR: RomSetDef = RomSetDef { name: "pgear", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwgj.03a", offset: 0x0, size: 0x80000, crc: 0xC79C0C02 },
            Op::Load { kind: Load::Word16Swap, file: "pwgj.04a", offset: 0x80000, size: 0x80000, crc: 0x167C6ED8 },
            Op::Load { kind: Load::Word16Swap, file: "pwgj.05a", offset: 0x100000, size: 0x80000, crc: 0xA63FB400 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09a", offset: 0x300000, size: 0x80000, crc: 0x4C26BAEE },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pgear.key", offset: 0x0, size: 0x14, crc: 0xC576D6FD },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PGEARR1: RomSetDef = RomSetDef { name: "pgearr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwgj.03", offset: 0x0, size: 0x80000, crc: 0xF264E74B },
            Op::Load { kind: Load::Word16Swap, file: "pwgj.04", offset: 0x80000, size: 0x80000, crc: 0x23A84983 },
            Op::Load { kind: Load::Word16Swap, file: "pwgj.05", offset: 0x100000, size: 0x80000, crc: 0xBEF58C62 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09", offset: 0x300000, size: 0x80000, crc: 0xDDC85CA6 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pgear.key", offset: 0x0, size: 0x14, crc: 0xC576D6FD },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWARA: RomSetDef = RomSetDef { name: "armwara", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwga.03b", offset: 0x0, size: 0x80000, crc: 0x347743E1 },
            Op::Load { kind: Load::Word16Swap, file: "pwga.04b", offset: 0x80000, size: 0x80000, crc: 0x42DBFB2E },
            Op::Load { kind: Load::Word16Swap, file: "pwga.05b", offset: 0x100000, size: 0x80000, crc: 0x835FBE73 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09a", offset: 0x300000, size: 0x80000, crc: 0x4C26BAEE },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwara.key", offset: 0x0, size: 0x14, crc: 0x525439C0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWARAR1: RomSetDef = RomSetDef { name: "armwarar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwga.03a", offset: 0x0, size: 0x80000, crc: 0x8D474AB1 },
            Op::Load { kind: Load::Word16Swap, file: "pwga.04a", offset: 0x80000, size: 0x80000, crc: 0x81B5AEC7 },
            Op::Load { kind: Load::Word16Swap, file: "pwga.05a", offset: 0x100000, size: 0x80000, crc: 0x2618E819 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09", offset: 0x300000, size: 0x80000, crc: 0xDDC85CA6 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "armwara.key", offset: 0x0, size: 0x14, crc: 0x525439C0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_AVSP: RomSetDef = RomSetDef { name: "avsp", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avpe.03d", offset: 0x0, size: 0x80000, crc: 0x774334A9 },
            Op::Load { kind: Load::Word16Swap, file: "avpe.04d", offset: 0x80000, size: 0x80000, crc: 0x7FA83769 },
            Op::Load { kind: Load::Word16Swap, file: "avp.05d", offset: 0x100000, size: 0x80000, crc: 0xFBFB5D7A },
            Op::Load { kind: Load::Word16Swap, file: "avp.06", offset: 0x180000, size: 0x80000, crc: 0x190B817F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "avp.13m", offset: 0x0, size: 0x200000, crc: 0x8F8B5AE4 },
            Op::Load { kind: Load::Word64, file: "avp.15m", offset: 0x2, size: 0x200000, crc: 0xB00280DF },
            Op::Load { kind: Load::Word64, file: "avp.17m", offset: 0x4, size: 0x200000, crc: 0x94403195 },
            Op::Load { kind: Load::Word64, file: "avp.19m", offset: 0x6, size: 0x200000, crc: 0xE1981245 },
            Op::Load { kind: Load::Word64, file: "avp.14m", offset: 0x800000, size: 0x100000, crc: 0x39933B1E },
            Op::Load { kind: Load::Word64, file: "avp.16m", offset: 0x800002, size: 0x100000, crc: 0x85412860 },
            Op::Load { kind: Load::Word64, file: "avp.18m", offset: 0x800004, size: 0x100000, crc: 0x2E2BEB06 },
            Op::Load { kind: Load::Word64, file: "avp.20m", offset: 0x800006, size: 0x100000, crc: 0xE4798D1A },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avp.01", offset: 0x0, size: 0x8000, crc: 0x2D3B4220 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avp.11m", offset: 0x0, size: 0x200000, crc: 0x83499817 },
            Op::Load { kind: Load::Word16Swap, file: "avp.12m", offset: 0x200000, size: 0x200000, crc: 0xF4110D49 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avsp.key", offset: 0x0, size: 0x14, crc: 0xE69FA35B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_AVSPU: RomSetDef = RomSetDef { name: "avspu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avpu.03d", offset: 0x0, size: 0x80000, crc: 0x42757950 },
            Op::Load { kind: Load::Word16Swap, file: "avpu.04d", offset: 0x80000, size: 0x80000, crc: 0x5ABCDEE6 },
            Op::Load { kind: Load::Word16Swap, file: "avp.05d", offset: 0x100000, size: 0x80000, crc: 0xFBFB5D7A },
            Op::Load { kind: Load::Word16Swap, file: "avp.06", offset: 0x180000, size: 0x80000, crc: 0x190B817F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "avp.13m", offset: 0x0, size: 0x200000, crc: 0x8F8B5AE4 },
            Op::Load { kind: Load::Word64, file: "avp.15m", offset: 0x2, size: 0x200000, crc: 0xB00280DF },
            Op::Load { kind: Load::Word64, file: "avp.17m", offset: 0x4, size: 0x200000, crc: 0x94403195 },
            Op::Load { kind: Load::Word64, file: "avp.19m", offset: 0x6, size: 0x200000, crc: 0xE1981245 },
            Op::Load { kind: Load::Word64, file: "avp.14m", offset: 0x800000, size: 0x100000, crc: 0x39933B1E },
            Op::Load { kind: Load::Word64, file: "avp.16m", offset: 0x800002, size: 0x100000, crc: 0x85412860 },
            Op::Load { kind: Load::Word64, file: "avp.18m", offset: 0x800004, size: 0x100000, crc: 0x2E2BEB06 },
            Op::Load { kind: Load::Word64, file: "avp.20m", offset: 0x800006, size: 0x100000, crc: 0xE4798D1A },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avp.01", offset: 0x0, size: 0x8000, crc: 0x2D3B4220 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avp.11m", offset: 0x0, size: 0x200000, crc: 0x83499817 },
            Op::Load { kind: Load::Word16Swap, file: "avp.12m", offset: 0x200000, size: 0x200000, crc: 0xF4110D49 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avspu.key", offset: 0x0, size: 0x14, crc: 0x4E68E346 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_AVSPJ: RomSetDef = RomSetDef { name: "avspj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avpj.03d", offset: 0x0, size: 0x80000, crc: 0x49799119 },
            Op::Load { kind: Load::Word16Swap, file: "avpj.04d", offset: 0x80000, size: 0x80000, crc: 0x8CD2BBA8 },
            Op::Load { kind: Load::Word16Swap, file: "avp.05d", offset: 0x100000, size: 0x80000, crc: 0xFBFB5D7A },
            Op::Load { kind: Load::Word16Swap, file: "avp.06", offset: 0x180000, size: 0x80000, crc: 0x190B817F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "avp.13m", offset: 0x0, size: 0x200000, crc: 0x8F8B5AE4 },
            Op::Load { kind: Load::Word64, file: "avp.15m", offset: 0x2, size: 0x200000, crc: 0xB00280DF },
            Op::Load { kind: Load::Word64, file: "avp.17m", offset: 0x4, size: 0x200000, crc: 0x94403195 },
            Op::Load { kind: Load::Word64, file: "avp.19m", offset: 0x6, size: 0x200000, crc: 0xE1981245 },
            Op::Load { kind: Load::Word64, file: "avp.14m", offset: 0x800000, size: 0x100000, crc: 0x39933B1E },
            Op::Load { kind: Load::Word64, file: "avp.16m", offset: 0x800002, size: 0x100000, crc: 0x85412860 },
            Op::Load { kind: Load::Word64, file: "avp.18m", offset: 0x800004, size: 0x100000, crc: 0x2E2BEB06 },
            Op::Load { kind: Load::Word64, file: "avp.20m", offset: 0x800006, size: 0x100000, crc: 0xE4798D1A },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avp.01", offset: 0x0, size: 0x8000, crc: 0x2D3B4220 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avp.11m", offset: 0x0, size: 0x200000, crc: 0x83499817 },
            Op::Load { kind: Load::Word16Swap, file: "avp.12m", offset: 0x200000, size: 0x200000, crc: 0xF4110D49 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avspj.key", offset: 0x0, size: 0x14, crc: 0x3D5CCC08 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_AVSPA: RomSetDef = RomSetDef { name: "avspa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avpa.03d", offset: 0x0, size: 0x80000, crc: 0x6C1C1858 },
            Op::Load { kind: Load::Word16Swap, file: "avpa.04d", offset: 0x80000, size: 0x80000, crc: 0x94F50B0C },
            Op::Load { kind: Load::Word16Swap, file: "avp.05d", offset: 0x100000, size: 0x80000, crc: 0xFBFB5D7A },
            Op::Load { kind: Load::Word16Swap, file: "avp.06", offset: 0x180000, size: 0x80000, crc: 0x190B817F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "avp.13m", offset: 0x0, size: 0x200000, crc: 0x8F8B5AE4 },
            Op::Load { kind: Load::Word64, file: "avp.15m", offset: 0x2, size: 0x200000, crc: 0xB00280DF },
            Op::Load { kind: Load::Word64, file: "avp.17m", offset: 0x4, size: 0x200000, crc: 0x94403195 },
            Op::Load { kind: Load::Word64, file: "avp.19m", offset: 0x6, size: 0x200000, crc: 0xE1981245 },
            Op::Load { kind: Load::Word64, file: "avp.14m", offset: 0x800000, size: 0x100000, crc: 0x39933B1E },
            Op::Load { kind: Load::Word64, file: "avp.16m", offset: 0x800002, size: 0x100000, crc: 0x85412860 },
            Op::Load { kind: Load::Word64, file: "avp.18m", offset: 0x800004, size: 0x100000, crc: 0x2E2BEB06 },
            Op::Load { kind: Load::Word64, file: "avp.20m", offset: 0x800006, size: 0x100000, crc: 0xE4798D1A },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avp.01", offset: 0x0, size: 0x8000, crc: 0x2D3B4220 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avp.11m", offset: 0x0, size: 0x200000, crc: 0x83499817 },
            Op::Load { kind: Load::Word16Swap, file: "avp.12m", offset: 0x200000, size: 0x200000, crc: 0xF4110D49 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avspa.key", offset: 0x0, size: 0x14, crc: 0x728EFC00 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_AVSPH: RomSetDef = RomSetDef { name: "avsph", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avph.03d", offset: 0x0, size: 0x80000, crc: 0x3E440447 },
            Op::Load { kind: Load::Word16Swap, file: "avph.04d", offset: 0x80000, size: 0x80000, crc: 0xAF6FC82F },
            Op::Load { kind: Load::Word16Swap, file: "avp.05d", offset: 0x100000, size: 0x80000, crc: 0xFBFB5D7A },
            Op::Load { kind: Load::Word16Swap, file: "avp.06", offset: 0x180000, size: 0x80000, crc: 0x190B817F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "avp.13m", offset: 0x0, size: 0x200000, crc: 0x8F8B5AE4 },
            Op::Load { kind: Load::Word64, file: "avp.15m", offset: 0x2, size: 0x200000, crc: 0xB00280DF },
            Op::Load { kind: Load::Word64, file: "avp.17m", offset: 0x4, size: 0x200000, crc: 0x94403195 },
            Op::Load { kind: Load::Word64, file: "avp.19m", offset: 0x6, size: 0x200000, crc: 0xE1981245 },
            Op::Load { kind: Load::Word64, file: "avp.14m", offset: 0x800000, size: 0x100000, crc: 0x39933B1E },
            Op::Load { kind: Load::Word64, file: "avp.16m", offset: 0x800002, size: 0x100000, crc: 0x85412860 },
            Op::Load { kind: Load::Word64, file: "avp.18m", offset: 0x800004, size: 0x100000, crc: 0x2E2BEB06 },
            Op::Load { kind: Load::Word64, file: "avp.20m", offset: 0x800006, size: 0x100000, crc: 0xE4798D1A },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avp.01", offset: 0x0, size: 0x8000, crc: 0x2D3B4220 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avp.11m", offset: 0x0, size: 0x200000, crc: 0x83499817 },
            Op::Load { kind: Load::Word16Swap, file: "avp.12m", offset: 0x200000, size: 0x200000, crc: 0xF4110D49 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avsph.key", offset: 0x0, size: 0x14, crc: 0xCAE7B680 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_BATCIR: RomSetDef = RomSetDef { name: "batcir", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btce.03", offset: 0x0, size: 0x80000, crc: 0xBC60484B },
            Op::Load { kind: Load::Word16Swap, file: "btce.04", offset: 0x80000, size: 0x80000, crc: 0x457D55F6 },
            Op::Load { kind: Load::Word16Swap, file: "btce.05", offset: 0x100000, size: 0x80000, crc: 0xE86560D7 },
            Op::Load { kind: Load::Word16Swap, file: "btce.06", offset: 0x180000, size: 0x80000, crc: 0xF778E61B },
            Op::Load { kind: Load::Word16Swap, file: "btc.07", offset: 0x200000, size: 0x80000, crc: 0x7322D5DB },
            Op::Load { kind: Load::Word16Swap, file: "btc.08", offset: 0x280000, size: 0x80000, crc: 0x6AAC85AB },
            Op::Load { kind: Load::Word16Swap, file: "btc.09", offset: 0x300000, size: 0x80000, crc: 0x1203DB08 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "btc.13m", offset: 0x0, size: 0x400000, crc: 0xDC705BAD },
            Op::Load { kind: Load::Word64, file: "btc.15m", offset: 0x2, size: 0x400000, crc: 0xE5779A3C },
            Op::Load { kind: Load::Word64, file: "btc.17m", offset: 0x4, size: 0x400000, crc: 0xB33F4112 },
            Op::Load { kind: Load::Word64, file: "btc.19m", offset: 0x6, size: 0x400000, crc: 0xA6FCDB7E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "btc.01", offset: 0x0, size: 0x8000, crc: 0x1E194310 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "btc.02", offset: 0x28000, size: 0x20000, crc: 0x01AEB8E6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btc.11m", offset: 0x0, size: 0x200000, crc: 0xC27F2229 },
            Op::Load { kind: Load::Word16Swap, file: "btc.12m", offset: 0x200000, size: 0x200000, crc: 0x418A2E33 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "batcir.key", offset: 0x0, size: 0x14, crc: 0xE316AE67 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_BATCIRJ: RomSetDef = RomSetDef { name: "batcirj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btcj.03", offset: 0x0, size: 0x80000, crc: 0x6B7E168D },
            Op::Load { kind: Load::Word16Swap, file: "btcj.04", offset: 0x80000, size: 0x80000, crc: 0x46BA3467 },
            Op::Load { kind: Load::Word16Swap, file: "btcj.05", offset: 0x100000, size: 0x80000, crc: 0x0E23A859 },
            Op::Load { kind: Load::Word16Swap, file: "btcj.06", offset: 0x180000, size: 0x80000, crc: 0xA853B59C },
            Op::Load { kind: Load::Word16Swap, file: "btc.07", offset: 0x200000, size: 0x80000, crc: 0x7322D5DB },
            Op::Load { kind: Load::Word16Swap, file: "btc.08", offset: 0x280000, size: 0x80000, crc: 0x6AAC85AB },
            Op::Load { kind: Load::Word16Swap, file: "btc.09", offset: 0x300000, size: 0x80000, crc: 0x1203DB08 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "btc.13m", offset: 0x0, size: 0x400000, crc: 0xDC705BAD },
            Op::Load { kind: Load::Word64, file: "btc.15m", offset: 0x2, size: 0x400000, crc: 0xE5779A3C },
            Op::Load { kind: Load::Word64, file: "btc.17m", offset: 0x4, size: 0x400000, crc: 0xB33F4112 },
            Op::Load { kind: Load::Word64, file: "btc.19m", offset: 0x6, size: 0x400000, crc: 0xA6FCDB7E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "btc.01", offset: 0x0, size: 0x8000, crc: 0x1E194310 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "btc.02", offset: 0x28000, size: 0x20000, crc: 0x01AEB8E6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btc.11m", offset: 0x0, size: 0x200000, crc: 0xC27F2229 },
            Op::Load { kind: Load::Word16Swap, file: "btc.12m", offset: 0x200000, size: 0x200000, crc: 0x418A2E33 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "batcirj.key", offset: 0x0, size: 0x14, crc: 0x9F9FB965 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_BATCIRA: RomSetDef = RomSetDef { name: "batcira", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btca.03", offset: 0x0, size: 0x80000, crc: 0x1AD20D87 },
            Op::Load { kind: Load::Word16Swap, file: "btca.04", offset: 0x80000, size: 0x80000, crc: 0x2B3F4DBE },
            Op::Load { kind: Load::Word16Swap, file: "btca.05", offset: 0x100000, size: 0x80000, crc: 0x8238A3D9 },
            Op::Load { kind: Load::Word16Swap, file: "btca.06", offset: 0x180000, size: 0x80000, crc: 0x446C7C02 },
            Op::Load { kind: Load::Word16Swap, file: "btc.07", offset: 0x200000, size: 0x80000, crc: 0x7322D5DB },
            Op::Load { kind: Load::Word16Swap, file: "btc.08", offset: 0x280000, size: 0x80000, crc: 0x6AAC85AB },
            Op::Load { kind: Load::Word16Swap, file: "btc.09", offset: 0x300000, size: 0x80000, crc: 0x1203DB08 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "btc.13m", offset: 0x0, size: 0x400000, crc: 0xDC705BAD },
            Op::Load { kind: Load::Word64, file: "btc.15m", offset: 0x2, size: 0x400000, crc: 0xE5779A3C },
            Op::Load { kind: Load::Word64, file: "btc.17m", offset: 0x4, size: 0x400000, crc: 0xB33F4112 },
            Op::Load { kind: Load::Word64, file: "btc.19m", offset: 0x6, size: 0x400000, crc: 0xA6FCDB7E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "btc.01", offset: 0x0, size: 0x8000, crc: 0x1E194310 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "btc.02", offset: 0x28000, size: 0x20000, crc: 0x01AEB8E6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btc.11m", offset: 0x0, size: 0x200000, crc: 0xC27F2229 },
            Op::Load { kind: Load::Word16Swap, file: "btc.12m", offset: 0x200000, size: 0x200000, crc: 0x418A2E33 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "batcira.key", offset: 0x0, size: 0x14, crc: 0x384500F3 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CHOKO: RomSetDef = RomSetDef { name: "choko", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0xff, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tkoj.03", offset: 0x0, size: 0x80000, crc: 0x11F5452F },
            Op::Load { kind: Load::Word16Swap, file: "tkoj.04", offset: 0x80000, size: 0x80000, crc: 0x68655378 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "tkoj1_d.simm1", offset: 0x0, size: 0x200000, crc: 0x6933377D },
            Op::Load { kind: Load::Byte64, file: "tkoj1_c.simm1", offset: 0x1, size: 0x200000, crc: 0x7F668950 },
            Op::Load { kind: Load::Byte64, file: "tkoj1_b.simm1", offset: 0x2, size: 0x200000, crc: 0xCFB68CA9 },
            Op::Load { kind: Load::Byte64, file: "tkoj1_a.simm1", offset: 0x3, size: 0x200000, crc: 0x437E21C5 },
            Op::Load { kind: Load::Byte64, file: "tkoj3_d.simm3", offset: 0x4, size: 0x200000, crc: 0xA9E32B57 },
            Op::Load { kind: Load::Byte64, file: "tkoj3_c.simm3", offset: 0x5, size: 0x200000, crc: 0xB7AB9338 },
            Op::Load { kind: Load::Byte64, file: "tkoj3_b.simm3", offset: 0x6, size: 0x200000, crc: 0x4D3F919A },
            Op::Load { kind: Load::Byte64, file: "tkoj3_a.simm3", offset: 0x7, size: 0x200000, crc: 0xCFEF17AB },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tko.01", offset: 0x0, size: 0x8000, crc: 0x6EDA50C2 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "tkoj5_a.simm5", offset: 0x0, size: 0x200000, crc: 0xAB45D509 },
            Op::Load { kind: Load::Byte16, file: "tkoj5_b.simm5", offset: 0x1, size: 0x200000, crc: 0xFA905C3D },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "choko.key", offset: 0x0, size: 0x14, crc: 0x08505E8B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CHOKOP: RomSetDef = RomSetDef { name: "chokop", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tkojp_03.bin", offset: 0x0, size: 0x80000, crc: 0x4D857F39 },
            Op::Load { kind: Load::Word16Swap, file: "tkojp_04.bin", offset: 0x80000, size: 0x80000, crc: 0x137AFB29 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "tko_14.bin", offset: 0x800000, size: 0x200000, crc: 0x336E8AA2 },
            Op::Load { kind: Load::Word64, file: "tko_16.bin", offset: 0x800002, size: 0x200000, crc: 0xE8429B54 },
            Op::Load { kind: Load::Word64, file: "tko_18.bin", offset: 0x800004, size: 0x200000, crc: 0x62092FBD },
            Op::Load { kind: Load::Word64, file: "tko_20.bin", offset: 0x800006, size: 0x200000, crc: 0xB468A666 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tko_01.bin", offset: 0x0, size: 0x8000, crc: 0x6EDA50C2 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tko_11.bin", offset: 0x0, size: 0x200000, crc: 0xCA7179B1 },
            Op::Load { kind: Load::Word16Swap, file: "tko_12.bin", offset: 0x200000, size: 0x100000, crc: 0x83A4E635 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUB: RomSetDef = RomSetDef { name: "csclub", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csce.03a", offset: 0x0, size: 0x80000, crc: 0x824082BE },
            Op::Load { kind: Load::Word16Swap, file: "csce.04a", offset: 0x80000, size: 0x80000, crc: 0x74E6A4FE },
            Op::Load { kind: Load::Word16Swap, file: "csce.05a", offset: 0x100000, size: 0x80000, crc: 0x8AE0DF19 },
            Op::Load { kind: Load::Word16Swap, file: "csce.06a", offset: 0x180000, size: 0x80000, crc: 0x51F2F0D3 },
            Op::Load { kind: Load::Word16Swap, file: "csce.07a", offset: 0x200000, size: 0x80000, crc: 0x003968FD },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.73", offset: 0x800000, size: 0x80000, crc: 0x335F07C3 },
            Op::Load { kind: Load::Word64, file: "csc.74", offset: 0xa00000, size: 0x80000, crc: 0xAB215357 },
            Op::Load { kind: Load::Word64, file: "csc.75", offset: 0xc00000, size: 0x80000, crc: 0xA2367381 },
            Op::Load { kind: Load::Word64, file: "csc.76", offset: 0xe00000, size: 0x80000, crc: 0x728AAC1F },
            Op::Load { kind: Load::Word64, file: "csc.63", offset: 0x800002, size: 0x80000, crc: 0x3711B8CA },
            Op::Load { kind: Load::Word64, file: "csc.64", offset: 0xa00002, size: 0x80000, crc: 0x828A06D8 },
            Op::Load { kind: Load::Word64, file: "csc.65", offset: 0xc00002, size: 0x80000, crc: 0x86EE4569 },
            Op::Load { kind: Load::Word64, file: "csc.66", offset: 0xe00002, size: 0x80000, crc: 0xC24F577F },
            Op::Load { kind: Load::Word64, file: "csc.83", offset: 0x800004, size: 0x80000, crc: 0x0750D12A },
            Op::Load { kind: Load::Word64, file: "csc.84", offset: 0xa00004, size: 0x80000, crc: 0x90A92F39 },
            Op::Load { kind: Load::Word64, file: "csc.85", offset: 0xc00004, size: 0x80000, crc: 0xD08AB012 },
            Op::Load { kind: Load::Word64, file: "csc.86", offset: 0xe00004, size: 0x80000, crc: 0x41652583 },
            Op::Load { kind: Load::Word64, file: "csc.93", offset: 0x800006, size: 0x80000, crc: 0xA756C7F7 },
            Op::Load { kind: Load::Word64, file: "csc.94", offset: 0xa00006, size: 0x80000, crc: 0xFB7CCC73 },
            Op::Load { kind: Load::Word64, file: "csc.95", offset: 0xc00006, size: 0x80000, crc: 0x4D014297 },
            Op::Load { kind: Load::Word64, file: "csc.96", offset: 0xe00006, size: 0x80000, crc: 0x6754B1EF },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.51", offset: 0x0, size: 0x80000, crc: 0x5A52AFD5 },
            Op::Load { kind: Load::Word16Swap, file: "csc.52", offset: 0x80000, size: 0x80000, crc: 0x1408A811 },
            Op::Load { kind: Load::Word16Swap, file: "csc.53", offset: 0x100000, size: 0x80000, crc: 0x4FB9F57C },
            Op::Load { kind: Load::Word16Swap, file: "csc.54", offset: 0x180000, size: 0x80000, crc: 0x9A8F40EC },
            Op::Load { kind: Load::Word16Swap, file: "csc.55", offset: 0x200000, size: 0x80000, crc: 0x91529A91 },
            Op::Load { kind: Load::Word16Swap, file: "csc.56", offset: 0x280000, size: 0x80000, crc: 0x9A345334 },
            Op::Load { kind: Load::Word16Swap, file: "csc.57", offset: 0x300000, size: 0x80000, crc: 0xAEDC27F2 },
            Op::Load { kind: Load::Word16Swap, file: "csc.58", offset: 0x380000, size: 0x80000, crc: 0x2300B7B3 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csclub.key", offset: 0x0, size: 0x14, crc: 0x903907D7 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUB1: RomSetDef = RomSetDef { name: "csclub1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csce.03", offset: 0x0, size: 0x80000, crc: 0xF2C852EF },
            Op::Load { kind: Load::Word16Swap, file: "csce.04", offset: 0x80000, size: 0x80000, crc: 0x1184530F },
            Op::Load { kind: Load::Word16Swap, file: "csce.05", offset: 0x100000, size: 0x80000, crc: 0x804E2B6B },
            Op::Load { kind: Load::Word16Swap, file: "csce.06", offset: 0x180000, size: 0x80000, crc: 0x09277CB9 },
            Op::Load { kind: Load::Word16Swap, file: "csc.07", offset: 0x200000, size: 0x80000, crc: 0x01B05CAA },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.14m", offset: 0x800000, size: 0x200000, crc: 0xE8904AFA },
            Op::Load { kind: Load::Word64, file: "csc.16m", offset: 0x800002, size: 0x200000, crc: 0xC98C8079 },
            Op::Load { kind: Load::Word64, file: "csc.18m", offset: 0x800004, size: 0x200000, crc: 0xC030DF5A },
            Op::Load { kind: Load::Word64, file: "csc.20m", offset: 0x800006, size: 0x200000, crc: 0xB4E55863 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.11m", offset: 0x0, size: 0x200000, crc: 0xA027B827 },
            Op::Load { kind: Load::Word16Swap, file: "csc.12m", offset: 0x200000, size: 0x200000, crc: 0xCB7F6E55 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csclub.key", offset: 0x0, size: 0x14, crc: 0x903907D7 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUBJ: RomSetDef = RomSetDef { name: "csclubj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cscj.03", offset: 0x0, size: 0x80000, crc: 0xEC4DDAA2 },
            Op::Load { kind: Load::Word16Swap, file: "cscj.04", offset: 0x80000, size: 0x80000, crc: 0x60C632BB },
            Op::Load { kind: Load::Word16Swap, file: "cscj.05", offset: 0x100000, size: 0x80000, crc: 0xAD042003 },
            Op::Load { kind: Load::Word16Swap, file: "cscj.06", offset: 0x180000, size: 0x80000, crc: 0x169E4D40 },
            Op::Load { kind: Load::Word16Swap, file: "csc.07", offset: 0x200000, size: 0x80000, crc: 0x01B05CAA },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.14m", offset: 0x800000, size: 0x200000, crc: 0xE8904AFA },
            Op::Load { kind: Load::Word64, file: "csc.16m", offset: 0x800002, size: 0x200000, crc: 0xC98C8079 },
            Op::Load { kind: Load::Word64, file: "csc.18m", offset: 0x800004, size: 0x200000, crc: 0xC030DF5A },
            Op::Load { kind: Load::Word64, file: "csc.20m", offset: 0x800006, size: 0x200000, crc: 0xB4E55863 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.11m", offset: 0x0, size: 0x200000, crc: 0xA027B827 },
            Op::Load { kind: Load::Word16Swap, file: "csc.12m", offset: 0x200000, size: 0x200000, crc: 0xCB7F6E55 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csclubj.key", offset: 0x0, size: 0x14, crc: 0x519A04DB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUBJY: RomSetDef = RomSetDef { name: "csclubjy", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cscj.03", offset: 0x0, size: 0x80000, crc: 0xEC4DDAA2 },
            Op::Load { kind: Load::Word16Swap, file: "cscj.04", offset: 0x80000, size: 0x80000, crc: 0x60C632BB },
            Op::Load { kind: Load::Word16Swap, file: "cscj.05", offset: 0x100000, size: 0x80000, crc: 0xAD042003 },
            Op::Load { kind: Load::Word16Swap, file: "cscj.06", offset: 0x180000, size: 0x80000, crc: 0x169E4D40 },
            Op::Load { kind: Load::Word16Swap, file: "csc.07", offset: 0x200000, size: 0x80000, crc: 0x01B05CAA },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.73", offset: 0x800000, size: 0x80000, crc: 0x335F07C3 },
            Op::Load { kind: Load::Word64, file: "csc.74", offset: 0xa00000, size: 0x80000, crc: 0xAB215357 },
            Op::Load { kind: Load::Word64, file: "csc.75", offset: 0xc00000, size: 0x80000, crc: 0xA2367381 },
            Op::Load { kind: Load::Word64, file: "csc.76", offset: 0xe00000, size: 0x80000, crc: 0x728AAC1F },
            Op::Load { kind: Load::Word64, file: "csc.63", offset: 0x800002, size: 0x80000, crc: 0x3711B8CA },
            Op::Load { kind: Load::Word64, file: "csc.64", offset: 0xa00002, size: 0x80000, crc: 0x828A06D8 },
            Op::Load { kind: Load::Word64, file: "csc.65", offset: 0xc00002, size: 0x80000, crc: 0x86EE4569 },
            Op::Load { kind: Load::Word64, file: "csc.66", offset: 0xe00002, size: 0x80000, crc: 0xC24F577F },
            Op::Load { kind: Load::Word64, file: "csc.83", offset: 0x800004, size: 0x80000, crc: 0x0750D12A },
            Op::Load { kind: Load::Word64, file: "csc.84", offset: 0xa00004, size: 0x80000, crc: 0x90A92F39 },
            Op::Load { kind: Load::Word64, file: "csc.85", offset: 0xc00004, size: 0x80000, crc: 0xD08AB012 },
            Op::Load { kind: Load::Word64, file: "csc.86", offset: 0xe00004, size: 0x80000, crc: 0x41652583 },
            Op::Load { kind: Load::Word64, file: "csc.93", offset: 0x800006, size: 0x80000, crc: 0xA756C7F7 },
            Op::Load { kind: Load::Word64, file: "csc.94", offset: 0xa00006, size: 0x80000, crc: 0xFB7CCC73 },
            Op::Load { kind: Load::Word64, file: "csc.95", offset: 0xc00006, size: 0x80000, crc: 0x4D014297 },
            Op::Load { kind: Load::Word64, file: "csc.96", offset: 0xe00006, size: 0x80000, crc: 0x6754B1EF },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.51", offset: 0x0, size: 0x80000, crc: 0x5A52AFD5 },
            Op::Load { kind: Load::Word16Swap, file: "csc.52", offset: 0x80000, size: 0x80000, crc: 0x1408A811 },
            Op::Load { kind: Load::Word16Swap, file: "csc.53", offset: 0x100000, size: 0x80000, crc: 0x4FB9F57C },
            Op::Load { kind: Load::Word16Swap, file: "csc.54", offset: 0x180000, size: 0x80000, crc: 0x9A8F40EC },
            Op::Load { kind: Load::Word16Swap, file: "csc.55", offset: 0x200000, size: 0x80000, crc: 0x91529A91 },
            Op::Load { kind: Load::Word16Swap, file: "csc.56", offset: 0x280000, size: 0x80000, crc: 0x9A345334 },
            Op::Load { kind: Load::Word16Swap, file: "csc.57", offset: 0x300000, size: 0x80000, crc: 0xAEDC27F2 },
            Op::Load { kind: Load::Word16Swap, file: "csc.58", offset: 0x380000, size: 0x80000, crc: 0x2300B7B3 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csclubj.key", offset: 0x0, size: 0x14, crc: 0x519A04DB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUBA: RomSetDef = RomSetDef { name: "cscluba", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csca.03", offset: 0x0, size: 0x80000, crc: 0xB6ACD708 },
            Op::Load { kind: Load::Word16Swap, file: "csca.04", offset: 0x80000, size: 0x80000, crc: 0xD44AE35F },
            Op::Load { kind: Load::Word16Swap, file: "csca.05", offset: 0x100000, size: 0x80000, crc: 0x8DA76AEC },
            Op::Load { kind: Load::Word16Swap, file: "csca.06", offset: 0x180000, size: 0x80000, crc: 0xA1B7B1EE },
            Op::Load { kind: Load::Word16Swap, file: "csc.07", offset: 0x200000, size: 0x80000, crc: 0x01B05CAA },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.14m", offset: 0x800000, size: 0x200000, crc: 0xE8904AFA },
            Op::Load { kind: Load::Word64, file: "csc.16m", offset: 0x800002, size: 0x200000, crc: 0xC98C8079 },
            Op::Load { kind: Load::Word64, file: "csc.18m", offset: 0x800004, size: 0x200000, crc: 0xC030DF5A },
            Op::Load { kind: Load::Word64, file: "csc.20m", offset: 0x800006, size: 0x200000, crc: 0xB4E55863 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.11m", offset: 0x0, size: 0x200000, crc: 0xA027B827 },
            Op::Load { kind: Load::Word16Swap, file: "csc.12m", offset: 0x200000, size: 0x200000, crc: 0xCB7F6E55 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cscluba.key", offset: 0x0, size: 0x14, crc: 0x591908DC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUBH: RomSetDef = RomSetDef { name: "csclubh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csch.03", offset: 0x0, size: 0x80000, crc: 0x0DD7E46D },
            Op::Load { kind: Load::Word16Swap, file: "csch.04", offset: 0x80000, size: 0x80000, crc: 0x486E8143 },
            Op::Load { kind: Load::Word16Swap, file: "csch.05", offset: 0x100000, size: 0x80000, crc: 0x9E509DFB },
            Op::Load { kind: Load::Word16Swap, file: "csch.06", offset: 0x180000, size: 0x80000, crc: 0x817BA313 },
            Op::Load { kind: Load::Word16Swap, file: "csc.07", offset: 0x200000, size: 0x80000, crc: 0x01B05CAA },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.14m", offset: 0x800000, size: 0x200000, crc: 0xE8904AFA },
            Op::Load { kind: Load::Word64, file: "csc.16m", offset: 0x800002, size: 0x200000, crc: 0xC98C8079 },
            Op::Load { kind: Load::Word64, file: "csc.18m", offset: 0x800004, size: 0x200000, crc: 0xC030DF5A },
            Op::Load { kind: Load::Word64, file: "csc.20m", offset: 0x800006, size: 0x200000, crc: 0xB4E55863 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.11m", offset: 0x0, size: 0x200000, crc: 0xA027B827 },
            Op::Load { kind: Load::Word16Swap, file: "csc.12m", offset: 0x200000, size: 0x200000, crc: 0xCB7F6E55 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csclubh.key", offset: 0x0, size: 0x14, crc: 0xB0ADC39E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CYBOTS: RomSetDef = RomSetDef { name: "cybots", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cybe.03", offset: 0x0, size: 0x80000, crc: 0x234381CD },
            Op::Load { kind: Load::Word16Swap, file: "cybe.04", offset: 0x80000, size: 0x80000, crc: 0x80691061 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.05", offset: 0x100000, size: 0x80000, crc: 0xEC40408E },
            Op::Load { kind: Load::Word16Swap, file: "cyb.06", offset: 0x180000, size: 0x80000, crc: 0x1AD0BED2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.07", offset: 0x200000, size: 0x80000, crc: 0x6245A39A },
            Op::Load { kind: Load::Word16Swap, file: "cyb.08", offset: 0x280000, size: 0x80000, crc: 0x4B48E223 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.09", offset: 0x300000, size: 0x80000, crc: 0xE15238F6 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.10", offset: 0x380000, size: 0x80000, crc: 0x75F4003B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cyb.13m", offset: 0x0, size: 0x400000, crc: 0xF0DCE192 },
            Op::Load { kind: Load::Word64, file: "cyb.15m", offset: 0x2, size: 0x400000, crc: 0x187AA39C },
            Op::Load { kind: Load::Word64, file: "cyb.17m", offset: 0x4, size: 0x400000, crc: 0x8A0E4B12 },
            Op::Load { kind: Load::Word64, file: "cyb.19m", offset: 0x6, size: 0x400000, crc: 0x34B62612 },
            Op::Load { kind: Load::Word64, file: "cyb.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1537957 },
            Op::Load { kind: Load::Word64, file: "cyb.16m", offset: 0x1000002, size: 0x400000, crc: 0x15349E86 },
            Op::Load { kind: Load::Word64, file: "cyb.18m", offset: 0x1000004, size: 0x400000, crc: 0xD83E977D },
            Op::Load { kind: Load::Word64, file: "cyb.20m", offset: 0x1000006, size: 0x400000, crc: 0x77CDAD5C },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cyb.01", offset: 0x0, size: 0x8000, crc: 0x9C0FB079 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "cyb.02", offset: 0x28000, size: 0x20000, crc: 0x51CB0C4E },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cyb.11m", offset: 0x0, size: 0x200000, crc: 0x362CCAB2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.12m", offset: 0x200000, size: 0x200000, crc: 0x7066E9CC },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cybots.key", offset: 0x0, size: 0x14, crc: 0x9BBCBEF3 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CYBOTSU: RomSetDef = RomSetDef { name: "cybotsu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cybu.03", offset: 0x0, size: 0x80000, crc: 0xDB4DA8F4 },
            Op::Load { kind: Load::Word16Swap, file: "cybu.04", offset: 0x80000, size: 0x80000, crc: 0x1EEC68AC },
            Op::Load { kind: Load::Word16Swap, file: "cyb.05", offset: 0x100000, size: 0x80000, crc: 0xEC40408E },
            Op::Load { kind: Load::Word16Swap, file: "cyb.06", offset: 0x180000, size: 0x80000, crc: 0x1AD0BED2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.07", offset: 0x200000, size: 0x80000, crc: 0x6245A39A },
            Op::Load { kind: Load::Word16Swap, file: "cyb.08", offset: 0x280000, size: 0x80000, crc: 0x4B48E223 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.09", offset: 0x300000, size: 0x80000, crc: 0xE15238F6 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.10", offset: 0x380000, size: 0x80000, crc: 0x75F4003B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cyb.13m", offset: 0x0, size: 0x400000, crc: 0xF0DCE192 },
            Op::Load { kind: Load::Word64, file: "cyb.15m", offset: 0x2, size: 0x400000, crc: 0x187AA39C },
            Op::Load { kind: Load::Word64, file: "cyb.17m", offset: 0x4, size: 0x400000, crc: 0x8A0E4B12 },
            Op::Load { kind: Load::Word64, file: "cyb.19m", offset: 0x6, size: 0x400000, crc: 0x34B62612 },
            Op::Load { kind: Load::Word64, file: "cyb.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1537957 },
            Op::Load { kind: Load::Word64, file: "cyb.16m", offset: 0x1000002, size: 0x400000, crc: 0x15349E86 },
            Op::Load { kind: Load::Word64, file: "cyb.18m", offset: 0x1000004, size: 0x400000, crc: 0xD83E977D },
            Op::Load { kind: Load::Word64, file: "cyb.20m", offset: 0x1000006, size: 0x400000, crc: 0x77CDAD5C },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cyb.01", offset: 0x0, size: 0x8000, crc: 0x9C0FB079 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "cyb.02", offset: 0x28000, size: 0x20000, crc: 0x51CB0C4E },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cyb.11m", offset: 0x0, size: 0x200000, crc: 0x362CCAB2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.12m", offset: 0x200000, size: 0x200000, crc: 0x7066E9CC },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cybotsu.key", offset: 0x0, size: 0x14, crc: 0x7A09403C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CYBOTSJ: RomSetDef = RomSetDef { name: "cybotsj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cybj.03", offset: 0x0, size: 0x80000, crc: 0x6096EADA },
            Op::Load { kind: Load::Word16Swap, file: "cybj.04", offset: 0x80000, size: 0x80000, crc: 0x7B0FFAA9 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.05", offset: 0x100000, size: 0x80000, crc: 0xEC40408E },
            Op::Load { kind: Load::Word16Swap, file: "cyb.06", offset: 0x180000, size: 0x80000, crc: 0x1AD0BED2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.07", offset: 0x200000, size: 0x80000, crc: 0x6245A39A },
            Op::Load { kind: Load::Word16Swap, file: "cyb.08", offset: 0x280000, size: 0x80000, crc: 0x4B48E223 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.09", offset: 0x300000, size: 0x80000, crc: 0xE15238F6 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.10", offset: 0x380000, size: 0x80000, crc: 0x75F4003B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cyb.13m", offset: 0x0, size: 0x400000, crc: 0xF0DCE192 },
            Op::Load { kind: Load::Word64, file: "cyb.15m", offset: 0x2, size: 0x400000, crc: 0x187AA39C },
            Op::Load { kind: Load::Word64, file: "cyb.17m", offset: 0x4, size: 0x400000, crc: 0x8A0E4B12 },
            Op::Load { kind: Load::Word64, file: "cyb.19m", offset: 0x6, size: 0x400000, crc: 0x34B62612 },
            Op::Load { kind: Load::Word64, file: "cyb.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1537957 },
            Op::Load { kind: Load::Word64, file: "cyb.16m", offset: 0x1000002, size: 0x400000, crc: 0x15349E86 },
            Op::Load { kind: Load::Word64, file: "cyb.18m", offset: 0x1000004, size: 0x400000, crc: 0xD83E977D },
            Op::Load { kind: Load::Word64, file: "cyb.20m", offset: 0x1000006, size: 0x400000, crc: 0x77CDAD5C },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cyb.01", offset: 0x0, size: 0x8000, crc: 0x9C0FB079 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "cyb.02", offset: 0x28000, size: 0x20000, crc: 0x51CB0C4E },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cyb.11m", offset: 0x0, size: 0x200000, crc: 0x362CCAB2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.12m", offset: 0x200000, size: 0x200000, crc: 0x7066E9CC },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cybotsj.key", offset: 0x0, size: 0x14, crc: 0xD4D560B7 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTOD: RomSetDef = RomSetDef { name: "ddtod", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dade.03c", offset: 0x0, size: 0x80000, crc: 0x8E73533D },
            Op::Load { kind: Load::Word16Swap, file: "dade.04c", offset: 0x80000, size: 0x80000, crc: 0x00C2E82E },
            Op::Load { kind: Load::Word16Swap, file: "dade.05c", offset: 0x100000, size: 0x80000, crc: 0xEA996008 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06a", offset: 0x180000, size: 0x80000, crc: 0x6225495A },
            Op::Load { kind: Load::Word16Swap, file: "dad.07a", offset: 0x200000, size: 0x80000, crc: 0xB3480EC3 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtod.key", offset: 0x0, size: 0x14, crc: 0x41DFCA41 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODR1: RomSetDef = RomSetDef { name: "ddtodr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dade.03a", offset: 0x0, size: 0x80000, crc: 0x665A035E },
            Op::Load { kind: Load::Word16Swap, file: "dade.04a", offset: 0x80000, size: 0x80000, crc: 0x02613207 },
            Op::Load { kind: Load::Word16Swap, file: "dade.05a", offset: 0x100000, size: 0x80000, crc: 0x36845996 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtod.key", offset: 0x0, size: 0x14, crc: 0x41DFCA41 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODU: RomSetDef = RomSetDef { name: "ddtodu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadu.03b", offset: 0x0, size: 0x80000, crc: 0xA519905F },
            Op::Load { kind: Load::Word16Swap, file: "dadu.04b", offset: 0x80000, size: 0x80000, crc: 0x52562D38 },
            Op::Load { kind: Load::Word16Swap, file: "dadu.05b", offset: 0x100000, size: 0x80000, crc: 0xEE1CFBFE },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodu.key", offset: 0x0, size: 0x14, crc: 0x7C03EC9E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODUR1: RomSetDef = RomSetDef { name: "ddtodur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadu.03a", offset: 0x0, size: 0x80000, crc: 0x4413F177 },
            Op::Load { kind: Load::Word16Swap, file: "dadu.04a", offset: 0x80000, size: 0x80000, crc: 0x168DE230 },
            Op::Load { kind: Load::Word16Swap, file: "dadu.05a", offset: 0x100000, size: 0x80000, crc: 0x03D39E91 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodu.key", offset: 0x0, size: 0x14, crc: 0x7C03EC9E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODJ: RomSetDef = RomSetDef { name: "ddtodj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadj.03c", offset: 0x0, size: 0x80000, crc: 0x0B1B5798 },
            Op::Load { kind: Load::Word16Swap, file: "dadj.04c", offset: 0x80000, size: 0x80000, crc: 0xC6A2FBC8 },
            Op::Load { kind: Load::Word16Swap, file: "dadj.05c", offset: 0x100000, size: 0x80000, crc: 0x189B15FE },
            Op::Load { kind: Load::Word16Swap, file: "dad.06a", offset: 0x180000, size: 0x80000, crc: 0x6225495A },
            Op::Load { kind: Load::Word16Swap, file: "dad.07a", offset: 0x200000, size: 0x80000, crc: 0xB3480EC3 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodj.key", offset: 0x0, size: 0x14, crc: 0x5414DFCA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODJR1: RomSetDef = RomSetDef { name: "ddtodjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadj.03b", offset: 0x0, size: 0x80000, crc: 0x87606B85 },
            Op::Load { kind: Load::Word16Swap, file: "dadj.04b", offset: 0x80000, size: 0x80000, crc: 0x24D49575 },
            Op::Load { kind: Load::Word16Swap, file: "dadj.05b", offset: 0x100000, size: 0x80000, crc: 0x56CE51F7 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodj.key", offset: 0x0, size: 0x14, crc: 0x5414DFCA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODJR2: RomSetDef = RomSetDef { name: "ddtodjr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadj.03a", offset: 0x0, size: 0x80000, crc: 0x711638DC },
            Op::Load { kind: Load::Word16Swap, file: "dadj.04a", offset: 0x80000, size: 0x80000, crc: 0x4869639C },
            Op::Load { kind: Load::Word16Swap, file: "dadj.05a", offset: 0x100000, size: 0x80000, crc: 0x484C0EFA },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodj.key", offset: 0x0, size: 0x14, crc: 0x5414DFCA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODA: RomSetDef = RomSetDef { name: "ddtoda", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dada.03c", offset: 0x0, size: 0x80000, crc: 0xBF243E15 },
            Op::Load { kind: Load::Word16Swap, file: "dada.04c", offset: 0x80000, size: 0x80000, crc: 0x76551EEC },
            Op::Load { kind: Load::Word16Swap, file: "dada.05c", offset: 0x100000, size: 0x80000, crc: 0x0A0AD827 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06a", offset: 0x180000, size: 0x80000, crc: 0x6225495A },
            Op::Load { kind: Load::Word16Swap, file: "dad.07a", offset: 0x200000, size: 0x80000, crc: 0xB3480EC3 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtoda.key", offset: 0x0, size: 0x14, crc: 0xE5E8D1B8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODAR1: RomSetDef = RomSetDef { name: "ddtodar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dada.03a", offset: 0x0, size: 0x80000, crc: 0xFC6F2DD7 },
            Op::Load { kind: Load::Word16Swap, file: "dada.04a", offset: 0x80000, size: 0x80000, crc: 0xD4BE4009 },
            Op::Load { kind: Load::Word16Swap, file: "dada.05a", offset: 0x100000, size: 0x80000, crc: 0x6712D1CF },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtoda.key", offset: 0x0, size: 0x14, crc: 0xE5E8D1B8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODH: RomSetDef = RomSetDef { name: "ddtodh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadh.03c", offset: 0x0, size: 0x80000, crc: 0x5750A861 },
            Op::Load { kind: Load::Word16Swap, file: "dadh.04c", offset: 0x80000, size: 0x80000, crc: 0xCFBF1B56 },
            Op::Load { kind: Load::Word16Swap, file: "dadh.05c", offset: 0x100000, size: 0x80000, crc: 0xA6E562BA },
            Op::Load { kind: Load::Word16Swap, file: "dad.06a", offset: 0x180000, size: 0x80000, crc: 0x6225495A },
            Op::Load { kind: Load::Word16Swap, file: "dad.07a", offset: 0x200000, size: 0x80000, crc: 0xB3480EC3 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodh.key", offset: 0x0, size: 0x14, crc: 0x65F33A1C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODHR1: RomSetDef = RomSetDef { name: "ddtodhr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadh.03b", offset: 0x0, size: 0x80000, crc: 0xAE0CB98E },
            Op::Load { kind: Load::Word16Swap, file: "dadh.04b", offset: 0x80000, size: 0x80000, crc: 0xB5774363 },
            Op::Load { kind: Load::Word16Swap, file: "dadh.05b", offset: 0x100000, size: 0x80000, crc: 0x6CE2A485 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodh.key", offset: 0x0, size: 0x14, crc: 0x65F33A1C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDTODHR2: RomSetDef = RomSetDef { name: "ddtodhr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dadh.03a", offset: 0x0, size: 0x80000, crc: 0x43D04AA3 },
            Op::Load { kind: Load::Word16Swap, file: "dadh.04a", offset: 0x80000, size: 0x80000, crc: 0x8B8D296C },
            Op::Load { kind: Load::Word16Swap, file: "dadh.05a", offset: 0x100000, size: 0x80000, crc: 0xDAAE6B14 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06", offset: 0x180000, size: 0x80000, crc: 0x13AA3E56 },
            Op::Load { kind: Load::Word16Swap, file: "dad.07", offset: 0x200000, size: 0x80000, crc: 0x431CB6DD },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddtodh.key", offset: 0x0, size: 0x14, crc: 0x65F33A1C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOM: RomSetDef = RomSetDef { name: "ddsom", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2e.03e", offset: 0x0, size: 0x80000, crc: 0x449361AF },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.04e", offset: 0x80000, size: 0x80000, crc: 0x5B7052B6 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.05e", offset: 0x100000, size: 0x80000, crc: 0x788D5F60 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.06e", offset: 0x180000, size: 0x80000, crc: 0xE0807E1E },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.07", offset: 0x200000, size: 0x80000, crc: 0xBB777A02 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.08", offset: 0x280000, size: 0x80000, crc: 0x30970890 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.09", offset: 0x300000, size: 0x80000, crc: 0x99E2194D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.10", offset: 0x380000, size: 0x80000, crc: 0xE198805E },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsom.key", offset: 0x0, size: 0x14, crc: 0x541E425D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMR1: RomSetDef = RomSetDef { name: "ddsomr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2e.03d", offset: 0x0, size: 0x80000, crc: 0x6C084AB5 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.04d", offset: 0x80000, size: 0x80000, crc: 0x9B94A947 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.05d", offset: 0x100000, size: 0x80000, crc: 0x5D6A63C6 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.06d", offset: 0x180000, size: 0x80000, crc: 0x31BDE8EE },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.07", offset: 0x200000, size: 0x80000, crc: 0xBB777A02 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.08", offset: 0x280000, size: 0x80000, crc: 0x30970890 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.09", offset: 0x300000, size: 0x80000, crc: 0x99E2194D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.10", offset: 0x380000, size: 0x80000, crc: 0xE198805E },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsom.key", offset: 0x0, size: 0x14, crc: 0x541E425D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMR2: RomSetDef = RomSetDef { name: "ddsomr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2e.03b", offset: 0x0, size: 0x80000, crc: 0xCD2DEB66 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.04b", offset: 0x80000, size: 0x80000, crc: 0xBFEE43CC },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.05b", offset: 0x100000, size: 0x80000, crc: 0x049AB19D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.06b", offset: 0x180000, size: 0x80000, crc: 0x3994FB8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.07", offset: 0x200000, size: 0x80000, crc: 0xBB777A02 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.08", offset: 0x280000, size: 0x80000, crc: 0x30970890 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.09", offset: 0x300000, size: 0x80000, crc: 0x99E2194D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.10", offset: 0x380000, size: 0x80000, crc: 0xE198805E },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsom.key", offset: 0x0, size: 0x14, crc: 0x541E425D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMR3: RomSetDef = RomSetDef { name: "ddsomr3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2e.03a", offset: 0x0, size: 0x80000, crc: 0x6DE67678 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.04a", offset: 0x80000, size: 0x80000, crc: 0x0E45739A },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.05a", offset: 0x100000, size: 0x80000, crc: 0x3DCE8025 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.06a", offset: 0x180000, size: 0x80000, crc: 0x51BAFBEF },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.07", offset: 0x200000, size: 0x80000, crc: 0xBB777A02 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.08", offset: 0x280000, size: 0x80000, crc: 0x30970890 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.09", offset: 0x300000, size: 0x80000, crc: 0x99E2194D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.10", offset: 0x380000, size: 0x80000, crc: 0xE198805E },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsom.key", offset: 0x0, size: 0x14, crc: 0x541E425D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMU: RomSetDef = RomSetDef { name: "ddsomu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2u.03g", offset: 0x0, size: 0x80000, crc: 0xFB089B39 },
            Op::Load { kind: Load::Word16Swap, file: "dd2u.04g", offset: 0x80000, size: 0x80000, crc: 0xCD432B73 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05g", offset: 0x100000, size: 0x80000, crc: 0x5EB1991C },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06g", offset: 0x180000, size: 0x80000, crc: 0xC26B5E55 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomu.key", offset: 0x0, size: 0x14, crc: 0x09AE0F7C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMUR1: RomSetDef = RomSetDef { name: "ddsomur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2u.03d", offset: 0x0, size: 0x80000, crc: 0x0F700D84 },
            Op::Load { kind: Load::Word16Swap, file: "dd2u.04d", offset: 0x80000, size: 0x80000, crc: 0xB99EB254 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05d", offset: 0x100000, size: 0x80000, crc: 0xB23061F3 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06d", offset: 0x180000, size: 0x80000, crc: 0x8BF1D8CE },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomu.key", offset: 0x0, size: 0x14, crc: 0x09AE0F7C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMJ: RomSetDef = RomSetDef { name: "ddsomj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2j.03g", offset: 0x0, size: 0x80000, crc: 0xE6C8C985 },
            Op::Load { kind: Load::Word16Swap, file: "dd2j.04g", offset: 0x80000, size: 0x80000, crc: 0x8386C0BD },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05g", offset: 0x100000, size: 0x80000, crc: 0x5EB1991C },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06g", offset: 0x180000, size: 0x80000, crc: 0xC26B5E55 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomj.key", offset: 0x0, size: 0x14, crc: 0xD8DADB22 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMJR1: RomSetDef = RomSetDef { name: "ddsomjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2j.03e", offset: 0x0, size: 0x80000, crc: 0xB2FD4A24 },
            Op::Load { kind: Load::Word16Swap, file: "dd2j.04e", offset: 0x80000, size: 0x80000, crc: 0x3A68C310 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05e", offset: 0x100000, size: 0x80000, crc: 0xAA56F42F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06e", offset: 0x180000, size: 0x80000, crc: 0x2F8CD040 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomj.key", offset: 0x0, size: 0x14, crc: 0xD8DADB22 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMJR2: RomSetDef = RomSetDef { name: "ddsomjr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2j.03b", offset: 0x0, size: 0x80000, crc: 0x965D74E5 },
            Op::Load { kind: Load::Word16Swap, file: "dd2j.04b", offset: 0x80000, size: 0x80000, crc: 0x958EB8F3 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05b", offset: 0x100000, size: 0x80000, crc: 0xD38571CA },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06b", offset: 0x180000, size: 0x80000, crc: 0x6D5A3BBB },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomj.key", offset: 0x0, size: 0x14, crc: 0xD8DADB22 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMA: RomSetDef = RomSetDef { name: "ddsoma", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2a.03g", offset: 0x0, size: 0x80000, crc: 0x0B4FEC22 },
            Op::Load { kind: Load::Word16Swap, file: "dd2a.04g", offset: 0x80000, size: 0x80000, crc: 0x055B7019 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05g", offset: 0x100000, size: 0x80000, crc: 0x5EB1991C },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06g", offset: 0x180000, size: 0x80000, crc: 0xC26B5E55 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsoma.key", offset: 0x0, size: 0x14, crc: 0x8C3CC560 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMAR1: RomSetDef = RomSetDef { name: "ddsomar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2a.03c", offset: 0x0, size: 0x80000, crc: 0x17162039 },
            Op::Load { kind: Load::Word16Swap, file: "dd2a.04c", offset: 0x80000, size: 0x80000, crc: 0x950BEC38 },
            Op::Load { kind: Load::Word16Swap, file: "dd2a.05c", offset: 0x100000, size: 0x80000, crc: 0xFA298EBA },
            Op::Load { kind: Load::Word16Swap, file: "dd2a.06c", offset: 0x180000, size: 0x80000, crc: 0x28F75B35 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2.10", offset: 0x380000, size: 0x80000, crc: 0xAD954C26 },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsoma.key", offset: 0x0, size: 0x14, crc: 0x8C3CC560 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMB: RomSetDef = RomSetDef { name: "ddsomb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2b.03a", offset: 0x0, size: 0x80000, crc: 0xE8CE7FBB },
            Op::Load { kind: Load::Word16Swap, file: "dd2b.04a", offset: 0x80000, size: 0x80000, crc: 0x6B679664 },
            Op::Load { kind: Load::Word16Swap, file: "dd2b.05a", offset: 0x100000, size: 0x80000, crc: 0x9B2534EB },
            Op::Load { kind: Load::Word16Swap, file: "dd2b.06a", offset: 0x180000, size: 0x80000, crc: 0x3B21BA59 },
            Op::Load { kind: Load::Word16Swap, file: "dd2b.07", offset: 0x200000, size: 0x80000, crc: 0xFCE2558D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.08", offset: 0x280000, size: 0x80000, crc: 0x30970890 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.09", offset: 0x300000, size: 0x80000, crc: 0x99E2194D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.10", offset: 0x380000, size: 0x80000, crc: 0xE198805E },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomb.key", offset: 0x0, size: 0x14, crc: 0x00B4CC49 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DDSOMH: RomSetDef = RomSetDef { name: "ddsomh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2h.03a", offset: 0x0, size: 0x80000, crc: 0xE472C9F3 },
            Op::Load { kind: Load::Word16Swap, file: "dd2h.04a", offset: 0x80000, size: 0x80000, crc: 0x315A7706 },
            Op::Load { kind: Load::Word16Swap, file: "dd2h.05a", offset: 0x100000, size: 0x80000, crc: 0x9B2534EB },
            Op::Load { kind: Load::Word16Swap, file: "dd2h.06a", offset: 0x180000, size: 0x80000, crc: 0x3B21BA59 },
            Op::Load { kind: Load::Word16Swap, file: "dd2h.07a", offset: 0x200000, size: 0x80000, crc: 0xFCE2558D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.08", offset: 0x280000, size: 0x80000, crc: 0x30970890 },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.09", offset: 0x300000, size: 0x80000, crc: 0x99E2194D },
            Op::Load { kind: Load::Word16Swap, file: "dd2e.10", offset: 0x380000, size: 0x80000, crc: 0xE198805E },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ddsomh.key", offset: 0x0, size: 0x14, crc: 0xCAF6B540 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DIMAHOO: RomSetDef = RomSetDef { name: "dimahoo", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmde.03", offset: 0x0, size: 0x80000, crc: 0x968FCECD },
            Op::Load { kind: Load::Word16Swap, file: "gmd.04", offset: 0x80000, size: 0x80000, crc: 0x37485567 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.05", offset: 0x100000, size: 0x80000, crc: 0xDA269FFB },
            Op::Load { kind: Load::Word16Swap, file: "gmd.06", offset: 0x180000, size: 0x80000, crc: 0x55B483C9 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "gmd.13m", offset: 0x0, size: 0x400000, crc: 0x80DD19F0 },
            Op::Load { kind: Load::Word64, file: "gmd.15m", offset: 0x2, size: 0x400000, crc: 0xDFD93A78 },
            Op::Load { kind: Load::Word64, file: "gmd.17m", offset: 0x4, size: 0x400000, crc: 0x16356520 },
            Op::Load { kind: Load::Word64, file: "gmd.19m", offset: 0x6, size: 0x400000, crc: 0xDFC33031 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gmd.01", offset: 0x0, size: 0x8000, crc: 0x3F9BC985 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "gmd.02", offset: 0x28000, size: 0x20000, crc: 0x3FD39DDE },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmd.11m", offset: 0x0, size: 0x400000, crc: 0x06A65542 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.12m", offset: 0x400000, size: 0x400000, crc: 0x50BC7A31 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dimahoo.key", offset: 0x0, size: 0x14, crc: 0x7D6D2DB9 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DIMAHOOU: RomSetDef = RomSetDef { name: "dimahoou", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmdu.03", offset: 0x0, size: 0x80000, crc: 0x43BCB15F },
            Op::Load { kind: Load::Word16Swap, file: "gmd.04", offset: 0x80000, size: 0x80000, crc: 0x37485567 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.05", offset: 0x100000, size: 0x80000, crc: 0xDA269FFB },
            Op::Load { kind: Load::Word16Swap, file: "gmd.06", offset: 0x180000, size: 0x80000, crc: 0x55B483C9 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "gmd.13m", offset: 0x0, size: 0x400000, crc: 0x80DD19F0 },
            Op::Load { kind: Load::Word64, file: "gmd.15m", offset: 0x2, size: 0x400000, crc: 0xDFD93A78 },
            Op::Load { kind: Load::Word64, file: "gmd.17m", offset: 0x4, size: 0x400000, crc: 0x16356520 },
            Op::Load { kind: Load::Word64, file: "gmd.19m", offset: 0x6, size: 0x400000, crc: 0xDFC33031 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gmd.01", offset: 0x0, size: 0x8000, crc: 0x3F9BC985 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "gmd.02", offset: 0x28000, size: 0x20000, crc: 0x3FD39DDE },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmd.11m", offset: 0x0, size: 0x400000, crc: 0x06A65542 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.12m", offset: 0x400000, size: 0x400000, crc: 0x50BC7A31 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dimahoou.key", offset: 0x0, size: 0x14, crc: 0x8254D7AB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_GMAHOU: RomSetDef = RomSetDef { name: "gmahou", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmdj.03", offset: 0x0, size: 0x80000, crc: 0xCD6979E3 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.04", offset: 0x80000, size: 0x80000, crc: 0x37485567 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.05", offset: 0x100000, size: 0x80000, crc: 0xDA269FFB },
            Op::Load { kind: Load::Word16Swap, file: "gmd.06", offset: 0x180000, size: 0x80000, crc: 0x55B483C9 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "gmd.13m", offset: 0x0, size: 0x400000, crc: 0x80DD19F0 },
            Op::Load { kind: Load::Word64, file: "gmd.15m", offset: 0x2, size: 0x400000, crc: 0xDFD93A78 },
            Op::Load { kind: Load::Word64, file: "gmd.17m", offset: 0x4, size: 0x400000, crc: 0x16356520 },
            Op::Load { kind: Load::Word64, file: "gmd.19m", offset: 0x6, size: 0x400000, crc: 0xDFC33031 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gmd.01", offset: 0x0, size: 0x8000, crc: 0x3F9BC985 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "gmd.02", offset: 0x28000, size: 0x20000, crc: 0x3FD39DDE },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmd.11m", offset: 0x0, size: 0x400000, crc: 0x06A65542 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.12m", offset: 0x400000, size: 0x400000, crc: 0x50BC7A31 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gmahou.key", offset: 0x0, size: 0x14, crc: 0x76A5E659 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLK: RomSetDef = RomSetDef { name: "dstlk", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vame.03a", offset: 0x0, size: 0x80000, crc: 0x004C9CFF },
            Op::Load { kind: Load::Word16Swap, file: "vame.04a", offset: 0x80000, size: 0x80000, crc: 0xAE413FF2 },
            Op::Load { kind: Load::Word16Swap, file: "vame.05a", offset: 0x100000, size: 0x80000, crc: 0x60678756 },
            Op::Load { kind: Load::Word16Swap, file: "vame.06a", offset: 0x180000, size: 0x80000, crc: 0x912870B3 },
            Op::Load { kind: Load::Word16Swap, file: "vame.07a", offset: 0x200000, size: 0x80000, crc: 0xDABAE3E8 },
            Op::Load { kind: Load::Word16Swap, file: "vame.08a", offset: 0x280000, size: 0x80000, crc: 0x2C6E3077 },
            Op::Load { kind: Load::Word16Swap, file: "vame.09a", offset: 0x300000, size: 0x80000, crc: 0xF16DB74B },
            Op::Load { kind: Load::Word16Swap, file: "vame.10a", offset: 0x380000, size: 0x80000, crc: 0x701E2147 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dstlk.key", offset: 0x0, size: 0x14, crc: 0xCFA46DEC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLKU: RomSetDef = RomSetDef { name: "dstlku", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamu.03b", offset: 0x0, size: 0x80000, crc: 0x68A6343F },
            Op::Load { kind: Load::Word16Swap, file: "vamu.04b", offset: 0x80000, size: 0x80000, crc: 0x58161453 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.05b", offset: 0x100000, size: 0x80000, crc: 0xDFC038B8 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.06b", offset: 0x180000, size: 0x80000, crc: 0xC3842C89 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.07b", offset: 0x200000, size: 0x80000, crc: 0x25B60B6E },
            Op::Load { kind: Load::Word16Swap, file: "vamu.08b", offset: 0x280000, size: 0x80000, crc: 0x2113C596 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.09b", offset: 0x300000, size: 0x80000, crc: 0x2D1E9AE5 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.10b", offset: 0x380000, size: 0x80000, crc: 0x81145622 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dstlku.key", offset: 0x0, size: 0x14, crc: 0xC76091BA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLKUR1: RomSetDef = RomSetDef { name: "dstlkur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamu.03a", offset: 0x0, size: 0x80000, crc: 0x628899F9 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.04a", offset: 0x80000, size: 0x80000, crc: 0x696D9B25 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.05a", offset: 0x100000, size: 0x80000, crc: 0x673ED50A },
            Op::Load { kind: Load::Word16Swap, file: "vamu.06a", offset: 0x180000, size: 0x80000, crc: 0xF2377BE7 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.07a", offset: 0x200000, size: 0x80000, crc: 0xD8F498C4 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.08a", offset: 0x280000, size: 0x80000, crc: 0xE6A8A1A0 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.09a", offset: 0x300000, size: 0x80000, crc: 0x8DD55B24 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.10a", offset: 0x380000, size: 0x80000, crc: 0xC1A3D9BE },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dstlku.key", offset: 0x0, size: 0x14, crc: 0xC76091BA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VAMPJ: RomSetDef = RomSetDef { name: "vampj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamj.03a", offset: 0x0, size: 0x80000, crc: 0xF55D3722 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.04b", offset: 0x80000, size: 0x80000, crc: 0x4D9C43C4 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.05a", offset: 0x100000, size: 0x80000, crc: 0x6C497E92 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.06a", offset: 0x180000, size: 0x80000, crc: 0xF1BBECB6 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.07a", offset: 0x200000, size: 0x80000, crc: 0x1067AD84 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.08a", offset: 0x280000, size: 0x80000, crc: 0x4B89F41F },
            Op::Load { kind: Load::Word16Swap, file: "vamj.09a", offset: 0x300000, size: 0x80000, crc: 0xFC0A4AAC },
            Op::Load { kind: Load::Word16Swap, file: "vamj.10a", offset: 0x380000, size: 0x80000, crc: 0x9270C26B },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vampj.key", offset: 0x0, size: 0x14, crc: 0x8418CC6F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VAMPJA: RomSetDef = RomSetDef { name: "vampja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamj.03a", offset: 0x0, size: 0x80000, crc: 0xF55D3722 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.04a", offset: 0x80000, size: 0x80000, crc: 0xFDCBDAE3 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.05a", offset: 0x100000, size: 0x80000, crc: 0x6C497E92 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.06a", offset: 0x180000, size: 0x80000, crc: 0xF1BBECB6 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.07a", offset: 0x200000, size: 0x80000, crc: 0x1067AD84 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.08a", offset: 0x280000, size: 0x80000, crc: 0x4B89F41F },
            Op::Load { kind: Load::Word16Swap, file: "vamj.09a", offset: 0x300000, size: 0x80000, crc: 0xFC0A4AAC },
            Op::Load { kind: Load::Word16Swap, file: "vamj.10a", offset: 0x380000, size: 0x80000, crc: 0x9270C26B },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vampj.key", offset: 0x0, size: 0x14, crc: 0x8418CC6F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VAMPJR1: RomSetDef = RomSetDef { name: "vampjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamj.03", offset: 0x0, size: 0x80000, crc: 0x8895BF77 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.04", offset: 0x80000, size: 0x80000, crc: 0x5027DB3D },
            Op::Load { kind: Load::Word16Swap, file: "vamj.05", offset: 0x100000, size: 0x80000, crc: 0x97C66FDB },
            Op::Load { kind: Load::Word16Swap, file: "vamj.06", offset: 0x180000, size: 0x80000, crc: 0x9B4C3426 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.07", offset: 0x200000, size: 0x80000, crc: 0x303BC4FD },
            Op::Load { kind: Load::Word16Swap, file: "vamj.08", offset: 0x280000, size: 0x80000, crc: 0x3DEA3646 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.09", offset: 0x300000, size: 0x80000, crc: 0xC119A827 },
            Op::Load { kind: Load::Word16Swap, file: "vamj.10", offset: 0x380000, size: 0x80000, crc: 0x46593B79 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vampj.key", offset: 0x0, size: 0x14, crc: 0x8418CC6F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLKA: RomSetDef = RomSetDef { name: "dstlka", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vama.03a", offset: 0x0, size: 0x80000, crc: 0x294E0BEC },
            Op::Load { kind: Load::Word16Swap, file: "vama.04a", offset: 0x80000, size: 0x80000, crc: 0xBC18E128 },
            Op::Load { kind: Load::Word16Swap, file: "vama.05a", offset: 0x100000, size: 0x80000, crc: 0xE709FA59 },
            Op::Load { kind: Load::Word16Swap, file: "vama.06a", offset: 0x180000, size: 0x80000, crc: 0x55E4D387 },
            Op::Load { kind: Load::Word16Swap, file: "vama.07a", offset: 0x200000, size: 0x80000, crc: 0x24E8F981 },
            Op::Load { kind: Load::Word16Swap, file: "vama.08a", offset: 0x280000, size: 0x80000, crc: 0x743F3A8E },
            Op::Load { kind: Load::Word16Swap, file: "vama.09a", offset: 0x300000, size: 0x80000, crc: 0x67FA5573 },
            Op::Load { kind: Load::Word16Swap, file: "vama.10a", offset: 0x380000, size: 0x80000, crc: 0x5E03D747 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dstlka.key", offset: 0x0, size: 0x14, crc: 0xD31D61BC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLKH: RomSetDef = RomSetDef { name: "dstlkh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamh.03c", offset: 0x0, size: 0x80000, crc: 0x4D7B9E8F },
            Op::Load { kind: Load::Word16Swap, file: "vamh.04c", offset: 0x80000, size: 0x80000, crc: 0x2217E9A0 },
            Op::Load { kind: Load::Word16Swap, file: "vamh.05c", offset: 0x100000, size: 0x80000, crc: 0x3A05B13C },
            Op::Load { kind: Load::Word16Swap, file: "vamh.06c", offset: 0x180000, size: 0x80000, crc: 0x11D70A1C },
            Op::Load { kind: Load::Word16Swap, file: "vamh.07c", offset: 0x200000, size: 0x80000, crc: 0xDB5A8767 },
            Op::Load { kind: Load::Word16Swap, file: "vamh.08c", offset: 0x280000, size: 0x80000, crc: 0x2A4FD79B },
            Op::Load { kind: Load::Word16Swap, file: "vamh.09c", offset: 0x300000, size: 0x80000, crc: 0x15187632 },
            Op::Load { kind: Load::Word16Swap, file: "vamh.10c", offset: 0x380000, size: 0x80000, crc: 0x192D2D81 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dstlkh.key", offset: 0x0, size: 0x14, crc: 0xD748CB77 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLKB: RomSetDef = RomSetDef { name: "dstlkb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamb.03a", offset: 0x0, size: 0x80000, crc: 0x48831596 },
            Op::Load { kind: Load::Word16Swap, file: "vamb.04a", offset: 0x80000, size: 0x80000, crc: 0x2217E9A0 },
            Op::Load { kind: Load::Word16Swap, file: "vamb.05a", offset: 0x100000, size: 0x80000, crc: 0x3A05B13C },
            Op::Load { kind: Load::Word16Swap, file: "vamb.06a", offset: 0x180000, size: 0x80000, crc: 0x11D70A1C },
            Op::Load { kind: Load::Word16Swap, file: "vamb.07a", offset: 0x200000, size: 0x80000, crc: 0xDB5A8767 },
            Op::Load { kind: Load::Word16Swap, file: "vamb.08a", offset: 0x280000, size: 0x80000, crc: 0x2A4FD79B },
            Op::Load { kind: Load::Word16Swap, file: "vamb.09a", offset: 0x300000, size: 0x80000, crc: 0x15187632 },
            Op::Load { kind: Load::Word16Swap, file: "vamb.10a", offset: 0x380000, size: 0x80000, crc: 0x192D2D81 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dstlkb.key", offset: 0x0, size: 0x14, crc: 0xD748CB77 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ECOFGHTR: RomSetDef = RomSetDef { name: "ecofghtr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uece.03", offset: 0x0, size: 0x80000, crc: 0xEC2C1137 },
            Op::Load { kind: Load::Word16Swap, file: "uece.04", offset: 0x80000, size: 0x80000, crc: 0xB35F99DB },
            Op::Load { kind: Load::Word16Swap, file: "uece.05", offset: 0x100000, size: 0x80000, crc: 0xD9D42D31 },
            Op::Load { kind: Load::Word16Swap, file: "uece.06", offset: 0x180000, size: 0x80000, crc: 0x9D9771CF },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ecofghtr.key", offset: 0x0, size: 0x14, crc: 0x2250FD9E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ECOFGHTRU: RomSetDef = RomSetDef { name: "ecofghtru", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uecu.03a", offset: 0x0, size: 0x80000, crc: 0x22D88A4D },
            Op::Load { kind: Load::Word16Swap, file: "uecu.04a", offset: 0x80000, size: 0x80000, crc: 0x6436CFCD },
            Op::Load { kind: Load::Word16Swap, file: "uecu.05a", offset: 0x100000, size: 0x80000, crc: 0x336F121B },
            Op::Load { kind: Load::Word16Swap, file: "uecu.06a", offset: 0x180000, size: 0x80000, crc: 0x6F99D984 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ecofghtru.key", offset: 0x0, size: 0x14, crc: 0x611AA137 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ECOFGHTRU1: RomSetDef = RomSetDef { name: "ecofghtru1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uecu.03", offset: 0x0, size: 0x80000, crc: 0x6792480C },
            Op::Load { kind: Load::Word16Swap, file: "uecu.04", offset: 0x80000, size: 0x80000, crc: 0x95CE69D5 },
            Op::Load { kind: Load::Word16Swap, file: "uecu.05", offset: 0x100000, size: 0x80000, crc: 0x3A1E78AD },
            Op::Load { kind: Load::Word16Swap, file: "uecu.06", offset: 0x180000, size: 0x80000, crc: 0xA3E2F3CC },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ecofghtru.key", offset: 0x0, size: 0x14, crc: 0x611AA137 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_UECOLOGY: RomSetDef = RomSetDef { name: "uecology", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uecj.03", offset: 0x0, size: 0x80000, crc: 0x94C40A4C },
            Op::Load { kind: Load::Word16Swap, file: "uecj.04", offset: 0x80000, size: 0x80000, crc: 0x8D6E3A09 },
            Op::Load { kind: Load::Word16Swap, file: "uecj.05", offset: 0x100000, size: 0x80000, crc: 0x8604ECD7 },
            Op::Load { kind: Load::Word16Swap, file: "uecj.06", offset: 0x180000, size: 0x80000, crc: 0xB7E1D31F },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uecology.key", offset: 0x0, size: 0x14, crc: 0x0BAB792D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ECOFGHTRA: RomSetDef = RomSetDef { name: "ecofghtra", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ueca.03", offset: 0x0, size: 0x80000, crc: 0xBD4589B1 },
            Op::Load { kind: Load::Word16Swap, file: "ueca.04", offset: 0x80000, size: 0x80000, crc: 0x1D134B7D },
            Op::Load { kind: Load::Word16Swap, file: "ueca.05", offset: 0x100000, size: 0x80000, crc: 0x9C581FC7 },
            Op::Load { kind: Load::Word16Swap, file: "ueca.06", offset: 0x180000, size: 0x80000, crc: 0xC92A7C50 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ecofghtra.key", offset: 0x0, size: 0x14, crc: 0x4F99A9F5 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ECOFGHTRH: RomSetDef = RomSetDef { name: "ecofghtrh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uech.03", offset: 0x0, size: 0x80000, crc: 0x14C9365E },
            Op::Load { kind: Load::Word16Swap, file: "uech.04", offset: 0x80000, size: 0x80000, crc: 0x579495DC },
            Op::Load { kind: Load::Word16Swap, file: "uech.05", offset: 0x100000, size: 0x80000, crc: 0x96807A8E },
            Op::Load { kind: Load::Word16Swap, file: "uech.06", offset: 0x180000, size: 0x80000, crc: 0x682B9DBC },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ecofghtrh.key", offset: 0x0, size: 0x14, crc: 0x9A9027C8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_GIGAWING: RomSetDef = RomSetDef { name: "gigawing", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwu.03", offset: 0x0, size: 0x80000, crc: 0xAC725EB2 },
            Op::Load { kind: Load::Word16Swap, file: "ggwu.04", offset: 0x80000, size: 0x80000, crc: 0x392F4118 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.05", offset: 0x100000, size: 0x80000, crc: 0x3239D642 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gigawing.key", offset: 0x0, size: 0x14, crc: 0x5076C26B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_GIGAWINGJ: RomSetDef = RomSetDef { name: "gigawingj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwj.03a", offset: 0x0, size: 0x80000, crc: 0xFDD23B91 },
            Op::Load { kind: Load::Word16Swap, file: "ggwj.04a", offset: 0x80000, size: 0x80000, crc: 0x8C6E093C },
            Op::Load { kind: Load::Word16Swap, file: "ggwj.05a", offset: 0x100000, size: 0x80000, crc: 0x43811454 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gigawingj.key", offset: 0x0, size: 0x14, crc: 0x8121A25E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_GIGAWINGA: RomSetDef = RomSetDef { name: "gigawinga", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwa.03a", offset: 0x0, size: 0x80000, crc: 0x116F8837 },
            Op::Load { kind: Load::Word16Swap, file: "ggwa.04a", offset: 0x80000, size: 0x80000, crc: 0xE6E3F0C4 },
            Op::Load { kind: Load::Word16Swap, file: "ggwa.05a", offset: 0x100000, size: 0x80000, crc: 0x465E8AC9 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gigawinga.key", offset: 0x0, size: 0x14, crc: 0x7401627E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_GIGAWINGH: RomSetDef = RomSetDef { name: "gigawingh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwh.03a", offset: 0x0, size: 0x80000, crc: 0xB9EE36EB },
            Op::Load { kind: Load::Word16Swap, file: "ggwh.04a", offset: 0x80000, size: 0x80000, crc: 0x72E548FE },
            Op::Load { kind: Load::Word16Swap, file: "ggw.05", offset: 0x100000, size: 0x80000, crc: 0x3239D642 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gigawingh.key", offset: 0x0, size: 0x14, crc: 0x43198223 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_GIGAWINGB: RomSetDef = RomSetDef { name: "gigawingb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwb.03", offset: 0x0, size: 0x80000, crc: 0xA1F8A448 },
            Op::Load { kind: Load::Word16Swap, file: "ggwb.04", offset: 0x80000, size: 0x80000, crc: 0x6A423E76 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.05", offset: 0x100000, size: 0x80000, crc: 0x3239D642 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gigawingb.key", offset: 0x0, size: 0x14, crc: 0x5E7805FA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_HSF2: RomSetDef = RomSetDef { name: "hsf2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2u.03", offset: 0x0, size: 0x80000, crc: 0xB308151E },
            Op::Load { kind: Load::Word16Swap, file: "hs2u.04", offset: 0x80000, size: 0x80000, crc: 0x327AA49C },
            Op::Load { kind: Load::Word16Swap, file: "hs2.05", offset: 0x100000, size: 0x80000, crc: 0xDDE34A35 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.06", offset: 0x180000, size: 0x80000, crc: 0xF4E56DDA },
            Op::Load { kind: Load::Word16Swap, file: "hs2.07", offset: 0x200000, size: 0x80000, crc: 0xEE4420FC },
            Op::Load { kind: Load::Word16Swap, file: "hs2.08", offset: 0x280000, size: 0x80000, crc: 0xC9441533 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.09", offset: 0x300000, size: 0x80000, crc: 0x3FC638A8 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.10", offset: 0x380000, size: 0x80000, crc: 0x20D0F9E4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "hs2.13m", offset: 0x0, size: 0x800000, crc: 0xA6ECAB17 },
            Op::Load { kind: Load::Word64, file: "hs2.15m", offset: 0x2, size: 0x800000, crc: 0x10A0AE4D },
            Op::Load { kind: Load::Word64, file: "hs2.17m", offset: 0x4, size: 0x800000, crc: 0xADFA7726 },
            Op::Load { kind: Load::Word64, file: "hs2.19m", offset: 0x6, size: 0x800000, crc: 0xBB3AE322 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hs2.01", offset: 0x0, size: 0x8000, crc: 0xC1A13786 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "hs2.02", offset: 0x28000, size: 0x20000, crc: 0x2D8794AA },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2.11m", offset: 0x0, size: 0x800000, crc: 0x0E15C359 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hsf2.key", offset: 0x0, size: 0x14, crc: 0xFC9B18C9 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_HSF2A: RomSetDef = RomSetDef { name: "hsf2a", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2a.03", offset: 0x0, size: 0x80000, crc: 0xD50A17E0 },
            Op::Load { kind: Load::Word16Swap, file: "hs2a.04", offset: 0x80000, size: 0x80000, crc: 0xA27F42DE },
            Op::Load { kind: Load::Word16Swap, file: "hs2.05", offset: 0x100000, size: 0x80000, crc: 0xDDE34A35 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.06", offset: 0x180000, size: 0x80000, crc: 0xF4E56DDA },
            Op::Load { kind: Load::Word16Swap, file: "hs2.07", offset: 0x200000, size: 0x80000, crc: 0xEE4420FC },
            Op::Load { kind: Load::Word16Swap, file: "hs2.08", offset: 0x280000, size: 0x80000, crc: 0xC9441533 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.09", offset: 0x300000, size: 0x80000, crc: 0x3FC638A8 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.10", offset: 0x380000, size: 0x80000, crc: 0x20D0F9E4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "hs2.13m", offset: 0x0, size: 0x800000, crc: 0xA6ECAB17 },
            Op::Load { kind: Load::Word64, file: "hs2.15m", offset: 0x2, size: 0x800000, crc: 0x10A0AE4D },
            Op::Load { kind: Load::Word64, file: "hs2.17m", offset: 0x4, size: 0x800000, crc: 0xADFA7726 },
            Op::Load { kind: Load::Word64, file: "hs2.19m", offset: 0x6, size: 0x800000, crc: 0xBB3AE322 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hs2.01", offset: 0x0, size: 0x8000, crc: 0xC1A13786 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "hs2.02", offset: 0x28000, size: 0x20000, crc: 0x2D8794AA },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2.11m", offset: 0x0, size: 0x800000, crc: 0x0E15C359 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hsf2a.key", offset: 0x0, size: 0x14, crc: 0x2CD9EB99 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_HSF2J: RomSetDef = RomSetDef { name: "hsf2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2j.03c", offset: 0x0, size: 0x80000, crc: 0x6EFE661F },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.04b", offset: 0x80000, size: 0x80000, crc: 0x93F2500A },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.05", offset: 0x100000, size: 0x80000, crc: 0xDDE34A35 },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.06", offset: 0x180000, size: 0x80000, crc: 0xF4E56DDA },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.07", offset: 0x200000, size: 0x80000, crc: 0xEE4420FC },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.08", offset: 0x280000, size: 0x80000, crc: 0xC9441533 },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.09", offset: 0x300000, size: 0x80000, crc: 0x3FC638A8 },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.10", offset: 0x380000, size: 0x80000, crc: 0x20D0F9E4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "hs2.13m", offset: 0x0, size: 0x800000, crc: 0xA6ECAB17 },
            Op::Load { kind: Load::Word64, file: "hs2.15m", offset: 0x2, size: 0x800000, crc: 0x10A0AE4D },
            Op::Load { kind: Load::Word64, file: "hs2.17m", offset: 0x4, size: 0x800000, crc: 0xADFA7726 },
            Op::Load { kind: Load::Word64, file: "hs2.19m", offset: 0x6, size: 0x800000, crc: 0xBB3AE322 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hs2.01", offset: 0x0, size: 0x8000, crc: 0xC1A13786 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "hs2.02", offset: 0x28000, size: 0x20000, crc: 0x2D8794AA },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2.11m", offset: 0x0, size: 0x800000, crc: 0x0E15C359 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hsf2j.key", offset: 0x0, size: 0x14, crc: 0x19455A93 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_HSF2J1: RomSetDef = RomSetDef { name: "hsf2j1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2j.03a", offset: 0x0, size: 0x80000, crc: 0x76C7D980 },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.04a", offset: 0x80000, size: 0x80000, crc: 0x3D1F5D9B },
            Op::Load { kind: Load::Word16Swap, file: "hs2.05", offset: 0x100000, size: 0x80000, crc: 0xDDE34A35 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.06", offset: 0x180000, size: 0x80000, crc: 0xF4E56DDA },
            Op::Load { kind: Load::Word16Swap, file: "hs2.07", offset: 0x200000, size: 0x80000, crc: 0xEE4420FC },
            Op::Load { kind: Load::Word16Swap, file: "hs2.08", offset: 0x280000, size: 0x80000, crc: 0xC9441533 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.09", offset: 0x300000, size: 0x80000, crc: 0x3FC638A8 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.10", offset: 0x380000, size: 0x80000, crc: 0x20D0F9E4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "hs2.13m", offset: 0x0, size: 0x800000, crc: 0xA6ECAB17 },
            Op::Load { kind: Load::Word64, file: "hs2.15m", offset: 0x2, size: 0x800000, crc: 0x10A0AE4D },
            Op::Load { kind: Load::Word64, file: "hs2.17m", offset: 0x4, size: 0x800000, crc: 0xADFA7726 },
            Op::Load { kind: Load::Word64, file: "hs2.19m", offset: 0x6, size: 0x800000, crc: 0xBB3AE322 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hs2.01", offset: 0x0, size: 0x8000, crc: 0xC1A13786 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "hs2.02", offset: 0x28000, size: 0x20000, crc: 0x2D8794AA },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2.11m", offset: 0x0, size: 0x800000, crc: 0x0E15C359 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hsf2j.key", offset: 0x0, size: 0x14, crc: 0x19455A93 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_HSF2J2: RomSetDef = RomSetDef { name: "hsf2j2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2j.03", offset: 0x0, size: 0x80000, crc: 0x00738F73 },
            Op::Load { kind: Load::Word16Swap, file: "hs2j.04", offset: 0x80000, size: 0x80000, crc: 0x40072C4A },
            Op::Load { kind: Load::Word16Swap, file: "hs2.05", offset: 0x100000, size: 0x80000, crc: 0xDDE34A35 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.06", offset: 0x180000, size: 0x80000, crc: 0xF4E56DDA },
            Op::Load { kind: Load::Word16Swap, file: "hs2.07", offset: 0x200000, size: 0x80000, crc: 0xEE4420FC },
            Op::Load { kind: Load::Word16Swap, file: "hs2.08", offset: 0x280000, size: 0x80000, crc: 0xC9441533 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.09", offset: 0x300000, size: 0x80000, crc: 0x3FC638A8 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.10", offset: 0x380000, size: 0x80000, crc: 0x20D0F9E4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "hs2.13m", offset: 0x0, size: 0x800000, crc: 0xA6ECAB17 },
            Op::Load { kind: Load::Word64, file: "hs2.15m", offset: 0x2, size: 0x800000, crc: 0x10A0AE4D },
            Op::Load { kind: Load::Word64, file: "hs2.17m", offset: 0x4, size: 0x800000, crc: 0xADFA7726 },
            Op::Load { kind: Load::Word64, file: "hs2.19m", offset: 0x6, size: 0x800000, crc: 0xBB3AE322 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hs2.01", offset: 0x0, size: 0x8000, crc: 0xC1A13786 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "hs2.02", offset: 0x28000, size: 0x20000, crc: 0x2D8794AA },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2.11m", offset: 0x0, size: 0x800000, crc: 0x0E15C359 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hsf2j.key", offset: 0x0, size: 0x14, crc: 0x19455A93 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_JYANGOKU: RomSetDef = RomSetDef { name: "jyangoku", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "majj.03", offset: 0x0, size: 0x80000, crc: 0x4614A3B2 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "maj1_d.simm1", offset: 0x0, size: 0x200000, crc: 0xBA0FE27B },
            Op::Load { kind: Load::Byte64, file: "maj1_c.simm1", offset: 0x1, size: 0x200000, crc: 0x2CD141BF },
            Op::Load { kind: Load::Byte64, file: "maj1_b.simm1", offset: 0x2, size: 0x200000, crc: 0xE29E4C26 },
            Op::Load { kind: Load::Byte64, file: "maj1_a.simm1", offset: 0x3, size: 0x200000, crc: 0x7F68B88A },
            Op::Load { kind: Load::Byte64, file: "maj3_d.simm3", offset: 0x4, size: 0x200000, crc: 0x3AAEB90B },
            Op::Load { kind: Load::Byte64, file: "maj3_c.simm3", offset: 0x5, size: 0x200000, crc: 0x97894CEA },
            Op::Load { kind: Load::Byte64, file: "maj3_b.simm3", offset: 0x6, size: 0x200000, crc: 0xEC737D9D },
            Op::Load { kind: Load::Byte64, file: "maj3_a.simm3", offset: 0x7, size: 0x200000, crc: 0xC23B6F22 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "maj.01", offset: 0x0, size: 0x8000, crc: 0x1FE8C213 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "maj5_a.simm5", offset: 0x0, size: 0x200000, crc: 0x5AD9EE53 },
            Op::Load { kind: Load::Byte16, file: "maj5_b.simm5", offset: 0x1, size: 0x200000, crc: 0xEFB3DBFB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "jyangoku.key", offset: 0x0, size: 0x14, crc: 0x95B0A560 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MEGAMAN2: RomSetDef = RomSetDef { name: "megaman2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2u.03", offset: 0x0, size: 0x80000, crc: 0x8FFC2CD1 },
            Op::Load { kind: Load::Word16Swap, file: "rm2u.04", offset: 0x80000, size: 0x80000, crc: 0xBB30083A },
            Op::Load { kind: Load::Word16Swap, file: "rm2.05", offset: 0x100000, size: 0x80000, crc: 0x02EE9EFC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rm2.14m", offset: 0x800000, size: 0x200000, crc: 0x9B1F00B4 },
            Op::Load { kind: Load::Word64, file: "rm2.16m", offset: 0x800002, size: 0x200000, crc: 0xC2BB0C24 },
            Op::Load { kind: Load::Word64, file: "rm2.18m", offset: 0x800004, size: 0x200000, crc: 0x12257251 },
            Op::Load { kind: Load::Word64, file: "rm2.20m", offset: 0x800006, size: 0x200000, crc: 0xF9B6E786 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rm2.01a", offset: 0x0, size: 0x8000, crc: 0xD18E7859 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "rm2.02", offset: 0x28000, size: 0x20000, crc: 0xC463ECE0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2.11m", offset: 0x0, size: 0x200000, crc: 0x2106174D },
            Op::Load { kind: Load::Word16Swap, file: "rm2.12m", offset: 0x200000, size: 0x200000, crc: 0x546C1636 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "megaman2.key", offset: 0x0, size: 0x14, crc: 0x6828ED6D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MEGAMAN2A: RomSetDef = RomSetDef { name: "megaman2a", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2a.03", offset: 0x0, size: 0x80000, crc: 0x2B330CA7 },
            Op::Load { kind: Load::Word16Swap, file: "rm2a.04", offset: 0x80000, size: 0x80000, crc: 0x8B47942B },
            Op::Load { kind: Load::Word16Swap, file: "rm2.05", offset: 0x100000, size: 0x80000, crc: 0x02EE9EFC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rm2.14m", offset: 0x800000, size: 0x200000, crc: 0x9B1F00B4 },
            Op::Load { kind: Load::Word64, file: "rm2.16m", offset: 0x800002, size: 0x200000, crc: 0xC2BB0C24 },
            Op::Load { kind: Load::Word64, file: "rm2.18m", offset: 0x800004, size: 0x200000, crc: 0x12257251 },
            Op::Load { kind: Load::Word64, file: "rm2.20m", offset: 0x800006, size: 0x200000, crc: 0xF9B6E786 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rm2.01a", offset: 0x0, size: 0x8000, crc: 0xD18E7859 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "rm2.02", offset: 0x28000, size: 0x20000, crc: 0xC463ECE0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2.11m", offset: 0x0, size: 0x200000, crc: 0x2106174D },
            Op::Load { kind: Load::Word16Swap, file: "rm2.12m", offset: 0x200000, size: 0x200000, crc: 0x546C1636 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "megaman2a.key", offset: 0x0, size: 0x14, crc: 0xD6E8DCD7 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ROCKMAN2J: RomSetDef = RomSetDef { name: "rockman2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2j.03", offset: 0x0, size: 0x80000, crc: 0xDBAA1437 },
            Op::Load { kind: Load::Word16Swap, file: "rm2j.04", offset: 0x80000, size: 0x80000, crc: 0xCF5BA612 },
            Op::Load { kind: Load::Word16Swap, file: "rm2.05", offset: 0x100000, size: 0x80000, crc: 0x02EE9EFC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rm2.14m", offset: 0x800000, size: 0x200000, crc: 0x9B1F00B4 },
            Op::Load { kind: Load::Word64, file: "rm2.16m", offset: 0x800002, size: 0x200000, crc: 0xC2BB0C24 },
            Op::Load { kind: Load::Word64, file: "rm2.18m", offset: 0x800004, size: 0x200000, crc: 0x12257251 },
            Op::Load { kind: Load::Word64, file: "rm2.20m", offset: 0x800006, size: 0x200000, crc: 0xF9B6E786 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rm2.01a", offset: 0x0, size: 0x8000, crc: 0xD18E7859 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "rm2.02", offset: 0x28000, size: 0x20000, crc: 0xC463ECE0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2.11m", offset: 0x0, size: 0x200000, crc: 0x2106174D },
            Op::Load { kind: Load::Word16Swap, file: "rm2.12m", offset: 0x200000, size: 0x200000, crc: 0x546C1636 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rockman2j.key", offset: 0x0, size: 0x14, crc: 0xC590187A },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MEGAMAN2H: RomSetDef = RomSetDef { name: "megaman2h", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2h.03", offset: 0x0, size: 0x80000, crc: 0xBB180378 },
            Op::Load { kind: Load::Word16Swap, file: "rm2h.04", offset: 0x80000, size: 0x80000, crc: 0x205FFCD6 },
            Op::Load { kind: Load::Word16Swap, file: "rm2.05", offset: 0x100000, size: 0x80000, crc: 0x02EE9EFC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rm2.14m", offset: 0x800000, size: 0x200000, crc: 0x9B1F00B4 },
            Op::Load { kind: Load::Word64, file: "rm2.16m", offset: 0x800002, size: 0x200000, crc: 0xC2BB0C24 },
            Op::Load { kind: Load::Word64, file: "rm2.18m", offset: 0x800004, size: 0x200000, crc: 0x12257251 },
            Op::Load { kind: Load::Word64, file: "rm2.20m", offset: 0x800006, size: 0x200000, crc: 0xF9B6E786 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rm2.01a", offset: 0x0, size: 0x8000, crc: 0xD18E7859 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "rm2.02", offset: 0x28000, size: 0x20000, crc: 0xC463ECE0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2.11m", offset: 0x0, size: 0x200000, crc: 0x2106174D },
            Op::Load { kind: Load::Word16Swap, file: "rm2.12m", offset: 0x200000, size: 0x200000, crc: 0x546C1636 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "megaman2h.key", offset: 0x0, size: 0x14, crc: 0x99CB8D19 },
        ] },
] };
/// Régions ignorées : mcu, oki
pub const RS2_GIGAMAN2: RomSetDef = RomSetDef { name: "gigaman2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sys_rom1.bin", offset: 0x0, size: 0x400000, crc: 0x2EAA5E10 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cg_rom1.bin", offset: 0x0, size: 0x400000, crc: 0xED55A641 },
            Op::Ignore { size: 0x400000 },
            Op::Load { kind: Load::Byte, file: "cg_rom2.bin", offset: 0x400000, size: 0x400000, crc: 0x63918C05 },
            Op::Ignore { size: 0x400000 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMATRIX: RomSetDef = RomSetDef { name: "mmatrix", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmxu.03", offset: 0x0, size: 0x80000, crc: 0xAB65B599 },
            Op::Load { kind: Load::Word16Swap, file: "mmxu.04", offset: 0x80000, size: 0x80000, crc: 0x0135FC6C },
            Op::Load { kind: Load::Word16Swap, file: "mmxu.05", offset: 0x100000, size: 0x80000, crc: 0xF1FD2B84 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mmx.13m", offset: 0x0, size: 0x400000, crc: 0x04748718 },
            Op::Load { kind: Load::Word64, file: "mmx.15m", offset: 0x2, size: 0x400000, crc: 0x38074F44 },
            Op::Load { kind: Load::Word64, file: "mmx.17m", offset: 0x4, size: 0x400000, crc: 0xE4635E35 },
            Op::Load { kind: Load::Word64, file: "mmx.19m", offset: 0x6, size: 0x400000, crc: 0x4400A3F2 },
            Op::Load { kind: Load::Word64, file: "mmx.14m", offset: 0x1000000, size: 0x400000, crc: 0xD52BF491 },
            Op::Load { kind: Load::Word64, file: "mmx.16m", offset: 0x1000002, size: 0x400000, crc: 0x23F70780 },
            Op::Load { kind: Load::Word64, file: "mmx.18m", offset: 0x1000004, size: 0x400000, crc: 0x2562C9D5 },
            Op::Load { kind: Load::Word64, file: "mmx.20m", offset: 0x1000006, size: 0x400000, crc: 0x583A9687 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmx.01", offset: 0x0, size: 0x8000, crc: 0xC57E8171 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmx.11m", offset: 0x0, size: 0x400000, crc: 0x4180B39F },
            Op::Load { kind: Load::Word16Swap, file: "mmx.12m", offset: 0x400000, size: 0x400000, crc: 0x95E22A59 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmatrix.key", offset: 0x0, size: 0x14, crc: 0x8ED66BC4 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMATRIXA: RomSetDef = RomSetDef { name: "mmatrixa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmxa.03", offset: 0x0, size: 0x80000, crc: 0x1D670ED7 },
            Op::Load { kind: Load::Word16Swap, file: "mmxa.04", offset: 0x80000, size: 0x80000, crc: 0x43BE66D2 },
            Op::Load { kind: Load::Word16Swap, file: "mmxa.05", offset: 0x100000, size: 0x80000, crc: 0x9BFE3082 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mmx.13m", offset: 0x0, size: 0x400000, crc: 0x04748718 },
            Op::Load { kind: Load::Word64, file: "mmx.15m", offset: 0x2, size: 0x400000, crc: 0x38074F44 },
            Op::Load { kind: Load::Word64, file: "mmx.17m", offset: 0x4, size: 0x400000, crc: 0xE4635E35 },
            Op::Load { kind: Load::Word64, file: "mmx.19m", offset: 0x6, size: 0x400000, crc: 0x4400A3F2 },
            Op::Load { kind: Load::Word64, file: "mmx.14m", offset: 0x1000000, size: 0x400000, crc: 0xD52BF491 },
            Op::Load { kind: Load::Word64, file: "mmx.16m", offset: 0x1000002, size: 0x400000, crc: 0x23F70780 },
            Op::Load { kind: Load::Word64, file: "mmx.18m", offset: 0x1000004, size: 0x400000, crc: 0x2562C9D5 },
            Op::Load { kind: Load::Word64, file: "mmx.20m", offset: 0x1000006, size: 0x400000, crc: 0x583A9687 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmx.01", offset: 0x0, size: 0x8000, crc: 0xC57E8171 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmx.11m", offset: 0x0, size: 0x400000, crc: 0x4180B39F },
            Op::Load { kind: Load::Word16Swap, file: "mmx.12m", offset: 0x400000, size: 0x400000, crc: 0x95E22A59 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmatrixa.key", offset: 0x0, size: 0x14, crc: 0xF6A944B4 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMATRIXJ: RomSetDef = RomSetDef { name: "mmatrixj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmxj.03", offset: 0x0, size: 0x80000, crc: 0x1D5DE213 },
            Op::Load { kind: Load::Word16Swap, file: "mmxj.04", offset: 0x80000, size: 0x80000, crc: 0xD943A339 },
            Op::Load { kind: Load::Word16Swap, file: "mmxj.05", offset: 0x100000, size: 0x80000, crc: 0x0C8B4ABB },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mmx.13m", offset: 0x0, size: 0x400000, crc: 0x04748718 },
            Op::Load { kind: Load::Word64, file: "mmx.15m", offset: 0x2, size: 0x400000, crc: 0x38074F44 },
            Op::Load { kind: Load::Word64, file: "mmx.17m", offset: 0x4, size: 0x400000, crc: 0xE4635E35 },
            Op::Load { kind: Load::Word64, file: "mmx.19m", offset: 0x6, size: 0x400000, crc: 0x4400A3F2 },
            Op::Load { kind: Load::Word64, file: "mmx.14m", offset: 0x1000000, size: 0x400000, crc: 0xD52BF491 },
            Op::Load { kind: Load::Word64, file: "mmx.16m", offset: 0x1000002, size: 0x400000, crc: 0x23F70780 },
            Op::Load { kind: Load::Word64, file: "mmx.18m", offset: 0x1000004, size: 0x400000, crc: 0x2562C9D5 },
            Op::Load { kind: Load::Word64, file: "mmx.20m", offset: 0x1000006, size: 0x400000, crc: 0x583A9687 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmx.01", offset: 0x0, size: 0x8000, crc: 0xC57E8171 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmx.11m", offset: 0x0, size: 0x400000, crc: 0x4180B39F },
            Op::Load { kind: Load::Word16Swap, file: "mmx.12m", offset: 0x400000, size: 0x400000, crc: 0x95E22A59 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmatrixj.key", offset: 0x0, size: 0x14, crc: 0x3B50D889 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSH: RomSetDef = RomSetDef { name: "msh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshe.03e", offset: 0x0, size: 0x80000, crc: 0xBD951414 },
            Op::Load { kind: Load::Word16Swap, file: "mshe.04e", offset: 0x80000, size: 0x80000, crc: 0x19DD42F2 },
            Op::Load { kind: Load::Word16Swap, file: "msh.05", offset: 0x100000, size: 0x80000, crc: 0x6A091B9E },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.key", offset: 0x0, size: 0x14, crc: 0xB494368E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHU: RomSetDef = RomSetDef { name: "mshu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshu.03", offset: 0x0, size: 0x80000, crc: 0xD2805BDD },
            Op::Load { kind: Load::Word16Swap, file: "mshu.04", offset: 0x80000, size: 0x80000, crc: 0x743F96FF },
            Op::Load { kind: Load::Word16Swap, file: "msh.05", offset: 0x100000, size: 0x80000, crc: 0x6A091B9E },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshu.key", offset: 0x0, size: 0x14, crc: 0x745C1BEE },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHJ: RomSetDef = RomSetDef { name: "mshj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshj.03g", offset: 0x0, size: 0x80000, crc: 0x261F4091 },
            Op::Load { kind: Load::Word16Swap, file: "mshj.04g", offset: 0x80000, size: 0x80000, crc: 0x61D791C6 },
            Op::Load { kind: Load::Word16Swap, file: "msh.05a", offset: 0x100000, size: 0x80000, crc: 0xF37539E6 },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshj.key", offset: 0x0, size: 0x14, crc: 0x888761AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHJR1: RomSetDef = RomSetDef { name: "mshjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshj.03f", offset: 0x0, size: 0x80000, crc: 0xFF172FD2 },
            Op::Load { kind: Load::Word16Swap, file: "mshj.04f", offset: 0x80000, size: 0x80000, crc: 0xEBBB205A },
            Op::Load { kind: Load::Word16Swap, file: "msh.05", offset: 0x100000, size: 0x80000, crc: 0x6A091B9E },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshj.key", offset: 0x0, size: 0x14, crc: 0x888761AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHA: RomSetDef = RomSetDef { name: "msha", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msha.03e", offset: 0x0, size: 0x80000, crc: 0xEC84EC44 },
            Op::Load { kind: Load::Word16Swap, file: "msha.04e", offset: 0x80000, size: 0x80000, crc: 0x098B8503 },
            Op::Load { kind: Load::Word16Swap, file: "msh.05", offset: 0x100000, size: 0x80000, crc: 0x6A091B9E },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msha.key", offset: 0x0, size: 0x14, crc: 0x00F3F2CA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHH: RomSetDef = RomSetDef { name: "mshh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshh.03c", offset: 0x0, size: 0x80000, crc: 0x8D84B0FA },
            Op::Load { kind: Load::Word16Swap, file: "mshh.04c", offset: 0x80000, size: 0x80000, crc: 0xD638F601 },
            Op::Load { kind: Load::Word16Swap, file: "msh.05a", offset: 0x100000, size: 0x80000, crc: 0xF37539E6 },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshh.key", offset: 0x0, size: 0x14, crc: 0x5DDDF5E7 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHB: RomSetDef = RomSetDef { name: "mshb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshb.03c", offset: 0x0, size: 0x80000, crc: 0x19697F74 },
            Op::Load { kind: Load::Word16Swap, file: "mshb.04c", offset: 0x80000, size: 0x80000, crc: 0x95317A6F },
            Op::Load { kind: Load::Word16Swap, file: "msh.05a", offset: 0x100000, size: 0x80000, crc: 0xF37539E6 },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshb.key", offset: 0x0, size: 0x14, crc: 0x92196837 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHBR1: RomSetDef = RomSetDef { name: "mshbr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshb.03b", offset: 0x0, size: 0x80000, crc: 0xEB16165B },
            Op::Load { kind: Load::Word16Swap, file: "mshb.04b", offset: 0x80000, size: 0x80000, crc: 0x94FB3C97 },
            Op::Load { kind: Load::Word16Swap, file: "msh.05", offset: 0x100000, size: 0x80000, crc: 0x6A091B9E },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshb.key", offset: 0x0, size: 0x14, crc: 0x92196837 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSF: RomSetDef = RomSetDef { name: "mshvsf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvse.03f", offset: 0x0, size: 0x80000, crc: 0xB72DC199 },
            Op::Load { kind: Load::Word16Swap, file: "mvse.04f", offset: 0x80000, size: 0x80000, crc: 0x6EF799F9 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05a", offset: 0x100000, size: 0x80000, crc: 0x1A5DE0CB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsf.key", offset: 0x0, size: 0x14, crc: 0x64660867 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFU: RomSetDef = RomSetDef { name: "mshvsfu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsu.03g", offset: 0x0, size: 0x80000, crc: 0x0664AB15 },
            Op::Load { kind: Load::Word16Swap, file: "mvsu.04g", offset: 0x80000, size: 0x80000, crc: 0x97E060EE },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05d", offset: 0x100000, size: 0x80000, crc: 0x921FC542 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfu.key", offset: 0x0, size: 0x14, crc: 0x4C04797B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFU1: RomSetDef = RomSetDef { name: "mshvsfu1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsu.03d", offset: 0x0, size: 0x80000, crc: 0xAE60A66A },
            Op::Load { kind: Load::Word16Swap, file: "mvsu.04d", offset: 0x80000, size: 0x80000, crc: 0x91F67D8A },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05a", offset: 0x100000, size: 0x80000, crc: 0x1A5DE0CB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfu.key", offset: 0x0, size: 0x14, crc: 0x4C04797B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFJ: RomSetDef = RomSetDef { name: "mshvsfj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsj.03i", offset: 0x0, size: 0x80000, crc: 0xD8CBB691 },
            Op::Load { kind: Load::Word16Swap, file: "mvsj.04i", offset: 0x80000, size: 0x80000, crc: 0x32741ACE },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05h", offset: 0x100000, size: 0x80000, crc: 0x77870DC3 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfj.key", offset: 0x0, size: 0x14, crc: 0x565EEEBB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFJ1: RomSetDef = RomSetDef { name: "mshvsfj1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsj.03h", offset: 0x0, size: 0x80000, crc: 0xFBE2115F },
            Op::Load { kind: Load::Word16Swap, file: "mvsj.04h", offset: 0x80000, size: 0x80000, crc: 0xB528A367 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05g", offset: 0x100000, size: 0x80000, crc: 0x9515A245 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfj.key", offset: 0x0, size: 0x14, crc: 0x565EEEBB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFJ2: RomSetDef = RomSetDef { name: "mshvsfj2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsj.03g", offset: 0x0, size: 0x80000, crc: 0xFDFA7E26 },
            Op::Load { kind: Load::Word16Swap, file: "mvsj.04g", offset: 0x80000, size: 0x80000, crc: 0xC921825F },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05a", offset: 0x100000, size: 0x80000, crc: 0x1A5DE0CB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfj.key", offset: 0x0, size: 0x14, crc: 0x565EEEBB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFH: RomSetDef = RomSetDef { name: "mshvsfh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsh.03f", offset: 0x0, size: 0x80000, crc: 0x4F60F41E },
            Op::Load { kind: Load::Word16Swap, file: "mvsh.04f", offset: 0x80000, size: 0x80000, crc: 0xDC08EC12 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05a", offset: 0x100000, size: 0x80000, crc: 0x1A5DE0CB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfh.key", offset: 0x0, size: 0x14, crc: 0xB93D576F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFA: RomSetDef = RomSetDef { name: "mshvsfa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsa.03f", offset: 0x0, size: 0x80000, crc: 0x5B863716 },
            Op::Load { kind: Load::Word16Swap, file: "mvsa.04f", offset: 0x80000, size: 0x80000, crc: 0x4886E65F },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05a", offset: 0x100000, size: 0x80000, crc: 0x1A5DE0CB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfa.key", offset: 0x0, size: 0x14, crc: 0x6810A3AF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFA1: RomSetDef = RomSetDef { name: "mshvsfa1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsa.03", offset: 0x0, size: 0x80000, crc: 0x92EF1933 },
            Op::Load { kind: Load::Word16Swap, file: "mvsa.04", offset: 0x80000, size: 0x80000, crc: 0x4B24373C },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05", offset: 0x100000, size: 0x80000, crc: 0xAC180C1C },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfa.key", offset: 0x0, size: 0x14, crc: 0x6810A3AF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFB: RomSetDef = RomSetDef { name: "mshvsfb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsb.03g", offset: 0x0, size: 0x80000, crc: 0x143895EF },
            Op::Load { kind: Load::Word16Swap, file: "mvsb.04g", offset: 0x80000, size: 0x80000, crc: 0xDD8A886C },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05d", offset: 0x100000, size: 0x80000, crc: 0x921FC542 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfb.key", offset: 0x0, size: 0x14, crc: 0x3F5BB6E4 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFB1: RomSetDef = RomSetDef { name: "mshvsfb1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsb.03f", offset: 0x0, size: 0x80000, crc: 0x9C4BB950 },
            Op::Load { kind: Load::Word16Swap, file: "mvsb.04f", offset: 0x80000, size: 0x80000, crc: 0xD3320D13 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.05a", offset: 0x100000, size: 0x80000, crc: 0x1A5DE0CB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mshvsfb.key", offset: 0x0, size: 0x14, crc: 0x3F5BB6E4 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSC: RomSetDef = RomSetDef { name: "mvsc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvce.03a", offset: 0x0, size: 0x80000, crc: 0x824E4A90 },
            Op::Load { kind: Load::Word16Swap, file: "mvce.04a", offset: 0x80000, size: 0x80000, crc: 0x436C5A4E },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvsc.key", offset: 0x0, size: 0x14, crc: 0x7E101E09 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCR1: RomSetDef = RomSetDef { name: "mvscr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvce.03", offset: 0x0, size: 0x80000, crc: 0xE0633FC0 },
            Op::Load { kind: Load::Word16Swap, file: "mvce.04", offset: 0x80000, size: 0x80000, crc: 0xA450A251 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05", offset: 0x100000, size: 0x80000, crc: 0x7DB71CE9 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06", offset: 0x180000, size: 0x80000, crc: 0x4B0B6D3E },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvsc.key", offset: 0x0, size: 0x14, crc: 0x7E101E09 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCU: RomSetDef = RomSetDef { name: "mvscu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvcu.03d", offset: 0x0, size: 0x80000, crc: 0xC6007557 },
            Op::Load { kind: Load::Word16Swap, file: "mvcu.04d", offset: 0x80000, size: 0x80000, crc: 0x724B2B20 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvscu.key", offset: 0x0, size: 0x14, crc: 0xA83DB333 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCUR1: RomSetDef = RomSetDef { name: "mvscur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvcu.03", offset: 0x0, size: 0x80000, crc: 0x13F2BE57 },
            Op::Load { kind: Load::Word16Swap, file: "mvcu.04", offset: 0x80000, size: 0x80000, crc: 0x5E9B380D },
            Op::Load { kind: Load::Word16Swap, file: "mvcu.05", offset: 0x100000, size: 0x80000, crc: 0x12F321BE },
            Op::Load { kind: Load::Word16Swap, file: "mvcu.06", offset: 0x180000, size: 0x80000, crc: 0x2F1524BC },
            Op::Load { kind: Load::Word16Swap, file: "mvcu.07", offset: 0x200000, size: 0x80000, crc: 0x5FDECADB },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvcu.10", offset: 0x380000, size: 0x80000, crc: 0x4F36CD63 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvscu.key", offset: 0x0, size: 0x14, crc: 0xA83DB333 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCJ: RomSetDef = RomSetDef { name: "mvscj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvcj.03a", offset: 0x0, size: 0x80000, crc: 0x3DF18879 },
            Op::Load { kind: Load::Word16Swap, file: "mvcj.04a", offset: 0x80000, size: 0x80000, crc: 0x07D212E8 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvscj.key", offset: 0x0, size: 0x14, crc: 0x9DEDBCAF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCJR1: RomSetDef = RomSetDef { name: "mvscjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvcj.03", offset: 0x0, size: 0x80000, crc: 0x2164213F },
            Op::Load { kind: Load::Word16Swap, file: "mvcj.04", offset: 0x80000, size: 0x80000, crc: 0xC905C86F },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05", offset: 0x100000, size: 0x80000, crc: 0x7DB71CE9 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06", offset: 0x180000, size: 0x80000, crc: 0x4B0B6D3E },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvscj.key", offset: 0x0, size: 0x14, crc: 0x9DEDBCAF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCJSING: RomSetDef = RomSetDef { name: "mvscjsing", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "mvc_ja.simm1", offset: 0x0, size: 0x200000, crc: 0x6A2EF7C2 },
            Op::Load { kind: Load::Byte16, file: "mvc_ja.simm3", offset: 0x1, size: 0x200000, crc: 0x699D09AD },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc64-13m.13", offset: 0x0, size: 0x800000, crc: 0x8428CE69 },
            Op::Load { kind: Load::Word64, file: "mvc64-15m.15", offset: 0x2, size: 0x800000, crc: 0x2E0028F4 },
            Op::Load { kind: Load::Word64, file: "mvc64-17m.17", offset: 0x4, size: 0x800000, crc: 0x308CA826 },
            Op::Load { kind: Load::Word64, file: "mvc64-19m.19", offset: 0x6, size: 0x800000, crc: 0x10699FE1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc64-11m.11", offset: 0x0, size: 0x800000, crc: 0x5D8819E0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvscj.key", offset: 0x0, size: 0x14, crc: 0x9DEDBCAF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCA: RomSetDef = RomSetDef { name: "mvsca", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvca.03a", offset: 0x0, size: 0x80000, crc: 0x2FF4AE25 },
            Op::Load { kind: Load::Word16Swap, file: "mvca.04a", offset: 0x80000, size: 0x80000, crc: 0xF28427EF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvsca.key", offset: 0x0, size: 0x14, crc: 0x31EDAEE8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCAR1: RomSetDef = RomSetDef { name: "mvscar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvca.03", offset: 0x0, size: 0x80000, crc: 0xFE5FA7B9 },
            Op::Load { kind: Load::Word16Swap, file: "mvca.04", offset: 0x80000, size: 0x80000, crc: 0x082B701C },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05", offset: 0x100000, size: 0x80000, crc: 0x7DB71CE9 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06", offset: 0x180000, size: 0x80000, crc: 0x4B0B6D3E },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvsca.key", offset: 0x0, size: 0x14, crc: 0x31EDAEE8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCH: RomSetDef = RomSetDef { name: "mvsch", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvch.03", offset: 0x0, size: 0x80000, crc: 0x6A0EC9F7 },
            Op::Load { kind: Load::Word16Swap, file: "mvch.04", offset: 0x80000, size: 0x80000, crc: 0x00F03FA4 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvsch.key", offset: 0x0, size: 0x14, crc: 0xDD647C0D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MVSCB: RomSetDef = RomSetDef { name: "mvscb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvcb.03a", offset: 0x0, size: 0x80000, crc: 0x7155953B },
            Op::Load { kind: Load::Word16Swap, file: "mvcb.04a", offset: 0x80000, size: 0x80000, crc: 0xFB117D0E },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.07", offset: 0x200000, size: 0x80000, crc: 0xC3BAA32B },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvscb.key", offset: 0x0, size: 0x14, crc: 0xD74A7A3D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MPANG: RomSetDef = RomSetDef { name: "mpang", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpne.03c", offset: 0x0, size: 0x80000, crc: 0xFE16FC9F },
            Op::Load { kind: Load::Word16Swap, file: "mpne.04c", offset: 0x80000, size: 0x80000, crc: 0x2CC5EC22 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01c", offset: 0x0, size: 0x200000, crc: 0x388DB66B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01d", offset: 0x1, size: 0x200000, crc: 0xAFF1B494 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01a", offset: 0x2, size: 0x200000, crc: 0xA9C4857B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01b", offset: 0x3, size: 0x200000, crc: 0xF759DF22 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03c", offset: 0x4, size: 0x200000, crc: 0xDEC6B720 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03d", offset: 0x5, size: 0x200000, crc: 0xF8774C18 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03a", offset: 0x6, size: 0x200000, crc: 0xC2AEA4EC },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03b", offset: 0x7, size: 0x200000, crc: 0x84D6DC33 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpn.01", offset: 0x0, size: 0x8000, crc: 0x90C7ADB6 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05a", offset: 0x0, size: 0x200000, crc: 0x318A2E21 },
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x5462F4E8 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpang.key", offset: 0x0, size: 0x14, crc: 0x95354B0F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MPANGR1: RomSetDef = RomSetDef { name: "mpangr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpne.03b", offset: 0x0, size: 0x80000, crc: 0x6EF0F9B2 },
            Op::Load { kind: Load::Word16Swap, file: "mpne.04b", offset: 0x80000, size: 0x80000, crc: 0x30A468BB },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01c", offset: 0x0, size: 0x200000, crc: 0x388DB66B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01d", offset: 0x1, size: 0x200000, crc: 0xAFF1B494 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01a", offset: 0x2, size: 0x200000, crc: 0xA9C4857B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01b", offset: 0x3, size: 0x200000, crc: 0xF759DF22 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03c", offset: 0x4, size: 0x200000, crc: 0xDEC6B720 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03d", offset: 0x5, size: 0x200000, crc: 0xF8774C18 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03a", offset: 0x6, size: 0x200000, crc: 0xC2AEA4EC },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03b", offset: 0x7, size: 0x200000, crc: 0x84D6DC33 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpn.01", offset: 0x0, size: 0x8000, crc: 0x90C7ADB6 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05a", offset: 0x0, size: 0x200000, crc: 0x318A2E21 },
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x5462F4E8 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpang.key", offset: 0x0, size: 0x14, crc: 0x95354B0F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MPANGU: RomSetDef = RomSetDef { name: "mpangu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpnu.03", offset: 0x0, size: 0x80000, crc: 0x6E7ED03C },
            Op::Load { kind: Load::Word16Swap, file: "mpnu.04", offset: 0x80000, size: 0x80000, crc: 0xDE079131 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "mpn.13m", offset: 0x800000, size: 0x200000, crc: 0xC5F123DC },
            Op::Load { kind: Load::Word64, file: "mpn.15m", offset: 0x800002, size: 0x200000, crc: 0x8E033265 },
            Op::Load { kind: Load::Word64, file: "mpn.17m", offset: 0x800004, size: 0x200000, crc: 0xCFCD73D2 },
            Op::Load { kind: Load::Word64, file: "mpn.19m", offset: 0x800006, size: 0x200000, crc: 0x2DB1FFBC },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpn.01", offset: 0x0, size: 0x8000, crc: 0x90C7ADB6 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpn.q1", offset: 0x0, size: 0x100000, crc: 0xD21C1F5A },
            Op::Load { kind: Load::Byte, file: "mpn.q2", offset: 0x100000, size: 0x100000, crc: 0xD22090B1 },
            Op::Load { kind: Load::Byte, file: "mpn.q3", offset: 0x200000, size: 0x100000, crc: 0x60AA5EF2 },
            Op::Load { kind: Load::Byte, file: "mpn.q4", offset: 0x300000, size: 0x100000, crc: 0x3A67D203 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpang.key", offset: 0x0, size: 0x14, crc: 0x95354B0F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MPANGJ: RomSetDef = RomSetDef { name: "mpangj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpnj.03a", offset: 0x0, size: 0x80000, crc: 0xBF597B1C },
            Op::Load { kind: Load::Word16Swap, file: "mpnj.04a", offset: 0x80000, size: 0x80000, crc: 0xF4A3AB0F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01c", offset: 0x0, size: 0x200000, crc: 0x388DB66B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01d", offset: 0x1, size: 0x200000, crc: 0xAFF1B494 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01a", offset: 0x2, size: 0x200000, crc: 0xA9C4857B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01b", offset: 0x3, size: 0x200000, crc: 0xF759DF22 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03c", offset: 0x4, size: 0x200000, crc: 0xDEC6B720 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03d", offset: 0x5, size: 0x200000, crc: 0xF8774C18 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03a", offset: 0x6, size: 0x200000, crc: 0xC2AEA4EC },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03b", offset: 0x7, size: 0x200000, crc: 0x84D6DC33 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpn.01", offset: 0x0, size: 0x8000, crc: 0x90C7ADB6 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05a", offset: 0x0, size: 0x200000, crc: 0x318A2E21 },
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x5462F4E8 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpang.key", offset: 0x0, size: 0x14, crc: 0x95354B0F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MPANGA: RomSetDef = RomSetDef { name: "mpanga", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpn_03a.6a", offset: 0x0, size: 0x80000, crc: 0xB05EBC34 },
            Op::Load { kind: Load::Word16Swap, file: "mpn_04a.7a", offset: 0x80000, size: 0x80000, crc: 0x8E4814B4 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01c", offset: 0x0, size: 0x200000, crc: 0x388DB66B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01d", offset: 0x1, size: 0x200000, crc: 0xAFF1B494 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01a", offset: 0x2, size: 0x200000, crc: 0xA9C4857B },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.01b", offset: 0x3, size: 0x200000, crc: 0xF759DF22 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03c", offset: 0x4, size: 0x200000, crc: 0xDEC6B720 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03d", offset: 0x5, size: 0x200000, crc: 0xF8774C18 },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03a", offset: 0x6, size: 0x200000, crc: 0xC2AEA4EC },
            Op::Load { kind: Load::Byte64, file: "mpn-simm.03b", offset: 0x7, size: 0x200000, crc: 0x84D6DC33 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpn.01", offset: 0x0, size: 0x8000, crc: 0x90C7ADB6 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05a", offset: 0x0, size: 0x200000, crc: 0x318A2E21 },
            Op::Load { kind: Load::Word16Swap, file: "mpn-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x5462F4E8 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpang.key", offset: 0x0, size: 0x14, crc: 0x95354B0F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_NWARR: RomSetDef = RomSetDef { name: "nwarr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphe.03f", offset: 0x0, size: 0x80000, crc: 0xA922C44F },
            Op::Load { kind: Load::Word16Swap, file: "vphe.04c", offset: 0x80000, size: 0x80000, crc: 0x7312D890 },
            Op::Load { kind: Load::Word16Swap, file: "vphe.05d", offset: 0x100000, size: 0x80000, crc: 0xCDE8B506 },
            Op::Load { kind: Load::Word16Swap, file: "vphe.06c", offset: 0x180000, size: 0x80000, crc: 0xBE99E7D0 },
            Op::Load { kind: Load::Word16Swap, file: "vphe.07b", offset: 0x200000, size: 0x80000, crc: 0x69E0E60C },
            Op::Load { kind: Load::Word16Swap, file: "vphe.08b", offset: 0x280000, size: 0x80000, crc: 0xD95A3849 },
            Op::Load { kind: Load::Word16Swap, file: "vphe.09b", offset: 0x300000, size: 0x80000, crc: 0x9882561C },
            Op::Load { kind: Load::Word16Swap, file: "vphe.10b", offset: 0x380000, size: 0x80000, crc: 0x976FA62F },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nwarr.key", offset: 0x0, size: 0x14, crc: 0x618A13CA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_NWARRU: RomSetDef = RomSetDef { name: "nwarru", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphu.03f", offset: 0x0, size: 0x80000, crc: 0x85D6A359 },
            Op::Load { kind: Load::Word16Swap, file: "vphu.04c", offset: 0x80000, size: 0x80000, crc: 0xCB7FCE77 },
            Op::Load { kind: Load::Word16Swap, file: "vphu.05e", offset: 0x100000, size: 0x80000, crc: 0xE08F2BBA },
            Op::Load { kind: Load::Word16Swap, file: "vphu.06c", offset: 0x180000, size: 0x80000, crc: 0x08C04CDB },
            Op::Load { kind: Load::Word16Swap, file: "vphu.07b", offset: 0x200000, size: 0x80000, crc: 0xB5A5AB19 },
            Op::Load { kind: Load::Word16Swap, file: "vphu.08b", offset: 0x280000, size: 0x80000, crc: 0x51BB20FB },
            Op::Load { kind: Load::Word16Swap, file: "vphu.09b", offset: 0x300000, size: 0x80000, crc: 0x41A64205 },
            Op::Load { kind: Load::Word16Swap, file: "vphu.10b", offset: 0x380000, size: 0x80000, crc: 0x2B1D43AE },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nwarru.key", offset: 0x0, size: 0x14, crc: 0x1C593F9B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_NWARRH: RomSetDef = RomSetDef { name: "nwarrh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphh.03d", offset: 0x0, size: 0x80000, crc: 0x6029C7BE },
            Op::Load { kind: Load::Word16Swap, file: "vphh.04a", offset: 0x80000, size: 0x80000, crc: 0xD26625EE },
            Op::Load { kind: Load::Word16Swap, file: "vphh.05c", offset: 0x100000, size: 0x80000, crc: 0x73EE0B8A },
            Op::Load { kind: Load::Word16Swap, file: "vphh.06a", offset: 0x180000, size: 0x80000, crc: 0xA5B3A50A },
            Op::Load { kind: Load::Word16Swap, file: "vphh.07", offset: 0x200000, size: 0x80000, crc: 0x5FC2BDC1 },
            Op::Load { kind: Load::Word16Swap, file: "vphh.08", offset: 0x280000, size: 0x80000, crc: 0xE65588D9 },
            Op::Load { kind: Load::Word16Swap, file: "vphh.09", offset: 0x300000, size: 0x80000, crc: 0xA2CE6D63 },
            Op::Load { kind: Load::Word16Swap, file: "vphh.10", offset: 0x380000, size: 0x80000, crc: 0xE2F4F4B9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nwarrh.key", offset: 0x0, size: 0x14, crc: 0x5FB16B23 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_NWARRB: RomSetDef = RomSetDef { name: "nwarrb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphb.03d", offset: 0x0, size: 0x80000, crc: 0x3A426D3F },
            Op::Load { kind: Load::Word16Swap, file: "vphb.04a", offset: 0x80000, size: 0x80000, crc: 0x51C4BB2F },
            Op::Load { kind: Load::Word16Swap, file: "vphb.05c", offset: 0x100000, size: 0x80000, crc: 0xAC44D997 },
            Op::Load { kind: Load::Word16Swap, file: "vphb.06a", offset: 0x180000, size: 0x80000, crc: 0x5072A5FE },
            Op::Load { kind: Load::Word16Swap, file: "vphb.07", offset: 0x200000, size: 0x80000, crc: 0x9B355192 },
            Op::Load { kind: Load::Word16Swap, file: "vphb.08", offset: 0x280000, size: 0x80000, crc: 0x42220F84 },
            Op::Load { kind: Load::Word16Swap, file: "vphb.09", offset: 0x300000, size: 0x80000, crc: 0x029E015D },
            Op::Load { kind: Load::Word16Swap, file: "vphb.10", offset: 0x380000, size: 0x80000, crc: 0x37B3CE37 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nwarrb.key", offset: 0x0, size: 0x14, crc: 0x4FFC0A54 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_NWARRA: RomSetDef = RomSetDef { name: "nwarra", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vpha.03b", offset: 0x0, size: 0x80000, crc: 0x0A70CDD6 },
            Op::Load { kind: Load::Word16Swap, file: "vpha.04b", offset: 0x80000, size: 0x80000, crc: 0x70CE62E4 },
            Op::Load { kind: Load::Word16Swap, file: "vpha.05b", offset: 0x100000, size: 0x80000, crc: 0x5692A03F },
            Op::Load { kind: Load::Word16Swap, file: "vpha.06b", offset: 0x180000, size: 0x80000, crc: 0xB810FE66 },
            Op::Load { kind: Load::Word16Swap, file: "vpha.07b", offset: 0x200000, size: 0x80000, crc: 0x1BE264F3 },
            Op::Load { kind: Load::Word16Swap, file: "vpha.08b", offset: 0x280000, size: 0x80000, crc: 0x86F1ED52 },
            Op::Load { kind: Load::Word16Swap, file: "vpha.09b", offset: 0x300000, size: 0x80000, crc: 0x7E96BD0A },
            Op::Load { kind: Load::Word16Swap, file: "vpha.10b", offset: 0x380000, size: 0x80000, crc: 0x58BCE2FD },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nwarra.key", offset: 0x0, size: 0x14, crc: 0x9BAFFF67 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNTJ: RomSetDef = RomSetDef { name: "vhuntj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphj.03f", offset: 0x0, size: 0x80000, crc: 0x3DE2E333 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.04c", offset: 0x80000, size: 0x80000, crc: 0xC95CF304 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.05d", offset: 0x100000, size: 0x80000, crc: 0x50DE5DDD },
            Op::Load { kind: Load::Word16Swap, file: "vphj.06c", offset: 0x180000, size: 0x80000, crc: 0xAC3BD3D5 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.07b", offset: 0x200000, size: 0x80000, crc: 0x0761309F },
            Op::Load { kind: Load::Word16Swap, file: "vphj.08b", offset: 0x280000, size: 0x80000, crc: 0x5A5C2BF5 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.09b", offset: 0x300000, size: 0x80000, crc: 0x823D6D99 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.10b", offset: 0x380000, size: 0x80000, crc: 0x32C7D8F0 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vhuntj.key", offset: 0x0, size: 0x14, crc: 0x72854F68 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNTJR1S: RomSetDef = RomSetDef { name: "vhuntjr1s", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphjstop.03b", offset: 0x0, size: 0x80000, crc: 0x9C4E6191 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.04c", offset: 0x80000, size: 0x80000, crc: 0xC95CF304 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.05d", offset: 0x100000, size: 0x80000, crc: 0x50DE5DDD },
            Op::Load { kind: Load::Word16Swap, file: "vphj.06c", offset: 0x180000, size: 0x80000, crc: 0xAC3BD3D5 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.07b", offset: 0x200000, size: 0x80000, crc: 0x0761309F },
            Op::Load { kind: Load::Word16Swap, file: "vphj.08b", offset: 0x280000, size: 0x80000, crc: 0x5A5C2BF5 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.09b", offset: 0x300000, size: 0x80000, crc: 0x823D6D99 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.10b", offset: 0x380000, size: 0x80000, crc: 0x32C7D8F0 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vhuntj.key", offset: 0x0, size: 0x14, crc: 0x72854F68 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNTJR1: RomSetDef = RomSetDef { name: "vhuntjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphj.03c", offset: 0x0, size: 0x80000, crc: 0x606B682A },
            Op::Load { kind: Load::Word16Swap, file: "vphj.04b", offset: 0x80000, size: 0x80000, crc: 0xA3B40393 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.05b", offset: 0x100000, size: 0x80000, crc: 0xFCCD5558 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.06b", offset: 0x180000, size: 0x80000, crc: 0x07E10A73 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.07b", offset: 0x200000, size: 0x80000, crc: 0x0761309F },
            Op::Load { kind: Load::Word16Swap, file: "vphj.08b", offset: 0x280000, size: 0x80000, crc: 0x5A5C2BF5 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.09b", offset: 0x300000, size: 0x80000, crc: 0x823D6D99 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.10b", offset: 0x380000, size: 0x80000, crc: 0x32C7D8F0 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vhuntj.key", offset: 0x0, size: 0x14, crc: 0x72854F68 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNTJR2: RomSetDef = RomSetDef { name: "vhuntjr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphj.03b", offset: 0x0, size: 0x80000, crc: 0x679C3FA9 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.04a", offset: 0x80000, size: 0x80000, crc: 0xEB6E71E4 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.05a", offset: 0x100000, size: 0x80000, crc: 0xEAF634EA },
            Op::Load { kind: Load::Word16Swap, file: "vphj.06a", offset: 0x180000, size: 0x80000, crc: 0xB70CC6BE },
            Op::Load { kind: Load::Word16Swap, file: "vphj.07a", offset: 0x200000, size: 0x80000, crc: 0x46AB907D },
            Op::Load { kind: Load::Word16Swap, file: "vphj.08a", offset: 0x280000, size: 0x80000, crc: 0x1C00355E },
            Op::Load { kind: Load::Word16Swap, file: "vphj.09a", offset: 0x300000, size: 0x80000, crc: 0x026E6F82 },
            Op::Load { kind: Load::Word16Swap, file: "vphj.10a", offset: 0x380000, size: 0x80000, crc: 0xAADFB3EA },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vhuntj.key", offset: 0x0, size: 0x14, crc: 0x72854F68 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEAR: RomSetDef = RomSetDef { name: "progear", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgae.03", offset: 0x0, size: 0x80000, crc: 0x8577BC86 },
            Op::Load { kind: Load::Word16Swap, file: "pgae.04", offset: 0x80000, size: 0x80000, crc: 0xD850DA04 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pga-simm.01c", offset: 0x0, size: 0x200000, crc: 0x452F98B0 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01d", offset: 0x1, size: 0x200000, crc: 0x9E672092 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01a", offset: 0x2, size: 0x200000, crc: 0xAE9DDAFE },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01b", offset: 0x3, size: 0x200000, crc: 0x94D72D94 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03c", offset: 0x4, size: 0x200000, crc: 0x48A1886D },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03d", offset: 0x5, size: 0x200000, crc: 0x172D7E37 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03a", offset: 0x6, size: 0x200000, crc: 0x9EE33D98 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03b", offset: 0x7, size: 0x200000, crc: 0x848DEE32 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05a", offset: 0x0, size: 0x200000, crc: 0xC0AAC80C },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x37A65D86 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06a", offset: 0x400000, size: 0x200000, crc: 0xD3F1E934 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06b", offset: 0x600000, size: 0x200000, crc: 0x8B39489A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "progear.key", offset: 0x0, size: 0x14, crc: 0xEEE6B2A8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEARU: RomSetDef = RomSetDef { name: "progearu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgau.03", offset: 0x0, size: 0x80000, crc: 0x343A783E },
            Op::Load { kind: Load::Word16Swap, file: "pgau.04", offset: 0x80000, size: 0x80000, crc: 0x16208D79 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pga-simm.01c", offset: 0x0, size: 0x200000, crc: 0x452F98B0 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01d", offset: 0x1, size: 0x200000, crc: 0x9E672092 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01a", offset: 0x2, size: 0x200000, crc: 0xAE9DDAFE },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01b", offset: 0x3, size: 0x200000, crc: 0x94D72D94 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03c", offset: 0x4, size: 0x200000, crc: 0x48A1886D },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03d", offset: 0x5, size: 0x200000, crc: 0x172D7E37 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03a", offset: 0x6, size: 0x200000, crc: 0x9EE33D98 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03b", offset: 0x7, size: 0x200000, crc: 0x848DEE32 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05a", offset: 0x0, size: 0x200000, crc: 0xC0AAC80C },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x37A65D86 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06a", offset: 0x400000, size: 0x200000, crc: 0xD3F1E934 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06b", offset: 0x600000, size: 0x200000, crc: 0x8B39489A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "progearu.key", offset: 0x0, size: 0x14, crc: 0x46736B17 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEARJ: RomSetDef = RomSetDef { name: "progearj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgaj.03", offset: 0x0, size: 0x80000, crc: 0x06DBBA54 },
            Op::Load { kind: Load::Word16Swap, file: "pgaj.04", offset: 0x80000, size: 0x80000, crc: 0xA1F1F1BC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pga-simm.01c", offset: 0x0, size: 0x200000, crc: 0x452F98B0 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01d", offset: 0x1, size: 0x200000, crc: 0x9E672092 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01a", offset: 0x2, size: 0x200000, crc: 0xAE9DDAFE },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01b", offset: 0x3, size: 0x200000, crc: 0x94D72D94 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03c", offset: 0x4, size: 0x200000, crc: 0x48A1886D },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03d", offset: 0x5, size: 0x200000, crc: 0x172D7E37 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03a", offset: 0x6, size: 0x200000, crc: 0x9EE33D98 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03b", offset: 0x7, size: 0x200000, crc: 0x848DEE32 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05a", offset: 0x0, size: 0x200000, crc: 0xC0AAC80C },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x37A65D86 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06a", offset: 0x400000, size: 0x200000, crc: 0xD3F1E934 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06b", offset: 0x600000, size: 0x200000, crc: 0x8B39489A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "progearj.key", offset: 0x0, size: 0x14, crc: 0xD8D515E5 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEARA: RomSetDef = RomSetDef { name: "progeara", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgaa.03", offset: 0x0, size: 0x80000, crc: 0x25E6E2CE },
            Op::Load { kind: Load::Word16Swap, file: "pgaa.04", offset: 0x80000, size: 0x80000, crc: 0x8104307E },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pga-simm.01c", offset: 0x0, size: 0x200000, crc: 0x452F98B0 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01d", offset: 0x1, size: 0x200000, crc: 0x9E672092 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01a", offset: 0x2, size: 0x200000, crc: 0xAE9DDAFE },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01b", offset: 0x3, size: 0x200000, crc: 0x94D72D94 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03c", offset: 0x4, size: 0x200000, crc: 0x48A1886D },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03d", offset: 0x5, size: 0x200000, crc: 0x172D7E37 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03a", offset: 0x6, size: 0x200000, crc: 0x9EE33D98 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03b", offset: 0x7, size: 0x200000, crc: 0x848DEE32 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05a", offset: 0x0, size: 0x200000, crc: 0xC0AAC80C },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x37A65D86 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06a", offset: 0x400000, size: 0x200000, crc: 0xD3F1E934 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06b", offset: 0x600000, size: 0x200000, crc: 0x8B39489A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "progeara.key", offset: 0x0, size: 0x14, crc: 0x30A0FAB6 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PZLOOP2: RomSetDef = RomSetDef { name: "pzloop2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2e.03", offset: 0x0, size: 0x80000, crc: 0x3B1285B2 },
            Op::Load { kind: Load::Word16Swap, file: "pl2e.04", offset: 0x80000, size: 0x80000, crc: 0x40A2D647 },
            Op::Load { kind: Load::Word16Swap, file: "pl2e.05", offset: 0x100000, size: 0x80000, crc: 0x0F11D818 },
            Op::Load { kind: Load::Word16Swap, file: "pl2e.06", offset: 0x180000, size: 0x80000, crc: 0x86FBBDF4 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01c", offset: 0x0, size: 0x200000, crc: 0x137B13A7 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01d", offset: 0x1, size: 0x200000, crc: 0xA2DB1507 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01a", offset: 0x2, size: 0x200000, crc: 0x7E80FF8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01b", offset: 0x3, size: 0x200000, crc: 0xCD93E6ED },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03c", offset: 0x4, size: 0x200000, crc: 0x0F52BBCA },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03d", offset: 0x5, size: 0x200000, crc: 0xA62712C3 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03a", offset: 0x6, size: 0x200000, crc: 0xB60C9F8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03b", offset: 0x7, size: 0x200000, crc: 0x83FEF284 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pl2.01", offset: 0x0, size: 0x8000, crc: 0x35697569 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05a", offset: 0x0, size: 0x200000, crc: 0x85D8FBE8 },
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x1ED62584 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzloop2.key", offset: 0x0, size: 0x14, crc: 0xAE13BE78 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PZLOOP2J: RomSetDef = RomSetDef { name: "pzloop2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2j.03c", offset: 0x0, size: 0x80000, crc: 0x3B76B806 },
            Op::Load { kind: Load::Word16Swap, file: "pl2j.04c", offset: 0x80000, size: 0x80000, crc: 0x8878A42A },
            Op::Load { kind: Load::Word16Swap, file: "pl2j.05c", offset: 0x100000, size: 0x80000, crc: 0x51081EA4 },
            Op::Load { kind: Load::Word16Swap, file: "pl2j.06c", offset: 0x180000, size: 0x80000, crc: 0x51C68494 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01c", offset: 0x0, size: 0x200000, crc: 0x137B13A7 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01d", offset: 0x1, size: 0x200000, crc: 0xA2DB1507 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01a", offset: 0x2, size: 0x200000, crc: 0x7E80FF8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01b", offset: 0x3, size: 0x200000, crc: 0xCD93E6ED },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03c", offset: 0x4, size: 0x200000, crc: 0x0F52BBCA },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03d", offset: 0x5, size: 0x200000, crc: 0xA62712C3 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03a", offset: 0x6, size: 0x200000, crc: 0xB60C9F8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03b", offset: 0x7, size: 0x200000, crc: 0x83FEF284 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pl2.01", offset: 0x0, size: 0x8000, crc: 0x35697569 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05a", offset: 0x0, size: 0x200000, crc: 0x85D8FBE8 },
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x1ED62584 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzloop2.key", offset: 0x0, size: 0x14, crc: 0xAE13BE78 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PZLOOP2JR1: RomSetDef = RomSetDef { name: "pzloop2jr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2j.03a", offset: 0x0, size: 0x80000, crc: 0x0A751BD0 },
            Op::Load { kind: Load::Word16Swap, file: "pl2j.04a", offset: 0x80000, size: 0x80000, crc: 0xC3F72AFE },
            Op::Load { kind: Load::Word16Swap, file: "pl2j.05a", offset: 0x100000, size: 0x80000, crc: 0x6EA9DBFC },
            Op::Load { kind: Load::Word16Swap, file: "pl2j.06a", offset: 0x180000, size: 0x80000, crc: 0x0F14848D },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01c", offset: 0x0, size: 0x200000, crc: 0x137B13A7 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01d", offset: 0x1, size: 0x200000, crc: 0xA2DB1507 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01a", offset: 0x2, size: 0x200000, crc: 0x7E80FF8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01b", offset: 0x3, size: 0x200000, crc: 0xCD93E6ED },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03c", offset: 0x4, size: 0x200000, crc: 0x0F52BBCA },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03d", offset: 0x5, size: 0x200000, crc: 0xA62712C3 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03a", offset: 0x6, size: 0x200000, crc: 0xB60C9F8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03b", offset: 0x7, size: 0x200000, crc: 0x83FEF284 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pl2.01", offset: 0x0, size: 0x8000, crc: 0x35697569 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05a", offset: 0x0, size: 0x200000, crc: 0x85D8FBE8 },
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x1ED62584 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzloop2.key", offset: 0x0, size: 0x14, crc: 0xAE13BE78 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PZLOOP2JP: RomSetDef = RomSetDef { name: "pzloop2jp", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2j_03.bin", offset: 0x0, size: 0x80000, crc: 0xE9F904C3 },
            Op::Load { kind: Load::Word16Swap, file: "pl2j_04.bin", offset: 0x80000, size: 0x80000, crc: 0x61C7C18F },
            Op::Load { kind: Load::Word16Swap, file: "pl2j_05.bin", offset: 0x100000, size: 0x80000, crc: 0xEE6DA97C },
            Op::Load { kind: Load::Word16Swap, file: "pl2j_06.bin", offset: 0x180000, size: 0x80000, crc: 0x97E1C3AC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pl2-13m.bin", offset: 0x0, size: 0x200000, crc: 0x5F3B5F91 },
            Op::Load { kind: Load::Word64, file: "pl2-15m.bin", offset: 0x2, size: 0x200000, crc: 0x3702F309 },
            Op::Load { kind: Load::Word64, file: "pl2-17m.bin", offset: 0x4, size: 0x200000, crc: 0x62D3FCE9 },
            Op::Load { kind: Load::Word64, file: "pl2-19m.bin", offset: 0x6, size: 0x200000, crc: 0x6DCBD8CE },
            Op::Load { kind: Load::Word64, file: "pl2-14m.bin", offset: 0x800000, size: 0x200000, crc: 0x2E22E71A },
            Op::Load { kind: Load::Word64, file: "pl2-16m.bin", offset: 0x800002, size: 0x200000, crc: 0x38090022 },
            Op::Load { kind: Load::Word64, file: "pl2-18m.bin", offset: 0x800004, size: 0x200000, crc: 0x33AFDD44 },
            Op::Load { kind: Load::Word64, file: "pl2-20m.bin", offset: 0x800006, size: 0x200000, crc: 0xD4AE0278 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pl2_01.bin", offset: 0x0, size: 0x8000, crc: 0x35697569 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2-11m.bin", offset: 0x0, size: 0x200000, crc: 0x85D8FBE8 },
            Op::Load { kind: Load::Word16Swap, file: "pl2-12m.bin", offset: 0x200000, size: 0x200000, crc: 0x1ED62584 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_QNDREAM: RomSetDef = RomSetDef { name: "qndream", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tqzj.03a", offset: 0x0, size: 0x80000, crc: 0x7ACF3E30 },
            Op::Load { kind: Load::Word16Swap, file: "tqzj.04", offset: 0x80000, size: 0x80000, crc: 0xF1044A87 },
            Op::Load { kind: Load::Word16Swap, file: "tqzj.05", offset: 0x100000, size: 0x80000, crc: 0x4105BA0E },
            Op::Load { kind: Load::Word16Swap, file: "tqzj.06", offset: 0x180000, size: 0x80000, crc: 0xC371E8A5 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "tqz.14m", offset: 0x800000, size: 0x200000, crc: 0x98AF88A2 },
            Op::Load { kind: Load::Word64, file: "tqz.16m", offset: 0x800002, size: 0x200000, crc: 0xDF82D491 },
            Op::Load { kind: Load::Word64, file: "tqz.18m", offset: 0x800004, size: 0x200000, crc: 0x42F132FF },
            Op::Load { kind: Load::Word64, file: "tqz.20m", offset: 0x800006, size: 0x200000, crc: 0xB2E128A3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tqz.01", offset: 0x0, size: 0x8000, crc: 0xE9CE9D0A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tqz.11m", offset: 0x0, size: 0x200000, crc: 0x78E7884F },
            Op::Load { kind: Load::Word16Swap, file: "tqz.12m", offset: 0x200000, size: 0x200000, crc: 0x2E049B13 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "qndream.key", offset: 0x0, size: 0x14, crc: 0x97EEE4FF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_RINGDEST: RomSetDef = RomSetDef { name: "ringdest", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smbe.03b", offset: 0x0, size: 0x80000, crc: 0xB8016278 },
            Op::Load { kind: Load::Word16Swap, file: "smbe.04b", offset: 0x80000, size: 0x80000, crc: 0x18C4C447 },
            Op::Load { kind: Load::Word16Swap, file: "smbe.05b", offset: 0x100000, size: 0x80000, crc: 0x18EBDA7F },
            Op::Load { kind: Load::Word16Swap, file: "smbe.06b", offset: 0x180000, size: 0x80000, crc: 0x89C80007 },
            Op::Load { kind: Load::Word16Swap, file: "smb.07", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smb.08", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ringdest.key", offset: 0x0, size: 0x14, crc: 0x17F9269C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SMBOMB: RomSetDef = RomSetDef { name: "smbomb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smbj.03a", offset: 0x0, size: 0x80000, crc: 0x1C5613DE },
            Op::Load { kind: Load::Word16Swap, file: "smbj.04a", offset: 0x80000, size: 0x80000, crc: 0x29071ED7 },
            Op::Load { kind: Load::Word16Swap, file: "smbj.05a", offset: 0x100000, size: 0x80000, crc: 0xEB20BCE4 },
            Op::Load { kind: Load::Word16Swap, file: "smbj.06a", offset: 0x180000, size: 0x80000, crc: 0x94B420CD },
            Op::Load { kind: Load::Word16Swap, file: "smb.07", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smb.08", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smbomb.key", offset: 0x0, size: 0x14, crc: 0xF690069B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SMBOMBR1: RomSetDef = RomSetDef { name: "smbombr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smbj.03", offset: 0x0, size: 0x80000, crc: 0x52EAFB10 },
            Op::Load { kind: Load::Word16Swap, file: "smbj.04", offset: 0x80000, size: 0x80000, crc: 0xAA6E8078 },
            Op::Load { kind: Load::Word16Swap, file: "smbj.05", offset: 0x100000, size: 0x80000, crc: 0xB69E7D5F },
            Op::Load { kind: Load::Word16Swap, file: "smbj.06", offset: 0x180000, size: 0x80000, crc: 0x8D857B56 },
            Op::Load { kind: Load::Word16Swap, file: "smb.07", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smb.08", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smbomb.key", offset: 0x0, size: 0x14, crc: 0xF690069B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_RINGDESTA: RomSetDef = RomSetDef { name: "ringdesta", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smba.03a", offset: 0x0, size: 0x80000, crc: 0xD3744DFD },
            Op::Load { kind: Load::Word16Swap, file: "smba.04a", offset: 0x80000, size: 0x80000, crc: 0xF32D5B4F },
            Op::Load { kind: Load::Word16Swap, file: "smba.05a", offset: 0x100000, size: 0x80000, crc: 0x1016454F },
            Op::Load { kind: Load::Word16Swap, file: "smba.06a", offset: 0x180000, size: 0x80000, crc: 0x94B420CD },
            Op::Load { kind: Load::Word16Swap, file: "smb.07", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smb.08", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ringdesta.key", offset: 0x0, size: 0x14, crc: 0x905C9065 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_RINGDESTH: RomSetDef = RomSetDef { name: "ringdesth", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smbh.03b", offset: 0x0, size: 0x80000, crc: 0x2E316584 },
            Op::Load { kind: Load::Word16Swap, file: "smbh.04b", offset: 0x80000, size: 0x80000, crc: 0x9950A23A },
            Op::Load { kind: Load::Word16Swap, file: "smbh.05b", offset: 0x100000, size: 0x80000, crc: 0x41E0B3FC },
            Op::Load { kind: Load::Word16Swap, file: "smbh.06b", offset: 0x180000, size: 0x80000, crc: 0x89C80007 },
            Op::Load { kind: Load::Word16Swap, file: "smbh.07b", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smbh.08b", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ringdesth.key", offset: 0x0, size: 0x14, crc: 0xFFB8D049 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_RINGDESTB: RomSetDef = RomSetDef { name: "ringdestb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smbb.03b", offset: 0x0, size: 0x80000, crc: 0x234EEBCC },
            Op::Load { kind: Load::Word16Swap, file: "smbb.04b", offset: 0x80000, size: 0x80000, crc: 0x9950A23A },
            Op::Load { kind: Load::Word16Swap, file: "smbb.05b", offset: 0x100000, size: 0x80000, crc: 0x41E0B3FC },
            Op::Load { kind: Load::Word16Swap, file: "smbb.06b", offset: 0x180000, size: 0x80000, crc: 0x89C80007 },
            Op::Load { kind: Load::Word16Swap, file: "smbb.07b", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smbb.08b", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ringdesth.key", offset: 0x0, size: 0x14, crc: 0xFFB8D049 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMANCP2U: RomSetDef = RomSetDef { name: "mmancp2u", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcmu.03b", offset: 0x0, size: 0x80000, crc: 0xC39F037F },
            Op::Load { kind: Load::Word16Swap, file: "rcmu.04b", offset: 0x80000, size: 0x80000, crc: 0xCD6F5E99 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.05b", offset: 0x100000, size: 0x80000, crc: 0x4376EA95 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rcm.73", offset: 0x800000, size: 0x80000, crc: 0x774C6E04 },
            Op::Load { kind: Load::Word64, file: "rcm.63", offset: 0x800002, size: 0x80000, crc: 0xACAD7C62 },
            Op::Load { kind: Load::Word64, file: "rcm.83", offset: 0x800004, size: 0x80000, crc: 0x6AF30499 },
            Op::Load { kind: Load::Word64, file: "rcm.93", offset: 0x800006, size: 0x80000, crc: 0x7A5A5166 },
            Op::Load { kind: Load::Word64, file: "rcm.74", offset: 0xa00000, size: 0x80000, crc: 0x004EC725 },
            Op::Load { kind: Load::Word64, file: "rcm.64", offset: 0xa00002, size: 0x80000, crc: 0x65C0464E },
            Op::Load { kind: Load::Word64, file: "rcm.84", offset: 0xa00004, size: 0x80000, crc: 0xFB3097CC },
            Op::Load { kind: Load::Word64, file: "rcm.94", offset: 0xa00006, size: 0x80000, crc: 0x2E16557A },
            Op::Load { kind: Load::Word64, file: "rcm.75", offset: 0xc00000, size: 0x80000, crc: 0x70A73F99 },
            Op::Load { kind: Load::Word64, file: "rcm.65", offset: 0xc00002, size: 0x80000, crc: 0xECEDAD3D },
            Op::Load { kind: Load::Word64, file: "rcm.85", offset: 0xc00004, size: 0x80000, crc: 0x3D6186D8 },
            Op::Load { kind: Load::Word64, file: "rcm.95", offset: 0xc00006, size: 0x80000, crc: 0x8C7700F1 },
            Op::Load { kind: Load::Word64, file: "rcm.76", offset: 0xe00000, size: 0x80000, crc: 0x89A889AD },
            Op::Load { kind: Load::Word64, file: "rcm.66", offset: 0xe00002, size: 0x80000, crc: 0x1300EB7B },
            Op::Load { kind: Load::Word64, file: "rcm.86", offset: 0xe00004, size: 0x80000, crc: 0x6D974EBD },
            Op::Load { kind: Load::Word64, file: "rcm.96", offset: 0xe00006, size: 0x80000, crc: 0x7DA4CD24 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm.01", offset: 0x0, size: 0x8000, crc: 0xD60CF8A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcm.51", offset: 0x0, size: 0x80000, crc: 0xB6D07080 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.52", offset: 0x80000, size: 0x80000, crc: 0xDFDDC493 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.53", offset: 0x100000, size: 0x80000, crc: 0x6062AE3A },
            Op::Load { kind: Load::Word16Swap, file: "rcm.54", offset: 0x180000, size: 0x80000, crc: 0x08C6F3BF },
            Op::Load { kind: Load::Word16Swap, file: "rcm.55", offset: 0x200000, size: 0x80000, crc: 0xF97DFCCC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.56", offset: 0x280000, size: 0x80000, crc: 0xADE475BC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.57", offset: 0x300000, size: 0x80000, crc: 0x075EFFB3 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.58", offset: 0x380000, size: 0x80000, crc: 0xF6C1F87B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmancp2u.key", offset: 0x0, size: 0x14, crc: 0x17CA6659 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMANCP2UR1: RomSetDef = RomSetDef { name: "mmancp2ur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcmu.03a", offset: 0x0, size: 0x80000, crc: 0xC6B75320 },
            Op::Load { kind: Load::Word16Swap, file: "rcmu.04a", offset: 0x80000, size: 0x80000, crc: 0x47880111 },
            Op::Load { kind: Load::Word16Swap, file: "rcmu.05a", offset: 0x100000, size: 0x80000, crc: 0x4376EA95 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rcm.73", offset: 0x800000, size: 0x80000, crc: 0x774C6E04 },
            Op::Load { kind: Load::Word64, file: "rcm.63", offset: 0x800002, size: 0x80000, crc: 0xACAD7C62 },
            Op::Load { kind: Load::Word64, file: "rcm.83", offset: 0x800004, size: 0x80000, crc: 0x6AF30499 },
            Op::Load { kind: Load::Word64, file: "rcm.93", offset: 0x800006, size: 0x80000, crc: 0x7A5A5166 },
            Op::Load { kind: Load::Word64, file: "rcm.74", offset: 0xa00000, size: 0x80000, crc: 0x004EC725 },
            Op::Load { kind: Load::Word64, file: "rcm.64", offset: 0xa00002, size: 0x80000, crc: 0x65C0464E },
            Op::Load { kind: Load::Word64, file: "rcm.84", offset: 0xa00004, size: 0x80000, crc: 0xFB3097CC },
            Op::Load { kind: Load::Word64, file: "rcm.94", offset: 0xa00006, size: 0x80000, crc: 0x2E16557A },
            Op::Load { kind: Load::Word64, file: "rcm.75", offset: 0xc00000, size: 0x80000, crc: 0x70A73F99 },
            Op::Load { kind: Load::Word64, file: "rcm.65", offset: 0xc00002, size: 0x80000, crc: 0xECEDAD3D },
            Op::Load { kind: Load::Word64, file: "rcm.85", offset: 0xc00004, size: 0x80000, crc: 0x3D6186D8 },
            Op::Load { kind: Load::Word64, file: "rcm.95", offset: 0xc00006, size: 0x80000, crc: 0x8C7700F1 },
            Op::Load { kind: Load::Word64, file: "rcm.76", offset: 0xe00000, size: 0x80000, crc: 0x89A889AD },
            Op::Load { kind: Load::Word64, file: "rcm.66", offset: 0xe00002, size: 0x80000, crc: 0x1300EB7B },
            Op::Load { kind: Load::Word64, file: "rcm.86", offset: 0xe00004, size: 0x80000, crc: 0x6D974EBD },
            Op::Load { kind: Load::Word64, file: "rcm.96", offset: 0xe00006, size: 0x80000, crc: 0x7DA4CD24 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm.01", offset: 0x0, size: 0x8000, crc: 0xD60CF8A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcm.51", offset: 0x0, size: 0x80000, crc: 0xB6D07080 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.52", offset: 0x80000, size: 0x80000, crc: 0xDFDDC493 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.53", offset: 0x100000, size: 0x80000, crc: 0x6062AE3A },
            Op::Load { kind: Load::Word16Swap, file: "rcm.54", offset: 0x180000, size: 0x80000, crc: 0x08C6F3BF },
            Op::Load { kind: Load::Word16Swap, file: "rcm.55", offset: 0x200000, size: 0x80000, crc: 0xF97DFCCC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.56", offset: 0x280000, size: 0x80000, crc: 0xADE475BC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.57", offset: 0x300000, size: 0x80000, crc: 0x075EFFB3 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.58", offset: 0x380000, size: 0x80000, crc: 0xF6C1F87B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmancp2u.key", offset: 0x0, size: 0x14, crc: 0x17CA6659 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMANCP2UR2: RomSetDef = RomSetDef { name: "mmancp2ur2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcmu.03", offset: 0x0, size: 0x80000, crc: 0x3CFDB1BB },
            Op::Load { kind: Load::Word16Swap, file: "rcmu.04", offset: 0x80000, size: 0x80000, crc: 0x169BF5E8 },
            Op::Load { kind: Load::Word16Swap, file: "rcmu.05", offset: 0x100000, size: 0x80000, crc: 0x4376EA95 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rcm.73", offset: 0x800000, size: 0x80000, crc: 0x774C6E04 },
            Op::Load { kind: Load::Word64, file: "rcm.63", offset: 0x800002, size: 0x80000, crc: 0xACAD7C62 },
            Op::Load { kind: Load::Word64, file: "rcm.83", offset: 0x800004, size: 0x80000, crc: 0x6AF30499 },
            Op::Load { kind: Load::Word64, file: "rcm.93", offset: 0x800006, size: 0x80000, crc: 0x7A5A5166 },
            Op::Load { kind: Load::Word64, file: "rcm.74", offset: 0xa00000, size: 0x80000, crc: 0x004EC725 },
            Op::Load { kind: Load::Word64, file: "rcm.64", offset: 0xa00002, size: 0x80000, crc: 0x65C0464E },
            Op::Load { kind: Load::Word64, file: "rcm.84", offset: 0xa00004, size: 0x80000, crc: 0xFB3097CC },
            Op::Load { kind: Load::Word64, file: "rcm.94", offset: 0xa00006, size: 0x80000, crc: 0x2E16557A },
            Op::Load { kind: Load::Word64, file: "rcm.75", offset: 0xc00000, size: 0x80000, crc: 0x70A73F99 },
            Op::Load { kind: Load::Word64, file: "rcm.65", offset: 0xc00002, size: 0x80000, crc: 0xECEDAD3D },
            Op::Load { kind: Load::Word64, file: "rcm.85", offset: 0xc00004, size: 0x80000, crc: 0x3D6186D8 },
            Op::Load { kind: Load::Word64, file: "rcm.95", offset: 0xc00006, size: 0x80000, crc: 0x8C7700F1 },
            Op::Load { kind: Load::Word64, file: "rcm.76", offset: 0xe00000, size: 0x80000, crc: 0x89A889AD },
            Op::Load { kind: Load::Word64, file: "rcm.66", offset: 0xe00002, size: 0x80000, crc: 0x1300EB7B },
            Op::Load { kind: Load::Word64, file: "rcm.86", offset: 0xe00004, size: 0x80000, crc: 0x6D974EBD },
            Op::Load { kind: Load::Word64, file: "rcm.96", offset: 0xe00006, size: 0x80000, crc: 0x7DA4CD24 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm.01", offset: 0x0, size: 0x8000, crc: 0xD60CF8A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcm.51", offset: 0x0, size: 0x80000, crc: 0xB6D07080 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.52", offset: 0x80000, size: 0x80000, crc: 0xDFDDC493 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.53", offset: 0x100000, size: 0x80000, crc: 0x6062AE3A },
            Op::Load { kind: Load::Word16Swap, file: "rcm.54", offset: 0x180000, size: 0x80000, crc: 0x08C6F3BF },
            Op::Load { kind: Load::Word16Swap, file: "rcm.55", offset: 0x200000, size: 0x80000, crc: 0xF97DFCCC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.56", offset: 0x280000, size: 0x80000, crc: 0xADE475BC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.57", offset: 0x300000, size: 0x80000, crc: 0x075EFFB3 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.58", offset: 0x380000, size: 0x80000, crc: 0xF6C1F87B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmancp2u.key", offset: 0x0, size: 0x14, crc: 0x17CA6659 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_RMANCP2J: RomSetDef = RomSetDef { name: "rmancp2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcmj.03a", offset: 0x0, size: 0x80000, crc: 0x30559F60 },
            Op::Load { kind: Load::Word16Swap, file: "rcmj.04a", offset: 0x80000, size: 0x80000, crc: 0x5EFC9366 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.05a", offset: 0x100000, size: 0x80000, crc: 0x517CCDE2 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rcm.73", offset: 0x800000, size: 0x80000, crc: 0x774C6E04 },
            Op::Load { kind: Load::Word64, file: "rcm.63", offset: 0x800002, size: 0x80000, crc: 0xACAD7C62 },
            Op::Load { kind: Load::Word64, file: "rcm.83", offset: 0x800004, size: 0x80000, crc: 0x6AF30499 },
            Op::Load { kind: Load::Word64, file: "rcm.93", offset: 0x800006, size: 0x80000, crc: 0x7A5A5166 },
            Op::Load { kind: Load::Word64, file: "rcm.74", offset: 0xa00000, size: 0x80000, crc: 0x004EC725 },
            Op::Load { kind: Load::Word64, file: "rcm.64", offset: 0xa00002, size: 0x80000, crc: 0x65C0464E },
            Op::Load { kind: Load::Word64, file: "rcm.84", offset: 0xa00004, size: 0x80000, crc: 0xFB3097CC },
            Op::Load { kind: Load::Word64, file: "rcm.94", offset: 0xa00006, size: 0x80000, crc: 0x2E16557A },
            Op::Load { kind: Load::Word64, file: "rcm.75", offset: 0xc00000, size: 0x80000, crc: 0x70A73F99 },
            Op::Load { kind: Load::Word64, file: "rcm.65", offset: 0xc00002, size: 0x80000, crc: 0xECEDAD3D },
            Op::Load { kind: Load::Word64, file: "rcm.85", offset: 0xc00004, size: 0x80000, crc: 0x3D6186D8 },
            Op::Load { kind: Load::Word64, file: "rcm.95", offset: 0xc00006, size: 0x80000, crc: 0x8C7700F1 },
            Op::Load { kind: Load::Word64, file: "rcm.76", offset: 0xe00000, size: 0x80000, crc: 0x89A889AD },
            Op::Load { kind: Load::Word64, file: "rcm.66", offset: 0xe00002, size: 0x80000, crc: 0x1300EB7B },
            Op::Load { kind: Load::Word64, file: "rcm.86", offset: 0xe00004, size: 0x80000, crc: 0x6D974EBD },
            Op::Load { kind: Load::Word64, file: "rcm.96", offset: 0xe00006, size: 0x80000, crc: 0x7DA4CD24 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm.01", offset: 0x0, size: 0x8000, crc: 0xD60CF8A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcm.51", offset: 0x0, size: 0x80000, crc: 0xB6D07080 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.52", offset: 0x80000, size: 0x80000, crc: 0xDFDDC493 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.53", offset: 0x100000, size: 0x80000, crc: 0x6062AE3A },
            Op::Load { kind: Load::Word16Swap, file: "rcm.54", offset: 0x180000, size: 0x80000, crc: 0x08C6F3BF },
            Op::Load { kind: Load::Word16Swap, file: "rcm.55", offset: 0x200000, size: 0x80000, crc: 0xF97DFCCC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.56", offset: 0x280000, size: 0x80000, crc: 0xADE475BC },
            Op::Load { kind: Load::Word16Swap, file: "rcm.57", offset: 0x300000, size: 0x80000, crc: 0x075EFFB3 },
            Op::Load { kind: Load::Word16Swap, file: "rcm.58", offset: 0x380000, size: 0x80000, crc: 0xF6C1F87B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rmancp2j.key", offset: 0x0, size: 0x14, crc: 0x17309A70 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA: RomSetDef = RomSetDef { name: "sfa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfze.03d", offset: 0x0, size: 0x80000, crc: 0xEBF2054D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04b", offset: 0x80000, size: 0x80000, crc: 0x8B73B0E5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa.key", offset: 0x0, size: 0x14, crc: 0x7C095631 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFAR1: RomSetDef = RomSetDef { name: "sfar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfze.03c", offset: 0x0, size: 0x80000, crc: 0xA1B69DD7 },
            Op::Load { kind: Load::Word16Swap, file: "sfze.04b", offset: 0x80000, size: 0x80000, crc: 0xBB90ACD5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa.key", offset: 0x0, size: 0x14, crc: 0x7C095631 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFAR2: RomSetDef = RomSetDef { name: "sfar2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfze.03b", offset: 0x0, size: 0x80000, crc: 0x2BF5708E },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04a", offset: 0x80000, size: 0x80000, crc: 0x5F99E9A5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa.key", offset: 0x0, size: 0x14, crc: 0x7C095631 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFAR3: RomSetDef = RomSetDef { name: "sfar3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfze.03a", offset: 0x0, size: 0x80000, crc: 0xFDBCD434 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04", offset: 0x80000, size: 0x80000, crc: 0x0C436D30 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05", offset: 0x100000, size: 0x80000, crc: 0x1F363612 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa.key", offset: 0x0, size: 0x14, crc: 0x7C095631 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFAU: RomSetDef = RomSetDef { name: "sfau", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzu.03a", offset: 0x0, size: 0x80000, crc: 0x49FC7DB9 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04a", offset: 0x80000, size: 0x80000, crc: 0x5F99E9A5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfau.key", offset: 0x0, size: 0x14, crc: 0x1DD0998D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZA: RomSetDef = RomSetDef { name: "sfza", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfza.03b", offset: 0x0, size: 0x80000, crc: 0xCA91BED9 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04a", offset: 0x80000, size: 0x80000, crc: 0x5F99E9A5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfza.key", offset: 0x0, size: 0x14, crc: 0x2AA6AC63 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZAR1: RomSetDef = RomSetDef { name: "sfzar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfza.03a", offset: 0x0, size: 0x80000, crc: 0xF38D8C8D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04", offset: 0x80000, size: 0x80000, crc: 0x0C436D30 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05", offset: 0x100000, size: 0x80000, crc: 0x1F363612 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfza.key", offset: 0x0, size: 0x14, crc: 0x2AA6AC63 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZJ: RomSetDef = RomSetDef { name: "sfzj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzj.03c", offset: 0x0, size: 0x80000, crc: 0xF5444120 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04b", offset: 0x80000, size: 0x80000, crc: 0x8B73B0E5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzj.key", offset: 0x0, size: 0x14, crc: 0x355D85B8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZJR1: RomSetDef = RomSetDef { name: "sfzjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzj.03b", offset: 0x0, size: 0x80000, crc: 0x844220C2 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04a", offset: 0x80000, size: 0x80000, crc: 0x5F99E9A5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzj.key", offset: 0x0, size: 0x14, crc: 0x355D85B8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZJR2: RomSetDef = RomSetDef { name: "sfzjr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzj.03a", offset: 0x0, size: 0x80000, crc: 0x3CFCE93C },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04", offset: 0x80000, size: 0x80000, crc: 0x0C436D30 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05", offset: 0x100000, size: 0x80000, crc: 0x1F363612 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzj.key", offset: 0x0, size: 0x14, crc: 0x355D85B8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZH: RomSetDef = RomSetDef { name: "sfzh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzh.03d", offset: 0x0, size: 0x80000, crc: 0x6E08CBE0 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04c", offset: 0x80000, size: 0x80000, crc: 0xBB90ACD5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05c", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzh.key", offset: 0x0, size: 0x14, crc: 0x4763446F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZHR1: RomSetDef = RomSetDef { name: "sfzhr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzh.03c", offset: 0x0, size: 0x80000, crc: 0xBCE635AA },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04a", offset: 0x80000, size: 0x80000, crc: 0x5F99E9A5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzh.key", offset: 0x0, size: 0x14, crc: 0x4763446F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZB: RomSetDef = RomSetDef { name: "sfzb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzb.03g", offset: 0x0, size: 0x80000, crc: 0x348862D4 },
            Op::Load { kind: Load::Word16Swap, file: "sfzb.04e", offset: 0x80000, size: 0x80000, crc: 0x8D9B2480 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzb.key", offset: 0x0, size: 0x14, crc: 0xB0570359 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZBR1: RomSetDef = RomSetDef { name: "sfzbr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzb.03e", offset: 0x0, size: 0x80000, crc: 0xECBA89A3 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04b", offset: 0x80000, size: 0x80000, crc: 0x8B73B0E5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfzb.key", offset: 0x0, size: 0x14, crc: 0xB0570359 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA2: RomSetDef = RomSetDef { name: "sfa2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2e.03", offset: 0x0, size: 0x80000, crc: 0x1061E6BB },
            Op::Load { kind: Load::Word16Swap, file: "sz2e.04", offset: 0x80000, size: 0x80000, crc: 0x22D17B26 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05", offset: 0x100000, size: 0x80000, crc: 0x4B442A7C },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.07", offset: 0x200000, size: 0x80000, crc: 0x8E184246 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.08", offset: 0x280000, size: 0x80000, crc: 0x0FE8585D },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa2.key", offset: 0x0, size: 0x14, crc: 0x1578DCB0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA2U: RomSetDef = RomSetDef { name: "sfa2u", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2u.03a", offset: 0x0, size: 0x80000, crc: 0xD03E504F },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.04a", offset: 0x80000, size: 0x80000, crc: 0xFAE0E9C3 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.05a", offset: 0x100000, size: 0x80000, crc: 0xD02DD758 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.06", offset: 0x180000, size: 0x80000, crc: 0xC5C8EB63 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.07", offset: 0x200000, size: 0x80000, crc: 0x5DE01CC5 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.08", offset: 0x280000, size: 0x80000, crc: 0xBEA11D56 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa2u.key", offset: 0x0, size: 0x14, crc: 0x4A8D91EF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA2UR1: RomSetDef = RomSetDef { name: "sfa2ur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2u.03", offset: 0x0, size: 0x80000, crc: 0x84A09006 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.04", offset: 0x80000, size: 0x80000, crc: 0xAC46E5ED },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.05", offset: 0x100000, size: 0x80000, crc: 0x6C0C79D3 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.06", offset: 0x180000, size: 0x80000, crc: 0xC5C8EB63 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.07", offset: 0x200000, size: 0x80000, crc: 0x5DE01CC5 },
            Op::Load { kind: Load::Word16Swap, file: "sz2u.08", offset: 0x280000, size: 0x80000, crc: 0xBEA11D56 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa2u.key", offset: 0x0, size: 0x14, crc: 0x4A8D91EF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2J: RomSetDef = RomSetDef { name: "sfz2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2j.03b", offset: 0x0, size: 0x80000, crc: 0x3E1E2E85 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.04b", offset: 0x80000, size: 0x80000, crc: 0xF53D6C45 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.05b", offset: 0x100000, size: 0x80000, crc: 0xDD224156 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.06b", offset: 0x180000, size: 0x80000, crc: 0xA45A75A6 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.07b", offset: 0x200000, size: 0x80000, crc: 0x6352F038 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.08b", offset: 0x280000, size: 0x80000, crc: 0x92B66E01 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2j.key", offset: 0x0, size: 0x14, crc: 0x455BD098 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2JR1: RomSetDef = RomSetDef { name: "sfz2jr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2j.03a", offset: 0x0, size: 0x80000, crc: 0x97461E28 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.04a", offset: 0x80000, size: 0x80000, crc: 0xAE4851A9 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05a", offset: 0x100000, size: 0x80000, crc: 0x98E8E992 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.07a", offset: 0x200000, size: 0x80000, crc: 0xD910B2A2 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.08", offset: 0x280000, size: 0x80000, crc: 0x0FE8585D },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2j.key", offset: 0x0, size: 0x14, crc: 0x455BD098 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2A: RomSetDef = RomSetDef { name: "sfz2a", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2a.03a", offset: 0x0, size: 0x80000, crc: 0x30D2099F },
            Op::Load { kind: Load::Word16Swap, file: "sz2a.04a", offset: 0x80000, size: 0x80000, crc: 0x1CC94DB1 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05a", offset: 0x100000, size: 0x80000, crc: 0x98E8E992 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2a.07a", offset: 0x200000, size: 0x80000, crc: 0x0AED2494 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.08", offset: 0x280000, size: 0x80000, crc: 0x0FE8585D },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2a.key", offset: 0x0, size: 0x14, crc: 0x777B7358 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2B: RomSetDef = RomSetDef { name: "sfz2b", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2b.03b", offset: 0x0, size: 0x80000, crc: 0x1AC12812 },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.04b", offset: 0x80000, size: 0x80000, crc: 0xE4FFAF68 },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.05a", offset: 0x100000, size: 0x80000, crc: 0xDD224156 },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.06a", offset: 0x180000, size: 0x80000, crc: 0xA45A75A6 },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.07a", offset: 0x200000, size: 0x80000, crc: 0x7D19D5EC },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.08", offset: 0x280000, size: 0x80000, crc: 0x92B66E01 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2b.key", offset: 0x0, size: 0x14, crc: 0x35B1DF07 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2BR1: RomSetDef = RomSetDef { name: "sfz2br1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2b.03", offset: 0x0, size: 0x80000, crc: 0xE6CE530B },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.04", offset: 0x80000, size: 0x80000, crc: 0x1605A0CB },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05", offset: 0x100000, size: 0x80000, crc: 0x4B442A7C },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.07", offset: 0x200000, size: 0x80000, crc: 0x947E8AC6 },
            Op::Load { kind: Load::Word16Swap, file: "sz2b.08", offset: 0x280000, size: 0x80000, crc: 0x92B66E01 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2b.key", offset: 0x0, size: 0x14, crc: 0x35B1DF07 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2H: RomSetDef = RomSetDef { name: "sfz2h", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2h.03", offset: 0x0, size: 0x80000, crc: 0xBFEDDF5B },
            Op::Load { kind: Load::Word16Swap, file: "sz2h.04", offset: 0x80000, size: 0x80000, crc: 0xEA5009FB },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05", offset: 0x100000, size: 0x80000, crc: 0x4B442A7C },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2h.07", offset: 0x200000, size: 0x80000, crc: 0x947E8AC6 },
            Op::Load { kind: Load::Word16Swap, file: "sz2h.08", offset: 0x280000, size: 0x80000, crc: 0x92B66E01 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2h.key", offset: 0x0, size: 0x14, crc: 0x2719EA16 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2N: RomSetDef = RomSetDef { name: "sfz2n", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2n.03", offset: 0x0, size: 0x80000, crc: 0x58924741 },
            Op::Load { kind: Load::Word16Swap, file: "sz2n.04", offset: 0x80000, size: 0x80000, crc: 0x592A17C5 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05", offset: 0x100000, size: 0x80000, crc: 0x4B442A7C },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.07", offset: 0x200000, size: 0x80000, crc: 0x8E184246 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.08", offset: 0x280000, size: 0x80000, crc: 0x0FE8585D },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2n.key", offset: 0x0, size: 0x14, crc: 0xD1CC49D5 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2AL: RomSetDef = RomSetDef { name: "sfz2al", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "szaa.03", offset: 0x0, size: 0x80000, crc: 0x88E7023E },
            Op::Load { kind: Load::Word16Swap, file: "szaa.04", offset: 0x80000, size: 0x80000, crc: 0xAE8EC36E },
            Op::Load { kind: Load::Word16Swap, file: "szaa.05", offset: 0x100000, size: 0x80000, crc: 0xF053A55E },
            Op::Load { kind: Load::Word16Swap, file: "szaa.06", offset: 0x180000, size: 0x80000, crc: 0xCFC0E7A8 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.07", offset: 0x200000, size: 0x80000, crc: 0x5FEB8B20 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.08", offset: 0x280000, size: 0x80000, crc: 0x6EB6D412 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sza.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sza.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sza.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sza.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sza.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sza.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sza.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sza.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sza.01", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sza.02", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sza.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sza.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2al.key", offset: 0x0, size: 0x14, crc: 0x2904963E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2ALR1: RomSetDef = RomSetDef { name: "sfz2alr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "szaa.03a", offset: 0x0, size: 0x80000, crc: 0x9F67C585 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.04a", offset: 0x80000, size: 0x80000, crc: 0x5A447A54 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.05a", offset: 0x100000, size: 0x80000, crc: 0xF053A55E },
            Op::Load { kind: Load::Word16Swap, file: "szaa.06a", offset: 0x180000, size: 0x80000, crc: 0xCFC0E7A8 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.07a", offset: 0x200000, size: 0x80000, crc: 0x5FEB8B20 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.08a", offset: 0x280000, size: 0x80000, crc: 0x6EB6D412 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sza.01", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sza.02", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sza.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sza.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2al.key", offset: 0x0, size: 0x14, crc: 0x2904963E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2ALJ: RomSetDef = RomSetDef { name: "sfz2alj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "szaj.03a", offset: 0x0, size: 0x80000, crc: 0xA3802FE3 },
            Op::Load { kind: Load::Word16Swap, file: "szaj.04a", offset: 0x80000, size: 0x80000, crc: 0xE7CA87C7 },
            Op::Load { kind: Load::Word16Swap, file: "szaj.05a", offset: 0x100000, size: 0x80000, crc: 0xC88EBF88 },
            Op::Load { kind: Load::Word16Swap, file: "szaj.06a", offset: 0x180000, size: 0x80000, crc: 0x35ED5B7A },
            Op::Load { kind: Load::Word16Swap, file: "szaj.07a", offset: 0x200000, size: 0x80000, crc: 0x975DCB3E },
            Op::Load { kind: Load::Word16Swap, file: "szaj.08a", offset: 0x280000, size: 0x80000, crc: 0xDC73F2D7 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sza.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sza.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sza.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sza.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sza.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sza.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sza.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sza.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sza.01", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sza.02", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sza.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sza.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2alj.key", offset: 0x0, size: 0x14, crc: 0x4C42320F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2ALH: RomSetDef = RomSetDef { name: "sfz2alh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "szah.03", offset: 0x0, size: 0x80000, crc: 0x06F93D1D },
            Op::Load { kind: Load::Word16Swap, file: "szah.04", offset: 0x80000, size: 0x80000, crc: 0xE62EE914 },
            Op::Load { kind: Load::Word16Swap, file: "szah.05", offset: 0x100000, size: 0x80000, crc: 0x2B7F4B20 },
            Op::Load { kind: Load::Word16Swap, file: "sza.06", offset: 0x180000, size: 0x80000, crc: 0x0ABDA2FC },
            Op::Load { kind: Load::Word16Swap, file: "sza.07", offset: 0x200000, size: 0x80000, crc: 0xE9430762 },
            Op::Load { kind: Load::Word16Swap, file: "sza.08", offset: 0x280000, size: 0x80000, crc: 0xB65711A9 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sza.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sza.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sza.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sza.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sza.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sza.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sza.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sza.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sza.01", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sza.02", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sza.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sza.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2alh.key", offset: 0x0, size: 0x14, crc: 0xF320F655 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2ALB: RomSetDef = RomSetDef { name: "sfz2alb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "szab.03", offset: 0x0, size: 0x80000, crc: 0xCB436ECA },
            Op::Load { kind: Load::Word16Swap, file: "szab.04", offset: 0x80000, size: 0x80000, crc: 0x14534BEA },
            Op::Load { kind: Load::Word16Swap, file: "szab.05", offset: 0x100000, size: 0x80000, crc: 0x7FB10658 },
            Op::Load { kind: Load::Word16Swap, file: "sza.06", offset: 0x180000, size: 0x80000, crc: 0x0ABDA2FC },
            Op::Load { kind: Load::Word16Swap, file: "sza.07", offset: 0x200000, size: 0x80000, crc: 0xE9430762 },
            Op::Load { kind: Load::Word16Swap, file: "sza.08", offset: 0x280000, size: 0x80000, crc: 0xB65711A9 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sza.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sza.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sza.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sza.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sza.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sza.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sza.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sza.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sza.01", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sza.02", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sza.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sza.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz2alb.key", offset: 0x0, size: 0x14, crc: 0xC8B3AC73 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3: RomSetDef = RomSetDef { name: "sfa3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3e.03c", offset: 0x0, size: 0x80000, crc: 0x9762B206 },
            Op::Load { kind: Load::Word16Swap, file: "sz3e.04c", offset: 0x80000, size: 0x80000, crc: 0x5AD3F721 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05c", offset: 0x100000, size: 0x80000, crc: 0x57FD0A40 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06c", offset: 0x180000, size: 0x80000, crc: 0xF6305F8B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07c", offset: 0x200000, size: 0x80000, crc: 0x6EAB0F6F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08c", offset: 0x280000, size: 0x80000, crc: 0x910C4A3B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09c", offset: 0x300000, size: 0x80000, crc: 0xB29E5199 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10b", offset: 0x380000, size: 0x80000, crc: 0xDEB2FF52 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3.key", offset: 0x0, size: 0x14, crc: 0x54FA39C6 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3U: RomSetDef = RomSetDef { name: "sfa3u", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3u.03c", offset: 0x0, size: 0x80000, crc: 0xE007DA2E },
            Op::Load { kind: Load::Word16Swap, file: "sz3u.04c", offset: 0x80000, size: 0x80000, crc: 0x5F78F0E7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05c", offset: 0x100000, size: 0x80000, crc: 0x57FD0A40 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06c", offset: 0x180000, size: 0x80000, crc: 0xF6305F8B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07c", offset: 0x200000, size: 0x80000, crc: 0x6EAB0F6F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08c", offset: 0x280000, size: 0x80000, crc: 0x910C4A3B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09c", offset: 0x300000, size: 0x80000, crc: 0xB29E5199 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10b", offset: 0x380000, size: 0x80000, crc: 0xDEB2FF52 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3u.key", offset: 0x0, size: 0x14, crc: 0x4A8F98C1 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3UR1: RomSetDef = RomSetDef { name: "sfa3ur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3u.03", offset: 0x0, size: 0x80000, crc: 0xB5984A19 },
            Op::Load { kind: Load::Word16Swap, file: "sz3u.04", offset: 0x80000, size: 0x80000, crc: 0x7E8158BA },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05", offset: 0x100000, size: 0x80000, crc: 0x9B21518A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06", offset: 0x180000, size: 0x80000, crc: 0xE7A6C3A7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07", offset: 0x200000, size: 0x80000, crc: 0xEC4C0CFD },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08", offset: 0x280000, size: 0x80000, crc: 0x5C7E7240 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09", offset: 0x300000, size: 0x80000, crc: 0xC5589553 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3u.key", offset: 0x0, size: 0x14, crc: 0x4A8F98C1 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3US: RomSetDef = RomSetDef { name: "sfa3us", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_03.6a", offset: 0x0, size: 0x80000, crc: 0x14319E29 },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_04.7a", offset: 0x80000, size: 0x80000, crc: 0x65FBC272 },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_05.8a", offset: 0x100000, size: 0x80000, crc: 0xE93C47D1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_06.9a", offset: 0x180000, size: 0x80000, crc: 0x1BF09DE3 },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_07.6d", offset: 0x200000, size: 0x80000, crc: 0xF6296D96 },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_08.7d", offset: 0x280000, size: 0x80000, crc: 0x1F4008FF },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_09.8d", offset: 0x300000, size: 0x80000, crc: 0x822FC451 },
            Op::Load { kind: Load::Word16Swap, file: "sz3-usam_10.9d", offset: 0x380000, size: 0x80000, crc: 0x92713468 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3-usam_01.1a", offset: 0x0, size: 0x8000, crc: 0xC180947D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3-usam_02.2a", offset: 0x28000, size: 0x20000, crc: 0x9EBC280F },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3u.key", offset: 0x0, size: 0x14, crc: 0x4A8F98C1 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ3J: RomSetDef = RomSetDef { name: "sfz3j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3j.03c", offset: 0x0, size: 0x80000, crc: 0xCADF4A51 },
            Op::Load { kind: Load::Word16Swap, file: "sz3j.04c", offset: 0x80000, size: 0x80000, crc: 0xFCB31228 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05c", offset: 0x100000, size: 0x80000, crc: 0x57FD0A40 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06c", offset: 0x180000, size: 0x80000, crc: 0xF6305F8B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07c", offset: 0x200000, size: 0x80000, crc: 0x6EAB0F6F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08c", offset: 0x280000, size: 0x80000, crc: 0x910C4A3B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09c", offset: 0x300000, size: 0x80000, crc: 0xB29E5199 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10b", offset: 0x380000, size: 0x80000, crc: 0xDEB2FF52 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz3j.key", offset: 0x0, size: 0x14, crc: 0xD30CCA8D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ3JR1: RomSetDef = RomSetDef { name: "sfz3jr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3j.03a", offset: 0x0, size: 0x80000, crc: 0x6EE0BEAE },
            Op::Load { kind: Load::Word16Swap, file: "sz3j.04a", offset: 0x80000, size: 0x80000, crc: 0xA6E2978D },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05a", offset: 0x100000, size: 0x80000, crc: 0x05964B7D },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06a", offset: 0x180000, size: 0x80000, crc: 0x78CE2179 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07a", offset: 0x200000, size: 0x80000, crc: 0x398BF52F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08a", offset: 0x280000, size: 0x80000, crc: 0x866D0588 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09a", offset: 0x300000, size: 0x80000, crc: 0x2180892C },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz3j.key", offset: 0x0, size: 0x14, crc: 0xD30CCA8D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ3JR2: RomSetDef = RomSetDef { name: "sfz3jr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3j.03", offset: 0x0, size: 0x80000, crc: 0xF7CB4B13 },
            Op::Load { kind: Load::Word16Swap, file: "sz3j.04", offset: 0x80000, size: 0x80000, crc: 0x0846C29D },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05", offset: 0x100000, size: 0x80000, crc: 0x9B21518A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06", offset: 0x180000, size: 0x80000, crc: 0xE7A6C3A7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07", offset: 0x200000, size: 0x80000, crc: 0xEC4C0CFD },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08", offset: 0x280000, size: 0x80000, crc: 0x5C7E7240 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09", offset: 0x300000, size: 0x80000, crc: 0xC5589553 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz3j.key", offset: 0x0, size: 0x14, crc: 0xD30CCA8D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ3A: RomSetDef = RomSetDef { name: "sfz3a", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3a.03d", offset: 0x0, size: 0x80000, crc: 0xD7E140D6 },
            Op::Load { kind: Load::Word16Swap, file: "sz3a.04d", offset: 0x80000, size: 0x80000, crc: 0xE06869A2 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05c", offset: 0x100000, size: 0x80000, crc: 0x57FD0A40 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06c", offset: 0x180000, size: 0x80000, crc: 0xF6305F8B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07c", offset: 0x200000, size: 0x80000, crc: 0x6EAB0F6F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08c", offset: 0x280000, size: 0x80000, crc: 0x910C4A3B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09c", offset: 0x300000, size: 0x80000, crc: 0xB29E5199 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10b", offset: 0x380000, size: 0x80000, crc: 0xDEB2FF52 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz3a.key", offset: 0x0, size: 0x14, crc: 0x09045D61 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ3AR1: RomSetDef = RomSetDef { name: "sfz3ar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3a.03a", offset: 0x0, size: 0x80000, crc: 0x29C681FD },
            Op::Load { kind: Load::Word16Swap, file: "sz3a.04", offset: 0x80000, size: 0x80000, crc: 0x9DDD1484 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05", offset: 0x100000, size: 0x80000, crc: 0x9B21518A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06", offset: 0x180000, size: 0x80000, crc: 0xE7A6C3A7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07", offset: 0x200000, size: 0x80000, crc: 0xEC4C0CFD },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08", offset: 0x280000, size: 0x80000, crc: 0x5C7E7240 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09", offset: 0x300000, size: 0x80000, crc: 0xC5589553 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz3a.key", offset: 0x0, size: 0x14, crc: 0x09045D61 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3H: RomSetDef = RomSetDef { name: "sfa3h", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3h.03c", offset: 0x0, size: 0x80000, crc: 0xB3B563A3 },
            Op::Load { kind: Load::Word16Swap, file: "sz3h.04c", offset: 0x80000, size: 0x80000, crc: 0x47891FEC },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05c", offset: 0x100000, size: 0x80000, crc: 0x57FD0A40 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06c", offset: 0x180000, size: 0x80000, crc: 0xF6305F8B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07c", offset: 0x200000, size: 0x80000, crc: 0x6EAB0F6F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08c", offset: 0x280000, size: 0x80000, crc: 0x910C4A3B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09c", offset: 0x300000, size: 0x80000, crc: 0xB29E5199 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10b", offset: 0x380000, size: 0x80000, crc: 0xDEB2FF52 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3h.key", offset: 0x0, size: 0x14, crc: 0x1B34998C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3HR1: RomSetDef = RomSetDef { name: "sfa3hr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3h.03", offset: 0x0, size: 0x80000, crc: 0x4B16CB3E },
            Op::Load { kind: Load::Word16Swap, file: "sz3h.04", offset: 0x80000, size: 0x80000, crc: 0x88AD2E6A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05", offset: 0x100000, size: 0x80000, crc: 0x9B21518A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06", offset: 0x180000, size: 0x80000, crc: 0xE7A6C3A7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07", offset: 0x200000, size: 0x80000, crc: 0xEC4C0CFD },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08", offset: 0x280000, size: 0x80000, crc: 0x5C7E7240 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09", offset: 0x300000, size: 0x80000, crc: 0xC5589553 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3h.key", offset: 0x0, size: 0x14, crc: 0x1B34998C },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFA3B: RomSetDef = RomSetDef { name: "sfa3b", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3b.03", offset: 0x0, size: 0x80000, crc: 0x046C9B4D },
            Op::Load { kind: Load::Word16Swap, file: "sz3b.04", offset: 0x80000, size: 0x80000, crc: 0xDA211919 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05", offset: 0x100000, size: 0x80000, crc: 0x9B21518A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06", offset: 0x180000, size: 0x80000, crc: 0xE7A6C3A7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07", offset: 0x200000, size: 0x80000, crc: 0xEC4C0CFD },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08", offset: 0x280000, size: 0x80000, crc: 0x5C7E7240 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09", offset: 0x300000, size: 0x80000, crc: 0xC5589553 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfa3b.key", offset: 0x0, size: 0x14, crc: 0x2D0A1351 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SGEMF: RomSetDef = RomSetDef { name: "sgemf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcfu.03", offset: 0x0, size: 0x80000, crc: 0xAC2E8566 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.04", offset: 0x80000, size: 0x80000, crc: 0xF4314C96 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.05", offset: 0x100000, size: 0x80000, crc: 0x215655F6 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.06", offset: 0x180000, size: 0x80000, crc: 0xEA6F13EA },
            Op::Load { kind: Load::Word16Swap, file: "pcf.07", offset: 0x200000, size: 0x80000, crc: 0x5AC6D5EA },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pcf.13m", offset: 0x0, size: 0x400000, crc: 0x22D72AB9 },
            Op::Load { kind: Load::Word64, file: "pcf.15m", offset: 0x2, size: 0x400000, crc: 0x16A4813C },
            Op::Load { kind: Load::Word64, file: "pcf.17m", offset: 0x4, size: 0x400000, crc: 0x1097E035 },
            Op::Load { kind: Load::Word64, file: "pcf.19m", offset: 0x6, size: 0x400000, crc: 0xD362D874 },
            Op::Load { kind: Load::Word64, file: "pcf.14m", offset: 0x1000000, size: 0x100000, crc: 0x0383897C },
            Op::Load { kind: Load::Word64, file: "pcf.16m", offset: 0x1000002, size: 0x100000, crc: 0x76F91084 },
            Op::Load { kind: Load::Word64, file: "pcf.18m", offset: 0x1000004, size: 0x100000, crc: 0x756C3754 },
            Op::Load { kind: Load::Word64, file: "pcf.20m", offset: 0x1000006, size: 0x100000, crc: 0x9EC9277D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pcf.01", offset: 0x0, size: 0x8000, crc: 0x254E5F33 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pcf.02", offset: 0x28000, size: 0x20000, crc: 0x6902F4F9 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcf.11m", offset: 0x0, size: 0x400000, crc: 0xA5DEA005 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.12m", offset: 0x400000, size: 0x400000, crc: 0x4CE235FE },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sgemf.key", offset: 0x0, size: 0x14, crc: 0x3D604021 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PFGHTJ: RomSetDef = RomSetDef { name: "pfghtj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcfj.03", offset: 0x0, size: 0x80000, crc: 0x681DA43E },
            Op::Load { kind: Load::Word16Swap, file: "pcf.04", offset: 0x80000, size: 0x80000, crc: 0xF4314C96 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.05", offset: 0x100000, size: 0x80000, crc: 0x215655F6 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.06", offset: 0x180000, size: 0x80000, crc: 0xEA6F13EA },
            Op::Load { kind: Load::Word16Swap, file: "pcf.07", offset: 0x200000, size: 0x80000, crc: 0x5AC6D5EA },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pcf.13m", offset: 0x0, size: 0x400000, crc: 0x22D72AB9 },
            Op::Load { kind: Load::Word64, file: "pcf.15m", offset: 0x2, size: 0x400000, crc: 0x16A4813C },
            Op::Load { kind: Load::Word64, file: "pcf.17m", offset: 0x4, size: 0x400000, crc: 0x1097E035 },
            Op::Load { kind: Load::Word64, file: "pcf.19m", offset: 0x6, size: 0x400000, crc: 0xD362D874 },
            Op::Load { kind: Load::Word64, file: "pcf.14m", offset: 0x1000000, size: 0x100000, crc: 0x0383897C },
            Op::Load { kind: Load::Word64, file: "pcf.16m", offset: 0x1000002, size: 0x100000, crc: 0x76F91084 },
            Op::Load { kind: Load::Word64, file: "pcf.18m", offset: 0x1000004, size: 0x100000, crc: 0x756C3754 },
            Op::Load { kind: Load::Word64, file: "pcf.20m", offset: 0x1000006, size: 0x100000, crc: 0x9EC9277D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pcf.01", offset: 0x0, size: 0x8000, crc: 0x254E5F33 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pcf.02", offset: 0x28000, size: 0x20000, crc: 0x6902F4F9 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcf.11m", offset: 0x0, size: 0x400000, crc: 0xA5DEA005 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.12m", offset: 0x400000, size: 0x400000, crc: 0x4CE235FE },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pfghtj.key", offset: 0x0, size: 0x14, crc: 0x62297638 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SGEMFA: RomSetDef = RomSetDef { name: "sgemfa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcfa.03", offset: 0x0, size: 0x80000, crc: 0xE17C089A },
            Op::Load { kind: Load::Word16Swap, file: "pcf.04", offset: 0x80000, size: 0x80000, crc: 0xF4314C96 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.05", offset: 0x100000, size: 0x80000, crc: 0x215655F6 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.06", offset: 0x180000, size: 0x80000, crc: 0xEA6F13EA },
            Op::Load { kind: Load::Word16Swap, file: "pcf.07", offset: 0x200000, size: 0x80000, crc: 0x5AC6D5EA },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pcf.13m", offset: 0x0, size: 0x400000, crc: 0x22D72AB9 },
            Op::Load { kind: Load::Word64, file: "pcf.15m", offset: 0x2, size: 0x400000, crc: 0x16A4813C },
            Op::Load { kind: Load::Word64, file: "pcf.17m", offset: 0x4, size: 0x400000, crc: 0x1097E035 },
            Op::Load { kind: Load::Word64, file: "pcf.19m", offset: 0x6, size: 0x400000, crc: 0xD362D874 },
            Op::Load { kind: Load::Word64, file: "pcf.14m", offset: 0x1000000, size: 0x100000, crc: 0x0383897C },
            Op::Load { kind: Load::Word64, file: "pcf.16m", offset: 0x1000002, size: 0x100000, crc: 0x76F91084 },
            Op::Load { kind: Load::Word64, file: "pcf.18m", offset: 0x1000004, size: 0x100000, crc: 0x756C3754 },
            Op::Load { kind: Load::Word64, file: "pcf.20m", offset: 0x1000006, size: 0x100000, crc: 0x9EC9277D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pcf.01", offset: 0x0, size: 0x8000, crc: 0x254E5F33 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pcf.02", offset: 0x28000, size: 0x20000, crc: 0x6902F4F9 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcf.11m", offset: 0x0, size: 0x400000, crc: 0xA5DEA005 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.12m", offset: 0x400000, size: 0x400000, crc: 0x4CE235FE },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sgemfa.key", offset: 0x0, size: 0x14, crc: 0xDD513738 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SGEMFH: RomSetDef = RomSetDef { name: "sgemfh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcfh.03", offset: 0x0, size: 0x80000, crc: 0xE9103347 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.04", offset: 0x80000, size: 0x80000, crc: 0xF4314C96 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.05", offset: 0x100000, size: 0x80000, crc: 0x215655F6 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.06", offset: 0x180000, size: 0x80000, crc: 0xEA6F13EA },
            Op::Load { kind: Load::Word16Swap, file: "pcf.07", offset: 0x200000, size: 0x80000, crc: 0x5AC6D5EA },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pcf.13m", offset: 0x0, size: 0x400000, crc: 0x22D72AB9 },
            Op::Load { kind: Load::Word64, file: "pcf.15m", offset: 0x2, size: 0x400000, crc: 0x16A4813C },
            Op::Load { kind: Load::Word64, file: "pcf.17m", offset: 0x4, size: 0x400000, crc: 0x1097E035 },
            Op::Load { kind: Load::Word64, file: "pcf.19m", offset: 0x6, size: 0x400000, crc: 0xD362D874 },
            Op::Load { kind: Load::Word64, file: "pcf.14m", offset: 0x1000000, size: 0x100000, crc: 0x0383897C },
            Op::Load { kind: Load::Word64, file: "pcf.16m", offset: 0x1000002, size: 0x100000, crc: 0x76F91084 },
            Op::Load { kind: Load::Word64, file: "pcf.18m", offset: 0x1000004, size: 0x100000, crc: 0x756C3754 },
            Op::Load { kind: Load::Word64, file: "pcf.20m", offset: 0x1000006, size: 0x100000, crc: 0x9EC9277D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pcf.01", offset: 0x0, size: 0x8000, crc: 0x254E5F33 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pcf.02", offset: 0x28000, size: 0x20000, crc: 0x6902F4F9 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcf.11m", offset: 0x0, size: 0x400000, crc: 0xA5DEA005 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.12m", offset: 0x400000, size: 0x400000, crc: 0x4CE235FE },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sgemfh.key", offset: 0x0, size: 0x14, crc: 0xF97F4B7D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SPF2T: RomSetDef = RomSetDef { name: "spf2t", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfe.03", offset: 0x0, size: 0x80000, crc: 0x2AF51954 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "spf2t.key", offset: 0x0, size: 0x14, crc: 0x4C4DC7E3 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SPF2TU: RomSetDef = RomSetDef { name: "spf2tu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfu.03a", offset: 0x0, size: 0x80000, crc: 0x346E62EF },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "spf2tu.key", offset: 0x0, size: 0x14, crc: 0x5D7B15E8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SPF2XJ: RomSetDef = RomSetDef { name: "spf2xj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfj.03a", offset: 0x0, size: 0x80000, crc: 0x2070554A },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "spf2xj.key", offset: 0x0, size: 0x14, crc: 0xDC39FD34 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SPF2TA: RomSetDef = RomSetDef { name: "spf2ta", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfa.03", offset: 0x0, size: 0x80000, crc: 0x3CECFA78 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "spf2ta.key", offset: 0x0, size: 0x14, crc: 0x61E93A18 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SPF2TH: RomSetDef = RomSetDef { name: "spf2th", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfh.03", offset: 0x0, size: 0x80000, crc: 0x20510F2D },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "spf2th.key", offset: 0x0, size: 0x14, crc: 0x292DB449 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2: RomSetDef = RomSetDef { name: "ssf2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfe-03b", offset: 0x0, size: 0x80000, crc: 0xAF654792 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.04", offset: 0x80000, size: 0x80000, crc: 0xB082AA67 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.05", offset: 0x100000, size: 0x80000, crc: 0x02B9C137 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe-06b", offset: 0x180000, size: 0x80000, crc: 0x1C8E44A8 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.07", offset: 0x200000, size: 0x80000, crc: 0x2409001D },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf-01a", offset: 0x0, size: 0x8000, crc: 0x71FCDFC9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2.key", offset: 0x0, size: 0x14, crc: 0xE469CCBB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2R1: RomSetDef = RomSetDef { name: "ssf2r1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfe.03", offset: 0x0, size: 0x80000, crc: 0xA597745D },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.04", offset: 0x80000, size: 0x80000, crc: 0xB082AA67 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.05", offset: 0x100000, size: 0x80000, crc: 0x02B9C137 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.06", offset: 0x180000, size: 0x80000, crc: 0x70D470C5 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.07", offset: 0x200000, size: 0x80000, crc: 0x2409001D },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2.key", offset: 0x0, size: 0x14, crc: 0xE469CCBB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2U: RomSetDef = RomSetDef { name: "ssf2u", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfu.03a", offset: 0x0, size: 0x80000, crc: 0x72F29C33 },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.04a", offset: 0x80000, size: 0x80000, crc: 0x935CEA44 },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.05", offset: 0x100000, size: 0x80000, crc: 0xA0ACB28A },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.06", offset: 0x180000, size: 0x80000, crc: 0x47413DCF },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.07", offset: 0x200000, size: 0x80000, crc: 0xE6066077 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2u.key", offset: 0x0, size: 0x14, crc: 0x2F4F8E9D },
        ] },
] };
/// Régions ignorées : plds
pub const RS2_SSF2US2: RomSetDef = RomSetDef { name: "ssf2us2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfu.03a", offset: 0x0, size: 0x80000, crc: 0x72F29C33 },
            Op::Load { kind: Load::Word16Swap, file: "super stf 2 super ii rom-08 usa", offset: 0x0, size: 0x80000, crc: 0xD48D35C9 },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.04a", offset: 0x80000, size: 0x80000, crc: 0x935CEA44 },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.05", offset: 0x100000, size: 0x80000, crc: 0xA0ACB28A },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.06", offset: 0x180000, size: 0x80000, crc: 0x47413DCF },
            Op::Load { kind: Load::Word16Swap, file: "super stf 2 super ii rom-07 usa", offset: 0x200000, size: 0x80000, crc: 0x460DF588 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2u.key", offset: 0x0, size: 0x14, crc: 0x2F4F8E9D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2A: RomSetDef = RomSetDef { name: "ssf2a", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfa.03b", offset: 0x0, size: 0x80000, crc: 0x83A059BF },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.04a", offset: 0x80000, size: 0x80000, crc: 0x5D873642 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.05", offset: 0x100000, size: 0x80000, crc: 0xF8FB4DE2 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.06b", offset: 0x180000, size: 0x80000, crc: 0x3185D19D },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.07", offset: 0x200000, size: 0x80000, crc: 0x36E29217 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2a.key", offset: 0x0, size: 0x14, crc: 0x5FB6013F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2AR1: RomSetDef = RomSetDef { name: "ssf2ar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfa.03a", offset: 0x0, size: 0x80000, crc: 0xD2A3C520 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.04a", offset: 0x80000, size: 0x80000, crc: 0x5D873642 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.05", offset: 0x100000, size: 0x80000, crc: 0xF8FB4DE2 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.06", offset: 0x180000, size: 0x80000, crc: 0xAA8ACEE7 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.07", offset: 0x200000, size: 0x80000, crc: 0x36E29217 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2a.key", offset: 0x0, size: 0x14, crc: 0x5FB6013F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2J: RomSetDef = RomSetDef { name: "ssf2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfj.03b", offset: 0x0, size: 0x80000, crc: 0x5C2E356D },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.04a", offset: 0x80000, size: 0x80000, crc: 0x013BD55C },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.05", offset: 0x100000, size: 0x80000, crc: 0x0918D19A },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.06b", offset: 0x180000, size: 0x80000, crc: 0x014E0C6D },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.07", offset: 0x200000, size: 0x80000, crc: 0xEB6A9B1B },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2j.key", offset: 0x0, size: 0x14, crc: 0xBCA45CC2 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2JR1: RomSetDef = RomSetDef { name: "ssf2jr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfj.03a", offset: 0x0, size: 0x80000, crc: 0x0BBF1304 },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.04a", offset: 0x80000, size: 0x80000, crc: 0x013BD55C },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.05", offset: 0x100000, size: 0x80000, crc: 0x0918D19A },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.06", offset: 0x180000, size: 0x80000, crc: 0xD808A6CD },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.07", offset: 0x200000, size: 0x80000, crc: 0xEB6A9B1B },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2j.key", offset: 0x0, size: 0x14, crc: 0xBCA45CC2 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2JR2: RomSetDef = RomSetDef { name: "ssf2jr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfj.03", offset: 0x0, size: 0x80000, crc: 0x7EB0EFED },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.04", offset: 0x80000, size: 0x80000, crc: 0xD7322164 },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.05", offset: 0x100000, size: 0x80000, crc: 0x0918D19A },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.06", offset: 0x180000, size: 0x80000, crc: 0xD808A6CD },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.07", offset: 0x200000, size: 0x80000, crc: 0xEB6A9B1B },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2j.key", offset: 0x0, size: 0x14, crc: 0xBCA45CC2 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2H: RomSetDef = RomSetDef { name: "ssf2h", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfh.03", offset: 0x0, size: 0x80000, crc: 0xB086B355 },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.04", offset: 0x80000, size: 0x80000, crc: 0x1E629B29 },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.05", offset: 0x100000, size: 0x80000, crc: 0xB5997E10 },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.06", offset: 0x180000, size: 0x80000, crc: 0x793B8FAD },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.07", offset: 0x200000, size: 0x80000, crc: 0xCBB92AC3 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2h.key", offset: 0x0, size: 0x14, crc: 0x8331BC8E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TB: RomSetDef = RomSetDef { name: "ssf2tb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfe.03tc", offset: 0x0, size: 0x80000, crc: 0x496A8409 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.04tc", offset: 0x80000, size: 0x80000, crc: 0x4B45C18B },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.05t", offset: 0x100000, size: 0x80000, crc: 0x6A9C6444 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.06tb", offset: 0x180000, size: 0x80000, crc: 0xE4944FC3 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.07t", offset: 0x200000, size: 0x80000, crc: 0x2C9F4782 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tb.key", offset: 0x0, size: 0x14, crc: 0x1ECC92B2 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBR1: RomSetDef = RomSetDef { name: "ssf2tbr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfe.03t", offset: 0x0, size: 0x80000, crc: 0x1E018E34 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.04t", offset: 0x80000, size: 0x80000, crc: 0xAC92EFAF },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.05t", offset: 0x100000, size: 0x80000, crc: 0x6A9C6444 },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.06t", offset: 0x180000, size: 0x80000, crc: 0xF442562B },
            Op::Load { kind: Load::Word16Swap, file: "ssfe.07t", offset: 0x200000, size: 0x80000, crc: 0x2C9F4782 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tb.key", offset: 0x0, size: 0x14, crc: 0x1ECC92B2 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBU: RomSetDef = RomSetDef { name: "ssf2tbu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfu.03t", offset: 0x0, size: 0x80000, crc: 0x4635B35E },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.04t", offset: 0x80000, size: 0x80000, crc: 0x2630B3DD },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.05t", offset: 0x100000, size: 0x80000, crc: 0xC0293C97 },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.06t", offset: 0x180000, size: 0x80000, crc: 0xAAA33888 },
            Op::Load { kind: Load::Word16Swap, file: "ssfu.07t", offset: 0x200000, size: 0x80000, crc: 0x44C54534 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tbu.key", offset: 0x0, size: 0x14, crc: 0x1DA3CCF3 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBJ: RomSetDef = RomSetDef { name: "ssf2tbj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssftj.03b", offset: 0x0, size: 0x80000, crc: 0xE78A3433 },
            Op::Load { kind: Load::Word16Swap, file: "ssftj.04t", offset: 0x80000, size: 0x80000, crc: 0xB4DC1906 },
            Op::Load { kind: Load::Word16Swap, file: "ssftj.05", offset: 0x100000, size: 0x80000, crc: 0xA7E35FBC },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.06tb", offset: 0x180000, size: 0x80000, crc: 0x0737C30D },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.07t", offset: 0x200000, size: 0x80000, crc: 0x1F239515 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tbj.key", offset: 0x0, size: 0x14, crc: 0xBCC2E017 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBJ1: RomSetDef = RomSetDef { name: "ssf2tbj1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfj.03t", offset: 0x0, size: 0x80000, crc: 0x980D4450 },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.04t", offset: 0x80000, size: 0x80000, crc: 0xB4DC1906 },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.05t", offset: 0x100000, size: 0x80000, crc: 0xA7E35FBC },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.06t", offset: 0x180000, size: 0x80000, crc: 0xCB592B30 },
            Op::Load { kind: Load::Word16Swap, file: "ssfj.07t", offset: 0x200000, size: 0x80000, crc: 0x1F239515 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tbj.key", offset: 0x0, size: 0x14, crc: 0xBCC2E017 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBA: RomSetDef = RomSetDef { name: "ssf2tba", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfa.03tb", offset: 0x0, size: 0x80000, crc: 0x8DE631D2 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.04ta", offset: 0x80000, size: 0x80000, crc: 0xABEF3042 },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.05t", offset: 0x100000, size: 0x80000, crc: 0xEDFA018F },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.06tb", offset: 0x180000, size: 0x80000, crc: 0x2B9D1DBC },
            Op::Load { kind: Load::Word16Swap, file: "ssfa.07t", offset: 0x200000, size: 0x80000, crc: 0xF4A25159 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf-01a", offset: 0x0, size: 0x8000, crc: 0x71FCDFC9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tba.key", offset: 0x0, size: 0x14, crc: 0x8D2740ED },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBH: RomSetDef = RomSetDef { name: "ssf2tbh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfh.03tb", offset: 0x0, size: 0x80000, crc: 0x6DB7D28B },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.04t", offset: 0x80000, size: 0x80000, crc: 0x0FE7D895 },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.05t", offset: 0x100000, size: 0x80000, crc: 0x41BE4F2D },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.06tb", offset: 0x180000, size: 0x80000, crc: 0xD2522EB1 },
            Op::Load { kind: Load::Word16Swap, file: "ssfh.07t", offset: 0x200000, size: 0x80000, crc: 0xB1C3A3C6 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tbh.key", offset: 0x0, size: 0x14, crc: 0xFDDECF4F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2T: RomSetDef = RomSetDef { name: "ssf2t", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxe.03c", offset: 0x0, size: 0x80000, crc: 0x2FA1F396 },
            Op::Load { kind: Load::Word16Swap, file: "sfxe.04a", offset: 0x80000, size: 0x80000, crc: 0xD0BC29C6 },
            Op::Load { kind: Load::Word16Swap, file: "sfxe.05", offset: 0x100000, size: 0x80000, crc: 0x65222964 },
            Op::Load { kind: Load::Word16Swap, file: "sfxe.06a", offset: 0x180000, size: 0x80000, crc: 0x8FE9F531 },
            Op::Load { kind: Load::Word16Swap, file: "sfxe.07", offset: 0x200000, size: 0x80000, crc: 0x8A7D0CB6 },
            Op::Load { kind: Load::Word16Swap, file: "sfxe.08", offset: 0x280000, size: 0x80000, crc: 0x74C24062 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2t.key", offset: 0x0, size: 0x14, crc: 0x524D608E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TU: RomSetDef = RomSetDef { name: "ssf2tu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxu.03e", offset: 0x0, size: 0x80000, crc: 0xD6FF689E },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.04a", offset: 0x80000, size: 0x80000, crc: 0x532B5FFD },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.05", offset: 0x100000, size: 0x80000, crc: 0xFFA3C6DE },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.06b", offset: 0x180000, size: 0x80000, crc: 0x83F9382B },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.07a", offset: 0x200000, size: 0x80000, crc: 0x6AB673E8 },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.08", offset: 0x280000, size: 0x80000, crc: 0xB3C71810 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tu.key", offset: 0x0, size: 0x14, crc: 0xF7D62DEF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TUR1: RomSetDef = RomSetDef { name: "ssf2tur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxu.03c", offset: 0x0, size: 0x80000, crc: 0x86E4A335 },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.04a", offset: 0x80000, size: 0x80000, crc: 0x532B5FFD },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.05", offset: 0x100000, size: 0x80000, crc: 0xFFA3C6DE },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.06a", offset: 0x180000, size: 0x80000, crc: 0xE4C04C99 },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.07", offset: 0x200000, size: 0x80000, crc: 0xD8199E41 },
            Op::Load { kind: Load::Word16Swap, file: "sfxu.08", offset: 0x280000, size: 0x80000, crc: 0xB3C71810 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2tu.key", offset: 0x0, size: 0x14, crc: 0xF7D62DEF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TA: RomSetDef = RomSetDef { name: "ssf2ta", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxa.03c", offset: 0x0, size: 0x80000, crc: 0x04B9FF34 },
            Op::Load { kind: Load::Word16Swap, file: "sfxa.04a", offset: 0x80000, size: 0x80000, crc: 0x16EA5F7A },
            Op::Load { kind: Load::Word16Swap, file: "sfxa.05", offset: 0x100000, size: 0x80000, crc: 0x53D61F0C },
            Op::Load { kind: Load::Word16Swap, file: "sfxa.06a", offset: 0x180000, size: 0x80000, crc: 0x066D09B5 },
            Op::Load { kind: Load::Word16Swap, file: "sfxa.07", offset: 0x200000, size: 0x80000, crc: 0xA428257B },
            Op::Load { kind: Load::Word16Swap, file: "sfxa.08", offset: 0x280000, size: 0x80000, crc: 0x39BE596C },
            Op::Load { kind: Load::Word16Swap, file: "sfx.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2ta.key", offset: 0x0, size: 0x14, crc: 0xC11FA8E9 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TH: RomSetDef = RomSetDef { name: "ssf2th", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxh.03c", offset: 0x0, size: 0x80000, crc: 0xFBE80DFE },
            Op::Load { kind: Load::Word16Swap, file: "sfxh.04a", offset: 0x80000, size: 0x80000, crc: 0xEF9DD4B1 },
            Op::Load { kind: Load::Word16Swap, file: "sfxh.05", offset: 0x100000, size: 0x80000, crc: 0x09E56ECC },
            Op::Load { kind: Load::Word16Swap, file: "sfxh.06a", offset: 0x180000, size: 0x80000, crc: 0xE6F210BE },
            Op::Load { kind: Load::Word16Swap, file: "sfxh.07", offset: 0x200000, size: 0x80000, crc: 0x900BA1A4 },
            Op::Load { kind: Load::Word16Swap, file: "sfxh.08", offset: 0x280000, size: 0x80000, crc: 0xC15F0424 },
            Op::Load { kind: Load::Word16Swap, file: "sfxh.09", offset: 0x300000, size: 0x80000, crc: 0x5B92B3F9 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2th.key", offset: 0x0, size: 0x14, crc: 0xF6CE6A35 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2XJ: RomSetDef = RomSetDef { name: "ssf2xj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxj.03d", offset: 0x0, size: 0x80000, crc: 0x50B52B37 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.04a", offset: 0x80000, size: 0x80000, crc: 0xAF7767B4 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.05", offset: 0x100000, size: 0x80000, crc: 0xF4FF18F5 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.06b", offset: 0x180000, size: 0x80000, crc: 0x413477C2 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.07a", offset: 0x200000, size: 0x80000, crc: 0xA18B3D83 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.08", offset: 0x280000, size: 0x80000, crc: 0x2DE76F10 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2xj.key", offset: 0x0, size: 0x14, crc: 0x160D1424 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2XJR1: RomSetDef = RomSetDef { name: "ssf2xjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxj.03c", offset: 0x0, size: 0x80000, crc: 0xA7417B79 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.04a", offset: 0x80000, size: 0x80000, crc: 0xAF7767B4 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.05", offset: 0x100000, size: 0x80000, crc: 0xF4FF18F5 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.06a", offset: 0x180000, size: 0x80000, crc: 0x260D0370 },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.07", offset: 0x200000, size: 0x80000, crc: 0x1324D02A },
            Op::Load { kind: Load::Word16Swap, file: "sfxj.08", offset: 0x280000, size: 0x80000, crc: 0x2DE76F10 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2xj.key", offset: 0x0, size: 0x14, crc: 0x160D1424 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2XJR1R: RomSetDef = RomSetDef { name: "ssf2xjr1r", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxo.03c", offset: 0x0, size: 0x80000, crc: 0x2BA33DC6 },
            Op::Load { kind: Load::Word16Swap, file: "sfxo.04a", offset: 0x80000, size: 0x80000, crc: 0xBA663DD7 },
            Op::Load { kind: Load::Word16Swap, file: "sfxo.05", offset: 0x100000, size: 0x80000, crc: 0x1321625C },
            Op::Load { kind: Load::Word16Swap, file: "sfxo.06a", offset: 0x180000, size: 0x80000, crc: 0x0CC490ED },
            Op::Load { kind: Load::Word16Swap, file: "sfxo.07", offset: 0x200000, size: 0x80000, crc: 0x64B9015E },
            Op::Load { kind: Load::Word16Swap, file: "sfxo.08", offset: 0x280000, size: 0x80000, crc: 0xB60F4B58 },
            Op::Load { kind: Load::Word16Swap, file: "sfxo.09", offset: 0x300000, size: 0x80000, crc: 0x642FAE3F },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.69", offset: 0x0, size: 0x80000, crc: 0xE9123F9F },
            Op::Load { kind: Load::Word64, file: "ssf.70", offset: 0x200000, size: 0x80000, crc: 0x2F8201F3 },
            Op::Load { kind: Load::Word64, file: "ssf.71", offset: 0x400000, size: 0x80000, crc: 0x0FA334B4 },
            Op::Load { kind: Load::Word64, file: "ssf.72", offset: 0x600000, size: 0x80000, crc: 0xB76740D3 },
            Op::Load { kind: Load::Word64, file: "ssf.73", offset: 0x800000, size: 0x80000, crc: 0x14F058EC },
            Op::Load { kind: Load::Word64, file: "ssf.74", offset: 0xa00000, size: 0x80000, crc: 0x800C3AE9 },
            Op::Load { kind: Load::Word64, file: "sfx.75", offset: 0xc00000, size: 0x80000, crc: 0x06CF540B },
            Op::Load { kind: Load::Word64, file: "sfx.76", offset: 0xe00000, size: 0x80000, crc: 0x71084E42 },
            Op::Load { kind: Load::Word64, file: "ssf.59", offset: 0x2, size: 0x80000, crc: 0x6EB3EE4D },
            Op::Load { kind: Load::Word64, file: "ssf.60", offset: 0x200002, size: 0x80000, crc: 0x2BCF1EDA },
            Op::Load { kind: Load::Word64, file: "ssf.61", offset: 0x400002, size: 0x80000, crc: 0x3330CC11 },
            Op::Load { kind: Load::Word64, file: "ssf.62", offset: 0x600002, size: 0x80000, crc: 0x96E2EAD3 },
            Op::Load { kind: Load::Word64, file: "ssf.63", offset: 0x800002, size: 0x80000, crc: 0xE356A275 },
            Op::Load { kind: Load::Word64, file: "ssf.64", offset: 0xa00002, size: 0x80000, crc: 0xFEC5698B },
            Op::Load { kind: Load::Word64, file: "sfx.65", offset: 0xc00002, size: 0x80000, crc: 0x69DA0751 },
            Op::Load { kind: Load::Word64, file: "sfx.66", offset: 0xe00002, size: 0x80000, crc: 0xCC53EC15 },
            Op::Load { kind: Load::Word64, file: "ssf.79", offset: 0x4, size: 0x80000, crc: 0xCF0D44A8 },
            Op::Load { kind: Load::Word64, file: "ssf.80", offset: 0x200004, size: 0x80000, crc: 0x56A153A4 },
            Op::Load { kind: Load::Word64, file: "ssf.81", offset: 0x400004, size: 0x80000, crc: 0x5484E5F6 },
            Op::Load { kind: Load::Word64, file: "ssf.82", offset: 0x600004, size: 0x80000, crc: 0xFCE6B7F5 },
            Op::Load { kind: Load::Word64, file: "ssf.83", offset: 0x800004, size: 0x80000, crc: 0x042D7970 },
            Op::Load { kind: Load::Word64, file: "ssf.84", offset: 0xa00004, size: 0x80000, crc: 0x88C472E6 },
            Op::Load { kind: Load::Word64, file: "sfx.85", offset: 0xc00004, size: 0x80000, crc: 0xA7D66348 },
            Op::Load { kind: Load::Word64, file: "sfx.86", offset: 0xe00004, size: 0x80000, crc: 0xCF9119C8 },
            Op::Load { kind: Load::Word64, file: "ssf.89", offset: 0x6, size: 0x80000, crc: 0x6D374AD9 },
            Op::Load { kind: Load::Word64, file: "ssf.90", offset: 0x200006, size: 0x80000, crc: 0x34CF8BCF },
            Op::Load { kind: Load::Word64, file: "ssf.91", offset: 0x400006, size: 0x80000, crc: 0xD796EA3F },
            Op::Load { kind: Load::Word64, file: "ssf.92", offset: 0x600006, size: 0x80000, crc: 0xC85FB7E3 },
            Op::Load { kind: Load::Word64, file: "ssf.93", offset: 0x800006, size: 0x80000, crc: 0x6C50C2B5 },
            Op::Load { kind: Load::Word64, file: "ssf.94", offset: 0xa00006, size: 0x80000, crc: 0x59549F63 },
            Op::Load { kind: Load::Word64, file: "sfx.95", offset: 0xc00006, size: 0x80000, crc: 0x86C97869 },
            Op::Load { kind: Load::Word64, file: "sfx.96", offset: 0xe00006, size: 0x80000, crc: 0x1C0E1989 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssf.51a", offset: 0x0, size: 0x80000, crc: 0x9EDA6954 },
            Op::Load { kind: Load::Word16Swap, file: "ssf.52a", offset: 0x80000, size: 0x80000, crc: 0x355F6589 },
            Op::Load { kind: Load::Word16Swap, file: "ssf.53a", offset: 0x100000, size: 0x80000, crc: 0xD5D08A05 },
            Op::Load { kind: Load::Word16Swap, file: "ssf.54a", offset: 0x180000, size: 0x80000, crc: 0x930725EB },
            Op::Load { kind: Load::Word16Swap, file: "ssf.55a", offset: 0x200000, size: 0x80000, crc: 0x827ABF3C },
            Op::Load { kind: Load::Word16Swap, file: "ssf.56a", offset: 0x280000, size: 0x80000, crc: 0x3919C0E5 },
            Op::Load { kind: Load::Word16Swap, file: "ssf.57a", offset: 0x300000, size: 0x80000, crc: 0x1BA9BFA6 },
            Op::Load { kind: Load::Word16Swap, file: "ssf.58a", offset: 0x380000, size: 0x80000, crc: 0x0C89A272 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf2xjr1r.key", offset: 0x0, size: 0x14, crc: 0x82C86E63 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNT2: RomSetDef = RomSetDef { name: "vhunt2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vh2j.03a", offset: 0x0, size: 0x80000, crc: 0x9AE8F186 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.04a", offset: 0x80000, size: 0x80000, crc: 0xE2FABF53 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.05", offset: 0x100000, size: 0x80000, crc: 0xDE34F624 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.06", offset: 0x180000, size: 0x80000, crc: 0x6A3B9897 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.07", offset: 0x200000, size: 0x80000, crc: 0xB021C029 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.08", offset: 0x280000, size: 0x80000, crc: 0xAC873DFF },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.09", offset: 0x300000, size: 0x80000, crc: 0xEAEFCE9C },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.10", offset: 0x380000, size: 0x80000, crc: 0x11730952 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vh2.13m", offset: 0x0, size: 0x400000, crc: 0x3B02DDAA },
            Op::Load { kind: Load::Word64, file: "vh2.15m", offset: 0x2, size: 0x400000, crc: 0x4E40DE66 },
            Op::Load { kind: Load::Word64, file: "vh2.17m", offset: 0x4, size: 0x400000, crc: 0xB31D00C9 },
            Op::Load { kind: Load::Word64, file: "vh2.19m", offset: 0x6, size: 0x400000, crc: 0x149BE3AB },
            Op::Load { kind: Load::Word64, file: "vh2.14m", offset: 0x1000000, size: 0x400000, crc: 0xCD09BD63 },
            Op::Load { kind: Load::Word64, file: "vh2.16m", offset: 0x1000002, size: 0x400000, crc: 0xE0182C15 },
            Op::Load { kind: Load::Word64, file: "vh2.18m", offset: 0x1000004, size: 0x400000, crc: 0x778DC4F6 },
            Op::Load { kind: Load::Word64, file: "vh2.20m", offset: 0x1000006, size: 0x400000, crc: 0x605D9D1D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vh2.01", offset: 0x0, size: 0x8000, crc: 0x67B9F779 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vh2.02", offset: 0x28000, size: 0x20000, crc: 0xAAF15FCB },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vh2.11m", offset: 0x0, size: 0x400000, crc: 0x38922EFD },
            Op::Load { kind: Load::Word16Swap, file: "vh2.12m", offset: 0x400000, size: 0x400000, crc: 0x6E2430AF },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vhunt2.key", offset: 0x0, size: 0x14, crc: 0x61306B20 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNT2R1: RomSetDef = RomSetDef { name: "vhunt2r1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vh2j.03", offset: 0x0, size: 0x80000, crc: 0x1A5FEB13 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.04", offset: 0x80000, size: 0x80000, crc: 0x434611A5 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.05", offset: 0x100000, size: 0x80000, crc: 0xDE34F624 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.06", offset: 0x180000, size: 0x80000, crc: 0x6A3B9897 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.07", offset: 0x200000, size: 0x80000, crc: 0xB021C029 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.08", offset: 0x280000, size: 0x80000, crc: 0xAC873DFF },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.09", offset: 0x300000, size: 0x80000, crc: 0xEAEFCE9C },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.10", offset: 0x380000, size: 0x80000, crc: 0x11730952 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vh2.13m", offset: 0x0, size: 0x400000, crc: 0x3B02DDAA },
            Op::Load { kind: Load::Word64, file: "vh2.15m", offset: 0x2, size: 0x400000, crc: 0x4E40DE66 },
            Op::Load { kind: Load::Word64, file: "vh2.17m", offset: 0x4, size: 0x400000, crc: 0xB31D00C9 },
            Op::Load { kind: Load::Word64, file: "vh2.19m", offset: 0x6, size: 0x400000, crc: 0x149BE3AB },
            Op::Load { kind: Load::Word64, file: "vh2.14m", offset: 0x1000000, size: 0x400000, crc: 0xCD09BD63 },
            Op::Load { kind: Load::Word64, file: "vh2.16m", offset: 0x1000002, size: 0x400000, crc: 0xE0182C15 },
            Op::Load { kind: Load::Word64, file: "vh2.18m", offset: 0x1000004, size: 0x400000, crc: 0x778DC4F6 },
            Op::Load { kind: Load::Word64, file: "vh2.20m", offset: 0x1000006, size: 0x400000, crc: 0x605D9D1D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vh2.01", offset: 0x0, size: 0x8000, crc: 0x67B9F779 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vh2.02", offset: 0x28000, size: 0x20000, crc: 0xAAF15FCB },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vh2.11m", offset: 0x0, size: 0x400000, crc: 0x38922EFD },
            Op::Load { kind: Load::Word16Swap, file: "vh2.12m", offset: 0x400000, size: 0x400000, crc: 0x6E2430AF },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vhunt2.key", offset: 0x0, size: 0x14, crc: 0x61306B20 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAV: RomSetDef = RomSetDef { name: "vsav", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3e.03d", offset: 0x0, size: 0x80000, crc: 0xF5962A8C },
            Op::Load { kind: Load::Word16Swap, file: "vm3e.04d", offset: 0x80000, size: 0x80000, crc: 0x21B40EA2 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.05a", offset: 0x100000, size: 0x80000, crc: 0x4118E00F },
            Op::Load { kind: Load::Word16Swap, file: "vm3.06a", offset: 0x180000, size: 0x80000, crc: 0x2F4FD3A9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.07b", offset: 0x200000, size: 0x80000, crc: 0xCBDA91B8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.08a", offset: 0x280000, size: 0x80000, crc: 0x6CA47259 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.09b", offset: 0x300000, size: 0x80000, crc: 0xF4A339E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.10b", offset: 0x380000, size: 0x80000, crc: 0xFFFBB5B8 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsav.key", offset: 0x0, size: 0x14, crc: 0xA6E3B164 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAVU: RomSetDef = RomSetDef { name: "vsavu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3u.03d", offset: 0x0, size: 0x80000, crc: 0x1F295274 },
            Op::Load { kind: Load::Word16Swap, file: "vm3u.04d", offset: 0x80000, size: 0x80000, crc: 0xC46ADF81 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.05a", offset: 0x100000, size: 0x80000, crc: 0x4118E00F },
            Op::Load { kind: Load::Word16Swap, file: "vm3.06a", offset: 0x180000, size: 0x80000, crc: 0x2F4FD3A9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.07b", offset: 0x200000, size: 0x80000, crc: 0xCBDA91B8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.08a", offset: 0x280000, size: 0x80000, crc: 0x6CA47259 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.09b", offset: 0x300000, size: 0x80000, crc: 0xF4A339E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.10b", offset: 0x380000, size: 0x80000, crc: 0xFFFBB5B8 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsavu.key", offset: 0x0, size: 0x14, crc: 0xFF21B9D7 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAVJ: RomSetDef = RomSetDef { name: "vsavj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3j.03d", offset: 0x0, size: 0x80000, crc: 0x2A2E74A4 },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.04d", offset: 0x80000, size: 0x80000, crc: 0x1C2427BC },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.05a", offset: 0x100000, size: 0x80000, crc: 0x95CE88D5 },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.06b", offset: 0x180000, size: 0x80000, crc: 0x2C4297E0 },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.07b", offset: 0x200000, size: 0x80000, crc: 0xA38AAAE7 },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.08a", offset: 0x280000, size: 0x80000, crc: 0x5773E5C9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.09b", offset: 0x300000, size: 0x80000, crc: 0xD064F8B9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3j.10b", offset: 0x380000, size: 0x80000, crc: 0x434518E9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsavj.key", offset: 0x0, size: 0x14, crc: 0x36D28AB8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAVA: RomSetDef = RomSetDef { name: "vsava", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3a.03d", offset: 0x0, size: 0x80000, crc: 0x44C1198F },
            Op::Load { kind: Load::Word16Swap, file: "vm3a.04d", offset: 0x80000, size: 0x80000, crc: 0x2218B781 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.05a", offset: 0x100000, size: 0x80000, crc: 0x4118E00F },
            Op::Load { kind: Load::Word16Swap, file: "vm3.06a", offset: 0x180000, size: 0x80000, crc: 0x2F4FD3A9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.07b", offset: 0x200000, size: 0x80000, crc: 0xCBDA91B8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.08a", offset: 0x280000, size: 0x80000, crc: 0x6CA47259 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.09b", offset: 0x300000, size: 0x80000, crc: 0xF4A339E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.10b", offset: 0x380000, size: 0x80000, crc: 0xFFFBB5B8 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsava.key", offset: 0x0, size: 0x14, crc: 0x8A3520F4 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAVH: RomSetDef = RomSetDef { name: "vsavh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3h.03a", offset: 0x0, size: 0x80000, crc: 0x7CC62DF8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3h.04d", offset: 0x80000, size: 0x80000, crc: 0xD716F3B5 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.05a", offset: 0x100000, size: 0x80000, crc: 0x4118E00F },
            Op::Load { kind: Load::Word16Swap, file: "vm3.06a", offset: 0x180000, size: 0x80000, crc: 0x2F4FD3A9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.07b", offset: 0x200000, size: 0x80000, crc: 0xCBDA91B8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.08a", offset: 0x280000, size: 0x80000, crc: 0x6CA47259 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.09b", offset: 0x300000, size: 0x80000, crc: 0xF4A339E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.10b", offset: 0x380000, size: 0x80000, crc: 0xFFFBB5B8 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsavh.key", offset: 0x0, size: 0x14, crc: 0xA7DD6409 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAVB: RomSetDef = RomSetDef { name: "vsavb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3b.03d", offset: 0x0, size: 0x80000, crc: 0x22FFBA4B },
            Op::Load { kind: Load::Word16Swap, file: "vm3b.04d", offset: 0x80000, size: 0x80000, crc: 0x88624909 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.05a", offset: 0x100000, size: 0x80000, crc: 0x4118E00F },
            Op::Load { kind: Load::Word16Swap, file: "vm3.06a", offset: 0x180000, size: 0x80000, crc: 0x2F4FD3A9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.07b", offset: 0x200000, size: 0x80000, crc: 0xCBDA91B8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.08a", offset: 0x280000, size: 0x80000, crc: 0x6CA47259 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.09b", offset: 0x300000, size: 0x80000, crc: 0xF4A339E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.10b", offset: 0x380000, size: 0x80000, crc: 0xFFFBB5B8 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsavb.key", offset: 0x0, size: 0x14, crc: 0x5274E635 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAV2: RomSetDef = RomSetDef { name: "vsav2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vs2j.03", offset: 0x0, size: 0x80000, crc: 0x89FD86B4 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.04", offset: 0x80000, size: 0x80000, crc: 0x107C091B },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.05", offset: 0x100000, size: 0x80000, crc: 0x61979638 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.06", offset: 0x180000, size: 0x80000, crc: 0xF37C5BC2 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.07", offset: 0x200000, size: 0x80000, crc: 0x8F885809 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.08", offset: 0x280000, size: 0x80000, crc: 0x2018C120 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.09", offset: 0x300000, size: 0x80000, crc: 0xFAC3C217 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.10", offset: 0x380000, size: 0x80000, crc: 0xEB490213 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vs2.13m", offset: 0x0, size: 0x400000, crc: 0x5C852F52 },
            Op::Load { kind: Load::Word64, file: "vs2.15m", offset: 0x2, size: 0x400000, crc: 0xA20F58AF },
            Op::Load { kind: Load::Word64, file: "vs2.17m", offset: 0x4, size: 0x400000, crc: 0x39DB59AD },
            Op::Load { kind: Load::Word64, file: "vs2.19m", offset: 0x6, size: 0x400000, crc: 0x00C763A7 },
            Op::Load { kind: Load::Word64, file: "vs2.14m", offset: 0x1000000, size: 0x400000, crc: 0xCD09BD63 },
            Op::Load { kind: Load::Word64, file: "vs2.16m", offset: 0x1000002, size: 0x400000, crc: 0xE0182C15 },
            Op::Load { kind: Load::Word64, file: "vs2.18m", offset: 0x1000004, size: 0x400000, crc: 0x778DC4F6 },
            Op::Load { kind: Load::Word64, file: "vs2.20m", offset: 0x1000006, size: 0x400000, crc: 0x605D9D1D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vs2.01", offset: 0x0, size: 0x8000, crc: 0x35190139 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vs2.02", offset: 0x28000, size: 0x20000, crc: 0xC32DBA09 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vs2.11m", offset: 0x0, size: 0x400000, crc: 0xD67E47B7 },
            Op::Load { kind: Load::Word16Swap, file: "vs2.12m", offset: 0x400000, size: 0x400000, crc: 0x6D020A14 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vsav2.key", offset: 0x0, size: 0x14, crc: 0x289028CE },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTA: RomSetDef = RomSetDef { name: "xmcota", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmne.03f", offset: 0x0, size: 0x80000, crc: 0x5A726D13 },
            Op::Load { kind: Load::Word16Swap, file: "xmne.04f", offset: 0x80000, size: 0x80000, crc: 0x06A83F3A },
            Op::Load { kind: Load::Word16Swap, file: "xmne.05b", offset: 0x100000, size: 0x80000, crc: 0x87B0ED0F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmne.10b", offset: 0x380000, size: 0x80000, crc: 0xCB36B0A4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcota.key", offset: 0x0, size: 0x14, crc: 0x6665BBFB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAR1: RomSetDef = RomSetDef { name: "xmcotar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmne.03e", offset: 0x0, size: 0x80000, crc: 0xA9A09B09 },
            Op::Load { kind: Load::Word16Swap, file: "xmne.04e", offset: 0x80000, size: 0x80000, crc: 0x52FA2106 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05a", offset: 0x100000, size: 0x80000, crc: 0xAC0D7759 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10a", offset: 0x380000, size: 0x80000, crc: 0x53C0EAB9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcota.key", offset: 0x0, size: 0x14, crc: 0x6665BBFB },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAU: RomSetDef = RomSetDef { name: "xmcotau", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnu.03e", offset: 0x0, size: 0x80000, crc: 0x0BAFEB0E },
            Op::Load { kind: Load::Word16Swap, file: "xmnu.04e", offset: 0x80000, size: 0x80000, crc: 0xC29BDAE3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05a", offset: 0x100000, size: 0x80000, crc: 0xAC0D7759 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10a", offset: 0x380000, size: 0x80000, crc: 0x53C0EAB9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotau.key", offset: 0x0, size: 0x14, crc: 0x623D3357 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAH: RomSetDef = RomSetDef { name: "xmcotah", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnh.03f", offset: 0x0, size: 0x80000, crc: 0xE4B85A90 },
            Op::Load { kind: Load::Word16Swap, file: "xmnh.04f", offset: 0x80000, size: 0x80000, crc: 0x7DFE1406 },
            Op::Load { kind: Load::Word16Swap, file: "xmnh.05b", offset: 0x100000, size: 0x80000, crc: 0x87B0ED0F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmnh.10b", offset: 0x380000, size: 0x80000, crc: 0xCB36B0A4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotah.key", offset: 0x0, size: 0x14, crc: 0xC9A45A5A },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAHR1: RomSetDef = RomSetDef { name: "xmcotahr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnh.03d", offset: 0x0, size: 0x80000, crc: 0x63B0A84F },
            Op::Load { kind: Load::Word16Swap, file: "xmnh.04d", offset: 0x80000, size: 0x80000, crc: 0xB1B9B727 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05a", offset: 0x100000, size: 0x80000, crc: 0xAC0D7759 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10a", offset: 0x380000, size: 0x80000, crc: 0x53C0EAB9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotah.key", offset: 0x0, size: 0x14, crc: 0xC9A45A5A },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAJ: RomSetDef = RomSetDef { name: "xmcotaj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnj.03e", offset: 0x0, size: 0x80000, crc: 0x0DF29F5F },
            Op::Load { kind: Load::Word16Swap, file: "xmnj.04e", offset: 0x80000, size: 0x80000, crc: 0x4A65833B },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05a", offset: 0x100000, size: 0x80000, crc: 0xAC0D7759 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10a", offset: 0x380000, size: 0x80000, crc: 0x53C0EAB9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaj.key", offset: 0x0, size: 0x14, crc: 0xD278B4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAJ1: RomSetDef = RomSetDef { name: "xmcotaj1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnj.03d", offset: 0x0, size: 0x80000, crc: 0x79086D62 },
            Op::Load { kind: Load::Word16Swap, file: "xmnj.04d", offset: 0x80000, size: 0x80000, crc: 0x38EED613 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05", offset: 0x100000, size: 0x80000, crc: 0xC3ED62A2 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06", offset: 0x180000, size: 0x80000, crc: 0xF03C52E1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07", offset: 0x200000, size: 0x80000, crc: 0x325626B1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08", offset: 0x280000, size: 0x80000, crc: 0x7194EA10 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09", offset: 0x300000, size: 0x80000, crc: 0xAE946DF3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10", offset: 0x380000, size: 0x80000, crc: 0x32A6BE1D },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaj.key", offset: 0x0, size: 0x14, crc: 0xD278B4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAJ2: RomSetDef = RomSetDef { name: "xmcotaj2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnj.03b", offset: 0x0, size: 0x80000, crc: 0xC8175FB3 },
            Op::Load { kind: Load::Word16Swap, file: "xmnj.04b", offset: 0x80000, size: 0x80000, crc: 0x54B3FBA3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05", offset: 0x100000, size: 0x80000, crc: 0xC3ED62A2 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06", offset: 0x180000, size: 0x80000, crc: 0xF03C52E1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07", offset: 0x200000, size: 0x80000, crc: 0x325626B1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08", offset: 0x280000, size: 0x80000, crc: 0x7194EA10 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09", offset: 0x300000, size: 0x80000, crc: 0xAE946DF3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10", offset: 0x380000, size: 0x80000, crc: 0x32A6BE1D },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaj.key", offset: 0x0, size: 0x14, crc: 0xD278B4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAJ3: RomSetDef = RomSetDef { name: "xmcotaj3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnj.03a", offset: 0x0, size: 0x80000, crc: 0x00761611 },
            Op::Load { kind: Load::Word16Swap, file: "xmnj.04a", offset: 0x80000, size: 0x80000, crc: 0x614D3F60 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05", offset: 0x100000, size: 0x80000, crc: 0xC3ED62A2 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06", offset: 0x180000, size: 0x80000, crc: 0xF03C52E1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07", offset: 0x200000, size: 0x80000, crc: 0x325626B1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08", offset: 0x280000, size: 0x80000, crc: 0x7194EA10 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09", offset: 0x300000, size: 0x80000, crc: 0xAE946DF3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10", offset: 0x380000, size: 0x80000, crc: 0x32A6BE1D },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaj.key", offset: 0x0, size: 0x14, crc: 0xD278B4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAJR: RomSetDef = RomSetDef { name: "xmcotajr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmno.03a", offset: 0x0, size: 0x80000, crc: 0x7AB19ACF },
            Op::Load { kind: Load::Word16Swap, file: "xmno.04a", offset: 0x80000, size: 0x80000, crc: 0x7615DD21 },
            Op::Load { kind: Load::Word16Swap, file: "xmno.05a", offset: 0x100000, size: 0x80000, crc: 0x0303D672 },
            Op::Load { kind: Load::Word16Swap, file: "xmno.06a", offset: 0x180000, size: 0x80000, crc: 0x332839A5 },
            Op::Load { kind: Load::Word16Swap, file: "xmno.07", offset: 0x200000, size: 0x80000, crc: 0x6255E8D5 },
            Op::Load { kind: Load::Word16Swap, file: "xmno.08", offset: 0x280000, size: 0x80000, crc: 0xB8EBE77C },
            Op::Load { kind: Load::Word16Swap, file: "xmno.09", offset: 0x300000, size: 0x80000, crc: 0x5440D950 },
            Op::Load { kind: Load::Word16Swap, file: "xmno.10a", offset: 0x380000, size: 0x80000, crc: 0xB8296966 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01", offset: 0x0, size: 0x8000, crc: 0x7178336E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02", offset: 0x28000, size: 0x20000, crc: 0x0EC58501 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaj.key", offset: 0x0, size: 0x14, crc: 0xD278B4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAA: RomSetDef = RomSetDef { name: "xmcotaa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmna.03e", offset: 0x0, size: 0x80000, crc: 0xF1ADE6E7 },
            Op::Load { kind: Load::Word16Swap, file: "xmna.04e", offset: 0x80000, size: 0x80000, crc: 0xB5A8843D },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05a", offset: 0x100000, size: 0x80000, crc: 0xAC0D7759 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10a", offset: 0x380000, size: 0x80000, crc: 0x53C0EAB9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaa.key", offset: 0x0, size: 0x14, crc: 0x3FDD2D42 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAAR1: RomSetDef = RomSetDef { name: "xmcotaar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmna.03b", offset: 0x0, size: 0x80000, crc: 0x2280D03F },
            Op::Load { kind: Load::Word16Swap, file: "xmna.04b", offset: 0x80000, size: 0x80000, crc: 0xEBBBF11E },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05", offset: 0x100000, size: 0x80000, crc: 0xC3ED62A2 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06", offset: 0x180000, size: 0x80000, crc: 0xF03C52E1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07", offset: 0x200000, size: 0x80000, crc: 0x325626B1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08", offset: 0x280000, size: 0x80000, crc: 0x7194EA10 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09", offset: 0x300000, size: 0x80000, crc: 0xAE946DF3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10", offset: 0x380000, size: 0x80000, crc: 0x32A6BE1D },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaa.key", offset: 0x0, size: 0x14, crc: 0x3FDD2D42 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAAR2: RomSetDef = RomSetDef { name: "xmcotaar2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmna.03a", offset: 0x0, size: 0x80000, crc: 0x7DF8B27E },
            Op::Load { kind: Load::Word16Swap, file: "xmna.04a", offset: 0x80000, size: 0x80000, crc: 0xB44E30A7 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05", offset: 0x100000, size: 0x80000, crc: 0xC3ED62A2 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06", offset: 0x180000, size: 0x80000, crc: 0xF03C52E1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07", offset: 0x200000, size: 0x80000, crc: 0x325626B1 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08", offset: 0x280000, size: 0x80000, crc: 0x7194EA10 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09", offset: 0x300000, size: 0x80000, crc: 0xAE946DF3 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10", offset: 0x380000, size: 0x80000, crc: 0x32A6BE1D },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotaa.key", offset: 0x0, size: 0x14, crc: 0x3FDD2D42 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMCOTAB: RomSetDef = RomSetDef { name: "xmcotab", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmnb.03c", offset: 0x0, size: 0x80000, crc: 0xAB48BCB0 },
            Op::Load { kind: Load::Word16Swap, file: "xmnb.04c", offset: 0x80000, size: 0x80000, crc: 0x8D8FCBB1 },
            Op::Load { kind: Load::Word16Swap, file: "xmne.05b", offset: 0x100000, size: 0x80000, crc: 0x87B0ED0F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmne.10b", offset: 0x380000, size: 0x80000, crc: 0xCB36B0A4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmcotab.key", offset: 0x0, size: 0x14, crc: 0xBB70F10F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSF: RomSetDef = RomSetDef { name: "xmvsf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvse.03f", offset: 0x0, size: 0x80000, crc: 0xDB06413F },
            Op::Load { kind: Load::Word16Swap, file: "xvse.04f", offset: 0x80000, size: 0x80000, crc: 0xEF015AEF },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsf.key", offset: 0x0, size: 0x14, crc: 0xD5C07311 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFR1: RomSetDef = RomSetDef { name: "xmvsfr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvse.03d", offset: 0x0, size: 0x80000, crc: 0x5AE5BD3B },
            Op::Load { kind: Load::Word16Swap, file: "xvse.04d", offset: 0x80000, size: 0x80000, crc: 0x5EB9C02E },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsf.key", offset: 0x0, size: 0x14, crc: 0xD5C07311 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFU: RomSetDef = RomSetDef { name: "xmvsfu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsu.03k", offset: 0x0, size: 0x80000, crc: 0x8739EF61 },
            Op::Load { kind: Load::Word16Swap, file: "xvsu.04k", offset: 0x80000, size: 0x80000, crc: 0xE11D35C1 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfu.key", offset: 0x0, size: 0x14, crc: 0xECA13458 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFUR1: RomSetDef = RomSetDef { name: "xmvsfur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsu.03i", offset: 0x0, size: 0x80000, crc: 0x5481155A },
            Op::Load { kind: Load::Word16Swap, file: "xvsu.04i", offset: 0x80000, size: 0x80000, crc: 0x1E236388 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfu.key", offset: 0x0, size: 0x14, crc: 0xECA13458 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFUR2: RomSetDef = RomSetDef { name: "xmvsfur2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsu.03d", offset: 0x0, size: 0x80000, crc: 0xBD8B152F },
            Op::Load { kind: Load::Word16Swap, file: "xvsu.04d", offset: 0x80000, size: 0x80000, crc: 0x7C7D1DA3 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfu.key", offset: 0x0, size: 0x14, crc: 0xECA13458 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFJ: RomSetDef = RomSetDef { name: "xmvsfj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsj.03m", offset: 0x0, size: 0x80000, crc: 0xE2944372 },
            Op::Load { kind: Load::Word16Swap, file: "xvsj.04m", offset: 0x80000, size: 0x80000, crc: 0xC20B8524 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfj.key", offset: 0x0, size: 0x14, crc: 0x87576CDA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFJR1: RomSetDef = RomSetDef { name: "xmvsfjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsj.03k", offset: 0x0, size: 0x80000, crc: 0x2A167526 },
            Op::Load { kind: Load::Word16Swap, file: "xvsj.04k", offset: 0x80000, size: 0x80000, crc: 0xD993436B },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfj.key", offset: 0x0, size: 0x14, crc: 0x87576CDA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFJR2: RomSetDef = RomSetDef { name: "xmvsfjr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsj.03i", offset: 0x0, size: 0x80000, crc: 0xEF24DA96 },
            Op::Load { kind: Load::Word16Swap, file: "xvsj.04i", offset: 0x80000, size: 0x80000, crc: 0x70A59B35 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfj.key", offset: 0x0, size: 0x14, crc: 0x87576CDA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFJR3: RomSetDef = RomSetDef { name: "xmvsfjr3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsj.03d", offset: 0x0, size: 0x80000, crc: 0xBEB81DE9 },
            Op::Load { kind: Load::Word16Swap, file: "xvsj.04d", offset: 0x80000, size: 0x80000, crc: 0x23D11271 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfj.key", offset: 0x0, size: 0x14, crc: 0x87576CDA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFJR4: RomSetDef = RomSetDef { name: "xmvsfjr4", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsj.03c", offset: 0x0, size: 0x80000, crc: 0x180656A1 },
            Op::Load { kind: Load::Word16Swap, file: "xvsj.04c", offset: 0x80000, size: 0x80000, crc: 0x5832811C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05", offset: 0x100000, size: 0x80000, crc: 0x030E0E1E },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06", offset: 0x180000, size: 0x80000, crc: 0x5D04A8FF },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfj.key", offset: 0x0, size: 0x14, crc: 0x87576CDA },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFA: RomSetDef = RomSetDef { name: "xmvsfa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsa.03k", offset: 0x0, size: 0x80000, crc: 0xD0CCA7A8 },
            Op::Load { kind: Load::Word16Swap, file: "xvsa.04k", offset: 0x80000, size: 0x80000, crc: 0x8C8E76FD },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfa.key", offset: 0x0, size: 0x14, crc: 0x44941468 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFAR1: RomSetDef = RomSetDef { name: "xmvsfar1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsa.03", offset: 0x0, size: 0x80000, crc: 0x520054DF },
            Op::Load { kind: Load::Word16Swap, file: "xvsa.04", offset: 0x80000, size: 0x80000, crc: 0x13086E55 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfa.key", offset: 0x0, size: 0x14, crc: 0x44941468 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFAR2: RomSetDef = RomSetDef { name: "xmvsfar2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsa.03e", offset: 0x0, size: 0x80000, crc: 0x9BDDE21C },
            Op::Load { kind: Load::Word16Swap, file: "xvsa.04e", offset: 0x80000, size: 0x80000, crc: 0x33300EDF },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfa.key", offset: 0x0, size: 0x14, crc: 0x44941468 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFAR3: RomSetDef = RomSetDef { name: "xmvsfar3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsa.03d", offset: 0x0, size: 0x80000, crc: 0x2B164FD7 },
            Op::Load { kind: Load::Word16Swap, file: "xvsa.04d", offset: 0x80000, size: 0x80000, crc: 0x2D32F039 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvsa.02", offset: 0x28000, size: 0x20000, crc: 0x19272E4C },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfa.key", offset: 0x0, size: 0x14, crc: 0x44941468 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFH: RomSetDef = RomSetDef { name: "xmvsfh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsh.03a", offset: 0x0, size: 0x80000, crc: 0xD4FFFB04 },
            Op::Load { kind: Load::Word16Swap, file: "xvsh.04a", offset: 0x80000, size: 0x80000, crc: 0x1B4EA638 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfh.key", offset: 0x0, size: 0x14, crc: 0xF632A36B },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_XMVSFB: RomSetDef = RomSetDef { name: "xmvsfb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsb.03h", offset: 0x0, size: 0x80000, crc: 0x05BACCCA },
            Op::Load { kind: Load::Word16Swap, file: "xvsb.04h", offset: 0x80000, size: 0x80000, crc: 0xE350C755 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmvsfb.key", offset: 0x0, size: 0x14, crc: 0xF0384798 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_1944D: RomSetDef = RomSetDef { name: "1944d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nffud.03", offset: 0x0, size: 0x80000, crc: 0x28E8AAE4 },
            Op::Load { kind: Load::Word16Swap, file: "nff.04", offset: 0x80000, size: 0x80000, crc: 0xDBA1C66E },
            Op::Load { kind: Load::Word16Swap, file: "nffu.05", offset: 0x100000, size: 0x80000, crc: 0xEA813EB7 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "nff.13m", offset: 0x0, size: 0x400000, crc: 0xC9FCA741 },
            Op::Load { kind: Load::Word64, file: "nff.15m", offset: 0x2, size: 0x400000, crc: 0xF809D898 },
            Op::Load { kind: Load::Word64, file: "nff.17m", offset: 0x4, size: 0x400000, crc: 0x15BA4507 },
            Op::Load { kind: Load::Word64, file: "nff.19m", offset: 0x6, size: 0x400000, crc: 0x3DD41B8C },
            Op::Load { kind: Load::Word64, file: "nff.14m", offset: 0x1000000, size: 0x100000, crc: 0x3FE3A54B },
            Op::Load { kind: Load::Word64, file: "nff.16m", offset: 0x1000002, size: 0x100000, crc: 0x565CD231 },
            Op::Load { kind: Load::Word64, file: "nff.18m", offset: 0x1000004, size: 0x100000, crc: 0x63CA5988 },
            Op::Load { kind: Load::Word64, file: "nff.20m", offset: 0x1000006, size: 0x100000, crc: 0x21EB8F3B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nff.01", offset: 0x0, size: 0x8000, crc: 0xD2E44318 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "nff.11m", offset: 0x0, size: 0x400000, crc: 0x243E4E05 },
            Op::Load { kind: Load::Word16Swap, file: "nff.12m", offset: 0x400000, size: 0x400000, crc: 0x4FCF1600 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_19XXD: RomSetDef = RomSetDef { name: "19xxd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19xud.03", offset: 0x0, size: 0x80000, crc: 0xF81B60E5 },
            Op::Load { kind: Load::Word16Swap, file: "19xud.04", offset: 0x80000, size: 0x80000, crc: 0xCC44638C },
            Op::Load { kind: Load::Word16Swap, file: "19xud.05", offset: 0x100000, size: 0x80000, crc: 0x33A168DE },
            Op::Load { kind: Load::Word16Swap, file: "19xud.06", offset: 0x180000, size: 0x80000, crc: 0xE0111282 },
            Op::Load { kind: Load::Word16Swap, file: "19x.07", offset: 0x200000, size: 0x80000, crc: 0x61C0296C },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "19x.13m", offset: 0x0, size: 0x80000, crc: 0x427AEB18 },
            Op::Load { kind: Load::Word64, file: "19x.15m", offset: 0x2, size: 0x80000, crc: 0x63BDBF54 },
            Op::Load { kind: Load::Word64, file: "19x.17m", offset: 0x4, size: 0x80000, crc: 0x2DFE18B5 },
            Op::Load { kind: Load::Word64, file: "19x.19m", offset: 0x6, size: 0x80000, crc: 0xCBEF9579 },
            Op::Load { kind: Load::Word64, file: "19x.14m", offset: 0x800000, size: 0x200000, crc: 0xE916967C },
            Op::Load { kind: Load::Word64, file: "19x.16m", offset: 0x800002, size: 0x200000, crc: 0x6E75F3DB },
            Op::Load { kind: Load::Word64, file: "19x.18m", offset: 0x800004, size: 0x200000, crc: 0x2213E798 },
            Op::Load { kind: Load::Word64, file: "19x.20m", offset: 0x800006, size: 0x200000, crc: 0xAB9D5B96 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "19x.01", offset: 0x0, size: 0x8000, crc: 0xEF55195E },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "19x.11m", offset: 0x0, size: 0x200000, crc: 0xD38BEEF3 },
            Op::Load { kind: Load::Word16Swap, file: "19x.12m", offset: 0x200000, size: 0x200000, crc: 0xD47C96E2 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2AD: RomSetDef = RomSetDef { name: "sfz2ad", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2ad.03a", offset: 0x0, size: 0x80000, crc: 0x017F8FAB },
            Op::Load { kind: Load::Word16Swap, file: "sz2ad.04a", offset: 0x80000, size: 0x80000, crc: 0xF50E5EA2 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05a", offset: 0x100000, size: 0x80000, crc: 0x98E8E992 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2a.07a", offset: 0x200000, size: 0x80000, crc: 0x0AED2494 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.08", offset: 0x280000, size: 0x80000, crc: 0x0FE8585D },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_AVSPD: RomSetDef = RomSetDef { name: "avspd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avped.03d", offset: 0x0, size: 0x80000, crc: 0x66AA8AAD },
            Op::Load { kind: Load::Word16Swap, file: "avped.04d", offset: 0x80000, size: 0x80000, crc: 0x579306C2 },
            Op::Load { kind: Load::Word16Swap, file: "avp.05d", offset: 0x100000, size: 0x80000, crc: 0xFBFB5D7A },
            Op::Load { kind: Load::Word16Swap, file: "avpd.06", offset: 0x180000, size: 0x80000, crc: 0x63094539 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "avp.13m", offset: 0x0, size: 0x200000, crc: 0x8F8B5AE4 },
            Op::Load { kind: Load::Word64, file: "avp.15m", offset: 0x2, size: 0x200000, crc: 0xB00280DF },
            Op::Load { kind: Load::Word64, file: "avp.17m", offset: 0x4, size: 0x200000, crc: 0x94403195 },
            Op::Load { kind: Load::Word64, file: "avp.19m", offset: 0x6, size: 0x200000, crc: 0xE1981245 },
            Op::Load { kind: Load::Word64, file: "avp.14m", offset: 0x800000, size: 0x100000, crc: 0x39933B1E },
            Op::Load { kind: Load::Word64, file: "avp.16m", offset: 0x800002, size: 0x100000, crc: 0x85412860 },
            Op::Load { kind: Load::Word64, file: "avp.18m", offset: 0x800004, size: 0x100000, crc: 0x2E2BEB06 },
            Op::Load { kind: Load::Word64, file: "avp.20m", offset: 0x800006, size: 0x100000, crc: 0xE4798D1A },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "avp.01", offset: 0x0, size: 0x8000, crc: 0x2D3B4220 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "avp.11m", offset: 0x0, size: 0x200000, crc: 0x83499817 },
            Op::Load { kind: Load::Word16Swap, file: "avp.12m", offset: 0x200000, size: 0x200000, crc: 0xF4110D49 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_BATCIRD: RomSetDef = RomSetDef { name: "batcird", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btced.03", offset: 0x0, size: 0x80000, crc: 0x0737DB6D },
            Op::Load { kind: Load::Word16Swap, file: "btced.04", offset: 0x80000, size: 0x80000, crc: 0xEF1A8823 },
            Op::Load { kind: Load::Word16Swap, file: "btced.05", offset: 0x100000, size: 0x80000, crc: 0x20BDBB14 },
            Op::Load { kind: Load::Word16Swap, file: "btced.06", offset: 0x180000, size: 0x80000, crc: 0xB4D8F5BC },
            Op::Load { kind: Load::Word16Swap, file: "btc.07", offset: 0x200000, size: 0x80000, crc: 0x7322D5DB },
            Op::Load { kind: Load::Word16Swap, file: "btc.08", offset: 0x280000, size: 0x80000, crc: 0x6AAC85AB },
            Op::Load { kind: Load::Word16Swap, file: "btc.09", offset: 0x300000, size: 0x80000, crc: 0x1203DB08 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "btc.13m", offset: 0x0, size: 0x400000, crc: 0xDC705BAD },
            Op::Load { kind: Load::Word64, file: "btc.15m", offset: 0x2, size: 0x400000, crc: 0xE5779A3C },
            Op::Load { kind: Load::Word64, file: "btc.17m", offset: 0x4, size: 0x400000, crc: 0xB33F4112 },
            Op::Load { kind: Load::Word64, file: "btc.19m", offset: 0x6, size: 0x400000, crc: 0xA6FCDB7E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "btc.01", offset: 0x0, size: 0x8000, crc: 0x1E194310 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "btc.02", offset: 0x28000, size: 0x20000, crc: 0x01AEB8E6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "btc.11m", offset: 0x0, size: 0x200000, crc: 0xC27F2229 },
            Op::Load { kind: Load::Word16Swap, file: "btc.12m", offset: 0x200000, size: 0x200000, crc: 0x418A2E33 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_DDTODD: RomSetDef = RomSetDef { name: "ddtodd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "daded.03c", offset: 0x0, size: 0x80000, crc: 0x843330F4 },
            Op::Load { kind: Load::Word16Swap, file: "daded.04c", offset: 0x80000, size: 0x80000, crc: 0x306F14FC },
            Op::Load { kind: Load::Word16Swap, file: "daded.05c", offset: 0x100000, size: 0x80000, crc: 0x8C6B8328 },
            Op::Load { kind: Load::Word16Swap, file: "dad.06a", offset: 0x180000, size: 0x80000, crc: 0x6225495A },
            Op::Load { kind: Load::Word16Swap, file: "dadd.07a", offset: 0x200000, size: 0x80000, crc: 0x0F0DF6CC },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dad.13m", offset: 0x0, size: 0x200000, crc: 0xDA3CB7D6 },
            Op::Load { kind: Load::Word64, file: "dad.15m", offset: 0x2, size: 0x200000, crc: 0x92B63172 },
            Op::Load { kind: Load::Word64, file: "dad.17m", offset: 0x4, size: 0x200000, crc: 0xB98757F5 },
            Op::Load { kind: Load::Word64, file: "dad.19m", offset: 0x6, size: 0x200000, crc: 0x8121CE46 },
            Op::Load { kind: Load::Word64, file: "dad.14m", offset: 0x800000, size: 0x100000, crc: 0x837E6F3F },
            Op::Load { kind: Load::Word64, file: "dad.16m", offset: 0x800002, size: 0x100000, crc: 0xF0916BDB },
            Op::Load { kind: Load::Word64, file: "dad.18m", offset: 0x800004, size: 0x100000, crc: 0xCEF393EF },
            Op::Load { kind: Load::Word64, file: "dad.20m", offset: 0x800006, size: 0x100000, crc: 0x8953FE9E },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dad.01", offset: 0x0, size: 0x8000, crc: 0x3F5E2424 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dad.11m", offset: 0x0, size: 0x200000, crc: 0x0C499B67 },
            Op::Load { kind: Load::Word16Swap, file: "dad.12m", offset: 0x200000, size: 0x200000, crc: 0x2F0B5A4E },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_DDSOMUD: RomSetDef = RomSetDef { name: "ddsomud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2ud.03g", offset: 0x0, size: 0x80000, crc: 0x816F695A },
            Op::Load { kind: Load::Word16Swap, file: "dd2ud.04g", offset: 0x80000, size: 0x80000, crc: 0x7CC81C6B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.05g", offset: 0x100000, size: 0x80000, crc: 0x5EB1991C },
            Op::Load { kind: Load::Word16Swap, file: "dd2.06g", offset: 0x180000, size: 0x80000, crc: 0xC26B5E55 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.07", offset: 0x200000, size: 0x80000, crc: 0x909A0B8B },
            Op::Load { kind: Load::Word16Swap, file: "dd2.08", offset: 0x280000, size: 0x80000, crc: 0xE53C4D01 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.09", offset: 0x300000, size: 0x80000, crc: 0x5F86279F },
            Op::Load { kind: Load::Word16Swap, file: "dd2d.10", offset: 0x380000, size: 0x80000, crc: 0x0C172F8F },
        ] },
        RegionDef { name: "gfx", size: 0x1800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dd2.13m", offset: 0x0, size: 0x400000, crc: 0xA46B4E6E },
            Op::Load { kind: Load::Word64, file: "dd2.15m", offset: 0x2, size: 0x400000, crc: 0xD5FC50FC },
            Op::Load { kind: Load::Word64, file: "dd2.17m", offset: 0x4, size: 0x400000, crc: 0x837C0867 },
            Op::Load { kind: Load::Word64, file: "dd2.19m", offset: 0x6, size: 0x400000, crc: 0xBB0EC21C },
            Op::Load { kind: Load::Word64, file: "dd2.14m", offset: 0x1000000, size: 0x200000, crc: 0x6D824CE2 },
            Op::Load { kind: Load::Word64, file: "dd2.16m", offset: 0x1000002, size: 0x200000, crc: 0x79682AE5 },
            Op::Load { kind: Load::Word64, file: "dd2.18m", offset: 0x1000004, size: 0x200000, crc: 0xACDDD149 },
            Op::Load { kind: Load::Word64, file: "dd2.20m", offset: 0x1000006, size: 0x200000, crc: 0x117FB0C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dd2.01", offset: 0x0, size: 0x8000, crc: 0x99D657E5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "dd2.02", offset: 0x28000, size: 0x20000, crc: 0x117A3824 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "dd2.11m", offset: 0x0, size: 0x200000, crc: 0x98D0C325 },
            Op::Load { kind: Load::Word16Swap, file: "dd2.12m", offset: 0x200000, size: 0x200000, crc: 0x5EA2E7FA },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ECOFGHTRD: RomSetDef = RomSetDef { name: "ecofghtrd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ueced.03", offset: 0x0, size: 0x80000, crc: 0xAC725D2B },
            Op::Load { kind: Load::Word16Swap, file: "ueced.04", offset: 0x80000, size: 0x80000, crc: 0xF800138D },
            Op::Load { kind: Load::Word16Swap, file: "ueced.05", offset: 0x100000, size: 0x80000, crc: 0xEB6A12F2 },
            Op::Load { kind: Load::Word16Swap, file: "ueced.06", offset: 0x180000, size: 0x80000, crc: 0x8380EC9A },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "uec.13m", offset: 0x0, size: 0x200000, crc: 0xDCAF1436 },
            Op::Load { kind: Load::Word64, file: "uec.15m", offset: 0x2, size: 0x200000, crc: 0x2807DF41 },
            Op::Load { kind: Load::Word64, file: "uec.17m", offset: 0x4, size: 0x200000, crc: 0x8A708D02 },
            Op::Load { kind: Load::Word64, file: "uec.19m", offset: 0x6, size: 0x200000, crc: 0xDE7BE0EF },
            Op::Load { kind: Load::Word64, file: "uec.14m", offset: 0x800000, size: 0x100000, crc: 0x1A003558 },
            Op::Load { kind: Load::Word64, file: "uec.16m", offset: 0x800002, size: 0x100000, crc: 0x4FF8A6F9 },
            Op::Load { kind: Load::Word64, file: "uec.18m", offset: 0x800004, size: 0x100000, crc: 0xB167AE12 },
            Op::Load { kind: Load::Word64, file: "uec.20m", offset: 0x800006, size: 0x100000, crc: 0x1064BDC2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "uec.01", offset: 0x0, size: 0x8000, crc: 0xC235BD15 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "uec.11m", offset: 0x0, size: 0x200000, crc: 0x81B25D39 },
            Op::Load { kind: Load::Word16Swap, file: "uec.12m", offset: 0x200000, size: 0x200000, crc: 0x27729E52 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_GIGAWINGJD: RomSetDef = RomSetDef { name: "gigawingjd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwjd.03a", offset: 0x0, size: 0x80000, crc: 0xCB1C756E },
            Op::Load { kind: Load::Word16Swap, file: "ggwjd.04a", offset: 0x80000, size: 0x80000, crc: 0xFA158E04 },
            Op::Load { kind: Load::Word16Swap, file: "ggwjd.05a", offset: 0x100000, size: 0x80000, crc: 0x1C5BC4E7 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_GIGAWINGD: RomSetDef = RomSetDef { name: "gigawingd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggwu_d.03", offset: 0x0, size: 0x80000, crc: 0xDDE92DFA },
            Op::Load { kind: Load::Word16Swap, file: "ggwu_d.04", offset: 0x80000, size: 0x80000, crc: 0xE0509AE2 },
            Op::Load { kind: Load::Word16Swap, file: "ggw_d.05", offset: 0x100000, size: 0x80000, crc: 0x722D0042 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ggw.13m", offset: 0x0, size: 0x400000, crc: 0x105530A4 },
            Op::Load { kind: Load::Word64, file: "ggw.15m", offset: 0x2, size: 0x400000, crc: 0x9E774AB9 },
            Op::Load { kind: Load::Word64, file: "ggw.17m", offset: 0x4, size: 0x400000, crc: 0x466E0BA4 },
            Op::Load { kind: Load::Word64, file: "ggw.19m", offset: 0x6, size: 0x400000, crc: 0x840C8DEA },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ggw.01", offset: 0x0, size: 0x8000, crc: 0x4C6351D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ggw.11m", offset: 0x0, size: 0x400000, crc: 0xE172ACF5 },
            Op::Load { kind: Load::Word16Swap, file: "ggw.12m", offset: 0x400000, size: 0x400000, crc: 0x4BEE4E8F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_HSF2D: RomSetDef = RomSetDef { name: "hsf2d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2ad.03", offset: 0x0, size: 0x80000, crc: 0x0153D371 },
            Op::Load { kind: Load::Word16Swap, file: "hs2ad.04", offset: 0x80000, size: 0x80000, crc: 0x0276B78A },
            Op::Load { kind: Load::Word16Swap, file: "hs2.05", offset: 0x100000, size: 0x80000, crc: 0xDDE34A35 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.06", offset: 0x180000, size: 0x80000, crc: 0xF4E56DDA },
            Op::Load { kind: Load::Word16Swap, file: "hs2.07", offset: 0x200000, size: 0x80000, crc: 0xEE4420FC },
            Op::Load { kind: Load::Word16Swap, file: "hs2.08", offset: 0x280000, size: 0x80000, crc: 0xC9441533 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.09", offset: 0x300000, size: 0x80000, crc: 0x3FC638A8 },
            Op::Load { kind: Load::Word16Swap, file: "hs2.10", offset: 0x380000, size: 0x80000, crc: 0x20D0F9E4 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "hs2.13m", offset: 0x0, size: 0x800000, crc: 0xA6ECAB17 },
            Op::Load { kind: Load::Word64, file: "hs2.15m", offset: 0x2, size: 0x800000, crc: 0x10A0AE4D },
            Op::Load { kind: Load::Word64, file: "hs2.17m", offset: 0x4, size: 0x800000, crc: 0xADFA7726 },
            Op::Load { kind: Load::Word64, file: "hs2.19m", offset: 0x6, size: 0x800000, crc: 0xBB3AE322 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "hs2.01", offset: 0x0, size: 0x8000, crc: 0xC1A13786 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "hs2.02", offset: 0x28000, size: 0x20000, crc: 0x2D8794AA },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "hs2.11m", offset: 0x0, size: 0x800000, crc: 0x0E15C359 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_MEGAMN2D: RomSetDef = RomSetDef { name: "megamn2d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2ud.03", offset: 0x0, size: 0x80000, crc: 0xD3635F25 },
            Op::Load { kind: Load::Word16Swap, file: "rm2ud.04", offset: 0x80000, size: 0x80000, crc: 0x768A1705 },
            Op::Load { kind: Load::Word16Swap, file: "rm2.05", offset: 0x100000, size: 0x80000, crc: 0x02EE9EFC },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "rm2.14m", offset: 0x800000, size: 0x200000, crc: 0x9B1F00B4 },
            Op::Load { kind: Load::Word64, file: "rm2.16m", offset: 0x800002, size: 0x200000, crc: 0xC2BB0C24 },
            Op::Load { kind: Load::Word64, file: "rm2.18m", offset: 0x800004, size: 0x200000, crc: 0x12257251 },
            Op::Load { kind: Load::Word64, file: "rm2.20m", offset: 0x800006, size: 0x200000, crc: 0xF9B6E786 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rm2.01a", offset: 0x0, size: 0x8000, crc: 0xD18E7859 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "rm2.02", offset: 0x28000, size: 0x20000, crc: 0xC463ECE0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rm2.11m", offset: 0x0, size: 0x200000, crc: 0x2106174D },
            Op::Load { kind: Load::Word16Swap, file: "rm2.12m", offset: 0x200000, size: 0x200000, crc: 0x546C1636 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_MVSCUD: RomSetDef = RomSetDef { name: "mvscud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvcud.03d", offset: 0x0, size: 0x80000, crc: 0x75CDE3E5 },
            Op::Load { kind: Load::Word16Swap, file: "mvcud.04d", offset: 0x80000, size: 0x80000, crc: 0xB32EA484 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.05a", offset: 0x100000, size: 0x80000, crc: 0x2D8C8E86 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.06a", offset: 0x180000, size: 0x80000, crc: 0x8528E1F5 },
            Op::Load { kind: Load::Word16Swap, file: "mvcd.07", offset: 0x200000, size: 0x80000, crc: 0x205293E9 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.08", offset: 0x280000, size: 0x80000, crc: 0xBC002FCD },
            Op::Load { kind: Load::Word16Swap, file: "mvc.09", offset: 0x300000, size: 0x80000, crc: 0xC67B26DF },
            Op::Load { kind: Load::Word16Swap, file: "mvc.10", offset: 0x380000, size: 0x80000, crc: 0x0FDD1E26 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvc.13m", offset: 0x0, size: 0x400000, crc: 0xFA5F74BC },
            Op::Load { kind: Load::Word64, file: "mvc.15m", offset: 0x2, size: 0x400000, crc: 0x71938A8F },
            Op::Load { kind: Load::Word64, file: "mvc.17m", offset: 0x4, size: 0x400000, crc: 0x92741D07 },
            Op::Load { kind: Load::Word64, file: "mvc.19m", offset: 0x6, size: 0x400000, crc: 0xBCB72FC6 },
            Op::Load { kind: Load::Word64, file: "mvc.14m", offset: 0x1000000, size: 0x400000, crc: 0x7F1DF4E4 },
            Op::Load { kind: Load::Word64, file: "mvc.16m", offset: 0x1000002, size: 0x400000, crc: 0x90BD3203 },
            Op::Load { kind: Load::Word64, file: "mvc.18m", offset: 0x1000004, size: 0x400000, crc: 0x67AAF727 },
            Op::Load { kind: Load::Word64, file: "mvc.20m", offset: 0x1000006, size: 0x400000, crc: 0x8B0BADE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvc.01", offset: 0x0, size: 0x8000, crc: 0x41629E95 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvc.02", offset: 0x28000, size: 0x20000, crc: 0x963ABF6B },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvc.11m", offset: 0x0, size: 0x400000, crc: 0x850FE663 },
            Op::Load { kind: Load::Word16Swap, file: "mvc.12m", offset: 0x400000, size: 0x400000, crc: 0x7CCB1896 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_NWARRUD: RomSetDef = RomSetDef { name: "nwarrud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vphud.03f", offset: 0x0, size: 0x80000, crc: 0x20D4D5A8 },
            Op::Load { kind: Load::Word16Swap, file: "vphud.04c", offset: 0x80000, size: 0x80000, crc: 0x61BE9B42 },
            Op::Load { kind: Load::Word16Swap, file: "vphud.05e", offset: 0x100000, size: 0x80000, crc: 0x1BA906D8 },
            Op::Load { kind: Load::Word16Swap, file: "vphu.06c", offset: 0x180000, size: 0x80000, crc: 0x08C04CDB },
            Op::Load { kind: Load::Word16Swap, file: "vphu.07b", offset: 0x200000, size: 0x80000, crc: 0xB5A5AB19 },
            Op::Load { kind: Load::Word16Swap, file: "vphu.08b", offset: 0x280000, size: 0x80000, crc: 0x51BB20FB },
            Op::Load { kind: Load::Word16Swap, file: "vphu.09b", offset: 0x300000, size: 0x80000, crc: 0x41A64205 },
            Op::Load { kind: Load::Word16Swap, file: "vphud.10b", offset: 0x380000, size: 0x80000, crc: 0x9619ADAD },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vph.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vph.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vph.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vph.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vph.14m", offset: 0x1000000, size: 0x400000, crc: 0x7A0E1ADD },
            Op::Load { kind: Load::Word64, file: "vph.16m", offset: 0x1000002, size: 0x400000, crc: 0x2F41CA75 },
            Op::Load { kind: Load::Word64, file: "vph.18m", offset: 0x1000004, size: 0x400000, crc: 0x64498EED },
            Op::Load { kind: Load::Word64, file: "vph.20m", offset: 0x1000006, size: 0x400000, crc: 0x17F2433F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vph.01", offset: 0x0, size: 0x8000, crc: 0x5045DCAC },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vph.02", offset: 0x28000, size: 0x20000, crc: 0x86B60E59 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vph.11m", offset: 0x0, size: 0x200000, crc: 0xE1837D33 },
            Op::Load { kind: Load::Word16Swap, file: "vph.12m", offset: 0x200000, size: 0x200000, crc: 0xFBD3CD90 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_RINGDSTD: RomSetDef = RomSetDef { name: "ringdstd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smbed.03b", offset: 0x0, size: 0x80000, crc: 0xF6FBA4CD },
            Op::Load { kind: Load::Word16Swap, file: "smbed.04b", offset: 0x80000, size: 0x80000, crc: 0x193BC493 },
            Op::Load { kind: Load::Word16Swap, file: "smbed.05b", offset: 0x100000, size: 0x80000, crc: 0x168CCCBB },
            Op::Load { kind: Load::Word16Swap, file: "smbed.06b", offset: 0x180000, size: 0x80000, crc: 0x04673262 },
            Op::Load { kind: Load::Word16Swap, file: "smb.07", offset: 0x200000, size: 0x80000, crc: 0xB9A11577 },
            Op::Load { kind: Load::Word16Swap, file: "smb.08", offset: 0x280000, size: 0x80000, crc: 0xF931B76B },
        ] },
        RegionDef { name: "gfx", size: 0x1200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "smb.13m", offset: 0x0, size: 0x200000, crc: 0xD9B2D1DE },
            Op::Load { kind: Load::Word64, file: "smb.15m", offset: 0x2, size: 0x200000, crc: 0x9A766D92 },
            Op::Load { kind: Load::Word64, file: "smb.17m", offset: 0x4, size: 0x200000, crc: 0x51800F0F },
            Op::Load { kind: Load::Word64, file: "smb.19m", offset: 0x6, size: 0x200000, crc: 0x35757E96 },
            Op::Load { kind: Load::Word64, file: "smb.14m", offset: 0x800000, size: 0x200000, crc: 0xE5BFD0E7 },
            Op::Load { kind: Load::Word64, file: "smb.16m", offset: 0x800002, size: 0x200000, crc: 0xC56C0866 },
            Op::Load { kind: Load::Word64, file: "smb.18m", offset: 0x800004, size: 0x200000, crc: 0x4DED3910 },
            Op::Load { kind: Load::Word64, file: "smb.20m", offset: 0x800006, size: 0x200000, crc: 0x26EA1EC5 },
            Op::Load { kind: Load::Word64, file: "smb.21m", offset: 0x1000000, size: 0x80000, crc: 0x0A08C5FC },
            Op::Load { kind: Load::Word64, file: "smb.23m", offset: 0x1000002, size: 0x80000, crc: 0x0911B6C4 },
            Op::Load { kind: Load::Word64, file: "smb.25m", offset: 0x1000004, size: 0x80000, crc: 0x82D6C4EC },
            Op::Load { kind: Load::Word64, file: "smb.27m", offset: 0x1000006, size: 0x80000, crc: 0x9B48678B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "smb.01", offset: 0x0, size: 0x8000, crc: 0x0ABC229A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "smb.02", offset: 0x28000, size: 0x20000, crc: 0xD051679A },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "smb.11m", offset: 0x0, size: 0x200000, crc: 0xC56935F9 },
            Op::Load { kind: Load::Word16Swap, file: "smb.12m", offset: 0x200000, size: 0x200000, crc: 0x955B0782 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_SFAD: RomSetDef = RomSetDef { name: "sfad", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzed.03d", offset: 0x0, size: 0x80000, crc: 0xA1A54827 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.04b", offset: 0x80000, size: 0x80000, crc: 0x8B73B0E5 },
            Op::Load { kind: Load::Word16Swap, file: "sfz.05a", offset: 0x100000, size: 0x80000, crc: 0x0810544D },
            Op::Load { kind: Load::Word16Swap, file: "sfz.06", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "sfz.14m", offset: 0x800000, size: 0x200000, crc: 0x90FEFDB3 },
            Op::Load { kind: Load::Word64, file: "sfz.16m", offset: 0x800002, size: 0x200000, crc: 0x5354C948 },
            Op::Load { kind: Load::Word64, file: "sfz.18m", offset: 0x800004, size: 0x200000, crc: 0x41A1E790 },
            Op::Load { kind: Load::Word64, file: "sfz.20m", offset: 0x800006, size: 0x200000, crc: 0xA549DF98 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz.01", offset: 0x0, size: 0x8000, crc: 0xFFFFEC7D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfz.02", offset: 0x28000, size: 0x20000, crc: 0x45F46A08 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfz.11m", offset: 0x0, size: 0x200000, crc: 0xC4B093CD },
            Op::Load { kind: Load::Word16Swap, file: "sfz.12m", offset: 0x200000, size: 0x200000, crc: 0x8BDBC4B4 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHUD: RomSetDef = RomSetDef { name: "mshud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mshud.03", offset: 0x0, size: 0x80000, crc: 0xC1D8C4C6 },
            Op::Load { kind: Load::Word16Swap, file: "mshud.04", offset: 0x80000, size: 0x80000, crc: 0xE73DDA16 },
            Op::Load { kind: Load::Word16Swap, file: "mshud.05", offset: 0x100000, size: 0x80000, crc: 0x3B493E84 },
            Op::Load { kind: Load::Word16Swap, file: "msh.06b", offset: 0x180000, size: 0x80000, crc: 0x803E3FA4 },
            Op::Load { kind: Load::Word16Swap, file: "msh.07a", offset: 0x200000, size: 0x80000, crc: 0xC45F8E27 },
            Op::Load { kind: Load::Word16Swap, file: "msh.08a", offset: 0x280000, size: 0x80000, crc: 0x9CA6F12C },
            Op::Load { kind: Load::Word16Swap, file: "msh.09a", offset: 0x300000, size: 0x80000, crc: 0x82EC27AF },
            Op::Load { kind: Load::Word16Swap, file: "msh.10b", offset: 0x380000, size: 0x80000, crc: 0x8D931196 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "msh.13m", offset: 0x0, size: 0x400000, crc: 0x09D14566 },
            Op::Load { kind: Load::Word64, file: "msh.15m", offset: 0x2, size: 0x400000, crc: 0xEE962057 },
            Op::Load { kind: Load::Word64, file: "msh.17m", offset: 0x4, size: 0x400000, crc: 0x604ECE14 },
            Op::Load { kind: Load::Word64, file: "msh.19m", offset: 0x6, size: 0x400000, crc: 0x94A731E8 },
            Op::Load { kind: Load::Word64, file: "msh.14m", offset: 0x1000000, size: 0x400000, crc: 0x4197973E },
            Op::Load { kind: Load::Word64, file: "msh.16m", offset: 0x1000002, size: 0x400000, crc: 0x438DA4A0 },
            Op::Load { kind: Load::Word64, file: "msh.18m", offset: 0x1000004, size: 0x400000, crc: 0x4DB92D94 },
            Op::Load { kind: Load::Word64, file: "msh.20m", offset: 0x1000006, size: 0x400000, crc: 0xA2B0C6C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "msh.01", offset: 0x0, size: 0x8000, crc: 0xC976E6F9 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "msh.02", offset: 0x28000, size: 0x20000, crc: 0xCE67D0D9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "msh.11m", offset: 0x0, size: 0x200000, crc: 0x37AC6D30 },
            Op::Load { kind: Load::Word16Swap, file: "msh.12m", offset: 0x200000, size: 0x200000, crc: 0xDE092570 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_SFZ2ALD: RomSetDef = RomSetDef { name: "sfz2ald", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "szaad.03", offset: 0x0, size: 0x80000, crc: 0x89F9483B },
            Op::Load { kind: Load::Word16Swap, file: "szaad.04", offset: 0x80000, size: 0x80000, crc: 0xAEF27AE5 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.05", offset: 0x100000, size: 0x80000, crc: 0xF053A55E },
            Op::Load { kind: Load::Word16Swap, file: "szaa.06", offset: 0x180000, size: 0x80000, crc: 0xCFC0E7A8 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.07", offset: 0x200000, size: 0x80000, crc: 0x5FEB8B20 },
            Op::Load { kind: Load::Word16Swap, file: "szaa.08", offset: 0x280000, size: 0x80000, crc: 0x6EB6D412 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_SFA3UD: RomSetDef = RomSetDef { name: "sfa3ud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3ud.03c", offset: 0x0, size: 0x80000, crc: 0x6DB8ADD7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3ud.04c", offset: 0x80000, size: 0x80000, crc: 0xD9C65A26 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05c", offset: 0x100000, size: 0x80000, crc: 0x57FD0A40 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06c", offset: 0x180000, size: 0x80000, crc: 0xF6305F8B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07c", offset: 0x200000, size: 0x80000, crc: 0x6EAB0F6F },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08c", offset: 0x280000, size: 0x80000, crc: 0x910C4A3B },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09c", offset: 0x300000, size: 0x80000, crc: 0xB29E5199 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10b", offset: 0x380000, size: 0x80000, crc: 0xDEB2FF52 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_SPF2XJD: RomSetDef = RomSetDef { name: "spf2xjd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfjd.03a", offset: 0x0, size: 0x80000, crc: 0x5E85ED08 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_VSAVD: RomSetDef = RomSetDef { name: "vsavd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3ed.03d", offset: 0x0, size: 0x80000, crc: 0x97D805E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3ed.04d", offset: 0x80000, size: 0x80000, crc: 0x5E07FDCE },
            Op::Load { kind: Load::Word16Swap, file: "vm3.05a", offset: 0x100000, size: 0x80000, crc: 0x4118E00F },
            Op::Load { kind: Load::Word16Swap, file: "vm3.06a", offset: 0x180000, size: 0x80000, crc: 0x2F4FD3A9 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.07b", offset: 0x200000, size: 0x80000, crc: 0xCBDA91B8 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.08a", offset: 0x280000, size: 0x80000, crc: 0x6CA47259 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.09b", offset: 0x300000, size: 0x80000, crc: 0xF4A339E3 },
            Op::Load { kind: Load::Word16Swap, file: "vm3.10b", offset: 0x380000, size: 0x80000, crc: 0xFFFBB5B8 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vm3.13m", offset: 0x0, size: 0x400000, crc: 0xFD8A11EB },
            Op::Load { kind: Load::Word64, file: "vm3.15m", offset: 0x2, size: 0x400000, crc: 0xDD1E7D4E },
            Op::Load { kind: Load::Word64, file: "vm3.17m", offset: 0x4, size: 0x400000, crc: 0x6B89445E },
            Op::Load { kind: Load::Word64, file: "vm3.19m", offset: 0x6, size: 0x400000, crc: 0x3830FDC7 },
            Op::Load { kind: Load::Word64, file: "vm3.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1A28E6C },
            Op::Load { kind: Load::Word64, file: "vm3.16m", offset: 0x1000002, size: 0x400000, crc: 0x194A7304 },
            Op::Load { kind: Load::Word64, file: "vm3.18m", offset: 0x1000004, size: 0x400000, crc: 0xDF9A9F47 },
            Op::Load { kind: Load::Word64, file: "vm3.20m", offset: 0x1000006, size: 0x400000, crc: 0xC22FC3D9 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vm3.01", offset: 0x0, size: 0x8000, crc: 0xF778769B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vm3.02", offset: 0x28000, size: 0x20000, crc: 0xCC09FAA1 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vm3.11m", offset: 0x0, size: 0x400000, crc: 0xE80E956E },
            Op::Load { kind: Load::Word16Swap, file: "vm3.12m", offset: 0x400000, size: 0x400000, crc: 0x9CD71557 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_XMCOTAR1D: RomSetDef = RomSetDef { name: "xmcotar1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmned.03e", offset: 0x0, size: 0x80000, crc: 0xBEF56003 },
            Op::Load { kind: Load::Word16Swap, file: "xmned.04e", offset: 0x80000, size: 0x80000, crc: 0xB1A21FA6 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.05a", offset: 0x100000, size: 0x80000, crc: 0xAC0D7759 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.06a", offset: 0x180000, size: 0x80000, crc: 0x1B86A328 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.07a", offset: 0x200000, size: 0x80000, crc: 0x2C142A44 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.08a", offset: 0x280000, size: 0x80000, crc: 0xF712D44F },
            Op::Load { kind: Load::Word16Swap, file: "xmn.09a", offset: 0x300000, size: 0x80000, crc: 0x9241CAE8 },
            Op::Load { kind: Load::Word16Swap, file: "xmn.10a", offset: 0x380000, size: 0x80000, crc: 0x53C0EAB9 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xmn.13m", offset: 0x0, size: 0x400000, crc: 0xBF4DF073 },
            Op::Load { kind: Load::Word64, file: "xmn.15m", offset: 0x2, size: 0x400000, crc: 0x4D7E4CEF },
            Op::Load { kind: Load::Word64, file: "xmn.17m", offset: 0x4, size: 0x400000, crc: 0x513EEA17 },
            Op::Load { kind: Load::Word64, file: "xmn.19m", offset: 0x6, size: 0x400000, crc: 0xD23897FC },
            Op::Load { kind: Load::Word64, file: "xmn.14m", offset: 0x1000000, size: 0x400000, crc: 0x778237B7 },
            Op::Load { kind: Load::Word64, file: "xmn.16m", offset: 0x1000002, size: 0x400000, crc: 0x67B36948 },
            Op::Load { kind: Load::Word64, file: "xmn.18m", offset: 0x1000004, size: 0x400000, crc: 0x015A7C4C },
            Op::Load { kind: Load::Word64, file: "xmn.20m", offset: 0x1000006, size: 0x400000, crc: 0x9DDE2758 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmn.01a", offset: 0x0, size: 0x8000, crc: 0x40F479EA },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xmn.02a", offset: 0x28000, size: 0x20000, crc: 0x39D9B5AD },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xmn.11m", offset: 0x0, size: 0x200000, crc: 0xC848A6BC },
            Op::Load { kind: Load::Word16Swap, file: "xmn.12m", offset: 0x200000, size: 0x200000, crc: 0x729C188F },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_XMVSFU1D: RomSetDef = RomSetDef { name: "xmvsfu1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvsud.03h", offset: 0x0, size: 0x80000, crc: 0x4E2E76B7 },
            Op::Load { kind: Load::Word16Swap, file: "xvsud.04h", offset: 0x80000, size: 0x80000, crc: 0x290C61A7 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.05a", offset: 0x100000, size: 0x80000, crc: 0x7DB6025D },
            Op::Load { kind: Load::Word16Swap, file: "xvs.06a", offset: 0x180000, size: 0x80000, crc: 0xE8E2C75C },
            Op::Load { kind: Load::Word16Swap, file: "xvs.07", offset: 0x200000, size: 0x80000, crc: 0x08F0ABED },
            Op::Load { kind: Load::Word16Swap, file: "xvs.08", offset: 0x280000, size: 0x80000, crc: 0x81929675 },
            Op::Load { kind: Load::Word16Swap, file: "xvs.09", offset: 0x300000, size: 0x80000, crc: 0x9641F36B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "xvs.13m", offset: 0x0, size: 0x400000, crc: 0xF6684EFD },
            Op::Load { kind: Load::Word64, file: "xvs.15m", offset: 0x2, size: 0x400000, crc: 0x29109221 },
            Op::Load { kind: Load::Word64, file: "xvs.17m", offset: 0x4, size: 0x400000, crc: 0x92DB3474 },
            Op::Load { kind: Load::Word64, file: "xvs.19m", offset: 0x6, size: 0x400000, crc: 0x3733473C },
            Op::Load { kind: Load::Word64, file: "xvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xBCAC2E41 },
            Op::Load { kind: Load::Word64, file: "xvs.16m", offset: 0x1000002, size: 0x400000, crc: 0xEA04A272 },
            Op::Load { kind: Load::Word64, file: "xvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xB0DEF86A },
            Op::Load { kind: Load::Word64, file: "xvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x4B40FF9F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xvs.01", offset: 0x0, size: 0x8000, crc: 0x3999E93A },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "xvs.02", offset: 0x28000, size: 0x20000, crc: 0x101BDEE9 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "xvs.11m", offset: 0x0, size: 0x200000, crc: 0x9CADCDBC },
            Op::Load { kind: Load::Word16Swap, file: "xvs.12m", offset: 0x200000, size: 0x200000, crc: 0x7B11E460 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DSTLKU1D: RomSetDef = RomSetDef { name: "dstlku1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vamud.03a", offset: 0x0, size: 0x80000, crc: 0x47B7A680 },
            Op::Load { kind: Load::Word16Swap, file: "vamud.04a", offset: 0x80000, size: 0x80000, crc: 0x3B7A4939 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.05a", offset: 0x100000, size: 0x80000, crc: 0x673ED50A },
            Op::Load { kind: Load::Word16Swap, file: "vamu.06a", offset: 0x180000, size: 0x80000, crc: 0xF2377BE7 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.07a", offset: 0x200000, size: 0x80000, crc: 0xD8F498C4 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.08a", offset: 0x280000, size: 0x80000, crc: 0xE6A8A1A0 },
            Op::Load { kind: Load::Word16Swap, file: "vamud.09a", offset: 0x300000, size: 0x80000, crc: 0x8B333A19 },
            Op::Load { kind: Load::Word16Swap, file: "vamu.10a", offset: 0x380000, size: 0x80000, crc: 0xC1A3D9BE },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vam.13m", offset: 0x0, size: 0x400000, crc: 0xC51BAF99 },
            Op::Load { kind: Load::Word64, file: "vam.15m", offset: 0x2, size: 0x400000, crc: 0x3CE83C77 },
            Op::Load { kind: Load::Word64, file: "vam.17m", offset: 0x4, size: 0x400000, crc: 0x4F2408E0 },
            Op::Load { kind: Load::Word64, file: "vam.19m", offset: 0x6, size: 0x400000, crc: 0x9FF60250 },
            Op::Load { kind: Load::Word64, file: "vam.14m", offset: 0x1000000, size: 0x100000, crc: 0xBD87243C },
            Op::Load { kind: Load::Word64, file: "vam.16m", offset: 0x1000002, size: 0x100000, crc: 0xAFEC855F },
            Op::Load { kind: Load::Word64, file: "vam.18m", offset: 0x1000004, size: 0x100000, crc: 0x3A033625 },
            Op::Load { kind: Load::Word64, file: "vam.20m", offset: 0x1000006, size: 0x100000, crc: 0x2BFF6A89 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vam.01", offset: 0x0, size: 0x8000, crc: 0x64B685D5 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vam.02", offset: 0x28000, size: 0x20000, crc: 0xCF7C97C7 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vam.11m", offset: 0x0, size: 0x200000, crc: 0x4A39DEB2 },
            Op::Load { kind: Load::Word16Swap, file: "vam.12m", offset: 0x200000, size: 0x200000, crc: 0x1A3E5C03 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEARUD: RomSetDef = RomSetDef { name: "progearud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgau_d.03", offset: 0x0, size: 0x80000, crc: 0xBA22B9C5 },
            Op::Load { kind: Load::Word16Swap, file: "pgau_d.04", offset: 0x80000, size: 0x80000, crc: 0xDF3927EF },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pga-simm.01c", offset: 0x0, size: 0x200000, crc: 0x452F98B0 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01d", offset: 0x1, size: 0x200000, crc: 0x9E672092 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01a", offset: 0x2, size: 0x200000, crc: 0xAE9DDAFE },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01b", offset: 0x3, size: 0x200000, crc: 0x94D72D94 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03c", offset: 0x4, size: 0x200000, crc: 0x48A1886D },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03d", offset: 0x5, size: 0x200000, crc: 0x172D7E37 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03a", offset: 0x6, size: 0x200000, crc: 0x9EE33D98 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03b", offset: 0x7, size: 0x200000, crc: 0x848DEE32 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05a", offset: 0x0, size: 0x200000, crc: 0xC0AAC80C },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x37A65D86 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06a", offset: 0x400000, size: 0x200000, crc: 0xD3F1E934 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06b", offset: 0x600000, size: 0x200000, crc: 0x8B39489A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEARJD: RomSetDef = RomSetDef { name: "progearjd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgaj_d.03", offset: 0x0, size: 0x80000, crc: 0x0271F3A3 },
            Op::Load { kind: Load::Word16Swap, file: "pgaj_d.04", offset: 0x80000, size: 0x80000, crc: 0xBE4B7799 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pga-simm.01c", offset: 0x0, size: 0x200000, crc: 0x452F98B0 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01d", offset: 0x1, size: 0x200000, crc: 0x9E672092 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01a", offset: 0x2, size: 0x200000, crc: 0xAE9DDAFE },
            Op::Load { kind: Load::Byte64, file: "pga-simm.01b", offset: 0x3, size: 0x200000, crc: 0x94D72D94 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03c", offset: 0x4, size: 0x200000, crc: 0x48A1886D },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03d", offset: 0x5, size: 0x200000, crc: 0x172D7E37 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03a", offset: 0x6, size: 0x200000, crc: 0x9EE33D98 },
            Op::Load { kind: Load::Byte64, file: "pga-simm.03b", offset: 0x7, size: 0x200000, crc: 0x848DEE32 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05a", offset: 0x0, size: 0x200000, crc: 0xC0AAC80C },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x37A65D86 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06a", offset: 0x400000, size: 0x200000, crc: 0xD3F1E934 },
            Op::Load { kind: Load::Word16Swap, file: "pga-simm.06b", offset: 0x600000, size: 0x200000, crc: 0x8B39489A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PROGEARJBL: RomSetDef = RomSetDef { name: "progearjbl", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pgaj_bl.03", offset: 0x0, size: 0x80000, crc: 0x4FEF676C },
            Op::Load { kind: Load::Word16Swap, file: "pgaj_bl.04", offset: 0x80000, size: 0x80000, crc: 0xA069BD3B },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pga.13m", offset: 0x0, size: 0x400000, crc: 0x5194C198 },
            Op::Load { kind: Load::Word64, file: "pga.15m", offset: 0x2, size: 0x400000, crc: 0xB794E83F },
            Op::Load { kind: Load::Word64, file: "pga.17m", offset: 0x4, size: 0x400000, crc: 0x87F22918 },
            Op::Load { kind: Load::Word64, file: "pga.19m", offset: 0x6, size: 0x400000, crc: 0x65FFB45B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pga.01", offset: 0x0, size: 0x8000, crc: 0xBDBFA992 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pga.11m", offset: 0x0, size: 0x400000, crc: 0xABDD224E },
            Op::Load { kind: Load::Word16Swap, file: "pga.12m", offset: 0x400000, size: 0x400000, crc: 0xDAC53406 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2UD: RomSetDef = RomSetDef { name: "ssf2ud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfud.03a", offset: 0x0, size: 0x80000, crc: 0xFAD5DAF8 },
            Op::Load { kind: Load::Word16Swap, file: "ssfud.04a", offset: 0x80000, size: 0x80000, crc: 0x0D31AF65 },
            Op::Load { kind: Load::Word16Swap, file: "ssfud.05", offset: 0x100000, size: 0x80000, crc: 0x75C651EF },
            Op::Load { kind: Load::Word16Swap, file: "ssfud.06", offset: 0x180000, size: 0x80000, crc: 0x85C3EC00 },
            Op::Load { kind: Load::Word16Swap, file: "ssfud.07", offset: 0x200000, size: 0x80000, crc: 0x247E2504 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TBD: RomSetDef = RomSetDef { name: "ssf2tbd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ssfed.3tc", offset: 0x0, size: 0x80000, crc: 0x5D86CAF8 },
            Op::Load { kind: Load::Word16Swap, file: "ssfed.4tc", offset: 0x80000, size: 0x80000, crc: 0xF6E1F98D },
            Op::Load { kind: Load::Word16Swap, file: "ssfed.5t", offset: 0x100000, size: 0x80000, crc: 0x75C651EF },
            Op::Load { kind: Load::Word16Swap, file: "ssfed.6tb", offset: 0x180000, size: 0x80000, crc: 0x9ADAC7D7 },
            Op::Load { kind: Load::Word16Swap, file: "ssfed.7t", offset: 0x200000, size: 0x80000, crc: 0x84F54DB3 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ssf.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "ssf.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "ssf.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "ssf.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "ssf.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "ssf.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "ssf.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "ssf.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.01", offset: 0x0, size: 0x8000, crc: 0xEB247E8C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ssf.q01", offset: 0x0, size: 0x80000, crc: 0xA6F9DA5C },
            Op::Load { kind: Load::Byte, file: "ssf.q02", offset: 0x80000, size: 0x80000, crc: 0x8C66AE26 },
            Op::Load { kind: Load::Byte, file: "ssf.q03", offset: 0x100000, size: 0x80000, crc: 0x695CC2CA },
            Op::Load { kind: Load::Byte, file: "ssf.q04", offset: 0x180000, size: 0x80000, crc: 0x9D9EBE32 },
            Op::Load { kind: Load::Byte, file: "ssf.q05", offset: 0x200000, size: 0x80000, crc: 0x4770E7B7 },
            Op::Load { kind: Load::Byte, file: "ssf.q06", offset: 0x280000, size: 0x80000, crc: 0x4E79C951 },
            Op::Load { kind: Load::Byte, file: "ssf.q07", offset: 0x300000, size: 0x80000, crc: 0xCDD14313 },
            Op::Load { kind: Load::Byte, file: "ssf.q08", offset: 0x380000, size: 0x80000, crc: 0x6F5A088C },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2XJR1D: RomSetDef = RomSetDef { name: "ssf2xjr1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxjd.03c", offset: 0x0, size: 0x80000, crc: 0x316DE996 },
            Op::Load { kind: Load::Word16Swap, file: "sfxjd.04a", offset: 0x80000, size: 0x80000, crc: 0x9BF3BB2E },
            Op::Load { kind: Load::Word16Swap, file: "sfxjd.05", offset: 0x100000, size: 0x80000, crc: 0xC63358D0 },
            Op::Load { kind: Load::Word16Swap, file: "sfxjd.06a", offset: 0x180000, size: 0x80000, crc: 0xCCB29808 },
            Op::Load { kind: Load::Word16Swap, file: "sfxjd.07", offset: 0x200000, size: 0x80000, crc: 0x61F94982 },
            Op::Load { kind: Load::Word16Swap, file: "sfxjd.08", offset: 0x280000, size: 0x80000, crc: 0xD399C36C },
            Op::Load { kind: Load::Word16Swap, file: "sfxd.09", offset: 0x300000, size: 0x80000, crc: 0x0B3A6196 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SSF2TAD: RomSetDef = RomSetDef { name: "ssf2tad", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfxad.03c", offset: 0x0, size: 0x80000, crc: 0xE3C92ECE },
            Op::Load { kind: Load::Word16Swap, file: "sfxad.04a", offset: 0x80000, size: 0x80000, crc: 0x9BF3BB2E },
            Op::Load { kind: Load::Word16Swap, file: "sfxad.05", offset: 0x100000, size: 0x80000, crc: 0xC63358D0 },
            Op::Load { kind: Load::Word16Swap, file: "sfxad.06a", offset: 0x180000, size: 0x80000, crc: 0xCCB29808 },
            Op::Load { kind: Load::Word16Swap, file: "sfxad.07", offset: 0x200000, size: 0x80000, crc: 0x61F94982 },
            Op::Load { kind: Load::Word16Swap, file: "sfxad.08", offset: 0x280000, size: 0x80000, crc: 0xD399C36C },
            Op::Load { kind: Load::Word16Swap, file: "sfxad.09", offset: 0x300000, size: 0x80000, crc: 0x436784AE },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfx.13m", offset: 0x0, size: 0x200000, crc: 0xCF94D275 },
            Op::Load { kind: Load::Word64, file: "sfx.15m", offset: 0x2, size: 0x200000, crc: 0x5EB703AF },
            Op::Load { kind: Load::Word64, file: "sfx.17m", offset: 0x4, size: 0x200000, crc: 0xFFA60E0F },
            Op::Load { kind: Load::Word64, file: "sfx.19m", offset: 0x6, size: 0x200000, crc: 0x34E825C5 },
            Op::Load { kind: Load::Word64, file: "sfx.14m", offset: 0x800000, size: 0x100000, crc: 0xB7CC32E7 },
            Op::Load { kind: Load::Word64, file: "sfx.16m", offset: 0x800002, size: 0x100000, crc: 0x8376AD18 },
            Op::Load { kind: Load::Word64, file: "sfx.18m", offset: 0x800004, size: 0x100000, crc: 0xF5B1B336 },
            Op::Load { kind: Load::Word64, file: "sfx.20m", offset: 0x800006, size: 0x100000, crc: 0x459D5C6B },
            Op::Load { kind: Load::Word64, file: "sfx.21m", offset: 0xc00000, size: 0x100000, crc: 0xE32854AF },
            Op::Load { kind: Load::Word64, file: "sfx.23m", offset: 0xc00002, size: 0x100000, crc: 0x760F2927 },
            Op::Load { kind: Load::Word64, file: "sfx.25m", offset: 0xc00004, size: 0x100000, crc: 0x1EE90208 },
            Op::Load { kind: Load::Word64, file: "sfx.27m", offset: 0xc00006, size: 0x100000, crc: 0xF814400F },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfx.01", offset: 0x0, size: 0x8000, crc: 0xB47B8835 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sfx.02", offset: 0x28000, size: 0x20000, crc: 0x0022633F },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfx.11m", offset: 0x0, size: 0x200000, crc: 0x9BDBD476 },
            Op::Load { kind: Load::Word16Swap, file: "sfx.12m", offset: 0x200000, size: 0x200000, crc: 0xA05E3AAB },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SGEMFD: RomSetDef = RomSetDef { name: "sgemfd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcfud.03", offset: 0x0, size: 0x80000, crc: 0x8B83674A },
            Op::Load { kind: Load::Word16Swap, file: "pcfd.04", offset: 0x80000, size: 0x80000, crc: 0xB58F1D03 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.05", offset: 0x100000, size: 0x80000, crc: 0x215655F6 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.06", offset: 0x180000, size: 0x80000, crc: 0xEA6F13EA },
            Op::Load { kind: Load::Word16Swap, file: "pcf.07", offset: 0x200000, size: 0x80000, crc: 0x5AC6D5EA },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pcf.13m", offset: 0x0, size: 0x400000, crc: 0x22D72AB9 },
            Op::Load { kind: Load::Word64, file: "pcf.15m", offset: 0x2, size: 0x400000, crc: 0x16A4813C },
            Op::Load { kind: Load::Word64, file: "pcf.17m", offset: 0x4, size: 0x400000, crc: 0x1097E035 },
            Op::Load { kind: Load::Word64, file: "pcf.19m", offset: 0x6, size: 0x400000, crc: 0xD362D874 },
            Op::Load { kind: Load::Word64, file: "pcf.14m", offset: 0x1000000, size: 0x100000, crc: 0x0383897C },
            Op::Load { kind: Load::Word64, file: "pcf.16m", offset: 0x1000002, size: 0x100000, crc: 0x76F91084 },
            Op::Load { kind: Load::Word64, file: "pcf.18m", offset: 0x1000004, size: 0x100000, crc: 0x756C3754 },
            Op::Load { kind: Load::Word64, file: "pcf.20m", offset: 0x1000006, size: 0x100000, crc: 0x9EC9277D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pcf.01", offset: 0x0, size: 0x8000, crc: 0x254E5F33 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pcf.02", offset: 0x28000, size: 0x20000, crc: 0x6902F4F9 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pcf.11m", offset: 0x0, size: 0x400000, crc: 0xA5DEA005 },
            Op::Load { kind: Load::Word16Swap, file: "pcf.12m", offset: 0x400000, size: 0x400000, crc: 0x4CE235FE },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_ARMWAR1D: RomSetDef = RomSetDef { name: "armwar1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwged.03b", offset: 0x0, size: 0x80000, crc: 0x496BD483 },
            Op::Load { kind: Load::Word16Swap, file: "pwged.04b", offset: 0x80000, size: 0x80000, crc: 0x9BD6A38F },
            Op::Load { kind: Load::Word16Swap, file: "pwged.05a", offset: 0x100000, size: 0x80000, crc: 0x4C11D30F },
            Op::Load { kind: Load::Word16Swap, file: "pwg.06", offset: 0x180000, size: 0x80000, crc: 0x87A60CE8 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.07", offset: 0x200000, size: 0x80000, crc: 0xF7B148DF },
            Op::Load { kind: Load::Word16Swap, file: "pwg.08", offset: 0x280000, size: 0x80000, crc: 0xCC62823E },
            Op::Load { kind: Load::Word16Swap, file: "pwg.09", offset: 0x300000, size: 0x80000, crc: 0xDDC85CA6 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.10", offset: 0x380000, size: 0x80000, crc: 0x07C4FB28 },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pwg.13m", offset: 0x0, size: 0x400000, crc: 0xAE8FE08E },
            Op::Load { kind: Load::Word64, file: "pwg.15m", offset: 0x2, size: 0x400000, crc: 0xDB560F58 },
            Op::Load { kind: Load::Word64, file: "pwg.17m", offset: 0x4, size: 0x400000, crc: 0xBC475B94 },
            Op::Load { kind: Load::Word64, file: "pwg.19m", offset: 0x6, size: 0x400000, crc: 0x07439FF7 },
            Op::Load { kind: Load::Word64, file: "pwg.14m", offset: 0x1000000, size: 0x100000, crc: 0xC3F9BA63 },
            Op::Load { kind: Load::Word64, file: "pwg.16m", offset: 0x1000002, size: 0x100000, crc: 0x815B0E7B },
            Op::Load { kind: Load::Word64, file: "pwg.18m", offset: 0x1000004, size: 0x100000, crc: 0x0109C71B },
            Op::Load { kind: Load::Word64, file: "pwg.20m", offset: 0x1000006, size: 0x100000, crc: 0xEB75FFBE },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pwg.01", offset: 0x0, size: 0x8000, crc: 0x18A5C0E4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pwg.02", offset: 0x28000, size: 0x20000, crc: 0xC9DFFFA6 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pwg.11m", offset: 0x0, size: 0x200000, crc: 0xA78F7433 },
            Op::Load { kind: Load::Word16Swap, file: "pwg.12m", offset: 0x200000, size: 0x200000, crc: 0x77438ED0 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_DIMAHOUD: RomSetDef = RomSetDef { name: "dimahoud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmdud.03", offset: 0x0, size: 0x80000, crc: 0x12888435 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.04", offset: 0x80000, size: 0x80000, crc: 0x37485567 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.05", offset: 0x100000, size: 0x80000, crc: 0xDA269FFB },
            Op::Load { kind: Load::Word16Swap, file: "gmdud.06", offset: 0x180000, size: 0x80000, crc: 0xD825EFDA },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "gmd.13m", offset: 0x0, size: 0x400000, crc: 0x80DD19F0 },
            Op::Load { kind: Load::Word64, file: "gmd.15m", offset: 0x2, size: 0x400000, crc: 0xDFD93A78 },
            Op::Load { kind: Load::Word64, file: "gmd.17m", offset: 0x4, size: 0x400000, crc: 0x16356520 },
            Op::Load { kind: Load::Word64, file: "gmd.19m", offset: 0x6, size: 0x400000, crc: 0xDFC33031 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gmd.01", offset: 0x0, size: 0x8000, crc: 0x3F9BC985 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "gmd.02", offset: 0x28000, size: 0x20000, crc: 0x3FD39DDE },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gmd.11m", offset: 0x0, size: 0x400000, crc: 0x06A65542 },
            Op::Load { kind: Load::Word16Swap, file: "gmd.12m", offset: 0x400000, size: 0x400000, crc: 0x50BC7A31 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MMATRIXD: RomSetDef = RomSetDef { name: "mmatrixd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmxud.03", offset: 0x0, size: 0x80000, crc: 0x36711E60 },
            Op::Load { kind: Load::Word16Swap, file: "mmxud.04", offset: 0x80000, size: 0x80000, crc: 0x4687226F },
            Op::Load { kind: Load::Word16Swap, file: "mmxud.05", offset: 0x100000, size: 0x80000, crc: 0x52124398 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mmx.13m", offset: 0x0, size: 0x400000, crc: 0x04748718 },
            Op::Load { kind: Load::Word64, file: "mmx.15m", offset: 0x2, size: 0x400000, crc: 0x38074F44 },
            Op::Load { kind: Load::Word64, file: "mmx.17m", offset: 0x4, size: 0x400000, crc: 0xE4635E35 },
            Op::Load { kind: Load::Word64, file: "mmx.19m", offset: 0x6, size: 0x400000, crc: 0x4400A3F2 },
            Op::Load { kind: Load::Word64, file: "mmx.14m", offset: 0x1000000, size: 0x400000, crc: 0xD52BF491 },
            Op::Load { kind: Load::Word64, file: "mmx.16m", offset: 0x1000002, size: 0x400000, crc: 0x23F70780 },
            Op::Load { kind: Load::Word64, file: "mmx.18m", offset: 0x1000004, size: 0x400000, crc: 0x2562C9D5 },
            Op::Load { kind: Load::Word64, file: "mmx.20m", offset: 0x1000006, size: 0x400000, crc: 0x583A9687 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mmx.01", offset: 0x0, size: 0x8000, crc: 0xC57E8171 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mmx.11m", offset: 0x0, size: 0x400000, crc: 0x4180B39F },
            Op::Load { kind: Load::Word16Swap, file: "mmx.12m", offset: 0x400000, size: 0x400000, crc: 0x95E22A59 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VHUNT2D: RomSetDef = RomSetDef { name: "vhunt2d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vh2j_d.03a", offset: 0x0, size: 0x80000, crc: 0x696E0157 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j_d.04a", offset: 0x80000, size: 0x80000, crc: 0xCED9BBA3 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.05", offset: 0x100000, size: 0x80000, crc: 0xDE34F624 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.06", offset: 0x180000, size: 0x80000, crc: 0x6A3B9897 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.07", offset: 0x200000, size: 0x80000, crc: 0xB021C029 },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.08", offset: 0x280000, size: 0x80000, crc: 0xAC873DFF },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.09", offset: 0x300000, size: 0x80000, crc: 0xEAEFCE9C },
            Op::Load { kind: Load::Word16Swap, file: "vh2j.10", offset: 0x380000, size: 0x80000, crc: 0x11730952 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vh2.13m", offset: 0x0, size: 0x400000, crc: 0x3B02DDAA },
            Op::Load { kind: Load::Word64, file: "vh2.15m", offset: 0x2, size: 0x400000, crc: 0x4E40DE66 },
            Op::Load { kind: Load::Word64, file: "vh2.17m", offset: 0x4, size: 0x400000, crc: 0xB31D00C9 },
            Op::Load { kind: Load::Word64, file: "vh2.19m", offset: 0x6, size: 0x400000, crc: 0x149BE3AB },
            Op::Load { kind: Load::Word64, file: "vh2.14m", offset: 0x1000000, size: 0x400000, crc: 0xCD09BD63 },
            Op::Load { kind: Load::Word64, file: "vh2.16m", offset: 0x1000002, size: 0x400000, crc: 0xE0182C15 },
            Op::Load { kind: Load::Word64, file: "vh2.18m", offset: 0x1000004, size: 0x400000, crc: 0x778DC4F6 },
            Op::Load { kind: Load::Word64, file: "vh2.20m", offset: 0x1000006, size: 0x400000, crc: 0x605D9D1D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vh2.01", offset: 0x0, size: 0x8000, crc: 0x67B9F779 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vh2.02", offset: 0x28000, size: 0x20000, crc: 0xAAF15FCB },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vh2.11m", offset: 0x0, size: 0x400000, crc: 0x38922EFD },
            Op::Load { kind: Load::Word16Swap, file: "vh2.12m", offset: 0x400000, size: 0x400000, crc: 0x6E2430AF },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_VSAV2D: RomSetDef = RomSetDef { name: "vsav2d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vs2j_d.03", offset: 0x0, size: 0x80000, crc: 0x50865F7B },
            Op::Load { kind: Load::Word16Swap, file: "vs2j_d.04", offset: 0x80000, size: 0x80000, crc: 0xC3BFF0E3 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.05", offset: 0x100000, size: 0x80000, crc: 0x61979638 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.06", offset: 0x180000, size: 0x80000, crc: 0xF37C5BC2 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.07", offset: 0x200000, size: 0x80000, crc: 0x8F885809 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.08", offset: 0x280000, size: 0x80000, crc: 0x2018C120 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.09", offset: 0x300000, size: 0x80000, crc: 0xFAC3C217 },
            Op::Load { kind: Load::Word16Swap, file: "vs2j.10", offset: 0x380000, size: 0x80000, crc: 0xEB490213 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "vs2.13m", offset: 0x0, size: 0x400000, crc: 0x5C852F52 },
            Op::Load { kind: Load::Word64, file: "vs2.15m", offset: 0x2, size: 0x400000, crc: 0xA20F58AF },
            Op::Load { kind: Load::Word64, file: "vs2.17m", offset: 0x4, size: 0x400000, crc: 0x39DB59AD },
            Op::Load { kind: Load::Word64, file: "vs2.19m", offset: 0x6, size: 0x400000, crc: 0x00C763A7 },
            Op::Load { kind: Load::Word64, file: "vs2.14m", offset: 0x1000000, size: 0x400000, crc: 0xCD09BD63 },
            Op::Load { kind: Load::Word64, file: "vs2.16m", offset: 0x1000002, size: 0x400000, crc: 0xE0182C15 },
            Op::Load { kind: Load::Word64, file: "vs2.18m", offset: 0x1000004, size: 0x400000, crc: 0x778DC4F6 },
            Op::Load { kind: Load::Word64, file: "vs2.20m", offset: 0x1000006, size: 0x400000, crc: 0x605D9D1D },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "vs2.01", offset: 0x0, size: 0x8000, crc: 0x35190139 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "vs2.02", offset: 0x28000, size: 0x20000, crc: 0xC32DBA09 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vs2.11m", offset: 0x0, size: 0x400000, crc: 0xD67E47B7 },
            Op::Load { kind: Load::Word16Swap, file: "vs2.12m", offset: 0x400000, size: 0x400000, crc: 0x6D020A14 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CSCLUB1D: RomSetDef = RomSetDef { name: "csclub1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csce_d.03", offset: 0x0, size: 0x80000, crc: 0x5AEDC6E6 },
            Op::Load { kind: Load::Word16Swap, file: "csce_d.04", offset: 0x80000, size: 0x80000, crc: 0xA3D9AA25 },
            Op::Load { kind: Load::Word16Swap, file: "csce_d.05", offset: 0x100000, size: 0x80000, crc: 0x0915C9D1 },
            Op::Load { kind: Load::Word16Swap, file: "csce_d.06", offset: 0x180000, size: 0x80000, crc: 0x09C77D99 },
            Op::Load { kind: Load::Word16Swap, file: "csc_d.07", offset: 0x200000, size: 0x80000, crc: 0x77478E25 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "csc.73", offset: 0x800000, size: 0x80000, crc: 0x335F07C3 },
            Op::Load { kind: Load::Word64, file: "csc.74", offset: 0xa00000, size: 0x80000, crc: 0xAB215357 },
            Op::Load { kind: Load::Word64, file: "csc.75", offset: 0xc00000, size: 0x80000, crc: 0xA2367381 },
            Op::Load { kind: Load::Word64, file: "csc.76", offset: 0xe00000, size: 0x80000, crc: 0x728AAC1F },
            Op::Load { kind: Load::Word64, file: "csc.63", offset: 0x800002, size: 0x80000, crc: 0x3711B8CA },
            Op::Load { kind: Load::Word64, file: "csc.64", offset: 0xa00002, size: 0x80000, crc: 0x828A06D8 },
            Op::Load { kind: Load::Word64, file: "csc.65", offset: 0xc00002, size: 0x80000, crc: 0x86EE4569 },
            Op::Load { kind: Load::Word64, file: "csc.66", offset: 0xe00002, size: 0x80000, crc: 0xC24F577F },
            Op::Load { kind: Load::Word64, file: "csc.83", offset: 0x800004, size: 0x80000, crc: 0x0750D12A },
            Op::Load { kind: Load::Word64, file: "csc.84", offset: 0xa00004, size: 0x80000, crc: 0x90A92F39 },
            Op::Load { kind: Load::Word64, file: "csc.85", offset: 0xc00004, size: 0x80000, crc: 0xD08AB012 },
            Op::Load { kind: Load::Word64, file: "csc.86", offset: 0xe00004, size: 0x80000, crc: 0x41652583 },
            Op::Load { kind: Load::Word64, file: "csc.93", offset: 0x800006, size: 0x80000, crc: 0xA756C7F7 },
            Op::Load { kind: Load::Word64, file: "csc.94", offset: 0xa00006, size: 0x80000, crc: 0xFB7CCC73 },
            Op::Load { kind: Load::Word64, file: "csc.95", offset: 0xc00006, size: 0x80000, crc: 0x4D014297 },
            Op::Load { kind: Load::Word64, file: "csc.96", offset: 0xe00006, size: 0x80000, crc: 0x6754B1EF },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csc.01", offset: 0x0, size: 0x8000, crc: 0xEE162111 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "csc.51", offset: 0x0, size: 0x80000, crc: 0x5A52AFD5 },
            Op::Load { kind: Load::Word16Swap, file: "csc.52", offset: 0x80000, size: 0x80000, crc: 0x1408A811 },
            Op::Load { kind: Load::Word16Swap, file: "csc.53", offset: 0x100000, size: 0x80000, crc: 0x4FB9F57C },
            Op::Load { kind: Load::Word16Swap, file: "csc.54", offset: 0x180000, size: 0x80000, crc: 0x9A8F40EC },
            Op::Load { kind: Load::Word16Swap, file: "csc.55", offset: 0x200000, size: 0x80000, crc: 0x91529A91 },
            Op::Load { kind: Load::Word16Swap, file: "csc.56", offset: 0x280000, size: 0x80000, crc: 0x9A345334 },
            Op::Load { kind: Load::Word16Swap, file: "csc.57", offset: 0x300000, size: 0x80000, crc: 0xAEDC27F2 },
            Op::Load { kind: Load::Word16Swap, file: "csc.58", offset: 0x380000, size: 0x80000, crc: 0x2300B7B3 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CYBOTSUD: RomSetDef = RomSetDef { name: "cybotsud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cybu_d.03", offset: 0x0, size: 0x80000, crc: 0xEE7560FB },
            Op::Load { kind: Load::Word16Swap, file: "cybu_d.04", offset: 0x80000, size: 0x80000, crc: 0x7E7425A0 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.05", offset: 0x100000, size: 0x80000, crc: 0xEC40408E },
            Op::Load { kind: Load::Word16Swap, file: "cyb.06", offset: 0x180000, size: 0x80000, crc: 0x1AD0BED2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.07", offset: 0x200000, size: 0x80000, crc: 0x6245A39A },
            Op::Load { kind: Load::Word16Swap, file: "cyb.08", offset: 0x280000, size: 0x80000, crc: 0x4B48E223 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.09", offset: 0x300000, size: 0x80000, crc: 0xE15238F6 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.10", offset: 0x380000, size: 0x80000, crc: 0x75F4003B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cyb.13m", offset: 0x0, size: 0x400000, crc: 0xF0DCE192 },
            Op::Load { kind: Load::Word64, file: "cyb.15m", offset: 0x2, size: 0x400000, crc: 0x187AA39C },
            Op::Load { kind: Load::Word64, file: "cyb.17m", offset: 0x4, size: 0x400000, crc: 0x8A0E4B12 },
            Op::Load { kind: Load::Word64, file: "cyb.19m", offset: 0x6, size: 0x400000, crc: 0x34B62612 },
            Op::Load { kind: Load::Word64, file: "cyb.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1537957 },
            Op::Load { kind: Load::Word64, file: "cyb.16m", offset: 0x1000002, size: 0x400000, crc: 0x15349E86 },
            Op::Load { kind: Load::Word64, file: "cyb.18m", offset: 0x1000004, size: 0x400000, crc: 0xD83E977D },
            Op::Load { kind: Load::Word64, file: "cyb.20m", offset: 0x1000006, size: 0x400000, crc: 0x77CDAD5C },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cyb.01", offset: 0x0, size: 0x8000, crc: 0x9C0FB079 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "cyb.02", offset: 0x28000, size: 0x20000, crc: 0x51CB0C4E },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cyb.11m", offset: 0x0, size: 0x200000, crc: 0x362CCAB2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.12m", offset: 0x200000, size: 0x200000, crc: 0x7066E9CC },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_CYBOTSJD: RomSetDef = RomSetDef { name: "cybotsjd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cybj_d.03", offset: 0x0, size: 0x80000, crc: 0x9EB34071 },
            Op::Load { kind: Load::Word16Swap, file: "cybj_d.04", offset: 0x80000, size: 0x80000, crc: 0xCF223CD7 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.05", offset: 0x100000, size: 0x80000, crc: 0xEC40408E },
            Op::Load { kind: Load::Word16Swap, file: "cyb.06", offset: 0x180000, size: 0x80000, crc: 0x1AD0BED2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.07", offset: 0x200000, size: 0x80000, crc: 0x6245A39A },
            Op::Load { kind: Load::Word16Swap, file: "cyb.08", offset: 0x280000, size: 0x80000, crc: 0x4B48E223 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.09", offset: 0x300000, size: 0x80000, crc: 0xE15238F6 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.10", offset: 0x380000, size: 0x80000, crc: 0x75F4003B },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cyb.13m", offset: 0x0, size: 0x400000, crc: 0xF0DCE192 },
            Op::Load { kind: Load::Word64, file: "cyb.15m", offset: 0x2, size: 0x400000, crc: 0x187AA39C },
            Op::Load { kind: Load::Word64, file: "cyb.17m", offset: 0x4, size: 0x400000, crc: 0x8A0E4B12 },
            Op::Load { kind: Load::Word64, file: "cyb.19m", offset: 0x6, size: 0x400000, crc: 0x34B62612 },
            Op::Load { kind: Load::Word64, file: "cyb.14m", offset: 0x1000000, size: 0x400000, crc: 0xC1537957 },
            Op::Load { kind: Load::Word64, file: "cyb.16m", offset: 0x1000002, size: 0x400000, crc: 0x15349E86 },
            Op::Load { kind: Load::Word64, file: "cyb.18m", offset: 0x1000004, size: 0x400000, crc: 0xD83E977D },
            Op::Load { kind: Load::Word64, file: "cyb.20m", offset: 0x1000006, size: 0x400000, crc: 0x77CDAD5C },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cyb.01", offset: 0x0, size: 0x8000, crc: 0x9C0FB079 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "cyb.02", offset: 0x28000, size: 0x20000, crc: 0x51CB0C4E },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cyb.11m", offset: 0x0, size: 0x200000, crc: 0x362CCAB2 },
            Op::Load { kind: Load::Word16Swap, file: "cyb.12m", offset: 0x200000, size: 0x200000, crc: 0x7066E9CC },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_MSHVSFU1D: RomSetDef = RomSetDef { name: "mshvsfu1d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvsu_d.03d", offset: 0x0, size: 0x80000, crc: 0x1C88BEE3 },
            Op::Load { kind: Load::Word16Swap, file: "mvsu_d.04d", offset: 0x80000, size: 0x80000, crc: 0x1E8B2535 },
            Op::Load { kind: Load::Word16Swap, file: "mvs_d.05a", offset: 0x100000, size: 0x80000, crc: 0x373856FB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.06a", offset: 0x180000, size: 0x80000, crc: 0x959F3030 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.07b", offset: 0x200000, size: 0x80000, crc: 0x7F915BDB },
            Op::Load { kind: Load::Word16Swap, file: "mvs.08a", offset: 0x280000, size: 0x80000, crc: 0xC2813884 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.09b", offset: 0x300000, size: 0x80000, crc: 0x3BA08818 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.10b", offset: 0x380000, size: 0x80000, crc: 0xCF0DBA98 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mvs.13m", offset: 0x0, size: 0x400000, crc: 0x29B05FD9 },
            Op::Load { kind: Load::Word64, file: "mvs.15m", offset: 0x2, size: 0x400000, crc: 0xFADDCCF1 },
            Op::Load { kind: Load::Word64, file: "mvs.17m", offset: 0x4, size: 0x400000, crc: 0x97AAF4C7 },
            Op::Load { kind: Load::Word64, file: "mvs.19m", offset: 0x6, size: 0x400000, crc: 0xCB70E915 },
            Op::Load { kind: Load::Word64, file: "mvs.14m", offset: 0x1000000, size: 0x400000, crc: 0xB3B1972D },
            Op::Load { kind: Load::Word64, file: "mvs.16m", offset: 0x1000002, size: 0x400000, crc: 0x08AADB5D },
            Op::Load { kind: Load::Word64, file: "mvs.18m", offset: 0x1000004, size: 0x400000, crc: 0xC1228B35 },
            Op::Load { kind: Load::Word64, file: "mvs.20m", offset: 0x1000006, size: 0x400000, crc: 0x366CC6C2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mvs.01", offset: 0x0, size: 0x8000, crc: 0x68252324 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "mvs.02", offset: 0x28000, size: 0x20000, crc: 0xB34E773D },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mvs.11m", offset: 0x0, size: 0x400000, crc: 0x86219770 },
            Op::Load { kind: Load::Word16Swap, file: "mvs.12m", offset: 0x400000, size: 0x400000, crc: 0xF2FD7F68 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : user1
pub const RS2_SPF2TD: RomSetDef = RomSetDef { name: "spf2td", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzfu_d.03a", offset: 0x0, size: 0x80000, crc: 0x7836B903 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.04", offset: 0x80000, size: 0x80000, crc: 0xB80649E2 },
        ] },
        RegionDef { name: "gfx", size: 0xc00000, fill: 0x0, ops: &[
            Op::Fill { offset: 0x0, size: 0x800000, value: 0x0 },
            Op::Load { kind: Load::Word64, file: "pzf.14m", offset: 0x800000, size: 0x100000, crc: 0x2D4881CB },
            Op::Load { kind: Load::Word64, file: "pzf.16m", offset: 0x800002, size: 0x100000, crc: 0x4B0FD1BE },
            Op::Load { kind: Load::Word64, file: "pzf.18m", offset: 0x800004, size: 0x100000, crc: 0xE43AAC33 },
            Op::Load { kind: Load::Word64, file: "pzf.20m", offset: 0x800006, size: 0x100000, crc: 0x7F536FF1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pzf.01", offset: 0x0, size: 0x8000, crc: 0x600FB2A3 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "pzf.02", offset: 0x28000, size: 0x20000, crc: 0x496076E0 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pzf.11m", offset: 0x0, size: 0x200000, crc: 0x78442743 },
            Op::Load { kind: Load::Word16Swap, file: "pzf.12m", offset: 0x200000, size: 0x200000, crc: 0x399D2C7B },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ3JR2D: RomSetDef = RomSetDef { name: "sfz3jr2d", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3j_d.03", offset: 0x0, size: 0x80000, crc: 0xB0436151 },
            Op::Load { kind: Load::Word16Swap, file: "sz3j_d.04", offset: 0x80000, size: 0x80000, crc: 0x642D8170 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.05", offset: 0x100000, size: 0x80000, crc: 0x9B21518A },
            Op::Load { kind: Load::Word16Swap, file: "sz3.06", offset: 0x180000, size: 0x80000, crc: 0xE7A6C3A7 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.07", offset: 0x200000, size: 0x80000, crc: 0xEC4C0CFD },
            Op::Load { kind: Load::Word16Swap, file: "sz3.08", offset: 0x280000, size: 0x80000, crc: 0x5C7E7240 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.09", offset: 0x300000, size: 0x80000, crc: 0xC5589553 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.10", offset: 0x380000, size: 0x80000, crc: 0xA9717252 },
        ] },
        RegionDef { name: "gfx", size: 0x2000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz3.13m", offset: 0x0, size: 0x400000, crc: 0x0F7A60D9 },
            Op::Load { kind: Load::Word64, file: "sz3.15m", offset: 0x2, size: 0x400000, crc: 0x8E933741 },
            Op::Load { kind: Load::Word64, file: "sz3.17m", offset: 0x4, size: 0x400000, crc: 0xD6E98147 },
            Op::Load { kind: Load::Word64, file: "sz3.19m", offset: 0x6, size: 0x400000, crc: 0xF31A728A },
            Op::Load { kind: Load::Word64, file: "sz3.14m", offset: 0x1000000, size: 0x400000, crc: 0x5FF98297 },
            Op::Load { kind: Load::Word64, file: "sz3.16m", offset: 0x1000002, size: 0x400000, crc: 0x52B5BDEE },
            Op::Load { kind: Load::Word64, file: "sz3.18m", offset: 0x1000004, size: 0x400000, crc: 0x40631ED5 },
            Op::Load { kind: Load::Word64, file: "sz3.20m", offset: 0x1000006, size: 0x400000, crc: 0x763409B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz3.01", offset: 0x0, size: 0x8000, crc: 0xDE810084 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz3.02", offset: 0x28000, size: 0x20000, crc: 0x72445DC4 },
        ] },
        RegionDef { name: "qsound", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz3.11m", offset: 0x0, size: 0x400000, crc: 0x1C89EED1 },
            Op::Load { kind: Load::Word16Swap, file: "sz3.12m", offset: 0x400000, size: 0x400000, crc: 0xF392B13A },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_SFZ2JD: RomSetDef = RomSetDef { name: "sfz2jd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2j_d.03a", offset: 0x0, size: 0x80000, crc: 0xB7325DF8 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j_d.04a", offset: 0x80000, size: 0x80000, crc: 0xA1022A3E },
            Op::Load { kind: Load::Word16Swap, file: "sz2.05a", offset: 0x100000, size: 0x80000, crc: 0x98E8E992 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.06", offset: 0x180000, size: 0x80000, crc: 0x5B1D49C0 },
            Op::Load { kind: Load::Word16Swap, file: "sz2j.07a", offset: 0x200000, size: 0x80000, crc: 0xD910B2A2 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.08", offset: 0x280000, size: 0x80000, crc: 0x0FE8585D },
        ] },
        RegionDef { name: "gfx", size: 0x1400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sz2.13m", offset: 0x0, size: 0x400000, crc: 0x4D1F1F22 },
            Op::Load { kind: Load::Word64, file: "sz2.15m", offset: 0x2, size: 0x400000, crc: 0x19CEA680 },
            Op::Load { kind: Load::Word64, file: "sz2.17m", offset: 0x4, size: 0x400000, crc: 0xE01B4588 },
            Op::Load { kind: Load::Word64, file: "sz2.19m", offset: 0x6, size: 0x400000, crc: 0x0FEEDA64 },
            Op::Load { kind: Load::Word64, file: "sz2.14m", offset: 0x1000000, size: 0x100000, crc: 0x0560C6AA },
            Op::Load { kind: Load::Word64, file: "sz2.16m", offset: 0x1000002, size: 0x100000, crc: 0xAE940F87 },
            Op::Load { kind: Load::Word64, file: "sz2.18m", offset: 0x1000004, size: 0x100000, crc: 0x4BC3C8BC },
            Op::Load { kind: Load::Word64, file: "sz2.20m", offset: 0x1000006, size: 0x100000, crc: 0x39E674C0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sz2.01a", offset: 0x0, size: 0x8000, crc: 0x1BC323CF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
            Op::Load { kind: Load::Byte, file: "sz2.02a", offset: 0x28000, size: 0x20000, crc: 0xBA6A5013 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sz2.11m", offset: 0x0, size: 0x200000, crc: 0xAA47A601 },
            Op::Load { kind: Load::Word16Swap, file: "sz2.12m", offset: 0x200000, size: 0x200000, crc: 0x2237BC53 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS2_PZLOOP2JD: RomSetDef = RomSetDef { name: "pzloop2jd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2j_d.03a", offset: 0x0, size: 0x80000, crc: 0x1F5D41C9 },
            Op::Load { kind: Load::Word16Swap, file: "pl2j_d.04a", offset: 0x80000, size: 0x80000, crc: 0x60721B73 },
            Op::Load { kind: Load::Word16Swap, file: "pl2j_d.05a", offset: 0x100000, size: 0x80000, crc: 0x0C39799F },
            Op::Load { kind: Load::Word16Swap, file: "pl2j_d.06a", offset: 0x180000, size: 0x80000, crc: 0xF3974D76 },
        ] },
        RegionDef { name: "gfx", size: 0x1000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01c", offset: 0x0, size: 0x200000, crc: 0x137B13A7 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01d", offset: 0x1, size: 0x200000, crc: 0xA2DB1507 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01a", offset: 0x2, size: 0x200000, crc: 0x7E80FF8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.01b", offset: 0x3, size: 0x200000, crc: 0xCD93E6ED },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03c", offset: 0x4, size: 0x200000, crc: 0x0F52BBCA },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03d", offset: 0x5, size: 0x200000, crc: 0xA62712C3 },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03a", offset: 0x6, size: 0x200000, crc: 0xB60C9F8E },
            Op::Load { kind: Load::Byte64, file: "pl2-simm.03b", offset: 0x7, size: 0x200000, crc: 0x83FEF284 },
        ] },
        RegionDef { name: "audiocpu", size: 0x50000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pl2.01", offset: 0x0, size: 0x8000, crc: 0x35697569 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05a", offset: 0x0, size: 0x200000, crc: 0x85D8FBE8 },
            Op::Load { kind: Load::Word16Swap, file: "pl2-simm.05b", offset: 0x200000, size: 0x200000, crc: 0x1ED62584 },
        ] },
        RegionDef { name: "key", size: 0x20, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "phoenix.key", offset: 0x0, size: 0x14, crc: 0x2CF772B0 },
        ] },
] };

/// Lignes GAME de cps2.cpp, dans l'ordre : (nom, parent, config machine, ports d'entrée, init, rotation, constructeur, titre, drapeaux).
pub static GAMES2: &[GameInfo] = &[
    GameInfo { name: "ssf2", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (World 931005)", not_working: false, romset: &RS2_SSF2 },
    GameInfo { name: "ssf2r1", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (World 930911)", not_working: false, romset: &RS2_SSF2R1 },
    GameInfo { name: "ssf2u", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (USA 930911)", not_working: false, romset: &RS2_SSF2U },
    GameInfo { name: "ssf2us2", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Street Fighter II: The New Challengers Super 2 (USA 930911)", not_working: false, romset: &RS2_SSF2US2 },
    GameInfo { name: "ssf2a", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (Asia 931005)", not_working: false, romset: &RS2_SSF2A },
    GameInfo { name: "ssf2ar1", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (Asia 930914)", not_working: false, romset: &RS2_SSF2AR1 },
    GameInfo { name: "ssf2j", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (Japan 931005)", not_working: false, romset: &RS2_SSF2J },
    GameInfo { name: "ssf2jr1", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (Japan 930911)", not_working: false, romset: &RS2_SSF2JR1 },
    GameInfo { name: "ssf2jr2", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (Japan 930910)", not_working: false, romset: &RS2_SSF2JR2 },
    GameInfo { name: "ssf2h", parent: "ssf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The New Challengers (Hispanic 930911)", not_working: false, romset: &RS2_SSF2H },
    GameInfo { name: "ssf2tb", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (World 931119)", not_working: false, romset: &RS2_SSF2TB },
    GameInfo { name: "ssf2tbr1", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (World 930911)", not_working: false, romset: &RS2_SSF2TBR1 },
    GameInfo { name: "ssf2tbu", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (USA 930911)", not_working: false, romset: &RS2_SSF2TBU },
    GameInfo { name: "ssf2tbj", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (Japan 931005)", not_working: false, romset: &RS2_SSF2TBJ },
    GameInfo { name: "ssf2tbj1", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (Japan 930911)", not_working: false, romset: &RS2_SSF2TBJ1 },
    GameInfo { name: "ssf2tba", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (Asia 931005)", not_working: false, romset: &RS2_SSF2TBA },
    GameInfo { name: "ssf2tbh", parent: "ssf2", machine: "cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II: The Tournament Battle (Hispanic 931005)", not_working: false, romset: &RS2_SSF2TBH },
    GameInfo { name: "ecofghtr", parent: "", machine: "cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "Capcom", title: "Eco Fighters (World 931203)", not_working: false, romset: &RS2_ECOFGHTR },
    GameInfo { name: "ecofghtru", parent: "ecofghtr", machine: "cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "Capcom", title: "Eco Fighters (USA 940215)", not_working: false, romset: &RS2_ECOFGHTRU },
    GameInfo { name: "ecofghtru1", parent: "ecofghtr", machine: "cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "Capcom", title: "Eco Fighters (USA 931203)", not_working: false, romset: &RS2_ECOFGHTRU1 },
    GameInfo { name: "uecology", parent: "ecofghtr", machine: "cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "Capcom", title: "Ultimate Ecology (Japan 931203)", not_working: false, romset: &RS2_UECOLOGY },
    GameInfo { name: "ecofghtra", parent: "ecofghtr", machine: "cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "Capcom", title: "Eco Fighters (Asia 931203)", not_working: false, romset: &RS2_ECOFGHTRA },
    GameInfo { name: "ecofghtrh", parent: "ecofghtr", machine: "cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "Capcom", title: "Eco Fighters (Hispanic 931203)", not_working: false, romset: &RS2_ECOFGHTRH },
    GameInfo { name: "ddtod", parent: "", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Europe 940412)", not_working: false, romset: &RS2_DDTOD },
    GameInfo { name: "ddtodr1", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Europe 940113)", not_working: false, romset: &RS2_DDTODR1 },
    GameInfo { name: "ddtodu", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (USA 940125)", not_working: false, romset: &RS2_DDTODU },
    GameInfo { name: "ddtodur1", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (USA 940113)", not_working: false, romset: &RS2_DDTODUR1 },
    GameInfo { name: "ddtodj", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Japan 940412)", not_working: false, romset: &RS2_DDTODJ },
    GameInfo { name: "ddtodjr1", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Japan 940125)", not_working: false, romset: &RS2_DDTODJR1 },
    GameInfo { name: "ddtodjr2", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Japan 940113)", not_working: false, romset: &RS2_DDTODJR2 },
    GameInfo { name: "ddtoda", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Asia 940412)", not_working: false, romset: &RS2_DDTODA },
    GameInfo { name: "ddtodar1", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Asia 940113)", not_working: false, romset: &RS2_DDTODAR1 },
    GameInfo { name: "ddtodh", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Hispanic 940412)", not_working: false, romset: &RS2_DDTODH },
    GameInfo { name: "ddtodhr1", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Hispanic 940125)", not_working: false, romset: &RS2_DDTODHR1 },
    GameInfo { name: "ddtodhr2", parent: "ddtod", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Tower of Doom (Hispanic 940113)", not_working: false, romset: &RS2_DDTODHR2 },
    GameInfo { name: "ssf2t", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II Turbo (World 940223)", not_working: false, romset: &RS2_SSF2T },
    GameInfo { name: "ssf2ta", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II Turbo (Asia 940223)", not_working: false, romset: &RS2_SSF2TA },
    GameInfo { name: "ssf2th", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II Turbo (Hispanic 940223)", not_working: false, romset: &RS2_SSF2TH },
    GameInfo { name: "ssf2tu", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II Turbo (USA 940323)", not_working: false, romset: &RS2_SSF2TU },
    GameInfo { name: "ssf2tur1", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II Turbo (USA 940223)", not_working: false, romset: &RS2_SSF2TUR1 },
    GameInfo { name: "ssf2xj", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II X: Grand Master Challenge (Japan 940311)", not_working: false, romset: &RS2_SSF2XJ },
    GameInfo { name: "ssf2xjr1", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II X: Grand Master Challenge (Japan 940223)", not_working: false, romset: &RS2_SSF2XJR1 },
    GameInfo { name: "ssf2xjr1r", parent: "ssf2t", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Street Fighter II X: Grand Master Challenge (Japan 940223 rent version)", not_working: false, romset: &RS2_SSF2XJR1R },
    GameInfo { name: "avsp", parent: "", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Alien vs. Predator (Europe 940520)", not_working: false, romset: &RS2_AVSP },
    GameInfo { name: "avspu", parent: "avsp", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Alien vs. Predator (USA 940520)", not_working: false, romset: &RS2_AVSPU },
    GameInfo { name: "avspj", parent: "avsp", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Alien vs. Predator (Japan 940520)", not_working: false, romset: &RS2_AVSPJ },
    GameInfo { name: "avspa", parent: "avsp", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Alien vs. Predator (Asia 940520)", not_working: false, romset: &RS2_AVSPA },
    GameInfo { name: "avsph", parent: "avsp", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Alien vs. Predator (Hispanic 940520)", not_working: false, romset: &RS2_AVSPH },
    GameInfo { name: "dstlk", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Darkstalkers: The Night Warriors (Europe 940705)", not_working: false, romset: &RS2_DSTLK },
    GameInfo { name: "dstlku", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Darkstalkers: The Night Warriors (USA 940818)", not_working: false, romset: &RS2_DSTLKU },
    GameInfo { name: "dstlkur1", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Darkstalkers: The Night Warriors (USA 940705)", not_working: false, romset: &RS2_DSTLKUR1 },
    GameInfo { name: "dstlka", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Darkstalkers: The Night Warriors (Asia 940705)", not_working: false, romset: &RS2_DSTLKA },
    GameInfo { name: "dstlkh", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Darkstalkers: The Night Warriors (Hispanic 940818)", not_working: false, romset: &RS2_DSTLKH },
    GameInfo { name: "dstlkb", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Darkstalkers: The Night Warriors (Brazil 940818)", not_working: false, romset: &RS2_DSTLKB },
    GameInfo { name: "vampj", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire: The Night Warriors (Japan 940705)", not_working: false, romset: &RS2_VAMPJ },
    GameInfo { name: "vampja", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire: The Night Warriors (Japan 940705 alt)", not_working: false, romset: &RS2_VAMPJA },
    GameInfo { name: "vampjr1", parent: "dstlk", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire: The Night Warriors (Japan 940630)", not_working: false, romset: &RS2_VAMPJR1 },
    GameInfo { name: "ringdest", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Ring of Destruction: Slammasters II (Europe 940902)", not_working: false, romset: &RS2_RINGDEST },
    GameInfo { name: "ringdesta", parent: "ringdest", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Ring of Destruction: Slammasters II (Asia 940831)", not_working: false, romset: &RS2_RINGDESTA },
    GameInfo { name: "ringdesth", parent: "ringdest", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Ring of Destruction: Slammasters II (Hispanic 940902)", not_working: false, romset: &RS2_RINGDESTH },
    GameInfo { name: "ringdestb", parent: "ringdest", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Ring of Destruction: Slammasters II (Brazil 940902)", not_working: false, romset: &RS2_RINGDESTB },
    GameInfo { name: "smbomb", parent: "ringdest", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Muscle Bomber: The International Blowout (Japan 940831)", not_working: false, romset: &RS2_SMBOMB },
    GameInfo { name: "smbombr1", parent: "ringdest", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Muscle Bomber: The International Blowout (Japan 940808)", not_working: false, romset: &RS2_SMBOMBR1 },
    GameInfo { name: "armwar", parent: "", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (Europe 941024)", not_working: false, romset: &RS2_ARMWAR },
    GameInfo { name: "armwarr1", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (Europe 941011)", not_working: false, romset: &RS2_ARMWARR1 },
    GameInfo { name: "armwaru", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (USA 941024)", not_working: false, romset: &RS2_ARMWARU },
    GameInfo { name: "armwaru1", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (USA 940920)", not_working: false, romset: &RS2_ARMWARU1 },
    GameInfo { name: "armwarb", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (Brazil 941024)", not_working: false, romset: &RS2_ARMWARB },
    GameInfo { name: "pgear", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Powered Gear: Strategic Variant Armor Equipment (Japan 941024)", not_working: false, romset: &RS2_PGEAR },
    GameInfo { name: "pgearr1", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Powered Gear: Strategic Variant Armor Equipment (Japan 940916)", not_working: false, romset: &RS2_PGEARR1 },
    GameInfo { name: "armwara", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (Asia 941024)", not_working: false, romset: &RS2_ARMWARA },
    GameInfo { name: "armwarar1", parent: "armwar", machine: "cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Armored Warriors (Asia 940920)", not_working: false, romset: &RS2_ARMWARAR1 },
    GameInfo { name: "xmcota", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Europe 950331)", not_working: false, romset: &RS2_XMCOTA },
    GameInfo { name: "xmcotar1", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Europe 950105)", not_working: false, romset: &RS2_XMCOTAR1 },
    GameInfo { name: "xmcotau", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (USA 950105)", not_working: false, romset: &RS2_XMCOTAU },
    GameInfo { name: "xmcotab", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Brazil 950331)", not_working: false, romset: &RS2_XMCOTAB },
    GameInfo { name: "xmcotah", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Hispanic 950331)", not_working: false, romset: &RS2_XMCOTAH },
    GameInfo { name: "xmcotahr1", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Hispanic 950105)", not_working: false, romset: &RS2_XMCOTAHR1 },
    GameInfo { name: "xmcotaj", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Japan 950105)", not_working: false, romset: &RS2_XMCOTAJ },
    GameInfo { name: "xmcotaj1", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Japan 941222)", not_working: false, romset: &RS2_XMCOTAJ1 },
    GameInfo { name: "xmcotaj2", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Japan 941219)", not_working: false, romset: &RS2_XMCOTAJ2 },
    GameInfo { name: "xmcotaj3", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Japan 941217)", not_working: false, romset: &RS2_XMCOTAJ3 },
    GameInfo { name: "xmcotajr", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Japan 941208 rent version)", not_working: false, romset: &RS2_XMCOTAJR },
    GameInfo { name: "xmcotaa", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Asia 950105)", not_working: false, romset: &RS2_XMCOTAA },
    GameInfo { name: "xmcotaar1", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Asia 941219)", not_working: false, romset: &RS2_XMCOTAAR1 },
    GameInfo { name: "xmcotaar2", parent: "xmcota", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men: Children of the Atom (Asia 941217)", not_working: false, romset: &RS2_XMCOTAAR2 },
    GameInfo { name: "nwarr", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Night Warriors: Darkstalkers' Revenge (Europe 950316)", not_working: false, romset: &RS2_NWARR },
    GameInfo { name: "nwarru", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Night Warriors: Darkstalkers' Revenge (USA 950406)", not_working: false, romset: &RS2_NWARRU },
    GameInfo { name: "nwarrh", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Night Warriors: Darkstalkers' Revenge (Hispanic 950403)", not_working: false, romset: &RS2_NWARRH },
    GameInfo { name: "nwarrb", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Night Warriors: Darkstalkers' Revenge (Brazil 950403)", not_working: false, romset: &RS2_NWARRB },
    GameInfo { name: "nwarra", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Night Warriors: Darkstalkers' Revenge (Asia 950302)", not_working: false, romset: &RS2_NWARRA },
    GameInfo { name: "vhuntj", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Hunter: Darkstalkers' Revenge (Japan 950316)", not_working: false, romset: &RS2_VHUNTJ },
    GameInfo { name: "vhuntjr1s", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Hunter: Darkstalkers' Revenge (Japan 950307 stop version)", not_working: false, romset: &RS2_VHUNTJR1S },
    GameInfo { name: "vhuntjr1", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Hunter: Darkstalkers' Revenge (Japan 950307)", not_working: false, romset: &RS2_VHUNTJR1 },
    GameInfo { name: "vhuntjr2", parent: "nwarr", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Hunter: Darkstalkers' Revenge (Japan 950302)", not_working: false, romset: &RS2_VHUNTJR2 },
    GameInfo { name: "cybots", parent: "", machine: "cps2", input: "cybots", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Cyberbots: Fullmetal Madness (Europe 950424)", not_working: false, romset: &RS2_CYBOTS },
    GameInfo { name: "cybotsu", parent: "cybots", machine: "cps2", input: "cybots", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Cyberbots: Fullmetal Madness (USA 950424)", not_working: false, romset: &RS2_CYBOTSU },
    GameInfo { name: "cybotsj", parent: "cybots", machine: "cps2", input: "cybots", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Cyberbots: Fullmetal Madness (Japan 950420)", not_working: false, romset: &RS2_CYBOTSJ },
    GameInfo { name: "sfa", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha: Warriors' Dreams (Europe 950727)", not_working: false, romset: &RS2_SFA },
    GameInfo { name: "sfar1", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha: Warriors' Dreams (Europe 950718)", not_working: false, romset: &RS2_SFAR1 },
    GameInfo { name: "sfar2", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha: Warriors' Dreams (Europe 950627)", not_working: false, romset: &RS2_SFAR2 },
    GameInfo { name: "sfar3", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha: Warriors' Dreams (Europe 950605)", not_working: false, romset: &RS2_SFAR3 },
    GameInfo { name: "sfau", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha: Warriors' Dreams (USA 950627)", not_working: false, romset: &RS2_SFAU },
    GameInfo { name: "sfza", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Asia 950627)", not_working: false, romset: &RS2_SFZA },
    GameInfo { name: "sfzar1", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Asia 950605)", not_working: false, romset: &RS2_SFZAR1 },
    GameInfo { name: "sfzj", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Japan 950727)", not_working: false, romset: &RS2_SFZJ },
    GameInfo { name: "sfzjr1", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Japan 950627)", not_working: false, romset: &RS2_SFZJR1 },
    GameInfo { name: "sfzjr2", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Japan 950605)", not_working: false, romset: &RS2_SFZJR2 },
    GameInfo { name: "sfzh", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Hispanic 950718)", not_working: false, romset: &RS2_SFZH },
    GameInfo { name: "sfzhr1", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Hispanic 950627)", not_working: false, romset: &RS2_SFZHR1 },
    GameInfo { name: "sfzb", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Brazil 951109)", not_working: false, romset: &RS2_SFZB },
    GameInfo { name: "sfzbr1", parent: "sfa", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero (Brazil 950727)", not_working: false, romset: &RS2_SFZBR1 },
    GameInfo { name: "mmancp2u", parent: "megaman", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Mega Man: The Power Battle (CPS2, USA 951006, SAMPLE Version)", not_working: false, romset: &RS2_MMANCP2U },
    GameInfo { name: "mmancp2ur1", parent: "megaman", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Mega Man: The Power Battle (CPS2, USA 950926, SAMPLE Version)", not_working: false, romset: &RS2_MMANCP2UR1 },
    GameInfo { name: "mmancp2ur2", parent: "megaman", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Mega Man: The Power Battle (CPS2, USA 950925, SAMPLE Version)", not_working: false, romset: &RS2_MMANCP2UR2 },
    GameInfo { name: "rmancp2j", parent: "megaman", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Rockman: The Power Battle (CPS2, Japan 950922)", not_working: false, romset: &RS2_RMANCP2J },
    GameInfo { name: "msh", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Europe 951024)", not_working: false, romset: &RS2_MSH },
    GameInfo { name: "mshu", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (USA 951024)", not_working: false, romset: &RS2_MSHU },
    GameInfo { name: "mshj", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Japan 951117)", not_working: false, romset: &RS2_MSHJ },
    GameInfo { name: "mshjr1", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Japan 951024)", not_working: false, romset: &RS2_MSHJR1 },
    GameInfo { name: "msha", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Asia 951024)", not_working: false, romset: &RS2_MSHA },
    GameInfo { name: "mshh", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Hispanic 951117)", not_working: false, romset: &RS2_MSHH },
    GameInfo { name: "mshb", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Brazil 951117)", not_working: false, romset: &RS2_MSHB },
    GameInfo { name: "mshbr1", parent: "msh", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes (Brazil 951024)", not_working: false, romset: &RS2_MSHBR1 },
    GameInfo { name: "19xx", parent: "", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Europe 960104)", not_working: false, romset: &RS2_19XX },
    GameInfo { name: "19xxu", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (USA 951207)", not_working: false, romset: &RS2_19XXU },
    GameInfo { name: "19xxa", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Asia 960104)", not_working: false, romset: &RS2_19XXA },
    GameInfo { name: "19xxar1", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Asia 951207)", not_working: false, romset: &RS2_19XXAR1 },
    GameInfo { name: "19xxj", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Japan 960104, yellow case)", not_working: false, romset: &RS2_19XXJ },
    GameInfo { name: "19xxjr1", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Japan 951225)", not_working: false, romset: &RS2_19XXJR1 },
    GameInfo { name: "19xxjr2", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Japan 951207)", not_working: false, romset: &RS2_19XXJR2 },
    GameInfo { name: "19xxh", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Hispanic 951218)", not_working: false, romset: &RS2_19XXH },
    GameInfo { name: "19xxb", parent: "19xx", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "Capcom", title: "19XX: The War Against Destiny (Brazil 951218)", not_working: false, romset: &RS2_19XXB },
    GameInfo { name: "ddsom", parent: "", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Europe 960619)", not_working: false, romset: &RS2_DDSOM },
    GameInfo { name: "ddsomr1", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Europe 960223)", not_working: false, romset: &RS2_DDSOMR1 },
    GameInfo { name: "ddsomr2", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Europe 960209)", not_working: false, romset: &RS2_DDSOMR2 },
    GameInfo { name: "ddsomr3", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Europe 960208)", not_working: false, romset: &RS2_DDSOMR3 },
    GameInfo { name: "ddsomu", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (USA 960619)", not_working: false, romset: &RS2_DDSOMU },
    GameInfo { name: "ddsomur1", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (USA 960209)", not_working: false, romset: &RS2_DDSOMUR1 },
    GameInfo { name: "ddsomj", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Japan 960619)", not_working: false, romset: &RS2_DDSOMJ },
    GameInfo { name: "ddsomjr1", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Japan 960223)", not_working: false, romset: &RS2_DDSOMJR1 },
    GameInfo { name: "ddsomjr2", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Japan 960206)", not_working: false, romset: &RS2_DDSOMJR2 },
    GameInfo { name: "ddsoma", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Asia 960619)", not_working: false, romset: &RS2_DDSOMA },
    GameInfo { name: "ddsomar1", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Asia 960208)", not_working: false, romset: &RS2_DDSOMAR1 },
    GameInfo { name: "ddsomh", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Hispanic 960223)", not_working: false, romset: &RS2_DDSOMH },
    GameInfo { name: "ddsomb", parent: "ddsom", machine: "cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Dungeons & Dragons: Shadow over Mystara (Brazil 960223)", not_working: false, romset: &RS2_DDSOMB },
    GameInfo { name: "sfa2", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 2 (Europe 960229)", not_working: false, romset: &RS2_SFA2 },
    GameInfo { name: "sfa2u", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 2 (USA 960430)", not_working: false, romset: &RS2_SFA2U },
    GameInfo { name: "sfa2ur1", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 2 (USA 960306)", not_working: false, romset: &RS2_SFA2UR1 },
    GameInfo { name: "sfz2j", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Japan 960430)", not_working: false, romset: &RS2_SFZ2J },
    GameInfo { name: "sfz2jr1", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Japan 960227)", not_working: false, romset: &RS2_SFZ2JR1 },
    GameInfo { name: "sfz2a", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Asia 960227)", not_working: false, romset: &RS2_SFZ2A },
    GameInfo { name: "sfz2b", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Brazil 960531)", not_working: false, romset: &RS2_SFZ2B },
    GameInfo { name: "sfz2br1", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Brazil 960304)", not_working: false, romset: &RS2_SFZ2BR1 },
    GameInfo { name: "sfz2h", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Hispanic 960304)", not_working: false, romset: &RS2_SFZ2H },
    GameInfo { name: "sfz2n", parent: "sfa2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 (Oceania 960229)", not_working: false, romset: &RS2_SFZ2N },
    GameInfo { name: "sfz2al", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 Alpha (Asia 960826)", not_working: false, romset: &RS2_SFZ2AL },
    GameInfo { name: "sfz2alr1", parent: "sfz2al", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 Alpha (Asia 960805)", not_working: false, romset: &RS2_SFZ2ALR1 },
    GameInfo { name: "sfz2alj", parent: "sfz2al", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 Alpha (Japan 960805)", not_working: false, romset: &RS2_SFZ2ALJ },
    GameInfo { name: "sfz2alh", parent: "sfz2al", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 Alpha (Hispanic 960813)", not_working: false, romset: &RS2_SFZ2ALH },
    GameInfo { name: "sfz2alb", parent: "sfz2al", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 2 Alpha (Brazil 960813)", not_working: false, romset: &RS2_SFZ2ALB },
    GameInfo { name: "spf2t", parent: "", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Puzzle Fighter II Turbo (Europe 960529)", not_working: false, romset: &RS2_SPF2T },
    GameInfo { name: "spf2tu", parent: "spf2t", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Puzzle Fighter II Turbo (USA 960620)", not_working: false, romset: &RS2_SPF2TU },
    GameInfo { name: "spf2xj", parent: "spf2t", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Puzzle Fighter II X (Japan 960531)", not_working: false, romset: &RS2_SPF2XJ },
    GameInfo { name: "spf2ta", parent: "spf2t", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Puzzle Fighter II Turbo (Asia 960529)", not_working: false, romset: &RS2_SPF2TA },
    GameInfo { name: "spf2th", parent: "spf2t", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Puzzle Fighter II Turbo (Hispanic 960531)", not_working: false, romset: &RS2_SPF2TH },
    GameInfo { name: "megaman2", parent: "", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Mega Man 2: The Power Fighters (USA 960708)", not_working: false, romset: &RS2_MEGAMAN2 },
    GameInfo { name: "megaman2a", parent: "megaman2", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Mega Man 2: The Power Fighters (Asia 960708)", not_working: false, romset: &RS2_MEGAMAN2A },
    GameInfo { name: "rockman2j", parent: "megaman2", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Rockman 2: The Power Fighters (Japan 960708)", not_working: false, romset: &RS2_ROCKMAN2J },
    GameInfo { name: "megaman2h", parent: "megaman2", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Mega Man 2: The Power Fighters (Hispanic 960712)", not_working: false, romset: &RS2_MEGAMAN2H },
    GameInfo { name: "qndream", parent: "", machine: "cps2", input: "qndream", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Quiz Nanairo Dreams: Nijiirochou no Kiseki (Japan 960826)", not_working: false, romset: &RS2_QNDREAM },
    GameInfo { name: "xmvsf", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Europe 961004)", not_working: false, romset: &RS2_XMVSF },
    GameInfo { name: "xmvsfr1", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Europe 960910)", not_working: false, romset: &RS2_XMVSFR1 },
    GameInfo { name: "xmvsfu", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (USA 961023)", not_working: false, romset: &RS2_XMVSFU },
    GameInfo { name: "xmvsfur1", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (USA 961004)", not_working: false, romset: &RS2_XMVSFUR1 },
    GameInfo { name: "xmvsfur2", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (USA 960910)", not_working: false, romset: &RS2_XMVSFUR2 },
    GameInfo { name: "xmvsfj", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Japan 970113)", not_working: false, romset: &RS2_XMVSFJ },
    GameInfo { name: "xmvsfjr1", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Japan 961023)", not_working: false, romset: &RS2_XMVSFJR1 },
    GameInfo { name: "xmvsfjr2", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Japan 961004)", not_working: false, romset: &RS2_XMVSFJR2 },
    GameInfo { name: "xmvsfjr3", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Japan 960910)", not_working: false, romset: &RS2_XMVSFJR3 },
    GameInfo { name: "xmvsfjr4", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Japan 960909)", not_working: false, romset: &RS2_XMVSFJR4 },
    GameInfo { name: "xmvsfa", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Asia 961023)", not_working: false, romset: &RS2_XMVSFA },
    GameInfo { name: "xmvsfar1", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Asia 961004)", not_working: false, romset: &RS2_XMVSFAR1 },
    GameInfo { name: "xmvsfar2", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Asia 960919)", not_working: false, romset: &RS2_XMVSFAR2 },
    GameInfo { name: "xmvsfar3", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Asia 960910)", not_working: false, romset: &RS2_XMVSFAR3 },
    GameInfo { name: "xmvsfh", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Hispanic 961004)", not_working: false, romset: &RS2_XMVSFH },
    GameInfo { name: "xmvsfb", parent: "xmvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "X-Men Vs. Street Fighter (Brazil 961023)", not_working: false, romset: &RS2_XMVSFB },
    GameInfo { name: "batcir", parent: "", machine: "cps2", input: "cps2_4p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Battle Circuit (Europe 970319)", not_working: false, romset: &RS2_BATCIR },
    GameInfo { name: "batcira", parent: "batcir", machine: "cps2", input: "cps2_4p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Battle Circuit (Asia 970319)", not_working: false, romset: &RS2_BATCIRA },
    GameInfo { name: "batcirj", parent: "batcir", machine: "cps2", input: "cps2_4p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Battle Circuit (Japan 970319)", not_working: false, romset: &RS2_BATCIRJ },
    GameInfo { name: "vsav", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior: The Lord of Vampire (Europe 970519)", not_working: false, romset: &RS2_VSAV },
    GameInfo { name: "vsavu", parent: "vsav", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior: The Lord of Vampire (USA 970519)", not_working: false, romset: &RS2_VSAVU },
    GameInfo { name: "vsavj", parent: "vsav", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior: The Lord of Vampire (Japan 970519)", not_working: false, romset: &RS2_VSAVJ },
    GameInfo { name: "vsava", parent: "vsav", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior: The Lord of Vampire (Asia 970519)", not_working: false, romset: &RS2_VSAVA },
    GameInfo { name: "vsavh", parent: "vsav", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior: The Lord of Vampire (Hispanic 970519)", not_working: false, romset: &RS2_VSAVH },
    GameInfo { name: "vsavb", parent: "vsav", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior: The Lord of Vampire (Brazil 970519)", not_working: false, romset: &RS2_VSAVB },
    GameInfo { name: "mshvsf", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Europe 970625)", not_working: false, romset: &RS2_MSHVSF },
    GameInfo { name: "mshvsfu", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (USA 970827)", not_working: false, romset: &RS2_MSHVSFU },
    GameInfo { name: "mshvsfu1", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (USA 970625)", not_working: false, romset: &RS2_MSHVSFU1 },
    GameInfo { name: "mshvsfj", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Japan 970707)", not_working: false, romset: &RS2_MSHVSFJ },
    GameInfo { name: "mshvsfj1", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Japan 970702)", not_working: false, romset: &RS2_MSHVSFJ1 },
    GameInfo { name: "mshvsfj2", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Japan 970625)", not_working: false, romset: &RS2_MSHVSFJ2 },
    GameInfo { name: "mshvsfh", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Hispanic 970625)", not_working: false, romset: &RS2_MSHVSFH },
    GameInfo { name: "mshvsfa", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Asia 970625)", not_working: false, romset: &RS2_MSHVSFA },
    GameInfo { name: "mshvsfa1", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Asia 970620)", not_working: false, romset: &RS2_MSHVSFA1 },
    GameInfo { name: "mshvsfb", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Brazil 970827)", not_working: false, romset: &RS2_MSHVSFB },
    GameInfo { name: "mshvsfb1", parent: "mshvsf", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Super Heroes Vs. Street Fighter (Brazil 970625)", not_working: false, romset: &RS2_MSHVSFB1 },
    GameInfo { name: "csclub", parent: "", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Capcom Sports Club (Europe 971017)", not_working: false, romset: &RS2_CSCLUB },
    GameInfo { name: "csclub1", parent: "csclub", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Capcom Sports Club (Europe 970722)", not_working: false, romset: &RS2_CSCLUB1 },
    GameInfo { name: "cscluba", parent: "csclub", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Capcom Sports Club (Asia 970722)", not_working: false, romset: &RS2_CSCLUBA },
    GameInfo { name: "csclubj", parent: "csclub", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Capcom Sports Club (Japan 970722)", not_working: false, romset: &RS2_CSCLUBJ },
    GameInfo { name: "csclubjy", parent: "csclub", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Capcom Sports Club (Japan 970722, yellow case)", not_working: false, romset: &RS2_CSCLUBJY },
    GameInfo { name: "csclubh", parent: "csclub", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Capcom Sports Club (Hispanic 970722)", not_working: false, romset: &RS2_CSCLUBH },
    GameInfo { name: "sgemf", parent: "", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Gem Fighter: Mini Mix (USA 970904)", not_working: false, romset: &RS2_SGEMF },
    GameInfo { name: "pfghtj", parent: "sgemf", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Pocket Fighter (Japan 970904)", not_working: false, romset: &RS2_PFGHTJ },
    GameInfo { name: "sgemfa", parent: "sgemf", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Gem Fighter: Mini Mix (Asia 970904)", not_working: false, romset: &RS2_SGEMFA },
    GameInfo { name: "sgemfh", parent: "sgemf", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Super Gem Fighter: Mini Mix (Hispanic 970904)", not_working: false, romset: &RS2_SGEMFH },
    GameInfo { name: "vhunt2", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Hunter 2: Darkstalkers Revenge (Japan 970929)", not_working: false, romset: &RS2_VHUNT2 },
    GameInfo { name: "vhunt2r1", parent: "vhunt2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Hunter 2: Darkstalkers Revenge (Japan 970913)", not_working: false, romset: &RS2_VHUNT2R1 },
    GameInfo { name: "vsav2", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Vampire Savior 2: The Lord of Vampire (Japan 970913)", not_working: false, romset: &RS2_VSAV2 },
    GameInfo { name: "mvsc", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Europe 980123)", not_working: false, romset: &RS2_MVSC },
    GameInfo { name: "mvscr1", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Europe 980112)", not_working: false, romset: &RS2_MVSCR1 },
    GameInfo { name: "mvscu", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (USA 980123)", not_working: false, romset: &RS2_MVSCU },
    GameInfo { name: "mvscur1", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (USA 971222)", not_working: false, romset: &RS2_MVSCUR1 },
    GameInfo { name: "mvscj", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Japan 980123)", not_working: false, romset: &RS2_MVSCJ },
    GameInfo { name: "mvscjr1", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Japan 980112)", not_working: false, romset: &RS2_MVSCJR1 },
    GameInfo { name: "mvscjsing", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_singbrd", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Japan 980123) (Single PCB)", not_working: false, romset: &RS2_MVSCJSING },
    GameInfo { name: "mvsca", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Asia 980123)", not_working: false, romset: &RS2_MVSCA },
    GameInfo { name: "mvscar1", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Asia 980112)", not_working: false, romset: &RS2_MVSCAR1 },
    GameInfo { name: "mvsch", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Hispanic 980123)", not_working: false, romset: &RS2_MVSCH },
    GameInfo { name: "mvscb", parent: "mvsc", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Marvel Vs. Capcom: Clash of Super Heroes (Brazil 980123)", not_working: false, romset: &RS2_MVSCB },
    GameInfo { name: "sfa3", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (Europe 980904)", not_working: false, romset: &RS2_SFA3 },
    GameInfo { name: "sfa3u", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (USA 980904)", not_working: false, romset: &RS2_SFA3U },
    GameInfo { name: "sfa3ur1", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (USA 980629)", not_working: false, romset: &RS2_SFA3UR1 },
    GameInfo { name: "sfa3us", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (USA 980616, SAMPLE Version)", not_working: false, romset: &RS2_SFA3US },
    GameInfo { name: "sfa3h", parent: "sfa3", machine: "cps2", input: "cps2_2p6bt", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (Hispanic 980904)", not_working: false, romset: &RS2_SFA3H },
    GameInfo { name: "sfa3hr1", parent: "sfa3", machine: "cps2", input: "cps2_2p6bt", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (Hispanic 980629)", not_working: false, romset: &RS2_SFA3HR1 },
    GameInfo { name: "sfa3b", parent: "sfa3", machine: "cps2", input: "cps2_2p6bt", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Alpha 3 (Brazil 980629)", not_working: false, romset: &RS2_SFA3B },
    GameInfo { name: "sfz3j", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 3 (Japan 980904)", not_working: false, romset: &RS2_SFZ3J },
    GameInfo { name: "sfz3jr1", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 3 (Japan 980727)", not_working: false, romset: &RS2_SFZ3JR1 },
    GameInfo { name: "sfz3jr2", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 3 (Japan 980629)", not_working: false, romset: &RS2_SFZ3JR2 },
    GameInfo { name: "sfz3a", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 3 (Asia 980904)", not_working: false, romset: &RS2_SFZ3A },
    GameInfo { name: "sfz3ar1", parent: "sfa3", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Street Fighter Zero 3 (Asia 980701)", not_working: false, romset: &RS2_SFZ3AR1 },
    GameInfo { name: "jyangoku", parent: "", machine: "cps2", input: "cps2_1p2b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Jyangokushi: Haoh no Saihai (Japan 990527)", not_working: false, romset: &RS2_JYANGOKU },
    GameInfo { name: "hsf2", parent: "", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Hyper Street Fighter II: The Anniversary Edition (USA 040202)", not_working: false, romset: &RS2_HSF2 },
    GameInfo { name: "hsf2a", parent: "hsf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Hyper Street Fighter II: The Anniversary Edition (Asia 040202)", not_working: false, romset: &RS2_HSF2A },
    GameInfo { name: "hsf2j", parent: "hsf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Hyper Street Fighter II: The Anniversary Edition (Japan 040202)", not_working: false, romset: &RS2_HSF2J },
    GameInfo { name: "hsf2j1", parent: "hsf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Hyper Street Fighter II: The Anniversary Edition (Japan 040123)", not_working: false, romset: &RS2_HSF2J1 },
    GameInfo { name: "hsf2j2", parent: "hsf2", machine: "cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "Capcom", title: "Hyper Street Fighter II: The Anniversary Edition (Japan 031222)", not_working: false, romset: &RS2_HSF2J2 },
    GameInfo { name: "gigawing", parent: "", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Giga Wing (USA 990222)", not_working: false, romset: &RS2_GIGAWING },
    GameInfo { name: "gigawingj", parent: "gigawing", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Giga Wing (Japan 990223)", not_working: false, romset: &RS2_GIGAWINGJ },
    GameInfo { name: "gigawinga", parent: "gigawing", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Giga Wing (Asia 990222)", not_working: false, romset: &RS2_GIGAWINGA },
    GameInfo { name: "gigawingh", parent: "gigawing", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Giga Wing (Hispanic 990222)", not_working: false, romset: &RS2_GIGAWINGH },
    GameInfo { name: "gigawingb", parent: "gigawing", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Giga Wing (Brazil 990222)", not_working: false, romset: &RS2_GIGAWINGB },
    GameInfo { name: "mmatrix", parent: "", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Mars Matrix: Hyper Solid Shooting (USA 000412)", not_working: false, romset: &RS2_MMATRIX },
    GameInfo { name: "mmatrixa", parent: "mmatrix", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Mars Matrix: Hyper Solid Shooting (Asia 000412)", not_working: false, romset: &RS2_MMATRIXA },
    GameInfo { name: "mmatrixj", parent: "mmatrix", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Takumi (Capcom license)", title: "Mars Matrix: Hyper Solid Shooting (Japan 000412)", not_working: false, romset: &RS2_MMATRIXJ },
    GameInfo { name: "mpang", parent: "", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Mighty! Pang (Europe 001010)", not_working: false, romset: &RS2_MPANG },
    GameInfo { name: "mpangr1", parent: "mpang", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Mighty! Pang (Europe 000925)", not_working: false, romset: &RS2_MPANGR1 },
    GameInfo { name: "mpangu", parent: "mpang", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Mighty! Pang (USA 001010)", not_working: false, romset: &RS2_MPANGU },
    GameInfo { name: "mpangj", parent: "mpang", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Mighty! Pang (Japan 001011)", not_working: false, romset: &RS2_MPANGJ },
    GameInfo { name: "mpanga", parent: "mpang", machine: "cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Mighty! Pang (Asia 001010)", not_working: false, romset: &RS2_MPANGA },
    GameInfo { name: "pzloop2", parent: "", machine: "cps2", input: "pzloop2", init: "init_pzloop2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Puzz Loop 2 (Europe 010302)", not_working: false, romset: &RS2_PZLOOP2 },
    GameInfo { name: "pzloop2j", parent: "pzloop2", machine: "cps2", input: "pzloop2", init: "init_pzloop2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Puzz Loop 2 (Japan 010226)", not_working: false, romset: &RS2_PZLOOP2J },
    GameInfo { name: "pzloop2jr1", parent: "pzloop2", machine: "cps2", input: "pzloop2", init: "init_pzloop2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Puzz Loop 2 (Japan 010205)", not_working: false, romset: &RS2_PZLOOP2JR1 },
    GameInfo { name: "pzloop2jp", parent: "pzloop2", machine: "dead_cps2", input: "pzloop2", init: "init_pzloop2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Puzz Loop 2 (Japan 010201 Publicity)", not_working: false, romset: &RS2_PZLOOP2JP },
    GameInfo { name: "choko", parent: "", machine: "cps2", input: "choko", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Janpai Puzzle Choukou (Japan 010820)", not_working: false, romset: &RS2_CHOKO },
    GameInfo { name: "chokop", parent: "choko", machine: "dead_cps2", input: "choko", init: "init_cps2", rotation: 0, maker: "Mitchell (Capcom license)", title: "Janpai Puzzle Choukou (Japan 010820 Publicity)", not_working: false, romset: &RS2_CHOKOP },
    GameInfo { name: "dimahoo", parent: "", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 270, maker: "Eighting / Raizing (Capcom license)", title: "Dimahoo (Europe 000121)", not_working: false, romset: &RS2_DIMAHOO },
    GameInfo { name: "dimahoou", parent: "dimahoo", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 270, maker: "Eighting / Raizing (Capcom license)", title: "Dimahoo (USA 000121)", not_working: false, romset: &RS2_DIMAHOOU },
    GameInfo { name: "gmahou", parent: "dimahoo", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 270, maker: "Eighting / Raizing (Capcom license)", title: "Great Mahou Daisakusen (Japan 000121)", not_working: false, romset: &RS2_GMAHOU },
    GameInfo { name: "1944", parent: "", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Eighting / Raizing (Capcom license)", title: "1944: The Loop Master (Europe 000620)", not_working: false, romset: &RS2_1944 },
    GameInfo { name: "1944j", parent: "1944", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Eighting / Raizing (Capcom license)", title: "1944: The Loop Master (Japan 000620)", not_working: false, romset: &RS2_1944J },
    GameInfo { name: "1944u", parent: "1944", machine: "cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "Eighting / Raizing (Capcom license)", title: "1944: The Loop Master (USA 000620)", not_working: false, romset: &RS2_1944U },
    GameInfo { name: "progear", parent: "", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Cave (Capcom license)", title: "Progear (Europe 010117)", not_working: false, romset: &RS2_PROGEAR },
    GameInfo { name: "progearu", parent: "progear", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Cave (Capcom license)", title: "Progear (USA 010117)", not_working: false, romset: &RS2_PROGEARU },
    GameInfo { name: "progearj", parent: "progear", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Cave (Capcom license)", title: "Progear no Arashi (Japan 010117)", not_working: false, romset: &RS2_PROGEARJ },
    GameInfo { name: "progeara", parent: "progear", machine: "cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "Cave (Capcom license)", title: "Progear (Asia 010117)", not_working: false, romset: &RS2_PROGEARA },
    GameInfo { name: "ddtodd", parent: "ddtod", machine: "dead_cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Dungeons & Dragons: Tower of Doom (Europe 940412 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_DDTODD },
    GameInfo { name: "ecofghtrd", parent: "ecofghtr", machine: "dead_cps2", input: "ecofghtr", init: "init_ecofghtr", rotation: 0, maker: "bootleg", title: "Eco Fighters (World 931203 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_ECOFGHTRD },
    GameInfo { name: "ssf2ud", parent: "ssf2", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Street Fighter II: The New Challengers (USA 930911 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SSF2UD },
    GameInfo { name: "ssf2tbd", parent: "ssf2", machine: "dead_cps2comm", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Street Fighter II: The Tournament Battle (World 931119 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SSF2TBD },
    GameInfo { name: "armwar1d", parent: "armwar", machine: "dead_cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Armored Warriors (Europe 941011 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_ARMWAR1D },
    GameInfo { name: "avspd", parent: "avsp", machine: "dead_cps2", input: "cps2_3p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Alien vs. Predator (Europe 940520 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_AVSPD },
    GameInfo { name: "dstlku1d", parent: "dstlk", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Darkstalkers: The Night Warriors (USA 940705 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_DSTLKU1D },
    GameInfo { name: "ringdstd", parent: "ringdest", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Ring of Destruction: Slammasters II (Europe 940902 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_RINGDSTD },
    GameInfo { name: "ssf2tad", parent: "ssf2t", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Street Fighter II Turbo (Asia 940223 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SSF2TAD },
    GameInfo { name: "ssf2xjr1d", parent: "ssf2t", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Street Fighter II X: Grand Master Challenge (Japan 940223 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SSF2XJR1D },
    GameInfo { name: "xmcotar1d", parent: "xmcota", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "X-Men: Children of the Atom (Europe 950105 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_XMCOTAR1D },
    GameInfo { name: "mshud", parent: "msh", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Marvel Super Heroes (USA 951024 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_MSHUD },
    GameInfo { name: "cybotsud", parent: "cybots", machine: "dead_cps2", input: "cybots", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Cyberbots: Fullmetal Madness (USA 950424 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_CYBOTSUD },
    GameInfo { name: "cybotsjd", parent: "cybots", machine: "dead_cps2", input: "cybots", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Cyberbots: Fullmetal Madness (Japan 950424) (decrypted bootleg)", not_working: false, romset: &RS2_CYBOTSJD },
    GameInfo { name: "nwarrud", parent: "nwarr", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Night Warriors: Darkstalkers' Revenge (USA 950406 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_NWARRUD },
    GameInfo { name: "sfad", parent: "sfa", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Street Fighter Alpha: Warriors' Dreams (Europe 950727 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SFAD },
    GameInfo { name: "19xxd", parent: "19xx", machine: "dead_cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 270, maker: "bootleg", title: "19XX: The War Against Destiny (USA 951207 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_19XXD },
    GameInfo { name: "ddsomud", parent: "ddsom", machine: "dead_cps2", input: "cps2_4p4b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Dungeons & Dragons: Shadow over Mystara (USA 960619 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_DDSOMUD },
    GameInfo { name: "gigaman2", parent: "megaman2", machine: "gigaman2", input: "cps2_2p3b", init: "init_gigaman2", rotation: 0, maker: "bootleg", title: "Giga Man 2: The Power Fighters (bootleg of Mega Man 2: The Power Fighters)", not_working: false, romset: &RS2_GIGAMAN2 },
    GameInfo { name: "megamn2d", parent: "megaman2", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Mega Man 2: The Power Fighters (USA 960708 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_MEGAMN2D },
    GameInfo { name: "sfz2ad", parent: "sfa2", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Street Fighter Zero 2 (Asia 960227 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SFZ2AD },
    GameInfo { name: "sfz2jd", parent: "sfa2", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Street Fighter Zero 2 (Japan 960227 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SFZ2JD },
    GameInfo { name: "spf2td", parent: "spf2t", machine: "dead_cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Puzzle Fighter II Turbo (USA 960620 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SPF2TD },
    GameInfo { name: "spf2xjd", parent: "spf2t", machine: "dead_cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Puzzle Fighter II X (Japan 960531 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SPF2XJD },
    GameInfo { name: "sfz2ald", parent: "sfz2al", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Street Fighter Zero 2 Alpha (Asia 960826 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SFZ2ALD },
    GameInfo { name: "xmvsfu1d", parent: "xmvsf", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "X-Men Vs. Street Fighter (USA 961004 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_XMVSFU1D },
    GameInfo { name: "batcird", parent: "batcir", machine: "dead_cps2", input: "cps2_4p2b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Battle Circuit (Europe 970319 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_BATCIRD },
    GameInfo { name: "csclub1d", parent: "csclub", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Capcom Sports Club (Europe 970722 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_CSCLUB1D },
    GameInfo { name: "mshvsfu1d", parent: "mshvsf", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Marvel Super Heroes Vs. Street Fighter (USA 970625 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_MSHVSFU1D },
    GameInfo { name: "sgemfd", parent: "sgemf", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Super Gem Fighter: Mini Mix (USA 970904 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SGEMFD },
    GameInfo { name: "vsavd", parent: "vsav", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Vampire Savior: The Lord of Vampire (Europe 970519 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_VSAVD },
    GameInfo { name: "vhunt2d", parent: "vhunt2", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Vampire Hunter 2: Darkstalkers Revenge (Japan 970913 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_VHUNT2D },
    GameInfo { name: "vsav2d", parent: "vsav2", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Vampire Savior 2: The Lord of Vampire (Japan 970913 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_VSAV2D },
    GameInfo { name: "mvscud", parent: "mvsc", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Marvel Vs. Capcom: Clash of Super Heroes (USA 980123 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_MVSCUD },
    GameInfo { name: "sfa3ud", parent: "sfa3", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Street Fighter Alpha 3 (USA 980904 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SFA3UD },
    GameInfo { name: "sfz3jr2d", parent: "sfa3", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Street Fighter Zero 3 (Japan 980629 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_SFZ3JR2D },
    GameInfo { name: "gigawingd", parent: "gigawing", machine: "dead_cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Giga Wing (USA 990222 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_GIGAWINGD },
    GameInfo { name: "gigawingjd", parent: "gigawing", machine: "dead_cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Giga Wing (Japan 990223 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_GIGAWINGJD },
    GameInfo { name: "1944d", parent: "1944", machine: "dead_cps2", input: "cps2_2p2b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "1944: The Loop Master (USA 000620 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_1944D },
    GameInfo { name: "dimahoud", parent: "dimahoo", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 270, maker: "bootleg", title: "Dimahoo (USA 000121 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_DIMAHOUD },
    GameInfo { name: "mmatrixd", parent: "mmatrix", machine: "dead_cps2", input: "cps2_2p1b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Mars Matrix: Hyper Solid Shooting (USA 000412 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_MMATRIXD },
    GameInfo { name: "progearud", parent: "progear", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Progear (USA 010117 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_PROGEARUD },
    GameInfo { name: "progearjd", parent: "progear", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Progear no Arashi (Japan 010117 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_PROGEARJD },
    GameInfo { name: "progearjbl", parent: "progear", machine: "dead_cps2", input: "cps2_2p3b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Progear no Arashi (Japan 010117) (decrypted bootleg)", not_working: false, romset: &RS2_PROGEARJBL },
    GameInfo { name: "pzloop2jd", parent: "pzloop2", machine: "dead_cps2", input: "pzloop2", init: "init_pzloop2", rotation: 0, maker: "bootleg", title: "Puzz Loop 2 (Japan 010226 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_PZLOOP2JD },
    GameInfo { name: "hsf2d", parent: "hsf2", machine: "dead_cps2", input: "cps2_2p6b", init: "init_cps2", rotation: 0, maker: "bootleg", title: "Hyper Street Fighter II: The Anniversary Edition (Asia 040202 Phoenix Edition) (bootleg)", not_working: false, romset: &RS2_HSF2D },
];
