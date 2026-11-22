// Fichier GÉNÉRÉ par tools/extract_romsets.py depuis ref/mame-cps1/cps1.cpp — NE PAS MODIFIER À LA MAIN.
// license:BSD-3-Clause (données MAME, copyright-holders: Paul Leaman)
//! Romsets CPS1 (régions maincpu, gfx, audiocpu, oki, stars) et table des jeux. spec: docs/spec/04

use crate::romset::{GameInfo, Load, Op, RegionDef, RomSetDef};

/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTN: RomSetDef = RomSetDef { name: "forgottn", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw40.12f", offset: 0x0, size: 0x20000, crc: 0x73E920B7 },
            Op::Load { kind: Load::Byte16, file: "lw41.12h", offset: 0x1, size: 0x20000, crc: 0x58210B9E },
            Op::Load { kind: Load::Byte16, file: "lw42.13f", offset: 0x40000, size: 0x20000, crc: 0xBEA45994 },
            Op::Load { kind: Load::Byte16, file: "lw43.13h", offset: 0x40001, size: 0x20000, crc: 0x539B2339 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.10g", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "lw_2.2b", offset: 0x0, size: 0x20000, crc: 0x4BD75FEE },
            Op::Load { kind: Load::Byte64, file: "lw_1.2a", offset: 0x1, size: 0x20000, crc: 0x65F41485 },
            Op::Load { kind: Load::Word64, file: "lw-08.9b", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Byte64, file: "lw_18.5e", offset: 0x4, size: 0x20000, crc: 0xB4B6241B },
            Op::Load { kind: Load::Byte64, file: "lw_17.5c", offset: 0x5, size: 0x20000, crc: 0xC5EEA115 },
            Op::Load { kind: Load::Byte64, file: "lw_30.8h", offset: 0x6, size: 0x20000, crc: 0xB385954E },
            Op::Load { kind: Load::Byte64, file: "lw_29.8f", offset: 0x7, size: 0x20000, crc: 0x7BDA1AC6 },
            Op::Load { kind: Load::Byte64, file: "lw_4.3b", offset: 0x100000, size: 0x20000, crc: 0x50CF757F },
            Op::Load { kind: Load::Byte64, file: "lw_3.3a", offset: 0x100001, size: 0x20000, crc: 0xC03EF278 },
            Op::Load { kind: Load::Byte64, file: "lw_20.7e", offset: 0x100004, size: 0x20000, crc: 0xDF1A3665 },
            Op::Load { kind: Load::Byte64, file: "lw_19.7c", offset: 0x100005, size: 0x20000, crc: 0x15AF8440 },
            Op::Load { kind: Load::Byte64, file: "lw_32.9h", offset: 0x100006, size: 0x20000, crc: 0x30967A15 },
            Op::Load { kind: Load::Byte64, file: "lw_31.9f", offset: 0x100007, size: 0x20000, crc: 0xC49D37FB },
            Op::Load { kind: Load::Word64, file: "lw-02.6b", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Byte64, file: "lw_14.10b", offset: 0x200002, size: 0x20000, crc: 0x82862CCE },
            Op::Load { kind: Load::Byte64, file: "lw_13.10a", offset: 0x200003, size: 0x20000, crc: 0xB81C0E96 },
            Op::Load { kind: Load::Word64, file: "lw-06.9d", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Byte64, file: "lw_26.10e", offset: 0x200006, size: 0x20000, crc: 0x57BCD032 },
            Op::Load { kind: Load::Byte64, file: "lw_25.10c", offset: 0x200007, size: 0x20000, crc: 0xBAC91554 },
            Op::Load { kind: Load::Byte64, file: "lw_16.11b", offset: 0x300002, size: 0x20000, crc: 0x40B26554 },
            Op::Load { kind: Load::Byte64, file: "lw_15.11a", offset: 0x300003, size: 0x20000, crc: 0x1B7D2E07 },
            Op::Load { kind: Load::Byte64, file: "lw_28.11e", offset: 0x300006, size: 0x20000, crc: 0xA805AD30 },
            Op::Load { kind: Load::Byte64, file: "lw_27.11c", offset: 0x300007, size: 0x20000, crc: 0x103C1BD2 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_37.13c", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.12e", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13e", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNA: RomSetDef = RomSetDef { name: "forgottna", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw11.12f", offset: 0x0, size: 0x20000, crc: 0x73E920B7 },
            Op::Load { kind: Load::Byte16, file: "lw15.12h", offset: 0x1, size: 0x20000, crc: 0x50D7012D },
            Op::Load { kind: Load::Byte16, file: "lw10.13f", offset: 0x40000, size: 0x20000, crc: 0xBEA45994 },
            Op::Load { kind: Load::Byte16, file: "lw14.13h", offset: 0x40001, size: 0x20000, crc: 0x539B2339 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.10g", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "lw_2.2b", offset: 0x0, size: 0x20000, crc: 0x4BD75FEE },
            Op::Load { kind: Load::Byte64, file: "lw_1.2a", offset: 0x1, size: 0x20000, crc: 0x65F41485 },
            Op::Load { kind: Load::Word64, file: "lw-08.9b", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.6d", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Byte64, file: "lw_30.8h", offset: 0x6, size: 0x20000, crc: 0xB385954E },
            Op::Load { kind: Load::Byte64, file: "lw_29.8f", offset: 0x7, size: 0x20000, crc: 0x7BDA1AC6 },
            Op::Load { kind: Load::Byte64, file: "lw_4.3b", offset: 0x100000, size: 0x20000, crc: 0x50CF757F },
            Op::Load { kind: Load::Byte64, file: "lw_3.3a", offset: 0x100001, size: 0x20000, crc: 0xC03EF278 },
            Op::Load { kind: Load::Byte64, file: "lw_32.9h", offset: 0x100006, size: 0x20000, crc: 0x30967A15 },
            Op::Load { kind: Load::Byte64, file: "lw_31.9f", offset: 0x100007, size: 0x20000, crc: 0xC49D37FB },
            Op::Load { kind: Load::Word64, file: "lw-02.6b", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Byte64, file: "lw_14.10b", offset: 0x200002, size: 0x20000, crc: 0x82862CCE },
            Op::Load { kind: Load::Byte64, file: "lw_13.10a", offset: 0x200003, size: 0x20000, crc: 0xB81C0E96 },
            Op::Load { kind: Load::Word64, file: "lw-06.9d", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Byte64, file: "lw_26.10e", offset: 0x200006, size: 0x20000, crc: 0x57BCD032 },
            Op::Load { kind: Load::Byte64, file: "lw_25.10c", offset: 0x200007, size: 0x20000, crc: 0xBAC91554 },
            Op::Load { kind: Load::Byte64, file: "lw_16.11b", offset: 0x300002, size: 0x20000, crc: 0x40B26554 },
            Op::Load { kind: Load::Byte64, file: "lw_15.11a", offset: 0x300003, size: 0x20000, crc: 0x1B7D2E07 },
            Op::Load { kind: Load::Byte64, file: "lw_28.11e", offset: 0x300006, size: 0x20000, crc: 0xA805AD30 },
            Op::Load { kind: Load::Byte64, file: "lw_27.11c", offset: 0x300007, size: 0x20000, crc: 0x103C1BD2 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_00.13c", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.12e", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13e", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNU: RomSetDef = RomSetDef { name: "forgottnu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw11c.12f", offset: 0x0, size: 0x20000, crc: 0xE62742B6 },
            Op::Load { kind: Load::Byte16, file: "lw15c.12h", offset: 0x1, size: 0x20000, crc: 0x1B70F216 },
            Op::Load { kind: Load::Byte16, file: "lw10c.13f", offset: 0x40000, size: 0x20000, crc: 0x8F5EA3F5 },
            Op::Load { kind: Load::Byte16, file: "lw14c.13h", offset: 0x40001, size: 0x20000, crc: 0x708E7472 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.10g", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "lw_2.2b", offset: 0x0, size: 0x20000, crc: 0x4BD75FEE },
            Op::Load { kind: Load::Byte64, file: "lw_1.2a", offset: 0x1, size: 0x20000, crc: 0x65F41485 },
            Op::Load { kind: Load::Word64, file: "lw-08.9b", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.6d", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Byte64, file: "lw_30.8h", offset: 0x6, size: 0x20000, crc: 0xB385954E },
            Op::Load { kind: Load::Byte64, file: "lw_29.8f", offset: 0x7, size: 0x20000, crc: 0x7BDA1AC6 },
            Op::Load { kind: Load::Byte64, file: "lw_4.3b", offset: 0x100000, size: 0x20000, crc: 0x50CF757F },
            Op::Load { kind: Load::Byte64, file: "lw_3.3a", offset: 0x100001, size: 0x20000, crc: 0xC03EF278 },
            Op::Load { kind: Load::Byte64, file: "lw_32.9h", offset: 0x100006, size: 0x20000, crc: 0x30967A15 },
            Op::Load { kind: Load::Byte64, file: "lw_31.9f", offset: 0x100007, size: 0x20000, crc: 0xC49D37FB },
            Op::Load { kind: Load::Word64, file: "lw-02.6b", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Byte64, file: "lw_14.10b", offset: 0x200002, size: 0x20000, crc: 0x82862CCE },
            Op::Load { kind: Load::Byte64, file: "lw_13.10a", offset: 0x200003, size: 0x20000, crc: 0xB81C0E96 },
            Op::Load { kind: Load::Word64, file: "lw-06.9d", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.10d", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
            Op::Load { kind: Load::Byte64, file: "lw_16.11b", offset: 0x300002, size: 0x20000, crc: 0x40B26554 },
            Op::Load { kind: Load::Byte64, file: "lw_15.11a", offset: 0x300003, size: 0x20000, crc: 0x1B7D2E07 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_00.13c", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.12e", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13e", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNUE: RomSetDef = RomSetDef { name: "forgottnue", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw11e.14f", offset: 0x0, size: 0x20000, crc: 0x82656910 },
            Op::Load { kind: Load::Byte16, file: "lw15e.14g", offset: 0x1, size: 0x20000, crc: 0xFB1E2BD0 },
            Op::Load { kind: Load::Byte16, file: "lw10e.13f", offset: 0x40000, size: 0x20000, crc: 0x3CE81DBE },
            Op::Load { kind: Load::Byte16, file: "lw14e.13g", offset: 0x40001, size: 0x20000, crc: 0x472EAAD1 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_00.14a", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.14c", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13c", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNUC: RomSetDef = RomSetDef { name: "forgottnuc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw11c.14f", offset: 0x0, size: 0x20000, crc: 0xE62742B6 },
            Op::Load { kind: Load::Byte16, file: "lw15c.14g", offset: 0x1, size: 0x20000, crc: 0x1B70F216 },
            Op::Load { kind: Load::Byte16, file: "lw10c.13f", offset: 0x40000, size: 0x20000, crc: 0x8F5EA3F5 },
            Op::Load { kind: Load::Byte16, file: "lw14c.13g", offset: 0x40001, size: 0x20000, crc: 0x708E7472 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_00.14a", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.14c", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13c", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNUA: RomSetDef = RomSetDef { name: "forgottnua", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lwu_11a.14f", offset: 0x0, size: 0x20000, crc: 0xDDF78831 },
            Op::Load { kind: Load::Byte16, file: "lwu_15a.14g", offset: 0x1, size: 0x20000, crc: 0xF7CE2097 },
            Op::Load { kind: Load::Byte16, file: "lwu_10a.13f", offset: 0x40000, size: 0x20000, crc: 0x8CB38C81 },
            Op::Load { kind: Load::Byte16, file: "lwu_14a.13g", offset: 0x40001, size: 0x20000, crc: 0xD70EF9FD },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lwu_00.14a", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.14c", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13c", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNUAA: RomSetDef = RomSetDef { name: "forgottnuaa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lwu_11aa.14f", offset: 0x0, size: 0x20000, crc: 0x73E920B7 },
            Op::Load { kind: Load::Byte16, file: "lwu_15aa.14g", offset: 0x1, size: 0x20000, crc: 0xE47524B9 },
            Op::Load { kind: Load::Byte16, file: "lwu_10aa.13f", offset: 0x40000, size: 0x20000, crc: 0xBEA45994 },
            Op::Load { kind: Load::Byte16, file: "lwu_14aa.13g", offset: 0x40001, size: 0x20000, crc: 0x539B2339 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lwu_00.14a", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.14c", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13c", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FORGOTTNJ: RomSetDef = RomSetDef { name: "forgottnj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "11.14f", offset: 0x0, size: 0x20000, crc: 0xC9A6B319 },
            Op::Load { kind: Load::Byte16, file: "15.14g", offset: 0x1, size: 0x20000, crc: 0x524F920E },
            Op::Load { kind: Load::Byte16, file: "10.13f", offset: 0x40000, size: 0x20000, crc: 0xFF7E41D9 },
            Op::Load { kind: Load::Byte16, file: "14.13g", offset: 0x40001, size: 0x20000, crc: 0x80C3A813 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "00.14a", offset: 0x0, size: 0x8000, crc: 0x3BC962EA },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03u.14c", offset: 0x0, size: 0x20000, crc: 0x807D051F },
            Op::Load { kind: Load::Byte, file: "lw-04u.13c", offset: 0x20000, size: 0x20000, crc: 0xE6CD098E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_LOSTWRLD: RomSetDef = RomSetDef { name: "lostwrld", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw_11c.14f", offset: 0x0, size: 0x20000, crc: 0x67E42546 },
            Op::Load { kind: Load::Byte16, file: "lw_15c.14g", offset: 0x1, size: 0x20000, crc: 0x402E2A46 },
            Op::Load { kind: Load::Byte16, file: "lw_10c.13f", offset: 0x40000, size: 0x20000, crc: 0xC46479D7 },
            Op::Load { kind: Load::Byte16, file: "lw_14c.13g", offset: 0x40001, size: 0x20000, crc: 0x97670F4A },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_00b.14a", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03.14c", offset: 0x0, size: 0x20000, crc: 0xCE2159E7 },
            Op::Load { kind: Load::Byte, file: "lw-04.13c", offset: 0x20000, size: 0x20000, crc: 0x39305536 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_LOSTWRLDO: RomSetDef = RomSetDef { name: "lostwrldo", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "lw_11.14f", offset: 0x0, size: 0x20000, crc: 0x61E2CC56 },
            Op::Load { kind: Load::Byte16, file: "lw_15.14g", offset: 0x1, size: 0x20000, crc: 0x8A0C18D3 },
            Op::Load { kind: Load::Byte16, file: "lw_10.13f", offset: 0x40000, size: 0x20000, crc: 0x23BCA4D5 },
            Op::Load { kind: Load::Byte16, file: "lw_14.13g", offset: 0x40001, size: 0x20000, crc: 0x3A023771 },
            Op::Load { kind: Load::Word16Swap, file: "lw-07.13e", offset: 0x80000, size: 0x80000, crc: 0xFD252A26 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "lw-01.9d", offset: 0x0, size: 0x80000, crc: 0x0318F298 },
            Op::Load { kind: Load::Word64, file: "lw-08.9f", offset: 0x2, size: 0x80000, crc: 0x25A8E43C },
            Op::Load { kind: Load::Word64, file: "lw-05.9e", offset: 0x4, size: 0x80000, crc: 0xE4552FD7 },
            Op::Load { kind: Load::Word64, file: "lw-12.9g", offset: 0x6, size: 0x80000, crc: 0x8E6A832B },
            Op::Load { kind: Load::Word64, file: "lw-02.12d", offset: 0x200000, size: 0x80000, crc: 0x43E6C5C8 },
            Op::Load { kind: Load::Word64, file: "lw-09.12f", offset: 0x200002, size: 0x80000, crc: 0x899CB4AD },
            Op::Load { kind: Load::Word64, file: "lw-06.12e", offset: 0x200004, size: 0x80000, crc: 0x5B9EDFFC },
            Op::Load { kind: Load::Word64, file: "lw-13.12g", offset: 0x200006, size: 0x80000, crc: 0x8E058EF5 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x200000, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw_00b.14a", offset: 0x0, size: 0x8000, crc: 0x59DF2A63 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "lw-03.14c", offset: 0x0, size: 0x20000, crc: 0xCE2159E7 },
            Op::Load { kind: Load::Byte, file: "lw-04.13c", offset: 0x20000, size: 0x20000, crc: 0x39305536 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_GHOULS: RomSetDef = RomSetDef { name: "ghouls", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "dme_29.10h", offset: 0x0, size: 0x20000, crc: 0x166A58A2 },
            Op::Load { kind: Load::Byte16, file: "dme_30.10j", offset: 0x1, size: 0x20000, crc: 0x7AC8407A },
            Op::Load { kind: Load::Byte16, file: "dme_27.9h", offset: 0x40000, size: 0x20000, crc: 0xF734B2BE },
            Op::Load { kind: Load::Byte16, file: "dme_28.9j", offset: 0x40001, size: 0x20000, crc: 0x03D3E714 },
            Op::Load { kind: Load::Word16, file: "dm-17.7j", offset: 0x80000, size: 0x80000, crc: 0x3EA1B0F2 },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dm-05.3a", offset: 0x0, size: 0x80000, crc: 0x0BA9C0B0 },
            Op::Load { kind: Load::Word64, file: "dm-07.3f", offset: 0x2, size: 0x80000, crc: 0x5D760AB9 },
            Op::Load { kind: Load::Word64, file: "dm-06.3c", offset: 0x4, size: 0x80000, crc: 0x4BA90B59 },
            Op::Load { kind: Load::Word64, file: "dm-08.3g", offset: 0x6, size: 0x80000, crc: 0x4BDEE9DE },
            Op::Load { kind: Load::Byte64, file: "09.4a", offset: 0x200000, size: 0x10000, crc: 0xAE24BB19 },
            Op::Load { kind: Load::Byte64, file: "18.7a", offset: 0x200001, size: 0x10000, crc: 0xD34E271A },
            Op::Load { kind: Load::Byte64, file: "13.4e", offset: 0x200002, size: 0x10000, crc: 0x3F70DD37 },
            Op::Load { kind: Load::Byte64, file: "22.7e", offset: 0x200003, size: 0x10000, crc: 0x7E69E2E6 },
            Op::Load { kind: Load::Byte64, file: "11.4c", offset: 0x200004, size: 0x10000, crc: 0x37C9B6C6 },
            Op::Load { kind: Load::Byte64, file: "20.7c", offset: 0x200005, size: 0x10000, crc: 0x2F1345B4 },
            Op::Load { kind: Load::Byte64, file: "15.4g", offset: 0x200006, size: 0x10000, crc: 0x3C2A212A },
            Op::Load { kind: Load::Byte64, file: "24.7g", offset: 0x200007, size: 0x10000, crc: 0x889AAC05 },
            Op::Load { kind: Load::Byte64, file: "10.4b", offset: 0x280000, size: 0x10000, crc: 0xBCC0F28C },
            Op::Load { kind: Load::Byte64, file: "19.7b", offset: 0x280001, size: 0x10000, crc: 0x2A40166A },
            Op::Load { kind: Load::Byte64, file: "14.4f", offset: 0x280002, size: 0x10000, crc: 0x20F85C03 },
            Op::Load { kind: Load::Byte64, file: "23.7f", offset: 0x280003, size: 0x10000, crc: 0x8426144B },
            Op::Load { kind: Load::Byte64, file: "12.4d", offset: 0x280004, size: 0x10000, crc: 0xDA088D61 },
            Op::Load { kind: Load::Byte64, file: "21.7d", offset: 0x280005, size: 0x10000, crc: 0x17E11DF0 },
            Op::Load { kind: Load::Byte64, file: "16.4h", offset: 0x280006, size: 0x10000, crc: 0xF187BA1C },
            Op::Load { kind: Load::Byte64, file: "25.7h", offset: 0x280007, size: 0x10000, crc: 0x29F79C78 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "26.10a", offset: 0x0, size: 0x8000, crc: 0x3692F6E5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0xff, ops: &[
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_GHOULSU: RomSetDef = RomSetDef { name: "ghoulsu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "dmu_29.10h", offset: 0x0, size: 0x20000, crc: 0x334D85B2 },
            Op::Load { kind: Load::Byte16, file: "dmu_30.10j", offset: 0x1, size: 0x20000, crc: 0xCEE8CEB5 },
            Op::Load { kind: Load::Byte16, file: "dmu_27.9h", offset: 0x40000, size: 0x20000, crc: 0x4A524140 },
            Op::Load { kind: Load::Byte16, file: "dmu_28.9j", offset: 0x40001, size: 0x20000, crc: 0x94AAE205 },
            Op::Load { kind: Load::Word16, file: "dm-17.7j", offset: 0x80000, size: 0x80000, crc: 0x3EA1B0F2 },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dm-05.3a", offset: 0x0, size: 0x80000, crc: 0x0BA9C0B0 },
            Op::Load { kind: Load::Word64, file: "dm-07.3f", offset: 0x2, size: 0x80000, crc: 0x5D760AB9 },
            Op::Load { kind: Load::Word64, file: "dm-06.3c", offset: 0x4, size: 0x80000, crc: 0x4BA90B59 },
            Op::Load { kind: Load::Word64, file: "dm-08.3g", offset: 0x6, size: 0x80000, crc: 0x4BDEE9DE },
            Op::Load { kind: Load::Byte64, file: "09.4a", offset: 0x200000, size: 0x10000, crc: 0xAE24BB19 },
            Op::Load { kind: Load::Byte64, file: "18.7a", offset: 0x200001, size: 0x10000, crc: 0xD34E271A },
            Op::Load { kind: Load::Byte64, file: "13.4e", offset: 0x200002, size: 0x10000, crc: 0x3F70DD37 },
            Op::Load { kind: Load::Byte64, file: "22.7e", offset: 0x200003, size: 0x10000, crc: 0x7E69E2E6 },
            Op::Load { kind: Load::Byte64, file: "11.4c", offset: 0x200004, size: 0x10000, crc: 0x37C9B6C6 },
            Op::Load { kind: Load::Byte64, file: "20.7c", offset: 0x200005, size: 0x10000, crc: 0x2F1345B4 },
            Op::Load { kind: Load::Byte64, file: "15.4g", offset: 0x200006, size: 0x10000, crc: 0x3C2A212A },
            Op::Load { kind: Load::Byte64, file: "24.7g", offset: 0x200007, size: 0x10000, crc: 0x889AAC05 },
            Op::Load { kind: Load::Byte64, file: "10.4b", offset: 0x280000, size: 0x10000, crc: 0xBCC0F28C },
            Op::Load { kind: Load::Byte64, file: "19.7b", offset: 0x280001, size: 0x10000, crc: 0x2A40166A },
            Op::Load { kind: Load::Byte64, file: "14.4f", offset: 0x280002, size: 0x10000, crc: 0x20F85C03 },
            Op::Load { kind: Load::Byte64, file: "23.7f", offset: 0x280003, size: 0x10000, crc: 0x8426144B },
            Op::Load { kind: Load::Byte64, file: "12.4d", offset: 0x280004, size: 0x10000, crc: 0xDA088D61 },
            Op::Load { kind: Load::Byte64, file: "21.7d", offset: 0x280005, size: 0x10000, crc: 0x17E11DF0 },
            Op::Load { kind: Load::Byte64, file: "16.4h", offset: 0x280006, size: 0x10000, crc: 0xF187BA1C },
            Op::Load { kind: Load::Byte64, file: "25.7h", offset: 0x280007, size: 0x10000, crc: 0x29F79C78 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "26.10a", offset: 0x0, size: 0x8000, crc: 0x3692F6E5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0xff, ops: &[
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_DAIMAKAI: RomSetDef = RomSetDef { name: "daimakai", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "dmj_38.12f", offset: 0x0, size: 0x20000, crc: 0x82FD1798 },
            Op::Load { kind: Load::Byte16, file: "dmj_39.12h", offset: 0x1, size: 0x20000, crc: 0x35366CCC },
            Op::Load { kind: Load::Byte16, file: "dmj_40.13f", offset: 0x40000, size: 0x20000, crc: 0xA17C170A },
            Op::Load { kind: Load::Byte16, file: "dmj_41.13h", offset: 0x40001, size: 0x20000, crc: 0x6AF0B391 },
            Op::Load { kind: Load::Byte16, file: "dm_33.10f", offset: 0x80000, size: 0x20000, crc: 0x384D60C4 },
            Op::Load { kind: Load::Byte16, file: "dm_34.10h", offset: 0x80001, size: 0x20000, crc: 0x19ABE30F },
            Op::Load { kind: Load::Byte16, file: "dm_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xC04B85C8 },
            Op::Load { kind: Load::Byte16, file: "dm_36.11h", offset: 0xc0001, size: 0x20000, crc: 0x89BE83DE },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "dm_02.4b", offset: 0x0, size: 0x20000, crc: 0x8B98DC48 },
            Op::Load { kind: Load::Byte64, file: "dm_01.4a", offset: 0x1, size: 0x20000, crc: 0x80896C33 },
            Op::Load { kind: Load::Byte64, file: "dm_10.9b", offset: 0x2, size: 0x20000, crc: 0xC2E7D9EF },
            Op::Load { kind: Load::Byte64, file: "dm_09.9a", offset: 0x3, size: 0x20000, crc: 0xC9C4AFA5 },
            Op::Load { kind: Load::Byte64, file: "dm_18.5e", offset: 0x4, size: 0x20000, crc: 0x1AA0DB99 },
            Op::Load { kind: Load::Byte64, file: "dm_17.5c", offset: 0x5, size: 0x20000, crc: 0xDC6ED8AD },
            Op::Load { kind: Load::Byte64, file: "dm_30.8h", offset: 0x6, size: 0x20000, crc: 0xD9D3F8BD },
            Op::Load { kind: Load::Byte64, file: "dm_29.8f", offset: 0x7, size: 0x20000, crc: 0x49A48796 },
            Op::Load { kind: Load::Byte64, file: "dm_04.5b", offset: 0x100000, size: 0x20000, crc: 0xA4F4F8F0 },
            Op::Load { kind: Load::Byte64, file: "dm_03.5a", offset: 0x100001, size: 0x20000, crc: 0xB1033E62 },
            Op::Load { kind: Load::Byte64, file: "dm_12.10b", offset: 0x100002, size: 0x20000, crc: 0x10FDD76A },
            Op::Load { kind: Load::Byte64, file: "dm_11.10a", offset: 0x100003, size: 0x20000, crc: 0x9040CB04 },
            Op::Load { kind: Load::Byte64, file: "dm_20.7e", offset: 0x100004, size: 0x20000, crc: 0x281D0B3E },
            Op::Load { kind: Load::Byte64, file: "dm_19.7c", offset: 0x100005, size: 0x20000, crc: 0x2623B52F },
            Op::Load { kind: Load::Byte64, file: "dm_32.9h", offset: 0x100006, size: 0x20000, crc: 0x99692344 },
            Op::Load { kind: Load::Byte64, file: "dm_31.9f", offset: 0x100007, size: 0x20000, crc: 0x54ACB729 },
            Op::Load { kind: Load::Byte64, file: "dm_06.7b", offset: 0x200000, size: 0x10000, crc: 0xAE24BB19 },
            Op::Load { kind: Load::Byte64, file: "dm_05.7a", offset: 0x200001, size: 0x10000, crc: 0xD34E271A },
            Op::Load { kind: Load::Byte64, file: "dm_14.11b", offset: 0x200002, size: 0x10000, crc: 0x3F70DD37 },
            Op::Load { kind: Load::Byte64, file: "dm_13.11a", offset: 0x200003, size: 0x10000, crc: 0x7E69E2E6 },
            Op::Load { kind: Load::Byte64, file: "dm_22.8e", offset: 0x200004, size: 0x10000, crc: 0x37C9B6C6 },
            Op::Load { kind: Load::Byte64, file: "dm_21.8c", offset: 0x200005, size: 0x10000, crc: 0x2F1345B4 },
            Op::Load { kind: Load::Byte64, file: "dm_26.10e", offset: 0x200006, size: 0x10000, crc: 0x3C2A212A },
            Op::Load { kind: Load::Byte64, file: "dm_25.10c", offset: 0x200007, size: 0x10000, crc: 0x889AAC05 },
            Op::Load { kind: Load::Byte64, file: "dm_08.8b", offset: 0x280000, size: 0x10000, crc: 0xBCC0F28C },
            Op::Load { kind: Load::Byte64, file: "dm_07.8a", offset: 0x280001, size: 0x10000, crc: 0x2A40166A },
            Op::Load { kind: Load::Byte64, file: "dm_16.12b", offset: 0x280002, size: 0x10000, crc: 0x20F85C03 },
            Op::Load { kind: Load::Byte64, file: "dm_15.12a", offset: 0x280003, size: 0x10000, crc: 0x8426144B },
            Op::Load { kind: Load::Byte64, file: "dm_24.9e", offset: 0x280004, size: 0x10000, crc: 0xDA088D61 },
            Op::Load { kind: Load::Byte64, file: "dm_23.9c", offset: 0x280005, size: 0x10000, crc: 0x17E11DF0 },
            Op::Load { kind: Load::Byte64, file: "dm_28.11e", offset: 0x280006, size: 0x10000, crc: 0xF187BA1C },
            Op::Load { kind: Load::Byte64, file: "dm_27.11c", offset: 0x280007, size: 0x10000, crc: 0x29F79C78 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "37.13c", offset: 0x0, size: 0x8000, crc: 0x3692F6E5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0xff, ops: &[
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_DAIMAKAIR: RomSetDef = RomSetDef { name: "daimakair", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "damj_23.8f", offset: 0x0, size: 0x80000, crc: 0xC3B248EC },
            Op::Load { kind: Load::Word16Swap, file: "damj_22.7f", offset: 0x80000, size: 0x80000, crc: 0x595FF2F3 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "dam_01.3a", offset: 0x0, size: 0x80000, crc: 0x0BA9C0B0 },
            Op::Load { kind: Load::Word64, file: "dam_02.4a", offset: 0x2, size: 0x80000, crc: 0x5D760AB9 },
            Op::Load { kind: Load::Word64, file: "dam_03.5a", offset: 0x4, size: 0x80000, crc: 0x4BA90B59 },
            Op::Load { kind: Load::Word64, file: "dam_04.6a", offset: 0x6, size: 0x80000, crc: 0x4BDEE9DE },
            Op::Load { kind: Load::Word64, file: "dam_05.7a", offset: 0x200000, size: 0x80000, crc: 0x7DC61B94 },
            Op::Load { kind: Load::Word64, file: "dam_06.8a", offset: 0x200002, size: 0x80000, crc: 0xFDE89758 },
            Op::Load { kind: Load::Word64, file: "dam_07.9a", offset: 0x200004, size: 0x80000, crc: 0xEC351D78 },
            Op::Load { kind: Load::Word64, file: "dam_08.10a", offset: 0x200006, size: 0x80000, crc: 0xEE2ACC1E },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "dam_09.12a", offset: 0x0, size: 0x8000, crc: 0x0656FF53 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0xff, ops: &[
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_STRIDER: RomSetDef = RomSetDef { name: "strider", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "30.11f", offset: 0x0, size: 0x20000, crc: 0xDA997474 },
            Op::Load { kind: Load::Byte16, file: "35.11h", offset: 0x1, size: 0x20000, crc: 0x5463AAA3 },
            Op::Load { kind: Load::Byte16, file: "31.12f", offset: 0x40000, size: 0x20000, crc: 0xD20786DB },
            Op::Load { kind: Load::Byte16, file: "36.12h", offset: 0x40001, size: 0x20000, crc: 0x21AA2863 },
            Op::Load { kind: Load::Word16Swap, file: "st-14.8h", offset: 0x80000, size: 0x80000, crc: 0x9B3CFC08 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "st-2.8a", offset: 0x0, size: 0x80000, crc: 0x4EEE9AEA },
            Op::Load { kind: Load::Word64, file: "st-11.10a", offset: 0x2, size: 0x80000, crc: 0x2D7F21E4 },
            Op::Load { kind: Load::Word64, file: "st-5.4a", offset: 0x4, size: 0x80000, crc: 0x7705AA46 },
            Op::Load { kind: Load::Word64, file: "st-9.6a", offset: 0x6, size: 0x80000, crc: 0x5B18B722 },
            Op::Load { kind: Load::Word64, file: "st-1.7a", offset: 0x200000, size: 0x80000, crc: 0x005F000B },
            Op::Load { kind: Load::Word64, file: "st-10.9a", offset: 0x200002, size: 0x80000, crc: 0xB9441519 },
            Op::Load { kind: Load::Word64, file: "st-4.3a", offset: 0x200004, size: 0x80000, crc: 0xB7D04E8B },
            Op::Load { kind: Load::Word64, file: "st-8.5a", offset: 0x200006, size: 0x80000, crc: 0x6B4713B4 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "09.12b", offset: 0x0, size: 0x8000, crc: 0x2ED403BC },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "18.11c", offset: 0x0, size: 0x20000, crc: 0x4386BC80 },
            Op::Load { kind: Load::Byte, file: "19.12c", offset: 0x20000, size: 0x20000, crc: 0x444536D7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_STRIDERUA: RomSetDef = RomSetDef { name: "striderua", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "30.11f", offset: 0x0, size: 0x20000, crc: 0x66AEC273 },
            Op::Load { kind: Load::Byte16, file: "35.11h", offset: 0x1, size: 0x20000, crc: 0x50E0E865 },
            Op::Load { kind: Load::Byte16, file: "31.12f", offset: 0x40000, size: 0x20000, crc: 0xEAE93BD1 },
            Op::Load { kind: Load::Byte16, file: "36.12h", offset: 0x40001, size: 0x20000, crc: 0xB904A31D },
            Op::Load { kind: Load::Word16Swap, file: "st-14.8h", offset: 0x80000, size: 0x80000, crc: 0x9B3CFC08 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "st-2.8a", offset: 0x0, size: 0x80000, crc: 0x4EEE9AEA },
            Op::Load { kind: Load::Word64, file: "st-11.10a", offset: 0x2, size: 0x80000, crc: 0x2D7F21E4 },
            Op::Load { kind: Load::Word64, file: "st-5.4a", offset: 0x4, size: 0x80000, crc: 0x7705AA46 },
            Op::Load { kind: Load::Word64, file: "st-9.6a", offset: 0x6, size: 0x80000, crc: 0x5B18B722 },
            Op::Load { kind: Load::Word64, file: "st-1.7a", offset: 0x200000, size: 0x80000, crc: 0x005F000B },
            Op::Load { kind: Load::Word64, file: "st-10.9a", offset: 0x200002, size: 0x80000, crc: 0xB9441519 },
            Op::Load { kind: Load::Word64, file: "st-4.3a", offset: 0x200004, size: 0x80000, crc: 0xB7D04E8B },
            Op::Load { kind: Load::Word64, file: "st-8.5a", offset: 0x200006, size: 0x80000, crc: 0x6B4713B4 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "09.12b", offset: 0x0, size: 0x8000, crc: 0x08D63519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "18.11c", offset: 0x0, size: 0x20000, crc: 0x4386BC80 },
            Op::Load { kind: Load::Byte, file: "19.12c", offset: 0x20000, size: 0x20000, crc: 0x444536D7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_STRIDERUC: RomSetDef = RomSetDef { name: "strideruc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "34.8f", offset: 0x0, size: 0x80000, crc: 0xE0FB5657 },
            Op::Load { kind: Load::Word16Swap, file: "33.6f", offset: 0x80000, size: 0x80000, crc: 0x9B3CFC08 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "6.8a", offset: 0x0, size: 0x80000, crc: 0x4EEE9AEA },
            Op::Load { kind: Load::Word64, file: "8.10a", offset: 0x2, size: 0x80000, crc: 0x2D7F21E4 },
            Op::Load { kind: Load::Word64, file: "5.7a", offset: 0x4, size: 0x80000, crc: 0x7705AA46 },
            Op::Load { kind: Load::Word64, file: "7.9a", offset: 0x6, size: 0x80000, crc: 0x5B18B722 },
            Op::Load { kind: Load::Word64, file: "15.8c", offset: 0x200000, size: 0x80000, crc: 0x005F000B },
            Op::Load { kind: Load::Word64, file: "17.10c", offset: 0x200002, size: 0x80000, crc: 0xB9441519 },
            Op::Load { kind: Load::Word64, file: "14.7c", offset: 0x200004, size: 0x80000, crc: 0xB7D04E8B },
            Op::Load { kind: Load::Word64, file: "16.9c", offset: 0x200006, size: 0x80000, crc: 0x6B4713B4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "9.12a", offset: 0x0, size: 0x8000, crc: 0x08D63519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "18.11c", offset: 0x0, size: 0x20000, crc: 0x4386BC80 },
            Op::Load { kind: Load::Byte, file: "19.12c", offset: 0x20000, size: 0x20000, crc: 0x444536D7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_STRIDERJ: RomSetDef = RomSetDef { name: "striderj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sth_36.12f", offset: 0x0, size: 0x20000, crc: 0x53C7B006 },
            Op::Load { kind: Load::Byte16, file: "sth_42.12h", offset: 0x1, size: 0x20000, crc: 0x4037F65F },
            Op::Load { kind: Load::Byte16, file: "sth_37.13f", offset: 0x40000, size: 0x20000, crc: 0x80E8877D },
            Op::Load { kind: Load::Byte16, file: "sth_43.13h", offset: 0x40001, size: 0x20000, crc: 0x6B3FA466 },
            Op::Load { kind: Load::Byte16, file: "sth_34.10f", offset: 0x80000, size: 0x20000, crc: 0xBEA770B5 },
            Op::Load { kind: Load::Byte16, file: "sth_40.10h", offset: 0x80001, size: 0x20000, crc: 0x43B922DC },
            Op::Load { kind: Load::Byte16, file: "sth_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x5CC429DA },
            Op::Load { kind: Load::Byte16, file: "sth_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x50AF457F },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "sth_09.4b", offset: 0x0, size: 0x20000, crc: 0x1EF6BFBD },
            Op::Load { kind: Load::Byte64, file: "sth_01.4a", offset: 0x1, size: 0x20000, crc: 0x1E21B0C1 },
            Op::Load { kind: Load::Byte64, file: "sth_13.9b", offset: 0x2, size: 0x20000, crc: 0x063263AE },
            Op::Load { kind: Load::Byte64, file: "sth_05.9a", offset: 0x3, size: 0x20000, crc: 0xEC9F8714 },
            Op::Load { kind: Load::Byte64, file: "sth_24.5e", offset: 0x4, size: 0x20000, crc: 0x6356F4D2 },
            Op::Load { kind: Load::Byte64, file: "sth_17.5c", offset: 0x5, size: 0x20000, crc: 0xB4F73D86 },
            Op::Load { kind: Load::Byte64, file: "sth_38.8h", offset: 0x6, size: 0x20000, crc: 0xEE5ABFC2 },
            Op::Load { kind: Load::Byte64, file: "sth_32.8f", offset: 0x7, size: 0x20000, crc: 0x44A206A3 },
            Op::Load { kind: Load::Byte64, file: "sth_10.5b", offset: 0x100000, size: 0x20000, crc: 0xDF3DD3BC },
            Op::Load { kind: Load::Byte64, file: "sth_02.5a", offset: 0x100001, size: 0x20000, crc: 0xC75F9EA0 },
            Op::Load { kind: Load::Byte64, file: "sth_14.10b", offset: 0x100002, size: 0x20000, crc: 0x6C03E19D },
            Op::Load { kind: Load::Byte64, file: "sth_06.10a", offset: 0x100003, size: 0x20000, crc: 0xD84F5478 },
            Op::Load { kind: Load::Byte64, file: "sth_25.7e", offset: 0x100004, size: 0x20000, crc: 0x921E506A },
            Op::Load { kind: Load::Byte64, file: "sth_18.7c", offset: 0x100005, size: 0x20000, crc: 0x5B318956 },
            Op::Load { kind: Load::Byte64, file: "sth_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9321D6AA },
            Op::Load { kind: Load::Byte64, file: "sth_33.9f", offset: 0x100007, size: 0x20000, crc: 0xB47DDFC7 },
            Op::Load { kind: Load::Byte64, file: "sth_11.7b", offset: 0x200000, size: 0x20000, crc: 0x2484F241 },
            Op::Load { kind: Load::Byte64, file: "sth_03.7a", offset: 0x200001, size: 0x20000, crc: 0xAAA07245 },
            Op::Load { kind: Load::Byte64, file: "sth_15.11b", offset: 0x200002, size: 0x20000, crc: 0xE415D943 },
            Op::Load { kind: Load::Byte64, file: "sth_07.11a", offset: 0x200003, size: 0x20000, crc: 0x97D072D2 },
            Op::Load { kind: Load::Byte64, file: "sth_26.8e", offset: 0x200004, size: 0x20000, crc: 0x0EBFCB02 },
            Op::Load { kind: Load::Byte64, file: "sth_19.8c", offset: 0x200005, size: 0x20000, crc: 0x257CE683 },
            Op::Load { kind: Load::Byte64, file: "sth_28.10e", offset: 0x200006, size: 0x20000, crc: 0x98AC8CD1 },
            Op::Load { kind: Load::Byte64, file: "sth_21.10c", offset: 0x200007, size: 0x20000, crc: 0x538D9423 },
            Op::Load { kind: Load::Byte64, file: "sth_12.8b", offset: 0x300000, size: 0x20000, crc: 0xF670A477 },
            Op::Load { kind: Load::Byte64, file: "sth_04.8a", offset: 0x300001, size: 0x20000, crc: 0x853D3E01 },
            Op::Load { kind: Load::Byte64, file: "sth_16.12b", offset: 0x300002, size: 0x20000, crc: 0x4092019F },
            Op::Load { kind: Load::Byte64, file: "sth_08.12a", offset: 0x300003, size: 0x20000, crc: 0x2CE9B4C7 },
            Op::Load { kind: Load::Byte64, file: "sth_27.9e", offset: 0x300004, size: 0x20000, crc: 0xF82C88D9 },
            Op::Load { kind: Load::Byte64, file: "sth_20.9c", offset: 0x300005, size: 0x20000, crc: 0xEB584DD4 },
            Op::Load { kind: Load::Byte64, file: "sth_29.11e", offset: 0x300006, size: 0x20000, crc: 0x34AE2997 },
            Op::Load { kind: Load::Byte64, file: "sth_22.11c", offset: 0x300007, size: 0x20000, crc: 0x78DD9C48 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sth_23.13c", offset: 0x0, size: 0x8000, crc: 0x2ED403BC },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sth_30.12e", offset: 0x0, size: 0x20000, crc: 0x4386BC80 },
            Op::Load { kind: Load::Byte, file: "sth_31.13e", offset: 0x20000, size: 0x20000, crc: 0x444536D7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_STRIDERJR: RomSetDef = RomSetDef { name: "striderjr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sthj_23.8f", offset: 0x0, size: 0x80000, crc: 0x046E7B12 },
            Op::Load { kind: Load::Word16Swap, file: "sthj_22.7f", offset: 0x80000, size: 0x80000, crc: 0x9B3CFC08 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sth_01.3a", offset: 0x0, size: 0x80000, crc: 0x4EEE9AEA },
            Op::Load { kind: Load::Word64, file: "sth_02.4a", offset: 0x2, size: 0x80000, crc: 0x2D7F21E4 },
            Op::Load { kind: Load::Word64, file: "sth_03.5a", offset: 0x4, size: 0x80000, crc: 0x7705AA46 },
            Op::Load { kind: Load::Word64, file: "sth_04.6a", offset: 0x6, size: 0x80000, crc: 0x5B18B722 },
            Op::Load { kind: Load::Word64, file: "sth_05.7a", offset: 0x200000, size: 0x80000, crc: 0x005F000B },
            Op::Load { kind: Load::Word64, file: "sth_06.8a", offset: 0x200002, size: 0x80000, crc: 0xB9441519 },
            Op::Load { kind: Load::Word64, file: "sth_07.9a", offset: 0x200004, size: 0x80000, crc: 0xB7D04E8B },
            Op::Load { kind: Load::Word64, file: "sth_08.10a", offset: 0x200006, size: 0x80000, crc: 0x6B4713B4 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sth_09.12a", offset: 0x0, size: 0x8000, crc: 0x08D63519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sth_18.11c", offset: 0x0, size: 0x20000, crc: 0x4386BC80 },
            Op::Load { kind: Load::Byte, file: "sth_19.12c", offset: 0x20000, size: 0x20000, crc: 0x444536D7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_DYNWAR: RomSetDef = RomSetDef { name: "dynwar", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "30.11f", offset: 0x0, size: 0x20000, crc: 0xF9EC6D68 },
            Op::Load { kind: Load::Byte16, file: "35.11h", offset: 0x1, size: 0x20000, crc: 0xE41FFF2F },
            Op::Load { kind: Load::Byte16, file: "31.12f", offset: 0x40000, size: 0x20000, crc: 0xE3DE76FF },
            Op::Load { kind: Load::Byte16, file: "36.12h", offset: 0x40001, size: 0x20000, crc: 0x7A13CFBF },
            Op::Load { kind: Load::Word16Swap, file: "tkm-9.8h", offset: 0x80000, size: 0x80000, crc: 0x93654BCF },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tkm-5.7a", offset: 0x0, size: 0x80000, crc: 0xF64BB6A0 },
            Op::Load { kind: Load::Word64, file: "tkm-8.9a", offset: 0x2, size: 0x80000, crc: 0x21FE6274 },
            Op::Load { kind: Load::Word64, file: "tkm-6.3a", offset: 0x4, size: 0x80000, crc: 0x0BF228CB },
            Op::Load { kind: Load::Word64, file: "tkm-7.5a", offset: 0x6, size: 0x80000, crc: 0x1255DFB1 },
            Op::Load { kind: Load::Word64, file: "tkm-1.8a", offset: 0x200000, size: 0x80000, crc: 0x44F7661E },
            Op::Load { kind: Load::Word64, file: "tkm-4.10a", offset: 0x200002, size: 0x80000, crc: 0xA54C515D },
            Op::Load { kind: Load::Word64, file: "tkm-2.4a", offset: 0x200004, size: 0x80000, crc: 0xCA5C687C },
            Op::Load { kind: Load::Word64, file: "tkm-3.6a", offset: 0x200006, size: 0x80000, crc: 0xF9FE6591 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tke_17.12b", offset: 0x0, size: 0x8000, crc: 0xB3B79D4F },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tke_18.11c", offset: 0x0, size: 0x20000, crc: 0xAC6E307D },
            Op::Load { kind: Load::Byte, file: "tke_19.12c", offset: 0x20000, size: 0x20000, crc: 0x068741DB },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_DYNWARA: RomSetDef = RomSetDef { name: "dynwara", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "tke_36.12f", offset: 0x0, size: 0x20000, crc: 0x895991D1 },
            Op::Load { kind: Load::Byte16, file: "tke_42.12h", offset: 0x1, size: 0x20000, crc: 0xC898D2E8 },
            Op::Load { kind: Load::Byte16, file: "tke_37.13f", offset: 0x40000, size: 0x20000, crc: 0xB228D58C },
            Op::Load { kind: Load::Byte16, file: "tke_43.13h", offset: 0x40001, size: 0x20000, crc: 0x1A14375A },
            Op::Load { kind: Load::Byte16, file: "34.10f", offset: 0x80000, size: 0x20000, crc: 0x8F663D00 },
            Op::Load { kind: Load::Byte16, file: "40.10h", offset: 0x80001, size: 0x20000, crc: 0x1586DBF3 },
            Op::Load { kind: Load::Byte16, file: "35.11f", offset: 0xc0000, size: 0x20000, crc: 0x9DB93D7A },
            Op::Load { kind: Load::Byte16, file: "41.11h", offset: 0xc0001, size: 0x20000, crc: 0x1AAE69A4 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "09.4b", offset: 0x0, size: 0x20000, crc: 0xC3E83C69 },
            Op::Load { kind: Load::Byte64, file: "01.4a", offset: 0x1, size: 0x20000, crc: 0x187B2886 },
            Op::Load { kind: Load::Byte64, file: "13.9b", offset: 0x2, size: 0x20000, crc: 0x0273D87D },
            Op::Load { kind: Load::Byte64, file: "05.9a", offset: 0x3, size: 0x20000, crc: 0x339378B8 },
            Op::Load { kind: Load::Byte64, file: "24.5e", offset: 0x4, size: 0x20000, crc: 0xC6909B6F },
            Op::Load { kind: Load::Byte64, file: "17.5c", offset: 0x5, size: 0x20000, crc: 0x2E2F8320 },
            Op::Load { kind: Load::Byte64, file: "38.8h", offset: 0x6, size: 0x20000, crc: 0xCD7923ED },
            Op::Load { kind: Load::Byte64, file: "32.8f", offset: 0x7, size: 0x20000, crc: 0x21A0A453 },
            Op::Load { kind: Load::Byte64, file: "10.5b", offset: 0x100000, size: 0x20000, crc: 0xFF28F8D0 },
            Op::Load { kind: Load::Byte64, file: "02.5a", offset: 0x100001, size: 0x20000, crc: 0xCC83C02F },
            Op::Load { kind: Load::Byte64, file: "14.10b", offset: 0x100002, size: 0x20000, crc: 0x58D9B32F },
            Op::Load { kind: Load::Byte64, file: "06.10a", offset: 0x100003, size: 0x20000, crc: 0x6F9EDD75 },
            Op::Load { kind: Load::Byte64, file: "25.7e", offset: 0x100004, size: 0x20000, crc: 0x152EA74A },
            Op::Load { kind: Load::Byte64, file: "18.7c", offset: 0x100005, size: 0x20000, crc: 0x1833F932 },
            Op::Load { kind: Load::Byte64, file: "39.9h", offset: 0x100006, size: 0x20000, crc: 0xBC09B360 },
            Op::Load { kind: Load::Byte64, file: "33.9f", offset: 0x100007, size: 0x20000, crc: 0x89DE1533 },
            Op::Load { kind: Load::Byte64, file: "11.7b", offset: 0x200000, size: 0x20000, crc: 0x29EAF490 },
            Op::Load { kind: Load::Byte64, file: "03.7a", offset: 0x200001, size: 0x20000, crc: 0x7BF51337 },
            Op::Load { kind: Load::Byte64, file: "15.11b", offset: 0x200002, size: 0x20000, crc: 0xD36CDB91 },
            Op::Load { kind: Load::Byte64, file: "07.11a", offset: 0x200003, size: 0x20000, crc: 0xE04AF054 },
            Op::Load { kind: Load::Byte64, file: "26.8e", offset: 0x200004, size: 0x20000, crc: 0x07FC714B },
            Op::Load { kind: Load::Byte64, file: "19.8c", offset: 0x200005, size: 0x20000, crc: 0x7114E5C6 },
            Op::Load { kind: Load::Byte64, file: "28.10e", offset: 0x200006, size: 0x20000, crc: 0xAF62BF07 },
            Op::Load { kind: Load::Byte64, file: "21.10c", offset: 0x200007, size: 0x20000, crc: 0x523F462A },
            Op::Load { kind: Load::Byte64, file: "12.8b", offset: 0x300000, size: 0x20000, crc: 0x38652339 },
            Op::Load { kind: Load::Byte64, file: "04.8a", offset: 0x300001, size: 0x20000, crc: 0x4951BC0F },
            Op::Load { kind: Load::Byte64, file: "16.12b", offset: 0x300002, size: 0x20000, crc: 0x381608AE },
            Op::Load { kind: Load::Byte64, file: "08.12a", offset: 0x300003, size: 0x20000, crc: 0xB475D4E9 },
            Op::Load { kind: Load::Byte64, file: "27.9e", offset: 0x300004, size: 0x20000, crc: 0xA27E81FA },
            Op::Load { kind: Load::Byte64, file: "20.9c", offset: 0x300005, size: 0x20000, crc: 0x002796DC },
            Op::Load { kind: Load::Byte64, file: "29.11e", offset: 0x300006, size: 0x20000, crc: 0x6B41F82D },
            Op::Load { kind: Load::Byte64, file: "22.11c", offset: 0x300007, size: 0x20000, crc: 0x52145369 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "23.13c", offset: 0x0, size: 0x8000, crc: 0xB3B79D4F },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tke_30.12e", offset: 0x0, size: 0x20000, crc: 0xAC6E307D },
            Op::Load { kind: Load::Byte, file: "tke_31.13e", offset: 0x20000, size: 0x20000, crc: 0x068741DB },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_DYNWARJ: RomSetDef = RomSetDef { name: "dynwarj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "36.12f", offset: 0x0, size: 0x20000, crc: 0x1A516657 },
            Op::Load { kind: Load::Byte16, file: "42.12h", offset: 0x1, size: 0x20000, crc: 0x12A290A0 },
            Op::Load { kind: Load::Byte16, file: "37.13f", offset: 0x40000, size: 0x20000, crc: 0x932FC943 },
            Op::Load { kind: Load::Byte16, file: "43.13h", offset: 0x40001, size: 0x20000, crc: 0x872AD76D },
            Op::Load { kind: Load::Byte16, file: "34.10f", offset: 0x80000, size: 0x20000, crc: 0x8F663D00 },
            Op::Load { kind: Load::Byte16, file: "40.10h", offset: 0x80001, size: 0x20000, crc: 0x1586DBF3 },
            Op::Load { kind: Load::Byte16, file: "35.11f", offset: 0xc0000, size: 0x20000, crc: 0x9DB93D7A },
            Op::Load { kind: Load::Byte16, file: "41.11h", offset: 0xc0001, size: 0x20000, crc: 0x1AAE69A4 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "09.4b", offset: 0x0, size: 0x20000, crc: 0xC3E83C69 },
            Op::Load { kind: Load::Byte64, file: "01.4a", offset: 0x1, size: 0x20000, crc: 0x187B2886 },
            Op::Load { kind: Load::Byte64, file: "13.9b", offset: 0x2, size: 0x20000, crc: 0x0273D87D },
            Op::Load { kind: Load::Byte64, file: "05.9a", offset: 0x3, size: 0x20000, crc: 0x339378B8 },
            Op::Load { kind: Load::Byte64, file: "24.5e", offset: 0x4, size: 0x20000, crc: 0xC6909B6F },
            Op::Load { kind: Load::Byte64, file: "17.5c", offset: 0x5, size: 0x20000, crc: 0x2E2F8320 },
            Op::Load { kind: Load::Byte64, file: "38.8h", offset: 0x6, size: 0x20000, crc: 0xCD7923ED },
            Op::Load { kind: Load::Byte64, file: "32.8f", offset: 0x7, size: 0x20000, crc: 0x21A0A453 },
            Op::Load { kind: Load::Byte64, file: "10.5b", offset: 0x100000, size: 0x20000, crc: 0xFF28F8D0 },
            Op::Load { kind: Load::Byte64, file: "02.5a", offset: 0x100001, size: 0x20000, crc: 0xCC83C02F },
            Op::Load { kind: Load::Byte64, file: "14.10b", offset: 0x100002, size: 0x20000, crc: 0x18FB232C },
            Op::Load { kind: Load::Byte64, file: "06.10a", offset: 0x100003, size: 0x20000, crc: 0x6F9EDD75 },
            Op::Load { kind: Load::Byte64, file: "25.7e", offset: 0x100004, size: 0x20000, crc: 0x152EA74A },
            Op::Load { kind: Load::Byte64, file: "18.7c", offset: 0x100005, size: 0x20000, crc: 0x1833F932 },
            Op::Load { kind: Load::Byte64, file: "39.9h", offset: 0x100006, size: 0x20000, crc: 0xBC09B360 },
            Op::Load { kind: Load::Byte64, file: "33.9f", offset: 0x100007, size: 0x20000, crc: 0x89DE1533 },
            Op::Load { kind: Load::Byte64, file: "11.7b", offset: 0x200000, size: 0x20000, crc: 0x29EAF490 },
            Op::Load { kind: Load::Byte64, file: "03.7a", offset: 0x200001, size: 0x20000, crc: 0x7BF51337 },
            Op::Load { kind: Load::Byte64, file: "15.11b", offset: 0x200002, size: 0x20000, crc: 0xD36CDB91 },
            Op::Load { kind: Load::Byte64, file: "07.11a", offset: 0x200003, size: 0x20000, crc: 0xE04AF054 },
            Op::Load { kind: Load::Byte64, file: "26.8e", offset: 0x200004, size: 0x20000, crc: 0x07FC714B },
            Op::Load { kind: Load::Byte64, file: "19.8c", offset: 0x200005, size: 0x20000, crc: 0x7114E5C6 },
            Op::Load { kind: Load::Byte64, file: "28.10e", offset: 0x200006, size: 0x20000, crc: 0xAF62BF07 },
            Op::Load { kind: Load::Byte64, file: "21.10c", offset: 0x200007, size: 0x20000, crc: 0x523F462A },
            Op::Load { kind: Load::Byte64, file: "12.8b", offset: 0x300000, size: 0x20000, crc: 0x38652339 },
            Op::Load { kind: Load::Byte64, file: "04.8a", offset: 0x300001, size: 0x20000, crc: 0x4951BC0F },
            Op::Load { kind: Load::Byte64, file: "16.12b", offset: 0x300002, size: 0x20000, crc: 0x381608AE },
            Op::Load { kind: Load::Byte64, file: "08.12a", offset: 0x300003, size: 0x20000, crc: 0xB475D4E9 },
            Op::Load { kind: Load::Byte64, file: "27.9e", offset: 0x300004, size: 0x20000, crc: 0xA27E81FA },
            Op::Load { kind: Load::Byte64, file: "20.9c", offset: 0x300005, size: 0x20000, crc: 0x002796DC },
            Op::Load { kind: Load::Byte64, file: "29.11e", offset: 0x300006, size: 0x20000, crc: 0x6B41F82D },
            Op::Load { kind: Load::Byte64, file: "22.11c", offset: 0x300007, size: 0x20000, crc: 0x52145369 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "23.13c", offset: 0x0, size: 0x8000, crc: 0xB3B79D4F },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "30.12e", offset: 0x0, size: 0x20000, crc: 0x7E5F6CB4 },
            Op::Load { kind: Load::Byte, file: "31.13e", offset: 0x20000, size: 0x20000, crc: 0x4A30C737 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_DYNWARJR: RomSetDef = RomSetDef { name: "dynwarjr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk1j_23.8f", offset: 0x0, size: 0x80000, crc: 0x088A3009 },
            Op::Load { kind: Load::Word16Swap, file: "tk1j_22.7f", offset: 0x80000, size: 0x80000, crc: 0x93654BCF },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk1_01.3a", offset: 0x0, size: 0x80000, crc: 0xF64BB6A0 },
            Op::Load { kind: Load::Word64, file: "tk1_02.4a", offset: 0x2, size: 0x80000, crc: 0x21FE6274 },
            Op::Load { kind: Load::Word64, file: "tk1_03.5a", offset: 0x4, size: 0x80000, crc: 0x0BF228CB },
            Op::Load { kind: Load::Word64, file: "tk1_04.6a", offset: 0x6, size: 0x80000, crc: 0x1255DFB1 },
            Op::Load { kind: Load::Word64, file: "tk1_05.7a", offset: 0x200000, size: 0x80000, crc: 0x44F7661E },
            Op::Load { kind: Load::Word64, file: "tk1_06.8a", offset: 0x200002, size: 0x80000, crc: 0xA54C515D },
            Op::Load { kind: Load::Word64, file: "tk1_07.9a", offset: 0x200004, size: 0x80000, crc: 0xCA5C687C },
            Op::Load { kind: Load::Word64, file: "tk1_08.10a", offset: 0x200006, size: 0x80000, crc: 0xF9FE6591 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk1_09.12a", offset: 0x0, size: 0x8000, crc: 0xDB77D899 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk1_18.11c", offset: 0x0, size: 0x20000, crc: 0x7E5F6CB4 },
            Op::Load { kind: Load::Byte, file: "tk1_19.12c", offset: 0x20000, size: 0x20000, crc: 0x4A30C737 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_WILLOW: RomSetDef = RomSetDef { name: "willow", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "wle_30.11f", offset: 0x0, size: 0x20000, crc: 0x15372AA2 },
            Op::Load { kind: Load::Byte16, file: "wle_35.11h", offset: 0x1, size: 0x20000, crc: 0x2E64623B },
            Op::Load { kind: Load::Byte16, file: "wlu_31.12f", offset: 0x40000, size: 0x20000, crc: 0x0EB48A83 },
            Op::Load { kind: Load::Byte16, file: "wlu_36.12h", offset: 0x40001, size: 0x20000, crc: 0x36100209 },
            Op::Load { kind: Load::Word16Swap, file: "wlm-32.8h", offset: 0x80000, size: 0x80000, crc: 0xDFD9F643 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "wlm-7.7a", offset: 0x0, size: 0x80000, crc: 0xAFA74B73 },
            Op::Load { kind: Load::Word64, file: "wlm-5.9a", offset: 0x2, size: 0x80000, crc: 0x12A0DC0B },
            Op::Load { kind: Load::Word64, file: "wlm-3.3a", offset: 0x4, size: 0x80000, crc: 0xC6F2ABCE },
            Op::Load { kind: Load::Word64, file: "wlm-1.5a", offset: 0x6, size: 0x80000, crc: 0x4AA4C6D3 },
            Op::Load { kind: Load::Byte64, file: "wl_24.7d", offset: 0x200000, size: 0x20000, crc: 0x6F0ADEE5 },
            Op::Load { kind: Load::Byte64, file: "wl_14.7c", offset: 0x200001, size: 0x20000, crc: 0x9CF3027D },
            Op::Load { kind: Load::Byte64, file: "wl_26.9d", offset: 0x200002, size: 0x20000, crc: 0xF09C8ECF },
            Op::Load { kind: Load::Byte64, file: "wl_16.9c", offset: 0x200003, size: 0x20000, crc: 0xE35407AA },
            Op::Load { kind: Load::Byte64, file: "wl_20.3d", offset: 0x200004, size: 0x20000, crc: 0x84992350 },
            Op::Load { kind: Load::Byte64, file: "wl_10.3c", offset: 0x200005, size: 0x20000, crc: 0xB87B5A36 },
            Op::Load { kind: Load::Byte64, file: "wl_22.5d", offset: 0x200006, size: 0x20000, crc: 0xFD3F89F0 },
            Op::Load { kind: Load::Byte64, file: "wl_12.5c", offset: 0x200007, size: 0x20000, crc: 0x7DA49D69 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_09.12b", offset: 0x0, size: 0x8000, crc: 0xF6B3D060 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_18.11c", offset: 0x0, size: 0x20000, crc: 0xBDE23D4D },
            Op::Load { kind: Load::Byte, file: "wl_19.12c", offset: 0x20000, size: 0x20000, crc: 0x683898F5 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_WILLOWU: RomSetDef = RomSetDef { name: "willowu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "wlu_30.11f", offset: 0x0, size: 0x20000, crc: 0xD604DBB1 },
            Op::Load { kind: Load::Byte16, file: "35.11h", offset: 0x1, size: 0x20000, crc: 0x7A791E77 },
            Op::Load { kind: Load::Byte16, file: "wlu_31.12f", offset: 0x40000, size: 0x20000, crc: 0x0EB48A83 },
            Op::Load { kind: Load::Byte16, file: "wlu_36.12h", offset: 0x40001, size: 0x20000, crc: 0x36100209 },
            Op::Load { kind: Load::Word16Swap, file: "wlm-32.8h", offset: 0x80000, size: 0x80000, crc: 0xDFD9F643 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "wlm-7.7a", offset: 0x0, size: 0x80000, crc: 0xAFA74B73 },
            Op::Load { kind: Load::Word64, file: "wlm-5.9a", offset: 0x2, size: 0x80000, crc: 0x12A0DC0B },
            Op::Load { kind: Load::Word64, file: "wlm-3.3a", offset: 0x4, size: 0x80000, crc: 0xC6F2ABCE },
            Op::Load { kind: Load::Word64, file: "wlm-1.5a", offset: 0x6, size: 0x80000, crc: 0x4AA4C6D3 },
            Op::Load { kind: Load::Byte64, file: "wl_24.7d", offset: 0x200000, size: 0x20000, crc: 0x6F0ADEE5 },
            Op::Load { kind: Load::Byte64, file: "wl_14.7c", offset: 0x200001, size: 0x20000, crc: 0x9CF3027D },
            Op::Load { kind: Load::Byte64, file: "wl_26.9d", offset: 0x200002, size: 0x20000, crc: 0xF09C8ECF },
            Op::Load { kind: Load::Byte64, file: "wl_16.9c", offset: 0x200003, size: 0x20000, crc: 0xE35407AA },
            Op::Load { kind: Load::Byte64, file: "wl_20.3d", offset: 0x200004, size: 0x20000, crc: 0x84992350 },
            Op::Load { kind: Load::Byte64, file: "wl_10.3c", offset: 0x200005, size: 0x20000, crc: 0xB87B5A36 },
            Op::Load { kind: Load::Byte64, file: "wl_22.5d", offset: 0x200006, size: 0x20000, crc: 0xFD3F89F0 },
            Op::Load { kind: Load::Byte64, file: "wl_12.5c", offset: 0x200007, size: 0x20000, crc: 0x7DA49D69 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_09.12b", offset: 0x0, size: 0x8000, crc: 0xF6B3D060 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_18.11c", offset: 0x0, size: 0x20000, crc: 0xBDE23D4D },
            Op::Load { kind: Load::Byte, file: "wl_19.12c", offset: 0x20000, size: 0x20000, crc: 0x683898F5 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_WILLOWUO: RomSetDef = RomSetDef { name: "willowuo", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "wlu_30.11f", offset: 0x0, size: 0x20000, crc: 0xD604DBB1 },
            Op::Load { kind: Load::Byte16, file: "wlu_35.11h", offset: 0x1, size: 0x20000, crc: 0xDAEE72FE },
            Op::Load { kind: Load::Byte16, file: "wlu_31.12f", offset: 0x40000, size: 0x20000, crc: 0x0EB48A83 },
            Op::Load { kind: Load::Byte16, file: "wlu_36.12h", offset: 0x40001, size: 0x20000, crc: 0x36100209 },
            Op::Load { kind: Load::Word16Swap, file: "wlm-32.8h", offset: 0x80000, size: 0x80000, crc: 0xDFD9F643 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "wlm-7.7a", offset: 0x0, size: 0x80000, crc: 0xAFA74B73 },
            Op::Load { kind: Load::Word64, file: "wlm-5.9a", offset: 0x2, size: 0x80000, crc: 0x12A0DC0B },
            Op::Load { kind: Load::Word64, file: "wlm-3.3a", offset: 0x4, size: 0x80000, crc: 0xC6F2ABCE },
            Op::Load { kind: Load::Word64, file: "wlm-1.5a", offset: 0x6, size: 0x80000, crc: 0x4AA4C6D3 },
            Op::Load { kind: Load::Byte64, file: "wl_24.7d", offset: 0x200000, size: 0x20000, crc: 0x6F0ADEE5 },
            Op::Load { kind: Load::Byte64, file: "wl_14.7c", offset: 0x200001, size: 0x20000, crc: 0x9CF3027D },
            Op::Load { kind: Load::Byte64, file: "wl_26.9d", offset: 0x200002, size: 0x20000, crc: 0xF09C8ECF },
            Op::Load { kind: Load::Byte64, file: "wl_16.9c", offset: 0x200003, size: 0x20000, crc: 0xE35407AA },
            Op::Load { kind: Load::Byte64, file: "wl_20.3d", offset: 0x200004, size: 0x20000, crc: 0x84992350 },
            Op::Load { kind: Load::Byte64, file: "wl_10.3c", offset: 0x200005, size: 0x20000, crc: 0xB87B5A36 },
            Op::Load { kind: Load::Byte64, file: "wl_22.5d", offset: 0x200006, size: 0x20000, crc: 0xFD3F89F0 },
            Op::Load { kind: Load::Byte64, file: "wl_12.5c", offset: 0x200007, size: 0x20000, crc: 0x7DA49D69 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_09.12b", offset: 0x0, size: 0x8000, crc: 0xF6B3D060 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_18.11c", offset: 0x0, size: 0x20000, crc: 0xBDE23D4D },
            Op::Load { kind: Load::Byte, file: "wl_19.12c", offset: 0x20000, size: 0x20000, crc: 0x683898F5 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_WILLOWJ: RomSetDef = RomSetDef { name: "willowj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "wl_36.12f", offset: 0x0, size: 0x20000, crc: 0x2B0D7CBC },
            Op::Load { kind: Load::Byte16, file: "wl_42.12h", offset: 0x1, size: 0x20000, crc: 0x1AC39615 },
            Op::Load { kind: Load::Byte16, file: "wl_37.13f", offset: 0x40000, size: 0x20000, crc: 0x30A717FA },
            Op::Load { kind: Load::Byte16, file: "wl_43.13h", offset: 0x40001, size: 0x20000, crc: 0xD0DDDC9E },
            Op::Load { kind: Load::Byte16, file: "wl_34.10f", offset: 0x80000, size: 0x20000, crc: 0x23A84F7A },
            Op::Load { kind: Load::Byte16, file: "wl_40.10h", offset: 0x80001, size: 0x20000, crc: 0xC7A0ED21 },
            Op::Load { kind: Load::Byte16, file: "wl_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x5EFF7951 },
            Op::Load { kind: Load::Byte16, file: "wl_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x8D6477A3 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "wl_09.4b", offset: 0x0, size: 0x20000, crc: 0x05AA71B4 },
            Op::Load { kind: Load::Byte64, file: "wl_01.4a", offset: 0x1, size: 0x20000, crc: 0x08C2DF12 },
            Op::Load { kind: Load::Byte64, file: "wl_13.9b", offset: 0x2, size: 0x20000, crc: 0x1F7C87CD },
            Op::Load { kind: Load::Byte64, file: "wl_05.9a", offset: 0x3, size: 0x20000, crc: 0xF5254BF2 },
            Op::Load { kind: Load::Byte64, file: "wl_24.5e", offset: 0x4, size: 0x20000, crc: 0xD9D73BA1 },
            Op::Load { kind: Load::Byte64, file: "wl_17.5c", offset: 0x5, size: 0x20000, crc: 0xA652F30C },
            Op::Load { kind: Load::Byte64, file: "wl_38.8h", offset: 0x6, size: 0x20000, crc: 0xF6F9111B },
            Op::Load { kind: Load::Byte64, file: "wl_32.8f", offset: 0x7, size: 0x20000, crc: 0x10F64027 },
            Op::Load { kind: Load::Byte64, file: "wl_10.5b", offset: 0x100000, size: 0x20000, crc: 0xDBBA0A3F },
            Op::Load { kind: Load::Byte64, file: "wl_02.5a", offset: 0x100001, size: 0x20000, crc: 0x86FBA7A5 },
            Op::Load { kind: Load::Byte64, file: "wl_14.10b", offset: 0x100002, size: 0x20000, crc: 0x7D5798B2 },
            Op::Load { kind: Load::Byte64, file: "wl_06.10a", offset: 0x100003, size: 0x20000, crc: 0x1F052948 },
            Op::Load { kind: Load::Byte64, file: "wl_25.7e", offset: 0x100004, size: 0x20000, crc: 0x857D17D2 },
            Op::Load { kind: Load::Byte64, file: "wl_18.7c", offset: 0x100005, size: 0x20000, crc: 0x316C7FBC },
            Op::Load { kind: Load::Byte64, file: "wl_39.9h", offset: 0x100006, size: 0x20000, crc: 0xE6FCE9B0 },
            Op::Load { kind: Load::Byte64, file: "wl_33.9f", offset: 0x100007, size: 0x20000, crc: 0xA15D5517 },
            Op::Load { kind: Load::Byte64, file: "wl_11.7b", offset: 0x200000, size: 0x20000, crc: 0x6F0ADEE5 },
            Op::Load { kind: Load::Byte64, file: "wl_03.7a", offset: 0x200001, size: 0x20000, crc: 0x9CF3027D },
            Op::Load { kind: Load::Byte64, file: "wl_15.11b", offset: 0x200002, size: 0x20000, crc: 0xF09C8ECF },
            Op::Load { kind: Load::Byte64, file: "wl_07.11a", offset: 0x200003, size: 0x20000, crc: 0xE35407AA },
            Op::Load { kind: Load::Byte64, file: "wl_26.8e", offset: 0x200004, size: 0x20000, crc: 0x84992350 },
            Op::Load { kind: Load::Byte64, file: "wl_19.8c", offset: 0x200005, size: 0x20000, crc: 0xB87B5A36 },
            Op::Load { kind: Load::Byte64, file: "wl_28.10e", offset: 0x200006, size: 0x20000, crc: 0xFD3F89F0 },
            Op::Load { kind: Load::Byte64, file: "wl_21.10c", offset: 0x200007, size: 0x20000, crc: 0x7DA49D69 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_23.13c", offset: 0x0, size: 0x8000, crc: 0xF6B3D060 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "wl_30.12e", offset: 0x0, size: 0x20000, crc: 0xBDE23D4D },
            Op::Load { kind: Load::Byte, file: "wl_31.13e", offset: 0x20000, size: 0x20000, crc: 0x683898F5 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_UNSQUAD: RomSetDef = RomSetDef { name: "unsquad", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "aru_30.11f", offset: 0x0, size: 0x20000, crc: 0x24D8F88D },
            Op::Load { kind: Load::Byte16, file: "aru_35.11h", offset: 0x1, size: 0x20000, crc: 0x8B954B59 },
            Op::Load { kind: Load::Byte16, file: "aru_31.12f", offset: 0x40000, size: 0x20000, crc: 0x33E9694B },
            Op::Load { kind: Load::Byte16, file: "aru_36.12h", offset: 0x40001, size: 0x20000, crc: 0x7CC8FB9E },
            Op::Load { kind: Load::Word16Swap, file: "ar-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xAE1D7FB0 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ar-5m.7a", offset: 0x0, size: 0x80000, crc: 0xBF4575D8 },
            Op::Load { kind: Load::Word64, file: "ar-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA02945F4 },
            Op::Load { kind: Load::Word64, file: "ar-1m.3a", offset: 0x4, size: 0x80000, crc: 0x5965CA8D },
            Op::Load { kind: Load::Word64, file: "ar-3m.5a", offset: 0x6, size: 0x80000, crc: 0xAC6DB17D },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ar_09.12b", offset: 0x0, size: 0x8000, crc: 0xF3DD1367 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "aru_18.11c", offset: 0x0, size: 0x20000, crc: 0x584B43A9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_AREA88: RomSetDef = RomSetDef { name: "area88", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ar_36.12f", offset: 0x0, size: 0x20000, crc: 0x65030392 },
            Op::Load { kind: Load::Byte16, file: "ar_42.12h", offset: 0x1, size: 0x20000, crc: 0xC48170DE },
            Op::Load { kind: Load::Byte16, file: "ar_37.13f", offset: 0x40000, size: 0x20000, crc: 0x33E9694B },
            Op::Load { kind: Load::Byte16, file: "ar_43.13h", offset: 0x40001, size: 0x20000, crc: 0x7CC8FB9E },
            Op::Load { kind: Load::Byte16, file: "ar_34.10f", offset: 0x80000, size: 0x20000, crc: 0xF6E80386 },
            Op::Load { kind: Load::Byte16, file: "ar_40.10h", offset: 0x80001, size: 0x20000, crc: 0xBE36C145 },
            Op::Load { kind: Load::Byte16, file: "ar_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x86D98FF3 },
            Op::Load { kind: Load::Byte16, file: "ar_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x758893D3 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ar_09.4b", offset: 0x0, size: 0x20000, crc: 0xDB9376F8 },
            Op::Load { kind: Load::Byte64, file: "ar_01.4a", offset: 0x1, size: 0x20000, crc: 0x392151B4 },
            Op::Load { kind: Load::Byte64, file: "ar_13.9b", offset: 0x2, size: 0x20000, crc: 0x81436481 },
            Op::Load { kind: Load::Byte64, file: "ar_05.9a", offset: 0x3, size: 0x20000, crc: 0xE246ED9F },
            Op::Load { kind: Load::Byte64, file: "ar_24.5e", offset: 0x4, size: 0x20000, crc: 0x9CD6E2A3 },
            Op::Load { kind: Load::Byte64, file: "ar_17.5c", offset: 0x5, size: 0x20000, crc: 0x0B8E0DF4 },
            Op::Load { kind: Load::Byte64, file: "ar_38.8h", offset: 0x6, size: 0x20000, crc: 0x8B9E75B9 },
            Op::Load { kind: Load::Byte64, file: "ar_32.8f", offset: 0x7, size: 0x20000, crc: 0xDB6ACDCF },
            Op::Load { kind: Load::Byte64, file: "ar_10.5b", offset: 0x100000, size: 0x20000, crc: 0x4219B622 },
            Op::Load { kind: Load::Byte64, file: "ar_02.5a", offset: 0x100001, size: 0x20000, crc: 0xBAC5DEC5 },
            Op::Load { kind: Load::Byte64, file: "ar_14.10b", offset: 0x100002, size: 0x20000, crc: 0xE6BAE179 },
            Op::Load { kind: Load::Byte64, file: "ar_06.10a", offset: 0x100003, size: 0x20000, crc: 0xC8F04223 },
            Op::Load { kind: Load::Byte64, file: "ar_25.7e", offset: 0x100004, size: 0x20000, crc: 0x15CCF981 },
            Op::Load { kind: Load::Byte64, file: "ar_18.7c", offset: 0x100005, size: 0x20000, crc: 0x9336DB6A },
            Op::Load { kind: Load::Byte64, file: "ar_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9B8E1363 },
            Op::Load { kind: Load::Byte64, file: "ar_33.9f", offset: 0x100007, size: 0x20000, crc: 0x3968F4B5 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ar_23.13c", offset: 0x0, size: 0x8000, crc: 0xF3DD1367 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ar_30.12e", offset: 0x0, size: 0x20000, crc: 0x584B43A9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_AREA88R: RomSetDef = RomSetDef { name: "area88r", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "araj_23.8f", offset: 0x0, size: 0x80000, crc: 0x7045D6CB },
            Op::Load { kind: Load::Word16Swap, file: "araj_22.7f", offset: 0x80000, size: 0x80000, crc: 0x9913002E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ara_01.3a", offset: 0x0, size: 0x80000, crc: 0xBF4575D8 },
            Op::Load { kind: Load::Word64, file: "ara_02.4a", offset: 0x2, size: 0x80000, crc: 0xA02945F4 },
            Op::Load { kind: Load::Word64, file: "ara_03.5a", offset: 0x4, size: 0x80000, crc: 0x5965CA8D },
            Op::Load { kind: Load::Word64, file: "ara_04.6a", offset: 0x6, size: 0x80000, crc: 0xAC6DB17D },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ara_09.12a", offset: 0x0, size: 0x8000, crc: 0xAF88359C },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ara_18.11c", offset: 0x0, size: 0x20000, crc: 0x584B43A9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHT: RomSetDef = RomSetDef { name: "ffight", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ff_36.11f", offset: 0x0, size: 0x20000, crc: 0xF9A5CE83 },
            Op::Load { kind: Load::Byte16, file: "ff_42.11h", offset: 0x1, size: 0x20000, crc: 0x65F11215 },
            Op::Load { kind: Load::Byte16, file: "ff_37.12f", offset: 0x40000, size: 0x20000, crc: 0xE1033784 },
            Op::Load { kind: Load::Byte16, file: "ffe_43.12h", offset: 0x40001, size: 0x20000, crc: 0x995E968A },
            Op::Load { kind: Load::Word16Swap, file: "ff-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xC747696E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_09.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTA: RomSetDef = RomSetDef { name: "ffighta", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffe_30.11f", offset: 0x0, size: 0x20000, crc: 0x2347BF51 },
            Op::Load { kind: Load::Byte16, file: "ffe_35.11h", offset: 0x1, size: 0x20000, crc: 0x5F694ECC },
            Op::Load { kind: Load::Byte16, file: "ffe_31.12f", offset: 0x40000, size: 0x20000, crc: 0x6DC6B792 },
            Op::Load { kind: Load::Byte16, file: "ffe_36.12h", offset: 0x40001, size: 0x20000, crc: 0xB36A0B99 },
            Op::Load { kind: Load::Word16Swap, file: "ff-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xC747696E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ffe_23.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTU: RomSetDef = RomSetDef { name: "ffightu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ff_36.11f", offset: 0x0, size: 0x20000, crc: 0xF9A5CE83 },
            Op::Load { kind: Load::Byte16, file: "ff_42.11h", offset: 0x1, size: 0x20000, crc: 0x65F11215 },
            Op::Load { kind: Load::Byte16, file: "ff_37.12f", offset: 0x40000, size: 0x20000, crc: 0xE1033784 },
            Op::Load { kind: Load::Byte16, file: "ffu_43.12h", offset: 0x40001, size: 0x20000, crc: 0x4CA65947 },
            Op::Load { kind: Load::Word16Swap, file: "ff-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xC747696E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_09.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTU1: RomSetDef = RomSetDef { name: "ffightu1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ff_36.11f", offset: 0x0, size: 0x20000, crc: 0xF9A5CE83 },
            Op::Load { kind: Load::Byte16, file: "ff_42.11h", offset: 0x1, size: 0x20000, crc: 0x65F11215 },
            Op::Load { kind: Load::Byte16, file: "ff_37.12f", offset: 0x40000, size: 0x20000, crc: 0xE1033784 },
            Op::Load { kind: Load::Byte16, file: "ffu_43.12h", offset: 0x40001, size: 0x20000, crc: 0x4CA65947 },
            Op::Load { kind: Load::Byte16, file: "ff_34.9f", offset: 0x80000, size: 0x20000, crc: 0x0C8DC3FC },
            Op::Load { kind: Load::Byte16, file: "ff_40.9h", offset: 0x80001, size: 0x20000, crc: 0x8075BAB9 },
            Op::Load { kind: Load::Byte16, file: "ff_35.10f", offset: 0xc0000, size: 0x20000, crc: 0x4A934121 },
            Op::Load { kind: Load::Byte16, file: "ff_41.10h", offset: 0xc0001, size: 0x20000, crc: 0x2AF68154 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_09.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTUA: RomSetDef = RomSetDef { name: "ffightua", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffu_36.11f", offset: 0x0, size: 0x20000, crc: 0xE2A48AF9 },
            Op::Load { kind: Load::Byte16, file: "ffu_42.11h", offset: 0x1, size: 0x20000, crc: 0xF4BB480E },
            Op::Load { kind: Load::Byte16, file: "ffu_37.12f", offset: 0x40000, size: 0x20000, crc: 0xC371C667 },
            Op::Load { kind: Load::Byte16, file: "ffu_43.12h", offset: 0x40001, size: 0x20000, crc: 0x2F5771F9 },
            Op::Load { kind: Load::Word16Swap, file: "ff-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xC747696E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_09.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTUB: RomSetDef = RomSetDef { name: "ffightub", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffu_30_3.11f", offset: 0x0, size: 0x20000, crc: 0xE619EB30 },
            Op::Load { kind: Load::Byte16, file: "ffu_35_3.11h", offset: 0x1, size: 0x20000, crc: 0xBCA85263 },
            Op::Load { kind: Load::Byte16, file: "ffu_31_3.12f", offset: 0x40000, size: 0x20000, crc: 0x59ABD207 },
            Op::Load { kind: Load::Byte16, file: "ffu_36_3.12h", offset: 0x40001, size: 0x20000, crc: 0xDF46ECE8 },
            Op::Load { kind: Load::Word16Swap, file: "ff-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xC747696E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_09.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTUC: RomSetDef = RomSetDef { name: "ffightuc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffu_30.11f", offset: 0x0, size: 0x20000, crc: 0xED988977 },
            Op::Load { kind: Load::Byte16, file: "ffu_35.11h", offset: 0x1, size: 0x20000, crc: 0x07BF1C21 },
            Op::Load { kind: Load::Byte16, file: "ffu_31.12f", offset: 0x40000, size: 0x20000, crc: 0xDBA5A476 },
            Op::Load { kind: Load::Byte16, file: "ffu_36.12h", offset: 0x40001, size: 0x20000, crc: 0x4D89F542 },
            Op::Load { kind: Load::Word16Swap, file: "ff-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xC747696E },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x9C284108 },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0xA7584DFB },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B605E44 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x52291CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_23.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTJ: RomSetDef = RomSetDef { name: "ffightj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ff36.bin", offset: 0x0, size: 0x20000, crc: 0xF9A5CE83 },
            Op::Load { kind: Load::Byte16, file: "ff42.bin", offset: 0x1, size: 0x20000, crc: 0x65F11215 },
            Op::Load { kind: Load::Byte16, file: "ff37.bin", offset: 0x40000, size: 0x20000, crc: 0xE1033784 },
            Op::Load { kind: Load::Byte16, file: "ff43.bin", offset: 0x40001, size: 0x20000, crc: 0xB6DEE1C3 },
            Op::Load { kind: Load::Byte16, file: "ffj_34.10f", offset: 0x80000, size: 0x20000, crc: 0x0C8DC3FC },
            Op::Load { kind: Load::Byte16, file: "ffj_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8075BAB9 },
            Op::Load { kind: Load::Byte16, file: "ffj_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x4A934121 },
            Op::Load { kind: Load::Byte16, file: "ffj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x2AF68154 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ffj_09.4b", offset: 0x0, size: 0x20000, crc: 0x5B116D0D },
            Op::Load { kind: Load::Byte64, file: "ffj_01.4a", offset: 0x1, size: 0x20000, crc: 0x815B1797 },
            Op::Load { kind: Load::Byte64, file: "ffj_13.9b", offset: 0x2, size: 0x20000, crc: 0x8721A7DA },
            Op::Load { kind: Load::Byte64, file: "ffj_05.9a", offset: 0x3, size: 0x20000, crc: 0xD0FCD4B5 },
            Op::Load { kind: Load::Byte64, file: "ffj_24.5e", offset: 0x4, size: 0x20000, crc: 0xA1AB607A },
            Op::Load { kind: Load::Byte64, file: "ffj_17.5c", offset: 0x5, size: 0x20000, crc: 0x2DC18CF4 },
            Op::Load { kind: Load::Byte64, file: "ffj_38.8h", offset: 0x6, size: 0x20000, crc: 0x6535A57F },
            Op::Load { kind: Load::Byte64, file: "ffj_32.8f", offset: 0x7, size: 0x20000, crc: 0xC8BC4A57 },
            Op::Load { kind: Load::Byte64, file: "ffj_10.5b", offset: 0x100000, size: 0x20000, crc: 0x624A924A },
            Op::Load { kind: Load::Byte64, file: "ffj_02.5a", offset: 0x100001, size: 0x20000, crc: 0x5D91F694 },
            Op::Load { kind: Load::Byte64, file: "ffj_14.10b", offset: 0x100002, size: 0x20000, crc: 0x0A2E9101 },
            Op::Load { kind: Load::Byte64, file: "ffj_06.10a", offset: 0x100003, size: 0x20000, crc: 0x1C18F042 },
            Op::Load { kind: Load::Byte64, file: "ffj_25.7e", offset: 0x100004, size: 0x20000, crc: 0x6E8181EA },
            Op::Load { kind: Load::Byte64, file: "ffj_18.7c", offset: 0x100005, size: 0x20000, crc: 0xB19EDE59 },
            Op::Load { kind: Load::Byte64, file: "ffj_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9416B477 },
            Op::Load { kind: Load::Byte64, file: "ffj_33.9f", offset: 0x100007, size: 0x20000, crc: 0x7369FA07 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_23.bin", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ffj_30.bin", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ffj_31.bin", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTJ1: RomSetDef = RomSetDef { name: "ffightj1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffj_36.12f", offset: 0x0, size: 0x20000, crc: 0xE2A48AF9 },
            Op::Load { kind: Load::Byte16, file: "ffj_42.12h", offset: 0x1, size: 0x20000, crc: 0xF4BB480E },
            Op::Load { kind: Load::Byte16, file: "ffj_37.13f", offset: 0x40000, size: 0x20000, crc: 0xC371C667 },
            Op::Load { kind: Load::Byte16, file: "ffj_43.13h", offset: 0x40001, size: 0x20000, crc: 0x6F81F194 },
            Op::Load { kind: Load::Byte16, file: "ffj_34.10f", offset: 0x80000, size: 0x20000, crc: 0x0C8DC3FC },
            Op::Load { kind: Load::Byte16, file: "ffj_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8075BAB9 },
            Op::Load { kind: Load::Byte16, file: "ffj_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x4A934121 },
            Op::Load { kind: Load::Byte16, file: "ffj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x2AF68154 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ffj_09.4b", offset: 0x0, size: 0x20000, crc: 0x5B116D0D },
            Op::Load { kind: Load::Byte64, file: "ffj_01.4a", offset: 0x1, size: 0x20000, crc: 0x815B1797 },
            Op::Load { kind: Load::Byte64, file: "ffj_13.9b", offset: 0x2, size: 0x20000, crc: 0x8721A7DA },
            Op::Load { kind: Load::Byte64, file: "ffj_05.9a", offset: 0x3, size: 0x20000, crc: 0xD0FCD4B5 },
            Op::Load { kind: Load::Byte64, file: "ffj_24.5e", offset: 0x4, size: 0x20000, crc: 0xA1AB607A },
            Op::Load { kind: Load::Byte64, file: "ffj_17.5c", offset: 0x5, size: 0x20000, crc: 0x2DC18CF4 },
            Op::Load { kind: Load::Byte64, file: "ffj_38.8h", offset: 0x6, size: 0x20000, crc: 0x6535A57F },
            Op::Load { kind: Load::Byte64, file: "ffj_32.8f", offset: 0x7, size: 0x20000, crc: 0xC8BC4A57 },
            Op::Load { kind: Load::Byte64, file: "ffj_10.5b", offset: 0x100000, size: 0x20000, crc: 0x624A924A },
            Op::Load { kind: Load::Byte64, file: "ffj_02.5a", offset: 0x100001, size: 0x20000, crc: 0x5D91F694 },
            Op::Load { kind: Load::Byte64, file: "ffj_14.10b", offset: 0x100002, size: 0x20000, crc: 0x0A2E9101 },
            Op::Load { kind: Load::Byte64, file: "ffj_06.10a", offset: 0x100003, size: 0x20000, crc: 0x1C18F042 },
            Op::Load { kind: Load::Byte64, file: "ffj_25.7e", offset: 0x100004, size: 0x20000, crc: 0x6E8181EA },
            Op::Load { kind: Load::Byte64, file: "ffj_18.7c", offset: 0x100005, size: 0x20000, crc: 0xB19EDE59 },
            Op::Load { kind: Load::Byte64, file: "ffj_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9416B477 },
            Op::Load { kind: Load::Byte64, file: "ffj_33.9f", offset: 0x100007, size: 0x20000, crc: 0x7369FA07 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_23.13b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ffj_30.12c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ffj_31.13c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTJ2: RomSetDef = RomSetDef { name: "ffightj2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffj_36a.12f", offset: 0x0, size: 0x20000, crc: 0x088ED1C9 },
            Op::Load { kind: Load::Byte16, file: "ffj_42a.12h", offset: 0x1, size: 0x20000, crc: 0xC4C491E6 },
            Op::Load { kind: Load::Byte16, file: "ffj_37a.13f", offset: 0x40000, size: 0x20000, crc: 0x708557FF },
            Op::Load { kind: Load::Byte16, file: "ffj_43a.13h", offset: 0x40001, size: 0x20000, crc: 0xC004004A },
            Op::Load { kind: Load::Byte16, file: "ff_34.10f", offset: 0x80000, size: 0x20000, crc: 0x0C8DC3FC },
            Op::Load { kind: Load::Byte16, file: "ff_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8075BAB9 },
            Op::Load { kind: Load::Byte16, file: "ff_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x4A934121 },
            Op::Load { kind: Load::Byte16, file: "ff_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x2AF68154 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ff_09.4b", offset: 0x0, size: 0x20000, crc: 0x5B116D0D },
            Op::Load { kind: Load::Byte64, file: "ff_01.4a", offset: 0x1, size: 0x20000, crc: 0x815B1797 },
            Op::Load { kind: Load::Byte64, file: "ff_13.9b", offset: 0x2, size: 0x20000, crc: 0x8721A7DA },
            Op::Load { kind: Load::Byte64, file: "ff_05.9a", offset: 0x3, size: 0x20000, crc: 0xD0FCD4B5 },
            Op::Load { kind: Load::Byte64, file: "ff_24.5e", offset: 0x4, size: 0x20000, crc: 0xA1AB607A },
            Op::Load { kind: Load::Byte64, file: "ff_17.5c", offset: 0x5, size: 0x20000, crc: 0x2DC18CF4 },
            Op::Load { kind: Load::Byte64, file: "ff_38.8h", offset: 0x6, size: 0x20000, crc: 0x6535A57F },
            Op::Load { kind: Load::Byte64, file: "ff_32.8f", offset: 0x7, size: 0x20000, crc: 0xC8BC4A57 },
            Op::Load { kind: Load::Byte64, file: "ff_10.5b", offset: 0x100000, size: 0x20000, crc: 0x624A924A },
            Op::Load { kind: Load::Byte64, file: "ff_02.5a", offset: 0x100001, size: 0x20000, crc: 0x5D91F694 },
            Op::Load { kind: Load::Byte64, file: "ff_14.10b", offset: 0x100002, size: 0x20000, crc: 0x0A2E9101 },
            Op::Load { kind: Load::Byte64, file: "ff_06.10a", offset: 0x100003, size: 0x20000, crc: 0x1C18F042 },
            Op::Load { kind: Load::Byte64, file: "ff_25.7e", offset: 0x100004, size: 0x20000, crc: 0x6E8181EA },
            Op::Load { kind: Load::Byte64, file: "ff_18.7c", offset: 0x100005, size: 0x20000, crc: 0xB19EDE59 },
            Op::Load { kind: Load::Byte64, file: "ff_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9416B477 },
            Op::Load { kind: Load::Byte64, file: "ff_33.9f", offset: 0x100007, size: 0x20000, crc: 0x7369FA07 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_23.13c", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_30.12e", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_31.13e", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTJ3: RomSetDef = RomSetDef { name: "ffightj3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ffj_36.12f", offset: 0x0, size: 0x20000, crc: 0xE619EB30 },
            Op::Load { kind: Load::Byte16, file: "ffj_42.12h", offset: 0x1, size: 0x20000, crc: 0xBCA85263 },
            Op::Load { kind: Load::Byte16, file: "ffj_37.13f", offset: 0x40000, size: 0x20000, crc: 0xA8127E4E },
            Op::Load { kind: Load::Byte16, file: "ffj_43.13h", offset: 0x40001, size: 0x20000, crc: 0x56CCD34A },
            Op::Load { kind: Load::Byte16, file: "ffj_34.10f", offset: 0x80000, size: 0x20000, crc: 0x0C8DC3FC },
            Op::Load { kind: Load::Byte16, file: "ffj_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8075BAB9 },
            Op::Load { kind: Load::Byte16, file: "ffj_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x4A934121 },
            Op::Load { kind: Load::Byte16, file: "ffj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x2AF68154 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ff_09.4b", offset: 0x0, size: 0x20000, crc: 0x5B116D0D },
            Op::Load { kind: Load::Byte64, file: "ff_01.4a", offset: 0x1, size: 0x20000, crc: 0x815B1797 },
            Op::Load { kind: Load::Byte64, file: "ff_13.9b", offset: 0x2, size: 0x20000, crc: 0x8721A7DA },
            Op::Load { kind: Load::Byte64, file: "ff_05.9a", offset: 0x3, size: 0x20000, crc: 0xD0FCD4B5 },
            Op::Load { kind: Load::Byte64, file: "ff_24.5e", offset: 0x4, size: 0x20000, crc: 0xA1AB607A },
            Op::Load { kind: Load::Byte64, file: "ff_17.5c", offset: 0x5, size: 0x20000, crc: 0x2DC18CF4 },
            Op::Load { kind: Load::Byte64, file: "ff_38.8h", offset: 0x6, size: 0x20000, crc: 0x6535A57F },
            Op::Load { kind: Load::Byte64, file: "ff_32.8f", offset: 0x7, size: 0x20000, crc: 0xC8BC4A57 },
            Op::Load { kind: Load::Byte64, file: "ff_10.5b", offset: 0x100000, size: 0x20000, crc: 0x624A924A },
            Op::Load { kind: Load::Byte64, file: "ff_02.5a", offset: 0x100001, size: 0x20000, crc: 0x5D91F694 },
            Op::Load { kind: Load::Byte64, file: "ff_14.10b", offset: 0x100002, size: 0x20000, crc: 0x0A2E9101 },
            Op::Load { kind: Load::Byte64, file: "ff_06.10a", offset: 0x100003, size: 0x20000, crc: 0x1C18F042 },
            Op::Load { kind: Load::Byte64, file: "ff_25.7e", offset: 0x100004, size: 0x20000, crc: 0x6E8181EA },
            Op::Load { kind: Load::Byte64, file: "ff_18.7c", offset: 0x100005, size: 0x20000, crc: 0xB19EDE59 },
            Op::Load { kind: Load::Byte64, file: "ff_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9416B477 },
            Op::Load { kind: Load::Byte64, file: "ff_33.9f", offset: 0x100007, size: 0x20000, crc: 0x7369FA07 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_23.13b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ffj_30.12e", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ffj_31.13e", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTJ4: RomSetDef = RomSetDef { name: "ffightj4", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ff_36.12f", offset: 0x0, size: 0x20000, crc: 0xED988977 },
            Op::Load { kind: Load::Byte16, file: "ffj_42.12h", offset: 0x1, size: 0x20000, crc: 0x07BF1C21 },
            Op::Load { kind: Load::Byte16, file: "ff_37.13f", offset: 0x40000, size: 0x20000, crc: 0xDBA5A476 },
            Op::Load { kind: Load::Byte16, file: "ffj_43.13h", offset: 0x40001, size: 0x20000, crc: 0xFBECA028 },
            Op::Load { kind: Load::Byte16, file: "ff_34.10f", offset: 0x80000, size: 0x20000, crc: 0x0C8DC3FC },
            Op::Load { kind: Load::Byte16, file: "ffj_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8075BAB9 },
            Op::Load { kind: Load::Byte16, file: "ff_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x4A934121 },
            Op::Load { kind: Load::Byte16, file: "ffj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x2AF68154 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ff_09.4b", offset: 0x0, size: 0x20000, crc: 0x5B116D0D },
            Op::Load { kind: Load::Byte64, file: "ff_01.4a", offset: 0x1, size: 0x20000, crc: 0x815B1797 },
            Op::Load { kind: Load::Byte64, file: "ff_13.9b", offset: 0x2, size: 0x20000, crc: 0x8721A7DA },
            Op::Load { kind: Load::Byte64, file: "ff_05.9a", offset: 0x3, size: 0x20000, crc: 0xD0FCD4B5 },
            Op::Load { kind: Load::Byte64, file: "ff_24.5e", offset: 0x4, size: 0x20000, crc: 0xA1AB607A },
            Op::Load { kind: Load::Byte64, file: "ff_17.5c", offset: 0x5, size: 0x20000, crc: 0x2DC18CF4 },
            Op::Load { kind: Load::Byte64, file: "ffj_38.8h", offset: 0x6, size: 0x20000, crc: 0x6535A57F },
            Op::Load { kind: Load::Byte64, file: "ff_32.8f", offset: 0x7, size: 0x20000, crc: 0xC8BC4A57 },
            Op::Load { kind: Load::Byte64, file: "ff_10.5b", offset: 0x100000, size: 0x20000, crc: 0x624A924A },
            Op::Load { kind: Load::Byte64, file: "ff_02.5a", offset: 0x100001, size: 0x20000, crc: 0x5D91F694 },
            Op::Load { kind: Load::Byte64, file: "ff_14.10b", offset: 0x100002, size: 0x20000, crc: 0x0A2E9101 },
            Op::Load { kind: Load::Byte64, file: "ff_06.10a", offset: 0x100003, size: 0x20000, crc: 0x1C18F042 },
            Op::Load { kind: Load::Byte64, file: "ff_25.7e", offset: 0x100004, size: 0x20000, crc: 0x6E8181EA },
            Op::Load { kind: Load::Byte64, file: "ff_18.7c", offset: 0x100005, size: 0x20000, crc: 0xB19EDE59 },
            Op::Load { kind: Load::Byte64, file: "ffj_39.9h", offset: 0x100006, size: 0x20000, crc: 0x9416B477 },
            Op::Load { kind: Load::Byte64, file: "ff_33.9f", offset: 0x100007, size: 0x20000, crc: 0x7369FA07 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_23.13b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_30.12c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_31.13c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_FFIGHTJH: RomSetDef = RomSetDef { name: "ffightjh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ff_23.8f", offset: 0x0, size: 0x80000, crc: 0xAE3DDA7F },
            Op::Load { kind: Load::Word16Swap, file: "ff_22.7f", offset: 0x80000, size: 0x80000, crc: 0xB2D5A3AA },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff_1.3a", offset: 0x0, size: 0x80000, crc: 0x969D18E2 },
            Op::Load { kind: Load::Word64, file: "ff_2.4a", offset: 0x2, size: 0x80000, crc: 0x02B59F99 },
            Op::Load { kind: Load::Word64, file: "ff_3.5a", offset: 0x4, size: 0x80000, crc: 0x01D507AE },
            Op::Load { kind: Load::Word64, file: "ff_4.6a", offset: 0x6, size: 0x80000, crc: 0xF7C4CEB0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_9.12a", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_FFIGHTAE: RomSetDef = RomSetDef { name: "ffightae", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ff-23m.8h", offset: 0x0, size: 0x80000, crc: 0x86DEF74F },
            Op::Load { kind: Load::Word16Swap, file: "ff-22m.7h", offset: 0x80000, size: 0x80000, crc: 0xCBDD8689 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ff-5m.7a", offset: 0x0, size: 0x80000, crc: 0x91A909BD },
            Op::Load { kind: Load::Word64, file: "ff-7m.9a", offset: 0x2, size: 0x80000, crc: 0x89F8B4CD },
            Op::Load { kind: Load::Word64, file: "ff-1m.3a", offset: 0x4, size: 0x80000, crc: 0xD5469303 },
            Op::Load { kind: Load::Word64, file: "ff-3m.5a", offset: 0x6, size: 0x80000, crc: 0x0C6302BF },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_09.12b", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ff_18.11c", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "ff_19.12c", offset: 0x20000, size: 0x20000, crc: 0x1EF137F9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_1941: RomSetDef = RomSetDef { name: "1941", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "41em_30.11f", offset: 0x0, size: 0x20000, crc: 0x4249EC61 },
            Op::Load { kind: Load::Byte16, file: "41em_35.11h", offset: 0x1, size: 0x20000, crc: 0xDDBEE5EB },
            Op::Load { kind: Load::Byte16, file: "41em_31.12f", offset: 0x40000, size: 0x20000, crc: 0x584E88E5 },
            Op::Load { kind: Load::Byte16, file: "41em_36.12h", offset: 0x40001, size: 0x20000, crc: 0x3CFC31D0 },
            Op::Load { kind: Load::Word16Swap, file: "41-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x4E9648CA },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "41-5m.7a", offset: 0x0, size: 0x80000, crc: 0x01D1CB11 },
            Op::Load { kind: Load::Word64, file: "41-7m.9a", offset: 0x2, size: 0x80000, crc: 0xAEAA3509 },
            Op::Load { kind: Load::Word64, file: "41-1m.3a", offset: 0x4, size: 0x80000, crc: 0xFF77985A },
            Op::Load { kind: Load::Word64, file: "41-3m.5a", offset: 0x6, size: 0x80000, crc: 0x983BE58F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_9.12b", offset: 0x0, size: 0x8000, crc: 0x0F9D8527 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_18.11c", offset: 0x0, size: 0x20000, crc: 0xD1F15AEB },
            Op::Load { kind: Load::Byte, file: "41_19.12c", offset: 0x20000, size: 0x20000, crc: 0x15AEC3A6 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_1941R1: RomSetDef = RomSetDef { name: "1941r1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "41e_30.11f", offset: 0x0, size: 0x20000, crc: 0x9DEB1E75 },
            Op::Load { kind: Load::Byte16, file: "41e_35.11h", offset: 0x1, size: 0x20000, crc: 0xD63942B3 },
            Op::Load { kind: Load::Byte16, file: "41e_31.12f", offset: 0x40000, size: 0x20000, crc: 0xDF201112 },
            Op::Load { kind: Load::Byte16, file: "41e_36.12h", offset: 0x40001, size: 0x20000, crc: 0x816A818F },
            Op::Load { kind: Load::Word16Swap, file: "41-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x4E9648CA },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "41-5m.7a", offset: 0x0, size: 0x80000, crc: 0x01D1CB11 },
            Op::Load { kind: Load::Word64, file: "41-7m.9a", offset: 0x2, size: 0x80000, crc: 0xAEAA3509 },
            Op::Load { kind: Load::Word64, file: "41-1m.3a", offset: 0x4, size: 0x80000, crc: 0xFF77985A },
            Op::Load { kind: Load::Word64, file: "41-3m.5a", offset: 0x6, size: 0x80000, crc: 0x983BE58F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_9.12b", offset: 0x0, size: 0x8000, crc: 0x0F9D8527 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_18.11c", offset: 0x0, size: 0x20000, crc: 0xD1F15AEB },
            Op::Load { kind: Load::Byte, file: "41_19.12c", offset: 0x20000, size: 0x20000, crc: 0x15AEC3A6 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_1941U: RomSetDef = RomSetDef { name: "1941u", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "41u_30.11f", offset: 0x0, size: 0x20000, crc: 0xBE5439D0 },
            Op::Load { kind: Load::Byte16, file: "41u_35.11h", offset: 0x1, size: 0x20000, crc: 0x6AC96595 },
            Op::Load { kind: Load::Byte16, file: "41u_31.12f", offset: 0x40000, size: 0x20000, crc: 0x9811D6EB },
            Op::Load { kind: Load::Byte16, file: "41u_36.12h", offset: 0x40001, size: 0x20000, crc: 0xA87E6137 },
            Op::Load { kind: Load::Word16Swap, file: "41-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x4E9648CA },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "41-5m.7a", offset: 0x0, size: 0x80000, crc: 0x01D1CB11 },
            Op::Load { kind: Load::Word64, file: "41-7m.9a", offset: 0x2, size: 0x80000, crc: 0xAEAA3509 },
            Op::Load { kind: Load::Word64, file: "41-1m.3a", offset: 0x4, size: 0x80000, crc: 0xFF77985A },
            Op::Load { kind: Load::Word64, file: "41-3m.5a", offset: 0x6, size: 0x80000, crc: 0x983BE58F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_9.12b", offset: 0x0, size: 0x8000, crc: 0x0F9D8527 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41e_18.11c", offset: 0x0, size: 0x20000, crc: 0xD1F15AEB },
            Op::Load { kind: Load::Byte, file: "41_19.12c", offset: 0x20000, size: 0x20000, crc: 0x15AEC3A6 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_1941J: RomSetDef = RomSetDef { name: "1941j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "41_36.12f", offset: 0x0, size: 0x20000, crc: 0x7FBD42AB },
            Op::Load { kind: Load::Byte16, file: "41_42.12h", offset: 0x1, size: 0x20000, crc: 0xC7781F89 },
            Op::Load { kind: Load::Byte16, file: "41_37.13f", offset: 0x40000, size: 0x20000, crc: 0xC6464B0B },
            Op::Load { kind: Load::Byte16, file: "41_43.13h", offset: 0x40001, size: 0x20000, crc: 0x440FC0B5 },
            Op::Load { kind: Load::Byte16, file: "41_34.10f", offset: 0x80000, size: 0x20000, crc: 0xB5F341EC },
            Op::Load { kind: Load::Byte16, file: "41_40.10h", offset: 0x80001, size: 0x20000, crc: 0x3979837D },
            Op::Load { kind: Load::Byte16, file: "41_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x95CC979A },
            Op::Load { kind: Load::Byte16, file: "41_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x57496819 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "41_09.4b", offset: 0x0, size: 0x20000, crc: 0xBE1B6BC2 },
            Op::Load { kind: Load::Byte64, file: "41_01.4a", offset: 0x1, size: 0x20000, crc: 0xD8946FC1 },
            Op::Load { kind: Load::Byte64, file: "41_13.9b", offset: 0x2, size: 0x20000, crc: 0x2E06D0EC },
            Op::Load { kind: Load::Byte64, file: "41_05.9a", offset: 0x3, size: 0x20000, crc: 0xD8BA28E0 },
            Op::Load { kind: Load::Byte64, file: "41_24.5e", offset: 0x4, size: 0x20000, crc: 0x5AA43CEE },
            Op::Load { kind: Load::Byte64, file: "41_17.5c", offset: 0x5, size: 0x20000, crc: 0xBBEFF902 },
            Op::Load { kind: Load::Byte64, file: "41_38.8h", offset: 0x6, size: 0x20000, crc: 0x8889C0AA },
            Op::Load { kind: Load::Byte64, file: "41_32.8f", offset: 0x7, size: 0x20000, crc: 0xF0168249 },
            Op::Load { kind: Load::Byte64, file: "41_10.5b", offset: 0x100000, size: 0x20000, crc: 0xB7EB6A6D },
            Op::Load { kind: Load::Byte64, file: "41_02.5a", offset: 0x100001, size: 0x20000, crc: 0x802E8153 },
            Op::Load { kind: Load::Byte64, file: "41_14.10b", offset: 0x100002, size: 0x20000, crc: 0x5A33F676 },
            Op::Load { kind: Load::Byte64, file: "41_06.10a", offset: 0x100003, size: 0x20000, crc: 0x4E53650B },
            Op::Load { kind: Load::Byte64, file: "41_25.7e", offset: 0x100004, size: 0x20000, crc: 0x94ADD360 },
            Op::Load { kind: Load::Byte64, file: "41_18.7c", offset: 0x100005, size: 0x20000, crc: 0xA5E1C1F3 },
            Op::Load { kind: Load::Byte64, file: "41_39.9h", offset: 0x100006, size: 0x20000, crc: 0x5B5C3949 },
            Op::Load { kind: Load::Byte64, file: "41_33.9f", offset: 0x100007, size: 0x20000, crc: 0x7A31B0E2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_23.13b", offset: 0x0, size: 0x8000, crc: 0x0F9D8527 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "41_30.12c", offset: 0x0, size: 0x20000, crc: 0xD1F15AEB },
            Op::Load { kind: Load::Byte, file: "41_31.13c", offset: 0x20000, size: 0x20000, crc: 0x15AEC3A6 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MERCS: RomSetDef = RomSetDef { name: "mercs", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "so2_30e.11f", offset: 0x0, size: 0x20000, crc: 0xE17F9BF7 },
            Op::Load { kind: Load::Byte16, file: "so2_35e.11h", offset: 0x1, size: 0x20000, crc: 0x78E63575 },
            Op::Load { kind: Load::Byte16, file: "so2_31e.12f", offset: 0x40000, size: 0x20000, crc: 0x51204D36 },
            Op::Load { kind: Load::Byte16, file: "so2_36e.12h", offset: 0x40001, size: 0x20000, crc: 0x9CFBA8B4 },
            Op::Load { kind: Load::Word16Swap, file: "so2-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x2EB5CF0C },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "so2-6m.8a", offset: 0x0, size: 0x80000, crc: 0xAA6102AF },
            Op::Load { kind: Load::Word64, file: "so2-8m.10a", offset: 0x2, size: 0x80000, crc: 0x839E6869 },
            Op::Load { kind: Load::Word64, file: "so2-2m.4a", offset: 0x4, size: 0x80000, crc: 0x597C2875 },
            Op::Load { kind: Load::Word64, file: "so2-4m.6a", offset: 0x6, size: 0x80000, crc: 0x912A9CA0 },
            Op::Load { kind: Load::Byte64, file: "so2_24.7d", offset: 0x200000, size: 0x20000, crc: 0x3F254EFE },
            Op::Load { kind: Load::Byte64, file: "so2_14.7c", offset: 0x200001, size: 0x20000, crc: 0xF5A8905E },
            Op::Load { kind: Load::Byte64, file: "so2_26.9d", offset: 0x200002, size: 0x20000, crc: 0xF3AA5A4A },
            Op::Load { kind: Load::Byte64, file: "so2_16.9c", offset: 0x200003, size: 0x20000, crc: 0xB43CD1A8 },
            Op::Load { kind: Load::Byte64, file: "so2_20.3d", offset: 0x200004, size: 0x20000, crc: 0x8CA751A3 },
            Op::Load { kind: Load::Byte64, file: "so2_10.3c", offset: 0x200005, size: 0x20000, crc: 0xE9F569FD },
            Op::Load { kind: Load::Byte64, file: "so2_22.5d", offset: 0x200006, size: 0x20000, crc: 0xFCE9A377 },
            Op::Load { kind: Load::Byte64, file: "so2_12.5c", offset: 0x200007, size: 0x20000, crc: 0xB7DF8A06 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_09.12b", offset: 0x0, size: 0x8000, crc: 0xD09D7C7A },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_18.11c", offset: 0x0, size: 0x20000, crc: 0xBBEA1643 },
            Op::Load { kind: Load::Byte, file: "so2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xAC58AA71 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MERCSU: RomSetDef = RomSetDef { name: "mercsu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "so2_30a.11f", offset: 0x0, size: 0x20000, crc: 0xE4E725D7 },
            Op::Load { kind: Load::Byte16, file: "so2_35a.11h", offset: 0x1, size: 0x20000, crc: 0xE7843445 },
            Op::Load { kind: Load::Byte16, file: "so2_31a.12f", offset: 0x40000, size: 0x20000, crc: 0xC0B91DEA },
            Op::Load { kind: Load::Byte16, file: "so2_36a.12h", offset: 0x40001, size: 0x20000, crc: 0x591EDF6C },
            Op::Load { kind: Load::Word16Swap, file: "so2-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x2EB5CF0C },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "so2-6m.8a", offset: 0x0, size: 0x80000, crc: 0xAA6102AF },
            Op::Load { kind: Load::Word64, file: "so2-8m.10a", offset: 0x2, size: 0x80000, crc: 0x839E6869 },
            Op::Load { kind: Load::Word64, file: "so2-2m.4a", offset: 0x4, size: 0x80000, crc: 0x597C2875 },
            Op::Load { kind: Load::Word64, file: "so2-4m.6a", offset: 0x6, size: 0x80000, crc: 0x912A9CA0 },
            Op::Load { kind: Load::Byte64, file: "so2_24.7d", offset: 0x200000, size: 0x20000, crc: 0x3F254EFE },
            Op::Load { kind: Load::Byte64, file: "so2_14.7c", offset: 0x200001, size: 0x20000, crc: 0xF5A8905E },
            Op::Load { kind: Load::Byte64, file: "so2_26.9d", offset: 0x200002, size: 0x20000, crc: 0xF3AA5A4A },
            Op::Load { kind: Load::Byte64, file: "so2_16.9c", offset: 0x200003, size: 0x20000, crc: 0xB43CD1A8 },
            Op::Load { kind: Load::Byte64, file: "so2_20.3d", offset: 0x200004, size: 0x20000, crc: 0x8CA751A3 },
            Op::Load { kind: Load::Byte64, file: "so2_10.3c", offset: 0x200005, size: 0x20000, crc: 0xE9F569FD },
            Op::Load { kind: Load::Byte64, file: "so2_22.5d", offset: 0x200006, size: 0x20000, crc: 0xFCE9A377 },
            Op::Load { kind: Load::Byte64, file: "so2_12.5c", offset: 0x200007, size: 0x20000, crc: 0xB7DF8A06 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_09.12b", offset: 0x0, size: 0x8000, crc: 0xD09D7C7A },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_18.11c", offset: 0x0, size: 0x20000, crc: 0xBBEA1643 },
            Op::Load { kind: Load::Byte, file: "so2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xAC58AA71 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MERCSUR1: RomSetDef = RomSetDef { name: "mercsur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "so2_30.11f", offset: 0x0, size: 0x20000, crc: 0xE17F9BF7 },
            Op::Load { kind: Load::Byte16, file: "so2_35.11h", offset: 0x1, size: 0x20000, crc: 0x4477DF61 },
            Op::Load { kind: Load::Byte16, file: "so2_31.12f", offset: 0x40000, size: 0x20000, crc: 0x51204D36 },
            Op::Load { kind: Load::Byte16, file: "so2_36.12h", offset: 0x40001, size: 0x20000, crc: 0x9CFBA8B4 },
            Op::Load { kind: Load::Word16Swap, file: "so2-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x2EB5CF0C },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "so2-6m.8a", offset: 0x0, size: 0x80000, crc: 0xAA6102AF },
            Op::Load { kind: Load::Word64, file: "so2-8m.10a", offset: 0x2, size: 0x80000, crc: 0x839E6869 },
            Op::Load { kind: Load::Word64, file: "so2-2m.4a", offset: 0x4, size: 0x80000, crc: 0x597C2875 },
            Op::Load { kind: Load::Word64, file: "so2-4m.6a", offset: 0x6, size: 0x80000, crc: 0x912A9CA0 },
            Op::Load { kind: Load::Byte64, file: "so2_24.7d", offset: 0x200000, size: 0x20000, crc: 0x3F254EFE },
            Op::Load { kind: Load::Byte64, file: "so2_14.7c", offset: 0x200001, size: 0x20000, crc: 0xF5A8905E },
            Op::Load { kind: Load::Byte64, file: "so2_26.9d", offset: 0x200002, size: 0x20000, crc: 0xF3AA5A4A },
            Op::Load { kind: Load::Byte64, file: "so2_16.9c", offset: 0x200003, size: 0x20000, crc: 0xB43CD1A8 },
            Op::Load { kind: Load::Byte64, file: "so2_20.3d", offset: 0x200004, size: 0x20000, crc: 0x8CA751A3 },
            Op::Load { kind: Load::Byte64, file: "so2_10.3c", offset: 0x200005, size: 0x20000, crc: 0xE9F569FD },
            Op::Load { kind: Load::Byte64, file: "so2_22.5d", offset: 0x200006, size: 0x20000, crc: 0xFCE9A377 },
            Op::Load { kind: Load::Byte64, file: "so2_12.5c", offset: 0x200007, size: 0x20000, crc: 0xB7DF8A06 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_09.12b", offset: 0x0, size: 0x8000, crc: 0xD09D7C7A },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_18.11c", offset: 0x0, size: 0x20000, crc: 0xBBEA1643 },
            Op::Load { kind: Load::Byte, file: "so2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xAC58AA71 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MERCSJ: RomSetDef = RomSetDef { name: "mercsj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "so2_36.12f", offset: 0x0, size: 0x20000, crc: 0xE17F9BF7 },
            Op::Load { kind: Load::Byte16, file: "so2_42.12h", offset: 0x1, size: 0x20000, crc: 0x2C3884C6 },
            Op::Load { kind: Load::Byte16, file: "so2_37.13f", offset: 0x40000, size: 0x20000, crc: 0x51204D36 },
            Op::Load { kind: Load::Byte16, file: "so2_43.13h", offset: 0x40001, size: 0x20000, crc: 0x9CFBA8B4 },
            Op::Load { kind: Load::Byte16, file: "so2_34.10f", offset: 0x80000, size: 0x20000, crc: 0xB8DAE95F },
            Op::Load { kind: Load::Byte16, file: "so2_40.10h", offset: 0x80001, size: 0x20000, crc: 0xDE37771C },
            Op::Load { kind: Load::Byte16, file: "so2_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x7D24394D },
            Op::Load { kind: Load::Byte16, file: "so2_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x914F85E0 },
        ] },
        RegionDef { name: "gfx", size: 0x300000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "so2_09.4b", offset: 0x0, size: 0x20000, crc: 0x690C261D },
            Op::Load { kind: Load::Byte64, file: "so2_01.4a", offset: 0x1, size: 0x20000, crc: 0x31FD2715 },
            Op::Load { kind: Load::Byte64, file: "so2_13.9b", offset: 0x2, size: 0x20000, crc: 0xB5E48282 },
            Op::Load { kind: Load::Byte64, file: "so2_05.9a", offset: 0x3, size: 0x20000, crc: 0x54BED82C },
            Op::Load { kind: Load::Byte64, file: "so2_24.5e", offset: 0x4, size: 0x20000, crc: 0x78B6F0CB },
            Op::Load { kind: Load::Byte64, file: "so2_17.5c", offset: 0x5, size: 0x20000, crc: 0xE78BB308 },
            Op::Load { kind: Load::Byte64, file: "so2_38.8h", offset: 0x6, size: 0x20000, crc: 0x0010A9A2 },
            Op::Load { kind: Load::Byte64, file: "so2_32.8f", offset: 0x7, size: 0x20000, crc: 0x75DFFC9A },
            Op::Load { kind: Load::Byte64, file: "so2_10.5b", offset: 0x100000, size: 0x20000, crc: 0x2F871714 },
            Op::Load { kind: Load::Byte64, file: "so2_02.5a", offset: 0x100001, size: 0x20000, crc: 0xB4B2A0B7 },
            Op::Load { kind: Load::Byte64, file: "so2_14.10b", offset: 0x100002, size: 0x20000, crc: 0x737A744B },
            Op::Load { kind: Load::Byte64, file: "so2_06.10a", offset: 0x100003, size: 0x20000, crc: 0x9D756F51 },
            Op::Load { kind: Load::Byte64, file: "so2_25.7e", offset: 0x100004, size: 0x20000, crc: 0x6D0E05D6 },
            Op::Load { kind: Load::Byte64, file: "so2_18.7c", offset: 0x100005, size: 0x20000, crc: 0x96F61F4E },
            Op::Load { kind: Load::Byte64, file: "so2_39.9h", offset: 0x100006, size: 0x20000, crc: 0xD52BA336 },
            Op::Load { kind: Load::Byte64, file: "so2_33.9f", offset: 0x100007, size: 0x20000, crc: 0x39B90D25 },
            Op::Load { kind: Load::Byte64, file: "so2_11.7b", offset: 0x200000, size: 0x20000, crc: 0x3F254EFE },
            Op::Load { kind: Load::Byte64, file: "so2_03.7a", offset: 0x200001, size: 0x20000, crc: 0xF5A8905E },
            Op::Load { kind: Load::Byte64, file: "so2_15.11b", offset: 0x200002, size: 0x20000, crc: 0xF3AA5A4A },
            Op::Load { kind: Load::Byte64, file: "so2_07.11a", offset: 0x200003, size: 0x20000, crc: 0xB43CD1A8 },
            Op::Load { kind: Load::Byte64, file: "so2_26.8e", offset: 0x200004, size: 0x20000, crc: 0x8CA751A3 },
            Op::Load { kind: Load::Byte64, file: "so2_19.8c", offset: 0x200005, size: 0x20000, crc: 0xE9F569FD },
            Op::Load { kind: Load::Byte64, file: "so2_28.10e", offset: 0x200006, size: 0x20000, crc: 0xFCE9A377 },
            Op::Load { kind: Load::Byte64, file: "so2_21.10c", offset: 0x200007, size: 0x20000, crc: 0xB7DF8A06 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_23.13b", offset: 0x0, size: 0x8000, crc: 0xD09D7C7A },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "so2_30.12c", offset: 0x0, size: 0x20000, crc: 0xBBEA1643 },
            Op::Load { kind: Load::Byte, file: "so2_31.13c", offset: 0x20000, size: 0x20000, crc: 0xAC58AA71 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_MTWINS: RomSetDef = RomSetDef { name: "mtwins", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "che_30.11f", offset: 0x0, size: 0x20000, crc: 0x9A2A2DB1 },
            Op::Load { kind: Load::Byte16, file: "che_35.11h", offset: 0x1, size: 0x20000, crc: 0xA7F96B02 },
            Op::Load { kind: Load::Byte16, file: "che_31.12f", offset: 0x40000, size: 0x20000, crc: 0xBBFF8A99 },
            Op::Load { kind: Load::Byte16, file: "che_36.12h", offset: 0x40001, size: 0x20000, crc: 0x0FA00C39 },
            Op::Load { kind: Load::Word16Swap, file: "ck-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x9B70BD41 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ck-5m.7a", offset: 0x0, size: 0x80000, crc: 0x4EC75F15 },
            Op::Load { kind: Load::Word64, file: "ck-7m.9a", offset: 0x2, size: 0x80000, crc: 0xD85D00D6 },
            Op::Load { kind: Load::Word64, file: "ck-1m.3a", offset: 0x4, size: 0x80000, crc: 0xF33CA9D4 },
            Op::Load { kind: Load::Word64, file: "ck-3m.5a", offset: 0x6, size: 0x80000, crc: 0x0BA2047F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ch_09.12b", offset: 0x0, size: 0x8000, crc: 0x4D4255B7 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ch_18.11c", offset: 0x0, size: 0x20000, crc: 0xF909E8DE },
            Op::Load { kind: Load::Byte, file: "ch_19.12c", offset: 0x20000, size: 0x20000, crc: 0xFC158CF7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_CHIKIJ: RomSetDef = RomSetDef { name: "chikij", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "chj_36a.12f", offset: 0x0, size: 0x20000, crc: 0xEC1328D8 },
            Op::Load { kind: Load::Byte16, file: "chj_42a.12h", offset: 0x1, size: 0x20000, crc: 0x4AE13503 },
            Op::Load { kind: Load::Byte16, file: "chj_37a.13f", offset: 0x40000, size: 0x20000, crc: 0x46D2CF7B },
            Op::Load { kind: Load::Byte16, file: "chj_43a.13h", offset: 0x40001, size: 0x20000, crc: 0x8D387FE8 },
            Op::Load { kind: Load::Byte16, file: "ch_34.10f", offset: 0x80000, size: 0x20000, crc: 0x609ED2F9 },
            Op::Load { kind: Load::Byte16, file: "ch_40.10h", offset: 0x80001, size: 0x20000, crc: 0xBE0D8301 },
            Op::Load { kind: Load::Byte16, file: "ch_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xB810867F },
            Op::Load { kind: Load::Byte16, file: "ch_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x8AD96155 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ch_09.4b", offset: 0x0, size: 0x20000, crc: 0x567AB3CA },
            Op::Load { kind: Load::Byte64, file: "ch_01.4a", offset: 0x1, size: 0x20000, crc: 0x7F3B7B56 },
            Op::Load { kind: Load::Byte64, file: "ch_13.9b", offset: 0x2, size: 0x20000, crc: 0x12A7A8BA },
            Op::Load { kind: Load::Byte64, file: "ch_05.9a", offset: 0x3, size: 0x20000, crc: 0x6C1AFB9A },
            Op::Load { kind: Load::Byte64, file: "ch_24.5e", offset: 0x4, size: 0x20000, crc: 0x9CB6E6BC },
            Op::Load { kind: Load::Byte64, file: "ch_17.5c", offset: 0x5, size: 0x20000, crc: 0xFE490846 },
            Op::Load { kind: Load::Byte64, file: "ch_38.8h", offset: 0x6, size: 0x20000, crc: 0x6E5C8CB6 },
            Op::Load { kind: Load::Byte64, file: "ch_32.8f", offset: 0x7, size: 0x20000, crc: 0x317D27B0 },
            Op::Load { kind: Load::Byte64, file: "ch_10.5b", offset: 0x100000, size: 0x20000, crc: 0xE8251A9B },
            Op::Load { kind: Load::Byte64, file: "ch_02.5a", offset: 0x100001, size: 0x20000, crc: 0x7C8C88FB },
            Op::Load { kind: Load::Byte64, file: "ch_14.10b", offset: 0x100002, size: 0x20000, crc: 0x4012EC4B },
            Op::Load { kind: Load::Byte64, file: "ch_06.10a", offset: 0x100003, size: 0x20000, crc: 0x81884B2B },
            Op::Load { kind: Load::Byte64, file: "ch_25.7e", offset: 0x100004, size: 0x20000, crc: 0x1DFCBAC5 },
            Op::Load { kind: Load::Byte64, file: "ch_18.7c", offset: 0x100005, size: 0x20000, crc: 0x516A34D1 },
            Op::Load { kind: Load::Byte64, file: "ch_39.9h", offset: 0x100006, size: 0x20000, crc: 0x872FB2A4 },
            Op::Load { kind: Load::Byte64, file: "ch_33.9f", offset: 0x100007, size: 0x20000, crc: 0x30DC5DED },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ch_23.13b", offset: 0x0, size: 0x8000, crc: 0x4D4255B7 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ch_30.12c", offset: 0x0, size: 0x20000, crc: 0xF909E8DE },
            Op::Load { kind: Load::Byte, file: "ch_31.13c", offset: 0x20000, size: 0x20000, crc: 0xFC158CF7 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_MSWORD: RomSetDef = RomSetDef { name: "msword", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "mse_30.11f", offset: 0x0, size: 0x20000, crc: 0x03FC8DBC },
            Op::Load { kind: Load::Byte16, file: "mse_35.11h", offset: 0x1, size: 0x20000, crc: 0xD5BF66CD },
            Op::Load { kind: Load::Byte16, file: "mse_31.12f", offset: 0x40000, size: 0x20000, crc: 0x30332BCF },
            Op::Load { kind: Load::Byte16, file: "mse_36.12h", offset: 0x40001, size: 0x20000, crc: 0x8F7D6CE9 },
            Op::Load { kind: Load::Word16Swap, file: "ms-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x2475DDFC },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ms-5m.7a", offset: 0x0, size: 0x80000, crc: 0xC00FE7E2 },
            Op::Load { kind: Load::Word64, file: "ms-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4CCACAC5 },
            Op::Load { kind: Load::Word64, file: "ms-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0D2BBE00 },
            Op::Load { kind: Load::Word64, file: "ms-3m.5a", offset: 0x6, size: 0x80000, crc: 0x3A1A5BF4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_09.12b", offset: 0x0, size: 0x8000, crc: 0x57B29519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_18.11c", offset: 0x0, size: 0x20000, crc: 0xFB64E90D },
            Op::Load { kind: Load::Byte, file: "ms_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74F892B9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_MSWORDR1: RomSetDef = RomSetDef { name: "mswordr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ms_30.11f", offset: 0x0, size: 0x20000, crc: 0x21C1F078 },
            Op::Load { kind: Load::Byte16, file: "ms_35.11h", offset: 0x1, size: 0x20000, crc: 0xA540A73A },
            Op::Load { kind: Load::Byte16, file: "ms_31.12f", offset: 0x40000, size: 0x20000, crc: 0xD7E762B5 },
            Op::Load { kind: Load::Byte16, file: "ms_36.12h", offset: 0x40001, size: 0x20000, crc: 0x66F2DCDB },
            Op::Load { kind: Load::Word16Swap, file: "ms-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x2475DDFC },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ms-5m.7a", offset: 0x0, size: 0x80000, crc: 0xC00FE7E2 },
            Op::Load { kind: Load::Word64, file: "ms-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4CCACAC5 },
            Op::Load { kind: Load::Word64, file: "ms-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0D2BBE00 },
            Op::Load { kind: Load::Word64, file: "ms-3m.5a", offset: 0x6, size: 0x80000, crc: 0x3A1A5BF4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_09.12b", offset: 0x0, size: 0x8000, crc: 0x57B29519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_18.11c", offset: 0x0, size: 0x20000, crc: 0xFB64E90D },
            Op::Load { kind: Load::Byte, file: "ms_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74F892B9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_MSWORDU: RomSetDef = RomSetDef { name: "mswordu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "msu_30.11f", offset: 0x0, size: 0x20000, crc: 0xD963C816 },
            Op::Load { kind: Load::Byte16, file: "msu_35.11h", offset: 0x1, size: 0x20000, crc: 0x72F179B3 },
            Op::Load { kind: Load::Byte16, file: "msu_31.12f", offset: 0x40000, size: 0x20000, crc: 0x20CD7904 },
            Op::Load { kind: Load::Byte16, file: "msu_36.12h", offset: 0x40001, size: 0x20000, crc: 0xBF88C080 },
            Op::Load { kind: Load::Word16Swap, file: "ms-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x2475DDFC },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ms-5m.7a", offset: 0x0, size: 0x80000, crc: 0xC00FE7E2 },
            Op::Load { kind: Load::Word64, file: "ms-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4CCACAC5 },
            Op::Load { kind: Load::Word64, file: "ms-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0D2BBE00 },
            Op::Load { kind: Load::Word64, file: "ms-3m.5a", offset: 0x6, size: 0x80000, crc: 0x3A1A5BF4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_09.12b", offset: 0x0, size: 0x8000, crc: 0x57B29519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_18.11c", offset: 0x0, size: 0x20000, crc: 0xFB64E90D },
            Op::Load { kind: Load::Byte, file: "ms_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74F892B9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_MSWORDJ: RomSetDef = RomSetDef { name: "mswordj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "msj_36.12f", offset: 0x0, size: 0x20000, crc: 0x04F0EF50 },
            Op::Load { kind: Load::Byte16, file: "msj_42.12h", offset: 0x1, size: 0x20000, crc: 0x9FCBB9CD },
            Op::Load { kind: Load::Byte16, file: "msj_37.13f", offset: 0x40000, size: 0x20000, crc: 0x6C060D70 },
            Op::Load { kind: Load::Byte16, file: "msj_43.13h", offset: 0x40001, size: 0x20000, crc: 0xAEC77787 },
            Op::Load { kind: Load::Byte16, file: "ms_34.10f", offset: 0x80000, size: 0x20000, crc: 0x0E59A62D },
            Op::Load { kind: Load::Byte16, file: "ms_40.10h", offset: 0x80001, size: 0x20000, crc: 0xBABADE3A },
            Op::Load { kind: Load::Byte16, file: "ms_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x03DA99D1 },
            Op::Load { kind: Load::Byte16, file: "ms_41.11h", offset: 0xc0001, size: 0x20000, crc: 0xFADF99EA },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "ms_09.4b", offset: 0x0, size: 0x20000, crc: 0x4ADEE6F6 },
            Op::Load { kind: Load::Byte64, file: "ms_01.4a", offset: 0x1, size: 0x20000, crc: 0xF7AB1B88 },
            Op::Load { kind: Load::Byte64, file: "ms_13.9b", offset: 0x2, size: 0x20000, crc: 0xE01ADC4B },
            Op::Load { kind: Load::Byte64, file: "ms_05.9a", offset: 0x3, size: 0x20000, crc: 0xF62C2369 },
            Op::Load { kind: Load::Byte64, file: "ms_24.5e", offset: 0x4, size: 0x20000, crc: 0xBE64A3A1 },
            Op::Load { kind: Load::Byte64, file: "ms_17.5c", offset: 0x5, size: 0x20000, crc: 0x0BC1665F },
            Op::Load { kind: Load::Byte64, file: "ms_38.8h", offset: 0x6, size: 0x20000, crc: 0x904A2ED5 },
            Op::Load { kind: Load::Byte64, file: "ms_32.8f", offset: 0x7, size: 0x20000, crc: 0x3D89C530 },
            Op::Load { kind: Load::Byte64, file: "ms_10.5b", offset: 0x100000, size: 0x20000, crc: 0xF02C0718 },
            Op::Load { kind: Load::Byte64, file: "ms_02.5a", offset: 0x100001, size: 0x20000, crc: 0xD071A405 },
            Op::Load { kind: Load::Byte64, file: "ms_14.10b", offset: 0x100002, size: 0x20000, crc: 0xDFB2E4DF },
            Op::Load { kind: Load::Byte64, file: "ms_06.10a", offset: 0x100003, size: 0x20000, crc: 0xD3CE2A91 },
            Op::Load { kind: Load::Byte64, file: "ms_25.7e", offset: 0x100004, size: 0x20000, crc: 0x0F199D56 },
            Op::Load { kind: Load::Byte64, file: "ms_18.7c", offset: 0x100005, size: 0x20000, crc: 0x1BA76DF2 },
            Op::Load { kind: Load::Byte64, file: "ms_39.9h", offset: 0x100006, size: 0x20000, crc: 0x01EFCE86 },
            Op::Load { kind: Load::Byte64, file: "ms_33.9f", offset: 0x100007, size: 0x20000, crc: 0xCE25DEFC },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_23.13b", offset: 0x0, size: 0x8000, crc: 0x57B29519 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms_30.12c", offset: 0x0, size: 0x20000, crc: 0xFB64E90D },
            Op::Load { kind: Load::Byte, file: "ms_31.13c", offset: 0x20000, size: 0x20000, crc: 0x74F892B9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_CAWING: RomSetDef = RomSetDef { name: "cawing", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "cae_30a.11f", offset: 0x0, size: 0x20000, crc: 0x91FCEACD },
            Op::Load { kind: Load::Byte16, file: "cae_35a.11h", offset: 0x1, size: 0x20000, crc: 0x3EF03083 },
            Op::Load { kind: Load::Byte16, file: "cae_31a.12f", offset: 0x40000, size: 0x20000, crc: 0xE5B75CAF },
            Op::Load { kind: Load::Byte16, file: "cae_36a.12h", offset: 0x40001, size: 0x20000, crc: 0xC73FD713 },
            Op::Load { kind: Load::Word16Swap, file: "ca-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x0C4837D4 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ca-5m.7a", offset: 0x0, size: 0x80000, crc: 0x66D4CC37 },
            Op::Load { kind: Load::Word64, file: "ca-7m.9a", offset: 0x2, size: 0x80000, crc: 0xB6F896F2 },
            Op::Load { kind: Load::Word64, file: "ca-1m.3a", offset: 0x4, size: 0x80000, crc: 0x4D0620FD },
            Op::Load { kind: Load::Word64, file: "ca-3m.5a", offset: 0x6, size: 0x80000, crc: 0x0B0341C3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ca_9.12b", offset: 0x0, size: 0x8000, crc: 0x96FE7485 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ca_18.11c", offset: 0x0, size: 0x20000, crc: 0x4A613A2C },
            Op::Load { kind: Load::Byte, file: "ca_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74584493 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_CAWINGR1: RomSetDef = RomSetDef { name: "cawingr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "cae_30.11f", offset: 0x0, size: 0x20000, crc: 0x23305CD5 },
            Op::Load { kind: Load::Byte16, file: "cae_35.11h", offset: 0x1, size: 0x20000, crc: 0x69419113 },
            Op::Load { kind: Load::Byte16, file: "cae_31.12f", offset: 0x40000, size: 0x20000, crc: 0x9008DFB3 },
            Op::Load { kind: Load::Byte16, file: "cae_36.12h", offset: 0x40001, size: 0x20000, crc: 0x4DBF6F8E },
            Op::Load { kind: Load::Word16Swap, file: "ca-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x0C4837D4 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ca-5m.7a", offset: 0x0, size: 0x80000, crc: 0x66D4CC37 },
            Op::Load { kind: Load::Word64, file: "ca-7m.9a", offset: 0x2, size: 0x80000, crc: 0xB6F896F2 },
            Op::Load { kind: Load::Word64, file: "ca-1m.3a", offset: 0x4, size: 0x80000, crc: 0x4D0620FD },
            Op::Load { kind: Load::Word64, file: "ca-3m.5a", offset: 0x6, size: 0x80000, crc: 0x0B0341C3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cae_09.12b", offset: 0x0, size: 0x8000, crc: 0x96FE7485 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cae_18.11c", offset: 0x0, size: 0x20000, crc: 0x4A613A2C },
            Op::Load { kind: Load::Byte, file: "cae_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74584493 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_CAWINGU: RomSetDef = RomSetDef { name: "cawingu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "cau_36.12f", offset: 0x0, size: 0x20000, crc: 0xC2574C0C },
            Op::Load { kind: Load::Byte16, file: "cau_42.12h", offset: 0x1, size: 0x20000, crc: 0xD89E00BE },
            Op::Load { kind: Load::Byte16, file: "cau_37.13f", offset: 0x40000, size: 0x20000, crc: 0x8E6D4F8A },
            Op::Load { kind: Load::Byte16, file: "cau_43.13h", offset: 0x40001, size: 0x20000, crc: 0xECE07955 },
            Op::Load { kind: Load::Byte16, file: "cau_34.10f", offset: 0x80000, size: 0x20000, crc: 0x5FDA906E },
            Op::Load { kind: Load::Byte16, file: "cau_40.10h", offset: 0x80001, size: 0x20000, crc: 0x736C1835 },
            Op::Load { kind: Load::Byte16, file: "cau_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x74C2DDF0 },
            Op::Load { kind: Load::Byte16, file: "cau_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x2A44BFE5 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "cau_09.4b", offset: 0x0, size: 0x20000, crc: 0xD4B17C3A },
            Op::Load { kind: Load::Byte64, file: "cau_01.4a", offset: 0x1, size: 0x20000, crc: 0x34C3094E },
            Op::Load { kind: Load::Byte64, file: "cau_13.9b", offset: 0x2, size: 0x20000, crc: 0x9D5C7911 },
            Op::Load { kind: Load::Byte64, file: "cau_05.9a", offset: 0x3, size: 0x20000, crc: 0xF042CC7B },
            Op::Load { kind: Load::Byte64, file: "cau_24.5e", offset: 0x4, size: 0x20000, crc: 0x0EAC450F },
            Op::Load { kind: Load::Byte64, file: "cau_17.5c", offset: 0x5, size: 0x20000, crc: 0x4FAB0D0C },
            Op::Load { kind: Load::Byte64, file: "cau_38.8h", offset: 0x6, size: 0x20000, crc: 0xCB96ED24 },
            Op::Load { kind: Load::Byte64, file: "cau_32.8f", offset: 0x7, size: 0x20000, crc: 0x433A0859 },
            Op::Load { kind: Load::Byte64, file: "cau_10.5b", offset: 0x100000, size: 0x20000, crc: 0x4AF10EF2 },
            Op::Load { kind: Load::Byte64, file: "cau_02.5a", offset: 0x100001, size: 0x20000, crc: 0x3E1F5B34 },
            Op::Load { kind: Load::Byte64, file: "cau_14.10b", offset: 0x100002, size: 0x20000, crc: 0x2BEF78C4 },
            Op::Load { kind: Load::Byte64, file: "cau_06.10a", offset: 0x100003, size: 0x20000, crc: 0x3777EDE1 },
            Op::Load { kind: Load::Byte64, file: "cau_25.7e", offset: 0x100004, size: 0x20000, crc: 0x859EE531 },
            Op::Load { kind: Load::Byte64, file: "cau_18.7c", offset: 0x100005, size: 0x20000, crc: 0x4C52EDF1 },
            Op::Load { kind: Load::Byte64, file: "cau_39.9h", offset: 0x100006, size: 0x20000, crc: 0x147BE975 },
            Op::Load { kind: Load::Byte64, file: "cau_33.9f", offset: 0x100007, size: 0x20000, crc: 0x8560C130 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cau_23.13b", offset: 0x0, size: 0x8000, crc: 0x96FE7485 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cau_30.12c", offset: 0x0, size: 0x20000, crc: 0x4A613A2C },
            Op::Load { kind: Load::Byte, file: "cau_31.13c", offset: 0x20000, size: 0x20000, crc: 0x74584493 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_CAWINGUR1: RomSetDef = RomSetDef { name: "cawingur1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "cau_30a.11f", offset: 0x0, size: 0x20000, crc: 0x91FCEACD },
            Op::Load { kind: Load::Byte16, file: "cau_35a.11h", offset: 0x1, size: 0x20000, crc: 0xF090D9B2 },
            Op::Load { kind: Load::Byte16, file: "cau_31a.12f", offset: 0x40000, size: 0x20000, crc: 0xE5B75CAF },
            Op::Load { kind: Load::Byte16, file: "cau_36a.12h", offset: 0x40001, size: 0x20000, crc: 0xC73FD713 },
            Op::Load { kind: Load::Word16Swap, file: "ca-32m.8h", offset: 0x80000, size: 0x80000, crc: 0x0C4837D4 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ca-5m.7a", offset: 0x0, size: 0x80000, crc: 0x66D4CC37 },
            Op::Load { kind: Load::Word64, file: "ca-7m.9a", offset: 0x2, size: 0x80000, crc: 0xB6F896F2 },
            Op::Load { kind: Load::Word64, file: "ca-1m.3a", offset: 0x4, size: 0x80000, crc: 0x4D0620FD },
            Op::Load { kind: Load::Word64, file: "ca-3m.5a", offset: 0x6, size: 0x80000, crc: 0x0B0341C3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cau_09.12b", offset: 0x0, size: 0x8000, crc: 0x96FE7485 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cau_18.11c", offset: 0x0, size: 0x20000, crc: 0x4A613A2C },
            Op::Load { kind: Load::Byte, file: "cau_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74584493 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_CAWINGJ: RomSetDef = RomSetDef { name: "cawingj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "caj_36a.12f", offset: 0x0, size: 0x20000, crc: 0x91FCEACD },
            Op::Load { kind: Load::Byte16, file: "caj_42a.12h", offset: 0x1, size: 0x20000, crc: 0x039F8362 },
            Op::Load { kind: Load::Byte16, file: "caj_37a.13f", offset: 0x40000, size: 0x20000, crc: 0xE5B75CAF },
            Op::Load { kind: Load::Byte16, file: "caj_43a.13h", offset: 0x40001, size: 0x20000, crc: 0xC73FD713 },
            Op::Load { kind: Load::Byte16, file: "caj_34.10f", offset: 0x80000, size: 0x20000, crc: 0x51EA57F4 },
            Op::Load { kind: Load::Byte16, file: "caj_40.10h", offset: 0x80001, size: 0x20000, crc: 0x2AB71AE1 },
            Op::Load { kind: Load::Byte16, file: "caj_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x01D71973 },
            Op::Load { kind: Load::Byte16, file: "caj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x3A43B538 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "caj_09.4b", offset: 0x0, size: 0x20000, crc: 0x41B0F9A6 },
            Op::Load { kind: Load::Byte64, file: "caj_01.4a", offset: 0x1, size: 0x20000, crc: 0x1002D0B8 },
            Op::Load { kind: Load::Byte64, file: "caj_13.9b", offset: 0x2, size: 0x20000, crc: 0x6F3948B2 },
            Op::Load { kind: Load::Byte64, file: "caj_05.9a", offset: 0x3, size: 0x20000, crc: 0x207373D7 },
            Op::Load { kind: Load::Byte64, file: "caj_24.5e", offset: 0x4, size: 0x20000, crc: 0xE356AAD7 },
            Op::Load { kind: Load::Byte64, file: "caj_17.5c", offset: 0x5, size: 0x20000, crc: 0x540F2FD8 },
            Op::Load { kind: Load::Byte64, file: "caj_38.8h", offset: 0x6, size: 0x20000, crc: 0x2464D4AB },
            Op::Load { kind: Load::Byte64, file: "caj_32.8f", offset: 0x7, size: 0x20000, crc: 0x9B5836B3 },
            Op::Load { kind: Load::Byte64, file: "caj_10.5b", offset: 0x100000, size: 0x20000, crc: 0xBF8A5F52 },
            Op::Load { kind: Load::Byte64, file: "caj_02.5a", offset: 0x100001, size: 0x20000, crc: 0x125B018D },
            Op::Load { kind: Load::Byte64, file: "caj_14.10b", offset: 0x100002, size: 0x20000, crc: 0x8458E7D7 },
            Op::Load { kind: Load::Byte64, file: "caj_06.10a", offset: 0x100003, size: 0x20000, crc: 0xCF80E164 },
            Op::Load { kind: Load::Byte64, file: "caj_25.7e", offset: 0x100004, size: 0x20000, crc: 0xCDD0204D },
            Op::Load { kind: Load::Byte64, file: "caj_18.7c", offset: 0x100005, size: 0x20000, crc: 0x29C1D4B1 },
            Op::Load { kind: Load::Byte64, file: "caj_39.9h", offset: 0x100006, size: 0x20000, crc: 0xEEA23B67 },
            Op::Load { kind: Load::Byte64, file: "caj_33.9f", offset: 0x100007, size: 0x20000, crc: 0xDDE3891F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "caj_23.13b", offset: 0x0, size: 0x8000, crc: 0x96FE7485 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "caj_30.12c", offset: 0x0, size: 0x20000, crc: 0x4A613A2C },
            Op::Load { kind: Load::Byte, file: "caj_31.13c", offset: 0x20000, size: 0x20000, crc: 0x74584493 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CAWINGJR: RomSetDef = RomSetDef { name: "cawingjr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "usnj_23.8f", offset: 0x0, size: 0x80000, crc: 0xBBE45B1E },
            Op::Load { kind: Load::Word16Swap, file: "usnj_22.7f", offset: 0x80000, size: 0x80000, crc: 0x54872F9F },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "usn_01.3a", offset: 0x0, size: 0x80000, crc: 0xC08FFB8B },
            Op::Load { kind: Load::Word64, file: "usn_02.4a", offset: 0x2, size: 0x80000, crc: 0x37B1B27A },
            Op::Load { kind: Load::Word64, file: "usn_03.5a", offset: 0x4, size: 0x80000, crc: 0x9F89A540 },
            Op::Load { kind: Load::Word64, file: "usn_04.6a", offset: 0x6, size: 0x80000, crc: 0x1F31F1D0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "usn_09.12f", offset: 0x0, size: 0x8000, crc: 0x0EB8A1D4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "usn_18.11c", offset: 0x0, size: 0x20000, crc: 0x4A613A2C },
            Op::Load { kind: Load::Byte, file: "usn_19.12c", offset: 0x20000, size: 0x20000, crc: 0x74584493 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_NEMO: RomSetDef = RomSetDef { name: "nemo", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "nme_30a.11f", offset: 0x0, size: 0x20000, crc: 0xD2C03E56 },
            Op::Load { kind: Load::Byte16, file: "nme_35a.11h", offset: 0x1, size: 0x20000, crc: 0x5FD31661 },
            Op::Load { kind: Load::Byte16, file: "nme_31a.12f", offset: 0x40000, size: 0x20000, crc: 0xB2BD4F6F },
            Op::Load { kind: Load::Byte16, file: "nme_36a.12h", offset: 0x40001, size: 0x20000, crc: 0xEE9450E3 },
            Op::Load { kind: Load::Word16Swap, file: "nm-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xD6D1ADD3 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "nm-5m.7a", offset: 0x0, size: 0x80000, crc: 0x487B8747 },
            Op::Load { kind: Load::Word64, file: "nm-7m.9a", offset: 0x2, size: 0x80000, crc: 0x203DC8C6 },
            Op::Load { kind: Load::Word64, file: "nm-1m.3a", offset: 0x4, size: 0x80000, crc: 0x9E878024 },
            Op::Load { kind: Load::Word64, file: "nm-3m.5a", offset: 0x6, size: 0x80000, crc: 0xBB01E6B6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nme_09.12b", offset: 0x0, size: 0x8000, crc: 0x0F4B0581 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nme_18.11c", offset: 0x0, size: 0x20000, crc: 0xBAB333D4 },
            Op::Load { kind: Load::Byte, file: "nme_19.12c", offset: 0x20000, size: 0x20000, crc: 0x2650A0A8 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_NEMOR1: RomSetDef = RomSetDef { name: "nemor1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "nme_30.11f", offset: 0x0, size: 0x20000, crc: 0x71B333DB },
            Op::Load { kind: Load::Byte16, file: "nme_35.11h", offset: 0x1, size: 0x20000, crc: 0xD153BC18 },
            Op::Load { kind: Load::Byte16, file: "nme_31.12f", offset: 0x40000, size: 0x20000, crc: 0x7E83DBD2 },
            Op::Load { kind: Load::Byte16, file: "nme_36.12h", offset: 0x40001, size: 0x20000, crc: 0x6AEEEC81 },
            Op::Load { kind: Load::Word16Swap, file: "nm-32m.8h", offset: 0x80000, size: 0x80000, crc: 0xD6D1ADD3 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "nm-5m.7a", offset: 0x0, size: 0x80000, crc: 0x487B8747 },
            Op::Load { kind: Load::Word64, file: "nm-7m.9a", offset: 0x2, size: 0x80000, crc: 0x203DC8C6 },
            Op::Load { kind: Load::Word64, file: "nm-1m.3a", offset: 0x4, size: 0x80000, crc: 0x9E878024 },
            Op::Load { kind: Load::Word64, file: "nm-3m.5a", offset: 0x6, size: 0x80000, crc: 0xBB01E6B6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nme_09.12b", offset: 0x0, size: 0x8000, crc: 0x0F4B0581 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nme_18.11c", offset: 0x0, size: 0x20000, crc: 0xBAB333D4 },
            Op::Load { kind: Load::Byte, file: "nme_19.12c", offset: 0x20000, size: 0x20000, crc: 0x2650A0A8 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_NEMOJ: RomSetDef = RomSetDef { name: "nemoj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "nmj_36a.12f", offset: 0x0, size: 0x20000, crc: 0xDAECEABB },
            Op::Load { kind: Load::Byte16, file: "nmj_42a.12h", offset: 0x1, size: 0x20000, crc: 0x55024740 },
            Op::Load { kind: Load::Byte16, file: "nmj_37a.13f", offset: 0x40000, size: 0x20000, crc: 0x619068B6 },
            Op::Load { kind: Load::Byte16, file: "nmj_43a.13h", offset: 0x40001, size: 0x20000, crc: 0xA948A53B },
            Op::Load { kind: Load::Byte16, file: "nmj_34.10f", offset: 0x80000, size: 0x20000, crc: 0x5737FEED },
            Op::Load { kind: Load::Byte16, file: "nmj_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8A4099F3 },
            Op::Load { kind: Load::Byte16, file: "nmj_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xBD11A7F8 },
            Op::Load { kind: Load::Byte16, file: "nmj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x6309603D },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "nmj_09.4b", offset: 0x0, size: 0x20000, crc: 0x9D60D286 },
            Op::Load { kind: Load::Byte64, file: "nmj_01.4a", offset: 0x1, size: 0x20000, crc: 0x8A83F7C4 },
            Op::Load { kind: Load::Byte64, file: "nmj_13.9b", offset: 0x2, size: 0x20000, crc: 0xA4909FE0 },
            Op::Load { kind: Load::Byte64, file: "nmj_05.9a", offset: 0x3, size: 0x20000, crc: 0x16DB1E61 },
            Op::Load { kind: Load::Byte64, file: "nmj_24.5e", offset: 0x4, size: 0x20000, crc: 0x3312C648 },
            Op::Load { kind: Load::Byte64, file: "nmj_17.5c", offset: 0x5, size: 0x20000, crc: 0xCCFC50E2 },
            Op::Load { kind: Load::Byte64, file: "nmj_38.8h", offset: 0x6, size: 0x20000, crc: 0xAE98A997 },
            Op::Load { kind: Load::Byte64, file: "nmj_32.8f", offset: 0x7, size: 0x20000, crc: 0xB3704DDE },
            Op::Load { kind: Load::Byte64, file: "nmj_10.5b", offset: 0x100000, size: 0x20000, crc: 0x33C1388C },
            Op::Load { kind: Load::Byte64, file: "nmj_02.5a", offset: 0x100001, size: 0x20000, crc: 0x84C69469 },
            Op::Load { kind: Load::Byte64, file: "nmj_14.10b", offset: 0x100002, size: 0x20000, crc: 0x66612270 },
            Op::Load { kind: Load::Byte64, file: "nmj_06.10a", offset: 0x100003, size: 0x20000, crc: 0x8B9BCF95 },
            Op::Load { kind: Load::Byte64, file: "nmj_25.7e", offset: 0x100004, size: 0x20000, crc: 0xACFC84D2 },
            Op::Load { kind: Load::Byte64, file: "nmj_18.7c", offset: 0x100005, size: 0x20000, crc: 0x4347DEED },
            Op::Load { kind: Load::Byte64, file: "nmj_39.9h", offset: 0x100006, size: 0x20000, crc: 0x6A274ECD },
            Op::Load { kind: Load::Byte64, file: "nmj_33.9f", offset: 0x100007, size: 0x20000, crc: 0xC469DC74 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nmj_23.13b", offset: 0x0, size: 0x8000, crc: 0x8D3C5A42 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nmj_30.12c", offset: 0x0, size: 0x20000, crc: 0xBAB333D4 },
            Op::Load { kind: Load::Byte, file: "nmj_31.13c", offset: 0x20000, size: 0x20000, crc: 0x2650A0A8 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_NEMOJA: RomSetDef = RomSetDef { name: "nemoja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "nmj_36a.12f", offset: 0x0, size: 0x20000, crc: 0xDAECEABB },
            Op::Load { kind: Load::Byte16, file: "nmj_42a.12h", offset: 0x1, size: 0x20000, crc: 0x55024740 },
            Op::Load { kind: Load::Byte16, file: "nmj_37a.13f", offset: 0x40000, size: 0x20000, crc: 0x619068B6 },
            Op::Load { kind: Load::Byte16, file: "nmj_43a.13h", offset: 0x40001, size: 0x20000, crc: 0xA948A53B },
            Op::Load { kind: Load::Byte16, file: "nm_34.10f", offset: 0x80000, size: 0x20000, crc: 0x5737FEED },
            Op::Load { kind: Load::Byte16, file: "nm_40.10h", offset: 0x80001, size: 0x20000, crc: 0x8A4099F3 },
            Op::Load { kind: Load::Byte16, file: "nm_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xBD11A7F8 },
            Op::Load { kind: Load::Byte16, file: "nm_41.11h", offset: 0xc0001, size: 0x20000, crc: 0x6309603D },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "nm_09.4b", offset: 0x0, size: 0x20000, crc: 0x9D60D286 },
            Op::Load { kind: Load::Byte64, file: "nm_01.4a", offset: 0x1, size: 0x20000, crc: 0x8A83F7C4 },
            Op::Load { kind: Load::Byte64, file: "nm_13.9b", offset: 0x2, size: 0x20000, crc: 0xA4909FE0 },
            Op::Load { kind: Load::Byte64, file: "nm_05.9a", offset: 0x3, size: 0x20000, crc: 0x16DB1E61 },
            Op::Load { kind: Load::Byte64, file: "nm_24.5e", offset: 0x4, size: 0x20000, crc: 0x3312C648 },
            Op::Load { kind: Load::Byte64, file: "nm_17.5c", offset: 0x5, size: 0x20000, crc: 0xCCFC50E2 },
            Op::Load { kind: Load::Byte64, file: "nm_38.8h", offset: 0x6, size: 0x20000, crc: 0xAE98A997 },
            Op::Load { kind: Load::Byte64, file: "nm_32.8f", offset: 0x7, size: 0x20000, crc: 0xB3704DDE },
            Op::Load { kind: Load::Byte64, file: "nm_10.5b", offset: 0x100000, size: 0x20000, crc: 0x33C1388C },
            Op::Load { kind: Load::Byte64, file: "nm_02.5a", offset: 0x100001, size: 0x20000, crc: 0x84C69469 },
            Op::Load { kind: Load::Byte64, file: "nm_14.10b", offset: 0x100002, size: 0x20000, crc: 0x66612270 },
            Op::Load { kind: Load::Byte64, file: "nm_06.10a", offset: 0x100003, size: 0x20000, crc: 0x8B9BCF95 },
            Op::Load { kind: Load::Byte64, file: "nm_25.7e", offset: 0x100004, size: 0x20000, crc: 0xACFC84D2 },
            Op::Load { kind: Load::Byte64, file: "nm_18.7c", offset: 0x100005, size: 0x20000, crc: 0x4347DEED },
            Op::Load { kind: Load::Byte64, file: "nm_39.9h", offset: 0x100006, size: 0x20000, crc: 0x6A274ECD },
            Op::Load { kind: Load::Byte64, file: "nm_33.9f", offset: 0x100007, size: 0x20000, crc: 0xC469DC74 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nm_23.13b", offset: 0x0, size: 0x8000, crc: 0x8D3C5A42 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "nm_30.12c", offset: 0x0, size: 0x20000, crc: 0xBAB333D4 },
            Op::Load { kind: Load::Byte, file: "nm_31.13c", offset: 0x20000, size: 0x20000, crc: 0x2650A0A8 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2: RomSetDef = RomSetDef { name: "sf2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30g.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2e_37g.11f", offset: 0x1, size: 0x20000, crc: 0xFB92CD74 },
            Op::Load { kind: Load::Byte16, file: "sf2e_31g.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2e_38g.12f", offset: 0x40001, size: 0x20000, crc: 0x5E22DB70 },
            Op::Load { kind: Load::Byte16, file: "sf2e_28g.9e", offset: 0x80000, size: 0x20000, crc: 0x8BF9F1E5 },
            Op::Load { kind: Load::Byte16, file: "sf2e_35g.9f", offset: 0x80001, size: 0x20000, crc: 0x626EF934 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2EA: RomSetDef = RomSetDef { name: "sf2ea", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30a.11e", offset: 0x0, size: 0x20000, crc: 0xBC02C14C },
            Op::Load { kind: Load::Byte16, file: "sf2e_37a.11f", offset: 0x1, size: 0x20000, crc: 0x1C1266B3 },
            Op::Load { kind: Load::Byte16, file: "sf2e_31a.12e", offset: 0x40000, size: 0x20000, crc: 0x8B8221E6 },
            Op::Load { kind: Load::Byte16, file: "sf2e_38a.12f", offset: 0x40001, size: 0x20000, crc: 0x2D42D82A },
            Op::Load { kind: Load::Byte16, file: "sf2_28a.9e", offset: 0x80000, size: 0x20000, crc: 0x852E10EC },
            Op::Load { kind: Load::Byte16, file: "sf2_35a.9f", offset: 0x80001, size: 0x20000, crc: 0x3B075DE1 },
            Op::Load { kind: Load::Byte16, file: "sf2_29.10e", offset: 0xc0000, size: 0x20000, crc: 0xFDD0B5C1 },
            Op::Load { kind: Load::Byte16, file: "sf2_36.10f", offset: 0xc0001, size: 0x20000, crc: 0xDB66B127 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2EB: RomSetDef = RomSetDef { name: "sf2eb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30b.11e", offset: 0x0, size: 0x20000, crc: 0x57BD7051 },
            Op::Load { kind: Load::Byte16, file: "sf2e_37b.11f", offset: 0x1, size: 0x20000, crc: 0x62691CDD },
            Op::Load { kind: Load::Byte16, file: "sf2e_31b.12e", offset: 0x40000, size: 0x20000, crc: 0xA673143D },
            Op::Load { kind: Load::Byte16, file: "sf2e_38b.12f", offset: 0x40001, size: 0x20000, crc: 0x4C2CCEF7 },
            Op::Load { kind: Load::Byte16, file: "sf2_28b.9e", offset: 0x80000, size: 0x20000, crc: 0x4009955E },
            Op::Load { kind: Load::Byte16, file: "sf2_35b.9f", offset: 0x80001, size: 0x20000, crc: 0x8C1F3994 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2ED: RomSetDef = RomSetDef { name: "sf2ed", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30d.11e", offset: 0x0, size: 0x20000, crc: 0x4BB2657C },
            Op::Load { kind: Load::Byte16, file: "sf2e_37d.11f", offset: 0x1, size: 0x20000, crc: 0x102F4561 },
            Op::Load { kind: Load::Byte16, file: "sf2e_31d.12e", offset: 0x40000, size: 0x20000, crc: 0xD57B67D7 },
            Op::Load { kind: Load::Byte16, file: "sf2e_38d.12f", offset: 0x40001, size: 0x20000, crc: 0x9C8916EF },
            Op::Load { kind: Load::Byte16, file: "sf2e_28d.9e", offset: 0x80000, size: 0x20000, crc: 0x175819D1 },
            Op::Load { kind: Load::Byte16, file: "sf2e_35d.9f", offset: 0x80001, size: 0x20000, crc: 0x82060DA4 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2EE: RomSetDef = RomSetDef { name: "sf2ee", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30e.11e", offset: 0x0, size: 0x20000, crc: 0xF37CD088 },
            Op::Load { kind: Load::Byte16, file: "sf2e_37e.11f", offset: 0x1, size: 0x20000, crc: 0xC39468E6 },
            Op::Load { kind: Load::Byte16, file: "sf2e_31e.12e", offset: 0x40000, size: 0x20000, crc: 0x7C4771B4 },
            Op::Load { kind: Load::Byte16, file: "sf2e_38e.12f", offset: 0x40001, size: 0x20000, crc: 0xA4BD0CD9 },
            Op::Load { kind: Load::Byte16, file: "sf2e_28e.9e", offset: 0x80000, size: 0x20000, crc: 0xE3B95625 },
            Op::Load { kind: Load::Byte16, file: "sf2e_35e.9f", offset: 0x80001, size: 0x20000, crc: 0x3648769A },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2EF: RomSetDef = RomSetDef { name: "sf2ef", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30f.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2e_37f.11f", offset: 0x1, size: 0x20000, crc: 0xB58A741B },
            Op::Load { kind: Load::Byte16, file: "sf2e_31f.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2e_38f.12f", offset: 0x40001, size: 0x20000, crc: 0x1510E4E2 },
            Op::Load { kind: Load::Byte16, file: "sf2e_28f.9e", offset: 0x80000, size: 0x20000, crc: 0xACD8175B },
            Op::Load { kind: Load::Byte16, file: "sf2e_35f.9f", offset: 0x80001, size: 0x20000, crc: 0xC0A80BD1 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2EM: RomSetDef = RomSetDef { name: "sf2em", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2e_30.11e", offset: 0x0, size: 0x20000, crc: 0x997BDAC4 },
            Op::Load { kind: Load::Byte16, file: "sf2e_37.11f", offset: 0x1, size: 0x20000, crc: 0xF11B3D64 },
            Op::Load { kind: Load::Byte16, file: "sf2e_31.12e", offset: 0x40000, size: 0x20000, crc: 0x53E54744 },
            Op::Load { kind: Load::Byte16, file: "sf2e_38.12f", offset: 0x40001, size: 0x20000, crc: 0x5FF4DC81 },
            Op::Load { kind: Load::Byte16, file: "sf2_28.9e", offset: 0x80000, size: 0x20000, crc: 0x55D88C35 },
            Op::Load { kind: Load::Byte16, file: "sf2_35.9f", offset: 0x80001, size: 0x20000, crc: 0x4B964478 },
            Op::Load { kind: Load::Byte16, file: "sf2_29.10e", offset: 0xc0000, size: 0x20000, crc: 0xFDD0B5C1 },
            Op::Load { kind: Load::Byte16, file: "sf2_36.10f", offset: 0xc0001, size: 0x20000, crc: 0xDB66B127 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2EN: RomSetDef = RomSetDef { name: "sf2en", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "epr-b-05.12f", offset: 0x0, size: 0x20000, crc: 0xBC02C14C },
            Op::Load { kind: Load::Byte16, file: "epr-b-09.12h", offset: 0x1, size: 0x20000, crc: 0x1C1266B3 },
            Op::Load { kind: Load::Byte16, file: "epr-b-06.13f", offset: 0x40000, size: 0x20000, crc: 0x8B8221E6 },
            Op::Load { kind: Load::Byte16, file: "epr-b-10.13h", offset: 0x40001, size: 0x20000, crc: 0x2D42D82A },
            Op::Load { kind: Load::Byte16, file: "epr-b-03.10f", offset: 0x80000, size: 0x20000, crc: 0x852E10EC },
            Op::Load { kind: Load::Byte16, file: "epr-b-07.10h", offset: 0x80001, size: 0x20000, crc: 0x3B075DE1 },
            Op::Load { kind: Load::Byte16, file: "epr-b-04.11f", offset: 0xc0000, size: 0x20000, crc: 0xFDD0B5C1 },
            Op::Load { kind: Load::Byte16, file: "epr-b-08.11h", offset: 0xc0001, size: 0x20000, crc: 0xDB66B127 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "epr-b-18.5e", offset: 0x0, size: 0x40000, crc: 0x88F3485A },
            Op::Load { kind: Load::Byte64, file: "epr-b-13.5c", offset: 0x1, size: 0x40000, crc: 0x63D63E0C },
            Op::Load { kind: Load::Byte64, file: "epr-b-24.8h", offset: 0x2, size: 0x40000, crc: 0x25AE23BC },
            Op::Load { kind: Load::Byte64, file: "epr-b-23.8f", offset: 0x3, size: 0x40000, crc: 0xE5819676 },
            Op::Load { kind: Load::Byte64, file: "epr-b-07.4b", offset: 0x4, size: 0x40000, crc: 0x84AFB959 },
            Op::Load { kind: Load::Byte64, file: "epr-b-01.4a", offset: 0x5, size: 0x40000, crc: 0xAB21635D },
            Op::Load { kind: Load::Byte64, file: "epr-b-10.9b", offset: 0x6, size: 0x40000, crc: 0x2CE56F9F },
            Op::Load { kind: Load::Byte64, file: "epr-b-04.9a", offset: 0x7, size: 0x40000, crc: 0x0AD7FB2B },
            Op::Load { kind: Load::Byte64, file: "epr-b-19.8e", offset: 0x200000, size: 0x40000, crc: 0x031525CC },
            Op::Load { kind: Load::Byte64, file: "epr-b-14.8c", offset: 0x200001, size: 0x40000, crc: 0xC97046A5 },
            Op::Load { kind: Load::Byte64, file: "epr-b-21.10e", offset: 0x200002, size: 0x40000, crc: 0xACBBDB09 },
            Op::Load { kind: Load::Byte64, file: "epr-b-16.10c", offset: 0x200003, size: 0x40000, crc: 0xFA6F32D9 },
            Op::Load { kind: Load::Byte64, file: "epr-b-08.7b", offset: 0x200004, size: 0x40000, crc: 0x14756473 },
            Op::Load { kind: Load::Byte64, file: "epr-b-02.7a", offset: 0x200005, size: 0x40000, crc: 0xEE3D878A },
            Op::Load { kind: Load::Byte64, file: "epr-b-11.11b", offset: 0x200006, size: 0x40000, crc: 0xDBBFD400 },
            Op::Load { kind: Load::Byte64, file: "epr-b-05.11a", offset: 0x200007, size: 0x40000, crc: 0x37635E97 },
            Op::Load { kind: Load::Byte64, file: "epr-b-20.9e", offset: 0x400000, size: 0x40000, crc: 0x27CAE573 },
            Op::Load { kind: Load::Byte64, file: "epr-b-15.9c", offset: 0x400001, size: 0x40000, crc: 0x14E46AB1 },
            Op::Load { kind: Load::Byte64, file: "epr-b-22.11e", offset: 0x400002, size: 0x40000, crc: 0xF241F0C7 },
            Op::Load { kind: Load::Byte64, file: "epr-b-17.11c", offset: 0x400003, size: 0x40000, crc: 0xF187086B },
            Op::Load { kind: Load::Byte64, file: "epr-b-09.8b", offset: 0x400004, size: 0x40000, crc: 0x4894AA8F },
            Op::Load { kind: Load::Byte64, file: "epr-b-03.8a", offset: 0x400005, size: 0x40000, crc: 0xE8877E9D },
            Op::Load { kind: Load::Byte64, file: "epr-b-12.12b", offset: 0x400006, size: 0x40000, crc: 0xC9D4ED76 },
            Op::Load { kind: Load::Byte64, file: "epr-b-06.12a", offset: 0x400007, size: 0x40000, crc: 0x67DCC295 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c512.13b", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "epr-b-01.12c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "epr-b-02.13c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UA: RomSetDef = RomSetDef { name: "sf2ua", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30a.11e", offset: 0x0, size: 0x20000, crc: 0x08BEB861 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37a.11f", offset: 0x1, size: 0x20000, crc: 0xB7638D69 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31a.12e", offset: 0x40000, size: 0x20000, crc: 0x0D5394E0 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38a.12f", offset: 0x40001, size: 0x20000, crc: 0x42D6A79E },
            Op::Load { kind: Load::Byte16, file: "sf2u_28a.9e", offset: 0x80000, size: 0x20000, crc: 0x387A175C },
            Op::Load { kind: Load::Byte16, file: "sf2u_35a.9f", offset: 0x80001, size: 0x20000, crc: 0xA1A5ADCC },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UB: RomSetDef = RomSetDef { name: "sf2ub", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30b.11e", offset: 0x0, size: 0x20000, crc: 0x57BD7051 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37b.11f", offset: 0x1, size: 0x20000, crc: 0x4A54D479 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31b.12e", offset: 0x40000, size: 0x20000, crc: 0xA673143D },
            Op::Load { kind: Load::Byte16, file: "sf2u_38b.12f", offset: 0x40001, size: 0x20000, crc: 0x4C2CCEF7 },
            Op::Load { kind: Load::Byte16, file: "sf2u_28b.9e", offset: 0x80000, size: 0x20000, crc: 0x4009955E },
            Op::Load { kind: Load::Byte16, file: "sf2u_35b.9f", offset: 0x80001, size: 0x20000, crc: 0x8C1F3994 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UC: RomSetDef = RomSetDef { name: "sf2uc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30c.11e", offset: 0x0, size: 0x20000, crc: 0x6CB59385 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37c.11f", offset: 0x1, size: 0x20000, crc: 0x32E2C278 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31c.12e", offset: 0x40000, size: 0x20000, crc: 0xC4FFF4A9 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38c.12f", offset: 0x40001, size: 0x20000, crc: 0x8210FC0E },
            Op::Load { kind: Load::Byte16, file: "sf2u_28c.9e", offset: 0x80000, size: 0x20000, crc: 0x6EDDD5E8 },
            Op::Load { kind: Load::Byte16, file: "sf2u_35c.9f", offset: 0x80001, size: 0x20000, crc: 0x6BCB404C },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UD: RomSetDef = RomSetDef { name: "sf2ud", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30d.11e", offset: 0x0, size: 0x20000, crc: 0x4BB2657C },
            Op::Load { kind: Load::Byte16, file: "sf2u_37d.11f", offset: 0x1, size: 0x20000, crc: 0xB33B42F2 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31d.12e", offset: 0x40000, size: 0x20000, crc: 0xD57B67D7 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38d.12f", offset: 0x40001, size: 0x20000, crc: 0x9C8916EF },
            Op::Load { kind: Load::Byte16, file: "sf2u_28d.9e", offset: 0x80000, size: 0x20000, crc: 0x175819D1 },
            Op::Load { kind: Load::Byte16, file: "sf2u_35d.9f", offset: 0x80001, size: 0x20000, crc: 0x82060DA4 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UE: RomSetDef = RomSetDef { name: "sf2ue", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30e.11e", offset: 0x0, size: 0x20000, crc: 0xF37CD088 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37e.11f", offset: 0x1, size: 0x20000, crc: 0x6C61A513 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31e.12e", offset: 0x40000, size: 0x20000, crc: 0x7C4771B4 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38e.12f", offset: 0x40001, size: 0x20000, crc: 0xA4BD0CD9 },
            Op::Load { kind: Load::Byte16, file: "sf2u_28e.9e", offset: 0x80000, size: 0x20000, crc: 0xE3B95625 },
            Op::Load { kind: Load::Byte16, file: "sf2u_35e.9f", offset: 0x80001, size: 0x20000, crc: 0x3648769A },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UF: RomSetDef = RomSetDef { name: "sf2uf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30f.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37f.11f", offset: 0x1, size: 0x20000, crc: 0x169E7388 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31f.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38f.12f", offset: 0x40001, size: 0x20000, crc: 0x1510E4E2 },
            Op::Load { kind: Load::Byte16, file: "sf2u_28f.9e", offset: 0x80000, size: 0x20000, crc: 0xACD8175B },
            Op::Load { kind: Load::Byte16, file: "sf2u_35f.9f", offset: 0x80001, size: 0x20000, crc: 0xC0A80BD1 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UG: RomSetDef = RomSetDef { name: "sf2ug", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30g.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37g.11f", offset: 0x1, size: 0x20000, crc: 0x5886CAE7 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31g.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38g.12f", offset: 0x40001, size: 0x20000, crc: 0x5E22DB70 },
            Op::Load { kind: Load::Byte16, file: "sf2u_28g.9e", offset: 0x80000, size: 0x20000, crc: 0x8BF9F1E5 },
            Op::Load { kind: Load::Byte16, file: "sf2u_35g.9f", offset: 0x80001, size: 0x20000, crc: 0x626EF934 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UH: RomSetDef = RomSetDef { name: "sf2uh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30h.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37h.11f", offset: 0x1, size: 0x20000, crc: 0xE4DFFBFE },
            Op::Load { kind: Load::Byte16, file: "sf2u_31h.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38h.12f", offset: 0x40001, size: 0x20000, crc: 0xA659F678 },
            Op::Load { kind: Load::Byte16, file: "sf2u_28h.9e", offset: 0x80000, size: 0x20000, crc: 0x8A5C8EE0 },
            Op::Load { kind: Load::Byte16, file: "sf2u_35h.9f", offset: 0x80001, size: 0x20000, crc: 0xC828FC4D },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UI: RomSetDef = RomSetDef { name: "sf2ui", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30i.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2u_37i.11f", offset: 0x1, size: 0x20000, crc: 0x9DF707DD },
            Op::Load { kind: Load::Byte16, file: "sf2u_31i.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38i.12f", offset: 0x40001, size: 0x20000, crc: 0x4CB46DAF },
            Op::Load { kind: Load::Byte16, file: "sf2u_28i.9e", offset: 0x80000, size: 0x20000, crc: 0x1580BE4C },
            Op::Load { kind: Load::Byte16, file: "sf2u_35i.9f", offset: 0x80001, size: 0x20000, crc: 0x1468D185 },
            Op::Load { kind: Load::Byte16, file: "sf2_29b.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36b.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_9.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UK: RomSetDef = RomSetDef { name: "sf2uk", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2u_30k.11e", offset: 0x0, size: 0x20000, crc: 0x8F66076C },
            Op::Load { kind: Load::Byte16, file: "sf2u_37k.11f", offset: 0x1, size: 0x20000, crc: 0x4E1F6A83 },
            Op::Load { kind: Load::Byte16, file: "sf2u_31k.12e", offset: 0x40000, size: 0x20000, crc: 0xF9F89F60 },
            Op::Load { kind: Load::Byte16, file: "sf2u_38k.12f", offset: 0x40001, size: 0x20000, crc: 0x6CE0A85A },
            Op::Load { kind: Load::Byte16, file: "sf2u_28k.9e", offset: 0x80000, size: 0x20000, crc: 0x8E958F31 },
            Op::Load { kind: Load::Byte16, file: "sf2u_35k.9f", offset: 0x80001, size: 0x20000, crc: 0xFCE76FAD },
            Op::Load { kind: Load::Byte16, file: "sf2u_29a.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2u_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2UM: RomSetDef = RomSetDef { name: "sf2um", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf-2u_30m.11e", offset: 0x0, size: 0x20000, crc: 0x34A1CE02 },
            Op::Load { kind: Load::Byte16, file: "sf-2u_37m.11f", offset: 0x1, size: 0x20000, crc: 0x8CBFF19C },
            Op::Load { kind: Load::Byte16, file: "sf-2u_31m.12e", offset: 0x40000, size: 0x20000, crc: 0x64EBC8D2 },
            Op::Load { kind: Load::Byte16, file: "sf-2u_38m.12f", offset: 0x40001, size: 0x20000, crc: 0x73847443 },
            Op::Load { kind: Load::Byte16, file: "sf-2u_28m.9e", offset: 0x80000, size: 0x20000, crc: 0xEEE2B426 },
            Op::Load { kind: Load::Byte16, file: "sf-2u_35m.9f", offset: 0x80001, size: 0x20000, crc: 0xECA8B452 },
            Op::Load { kind: Load::Byte16, file: "sf-2u_29m.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf-2u_36m.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2-5m.4a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2-7m.6a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2-1m.3a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2-3m.5a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2-8m.6c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2-13m.4d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2-15m.6d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2-9m.3d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2-11m.5d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2J: RomSetDef = RomSetDef { name: "sf2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2j30.bin", offset: 0x0, size: 0x20000, crc: 0x79022B31 },
            Op::Load { kind: Load::Byte16, file: "sf2j37.bin", offset: 0x1, size: 0x20000, crc: 0x516776EC },
            Op::Load { kind: Load::Byte16, file: "sf2j31.bin", offset: 0x40000, size: 0x20000, crc: 0xFE15CB39 },
            Op::Load { kind: Load::Byte16, file: "sf2j38.bin", offset: 0x40001, size: 0x20000, crc: 0x38614D70 },
            Op::Load { kind: Load::Byte16, file: "sf2j28.bin", offset: 0x80000, size: 0x20000, crc: 0xD283187A },
            Op::Load { kind: Load::Byte16, file: "sf2j35.bin", offset: 0x80001, size: 0x20000, crc: 0xD28158E4 },
            Op::Load { kind: Load::Byte16, file: "sf2_29a.bin", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36a.bin", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.bin", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2J17: RomSetDef = RomSetDef { name: "sf2j17", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2_30l.11e", offset: 0x0, size: 0x20000, crc: 0x79022B31 },
            Op::Load { kind: Load::Byte16, file: "sf2j_37l.11f", offset: 0x1, size: 0x20000, crc: 0x04BA20C7 },
            Op::Load { kind: Load::Byte16, file: "sf2_31l.12e", offset: 0x40000, size: 0x20000, crc: 0xFE15CB39 },
            Op::Load { kind: Load::Byte16, file: "sf2_38l.12f", offset: 0x40001, size: 0x20000, crc: 0x65CB1883 },
            Op::Load { kind: Load::Byte16, file: "sf2_28l.9e", offset: 0x80000, size: 0x20000, crc: 0xD283187A },
            Op::Load { kind: Load::Byte16, file: "sf2_35l.9f", offset: 0x80001, size: 0x20000, crc: 0xE3266622 },
            Op::Load { kind: Load::Byte16, file: "sf2_29l.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2_36l.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2JA: RomSetDef = RomSetDef { name: "sf2ja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2j_30a.11e", offset: 0x0, size: 0x20000, crc: 0x57BD7051 },
            Op::Load { kind: Load::Byte16, file: "sf2j_37a.11f", offset: 0x1, size: 0x20000, crc: 0x1E1F6844 },
            Op::Load { kind: Load::Byte16, file: "sf2j_31a.12e", offset: 0x40000, size: 0x20000, crc: 0xA673143D },
            Op::Load { kind: Load::Byte16, file: "sf2j_38a.12f", offset: 0x40001, size: 0x20000, crc: 0x4C2CCEF7 },
            Op::Load { kind: Load::Byte16, file: "sf2j_28a.9e", offset: 0x80000, size: 0x20000, crc: 0x4009955E },
            Op::Load { kind: Load::Byte16, file: "sf2j_35a.9f", offset: 0x80001, size: 0x20000, crc: 0x8C1F3994 },
            Op::Load { kind: Load::Byte16, file: "sf2j_29a.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2j_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2j_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2j_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2j_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2JC: RomSetDef = RomSetDef { name: "sf2jc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2j_30c.11e", offset: 0x0, size: 0x20000, crc: 0x8ADD35EC },
            Op::Load { kind: Load::Byte16, file: "sf2j_37c.11f", offset: 0x1, size: 0x20000, crc: 0x0D74A256 },
            Op::Load { kind: Load::Byte16, file: "sf2j_31c.12e", offset: 0x40000, size: 0x20000, crc: 0xC4FFF4A9 },
            Op::Load { kind: Load::Byte16, file: "sf2j_38c.12f", offset: 0x40001, size: 0x20000, crc: 0x8210FC0E },
            Op::Load { kind: Load::Byte16, file: "sf2j_28c.9e", offset: 0x80000, size: 0x20000, crc: 0x6EDDD5E8 },
            Op::Load { kind: Load::Byte16, file: "sf2j_35c.9f", offset: 0x80001, size: 0x20000, crc: 0x6BCB404C },
            Op::Load { kind: Load::Byte16, file: "sf2j_29a.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2j_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2j_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2j_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2JF: RomSetDef = RomSetDef { name: "sf2jf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2j_30f.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2j_37f.11f", offset: 0x1, size: 0x20000, crc: 0xC1428CC6 },
            Op::Load { kind: Load::Byte16, file: "sf2j_31f.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2j_38f.12f", offset: 0x40001, size: 0x20000, crc: 0x1510E4E2 },
            Op::Load { kind: Load::Byte16, file: "sf2j_28f.9e", offset: 0x80000, size: 0x20000, crc: 0xACD8175B },
            Op::Load { kind: Load::Byte16, file: "sf2j_35f.9f", offset: 0x80001, size: 0x20000, crc: 0xC0A80BD1 },
            Op::Load { kind: Load::Byte16, file: "sf2j_29a.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2j_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2JH: RomSetDef = RomSetDef { name: "sf2jh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2j_30h.11e", offset: 0x0, size: 0x20000, crc: 0xFE39EE33 },
            Op::Load { kind: Load::Byte16, file: "sf2j_37h.11f", offset: 0x1, size: 0x20000, crc: 0x330304B0 },
            Op::Load { kind: Load::Byte16, file: "sf2j_31h.12e", offset: 0x40000, size: 0x20000, crc: 0x69A0A301 },
            Op::Load { kind: Load::Byte16, file: "sf2j_38h.12f", offset: 0x40001, size: 0x20000, crc: 0xA659F678 },
            Op::Load { kind: Load::Byte16, file: "sf2j_28h.9e", offset: 0x80000, size: 0x20000, crc: 0x8A5C8EE0 },
            Op::Load { kind: Load::Byte16, file: "sf2j_35h.9f", offset: 0x80001, size: 0x20000, crc: 0xC828FC4D },
            Op::Load { kind: Load::Byte16, file: "sf2j_29a.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2j_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2JL: RomSetDef = RomSetDef { name: "sf2jl", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf-2_30l.11e", offset: 0x0, size: 0x20000, crc: 0x34A1CE02 },
            Op::Load { kind: Load::Byte16, file: "sf-2_37l.11f", offset: 0x1, size: 0x20000, crc: 0x5B630ED2 },
            Op::Load { kind: Load::Byte16, file: "sf-2_31l.12e", offset: 0x40000, size: 0x20000, crc: 0x64EBC8D2 },
            Op::Load { kind: Load::Byte16, file: "sf-2_38l.12f", offset: 0x40001, size: 0x20000, crc: 0x73847443 },
            Op::Load { kind: Load::Byte16, file: "sf-2_28l.9e", offset: 0x80000, size: 0x20000, crc: 0xEEE2B426 },
            Op::Load { kind: Load::Byte16, file: "sf-2_35l.9f", offset: 0x80001, size: 0x20000, crc: 0xECA8B452 },
            Op::Load { kind: Load::Byte16, file: "sf2j_29a.10e", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "sf2j_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.8a", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.10a", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.7a", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.9a", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.8c", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.10c", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.7c", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.9c", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.8d", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.10d", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.7d", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.9d", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2j_09.12a", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2j_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2j_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2EBBL: RomSetDef = RomSetDef { name: "sf2ebbl", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "12.bin", offset: 0x0, size: 0x40000, crc: 0xA258B4D5 },
            Op::Load { kind: Load::Byte16, file: "09.bin", offset: 0x1, size: 0x40000, crc: 0x59CCD474 },
            Op::Load { kind: Load::Byte16, file: "11.bin", offset: 0x80000, size: 0x40000, crc: 0x82097D63 },
            Op::Load { kind: Load::Byte16, file: "10.bin", offset: 0x80001, size: 0x40000, crc: 0x0C83844D },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1b_yf082.bin", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "1d_yf028.bin", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "1a_yf087.bin", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "1c_yf088.bin", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "1f_yf085.bin", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "1h_yf115.bin", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "1e_yf111.bin", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "1g_yf002.bin", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "1j_yf117.bin", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "1l_ye040.bin", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "1i_yf038.bin", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "1k_ye039.bin", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
            Op::Load { kind: Load::Byte64, file: "05.bin", offset: 0x400000, size: 0x10000, crc: 0xA505621E },
            Op::Continue { offset: 0x400004, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "07.bin", offset: 0x400002, size: 0x10000, crc: 0xDE6271FB },
            Op::Continue { offset: 0x400006, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "06.bin", offset: 0x400001, size: 0x10000, crc: 0x23775344 },
            Op::Continue { offset: 0x400005, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "08.bin", offset: 0x400003, size: 0x10000, crc: 0x81C9550F },
            Op::Continue { offset: 0x400007, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "03.bin", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "04.bin", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "02.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "01.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2EBBL2: RomSetDef = RomSetDef { name: "sf2ebbl2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "27c020.7", offset: 0x0, size: 0x40000, crc: 0xA258B4D5 },
            Op::Load { kind: Load::Byte16, file: "27c020.5", offset: 0x1, size: 0x40000, crc: 0x59CCD474 },
            Op::Load { kind: Load::Byte16, file: "27c020.6", offset: 0x80000, size: 0x40000, crc: 0x82097D63 },
            Op::Load { kind: Load::Byte16, file: "27c020.4", offset: 0x80001, size: 0x40000, crc: 0x0C83844D },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "a-se235.bin", offset: 0x0, size: 0x80000, crc: 0xA258DE13 },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "c-se005.bin", offset: 0x2, size: 0x80000, crc: 0xC781BF87 },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "b-se194.bin", offset: 0x200000, size: 0x80000, crc: 0x5726CAB8 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "d-se064.bin", offset: 0x200002, size: 0x80000, crc: 0x4DD24197 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "e-sf004.bin", offset: 0x400000, size: 0x80000, crc: 0x187667CC },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "f-sf001.bin", offset: 0x400002, size: 0x80000, crc: 0x5B585071 },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "27c1024.10", offset: 0x400000, size: 0x20000, crc: 0x84427D1B },
            Op::Load { kind: Load::Word64, file: "27c1024.12", offset: 0x400002, size: 0x20000, crc: 0x55BC790C },
            Op::Load { kind: Load::Word64, file: "27c1024.9", offset: 0x400004, size: 0x20000, crc: 0xF8725ADD },
            Op::Load { kind: Load::Word64, file: "27c1024.11", offset: 0x400006, size: 0x20000, crc: 0xC2A5373E },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c512.3", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c512.8", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c010.2", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "27c010.1", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2EBBL3: RomSetDef = RomSetDef { name: "sf2ebbl3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ce91e-b.bin", offset: 0x0, size: 0x80000, crc: 0x963200D2 },
            Op::Load { kind: Load::Byte16, file: "ce91e-a.bin", offset: 0x1, size: 0x80000, crc: 0x02E88EC7 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1-b-yf197.07", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "1-d-yf207.12", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Byte64, file: "27020.1", offset: 0x4, size: 0x40000, crc: 0x84AFB959 },
            Op::Load { kind: Load::Byte64, file: "27020.3", offset: 0x5, size: 0x40000, crc: 0xAB21635D },
            Op::Load { kind: Load::Byte64, file: "27020.5", offset: 0x6, size: 0x40000, crc: 0x2CE56F9F },
            Op::Load { kind: Load::Byte64, file: "27020.7", offset: 0x7, size: 0x40000, crc: 0x0AD7FB2B },
            Op::Load { kind: Load::Byte64, file: "27020.2", offset: 0x200000, size: 0x40000, crc: 0x031525CC },
            Op::Load { kind: Load::Byte64, file: "27020.4", offset: 0x200001, size: 0x40000, crc: 0xC97046A5 },
            Op::Load { kind: Load::Word64, file: "1-h-yg010.10", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "1-e-yg003.02", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Byte64, file: "27020.6", offset: 0x200006, size: 0x40000, crc: 0xDBBFD400 },
            Op::Load { kind: Load::Byte64, file: "27020.8", offset: 0x200007, size: 0x40000, crc: 0x37635E97 },
            Op::Load { kind: Load::Word64, file: "1-j-yf213.09", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "wm91m-11-yd025.11", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "1-i-yf224.03", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "1-k-yf036.06", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
            Op::Load { kind: Load::Byte64, file: "27010.09hi", offset: 0x400000, size: 0x10000, crc: 0xA505621E },
            Op::Continue { offset: 0x400004, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "27010.11", offset: 0x400002, size: 0x10000, crc: 0xDE6271FB },
            Op::Continue { offset: 0x400006, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "27010.09lo", offset: 0x400001, size: 0x10000, crc: 0x23775344 },
            Op::Continue { offset: 0x400005, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "27010.06", offset: 0x400003, size: 0x10000, crc: 0x81C9550F },
            Op::Continue { offset: 0x400007, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.3", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.8", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27010.2", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "27010.1", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2STT: RomSetDef = RomSetDef { name: "sf2stt", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::NoDump,
            Op::NoDump,
            Op::Load { kind: Load::Byte16, file: "ce91e-b", offset: 0x80000, size: 0x40000, crc: 0x0862386E },
            Op::Load { kind: Load::Byte16, file: "ce91e-a", offset: 0x80001, size: 0x40000, crc: 0x0C83844D },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "stt-a", offset: 0x0, size: 0x80000, crc: 0x10A7036D },
            Op::Continue { offset: 0x2, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "stt-d", offset: 0x4, size: 0x80000, crc: 0x3580B124 },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "stt-b", offset: 0x200000, size: 0x80000, crc: 0x7A09224E },
            Op::Continue { offset: 0x200002, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "stt-e", offset: 0x200004, size: 0x80000, crc: 0x382A612C },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "stt-c", offset: 0x400000, size: 0x80000, crc: 0x11701B8F },
            Op::Continue { offset: 0x400002, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "stt-f", offset: 0x400004, size: 0x80000, crc: 0x101A0B72 },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "5.stt", offset: 0x400000, size: 0x10000, crc: 0xA505621E },
            Op::Continue { offset: 0x400004, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "7.stt", offset: 0x400002, size: 0x10000, crc: 0xDE6271FB },
            Op::Continue { offset: 0x400006, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "6.stt", offset: 0x400001, size: 0x10000, crc: 0x23775344 },
            Op::Continue { offset: 0x400005, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "8.stt", offset: 0x400003, size: 0x10000, crc: 0x81C9550F },
            Op::Continue { offset: 0x400007, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "3.stt", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "4.stt", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "2.stt", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "1.stt", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RE: RomSetDef = RomSetDef { name: "sf2re", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "4.u222", offset: 0x0, size: 0x80000, crc: 0xDB567B66 },
            Op::Load { kind: Load::Byte16, file: "3.u196", offset: 0x1, size: 0x80000, crc: 0x95EA597E },
            Op::Load { kind: Load::Byte16, file: "2.u221", offset: 0x100000, size: 0x20000, crc: 0x1073B7B6 },
            Op::Load { kind: Load::Byte16, file: "1.u195", offset: 0x100001, size: 0x20000, crc: 0x924C6CE2 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1mre125.u70", offset: 0x0, size: 0x80000, crc: 0xBAA0F81F },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "3mre121.u68", offset: 0x2, size: 0x80000, crc: 0x8EDFF95A },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "5mre148.u69", offset: 0x200000, size: 0x80000, crc: 0x468962B1 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "7mrd413.u64", offset: 0x200002, size: 0x80000, crc: 0x8165F536 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "sfiire143.u19", offset: 0x400000, size: 0x80000, crc: 0x39D763D3 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "sfiire073.u18", offset: 0x400002, size: 0x80000, crc: 0x93EC42AE },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "27c010.u31", offset: 0x400004, size: 0x10000, crc: 0x6DE44671 },
            Op::Continue { offset: 0x400000, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "27c010.u30", offset: 0x400005, size: 0x10000, crc: 0xBF0CD819 },
            Op::Continue { offset: 0x400001, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "27c010.u29", offset: 0x400006, size: 0x10000, crc: 0xE8F14362 },
            Op::Continue { offset: 0x400002, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "27c010.u28", offset: 0x400007, size: 0x10000, crc: 0x76F9F91F },
            Op::Continue { offset: 0x400003, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "csicat27c512.u191", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c020.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : plds
pub const RS_SF2MKOT: RomSetDef = RomSetDef { name: "sf2mkot", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222.bin", offset: 0x0, size: 0x80000, crc: 0xB01A94B6 },
            Op::Load { kind: Load::Byte16, file: "u196.bin", offset: 0x1, size: 0x80000, crc: 0x20461C47 },
            Op::Load { kind: Load::Byte16, file: "u221.bin", offset: 0x100000, size: 0x20000, crc: 0x64E6E091 },
            Op::Load { kind: Load::Byte16, file: "u195.bin", offset: 0x100001, size: 0x20000, crc: 0xC95E4443 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "d21.u70", offset: 0x0, size: 0x80000, crc: 0xBAA0F81F },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "d24.u68", offset: 0x2, size: 0x80000, crc: 0x8EDFF95A },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "d22.u69", offset: 0x200000, size: 0x80000, crc: 0x468962B1 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "d25.u64", offset: 0x200002, size: 0x80000, crc: 0x8165F536 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "d23.u19", offset: 0x400000, size: 0x80000, crc: 0x39D763D3 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "d26.u18", offset: 0x400002, size: 0x80000, crc: 0x93EC42AE },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "moon-1.c173.u30", offset: 0x400004, size: 0x20000, crc: 0x7E36EC84 },
            Op::Continue { offset: 0x400000, size: 0x20000 },
            Op::Load { kind: Load::Word64, file: "moon-2.c132.u29", offset: 0x400006, size: 0x20000, crc: 0x66403570 },
            Op::Continue { offset: 0x400002, size: 0x20000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "conv2.u191", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "u133.bin", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "voice.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RK: RomSetDef = RomSetDef { name: "sf2rk", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "w6.u222", offset: 0x0, size: 0x80000, crc: 0x49422B6F },
            Op::Load { kind: Load::Byte16, file: "w5.u196", offset: 0x1, size: 0x80000, crc: 0x7E9C8C2F },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "01 rk098", offset: 0x0, size: 0x80000, crc: 0x4296DE4D },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "03 rk097", offset: 0x2, size: 0x80000, crc: 0x16CF11D0 },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "02 rk037", offset: 0x200000, size: 0x80000, crc: 0x68CA7FCE },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "04 rk033", offset: 0x200002, size: 0x80000, crc: 0x9F46F926 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "05 rk116", offset: 0x400000, size: 0x80000, crc: 0x4C161FA9 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "06 rk077", offset: 0x400002, size: 0x80000, crc: 0xEC949F8C },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "w-1", offset: 0x400004, size: 0x10000, crc: 0x6DE44671 },
            Op::Continue { offset: 0x400000, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "w-3", offset: 0x400005, size: 0x10000, crc: 0xBF0CD819 },
            Op::Continue { offset: 0x400001, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "w-2", offset: 0x400006, size: 0x10000, crc: 0xE8F14362 },
            Op::Continue { offset: 0x400002, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "w-4", offset: 0x400007, size: 0x10000, crc: 0x76F9F91F },
            Op::Continue { offset: 0x400003, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "1.rk", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "2.rk", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "w7.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : aboardplds
pub const RS_SF2QP1: RomSetDef = RomSetDef { name: "sf2qp1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "stfii-qkn-cps-17.33", offset: 0x0, size: 0x40000, crc: 0x3A9458EE },
            Op::Continue { offset: 0xc0000, size: 0x40000 },
            Op::Load { kind: Load::Word16Swap, file: "stfii-qkn-cps-17.34", offset: 0x80000, size: 0x40000, crc: 0x4ED215D8 },
            Op::Continue { offset: 0x40000, size: 0x40000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.bin", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.bin", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.bin", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.bin", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.bin", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.bin", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.bin", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.bin", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.bin", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.bin", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.bin", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.bin", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.bin", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds
pub const RS_SF2QP2: RomSetDef = RomSetDef { name: "sf2qp2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "qkn.33", offset: 0x0, size: 0x40000, crc: 0x43AA343D },
            Op::Continue { offset: 0xc0000, size: 0x40000 },
            Op::Load { kind: Load::Word16Swap, file: "qkn.34", offset: 0x80000, size: 0x40000, crc: 0xD03B553F },
            Op::Continue { offset: 0x40000, size: 0x40000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.bin", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.bin", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.bin", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.bin", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.bin", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.bin", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.bin", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.bin", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.bin", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.bin", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.bin", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.bin", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.bin", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds
pub const RS_SF2THNDR: RomSetDef = RomSetDef { name: "sf2thndr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "17_30.11e", offset: 0x0, size: 0x20000, crc: 0xD3CD6D18 },
            Op::Load { kind: Load::Byte16, file: "17_37.11f", offset: 0x1, size: 0x20000, crc: 0xE892716E },
            Op::Load { kind: Load::Byte16, file: "sf2u_31b.12e", offset: 0x40000, size: 0x20000, crc: 0xA673143D },
            Op::Load { kind: Load::Byte16, file: "sf2u_38b.12f", offset: 0x40001, size: 0x20000, crc: 0x4C2CCEF7 },
            Op::Load { kind: Load::Byte16, file: "sf2u_28b.9e", offset: 0x80000, size: 0x20000, crc: 0x4009955E },
            Op::Load { kind: Load::Byte16, file: "sf2u_35b.9f", offset: 0x80001, size: 0x20000, crc: 0x8C1F3994 },
            Op::Load { kind: Load::Byte16, file: "17_29.10e", offset: 0xc0000, size: 0x20000, crc: 0x8830B54D },
            Op::Load { kind: Load::Byte16, file: "17_36.10f", offset: 0xc0001, size: 0x20000, crc: 0x3F13ADA3 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.bin", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.bin", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.bin", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.bin", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.bin", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.bin", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.bin", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.bin", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.bin", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.bin", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.bin", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.bin", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.bin", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds
pub const RS_SF2THNDR2: RomSetDef = RomSetDef { name: "sf2thndr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "31.bin", offset: 0x0, size: 0x20000, crc: 0xEEC8CBE3 },
            Op::Load { kind: Load::Byte16, file: "38.bin", offset: 0x1, size: 0x20000, crc: 0x1C0FC4E1 },
            Op::Load { kind: Load::Byte16, file: "30.bin", offset: 0x40000, size: 0x20000, crc: 0xA673143D },
            Op::Load { kind: Load::Byte16, file: "37.bin", offset: 0x40001, size: 0x20000, crc: 0x4C2CCEF7 },
            Op::Load { kind: Load::Byte16, file: "28.bin", offset: 0x80000, size: 0x20000, crc: 0x4009955E },
            Op::Load { kind: Load::Byte16, file: "35.bin", offset: 0x80001, size: 0x20000, crc: 0x8C1F3994 },
            Op::Load { kind: Load::Byte16, file: "29.bin", offset: 0xc0000, size: 0x20000, crc: 0xBB4AF315 },
            Op::Load { kind: Load::Byte16, file: "36.bin", offset: 0xc0001, size: 0x20000, crc: 0xC02A13EB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sf2_06.bin", offset: 0x0, size: 0x80000, crc: 0x22C9CC8E },
            Op::Load { kind: Load::Word64, file: "sf2_08.bin", offset: 0x2, size: 0x80000, crc: 0x57213BE8 },
            Op::Load { kind: Load::Word64, file: "sf2_05.bin", offset: 0x4, size: 0x80000, crc: 0xBA529B4F },
            Op::Load { kind: Load::Word64, file: "sf2_07.bin", offset: 0x6, size: 0x80000, crc: 0x4B1B33A8 },
            Op::Load { kind: Load::Word64, file: "sf2_15.bin", offset: 0x200000, size: 0x80000, crc: 0x2C7E2229 },
            Op::Load { kind: Load::Word64, file: "sf2_17.bin", offset: 0x200002, size: 0x80000, crc: 0xB5548F17 },
            Op::Load { kind: Load::Word64, file: "sf2_14.bin", offset: 0x200004, size: 0x80000, crc: 0x14B84312 },
            Op::Load { kind: Load::Word64, file: "sf2_16.bin", offset: 0x200006, size: 0x80000, crc: 0x5E9CD89A },
            Op::Load { kind: Load::Word64, file: "sf2_25.bin", offset: 0x400000, size: 0x80000, crc: 0x994BFA58 },
            Op::Load { kind: Load::Word64, file: "sf2_27.bin", offset: 0x400002, size: 0x80000, crc: 0x3E66AD9D },
            Op::Load { kind: Load::Word64, file: "sf2_24.bin", offset: 0x400004, size: 0x80000, crc: 0xC1BEFAA8 },
            Op::Load { kind: Load::Word64, file: "sf2_26.bin", offset: 0x400006, size: 0x80000, crc: 0x0627C831 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_09.bin", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sf2_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "sf2_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_3WONDERS: RomSetDef = RomSetDef { name: "3wonders", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "rte_30a.11f", offset: 0x0, size: 0x20000, crc: 0xEF5B8B33 },
            Op::Load { kind: Load::Byte16, file: "rte_35a.11h", offset: 0x1, size: 0x20000, crc: 0x7D705529 },
            Op::Load { kind: Load::Byte16, file: "rte_31a.12f", offset: 0x40000, size: 0x20000, crc: 0x32835E5E },
            Op::Load { kind: Load::Byte16, file: "rte_36a.12h", offset: 0x40001, size: 0x20000, crc: 0x7637975F },
            Op::Load { kind: Load::Byte16, file: "rt_28a.9f", offset: 0x80000, size: 0x20000, crc: 0x054137C8 },
            Op::Load { kind: Load::Byte16, file: "rt_33a.9h", offset: 0x80001, size: 0x20000, crc: 0x7264CB1B },
            Op::Load { kind: Load::Byte16, file: "rte_29a.10f", offset: 0xc0000, size: 0x20000, crc: 0xCDDAA919 },
            Op::Load { kind: Load::Byte16, file: "rte_34a.10h", offset: 0xc0001, size: 0x20000, crc: 0xED52E7E5 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "rt-5m.7a", offset: 0x0, size: 0x80000, crc: 0x86AEF804 },
            Op::Load { kind: Load::Word64, file: "rt-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4F057110 },
            Op::Load { kind: Load::Word64, file: "rt-1m.3a", offset: 0x4, size: 0x80000, crc: 0x902489D0 },
            Op::Load { kind: Load::Word64, file: "rt-3m.5a", offset: 0x6, size: 0x80000, crc: 0xE35CE720 },
            Op::Load { kind: Load::Word64, file: "rt-6m.8a", offset: 0x200000, size: 0x80000, crc: 0x13CB0E7C },
            Op::Load { kind: Load::Word64, file: "rt-8m.10a", offset: 0x200002, size: 0x80000, crc: 0x1F055014 },
            Op::Load { kind: Load::Word64, file: "rt-2m.4a", offset: 0x200004, size: 0x80000, crc: 0xE9A034F4 },
            Op::Load { kind: Load::Word64, file: "rt-4m.6a", offset: 0x200006, size: 0x80000, crc: 0xDF0EEA8B },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_9.12b", offset: 0x0, size: 0x8000, crc: 0xABFCA165 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_18.11c", offset: 0x0, size: 0x20000, crc: 0x26B211AB },
            Op::Load { kind: Load::Byte, file: "rt_19.12c", offset: 0x20000, size: 0x20000, crc: 0xDBE64AD0 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_3WONDERSR1: RomSetDef = RomSetDef { name: "3wondersr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "rte_30.11f", offset: 0x0, size: 0x20000, crc: 0x0D541519 },
            Op::Load { kind: Load::Byte16, file: "rte_35.11h", offset: 0x1, size: 0x20000, crc: 0x73DD0E20 },
            Op::Load { kind: Load::Byte16, file: "rte_31.12f", offset: 0x40000, size: 0x20000, crc: 0x33E0337D },
            Op::Load { kind: Load::Byte16, file: "rte_36.12h", offset: 0x40001, size: 0x20000, crc: 0xA8865243 },
            Op::Load { kind: Load::Byte16, file: "rte_28.9f", offset: 0x80000, size: 0x20000, crc: 0x054137C8 },
            Op::Load { kind: Load::Byte16, file: "rte_33.9h", offset: 0x80001, size: 0x20000, crc: 0x7264CB1B },
            Op::Load { kind: Load::Byte16, file: "rte_29.10f", offset: 0xc0000, size: 0x20000, crc: 0x9A8DF1E4 },
            Op::Load { kind: Load::Byte16, file: "rte_34.10h", offset: 0xc0001, size: 0x20000, crc: 0x6348A79D },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "rt-5m.7a", offset: 0x0, size: 0x80000, crc: 0x86AEF804 },
            Op::Load { kind: Load::Word64, file: "rt-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4F057110 },
            Op::Load { kind: Load::Word64, file: "rt-1m.3a", offset: 0x4, size: 0x80000, crc: 0x902489D0 },
            Op::Load { kind: Load::Word64, file: "rt-3m.5a", offset: 0x6, size: 0x80000, crc: 0xE35CE720 },
            Op::Load { kind: Load::Word64, file: "rt-6m.8a", offset: 0x200000, size: 0x80000, crc: 0x13CB0E7C },
            Op::Load { kind: Load::Word64, file: "rt-8m.10a", offset: 0x200002, size: 0x80000, crc: 0x1F055014 },
            Op::Load { kind: Load::Word64, file: "rt-2m.4a", offset: 0x200004, size: 0x80000, crc: 0xE9A034F4 },
            Op::Load { kind: Load::Word64, file: "rt-4m.6a", offset: 0x200006, size: 0x80000, crc: 0xDF0EEA8B },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_9.12b", offset: 0x0, size: 0x8000, crc: 0x7D5A77A7 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_18.11c", offset: 0x0, size: 0x20000, crc: 0xF6DC0D3D },
            Op::Load { kind: Load::Byte, file: "rt_19.12c", offset: 0x20000, size: 0x20000, crc: 0xDBE64AD0 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_3WONDERSU: RomSetDef = RomSetDef { name: "3wondersu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "rtu_30a.11f", offset: 0x0, size: 0x20000, crc: 0x0B156FD8 },
            Op::Load { kind: Load::Byte16, file: "rtu_35a.11h", offset: 0x1, size: 0x20000, crc: 0x57350BF4 },
            Op::Load { kind: Load::Byte16, file: "rtu_31a.12f", offset: 0x40000, size: 0x20000, crc: 0x0E723FCC },
            Op::Load { kind: Load::Byte16, file: "rtu_36a.12h", offset: 0x40001, size: 0x20000, crc: 0x523A45DC },
            Op::Load { kind: Load::Byte16, file: "rt_28a.9f", offset: 0x80000, size: 0x20000, crc: 0x054137C8 },
            Op::Load { kind: Load::Byte16, file: "rt_33a.9h", offset: 0x80001, size: 0x20000, crc: 0x7264CB1B },
            Op::Load { kind: Load::Byte16, file: "rtu_29a.10f", offset: 0xc0000, size: 0x20000, crc: 0x37BA3E20 },
            Op::Load { kind: Load::Byte16, file: "rtu_34a.10h", offset: 0xc0001, size: 0x20000, crc: 0xF99F46C0 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "rt-5m.7a", offset: 0x0, size: 0x80000, crc: 0x86AEF804 },
            Op::Load { kind: Load::Word64, file: "rt-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4F057110 },
            Op::Load { kind: Load::Word64, file: "rt-1m.3a", offset: 0x4, size: 0x80000, crc: 0x902489D0 },
            Op::Load { kind: Load::Word64, file: "rt-3m.5a", offset: 0x6, size: 0x80000, crc: 0xE35CE720 },
            Op::Load { kind: Load::Word64, file: "rt-6m.8a", offset: 0x200000, size: 0x80000, crc: 0x13CB0E7C },
            Op::Load { kind: Load::Word64, file: "rt-8m.10a", offset: 0x200002, size: 0x80000, crc: 0x1F055014 },
            Op::Load { kind: Load::Word64, file: "rt-2m.4a", offset: 0x200004, size: 0x80000, crc: 0xE9A034F4 },
            Op::Load { kind: Load::Word64, file: "rt-4m.6a", offset: 0x200006, size: 0x80000, crc: 0xDF0EEA8B },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_9.12b", offset: 0x0, size: 0x8000, crc: 0xABFCA165 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_18.11c", offset: 0x0, size: 0x20000, crc: 0x26B211AB },
            Op::Load { kind: Load::Byte, file: "rt_19.12c", offset: 0x20000, size: 0x20000, crc: 0xDBE64AD0 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_WONDER3: RomSetDef = RomSetDef { name: "wonder3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "rtj_36.12f", offset: 0x0, size: 0x20000, crc: 0xE3741247 },
            Op::Load { kind: Load::Byte16, file: "rtj_42.12h", offset: 0x1, size: 0x20000, crc: 0xB4BAA117 },
            Op::Load { kind: Load::Byte16, file: "rtj_37.13f", offset: 0x40000, size: 0x20000, crc: 0xA1F677B0 },
            Op::Load { kind: Load::Byte16, file: "rtj_43.13h", offset: 0x40001, size: 0x20000, crc: 0x85337A47 },
            Op::Load { kind: Load::Byte16, file: "rt_34.10f", offset: 0x80000, size: 0x20000, crc: 0x054137C8 },
            Op::Load { kind: Load::Byte16, file: "rt_40.10h", offset: 0x80001, size: 0x20000, crc: 0x7264CB1B },
            Op::Load { kind: Load::Byte16, file: "rtj_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xE72F9EA3 },
            Op::Load { kind: Load::Byte16, file: "rtj_41.11h", offset: 0xc0001, size: 0x20000, crc: 0xA11EE998 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "rt_09.4b", offset: 0x0, size: 0x20000, crc: 0x2C40E480 },
            Op::Load { kind: Load::Byte64, file: "rt_01.4a", offset: 0x1, size: 0x20000, crc: 0x3E11F8CD },
            Op::Load { kind: Load::Byte64, file: "rt_13.9b", offset: 0x2, size: 0x20000, crc: 0x51009117 },
            Op::Load { kind: Load::Byte64, file: "rt_05.9a", offset: 0x3, size: 0x20000, crc: 0x283FD470 },
            Op::Load { kind: Load::Byte64, file: "rt_24.5e", offset: 0x4, size: 0x20000, crc: 0xEE4484CE },
            Op::Load { kind: Load::Byte64, file: "rt_17.5c", offset: 0x5, size: 0x20000, crc: 0xE5DCDDEB },
            Op::Load { kind: Load::Byte64, file: "rt_38.8h", offset: 0x6, size: 0x20000, crc: 0xB2940C2D },
            Op::Load { kind: Load::Byte64, file: "rt_32.8f", offset: 0x7, size: 0x20000, crc: 0x08E2B758 },
            Op::Load { kind: Load::Byte64, file: "rt_10.5b", offset: 0x100000, size: 0x20000, crc: 0xE3F3FF94 },
            Op::Load { kind: Load::Byte64, file: "rt_02.5a", offset: 0x100001, size: 0x20000, crc: 0xFCFFD73C },
            Op::Load { kind: Load::Byte64, file: "rt_14.10b", offset: 0x100002, size: 0x20000, crc: 0x5C546D9A },
            Op::Load { kind: Load::Byte64, file: "rt_06.10a", offset: 0x100003, size: 0x20000, crc: 0xD9650BC4 },
            Op::Load { kind: Load::Byte64, file: "rt_25.7e", offset: 0x100004, size: 0x20000, crc: 0x11B28831 },
            Op::Load { kind: Load::Byte64, file: "rt_18.7c", offset: 0x100005, size: 0x20000, crc: 0xCE1AFB7C },
            Op::Load { kind: Load::Byte64, file: "rt_39.9h", offset: 0x100006, size: 0x20000, crc: 0xEA7AC9EE },
            Op::Load { kind: Load::Byte64, file: "rt_33.9f", offset: 0x100007, size: 0x20000, crc: 0xD6A99384 },
            Op::Load { kind: Load::Byte64, file: "rt_11.7b", offset: 0x200000, size: 0x20000, crc: 0x04F3C298 },
            Op::Load { kind: Load::Byte64, file: "rt_03.7a", offset: 0x200001, size: 0x20000, crc: 0x98087E08 },
            Op::Load { kind: Load::Byte64, file: "rt_15.11b", offset: 0x200002, size: 0x20000, crc: 0xB6ABA565 },
            Op::Load { kind: Load::Byte64, file: "rt_07.11a", offset: 0x200003, size: 0x20000, crc: 0xC62DEFA1 },
            Op::Load { kind: Load::Byte64, file: "rt_26.8e", offset: 0x200004, size: 0x20000, crc: 0x532F542E },
            Op::Load { kind: Load::Byte64, file: "rt_19.8c", offset: 0x200005, size: 0x20000, crc: 0x1F0F72BD },
            Op::Load { kind: Load::Byte64, file: "rt_28.10e", offset: 0x200006, size: 0x20000, crc: 0x6064E499 },
            Op::Load { kind: Load::Byte64, file: "rt_21.10c", offset: 0x200007, size: 0x20000, crc: 0x20012DDC },
            Op::Load { kind: Load::Byte64, file: "rt_12.8b", offset: 0x300000, size: 0x20000, crc: 0xE54664CC },
            Op::Load { kind: Load::Byte64, file: "rt_04.8a", offset: 0x300001, size: 0x20000, crc: 0x4D7B9A1A },
            Op::Load { kind: Load::Byte64, file: "rt_16.12b", offset: 0x300002, size: 0x20000, crc: 0x37C96CFC },
            Op::Load { kind: Load::Byte64, file: "rt_08.12a", offset: 0x300003, size: 0x20000, crc: 0x75F4975B },
            Op::Load { kind: Load::Byte64, file: "rt_27.9e", offset: 0x300004, size: 0x20000, crc: 0xEC6EDC0F },
            Op::Load { kind: Load::Byte64, file: "rt_20.9c", offset: 0x300005, size: 0x20000, crc: 0x4FE52659 },
            Op::Load { kind: Load::Byte64, file: "rt_29.11e", offset: 0x300006, size: 0x20000, crc: 0x8FA77F9F },
            Op::Load { kind: Load::Byte64, file: "rt_22.11c", offset: 0x300007, size: 0x20000, crc: 0x228A0D4A },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_23.13b", offset: 0x0, size: 0x8000, crc: 0xABFCA165 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_30.12c", offset: 0x0, size: 0x20000, crc: 0x26B211AB },
            Op::Load { kind: Load::Byte, file: "rt_31.13c", offset: 0x20000, size: 0x20000, crc: 0xDBE64AD0 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_3WONDERSB: RomSetDef = RomSetDef { name: "3wondersb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "274001.4", offset: 0x0, size: 0x80000, crc: 0x47887CF3 },
            Op::Load { kind: Load::Byte16, file: "274001.3", offset: 0x1, size: 0x80000, crc: 0xE79EACB3 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "274001.12", offset: 0x0, size: 0x80000, crc: 0x47CF8DFB },
            Op::Load { kind: Load::Byte64, file: "274001.8", offset: 0x1, size: 0x80000, crc: 0x3BC2EF5E },
            Op::Load { kind: Load::Byte64, file: "274001.10", offset: 0x2, size: 0x80000, crc: 0xA0D27605 },
            Op::Load { kind: Load::Byte64, file: "274001.6", offset: 0x3, size: 0x80000, crc: 0x312D790C },
            Op::Load { kind: Load::Byte64, file: "274001.11", offset: 0x4, size: 0x80000, crc: 0x8112BBB4 },
            Op::Load { kind: Load::Byte64, file: "274001.7", offset: 0x5, size: 0x80000, crc: 0x58307167 },
            Op::Load { kind: Load::Byte64, file: "274001.9", offset: 0x6, size: 0x80000, crc: 0xCB73759D },
            Op::Load { kind: Load::Byte64, file: "274001.5", offset: 0x7, size: 0x80000, crc: 0x3F765AE8 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.2", offset: 0x0, size: 0x8000, crc: 0xABFCA165 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27020.1", offset: 0x0, size: 0x40000, crc: 0x3C4348CF },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_3WONDERSBI: RomSetDef = RomSetDef { name: "3wondersbi", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "3.u11", offset: 0x0, size: 0x80000, crc: 0x65AE8C7E },
            Op::Load { kind: Load::Byte16, file: "4.u8", offset: 0x1, size: 0x80000, crc: 0x67A60FE6 },
        ] },
        RegionDef { name: "gfx", size: 0x500000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "5.u140", offset: 0x0, size: 0x80000, crc: 0x156C487B },
            Op::Load { kind: Load::Byte, file: "6.u128", offset: 0x80000, size: 0x80000, crc: 0xB671F752 },
            Op::Load { kind: Load::Byte, file: "7.u126", offset: 0x100000, size: 0x80000, crc: 0x2F835213 },
            Op::Load { kind: Load::Byte, file: "8.u139", offset: 0x180000, size: 0x80000, crc: 0xE9421CBA },
            Op::Load { kind: Load::Byte, file: "9.u127", offset: 0x200000, size: 0x80000, crc: 0x8D08103C },
            Op::Load { kind: Load::Byte, file: "10.u125", offset: 0x280000, size: 0x80000, crc: 0xDA529062 },
            Op::Load { kind: Load::Byte, file: "11.u138", offset: 0x300000, size: 0x80000, crc: 0x2007B45A },
            Op::Load { kind: Load::Byte, file: "12.u137", offset: 0x380000, size: 0x80000, crc: 0x7DFF1790 },
            Op::Load { kind: Load::Byte, file: "13.u130", offset: 0x400000, size: 0x80000, crc: 0x1054D40C },
            Op::Load { kind: Load::Byte, file: "14.u129", offset: 0x480000, size: 0x80000, crc: 0x2E32610B },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "1.u18", offset: 0x0, size: 0x8000, crc: 0x9F4017FB },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "2.u24", offset: 0x0, size: 0x40000, crc: 0xDB01F9FE },
            Op::Continue { offset: 0x0, size: 0x40000 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_3WONDERSH: RomSetDef = RomSetDef { name: "3wondersh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "22.bin", offset: 0x0, size: 0x20000, crc: 0xE6071884 },
            Op::Load { kind: Load::Byte16, file: "26.bin", offset: 0x1, size: 0x20000, crc: 0xA28447B7 },
            Op::Load { kind: Load::Byte16, file: "23.bin", offset: 0x40000, size: 0x20000, crc: 0xFD3D6509 },
            Op::Load { kind: Load::Byte16, file: "27.bin", offset: 0x40001, size: 0x20000, crc: 0x999CBA3D },
            Op::Load { kind: Load::Byte16, file: "rt_28a.9f", offset: 0x80000, size: 0x20000, crc: 0x054137C8 },
            Op::Load { kind: Load::Byte16, file: "rt_33a.9h", offset: 0x80001, size: 0x20000, crc: 0x7264CB1B },
            Op::Load { kind: Load::Byte16, file: "rte_29a.10f", offset: 0xc0000, size: 0x20000, crc: 0xCDDAA919 },
            Op::Load { kind: Load::Byte16, file: "rte_34a.10h", offset: 0xc0001, size: 0x20000, crc: 0xED52E7E5 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "16.bin", offset: 0x0, size: 0x40000, crc: 0xC997BCA2 },
            Op::Load { kind: Load::Byte64, file: "6.bin", offset: 0x1, size: 0x40000, crc: 0x3EEA321A },
            Op::Load { kind: Load::Byte64, file: "18.bin", offset: 0x2, size: 0x40000, crc: 0x98ACDFD4 },
            Op::Load { kind: Load::Byte64, file: "8.bin", offset: 0x3, size: 0x40000, crc: 0xDC9CA6F9 },
            Op::Load { kind: Load::Byte64, file: "12.bin", offset: 0x4, size: 0x40000, crc: 0x0D8A6007 },
            Op::Load { kind: Load::Byte64, file: "2.bin", offset: 0x5, size: 0x40000, crc: 0xD75563B9 },
            Op::Load { kind: Load::Byte64, file: "14.bin", offset: 0x6, size: 0x40000, crc: 0x84369A28 },
            Op::Load { kind: Load::Byte64, file: "4.bin", offset: 0x7, size: 0x40000, crc: 0xD4831578 },
            Op::Load { kind: Load::Byte64, file: "17.bin", offset: 0x200000, size: 0x40000, crc: 0x040EDFF5 },
            Op::Load { kind: Load::Byte64, file: "7.bin", offset: 0x200001, size: 0x40000, crc: 0xC7C0468C },
            Op::Load { kind: Load::Byte64, file: "19.bin", offset: 0x200002, size: 0x40000, crc: 0x9FEF114F },
            Op::Load { kind: Load::Byte64, file: "9.bin", offset: 0x200003, size: 0x40000, crc: 0x48CBFBA5 },
            Op::Load { kind: Load::Byte64, file: "13.bin", offset: 0x200004, size: 0x40000, crc: 0x8FC3D7D1 },
            Op::Load { kind: Load::Byte64, file: "3.bin", offset: 0x200005, size: 0x40000, crc: 0xC65E9A86 },
            Op::Load { kind: Load::Byte64, file: "15.bin", offset: 0x200006, size: 0x40000, crc: 0xF239341A },
            Op::Load { kind: Load::Byte64, file: "5.bin", offset: 0x200007, size: 0x40000, crc: 0x947AC944 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_9.12b", offset: 0x0, size: 0x8000, crc: 0xABFCA165 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rt_18.11c", offset: 0x0, size: 0x20000, crc: 0x26B211AB },
            Op::Load { kind: Load::Byte, file: "rt_19.12c", offset: 0x20000, size: 0x20000, crc: 0xDBE64AD0 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KOD: RomSetDef = RomSetDef { name: "kod", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "kde_30a.11e", offset: 0x0, size: 0x20000, crc: 0xFCB5EFE2 },
            Op::Load { kind: Load::Byte16, file: "kde_37a.11f", offset: 0x1, size: 0x20000, crc: 0xF22E5266 },
            Op::Load { kind: Load::Byte16, file: "kde_31a.12e", offset: 0x40000, size: 0x20000, crc: 0xC710D722 },
            Op::Load { kind: Load::Byte16, file: "kde_38a.12f", offset: 0x40001, size: 0x20000, crc: 0x57D6ED3A },
            Op::Load { kind: Load::Byte16, file: "kd_28.9e", offset: 0x80000, size: 0x20000, crc: 0x9367BCD9 },
            Op::Load { kind: Load::Byte16, file: "kd_35.9f", offset: 0x80001, size: 0x20000, crc: 0x4CA6A48A },
            Op::Load { kind: Load::Byte16, file: "kd_29.10e", offset: 0xc0000, size: 0x20000, crc: 0x0360FA72 },
            Op::Load { kind: Load::Byte16, file: "kd_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0x95A3CEF8 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kd-5m.4a", offset: 0x0, size: 0x80000, crc: 0xE45B8701 },
            Op::Load { kind: Load::Word64, file: "kd-7m.6a", offset: 0x2, size: 0x80000, crc: 0xA7750322 },
            Op::Load { kind: Load::Word64, file: "kd-1m.3a", offset: 0x4, size: 0x80000, crc: 0x5F74BF78 },
            Op::Load { kind: Load::Word64, file: "kd-3m.5a", offset: 0x6, size: 0x80000, crc: 0x5E5303BF },
            Op::Load { kind: Load::Word64, file: "kd-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x113358F3 },
            Op::Load { kind: Load::Word64, file: "kd-8m.6c", offset: 0x200002, size: 0x80000, crc: 0x38853C44 },
            Op::Load { kind: Load::Word64, file: "kd-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x9EF36604 },
            Op::Load { kind: Load::Word64, file: "kd-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x402B9B4F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_9.12a", offset: 0x0, size: 0x8000, crc: 0xBAC6EC26 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_18.11c", offset: 0x0, size: 0x20000, crc: 0x4C63181D },
            Op::Load { kind: Load::Byte, file: "kd_19.12c", offset: 0x20000, size: 0x20000, crc: 0x92941B80 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KODR1: RomSetDef = RomSetDef { name: "kodr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "kde_30.11e", offset: 0x0, size: 0x20000, crc: 0xC7414FD4 },
            Op::Load { kind: Load::Byte16, file: "kde_37.11f", offset: 0x1, size: 0x20000, crc: 0xA5BF40D2 },
            Op::Load { kind: Load::Byte16, file: "kde_31.12e", offset: 0x40000, size: 0x20000, crc: 0x1FFFC7BD },
            Op::Load { kind: Load::Byte16, file: "kde_38.12f", offset: 0x40001, size: 0x20000, crc: 0x89E57A82 },
            Op::Load { kind: Load::Byte16, file: "kde_28.9e", offset: 0x80000, size: 0x20000, crc: 0x9367BCD9 },
            Op::Load { kind: Load::Byte16, file: "kde_35.9f", offset: 0x80001, size: 0x20000, crc: 0x4CA6A48A },
            Op::Load { kind: Load::Byte16, file: "kde_29.10e", offset: 0xc0000, size: 0x20000, crc: 0x6A0BA878 },
            Op::Load { kind: Load::Byte16, file: "kde_36.10f", offset: 0xc0001, size: 0x20000, crc: 0xB509B39D },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kd-5m.4a", offset: 0x0, size: 0x80000, crc: 0xE45B8701 },
            Op::Load { kind: Load::Word64, file: "kd-7m.6a", offset: 0x2, size: 0x80000, crc: 0xA7750322 },
            Op::Load { kind: Load::Word64, file: "kd-1m.3a", offset: 0x4, size: 0x80000, crc: 0x5F74BF78 },
            Op::Load { kind: Load::Word64, file: "kd-3m.5a", offset: 0x6, size: 0x80000, crc: 0x5E5303BF },
            Op::Load { kind: Load::Word64, file: "kd-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x113358F3 },
            Op::Load { kind: Load::Word64, file: "kd-8m.6c", offset: 0x200002, size: 0x80000, crc: 0x38853C44 },
            Op::Load { kind: Load::Word64, file: "kd-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x9EF36604 },
            Op::Load { kind: Load::Word64, file: "kd-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x402B9B4F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_9.12a", offset: 0x0, size: 0x8000, crc: 0xF5514510 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_18.11c", offset: 0x0, size: 0x20000, crc: 0x69ECB2C8 },
            Op::Load { kind: Load::Byte, file: "kd_19.12c", offset: 0x20000, size: 0x20000, crc: 0x02D851C1 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KODR2: RomSetDef = RomSetDef { name: "kodr2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "kde_30.11e", offset: 0x0, size: 0x20000, crc: 0xF8DC4CE3 },
            Op::Load { kind: Load::Byte16, file: "kde_37.11f", offset: 0x1, size: 0x20000, crc: 0xD1276C1C },
            Op::Load { kind: Load::Byte16, file: "kde_31.12e", offset: 0x40000, size: 0x20000, crc: 0x309DEBD8 },
            Op::Load { kind: Load::Byte16, file: "kde_38.12f", offset: 0x40001, size: 0x20000, crc: 0x76CD5738 },
            Op::Load { kind: Load::Byte16, file: "kd_28.9e", offset: 0x80000, size: 0x20000, crc: 0x9367BCD9 },
            Op::Load { kind: Load::Byte16, file: "kd_35.9f", offset: 0x80001, size: 0x20000, crc: 0x4CA6A48A },
            Op::Load { kind: Load::Byte16, file: "kd_29.10e", offset: 0xc0000, size: 0x20000, crc: 0x0360FA72 },
            Op::Load { kind: Load::Byte16, file: "kd_36.10f", offset: 0xc0001, size: 0x20000, crc: 0x3C66C32B },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kd-5m.4a", offset: 0x0, size: 0x80000, crc: 0xE45B8701 },
            Op::Load { kind: Load::Word64, file: "kd-7m.6a", offset: 0x2, size: 0x80000, crc: 0xA7750322 },
            Op::Load { kind: Load::Word64, file: "kd-1m.3a", offset: 0x4, size: 0x80000, crc: 0x5F74BF78 },
            Op::Load { kind: Load::Word64, file: "kd-3m.5a", offset: 0x6, size: 0x80000, crc: 0x5E5303BF },
            Op::Load { kind: Load::Word64, file: "kd-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x113358F3 },
            Op::Load { kind: Load::Word64, file: "kd-8m.6c", offset: 0x200002, size: 0x80000, crc: 0x38853C44 },
            Op::Load { kind: Load::Word64, file: "kd-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x9EF36604 },
            Op::Load { kind: Load::Word64, file: "kd-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x402B9B4F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_9.12a", offset: 0x0, size: 0x8000, crc: 0xBAC6EC26 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_18.11c", offset: 0x0, size: 0x20000, crc: 0x4C63181D },
            Op::Load { kind: Load::Byte, file: "kd_19.12c", offset: 0x20000, size: 0x20000, crc: 0x92941B80 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KODU: RomSetDef = RomSetDef { name: "kodu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "kdu_30b.11e", offset: 0x0, size: 0x20000, crc: 0x825817F9 },
            Op::Load { kind: Load::Byte16, file: "kdu_37b.11f", offset: 0x1, size: 0x20000, crc: 0xD2422DFB },
            Op::Load { kind: Load::Byte16, file: "kdu_31b.12e", offset: 0x40000, size: 0x20000, crc: 0x9AF36039 },
            Op::Load { kind: Load::Byte16, file: "kdu_38b.12f", offset: 0x40001, size: 0x20000, crc: 0xBE8405A1 },
            Op::Load { kind: Load::Byte16, file: "kdu_28.9e", offset: 0x80000, size: 0x20000, crc: 0x9367BCD9 },
            Op::Load { kind: Load::Byte16, file: "kdu_35.9f", offset: 0x80001, size: 0x20000, crc: 0x4CA6A48A },
            Op::Load { kind: Load::Byte16, file: "kdu_29.10e", offset: 0xc0000, size: 0x20000, crc: 0x0360FA72 },
            Op::Load { kind: Load::Byte16, file: "kdu_36a.10f", offset: 0xc0001, size: 0x20000, crc: 0x95A3CEF8 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kd-5m.4a", offset: 0x0, size: 0x80000, crc: 0xE45B8701 },
            Op::Load { kind: Load::Word64, file: "kd-7m.6a", offset: 0x2, size: 0x80000, crc: 0xA7750322 },
            Op::Load { kind: Load::Word64, file: "kd-1m.3a", offset: 0x4, size: 0x80000, crc: 0x5F74BF78 },
            Op::Load { kind: Load::Word64, file: "kd-3m.5a", offset: 0x6, size: 0x80000, crc: 0x5E5303BF },
            Op::Load { kind: Load::Word64, file: "kd-6m.4c", offset: 0x200000, size: 0x80000, crc: 0x113358F3 },
            Op::Load { kind: Load::Word64, file: "kd-8m.6c", offset: 0x200002, size: 0x80000, crc: 0x38853C44 },
            Op::Load { kind: Load::Word64, file: "kd-2m.3c", offset: 0x200004, size: 0x80000, crc: 0x9EF36604 },
            Op::Load { kind: Load::Word64, file: "kd-4m.5c", offset: 0x200006, size: 0x80000, crc: 0x402B9B4F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_09.12a", offset: 0x0, size: 0x8000, crc: 0xBAC6EC26 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_18.11c", offset: 0x0, size: 0x20000, crc: 0x4C63181D },
            Op::Load { kind: Load::Byte, file: "kd_19.12c", offset: 0x20000, size: 0x20000, crc: 0x92941B80 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KODJ: RomSetDef = RomSetDef { name: "kodj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "kdj_30a.11e", offset: 0x0, size: 0x20000, crc: 0xEBC788AD },
            Op::Load { kind: Load::Byte16, file: "kdj_37a.11f", offset: 0x1, size: 0x20000, crc: 0xE55C3529 },
            Op::Load { kind: Load::Byte16, file: "kdj_31a.12e", offset: 0x40000, size: 0x20000, crc: 0xC710D722 },
            Op::Load { kind: Load::Byte16, file: "kdj_38a.12f", offset: 0x40001, size: 0x20000, crc: 0x57D6ED3A },
            Op::Load { kind: Load::Word16Swap, file: "kd_33.6f", offset: 0x80000, size: 0x80000, crc: 0x9BD7AD4B },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kd_06.8a", offset: 0x0, size: 0x80000, crc: 0xE45B8701 },
            Op::Load { kind: Load::Word64, file: "kd_08.10a", offset: 0x2, size: 0x80000, crc: 0xA7750322 },
            Op::Load { kind: Load::Word64, file: "kd_05.7a", offset: 0x4, size: 0x80000, crc: 0x5F74BF78 },
            Op::Load { kind: Load::Word64, file: "kd_07.9a", offset: 0x6, size: 0x80000, crc: 0x5E5303BF },
            Op::Load { kind: Load::Word64, file: "kd_15.8c", offset: 0x200000, size: 0x80000, crc: 0x113358F3 },
            Op::Load { kind: Load::Word64, file: "kd_17.10c", offset: 0x200002, size: 0x80000, crc: 0x38853C44 },
            Op::Load { kind: Load::Word64, file: "kd_14.7c", offset: 0x200004, size: 0x80000, crc: 0x9EF36604 },
            Op::Load { kind: Load::Word64, file: "kd_16.9c", offset: 0x200006, size: 0x80000, crc: 0x402B9B4F },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_09.12a", offset: 0x0, size: 0x8000, crc: 0xBAC6EC26 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_18.11c", offset: 0x0, size: 0x20000, crc: 0x4C63181D },
            Op::Load { kind: Load::Byte, file: "kd_19.12c", offset: 0x20000, size: 0x20000, crc: 0x92941B80 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KODJA: RomSetDef = RomSetDef { name: "kodja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "kdj_36a.12f", offset: 0x0, size: 0x20000, crc: 0xEBC788AD },
            Op::Load { kind: Load::Byte16, file: "kdj_42a.12h", offset: 0x1, size: 0x20000, crc: 0xE55C3529 },
            Op::Load { kind: Load::Byte16, file: "kdj_37a.13f", offset: 0x40000, size: 0x20000, crc: 0xC710D722 },
            Op::Load { kind: Load::Byte16, file: "kdj_43a.13h", offset: 0x40001, size: 0x20000, crc: 0x57D6ED3A },
            Op::Load { kind: Load::Byte16, file: "kd_34.10f", offset: 0x80000, size: 0x20000, crc: 0x9367BCD9 },
            Op::Load { kind: Load::Byte16, file: "kd_40.10h", offset: 0x80001, size: 0x20000, crc: 0x4CA6A48A },
            Op::Load { kind: Load::Byte16, file: "kd_35.11f", offset: 0xc0000, size: 0x20000, crc: 0x0360FA72 },
            Op::Load { kind: Load::Byte16, file: "kd_41a.11h", offset: 0xc0001, size: 0x20000, crc: 0x95A3CEF8 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "kd_9.4b", offset: 0x0, size: 0x20000, crc: 0x401A98E3 },
            Op::Load { kind: Load::Byte64, file: "kd_1.4a", offset: 0x1, size: 0x20000, crc: 0x5894399A },
            Op::Load { kind: Load::Byte64, file: "kd_13.9b", offset: 0x2, size: 0x20000, crc: 0xB6685131 },
            Op::Load { kind: Load::Byte64, file: "kd_5.9a", offset: 0x3, size: 0x20000, crc: 0xC29B9AB3 },
            Op::Load { kind: Load::Byte64, file: "kd_24.5e", offset: 0x4, size: 0x20000, crc: 0x97008FDB },
            Op::Load { kind: Load::Byte64, file: "kd_17.5c", offset: 0x5, size: 0x20000, crc: 0xDC9A83D3 },
            Op::Load { kind: Load::Byte64, file: "kd_38.8h", offset: 0x6, size: 0x20000, crc: 0x9C3DD2D1 },
            Op::Load { kind: Load::Byte64, file: "kd_32.8f", offset: 0x7, size: 0x20000, crc: 0x1B2A802A },
            Op::Load { kind: Load::Byte64, file: "kd_10.5b", offset: 0x100000, size: 0x20000, crc: 0xE788AE96 },
            Op::Load { kind: Load::Byte64, file: "kd_2.5a", offset: 0x100001, size: 0x20000, crc: 0xB022E3E3 },
            Op::Load { kind: Load::Byte64, file: "kd_14.10b", offset: 0x100002, size: 0x20000, crc: 0x4840C5EF },
            Op::Load { kind: Load::Byte64, file: "kd_6.10a", offset: 0x100003, size: 0x20000, crc: 0x519FAEE4 },
            Op::Load { kind: Load::Byte64, file: "kd_25.7e", offset: 0x100004, size: 0x20000, crc: 0x5D0FA853 },
            Op::Load { kind: Load::Byte64, file: "kd_18.7c", offset: 0x100005, size: 0x20000, crc: 0x6AD3B2BB },
            Op::Load { kind: Load::Byte64, file: "kd_39.9h", offset: 0x100006, size: 0x20000, crc: 0xD7920213 },
            Op::Load { kind: Load::Byte64, file: "kd_33.9f", offset: 0x100007, size: 0x20000, crc: 0x65C2BED6 },
            Op::Load { kind: Load::Byte64, file: "kd_11.7b", offset: 0x200000, size: 0x20000, crc: 0x147E3310 },
            Op::Load { kind: Load::Byte64, file: "kd_3.7a", offset: 0x200001, size: 0x20000, crc: 0x5D18BC83 },
            Op::Load { kind: Load::Byte64, file: "kd_15.11b", offset: 0x200002, size: 0x20000, crc: 0x57359746 },
            Op::Load { kind: Load::Byte64, file: "kd_7.11a", offset: 0x200003, size: 0x20000, crc: 0x7FE03079 },
            Op::Load { kind: Load::Byte64, file: "kd_26.8e", offset: 0x200004, size: 0x20000, crc: 0x57E5FAB5 },
            Op::Load { kind: Load::Byte64, file: "kd_19.8c", offset: 0x200005, size: 0x20000, crc: 0xB1F30F7C },
            Op::Load { kind: Load::Byte64, file: "kd_28.10e", offset: 0x200006, size: 0x20000, crc: 0x3A424135 },
            Op::Load { kind: Load::Byte64, file: "kd_21.10c", offset: 0x200007, size: 0x20000, crc: 0xCE10D2C3 },
            Op::Load { kind: Load::Byte64, file: "kd_12.8b", offset: 0x300000, size: 0x20000, crc: 0xA6042AA2 },
            Op::Load { kind: Load::Byte64, file: "kd_4.8a", offset: 0x300001, size: 0x20000, crc: 0x0CE0BA30 },
            Op::Load { kind: Load::Byte64, file: "kd_16.12b", offset: 0x300002, size: 0x20000, crc: 0x63DCB7E0 },
            Op::Load { kind: Load::Byte64, file: "kd_8.12a", offset: 0x300003, size: 0x20000, crc: 0xC69B77AE },
            Op::Load { kind: Load::Byte64, file: "kd_27.9e", offset: 0x300004, size: 0x20000, crc: 0x40D7BFED },
            Op::Load { kind: Load::Byte64, file: "kd_20.9c", offset: 0x300005, size: 0x20000, crc: 0x01C1F399 },
            Op::Load { kind: Load::Byte64, file: "kd_29.11e", offset: 0x300006, size: 0x20000, crc: 0xA1EEAC03 },
            Op::Load { kind: Load::Byte64, file: "kd_22.11c", offset: 0x300007, size: 0x20000, crc: 0x5ADE98EB },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_23.13b", offset: 0x0, size: 0x8000, crc: 0xBAC6EC26 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kd_30.12c", offset: 0x0, size: 0x20000, crc: 0x4C63181D },
            Op::Load { kind: Load::Byte, file: "kd_31.13c", offset: 0x20000, size: 0x20000, crc: 0x92941B80 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CAPTCOMM: RomSetDef = RomSetDef { name: "captcomm", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cce_23f.8f", offset: 0x0, size: 0x80000, crc: 0x42C814C5 },
            Op::Load { kind: Load::Word16Swap, file: "cc_22f.7f", offset: 0x80000, size: 0x80000, crc: 0x0FD34195 },
            Op::Load { kind: Load::Byte16, file: "cc_24f.9e", offset: 0x100000, size: 0x20000, crc: 0x3A794F25 },
            Op::Load { kind: Load::Byte16, file: "cc_28f.9f", offset: 0x100001, size: 0x20000, crc: 0xFC3C2906 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cc-5m.3a", offset: 0x0, size: 0x80000, crc: 0x7261D8BA },
            Op::Load { kind: Load::Word64, file: "cc-7m.5a", offset: 0x2, size: 0x80000, crc: 0x6A60F949 },
            Op::Load { kind: Load::Word64, file: "cc-1m.4a", offset: 0x4, size: 0x80000, crc: 0x00637302 },
            Op::Load { kind: Load::Word64, file: "cc-3m.6a", offset: 0x6, size: 0x80000, crc: 0xCC87CF61 },
            Op::Load { kind: Load::Word64, file: "cc-6m.7a", offset: 0x200000, size: 0x80000, crc: 0x28718BED },
            Op::Load { kind: Load::Word64, file: "cc-8m.9a", offset: 0x200002, size: 0x80000, crc: 0xD4ACC53A },
            Op::Load { kind: Load::Word64, file: "cc-2m.8a", offset: 0x200004, size: 0x80000, crc: 0x0C69F151 },
            Op::Load { kind: Load::Word64, file: "cc-4m.10a", offset: 0x200006, size: 0x80000, crc: 0x1F9EBB97 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cc_09.11a", offset: 0x0, size: 0x8000, crc: 0x698E8B58 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cc_18.11c", offset: 0x0, size: 0x20000, crc: 0x6DE2C2DB },
            Op::Load { kind: Load::Byte, file: "cc_19.12c", offset: 0x20000, size: 0x20000, crc: 0xB99091AE },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CAPTCOMMR1: RomSetDef = RomSetDef { name: "captcommr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cce_23d.8f", offset: 0x0, size: 0x80000, crc: 0x19C58ECE },
            Op::Load { kind: Load::Word16Swap, file: "cc_22d.7f", offset: 0x80000, size: 0x80000, crc: 0xA91949B7 },
            Op::Load { kind: Load::Byte16, file: "cc_24d.9e", offset: 0x100000, size: 0x20000, crc: 0x680E543F },
            Op::Load { kind: Load::Byte16, file: "cc_28d.9f", offset: 0x100001, size: 0x20000, crc: 0x8820039F },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cc-5m.3a", offset: 0x0, size: 0x80000, crc: 0x7261D8BA },
            Op::Load { kind: Load::Word64, file: "cc-7m.5a", offset: 0x2, size: 0x80000, crc: 0x6A60F949 },
            Op::Load { kind: Load::Word64, file: "cc-1m.4a", offset: 0x4, size: 0x80000, crc: 0x00637302 },
            Op::Load { kind: Load::Word64, file: "cc-3m.6a", offset: 0x6, size: 0x80000, crc: 0xCC87CF61 },
            Op::Load { kind: Load::Word64, file: "cc-6m.7a", offset: 0x200000, size: 0x80000, crc: 0x28718BED },
            Op::Load { kind: Load::Word64, file: "cc-8m.9a", offset: 0x200002, size: 0x80000, crc: 0xD4ACC53A },
            Op::Load { kind: Load::Word64, file: "cc-2m.8a", offset: 0x200004, size: 0x80000, crc: 0x0C69F151 },
            Op::Load { kind: Load::Word64, file: "cc-4m.10a", offset: 0x200006, size: 0x80000, crc: 0x1F9EBB97 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cc_09.11a", offset: 0x0, size: 0x8000, crc: 0x698E8B58 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cc_18.11c", offset: 0x0, size: 0x20000, crc: 0x6DE2C2DB },
            Op::Load { kind: Load::Byte, file: "cc_19.12c", offset: 0x20000, size: 0x20000, crc: 0xB99091AE },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CAPTCOMMU: RomSetDef = RomSetDef { name: "captcommu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ccu_23b.8f", offset: 0x0, size: 0x80000, crc: 0x03DA44FD },
            Op::Load { kind: Load::Word16Swap, file: "ccu_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x9B82A052 },
            Op::Load { kind: Load::Byte16, file: "ccu_24b.9e", offset: 0x100000, size: 0x20000, crc: 0x84FF99B2 },
            Op::Load { kind: Load::Byte16, file: "ccu_28b.9f", offset: 0x100001, size: 0x20000, crc: 0xFBCEC223 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cc-5m.3a", offset: 0x0, size: 0x80000, crc: 0x7261D8BA },
            Op::Load { kind: Load::Word64, file: "cc-7m.5a", offset: 0x2, size: 0x80000, crc: 0x6A60F949 },
            Op::Load { kind: Load::Word64, file: "cc-1m.4a", offset: 0x4, size: 0x80000, crc: 0x00637302 },
            Op::Load { kind: Load::Word64, file: "cc-3m.6a", offset: 0x6, size: 0x80000, crc: 0xCC87CF61 },
            Op::Load { kind: Load::Word64, file: "cc-6m.7a", offset: 0x200000, size: 0x80000, crc: 0x28718BED },
            Op::Load { kind: Load::Word64, file: "cc-8m.9a", offset: 0x200002, size: 0x80000, crc: 0xD4ACC53A },
            Op::Load { kind: Load::Word64, file: "cc-2m.8a", offset: 0x200004, size: 0x80000, crc: 0x0C69F151 },
            Op::Load { kind: Load::Word64, file: "cc-4m.10a", offset: 0x200006, size: 0x80000, crc: 0x1F9EBB97 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ccu_09.11a", offset: 0x0, size: 0x8000, crc: 0x698E8B58 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ccu_18.11c", offset: 0x0, size: 0x20000, crc: 0x6DE2C2DB },
            Op::Load { kind: Load::Byte, file: "ccu_19.12c", offset: 0x20000, size: 0x20000, crc: 0xB99091AE },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CAPTCOMMJ: RomSetDef = RomSetDef { name: "captcommj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ccj_23f.8f", offset: 0x0, size: 0x80000, crc: 0x5B482B62 },
            Op::Load { kind: Load::Word16Swap, file: "ccj_22f.7f", offset: 0x80000, size: 0x80000, crc: 0x0FD34195 },
            Op::Load { kind: Load::Byte16, file: "ccj_24f.9e", offset: 0x100000, size: 0x20000, crc: 0x3A794F25 },
            Op::Load { kind: Load::Byte16, file: "ccj_28f.9f", offset: 0x100001, size: 0x20000, crc: 0xFC3C2906 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cc_01.3a", offset: 0x0, size: 0x80000, crc: 0x7261D8BA },
            Op::Load { kind: Load::Word64, file: "cc_02.4a", offset: 0x2, size: 0x80000, crc: 0x6A60F949 },
            Op::Load { kind: Load::Word64, file: "cc_03.5a", offset: 0x4, size: 0x80000, crc: 0x00637302 },
            Op::Load { kind: Load::Word64, file: "cc_04.6a", offset: 0x6, size: 0x80000, crc: 0xCC87CF61 },
            Op::Load { kind: Load::Word64, file: "cc_05.7a", offset: 0x200000, size: 0x80000, crc: 0x28718BED },
            Op::Load { kind: Load::Word64, file: "cc_06.8a", offset: 0x200002, size: 0x80000, crc: 0xD4ACC53A },
            Op::Load { kind: Load::Word64, file: "cc_07.9a", offset: 0x200004, size: 0x80000, crc: 0x0C69F151 },
            Op::Load { kind: Load::Word64, file: "cc_08.10a", offset: 0x200006, size: 0x80000, crc: 0x1F9EBB97 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ccj_09.12a", offset: 0x0, size: 0x8000, crc: 0x698E8B58 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ccj_18.11c", offset: 0x0, size: 0x20000, crc: 0x6DE2C2DB },
            Op::Load { kind: Load::Byte, file: "ccj_19.12c", offset: 0x20000, size: 0x20000, crc: 0xB99091AE },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CAPTCOMMJR1: RomSetDef = RomSetDef { name: "captcommjr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ccj_23b.8f", offset: 0x0, size: 0x80000, crc: 0xE2A2D80E },
            Op::Load { kind: Load::Word16Swap, file: "ccj_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x9B82A052 },
            Op::Load { kind: Load::Byte16, file: "ccj_24b.9e", offset: 0x100000, size: 0x20000, crc: 0x84FF99B2 },
            Op::Load { kind: Load::Byte16, file: "ccj_28b.9f", offset: 0x100001, size: 0x20000, crc: 0xFBCEC223 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cc_01.3a", offset: 0x0, size: 0x80000, crc: 0x7261D8BA },
            Op::Load { kind: Load::Word64, file: "cc_02.4a", offset: 0x2, size: 0x80000, crc: 0x6A60F949 },
            Op::Load { kind: Load::Word64, file: "cc_03.5a", offset: 0x4, size: 0x80000, crc: 0x00637302 },
            Op::Load { kind: Load::Word64, file: "cc_04.6a", offset: 0x6, size: 0x80000, crc: 0xCC87CF61 },
            Op::Load { kind: Load::Word64, file: "cc_05.7a", offset: 0x200000, size: 0x80000, crc: 0x28718BED },
            Op::Load { kind: Load::Word64, file: "cc_06.8a", offset: 0x200002, size: 0x80000, crc: 0xD4ACC53A },
            Op::Load { kind: Load::Word64, file: "cc_07.9a", offset: 0x200004, size: 0x80000, crc: 0x0C69F151 },
            Op::Load { kind: Load::Word64, file: "cc_08.10a", offset: 0x200006, size: 0x80000, crc: 0x1F9EBB97 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ccj_09.12a", offset: 0x0, size: 0x8000, crc: 0x698E8B58 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ccj_18.11c", offset: 0x0, size: 0x20000, crc: 0x6DE2C2DB },
            Op::Load { kind: Load::Byte, file: "ccj_19.12c", offset: 0x20000, size: 0x20000, crc: 0xB99091AE },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_CAPTCOMMB: RomSetDef = RomSetDef { name: "captcommb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "25.bin", offset: 0x0, size: 0x80000, crc: 0xCB71ED7A },
            Op::Load { kind: Load::Byte16, file: "27.bin", offset: 0x1, size: 0x80000, crc: 0x47CB2E87 },
            Op::Load { kind: Load::Byte16, file: "24.bin", offset: 0x100000, size: 0x40000, crc: 0x79794279 },
            Op::Load { kind: Load::Byte16, file: "26.bin", offset: 0x100001, size: 0x40000, crc: 0xB01077BA },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "c91e-01.bin", offset: 0x0, size: 0x40000, crc: 0xF863071C },
            Op::Continue { offset: 0x4, size: 0x40000 },
            Op::Continue { offset: 0x200000, size: 0x40000 },
            Op::Continue { offset: 0x200004, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "c91e-02.bin", offset: 0x1, size: 0x40000, crc: 0x4B03C308 },
            Op::Continue { offset: 0x5, size: 0x40000 },
            Op::Continue { offset: 0x200001, size: 0x40000 },
            Op::Continue { offset: 0x200005, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "c91e-03.bin", offset: 0x2, size: 0x40000, crc: 0x3383EA96 },
            Op::Continue { offset: 0x6, size: 0x40000 },
            Op::Continue { offset: 0x200002, size: 0x40000 },
            Op::Continue { offset: 0x200006, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "c91e-04.bin", offset: 0x3, size: 0x40000, crc: 0xB8E1F4CF },
            Op::Continue { offset: 0x7, size: 0x40000 },
            Op::Continue { offset: 0x200003, size: 0x40000 },
            Op::Continue { offset: 0x200007, size: 0x40000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "l.bin", offset: 0x0, size: 0x8000, crc: 0x698E8B58 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "c91e-05.bin", offset: 0x0, size: 0x40000, crc: 0x096115FB },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KNIGHTS: RomSetDef = RomSetDef { name: "knights", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "kr_23e.8f", offset: 0x0, size: 0x80000, crc: 0x1B3997EB },
            Op::Load { kind: Load::Word16Swap, file: "kr_22.7f", offset: 0x80000, size: 0x80000, crc: 0xD0B671A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kr-5m.3a", offset: 0x0, size: 0x80000, crc: 0x9E36C1A4 },
            Op::Load { kind: Load::Word64, file: "kr-7m.5a", offset: 0x2, size: 0x80000, crc: 0xC5832CAE },
            Op::Load { kind: Load::Word64, file: "kr-1m.4a", offset: 0x4, size: 0x80000, crc: 0xF095BE2D },
            Op::Load { kind: Load::Word64, file: "kr-3m.6a", offset: 0x6, size: 0x80000, crc: 0x179DFD96 },
            Op::Load { kind: Load::Word64, file: "kr-6m.7a", offset: 0x200000, size: 0x80000, crc: 0x1F4298D2 },
            Op::Load { kind: Load::Word64, file: "kr-8m.9a", offset: 0x200002, size: 0x80000, crc: 0x37FA8751 },
            Op::Load { kind: Load::Word64, file: "kr-2m.8a", offset: 0x200004, size: 0x80000, crc: 0x0200BC3D },
            Op::Load { kind: Load::Word64, file: "kr-4m.10a", offset: 0x200006, size: 0x80000, crc: 0x0BB2B4E7 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_09.11a", offset: 0x0, size: 0x8000, crc: 0x5E44D9EE },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_18.11c", offset: 0x0, size: 0x20000, crc: 0xDA69D15F },
            Op::Load { kind: Load::Byte, file: "kr_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBFC654E9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KNIGHTSU: RomSetDef = RomSetDef { name: "knightsu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "kr_23u.8f", offset: 0x0, size: 0x80000, crc: 0x252BC2BA },
            Op::Load { kind: Load::Word16Swap, file: "kr_22.7f", offset: 0x80000, size: 0x80000, crc: 0xD0B671A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kr-5m.3a", offset: 0x0, size: 0x80000, crc: 0x9E36C1A4 },
            Op::Load { kind: Load::Word64, file: "kr-7m.5a", offset: 0x2, size: 0x80000, crc: 0xC5832CAE },
            Op::Load { kind: Load::Word64, file: "kr-1m.4a", offset: 0x4, size: 0x80000, crc: 0xF095BE2D },
            Op::Load { kind: Load::Word64, file: "kr-3m.6a", offset: 0x6, size: 0x80000, crc: 0x179DFD96 },
            Op::Load { kind: Load::Word64, file: "kr-6m.7a", offset: 0x200000, size: 0x80000, crc: 0x1F4298D2 },
            Op::Load { kind: Load::Word64, file: "kr-8m.9a", offset: 0x200002, size: 0x80000, crc: 0x37FA8751 },
            Op::Load { kind: Load::Word64, file: "kr-2m.8a", offset: 0x200004, size: 0x80000, crc: 0x0200BC3D },
            Op::Load { kind: Load::Word64, file: "kr-4m.10a", offset: 0x200006, size: 0x80000, crc: 0x0BB2B4E7 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_09.11a", offset: 0x0, size: 0x8000, crc: 0x5E44D9EE },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_18.11c", offset: 0x0, size: 0x20000, crc: 0xDA69D15F },
            Op::Load { kind: Load::Byte, file: "kr_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBFC654E9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KNIGHTSJ: RomSetDef = RomSetDef { name: "knightsj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "kr_23j.8f", offset: 0x0, size: 0x80000, crc: 0xEAE7417F },
            Op::Load { kind: Load::Word16Swap, file: "kr_22.7f", offset: 0x80000, size: 0x80000, crc: 0xD0B671A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "kr_01.3a", offset: 0x0, size: 0x80000, crc: 0x9E36C1A4 },
            Op::Load { kind: Load::Word64, file: "kr_02.4a", offset: 0x2, size: 0x80000, crc: 0xC5832CAE },
            Op::Load { kind: Load::Word64, file: "kr_03.5a", offset: 0x4, size: 0x80000, crc: 0xF095BE2D },
            Op::Load { kind: Load::Word64, file: "kr_04.6a", offset: 0x6, size: 0x80000, crc: 0x179DFD96 },
            Op::Load { kind: Load::Word64, file: "kr_05.7a", offset: 0x200000, size: 0x80000, crc: 0x1F4298D2 },
            Op::Load { kind: Load::Word64, file: "kr_06.8a", offset: 0x200002, size: 0x80000, crc: 0x37FA8751 },
            Op::Load { kind: Load::Word64, file: "kr_07.9a", offset: 0x200004, size: 0x80000, crc: 0x0200BC3D },
            Op::Load { kind: Load::Word64, file: "kr_08.10a", offset: 0x200006, size: 0x80000, crc: 0x0BB2B4E7 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_09.12a", offset: 0x0, size: 0x8000, crc: 0x5E44D9EE },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_18.11c", offset: 0x0, size: 0x20000, crc: 0xDA69D15F },
            Op::Load { kind: Load::Byte, file: "kr_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBFC654E9 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_KNIGHTSJA: RomSetDef = RomSetDef { name: "knightsja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "krj_36.12f", offset: 0x0, size: 0x20000, crc: 0xAD3D1A8E },
            Op::Load { kind: Load::Byte16, file: "krj_42.12h", offset: 0x1, size: 0x20000, crc: 0xE694A491 },
            Op::Load { kind: Load::Byte16, file: "krj_37.13f", offset: 0x40000, size: 0x20000, crc: 0x85596094 },
            Op::Load { kind: Load::Byte16, file: "krj_43.13h", offset: 0x40001, size: 0x20000, crc: 0x9198BF8F },
            Op::Load { kind: Load::Byte16, file: "kr_34.10f", offset: 0x80000, size: 0x20000, crc: 0xFE6EB08D },
            Op::Load { kind: Load::Byte16, file: "kr_40.10h", offset: 0x80001, size: 0x20000, crc: 0x1172806D },
            Op::Load { kind: Load::Byte16, file: "kr_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xF854B020 },
            Op::Load { kind: Load::Byte16, file: "kr_41.11h", offset: 0xc0001, size: 0x20000, crc: 0xEB52E78D },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "kr_09.4b", offset: 0x0, size: 0x20000, crc: 0x08B76E10 },
            Op::Load { kind: Load::Byte64, file: "kr_01.4a", offset: 0x1, size: 0x20000, crc: 0x40CECF5C },
            Op::Load { kind: Load::Byte64, file: "kr_13.9b", offset: 0x2, size: 0x20000, crc: 0x435AAA03 },
            Op::Load { kind: Load::Byte64, file: "kr_05.9a", offset: 0x3, size: 0x20000, crc: 0x5B8A615B },
            Op::Load { kind: Load::Byte64, file: "kr_24.5e", offset: 0x4, size: 0x20000, crc: 0xDE65153E },
            Op::Load { kind: Load::Byte64, file: "kr_17.5c", offset: 0x5, size: 0x20000, crc: 0xB171C968 },
            Op::Load { kind: Load::Byte64, file: "kr_38.8h", offset: 0x6, size: 0x20000, crc: 0xF4466BF4 },
            Op::Load { kind: Load::Byte64, file: "kr_32.8f", offset: 0x7, size: 0x20000, crc: 0x87380DDD },
            Op::Load { kind: Load::Byte64, file: "kr_10.5b", offset: 0x100000, size: 0x20000, crc: 0x37006D66 },
            Op::Load { kind: Load::Byte64, file: "kr_02.5a", offset: 0x100001, size: 0x20000, crc: 0xB54612E3 },
            Op::Load { kind: Load::Byte64, file: "kr_14.10b", offset: 0x100002, size: 0x20000, crc: 0x0AE88766 },
            Op::Load { kind: Load::Byte64, file: "kr_06.10a", offset: 0x100003, size: 0x20000, crc: 0xECB1A09A },
            Op::Load { kind: Load::Byte64, file: "kr_25.7e", offset: 0x100004, size: 0x20000, crc: 0x9AACE189 },
            Op::Load { kind: Load::Byte64, file: "kr_18.7c", offset: 0x100005, size: 0x20000, crc: 0x09FA14A5 },
            Op::Load { kind: Load::Byte64, file: "kr_39.9h", offset: 0x100006, size: 0x20000, crc: 0xFD8A9AEB },
            Op::Load { kind: Load::Byte64, file: "kr_33.9f", offset: 0x100007, size: 0x20000, crc: 0x11803E95 },
            Op::Load { kind: Load::Byte64, file: "kr_11.7b", offset: 0x200000, size: 0x20000, crc: 0xA967CEB3 },
            Op::Load { kind: Load::Byte64, file: "kr_03.7a", offset: 0x200001, size: 0x20000, crc: 0xEA10DB07 },
            Op::Load { kind: Load::Byte64, file: "kr_15.11b", offset: 0x200002, size: 0x20000, crc: 0x8140B83B },
            Op::Load { kind: Load::Byte64, file: "kr_07.11a", offset: 0x200003, size: 0x20000, crc: 0x6AF10648 },
            Op::Load { kind: Load::Byte64, file: "kr_26.8e", offset: 0x200004, size: 0x20000, crc: 0x8865D86B },
            Op::Load { kind: Load::Byte64, file: "kr_19.8c", offset: 0x200005, size: 0x20000, crc: 0x029F4ABE },
            Op::Load { kind: Load::Byte64, file: "kr_28.10e", offset: 0x200006, size: 0x20000, crc: 0x5F84F92F },
            Op::Load { kind: Load::Byte64, file: "kr_21.10c", offset: 0x200007, size: 0x20000, crc: 0x01B35065 },
            Op::Load { kind: Load::Byte64, file: "kr_12.8b", offset: 0x300000, size: 0x20000, crc: 0x03D945B1 },
            Op::Load { kind: Load::Byte64, file: "kr_04.8a", offset: 0x300001, size: 0x20000, crc: 0xE30C8388 },
            Op::Load { kind: Load::Byte64, file: "kr_16.12b", offset: 0x300002, size: 0x20000, crc: 0x40C39D1B },
            Op::Load { kind: Load::Byte64, file: "kr_08.12a", offset: 0x300003, size: 0x20000, crc: 0xD310C9E8 },
            Op::Load { kind: Load::Byte64, file: "kr_27.9e", offset: 0x300004, size: 0x20000, crc: 0x3E041444 },
            Op::Load { kind: Load::Byte64, file: "kr_20.9c", offset: 0x300005, size: 0x20000, crc: 0xBD4BFFB8 },
            Op::Load { kind: Load::Byte64, file: "kr_29.11e", offset: 0x300006, size: 0x20000, crc: 0x1387A076 },
            Op::Load { kind: Load::Byte64, file: "kr_22.11c", offset: 0x300007, size: 0x20000, crc: 0xFD351922 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_23.13b", offset: 0x0, size: 0x8000, crc: 0x5E44D9EE },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "kr_30.12c", offset: 0x0, size: 0x20000, crc: 0xDA69D15F },
            Op::Load { kind: Load::Byte, file: "kr_31.13c", offset: 0x20000, size: 0x20000, crc: 0xBFC654E9 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_KNIGHTSB2: RomSetDef = RomSetDef { name: "knightsb2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "left.code.040", offset: 0x0, size: 0x80000, crc: 0x95D00A7E },
            Op::Load { kind: Load::Byte16, file: "right.code.040", offset: 0x1, size: 0x80000, crc: 0x5A9D0B64 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "spe-a.japan9207d.mask1.801", offset: 0x0, size: 0x40000, crc: 0x14A15FCD },
            Op::Continue { offset: 0x4, size: 0x40000 },
            Op::Continue { offset: 0x200000, size: 0x40000 },
            Op::Continue { offset: 0x200004, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "spe-b.japan9207d.mask2.801", offset: 0x1, size: 0x40000, crc: 0x250D2957 },
            Op::Continue { offset: 0x5, size: 0x40000 },
            Op::Continue { offset: 0x200001, size: 0x40000 },
            Op::Continue { offset: 0x200005, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "spe-c.japan9207d.mask4.801", offset: 0x2, size: 0x40000, crc: 0x0721C26D },
            Op::Continue { offset: 0x6, size: 0x40000 },
            Op::Continue { offset: 0x200002, size: 0x40000 },
            Op::Continue { offset: 0x200006, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "spe-d.japan9207d.mask3.801", offset: 0x3, size: 0x40000, crc: 0xDB97F56A },
            Op::Continue { offset: 0x7, size: 0x40000 },
            Op::Continue { offset: 0x200003, size: 0x40000 },
            Op::Continue { offset: 0x200007, size: 0x40000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sound.code.512", offset: 0x0, size: 0x8000, crc: 0x5E44D9EE },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "spe-e.japan9208d.snd.mask.020", offset: 0x0, size: 0x40000, crc: 0x85F837A0 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CE: RomSetDef = RomSetDef { name: "sf2ce", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92e_23b.8f", offset: 0x0, size: 0x80000, crc: 0x0AAA1A3A },
            Op::Load { kind: Load::Word16Swap, file: "s92_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x2BBE15ED },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEEA: RomSetDef = RomSetDef { name: "sf2ceea", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92e_23a.8f", offset: 0x0, size: 0x80000, crc: 0x3F846B74 },
            Op::Load { kind: Load::Word16Swap, file: "s92_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x99F1CCA4 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEEC: RomSetDef = RomSetDef { name: "sf2ceec", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92e_23c.8f", offset: 0x0, size: 0x80000, crc: 0x994B408D },
            Op::Load { kind: Load::Word16Swap, file: "s92_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x5FD8630B },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEUA: RomSetDef = RomSetDef { name: "sf2ceua", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92u_23a.8f", offset: 0x0, size: 0x80000, crc: 0xAC44415B },
            Op::Load { kind: Load::Word16Swap, file: "s92_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x99F1CCA4 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEUB: RomSetDef = RomSetDef { name: "sf2ceub", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92u_23b.8f", offset: 0x0, size: 0x80000, crc: 0x996A3015 },
            Op::Load { kind: Load::Word16Swap, file: "s92_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x2BBE15ED },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEUC: RomSetDef = RomSetDef { name: "sf2ceuc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92u_23c.8f", offset: 0x0, size: 0x80000, crc: 0x0A8B6AA2 },
            Op::Load { kind: Load::Word16Swap, file: "s92_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x5FD8630B },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CET: RomSetDef = RomSetDef { name: "sf2cet", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92t_23a.8f", offset: 0x0, size: 0x80000, crc: 0xD7C28ADE },
            Op::Load { kind: Load::Word16Swap, file: "s92_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x99F1CCA4 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEJA: RomSetDef = RomSetDef { name: "sf2ceja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92j_23a.8f", offset: 0x0, size: 0x80000, crc: 0x4F42BB5A },
            Op::Load { kind: Load::Word16Swap, file: "s92j_22a.7f", offset: 0x80000, size: 0x80000, crc: 0xC4F64BCD },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.4a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.5a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.9a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.4c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.5c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.12a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEJB: RomSetDef = RomSetDef { name: "sf2cejb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92j_23b.8f", offset: 0x0, size: 0x80000, crc: 0x140876C5 },
            Op::Load { kind: Load::Word16Swap, file: "s92j_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x2FBB3BFE },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.4a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.5a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.9a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.4c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.5c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.12a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEJC: RomSetDef = RomSetDef { name: "sf2cejc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s92j_23c.8f", offset: 0x0, size: 0x80000, crc: 0xF0120635 },
            Op::Load { kind: Load::Word16Swap, file: "s92j_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x8C0B2ED6 },
            Op::Load { kind: Load::Word16Swap, file: "s92j_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.4a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.5a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.9a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.4c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.5c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.12a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2BHH: RomSetDef = RomSetDef { name: "sf2bhh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "23", offset: 0x0, size: 0x40000, crc: 0x5CF63A9E },
            Op::Continue { offset: 0xc0000, size: 0x40000 },
            Op::Load { kind: Load::Word16Swap, file: "22", offset: 0x80000, size: 0x40000, crc: 0x3ED72BCA },
            Op::Continue { offset: 0x40000, size: 0x40000 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RB: RomSetDef = RomSetDef { name: "sf2rb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16, file: "sf2d__23.rom", offset: 0x0, size: 0x80000, crc: 0x450532B0 },
            Op::Load { kind: Load::Word16, file: "sf2d__22.rom", offset: 0x80000, size: 0x80000, crc: 0xFE9D9CF5 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RB2: RomSetDef = RomSetDef { name: "sf2rb2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "27.bin", offset: 0x0, size: 0x20000, crc: 0x40296ECD },
            Op::Load { kind: Load::Byte16, file: "31.bin", offset: 0x1, size: 0x20000, crc: 0x87954A41 },
            Op::Load { kind: Load::Byte16, file: "26.bin", offset: 0x40000, size: 0x20000, crc: 0xA6974195 },
            Op::Load { kind: Load::Byte16, file: "30.bin", offset: 0x40001, size: 0x20000, crc: 0x8141FE32 },
            Op::Load { kind: Load::Byte16, file: "25.bin", offset: 0x80000, size: 0x20000, crc: 0x9EF8F772 },
            Op::Load { kind: Load::Byte16, file: "29.bin", offset: 0x80001, size: 0x20000, crc: 0x7D9C479C },
            Op::Load { kind: Load::Byte16, file: "24.bin", offset: 0xc0000, size: 0x20000, crc: 0x93579684 },
            Op::Load { kind: Load::Byte16, file: "28.bin", offset: 0xc0001, size: 0x20000, crc: 0xFF728865 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RB3: RomSetDef = RomSetDef { name: "sf2rb3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sf2_ce_rb.23", offset: 0x0, size: 0x80000, crc: 0x202F9E50 },
            Op::Load { kind: Load::Word16Swap, file: "sf2_ce_rb.22", offset: 0x80000, size: 0x80000, crc: 0x145E5219 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RED: RomSetDef = RomSetDef { name: "sf2red", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sf2red.23", offset: 0x0, size: 0x80000, crc: 0x40276ABB },
            Op::Load { kind: Load::Word16Swap, file: "sf2red.22", offset: 0x80000, size: 0x80000, crc: 0x18DAF387 },
            Op::Load { kind: Load::Word16Swap, file: "sf2red.21", offset: 0x100000, size: 0x80000, crc: 0x52C486BB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2REDA: RomSetDef = RomSetDef { name: "sf2reda", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "stf champ wave rom 21.6f", offset: 0x0, size: 0x40000, crc: 0x04FFF17B },
            Op::Continue { offset: 0xc0000, size: 0x40000 },
            Op::Load { kind: Load::Word16Swap, file: "stf champ wave rom 23.8f", offset: 0x80000, size: 0x40000, crc: 0xEB265DC7 },
            Op::Continue { offset: 0x140000, size: 0x40000 },
            Op::Load { kind: Load::Word16Swap, file: "stf champ wave rom 22.7f", offset: 0x100000, size: 0x40000, crc: 0x27E80CB1 },
            Op::Continue { offset: 0x40000, size: 0x40000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2REDP2: RomSetDef = RomSetDef { name: "sf2redp2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sf2red.23", offset: 0x0, size: 0x80000, crc: 0x2D3C4F72 },
            Op::Load { kind: Load::Word16Swap, file: "sf2red.22", offset: 0x80000, size: 0x80000, crc: 0x18DAF387 },
            Op::Load { kind: Load::Word16Swap, file: "sf2red.21", offset: 0x100000, size: 0x80000, crc: 0xAAF693B5 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2V004: RomSetDef = RomSetDef { name: "sf2v004", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sf2v004.23", offset: 0x0, size: 0x80000, crc: 0x52D19F2C },
            Op::Load { kind: Load::Word16Swap, file: "sf2v004.22", offset: 0x80000, size: 0x80000, crc: 0x4B26FDE7 },
            Op::Load { kind: Load::Word16Swap, file: "sf2red.21", offset: 0x100000, size: 0x80000, crc: 0x52C486BB },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2ACC: RomSetDef = RomSetDef { name: "sf2acc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sf2ca_23-c.bin", offset: 0x0, size: 0x80000, crc: 0xE7C8C5A6 },
            Op::Load { kind: Load::Word16Swap, file: "sf2ca_22-c.bin", offset: 0x80000, size: 0x80000, crc: 0x99F1CCA4 },
            Op::Load { kind: Load::Word16Swap, file: "sf2ca_21-c.bin", offset: 0x100000, size: 0x40000, crc: 0xCF7FCC8C },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2ACCA: RomSetDef = RomSetDef { name: "sf2acca", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "23-c.8f", offset: 0x0, size: 0x80000, crc: 0x35F9517B },
            Op::Load { kind: Load::Word16Swap, file: "22-c.7f", offset: 0x80000, size: 0x80000, crc: 0x99F1CCA4 },
            Op::Load { kind: Load::Word16Swap, file: "21-c.6f", offset: 0x100000, size: 0x80000, crc: 0x2AB2034F },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2ACCP2: RomSetDef = RomSetDef { name: "sf2accp2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sf2ca-23.bin", offset: 0x0, size: 0x80000, crc: 0x36C3BA2F },
            Op::Load { kind: Load::Word16Swap, file: "sf2ca-22.bin", offset: 0x80000, size: 0x80000, crc: 0x0550453D },
            Op::Load { kind: Load::Word16Swap, file: "sf2ca-21.bin", offset: 0x100000, size: 0x40000, crc: 0x4C1C43BA },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : protection_pic
pub const RS_SF2CEBLP: RomSetDef = RomSetDef { name: "sf2ceblp", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "1k.31.e13", offset: 0x0, size: 0x20000, crc: 0xEA78F9B4 },
            Op::Load { kind: Load::Byte16, file: "se.36.j13", offset: 0x1, size: 0x20000, crc: 0xD30C263E },
            Op::Load { kind: Load::Byte16, file: "0e.30.e11", offset: 0x40000, size: 0x20000, crc: 0x005B54CC },
            Op::Load { kind: Load::Byte16, file: "de.35.j11", offset: 0x40001, size: 0x20000, crc: 0xC184D26D },
            Op::Load { kind: Load::Byte16, file: "bk.29.e10", offset: 0x80000, size: 0x20000, crc: 0x524F5C55 },
            Op::Load { kind: Load::Byte16, file: "ff.34.j10", offset: 0x80001, size: 0x20000, crc: 0xF06A12F2 },
            Op::Load { kind: Load::Byte16, file: "8k.28.e9", offset: 0xc0000, size: 0x40000, crc: 0xB7AD5214 },
            Op::Load { kind: Load::Byte16, file: "tf.33.j9", offset: 0xc0001, size: 0x40000, crc: 0x6340B914 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "24.e7", offset: 0x0, size: 0x40000, crc: 0xA8B5633A },
            Op::Load { kind: Load::Byte64, file: "14.c7", offset: 0x1, size: 0x40000, crc: 0x5DB24CA7 },
            Op::Load { kind: Load::Byte64, file: "26.e9", offset: 0x2, size: 0x40000, crc: 0x82E8E384 },
            Op::Load { kind: Load::Byte64, file: "16.c9", offset: 0x3, size: 0x40000, crc: 0x1FD98AD0 },
            Op::Load { kind: Load::Byte64, file: "20.e3", offset: 0x4, size: 0x40000, crc: 0x8D5D0045 },
            Op::Load { kind: Load::Byte64, file: "10.c3", offset: 0x5, size: 0x40000, crc: 0xEB48F7F2 },
            Op::Load { kind: Load::Byte64, file: "22.e5", offset: 0x6, size: 0x40000, crc: 0x4109D637 },
            Op::Load { kind: Load::Byte64, file: "12.c5", offset: 0x7, size: 0x40000, crc: 0x5D21D8B3 },
            Op::Load { kind: Load::Byte64, file: "25.e8", offset: 0x200000, size: 0x40000, crc: 0x72E923DF },
            Op::Load { kind: Load::Byte64, file: "15.c8", offset: 0x200001, size: 0x40000, crc: 0x9A96BE48 },
            Op::Load { kind: Load::Byte64, file: "27.e10", offset: 0x200002, size: 0x40000, crc: 0x4A3A8D09 },
            Op::Load { kind: Load::Byte64, file: "17.c10", offset: 0x200003, size: 0x40000, crc: 0xA917A922 },
            Op::Load { kind: Load::Byte64, file: "21.e4", offset: 0x200004, size: 0x40000, crc: 0x55C2B455 },
            Op::Load { kind: Load::Byte64, file: "11.c4", offset: 0x200005, size: 0x40000, crc: 0xFF36859E },
            Op::Load { kind: Load::Byte64, file: "23.e6", offset: 0x200006, size: 0x40000, crc: 0xEF9C2D4D },
            Op::Load { kind: Load::Byte64, file: "13.c6", offset: 0x200007, size: 0x40000, crc: 0xBC937C96 },
            Op::Load { kind: Load::Byte64, file: "6.a8", offset: 0x400000, size: 0x40000, crc: 0x023BAA18 },
            Op::Load { kind: Load::Byte64, file: "5.a7", offset: 0x400001, size: 0x40000, crc: 0xA6AD6EF3 },
            Op::Load { kind: Load::Byte64, file: "8.a10", offset: 0x400002, size: 0x40000, crc: 0x26FB340C },
            Op::Load { kind: Load::Byte64, file: "7.a9", offset: 0x400003, size: 0x40000, crc: 0xF56085BA },
            Op::Load { kind: Load::Byte64, file: "1.a3", offset: 0x400004, size: 0x40000, crc: 0x877B2B18 },
            Op::Load { kind: Load::Byte64, file: "2.a4", offset: 0x400005, size: 0x40000, crc: 0x144AA4C9 },
            Op::Load { kind: Load::Byte64, file: "4.a6", offset: 0x400006, size: 0x40000, crc: 0xF2C400B4 },
            Op::Load { kind: Load::Byte64, file: "3.a5", offset: 0x400007, size: 0x40000, crc: 0x8053335D },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "snd.9.b13", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "9k.18.c11", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "lk.19.c13", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : pal
pub const RS_SF2CEBLTW: RomSetDef = RomSetDef { name: "sf2cebltw", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "27.12e", offset: 0x0, size: 0x20000, crc: 0x035EE5D9 },
            Op::Load { kind: Load::Byte16, file: "31.12f", offset: 0x1, size: 0x20000, crc: 0x353DBDE1 },
            Op::Load { kind: Load::Byte16, file: "24.9e", offset: 0x40000, size: 0x20000, crc: 0x005B54CC },
            Op::Load { kind: Load::Byte16, file: "28.9f", offset: 0x40001, size: 0x20000, crc: 0xC184D26D },
            Op::Load { kind: Load::Byte16, file: "25.10e", offset: 0x80000, size: 0x20000, crc: 0x524F5C55 },
            Op::Load { kind: Load::Byte16, file: "29.10f", offset: 0x80001, size: 0x20000, crc: 0xF06A12F2 },
            Op::Load { kind: Load::Byte16, file: "26.11e", offset: 0xc0000, size: 0x20000, crc: 0x8312D055 },
            Op::Load { kind: Load::Byte16, file: "30.11f", offset: 0xc0001, size: 0x20000, crc: 0xD0580FF2 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.5f", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2AMF: RomSetDef = RomSetDef { name: "sf2amf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "5.amf", offset: 0x0, size: 0x80000, crc: 0x03991FBA },
            Op::Load { kind: Load::Byte16, file: "4.amf", offset: 0x1, size: 0x80000, crc: 0x39F15A1E },
            Op::NoDump,
            Op::NoDump,
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "y.c.e.c m.k.r-001", offset: 0x0, size: 0x80000, crc: 0xA258DE13 },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "y.c.e.c m.k.r-003", offset: 0x2, size: 0x80000, crc: 0xC781BF87 },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "y.c.e.c m.k.r-002", offset: 0x200000, size: 0x80000, crc: 0x5726CAB8 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "y.c.e.c d.w.c-011", offset: 0x200002, size: 0x80000, crc: 0xBC90C12F },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "y.c.e.c d.w.c-012", offset: 0x400000, size: 0x80000, crc: 0x187667CC },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "y.c.e.c d.w.c-013", offset: 0x400002, size: 0x80000, crc: 0x5B585071 },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "6.amf", offset: 0x400000, size: 0x10000, crc: 0x3A85A275 },
            Op::Continue { offset: 0x400004, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "9.amf", offset: 0x400002, size: 0x10000, crc: 0x9156472F },
            Op::Continue { offset: 0x400006, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "8.amf", offset: 0x400001, size: 0x10000, crc: 0xECDB083B },
            Op::Continue { offset: 0x400005, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "10.amf", offset: 0x400003, size: 0x10000, crc: 0x8FEA8384 },
            Op::Continue { offset: 0x400007, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "3.amf", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "7.amf", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "2.amf", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "1.amf", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2AMF2: RomSetDef = RomSetDef { name: "sf2amf2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "m5m27c401.u222", offset: 0x0, size: 0x80000, crc: 0x03991FBA },
            Op::Load { kind: Load::Byte16, file: "m5m27c401.u196", offset: 0x1, size: 0x80000, crc: 0x39F15A1E },
            Op::Load { kind: Load::Byte16, file: "27020.u221", offset: 0x100000, size: 0x40000, crc: 0xAA4D55A6 },
            Op::Load { kind: Load::Byte16, file: "27020.u195", offset: 0x100001, size: 0x40000, crc: 0x2BFFA6F9 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "fun-u70.bin", offset: 0x4, size: 0x80000, crc: 0xA94A8B19 },
            Op::Continue { offset: 0x0, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u68.bin", offset: 0x6, size: 0x80000, crc: 0x0405F21F },
            Op::Continue { offset: 0x2, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u69.bin", offset: 0x200004, size: 0x80000, crc: 0x05DC2043 },
            Op::Continue { offset: 0x200000, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u67.bin", offset: 0x200006, size: 0x80000, crc: 0x055B64F1 },
            Op::Continue { offset: 0x200002, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u19.bin", offset: 0x400004, size: 0x80000, crc: 0x1A518609 },
            Op::Continue { offset: 0x400000, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u18.bin", offset: 0x400006, size: 0x80000, crc: 0x84F9354F },
            Op::Continue { offset: 0x400002, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "grp1.u31", offset: 0x400004, size: 0x10000, crc: 0x6DE44671 },
            Op::Continue { offset: 0x400000, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp3.u29", offset: 0x400006, size: 0x10000, crc: 0xE8F14362 },
            Op::Continue { offset: 0x400002, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp2.u30", offset: 0x400005, size: 0x10000, crc: 0xBF0CD819 },
            Op::Continue { offset: 0x400001, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp4.u28", offset: 0x400007, size: 0x10000, crc: 0x76F9F91F },
            Op::Continue { offset: 0x400003, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.u191", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "fun-u210.bin", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2AMF3: RomSetDef = RomSetDef { name: "sf2amf3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222.bin", offset: 0x0, size: 0x80000, crc: 0x0D305E8B },
            Op::Load { kind: Load::Byte16, file: "u196.bin", offset: 0x1, size: 0x80000, crc: 0x137D8665 },
            Op::Load { kind: Load::Byte16, file: "u221.bin", offset: 0x100000, size: 0x40000, crc: 0x0B3FE5DD },
            Op::Load { kind: Load::Byte16, file: "u195.bin", offset: 0x100001, size: 0x40000, crc: 0xDBEE7B18 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "fun-u70.bin", offset: 0x4, size: 0x80000, crc: 0xA94A8B19 },
            Op::Continue { offset: 0x0, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u68.bin", offset: 0x6, size: 0x80000, crc: 0x0405F21F },
            Op::Continue { offset: 0x2, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u69.bin", offset: 0x200004, size: 0x80000, crc: 0x05DC2043 },
            Op::Continue { offset: 0x200000, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u67.bin", offset: 0x200006, size: 0x80000, crc: 0x055B64F1 },
            Op::Continue { offset: 0x200002, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u19.bin", offset: 0x400004, size: 0x80000, crc: 0x1A518609 },
            Op::Continue { offset: 0x400000, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "fun-u18.bin", offset: 0x400006, size: 0x80000, crc: 0x84F9354F },
            Op::Continue { offset: 0x400002, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "grp1.u31", offset: 0x400004, size: 0x10000, crc: 0x6DE44671 },
            Op::Continue { offset: 0x400000, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp3.u29", offset: 0x400006, size: 0x10000, crc: 0xE8F14362 },
            Op::Continue { offset: 0x400002, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp2.u30", offset: 0x400005, size: 0x10000, crc: 0xBF0CD819 },
            Op::Continue { offset: 0x400001, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp4.u28", offset: 0x400007, size: 0x10000, crc: 0x76F9F91F },
            Op::Continue { offset: 0x400003, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.u191", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27512.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "fun-u210.bin", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2RULES: RomSetDef = RomSetDef { name: "sf2rules", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "prh2.u222", offset: 0x0, size: 0x80000, crc: 0xFFF85F9B },
            Op::Load { kind: Load::Byte16, file: "prl1.u196", offset: 0x1, size: 0x80000, crc: 0x65C28BC9 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ycecmkr001.u70", offset: 0x0, size: 0x80000, crc: 0xA258DE13 },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecmkr003.u69", offset: 0x2, size: 0x80000, crc: 0xC781BF87 },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecmkr002.u68", offset: 0x200000, size: 0x80000, crc: 0x5726CAB8 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecdwc011.u64", offset: 0x200002, size: 0x80000, crc: 0xBC90C12F },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecdwc012.u19", offset: 0x400000, size: 0x80000, crc: 0x187667CC },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecdwc013.u18", offset: 0x400002, size: 0x80000, crc: 0x5B585071 },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "grp1.u31", offset: 0x400004, size: 0x10000, crc: 0x6DE44671 },
            Op::Continue { offset: 0x400000, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp3.u29", offset: 0x400006, size: 0x10000, crc: 0xE8F14362 },
            Op::Continue { offset: 0x400002, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp2.u30", offset: 0x400005, size: 0x10000, crc: 0xBF0CD819 },
            Op::Continue { offset: 0x400001, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp4.u28", offset: 0x400007, size: 0x10000, crc: 0x76F9F91F },
            Op::Continue { offset: 0x400003, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sound.u191", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "conv.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "voice.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2DKOT2: RomSetDef = RomSetDef { name: "sf2dkot2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "turboii.23", offset: 0x0, size: 0x80000, crc: 0x9BBFE420 },
            Op::Reload { offset: 0x280000, size: 0x80000 },
            Op::Load { kind: Load::Word16Swap, file: "turboii.22", offset: 0x80000, size: 0x80000, crc: 0x3E57BA19 },
            Op::Reload { offset: 0x200000, size: 0x80000 },
            Op::Load { kind: Load::Word16Swap, file: "turboii.21", offset: 0x100000, size: 0x80000, crc: 0xED4186BD },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M2: RomSetDef = RomSetDef { name: "sf2m2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "ch222esp", offset: 0x0, size: 0x80000, crc: 0x9E6D058A },
            Op::Load { kind: Load::Byte16, file: "ch196esp", offset: 0x1, size: 0x80000, crc: 0xED2FF437 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M3: RomSetDef = RomSetDef { name: "sf2m3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222chp", offset: 0x0, size: 0x80000, crc: 0xDB567B66 },
            Op::Load { kind: Load::Byte16, file: "u196chp", offset: 0x1, size: 0x80000, crc: 0x95EA597E },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M4: RomSetDef = RomSetDef { name: "sf2m4", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222ne", offset: 0x0, size: 0x80000, crc: 0x7133489E },
            Op::Load { kind: Load::Byte16, file: "u196ne", offset: 0x1, size: 0x80000, crc: 0xB07A4F90 },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M5: RomSetDef = RomSetDef { name: "sf2m5", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222", offset: 0x0, size: 0x80000, crc: 0x03991FBA },
            Op::Load { kind: Load::Byte16, file: "u196", offset: 0x1, size: 0x80000, crc: 0x39F15A1E },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M6: RomSetDef = RomSetDef { name: "sf2m6", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "27c040.u222", offset: 0x0, size: 0x80000, crc: 0x0A3692BE },
            Op::Load { kind: Load::Byte16, file: "27c040.u196", offset: 0x1, size: 0x80000, crc: 0x80454DA7 },
            Op::Load { kind: Load::Byte16, file: "27c010.u221", offset: 0x100000, size: 0x20000, crc: 0x8226C11C },
            Op::Load { kind: Load::Byte16, file: "27c010.u195", offset: 0x100001, size: 0x20000, crc: 0x924C6CE2 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ycecmkr001.u70", offset: 0x0, size: 0x80000, crc: 0xA258DE13 },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecmkr003.u69", offset: 0x2, size: 0x80000, crc: 0xC781BF87 },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecmkr002.u68", offset: 0x200000, size: 0x80000, crc: 0x5726CAB8 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecdwc011.u64", offset: 0x200002, size: 0x80000, crc: 0xBC90C12F },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecdwc012.u19", offset: 0x400000, size: 0x80000, crc: 0x187667CC },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ycecdwc013.u18", offset: 0x400002, size: 0x80000, crc: 0x5B585071 },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Byte64, file: "grp1.u31", offset: 0x400004, size: 0x10000, crc: 0x6DE44671 },
            Op::Continue { offset: 0x400000, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp3.u29", offset: 0x400006, size: 0x10000, crc: 0xE8F14362 },
            Op::Continue { offset: 0x400002, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp2.u30", offset: 0x400005, size: 0x10000, crc: 0xBF0CD819 },
            Op::Continue { offset: 0x400001, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "grp4.u28", offset: 0x400007, size: 0x10000, crc: 0x76F9F91F },
            Op::Continue { offset: 0x400003, size: 0x10000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "conv.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sound.u191", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "voice.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M7: RomSetDef = RomSetDef { name: "sf2m7", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222-2i", offset: 0x0, size: 0x40000, crc: 0x1CA7ADBD },
            Op::Load { kind: Load::Byte16, file: "u196-2i", offset: 0x1, size: 0x40000, crc: 0xF758408C },
            Op::Load { kind: Load::Byte16, file: "u222-2s", offset: 0x80000, size: 0x40000, crc: 0x720CEA3E },
            Op::Load { kind: Load::Byte16, file: "u196-2s", offset: 0x80001, size: 0x40000, crc: 0x9932832C },
            Op::Load { kind: Load::Word16Swap, file: "s92_21a.bin", offset: 0x100000, size: 0x80000, crc: 0x925A7877 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : user2
pub const RS_SF2M8: RomSetDef = RomSetDef { name: "sf2m8", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "yyc-2.2", offset: 0x0, size: 0x80000, crc: 0xDB567B66 },
            Op::Load { kind: Load::Byte16, file: "yyc-3.4", offset: 0x1, size: 0x80000, crc: 0x95EA597E },
            Op::Load { kind: Load::Byte16, file: "yyc-4.1", offset: 0x100000, size: 0x20000, crc: 0x1073B7B6 },
            Op::Load { kind: Load::Byte16, file: "yyc-5.3", offset: 0x100001, size: 0x20000, crc: 0x924C6CE2 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "yyc-a", offset: 0x0, size: 0x40000, crc: 0x8242621F },
            Op::Continue { offset: 0x4, size: 0x40000 },
            Op::Continue { offset: 0x200000, size: 0x40000 },
            Op::Continue { offset: 0x200004, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "yyc-c", offset: 0x1, size: 0x40000, crc: 0x0793A960 },
            Op::Continue { offset: 0x5, size: 0x40000 },
            Op::Continue { offset: 0x200001, size: 0x40000 },
            Op::Continue { offset: 0x200005, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "yyc-b", offset: 0x2, size: 0x40000, crc: 0xB0159973 },
            Op::Continue { offset: 0x6, size: 0x40000 },
            Op::Continue { offset: 0x200002, size: 0x40000 },
            Op::Continue { offset: 0x200006, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "yyc-d", offset: 0x3, size: 0x40000, crc: 0x92A8B572 },
            Op::Continue { offset: 0x7, size: 0x40000 },
            Op::Continue { offset: 0x200003, size: 0x40000 },
            Op::Continue { offset: 0x200007, size: 0x40000 },
            Op::Load { kind: Load::Byte64, file: "yyc-6.1", offset: 0x400000, size: 0x10000, crc: 0x94778332 },
            Op::Continue { offset: 0x400004, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "yyc-8.9", offset: 0x400002, size: 0x10000, crc: 0xF95BC505 },
            Op::Continue { offset: 0x400006, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "yyc-7.10", offset: 0x400001, size: 0x10000, crc: 0xD1E452D3 },
            Op::Continue { offset: 0x400005, size: 0x10000 },
            Op::Load { kind: Load::Byte64, file: "yyc-9.8", offset: 0x400003, size: 0x10000, crc: 0x155824A9 },
            Op::Continue { offset: 0x400007, size: 0x10000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "a-15.5", offset: 0x0, size: 0x8000, crc: 0x6F07D2CB },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x10000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "c-27.7", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "b-16.6", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2M10: RomSetDef = RomSetDef { name: "sf2m10", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "sf2h14.7", offset: 0x0, size: 0x80000, crc: 0x74803532 },
            Op::Load { kind: Load::Byte16, file: "sf2h14.5", offset: 0x1, size: 0x80000, crc: 0x66C91972 },
            Op::Load { kind: Load::Byte16, file: "yyc-4.1", offset: 0x100000, size: 0x20000, crc: 0x1073B7B6 },
            Op::Load { kind: Load::Byte16, file: "yyc-5.3", offset: 0x100001, size: 0x20000, crc: 0x924C6CE2 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
            Op::Load { kind: Load::Word64, file: "27c1024.10", offset: 0x400000, size: 0x20000, crc: 0x84427D1B },
            Op::Load { kind: Load::Word64, file: "27c1024.12", offset: 0x400002, size: 0x20000, crc: 0x55BC790C },
            Op::Load { kind: Load::Word64, file: "27c1024.9", offset: 0x400004, size: 0x20000, crc: 0xF8725ADD },
            Op::Load { kind: Load::Word64, file: "27c1024.11", offset: 0x400006, size: 0x20000, crc: 0xC2A5373E },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c512.3", offset: 0x0, size: 0x8000, crc: 0xA4823A1B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c512.8", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "27c010.2", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "27c010.1", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2YYC: RomSetDef = RomSetDef { name: "sf2yyc", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "b12.rom", offset: 0x0, size: 0x80000, crc: 0x8F742FD5 },
            Op::Load { kind: Load::Byte16, file: "b14.rom", offset: 0x1, size: 0x80000, crc: 0x8831EC7F },
            Op::Load { kind: Load::Byte16, file: "b11.rom", offset: 0x100000, size: 0x20000, crc: 0x94A46525 },
            Op::Reload { offset: 0x140000, size: 0x20000 },
            Op::Load { kind: Load::Byte16, file: "b13.rom", offset: 0x100001, size: 0x20000, crc: 0x8FB3DD47 },
            Op::Reload { offset: 0x140001, size: 0x20000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2KORYU: RomSetDef = RomSetDef { name: "sf2koryu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "u222.rom", offset: 0x0, size: 0x80000, crc: 0x9236A79A },
            Op::Load { kind: Load::Byte16, file: "u196.rom", offset: 0x1, size: 0x80000, crc: 0xB23A869D },
            Op::Load { kind: Load::Byte16, file: "u221.rom", offset: 0x100000, size: 0x20000, crc: 0x64E6E091 },
            Op::Load { kind: Load::Byte16, file: "u195.rom", offset: 0x100001, size: 0x20000, crc: 0xC95E4443 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.bin", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.bin", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.bin", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.bin", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.bin", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.bin", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.bin", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.bin", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.bin", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.bin", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.bin", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.bin", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.bin", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.bin", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.bin", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2CEUPL: RomSetDef = RomSetDef { name: "sf2ceupl", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "s2.u222", offset: 0x0, size: 0x80000, crc: 0x0804F973 },
            Op::Load { kind: Load::Byte16, file: "s1.u196", offset: 0x1, size: 0x80000, crc: 0x2BC76A02 },
            Op::Load { kind: Load::Byte16, file: "10.u221", offset: 0x100000, size: 0x20000, crc: 0xD1707134 },
            Op::Load { kind: Load::Byte16, file: "9.u195", offset: 0x100001, size: 0x20000, crc: 0xCD1D5666 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.4a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.5a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.9a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.4c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.5c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.12a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2DONGB: RomSetDef = RomSetDef { name: "sf2dongb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "1.8f", offset: 0x0, size: 0x80000, crc: 0x19FFFA37 },
            Op::Load { kind: Load::Word16Swap, file: "1.7f", offset: 0x80000, size: 0x80000, crc: 0x99F1CCA4 },
            Op::Load { kind: Load::Word16Swap, file: "1.6f", offset: 0x100000, size: 0x80000, crc: 0x65C2C719 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_SF2LEVEL: RomSetDef = RomSetDef { name: "sf2level", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rj313.u196.800", offset: 0x0, size: 0x100000, crc: 0x435153D5 },
            Op::Load { kind: Load::Word16Swap, file: "ci030.u10.400", offset: 0x100000, size: 0x80000, crc: 0xED4186BD },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.4a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.5a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.9a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92_10.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92_11.4c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92_12.5c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92_13.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "user1", size: 0x20000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "km6264-10.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "km6264b-10.u191", offset: 0x0, size: 0x8000, crc: 0x6F07D2CB },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "km418c256z-80.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
] };
/// Régions ignorées : plds
pub const RS_SF2CEMS6A: RomSetDef = RomSetDef { name: "sf2cems6a", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ms6.u196", offset: 0x0, size: 0x100000, crc: 0x596609D4 },
            Op::Load { kind: Load::Word16Swap, file: "ms6.u10", offset: 0x100000, size: 0x80000, crc: 0xED4186BD },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ms6.u70", offset: 0x0, size: 0x80000, crc: 0xBAA0F81F },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u68", offset: 0x2, size: 0x80000, crc: 0x8EDFF95A },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u69", offset: 0x200000, size: 0x80000, crc: 0x468962B1 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u64", offset: 0x200002, size: 0x80000, crc: 0x8165F536 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u19", offset: 0x400000, size: 0x80000, crc: 0x39D763D3 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u18", offset: 0x400002, size: 0x80000, crc: 0x2DDFE46E },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u31", offset: 0x400000, size: 0x20000, crc: 0x35486F2D },
            Op::Continue { offset: 0x400004, size: 0x20000 },
            Op::Load { kind: Load::Word64, file: "ms6.u29", offset: 0x400002, size: 0x20000, crc: 0xE4ECA601 },
            Op::Continue { offset: 0x400006, size: 0x20000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u191", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
        RegionDef { name: "user1", size: 0x10000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
] };
/// Régions ignorées : patch, patchpld, plds
pub const RS_SF2CEMS6B: RomSetDef = RomSetDef { name: "sf2cems6b", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ms6b.u196", offset: 0x0, size: 0x100000, crc: 0x435153D5 },
            Op::Load { kind: Load::Word16Swap, file: "ms6b.u10", offset: 0x100000, size: 0x40000, crc: 0xC812B7B2 },
            Op::Copy { src: "patch", src_offset: 0x0, offset: 0x0, size: 0x8000 },
            Op::Copy { src: "patch", src_offset: 0x8000, offset: 0x30000, size: 0x20000 },
            Op::Copy { src: "patch", src_offset: 0x28000, offset: 0x170000, size: 0x10000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ms6.u70", offset: 0x0, size: 0x80000, crc: 0xBAA0F81F },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u68", offset: 0x2, size: 0x80000, crc: 0x8EDFF95A },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u69", offset: 0x200000, size: 0x80000, crc: 0x468962B1 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u64", offset: 0x200002, size: 0x80000, crc: 0x8165F536 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u19", offset: 0x400000, size: 0x80000, crc: 0x39D763D3 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u18", offset: 0x400002, size: 0x80000, crc: 0x2DDFE46E },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u31", offset: 0x400000, size: 0x20000, crc: 0x35486F2D },
            Op::Continue { offset: 0x400004, size: 0x20000 },
            Op::Load { kind: Load::Word64, file: "ms6.u29", offset: 0x400002, size: 0x20000, crc: 0xE4ECA601 },
            Op::Continue { offset: 0x400006, size: 0x20000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u191", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
        RegionDef { name: "user1", size: 0x10000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
] };
/// Régions ignorées : patch, patchpld, plds
pub const RS_SF2CEMS6C: RomSetDef = RomSetDef { name: "sf2cems6c", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "ms6b.u196", offset: 0x0, size: 0x100000, crc: 0x435153D5 },
            Op::Load { kind: Load::Word16Swap, file: "ms6b.u10", offset: 0x100000, size: 0x40000, crc: 0xC812B7B2 },
            Op::Copy { src: "patch", src_offset: 0x0, offset: 0x0, size: 0x8000 },
            Op::Copy { src: "patch", src_offset: 0x8000, offset: 0x30000, size: 0x20000 },
            Op::Copy { src: "patch", src_offset: 0x28000, offset: 0x170000, size: 0x10000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ms6.u70", offset: 0x0, size: 0x80000, crc: 0xBAA0F81F },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u68", offset: 0x2, size: 0x80000, crc: 0x8EDFF95A },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u69", offset: 0x200000, size: 0x80000, crc: 0x468962B1 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u64", offset: 0x200002, size: 0x80000, crc: 0x8165F536 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u19", offset: 0x400000, size: 0x80000, crc: 0x39D763D3 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u18", offset: 0x400002, size: 0x80000, crc: 0x2DDFE46E },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "ms6.u31", offset: 0x400000, size: 0x20000, crc: 0x35486F2D },
            Op::Continue { offset: 0x400004, size: 0x20000 },
            Op::Load { kind: Load::Word64, file: "ms6.u29", offset: 0x400002, size: 0x20000, crc: 0xE4ECA601 },
            Op::Continue { offset: 0x400006, size: 0x20000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u191", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
        RegionDef { name: "user1", size: 0x10000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
] };
/// Régions ignorées : patch, patchpld, plds
pub const RS_SF2CEDS6: RomSetDef = RomSetDef { name: "sf2ceds6", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "6st-u196.2m1", offset: 0x0, size: 0x100000, crc: 0x596609D4 },
            Op::Load { kind: Load::Word16Swap, file: "6st-u210.2m1", offset: 0x100000, size: 0x80000, crc: 0xED4186BD },
            Op::Copy { src: "patch", src_offset: 0x0, offset: 0x0, size: 0x18000 },
            Op::Copy { src: "patch", src_offset: 0x18000, offset: 0x30000, size: 0x18000 },
            Op::Copy { src: "patch", src_offset: 0x30000, offset: 0xe0000, size: 0x8000 },
            Op::Copy { src: "patch", src_offset: 0x38000, offset: 0x108000, size: 0x8000 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "6st.u70", offset: 0x0, size: 0x80000, crc: 0xBAA0F81F },
            Op::Continue { offset: 0x4, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "6st.u68", offset: 0x2, size: 0x80000, crc: 0x8EDFF95A },
            Op::Continue { offset: 0x6, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "6st.u69", offset: 0x200000, size: 0x80000, crc: 0x468962B1 },
            Op::Continue { offset: 0x200004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "6st.u64", offset: 0x200002, size: 0x80000, crc: 0x8165F536 },
            Op::Continue { offset: 0x200006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "6st.u19", offset: 0x400000, size: 0x80000, crc: 0x39D763D3 },
            Op::Continue { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "6st.u18", offset: 0x400002, size: 0x80000, crc: 0x2DDFE46E },
            Op::Continue { offset: 0x400006, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "6st.u31", offset: 0x400000, size: 0x20000, crc: 0x35486F2D },
            Op::Continue { offset: 0x400004, size: 0x20000 },
            Op::Load { kind: Load::Word64, file: "6st.u29", offset: 0x400002, size: 0x20000, crc: 0xE4ECA601 },
            Op::Continue { offset: 0x400006, size: 0x20000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "o.u191", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "fun-s3-j1.u210", offset: 0x0, size: 0x40000, crc: 0x6CFFFB11 },
        ] },
        RegionDef { name: "user1", size: 0x10000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ms6.u133", offset: 0x0, size: 0x10000, crc: 0x13EA1C44 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CWORLD2J: RomSetDef = RomSetDef { name: "cworld2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "q5_36.12f", offset: 0x0, size: 0x20000, crc: 0x38A08099 },
            Op::Load { kind: Load::Byte16, file: "q5_42.12h", offset: 0x1, size: 0x20000, crc: 0x4D29B3A4 },
            Op::Load { kind: Load::Byte16, file: "q5_37.13f", offset: 0x40000, size: 0x20000, crc: 0xEB547EBC },
            Op::Load { kind: Load::Byte16, file: "q5_43.13h", offset: 0x40001, size: 0x20000, crc: 0x3EF65EA8 },
            Op::Load { kind: Load::Byte16, file: "q5_34.10f", offset: 0x80000, size: 0x20000, crc: 0x7FCC1317 },
            Op::Load { kind: Load::Byte16, file: "q5_40.10h", offset: 0x80001, size: 0x20000, crc: 0x7F14B7B4 },
            Op::Load { kind: Load::Byte16, file: "q5_35.11f", offset: 0xc0000, size: 0x20000, crc: 0xABACEE26 },
            Op::Load { kind: Load::Byte16, file: "q5_41.11h", offset: 0xc0001, size: 0x20000, crc: 0xD3654067 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "q5_09.4b", offset: 0x0, size: 0x20000, crc: 0x48496D80 },
            Op::Load { kind: Load::Byte64, file: "q5_01.4a", offset: 0x1, size: 0x20000, crc: 0xC5453F56 },
            Op::Load { kind: Load::Byte64, file: "q5_13.9b", offset: 0x2, size: 0x20000, crc: 0xC741AC52 },
            Op::Load { kind: Load::Byte64, file: "q5_05.9a", offset: 0x3, size: 0x20000, crc: 0x143E068F },
            Op::Load { kind: Load::Byte64, file: "q5_24.5e", offset: 0x4, size: 0x20000, crc: 0xB419D139 },
            Op::Load { kind: Load::Byte64, file: "q5_17.5c", offset: 0x5, size: 0x20000, crc: 0xBD3B4D11 },
            Op::Load { kind: Load::Byte64, file: "q5_38.8h", offset: 0x6, size: 0x20000, crc: 0x9C24670C },
            Op::Load { kind: Load::Byte64, file: "q5_32.8f", offset: 0x7, size: 0x20000, crc: 0x3EF9C7C2 },
            Op::Load { kind: Load::Byte64, file: "q5_10.5b", offset: 0x100000, size: 0x20000, crc: 0x119E5E93 },
            Op::Load { kind: Load::Byte64, file: "q5_02.5a", offset: 0x100001, size: 0x20000, crc: 0xA2CADCBE },
            Op::Load { kind: Load::Byte64, file: "q5_14.10b", offset: 0x100002, size: 0x20000, crc: 0xA8755F82 },
            Op::Load { kind: Load::Byte64, file: "q5_06.10a", offset: 0x100003, size: 0x20000, crc: 0xC92A91FC },
            Op::Load { kind: Load::Byte64, file: "q5_25.7e", offset: 0x100004, size: 0x20000, crc: 0x979237CB },
            Op::Load { kind: Load::Byte64, file: "q5_18.7c", offset: 0x100005, size: 0x20000, crc: 0xC57DA03C },
            Op::Load { kind: Load::Byte64, file: "q5_39.9h", offset: 0x100006, size: 0x20000, crc: 0xA5839B25 },
            Op::Load { kind: Load::Byte64, file: "q5_33.9f", offset: 0x100007, size: 0x20000, crc: 0x04D03930 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "q5_23.13b", offset: 0x0, size: 0x8000, crc: 0xE14DC524 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "q5_30.12c", offset: 0x0, size: 0x20000, crc: 0xD10C1B68 },
            Op::Load { kind: Load::Byte, file: "q5_31.13c", offset: 0x20000, size: 0x20000, crc: 0x7D17E496 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CWORLD2JA: RomSetDef = RomSetDef { name: "cworld2ja", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "q5 - 34_90629b.8f", offset: 0x0, size: 0x80000, crc: 0xDE54487F },
            Op::Load { kind: Load::Word16Swap, file: "q5 - 33_90629b.6f", offset: 0x80000, size: 0x80000, crc: 0x93248458 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "q5 - 06_90629b.8a", offset: 0x0, size: 0x80000, crc: 0x09D0E7CE },
            Op::Load { kind: Load::Word64, file: "q5 - 08_90629b.10a", offset: 0x2, size: 0x80000, crc: 0x22E4CE9A },
            Op::Load { kind: Load::Word64, file: "q5 - 05_90629b.7a", offset: 0x4, size: 0x80000, crc: 0xF7B3AED6 },
            Op::Load { kind: Load::Word64, file: "q5 - 07_90629b.9a", offset: 0x6, size: 0x80000, crc: 0x520C6C88 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "q5 - 09_90629b.12a", offset: 0x0, size: 0x8000, crc: 0xE14DC524 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "q5 - 18_90629b.11c", offset: 0x0, size: 0x20000, crc: 0xD10C1B68 },
            Op::Load { kind: Load::Byte, file: "q5 - 19_90629b.12c", offset: 0x20000, size: 0x20000, crc: 0x7D17E496 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_CWORLD2JB: RomSetDef = RomSetDef { name: "cworld2jb", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "q5 - 23_91634b.8f", offset: 0x0, size: 0x80000, crc: 0x709F577F },
            Op::Load { kind: Load::Word16Swap, file: "q5 - 22_91634b.7f", offset: 0x80000, size: 0x80000, crc: 0x93248458 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "q5 - 01_91634b.3a", offset: 0x0, size: 0x80000, crc: 0x09D0E7CE },
            Op::Load { kind: Load::Word64, file: "q5 - 02_91634b.4a", offset: 0x2, size: 0x80000, crc: 0x22E4CE9A },
            Op::Load { kind: Load::Word64, file: "q5 - 03_91634b.5a", offset: 0x4, size: 0x80000, crc: 0xF7B3AED6 },
            Op::Load { kind: Load::Word64, file: "q5 - 04_91634b.6a", offset: 0x6, size: 0x80000, crc: 0x520C6C88 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "q5 - 09_91634b.12a", offset: 0x0, size: 0x8000, crc: 0xE14DC524 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "q5 - 18_91634b.11c", offset: 0x0, size: 0x20000, crc: 0xD10C1B68 },
            Op::Load { kind: Load::Byte, file: "q5 - 19_91634b.12c", offset: 0x20000, size: 0x20000, crc: 0x7D17E496 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_VARTH: RomSetDef = RomSetDef { name: "varth", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "vae_30b.11f", offset: 0x0, size: 0x20000, crc: 0xADB8D391 },
            Op::Load { kind: Load::Byte16, file: "vae_35b.11h", offset: 0x1, size: 0x20000, crc: 0x44E5548F },
            Op::Load { kind: Load::Byte16, file: "vae_31b.12f", offset: 0x40000, size: 0x20000, crc: 0x1749A71C },
            Op::Load { kind: Load::Byte16, file: "vae_36b.12h", offset: 0x40001, size: 0x20000, crc: 0x5F2E2450 },
            Op::Load { kind: Load::Byte16, file: "vae_28b.9f", offset: 0x80000, size: 0x20000, crc: 0xE524CA50 },
            Op::Load { kind: Load::Byte16, file: "vae_33b.9h", offset: 0x80001, size: 0x20000, crc: 0xC0BBF8C9 },
            Op::Load { kind: Load::Byte16, file: "vae_29b.10f", offset: 0xc0000, size: 0x20000, crc: 0x6640996A },
            Op::Load { kind: Load::Byte16, file: "vae_34b.10h", offset: 0xc0001, size: 0x20000, crc: 0xFA59BE8A },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "va-5m.7a", offset: 0x0, size: 0x80000, crc: 0xB1FB726E },
            Op::Load { kind: Load::Word64, file: "va-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4C6588CD },
            Op::Load { kind: Load::Word64, file: "va-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B1ACE37 },
            Op::Load { kind: Load::Word64, file: "va-3m.5a", offset: 0x6, size: 0x80000, crc: 0x44DFE706 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_09.12b", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_18.11c", offset: 0x0, size: 0x20000, crc: 0xDE30510E },
            Op::Load { kind: Load::Byte, file: "va_19.12c", offset: 0x20000, size: 0x20000, crc: 0x0610A4AC },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_VARTHR1: RomSetDef = RomSetDef { name: "varthr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "vae_30a.11f", offset: 0x0, size: 0x20000, crc: 0x7FCD0091 },
            Op::Load { kind: Load::Byte16, file: "vae_35a.11h", offset: 0x1, size: 0x20000, crc: 0x35CF9509 },
            Op::Load { kind: Load::Byte16, file: "vae_31a.12f", offset: 0x40000, size: 0x20000, crc: 0x15E5EE81 },
            Op::Load { kind: Load::Byte16, file: "vae_36a.12h", offset: 0x40001, size: 0x20000, crc: 0x153A201E },
            Op::Load { kind: Load::Byte16, file: "vae_28a.9f", offset: 0x80000, size: 0x20000, crc: 0x7A0E0D25 },
            Op::Load { kind: Load::Byte16, file: "vae_33a.9h", offset: 0x80001, size: 0x20000, crc: 0xF2365922 },
            Op::Load { kind: Load::Byte16, file: "vae_29a.10f", offset: 0xc0000, size: 0x20000, crc: 0x5E2CD2C3 },
            Op::Load { kind: Load::Byte16, file: "vae_34a.10h", offset: 0xc0001, size: 0x20000, crc: 0x3D9BDF83 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "va-5m.7a", offset: 0x0, size: 0x80000, crc: 0xB1FB726E },
            Op::Load { kind: Load::Word64, file: "va-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4C6588CD },
            Op::Load { kind: Load::Word64, file: "va-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B1ACE37 },
            Op::Load { kind: Load::Word64, file: "va-3m.5a", offset: 0x6, size: 0x80000, crc: 0x44DFE706 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_09.12b", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_18.11c", offset: 0x0, size: 0x20000, crc: 0xDE30510E },
            Op::Load { kind: Load::Byte, file: "va_19.12c", offset: 0x20000, size: 0x20000, crc: 0x0610A4AC },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_VARTHU: RomSetDef = RomSetDef { name: "varthu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vau_23a.8f", offset: 0x0, size: 0x80000, crc: 0xFBE68726 },
            Op::Load { kind: Load::Word16Swap, file: "vau_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x0ED71BBD },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "va-5m.3a", offset: 0x0, size: 0x80000, crc: 0xB1FB726E },
            Op::Load { kind: Load::Word64, file: "va-7m.5a", offset: 0x2, size: 0x80000, crc: 0x4C6588CD },
            Op::Load { kind: Load::Word64, file: "va-1m.4a", offset: 0x4, size: 0x80000, crc: 0x0B1ACE37 },
            Op::Load { kind: Load::Word64, file: "va-3m.6a", offset: 0x6, size: 0x80000, crc: 0x44DFE706 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_09.11a", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_18.11c", offset: 0x0, size: 0x20000, crc: 0xDE30510E },
            Op::Load { kind: Load::Byte, file: "va_19.12c", offset: 0x20000, size: 0x20000, crc: 0x0610A4AC },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_VARTHJ: RomSetDef = RomSetDef { name: "varthj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "vaj_36b.12f", offset: 0x0, size: 0x20000, crc: 0x1D798D6A },
            Op::Load { kind: Load::Byte16, file: "vaj_42b.12h", offset: 0x1, size: 0x20000, crc: 0x0F720233 },
            Op::Load { kind: Load::Byte16, file: "vaj_37b.13f", offset: 0x40000, size: 0x20000, crc: 0x24414B17 },
            Op::Load { kind: Load::Byte16, file: "vaj_43b.13h", offset: 0x40001, size: 0x20000, crc: 0x34B4B06C },
            Op::Load { kind: Load::Byte16, file: "vaj_34b.10f", offset: 0x80000, size: 0x20000, crc: 0x87C79AED },
            Op::Load { kind: Load::Byte16, file: "vaj_40b.10h", offset: 0x80001, size: 0x20000, crc: 0x210B4BD0 },
            Op::Load { kind: Load::Byte16, file: "vaj_35b.11f", offset: 0xc0000, size: 0x20000, crc: 0x6B0DA69F },
            Op::Load { kind: Load::Byte16, file: "vaj_41b.11h", offset: 0xc0001, size: 0x20000, crc: 0x6542C8A4 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "va_09.4b", offset: 0x0, size: 0x20000, crc: 0x183DFAA8 },
            Op::Load { kind: Load::Byte64, file: "va_01.4a", offset: 0x1, size: 0x20000, crc: 0xC41312B5 },
            Op::Load { kind: Load::Byte64, file: "va_13.9b", offset: 0x2, size: 0x20000, crc: 0x45537E69 },
            Op::Load { kind: Load::Byte64, file: "va_05.9a", offset: 0x3, size: 0x20000, crc: 0x7065D4E9 },
            Op::Load { kind: Load::Byte64, file: "va_24.5e", offset: 0x4, size: 0x20000, crc: 0x57191CCF },
            Op::Load { kind: Load::Byte64, file: "va_17.5c", offset: 0x5, size: 0x20000, crc: 0x054F5A5B },
            Op::Load { kind: Load::Byte64, file: "va_38.8h", offset: 0x6, size: 0x20000, crc: 0xE117A17E },
            Op::Load { kind: Load::Byte64, file: "va_32.8f", offset: 0x7, size: 0x20000, crc: 0x3B4F40B2 },
            Op::Load { kind: Load::Byte64, file: "va_10.5b", offset: 0x100000, size: 0x20000, crc: 0xD62750CD },
            Op::Load { kind: Load::Byte64, file: "va_02.5a", offset: 0x100001, size: 0x20000, crc: 0x11590325 },
            Op::Load { kind: Load::Byte64, file: "va_14.10b", offset: 0x100002, size: 0x20000, crc: 0xDC2F4783 },
            Op::Load { kind: Load::Byte64, file: "va_06.10a", offset: 0x100003, size: 0x20000, crc: 0x06E833AC },
            Op::Load { kind: Load::Byte64, file: "va_25.7e", offset: 0x100004, size: 0x20000, crc: 0x51D90690 },
            Op::Load { kind: Load::Byte64, file: "va_18.7c", offset: 0x100005, size: 0x20000, crc: 0xA17817C0 },
            Op::Load { kind: Load::Byte64, file: "va_39.9h", offset: 0x100006, size: 0x20000, crc: 0xB0B12F51 },
            Op::Load { kind: Load::Byte64, file: "va_33.9f", offset: 0x100007, size: 0x20000, crc: 0x4B003AF7 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_23.13c", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_30.12e", offset: 0x0, size: 0x20000, crc: 0xDE30510E },
            Op::Load { kind: Load::Byte, file: "va_31.13e", offset: 0x20000, size: 0x20000, crc: 0x0610A4AC },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_VARTHJR: RomSetDef = RomSetDef { name: "varthjr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "vaj_23b.8f", offset: 0x0, size: 0x80000, crc: 0xAD3D3522 },
            Op::Load { kind: Load::Word16Swap, file: "vaj_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x034E3E55 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "va_01.3a", offset: 0x0, size: 0x80000, crc: 0xB1FB726E },
            Op::Load { kind: Load::Word64, file: "va_02.4a", offset: 0x2, size: 0x80000, crc: 0x4C6588CD },
            Op::Load { kind: Load::Word64, file: "va_03.5a", offset: 0x4, size: 0x80000, crc: 0x0B1ACE37 },
            Op::Load { kind: Load::Word64, file: "va_04.6a", offset: 0x6, size: 0x80000, crc: 0x44DFE706 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_09.12a", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_18.11c", offset: 0x0, size: 0x20000, crc: 0xDE30510E },
            Op::Load { kind: Load::Byte, file: "va_19.12c", offset: 0x20000, size: 0x20000, crc: 0x0610A4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_VARTHB2: RomSetDef = RomSetDef { name: "varthb2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "varth-u222", offset: 0x0, size: 0x80000, crc: 0xFD2F93A8 },
            Op::Load { kind: Load::Byte16, file: "varth-u196", offset: 0x1, size: 0x80000, crc: 0x3B187DC3 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "va-5m.7a", offset: 0x0, size: 0x80000, crc: 0xB1FB726E },
            Op::Reload { offset: 0x400000, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "va-7m.9a", offset: 0x2, size: 0x80000, crc: 0x4C6588CD },
            Op::Reload { offset: 0x400002, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "va-1m.3a", offset: 0x4, size: 0x80000, crc: 0x0B1ACE37 },
            Op::Reload { offset: 0x400004, size: 0x80000 },
            Op::Load { kind: Load::Word64, file: "va-3m.5a", offset: 0x6, size: 0x80000, crc: 0x44DFE706 },
            Op::Reload { offset: 0x400006, size: 0x80000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_09.12b", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "va_18.11c", offset: 0x0, size: 0x20000, crc: 0xDE30510E },
            Op::Load { kind: Load::Byte, file: "va_19.12c", offset: 0x20000, size: 0x20000, crc: 0x0610A4AC },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_VARTHB3: RomSetDef = RomSetDef { name: "varthb3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "pd1.bin", offset: 0x0, size: 0x80000, crc: 0x8208C0D7 },
            Op::Load { kind: Load::Byte16, file: "c6.bin", offset: 0x1, size: 0x80000, crc: 0x7F654421 },
            Op::Load { kind: Load::Byte16, file: "pd2.bin", offset: 0x100000, size: 0x20000, crc: 0xD8325C94 },
            Op::Load { kind: Load::Byte16, file: "c5.bin", offset: 0x100001, size: 0x20000, crc: 0x6152277D },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte32, file: "rom1.bin", offset: 0x0, size: 0x80000, crc: 0x473961B3 },
            Op::Load { kind: Load::Byte32, file: "rom3.bin", offset: 0x1, size: 0x80000, crc: 0x9B50384F },
            Op::Load { kind: Load::Byte32, file: "rom5.bin", offset: 0x2, size: 0x80000, crc: 0x516A4EEA },
            Op::Load { kind: Load::Byte32, file: "rom7.bin", offset: 0x3, size: 0x80000, crc: 0x81023052 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "j4.bin", offset: 0x0, size: 0x8000, crc: 0x7A99446E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "j1.bin", offset: 0x0, size: 0x40000, crc: 0x1547E595 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_QAD: RomSetDef = RomSetDef { name: "qad", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "qdu_36a.12f", offset: 0x0, size: 0x20000, crc: 0xDE9C24A0 },
            Op::Load { kind: Load::Byte16, file: "qdu_42a.12h", offset: 0x1, size: 0x20000, crc: 0xCFE36F0C },
            Op::Load { kind: Load::Byte16, file: "qdu_37a.13f", offset: 0x40000, size: 0x20000, crc: 0x10D22320 },
            Op::Load { kind: Load::Byte16, file: "qdu_43a.13h", offset: 0x40001, size: 0x20000, crc: 0x15E6BEB9 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "qd_09.4b", offset: 0x0, size: 0x20000, crc: 0x8C3F9F44 },
            Op::Load { kind: Load::Byte64, file: "qd_01.4a", offset: 0x1, size: 0x20000, crc: 0xF688CF8F },
            Op::Load { kind: Load::Byte64, file: "qd_13.9b", offset: 0x2, size: 0x20000, crc: 0xAFBD551B },
            Op::Load { kind: Load::Byte64, file: "qd_05.9a", offset: 0x3, size: 0x20000, crc: 0xC3DB0910 },
            Op::Load { kind: Load::Byte64, file: "qd_24.5e", offset: 0x4, size: 0x20000, crc: 0x2F1BD0EC },
            Op::Load { kind: Load::Byte64, file: "qd_17.5c", offset: 0x5, size: 0x20000, crc: 0xA812F9E2 },
            Op::Load { kind: Load::Byte64, file: "qd_38.8h", offset: 0x6, size: 0x20000, crc: 0xCCDDDD1F },
            Op::Load { kind: Load::Byte64, file: "qd_32.8f", offset: 0x7, size: 0x20000, crc: 0xA8D295D3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "qd_23.13b", offset: 0x0, size: 0x8000, crc: 0xCFB5264B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "qdu_30.12c", offset: 0x0, size: 0x20000, crc: 0xF190DA84 },
            Op::Load { kind: Load::Byte, file: "qdu_31.13c", offset: 0x20000, size: 0x20000, crc: 0xB7583F73 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_QADJR: RomSetDef = RomSetDef { name: "qadjr", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "qad_23a.8f", offset: 0x0, size: 0x80000, crc: 0x4D3553DE },
            Op::Load { kind: Load::Word16Swap, file: "qad_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x3191DDD0 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "qad_01.3a", offset: 0x0, size: 0x80000, crc: 0x9D853B57 },
            Op::Load { kind: Load::Word64, file: "qad_02.4a", offset: 0x2, size: 0x80000, crc: 0xB35976C4 },
            Op::Load { kind: Load::Word64, file: "qad_03.5a", offset: 0x4, size: 0x80000, crc: 0xCEA4CA8C },
            Op::Load { kind: Load::Word64, file: "qad_04.6a", offset: 0x6, size: 0x80000, crc: 0x41B74D1B },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "qad_09.12a", offset: 0x0, size: 0x8000, crc: 0x733161CC },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "qad_18.11c", offset: 0x0, size: 0x20000, crc: 0x2BFE6F6A },
            Op::Load { kind: Load::Byte, file: "qad_19.12c", offset: 0x20000, size: 0x20000, crc: 0x13D3236B },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_WOF: RomSetDef = RomSetDef { name: "wof", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk2e_23c.8f", offset: 0x0, size: 0x80000, crc: 0x0D708505 },
            Op::Load { kind: Load::Word16Swap, file: "tk2e_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x608C17E3 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk2-1m.3a", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "tk2-3m.5a", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "tk2-2m.4a", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "tk2-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "tk2-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x291F0F0B },
            Op::Load { kind: Load::Word64, file: "tk2-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x3EDEB949 },
            Op::Load { kind: Load::Word64, file: "tk2-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x1ABD14D6 },
            Op::Load { kind: Load::Word64, file: "tk2-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xB27948E3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2_qa.5k", offset: 0x0, size: 0x8000, crc: 0xC9183A0D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2-q1.1k", offset: 0x0, size: 0x80000, crc: 0x611268CF },
            Op::Load { kind: Load::Byte, file: "tk2-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x20F55CA9 },
            Op::Load { kind: Load::Byte, file: "tk2-q3.3k", offset: 0x100000, size: 0x80000, crc: 0xBFCF6F52 },
            Op::Load { kind: Load::Byte, file: "tk2-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x36642E88 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_WOFR1: RomSetDef = RomSetDef { name: "wofr1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk2e_23b.8f", offset: 0x0, size: 0x80000, crc: 0x11FB2ED1 },
            Op::Load { kind: Load::Word16Swap, file: "tk2e_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x479B3F24 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk2-1m.3a", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "tk2-3m.5a", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "tk2-2m.4a", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "tk2-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "tk2-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x291F0F0B },
            Op::Load { kind: Load::Word64, file: "tk2-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x3EDEB949 },
            Op::Load { kind: Load::Word64, file: "tk2-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x1ABD14D6 },
            Op::Load { kind: Load::Word64, file: "tk2-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xB27948E3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2_qa.5k", offset: 0x0, size: 0x8000, crc: 0xC9183A0D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2-q1.1k", offset: 0x0, size: 0x80000, crc: 0x611268CF },
            Op::Load { kind: Load::Byte, file: "tk2-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x20F55CA9 },
            Op::Load { kind: Load::Byte, file: "tk2-q3.3k", offset: 0x100000, size: 0x80000, crc: 0xBFCF6F52 },
            Op::Load { kind: Load::Byte, file: "tk2-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x36642E88 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_WOFU: RomSetDef = RomSetDef { name: "wofu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk2u_23c.8f", offset: 0x0, size: 0x80000, crc: 0x29B89C12 },
            Op::Load { kind: Load::Word16Swap, file: "tk2u_22c.7f", offset: 0x80000, size: 0x80000, crc: 0xF5AF4774 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk2-1m.3a", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "tk2-3m.5a", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "tk2-2m.4a", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "tk2-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "tk2-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x291F0F0B },
            Op::Load { kind: Load::Word64, file: "tk2-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x3EDEB949 },
            Op::Load { kind: Load::Word64, file: "tk2-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x1ABD14D6 },
            Op::Load { kind: Load::Word64, file: "tk2-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xB27948E3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2_qa.5k", offset: 0x0, size: 0x8000, crc: 0xC9183A0D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2-q1.1k", offset: 0x0, size: 0x80000, crc: 0x611268CF },
            Op::Load { kind: Load::Byte, file: "tk2-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x20F55CA9 },
            Op::Load { kind: Load::Byte, file: "tk2-q3.3k", offset: 0x100000, size: 0x80000, crc: 0xBFCF6F52 },
            Op::Load { kind: Load::Byte, file: "tk2-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x36642E88 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_WOFA: RomSetDef = RomSetDef { name: "wofa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk2a_23c.8f", offset: 0x0, size: 0x80000, crc: 0x2E024628 },
            Op::Load { kind: Load::Word16Swap, file: "tk2a_22c.7f", offset: 0x80000, size: 0x80000, crc: 0x900AD4CD },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk2-1m.3a", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "tk2-3m.5a", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "tk2-2m.4a", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "tk2-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "tk2-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x291F0F0B },
            Op::Load { kind: Load::Word64, file: "tk2-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x3EDEB949 },
            Op::Load { kind: Load::Word64, file: "tk2-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x1ABD14D6 },
            Op::Load { kind: Load::Word64, file: "tk2-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xB27948E3 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2_qa.5k", offset: 0x0, size: 0x8000, crc: 0xC9183A0D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2-q1.1k", offset: 0x0, size: 0x80000, crc: 0x611268CF },
            Op::Load { kind: Load::Byte, file: "tk2-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x20F55CA9 },
            Op::Load { kind: Load::Byte, file: "tk2-q3.3k", offset: 0x100000, size: 0x80000, crc: 0xBFCF6F52 },
            Op::Load { kind: Load::Byte, file: "tk2-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x36642E88 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_WOFJ: RomSetDef = RomSetDef { name: "wofj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk2j_23c.8f", offset: 0x0, size: 0x80000, crc: 0x9B215A68 },
            Op::Load { kind: Load::Word16Swap, file: "tk2j_22c.7f", offset: 0x80000, size: 0x80000, crc: 0xB74B09AC },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk2_01.3a", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "tk2_02.4a", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "tk2_03.5a", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "tk2_04.6a", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "tk2_05.7a", offset: 0x200000, size: 0x80000, crc: 0xE4A44D53 },
            Op::Load { kind: Load::Word64, file: "tk2_06.8a", offset: 0x200002, size: 0x80000, crc: 0x58066BA8 },
            Op::Load { kind: Load::Word64, file: "tk2_07.9a", offset: 0x200004, size: 0x80000, crc: 0xD706568E },
            Op::Load { kind: Load::Word64, file: "tk2_08.10a", offset: 0x200006, size: 0x80000, crc: 0xD4A19A02 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2_qa.5k", offset: 0x0, size: 0x8000, crc: 0xC9183A0D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2-q1.1k", offset: 0x0, size: 0x80000, crc: 0x611268CF },
            Op::Load { kind: Load::Byte, file: "tk2-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x20F55CA9 },
            Op::Load { kind: Load::Byte, file: "tk2-q3.3k", offset: 0x100000, size: 0x80000, crc: 0xBFCF6F52 },
            Op::Load { kind: Load::Byte, file: "tk2-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x36642E88 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_WOFHFH: RomSetDef = RomSetDef { name: "wofhfh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "23", offset: 0x0, size: 0x80000, crc: 0x6AE4B312 },
            Op::Load { kind: Load::Word16Swap, file: "22", offset: 0x80000, size: 0x80000, crc: 0x94E8D01A },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "2", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "3", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "4", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "5", offset: 0x200000, size: 0x80000, crc: 0x34949D7B },
            Op::Load { kind: Load::Word64, file: "6", offset: 0x200002, size: 0x80000, crc: 0xDFA70971 },
            Op::Load { kind: Load::Word64, file: "7", offset: 0x200004, size: 0x80000, crc: 0x073686A6 },
            Op::Load { kind: Load::Word64, file: "8", offset: 0x200006, size: 0x80000, crc: 0x5300F8DB },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "9", offset: 0x0, size: 0x8000, crc: 0x86FE8A97 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "18", offset: 0x0, size: 0x20000, crc: 0xC04BE720 },
            Op::Load { kind: Load::Byte, file: "19", offset: 0x20000, size: 0x20000, crc: 0xFBB8D8C1 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2HF: RomSetDef = RomSetDef { name: "sf2hf", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s2te_23.8f", offset: 0x0, size: 0x80000, crc: 0x2DD72514 },
            Op::Load { kind: Load::Word16Swap, file: "s2te_22.7f", offset: 0x80000, size: 0x80000, crc: 0xAEA6E035 },
            Op::Load { kind: Load::Word16Swap, file: "s2te_21.6f", offset: 0x100000, size: 0x80000, crc: 0xFD200288 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2HFU: RomSetDef = RomSetDef { name: "sf2hfu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s2tu_23.8f", offset: 0x0, size: 0x80000, crc: 0x89A1FC38 },
            Op::Load { kind: Load::Word16Swap, file: "s2tu_22.7f", offset: 0x80000, size: 0x80000, crc: 0xAEA6E035 },
            Op::Load { kind: Load::Word16Swap, file: "s2tu_21.6f", offset: 0x100000, size: 0x80000, crc: 0xFD200288 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92-1m.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92-3m.5a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92-2m.4a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92-6m.8a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s92-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x960687D5 },
            Op::Load { kind: Load::Word64, file: "s92-12m.5c", offset: 0x400002, size: 0x80000, crc: 0x978ECD18 },
            Op::Load { kind: Load::Word64, file: "s92-11m.4c", offset: 0x400004, size: 0x80000, crc: 0xD6EC9A0A },
            Op::Load { kind: Load::Word64, file: "s92-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xED2C67F6 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.11a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_SF2HFJ: RomSetDef = RomSetDef { name: "sf2hfj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "s2tj_23.8f", offset: 0x0, size: 0x80000, crc: 0xEA73B4DC },
            Op::Load { kind: Load::Word16Swap, file: "s2tj_22.7f", offset: 0x80000, size: 0x80000, crc: 0xAEA6E035 },
            Op::Load { kind: Load::Word16Swap, file: "s2tj_21.6f", offset: 0x100000, size: 0x80000, crc: 0xFD200288 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "s92_01.3a", offset: 0x0, size: 0x80000, crc: 0x03B0D852 },
            Op::Load { kind: Load::Word64, file: "s92_02.4a", offset: 0x2, size: 0x80000, crc: 0x840289EC },
            Op::Load { kind: Load::Word64, file: "s92_03.5a", offset: 0x4, size: 0x80000, crc: 0xCDB5F027 },
            Op::Load { kind: Load::Word64, file: "s92_04.6a", offset: 0x6, size: 0x80000, crc: 0xE2799472 },
            Op::Load { kind: Load::Word64, file: "s92_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8A2761 },
            Op::Load { kind: Load::Word64, file: "s92_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE584BFB5 },
            Op::Load { kind: Load::Word64, file: "s92_07.9a", offset: 0x200004, size: 0x80000, crc: 0x21E3F87D },
            Op::Load { kind: Load::Word64, file: "s92_08.10a", offset: 0x200006, size: 0x80000, crc: 0xBEFC47DF },
            Op::Load { kind: Load::Word64, file: "s2t_10.3c", offset: 0x400000, size: 0x80000, crc: 0x3C042686 },
            Op::Load { kind: Load::Word64, file: "s2t_11.4c", offset: 0x400002, size: 0x80000, crc: 0x8B7E7183 },
            Op::Load { kind: Load::Word64, file: "s2t_12.5c", offset: 0x400004, size: 0x80000, crc: 0x293C888C },
            Op::Load { kind: Load::Word64, file: "s2t_13.6c", offset: 0x400006, size: 0x80000, crc: 0x842B35A4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_09.12a", offset: 0x0, size: 0x8000, crc: 0x08F6B60E },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "s92_18.11c", offset: 0x0, size: 0x20000, crc: 0x7F162009 },
            Op::Load { kind: Load::Byte, file: "s92_19.12c", offset: 0x20000, size: 0x20000, crc: 0xBEADE53F },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_DINO: RomSetDef = RomSetDef { name: "dino", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cde_23a.8f", offset: 0x0, size: 0x80000, crc: 0x8F4E585E },
            Op::Load { kind: Load::Word16Swap, file: "cde_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x9278AA12 },
            Op::Load { kind: Load::Word16Swap, file: "cde_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x66D23DE2 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cd-1m.3a", offset: 0x0, size: 0x80000, crc: 0x8DA4F917 },
            Op::Load { kind: Load::Word64, file: "cd-3m.5a", offset: 0x2, size: 0x80000, crc: 0x6C40F603 },
            Op::Load { kind: Load::Word64, file: "cd-2m.4a", offset: 0x4, size: 0x80000, crc: 0x09C8FC2D },
            Op::Load { kind: Load::Word64, file: "cd-4m.6a", offset: 0x6, size: 0x80000, crc: 0x637FF38F },
            Op::Load { kind: Load::Word64, file: "cd-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x470BEFEE },
            Op::Load { kind: Load::Word64, file: "cd-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x22BFB7A3 },
            Op::Load { kind: Load::Word64, file: "cd-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xE7599AC4 },
            Op::Load { kind: Load::Word64, file: "cd-8m.10a", offset: 0x200006, size: 0x80000, crc: 0x211B4B15 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd_q.5k", offset: 0x0, size: 0x8000, crc: 0x605FDB0B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd-q1.1k", offset: 0x0, size: 0x80000, crc: 0x60927775 },
            Op::Load { kind: Load::Byte, file: "cd-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x770F4C47 },
            Op::Load { kind: Load::Byte, file: "cd-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x2F273FFC },
            Op::Load { kind: Load::Byte, file: "cd-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x2C67821D },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_DINOU: RomSetDef = RomSetDef { name: "dinou", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cdu_23a.8f", offset: 0x0, size: 0x80000, crc: 0x7C2543CD },
            Op::Load { kind: Load::Word16Swap, file: "cdu_22a.7f", offset: 0x80000, size: 0x80000, crc: 0xD19F981E },
            Op::Load { kind: Load::Word16Swap, file: "cdu_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x66D23DE2 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cd-1m.3a", offset: 0x0, size: 0x80000, crc: 0x8DA4F917 },
            Op::Load { kind: Load::Word64, file: "cd-3m.5a", offset: 0x2, size: 0x80000, crc: 0x6C40F603 },
            Op::Load { kind: Load::Word64, file: "cd-2m.4a", offset: 0x4, size: 0x80000, crc: 0x09C8FC2D },
            Op::Load { kind: Load::Word64, file: "cd-4m.6a", offset: 0x6, size: 0x80000, crc: 0x637FF38F },
            Op::Load { kind: Load::Word64, file: "cd-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x470BEFEE },
            Op::Load { kind: Load::Word64, file: "cd-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x22BFB7A3 },
            Op::Load { kind: Load::Word64, file: "cd-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xE7599AC4 },
            Op::Load { kind: Load::Word64, file: "cd-8m.10a", offset: 0x200006, size: 0x80000, crc: 0x211B4B15 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd_q.5k", offset: 0x0, size: 0x8000, crc: 0x605FDB0B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd-q1.1k", offset: 0x0, size: 0x80000, crc: 0x60927775 },
            Op::Load { kind: Load::Byte, file: "cd-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x770F4C47 },
            Op::Load { kind: Load::Byte, file: "cd-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x2F273FFC },
            Op::Load { kind: Load::Byte, file: "cd-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x2C67821D },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_DINOA: RomSetDef = RomSetDef { name: "dinoa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cdt_23.8f", offset: 0x0, size: 0x80000, crc: 0xF477F7A0 },
            Op::Load { kind: Load::Word16Swap, file: "cdt_22.7f", offset: 0x80000, size: 0x80000, crc: 0x1E534CA5 },
            Op::Load { kind: Load::Word16Swap, file: "cdt_21.6f", offset: 0x100000, size: 0x80000, crc: 0x74B04329 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cd-1m.3a", offset: 0x0, size: 0x80000, crc: 0x8DA4F917 },
            Op::Load { kind: Load::Word64, file: "cd-3m.5a", offset: 0x2, size: 0x80000, crc: 0x6C40F603 },
            Op::Load { kind: Load::Word64, file: "cd-2m.4a", offset: 0x4, size: 0x80000, crc: 0x09C8FC2D },
            Op::Load { kind: Load::Word64, file: "cd-4m.6a", offset: 0x6, size: 0x80000, crc: 0x637FF38F },
            Op::Load { kind: Load::Word64, file: "cd-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x470BEFEE },
            Op::Load { kind: Load::Word64, file: "cd-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x22BFB7A3 },
            Op::Load { kind: Load::Word64, file: "cd-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xE7599AC4 },
            Op::Load { kind: Load::Word64, file: "cd-8m.10a", offset: 0x200006, size: 0x80000, crc: 0x211B4B15 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd_q.5k", offset: 0x0, size: 0x8000, crc: 0x605FDB0B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd-q1.1k", offset: 0x0, size: 0x80000, crc: 0x60927775 },
            Op::Load { kind: Load::Byte, file: "cd-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x770F4C47 },
            Op::Load { kind: Load::Byte, file: "cd-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x2F273FFC },
            Op::Load { kind: Load::Byte, file: "cd-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x2C67821D },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_DINOJ: RomSetDef = RomSetDef { name: "dinoj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "cdj_23a.8f", offset: 0x0, size: 0x80000, crc: 0x5F3ECE96 },
            Op::Load { kind: Load::Word16Swap, file: "cdj_22a.7f", offset: 0x80000, size: 0x80000, crc: 0xA0D8DE29 },
            Op::Load { kind: Load::Word16Swap, file: "cdj_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x66D23DE2 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "cd_01.3a", offset: 0x0, size: 0x80000, crc: 0x8DA4F917 },
            Op::Load { kind: Load::Word64, file: "cd_02.4a", offset: 0x2, size: 0x80000, crc: 0x6C40F603 },
            Op::Load { kind: Load::Word64, file: "cd_03.5a", offset: 0x4, size: 0x80000, crc: 0x09C8FC2D },
            Op::Load { kind: Load::Word64, file: "cd_04.6a", offset: 0x6, size: 0x80000, crc: 0x637FF38F },
            Op::Load { kind: Load::Word64, file: "cd_05.7a", offset: 0x200000, size: 0x80000, crc: 0x470BEFEE },
            Op::Load { kind: Load::Word64, file: "cd_06.8a", offset: 0x200002, size: 0x80000, crc: 0x22BFB7A3 },
            Op::Load { kind: Load::Word64, file: "cd_07.9a", offset: 0x200004, size: 0x80000, crc: 0xE7599AC4 },
            Op::Load { kind: Load::Word64, file: "cd_08.10a", offset: 0x200006, size: 0x80000, crc: 0x211B4B15 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd_q.5k", offset: 0x0, size: 0x8000, crc: 0x605FDB0B },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "cd-q1.1k", offset: 0x0, size: 0x80000, crc: 0x60927775 },
            Op::Load { kind: Load::Byte, file: "cd-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x770F4C47 },
            Op::Load { kind: Load::Byte, file: "cd-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x2F273FFC },
            Op::Load { kind: Load::Byte, file: "cd-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x2C67821D },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_DINOHUNT: RomSetDef = RomSetDef { name: "dinohunt", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "u23", offset: 0x0, size: 0x80000, crc: 0x8D5DDC5D },
            Op::Load { kind: Load::Word16Swap, file: "u22", offset: 0x80000, size: 0x80000, crc: 0xF72CD219 },
            Op::Load { kind: Load::Word16Swap, file: "u21", offset: 0x100000, size: 0x80000, crc: 0xBC275B76 },
            Op::Load { kind: Load::Word16Swap, file: "u20", offset: 0x180000, size: 0x80000, crc: 0x8987C975 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "u1", offset: 0x0, size: 0x80000, crc: 0xA01A9FB5 },
            Op::Load { kind: Load::Word64, file: "u2", offset: 0x2, size: 0x80000, crc: 0xBDF02C17 },
            Op::Load { kind: Load::Word64, file: "u3", offset: 0x4, size: 0x80000, crc: 0x058BEEFA },
            Op::Load { kind: Load::Word64, file: "u4", offset: 0x6, size: 0x80000, crc: 0x5028A9F1 },
            Op::Load { kind: Load::Word64, file: "u5", offset: 0x200000, size: 0x80000, crc: 0xD77F89EA },
            Op::Load { kind: Load::Word64, file: "u6", offset: 0x200002, size: 0x80000, crc: 0xBFBCB034 },
            Op::Load { kind: Load::Word64, file: "u7", offset: 0x200004, size: 0x80000, crc: 0xA2544D4E },
            Op::Load { kind: Load::Word64, file: "u8", offset: 0x200006, size: 0x80000, crc: 0x8869BBB1 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "u9", offset: 0x0, size: 0x8000, crc: 0x2EB16A83 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "u18", offset: 0x0, size: 0x20000, crc: 0x8D2899BA },
            Op::Load { kind: Load::Byte, file: "u19", offset: 0x20000, size: 0x20000, crc: 0xB34A4B42 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_PUNISHER: RomSetDef = RomSetDef { name: "punisher", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "pse_26.11e", offset: 0x0, size: 0x20000, crc: 0x389A99D2 },
            Op::Load { kind: Load::Byte16, file: "pse_30.11f", offset: 0x1, size: 0x20000, crc: 0x68FB06AC },
            Op::Load { kind: Load::Byte16, file: "pse_27.12e", offset: 0x40000, size: 0x20000, crc: 0x3EB181C3 },
            Op::Load { kind: Load::Byte16, file: "pse_31.12f", offset: 0x40001, size: 0x20000, crc: 0x37108E7B },
            Op::Load { kind: Load::Byte16, file: "pse_24.9e", offset: 0x80000, size: 0x20000, crc: 0x0F434414 },
            Op::Load { kind: Load::Byte16, file: "pse_28.9f", offset: 0x80001, size: 0x20000, crc: 0xB732345D },
            Op::Load { kind: Load::Byte16, file: "pse_25.10e", offset: 0xc0000, size: 0x20000, crc: 0xB77102E2 },
            Op::Load { kind: Load::Byte16, file: "pse_29.10f", offset: 0xc0001, size: 0x20000, crc: 0xEC037BCE },
            Op::Load { kind: Load::Word16Swap, file: "ps_21.6f", offset: 0x100000, size: 0x80000, crc: 0x8AFFA5A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ps-1m.3a", offset: 0x0, size: 0x80000, crc: 0x77B7CCAB },
            Op::Load { kind: Load::Word64, file: "ps-3m.5a", offset: 0x2, size: 0x80000, crc: 0x0122720B },
            Op::Load { kind: Load::Word64, file: "ps-2m.4a", offset: 0x4, size: 0x80000, crc: 0x64FA58D4 },
            Op::Load { kind: Load::Word64, file: "ps-4m.6a", offset: 0x6, size: 0x80000, crc: 0x60DA42C8 },
            Op::Load { kind: Load::Word64, file: "ps-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xC54EA839 },
            Op::Load { kind: Load::Word64, file: "ps-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x04C5ACBD },
            Op::Load { kind: Load::Word64, file: "ps-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xA544F4CC },
            Op::Load { kind: Load::Word64, file: "ps-8m.10a", offset: 0x200006, size: 0x80000, crc: 0x8F02F436 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps_q.5k", offset: 0x0, size: 0x8000, crc: 0x49FF4446 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps-q1.1k", offset: 0x0, size: 0x80000, crc: 0x31FD8726 },
            Op::Load { kind: Load::Byte, file: "ps-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x980A9EEF },
            Op::Load { kind: Load::Byte, file: "ps-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x0DD44491 },
            Op::Load { kind: Load::Byte, file: "ps-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xBED42F03 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_PUNISHERU: RomSetDef = RomSetDef { name: "punisheru", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "psu_26.11e", offset: 0x0, size: 0x20000, crc: 0x9236D121 },
            Op::Load { kind: Load::Byte16, file: "psu_30.11f", offset: 0x1, size: 0x20000, crc: 0x8320E501 },
            Op::Load { kind: Load::Byte16, file: "psu_27.12e", offset: 0x40000, size: 0x20000, crc: 0x61C960A1 },
            Op::Load { kind: Load::Byte16, file: "psu_31.12f", offset: 0x40001, size: 0x20000, crc: 0x78D4C298 },
            Op::Load { kind: Load::Byte16, file: "psu_24.9e", offset: 0x80000, size: 0x20000, crc: 0x1CFECAD7 },
            Op::Load { kind: Load::Byte16, file: "psu_28.9f", offset: 0x80001, size: 0x20000, crc: 0xBDF921C1 },
            Op::Load { kind: Load::Byte16, file: "psu_25.10e", offset: 0xc0000, size: 0x20000, crc: 0xC51ACC94 },
            Op::Load { kind: Load::Byte16, file: "psu_29.10f", offset: 0xc0001, size: 0x20000, crc: 0x52DCE1CA },
            Op::Load { kind: Load::Word16Swap, file: "ps_21.6f", offset: 0x100000, size: 0x80000, crc: 0x8AFFA5A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ps-1m.3a", offset: 0x0, size: 0x80000, crc: 0x77B7CCAB },
            Op::Load { kind: Load::Word64, file: "ps-3m.5a", offset: 0x2, size: 0x80000, crc: 0x0122720B },
            Op::Load { kind: Load::Word64, file: "ps-2m.4a", offset: 0x4, size: 0x80000, crc: 0x64FA58D4 },
            Op::Load { kind: Load::Word64, file: "ps-4m.6a", offset: 0x6, size: 0x80000, crc: 0x60DA42C8 },
            Op::Load { kind: Load::Word64, file: "ps-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xC54EA839 },
            Op::Load { kind: Load::Word64, file: "ps-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x04C5ACBD },
            Op::Load { kind: Load::Word64, file: "ps-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xA544F4CC },
            Op::Load { kind: Load::Word64, file: "ps-8m.10a", offset: 0x200006, size: 0x80000, crc: 0x8F02F436 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps_q.5k", offset: 0x0, size: 0x8000, crc: 0x49FF4446 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps-q1.1k", offset: 0x0, size: 0x80000, crc: 0x31FD8726 },
            Op::Load { kind: Load::Byte, file: "ps-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x980A9EEF },
            Op::Load { kind: Load::Byte, file: "ps-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x0DD44491 },
            Op::Load { kind: Load::Byte, file: "ps-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xBED42F03 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_PUNISHERH: RomSetDef = RomSetDef { name: "punisherh", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "psh_26.11e", offset: 0x0, size: 0x20000, crc: 0x6AD2BB83 },
            Op::Load { kind: Load::Byte16, file: "psh_30.11f", offset: 0x1, size: 0x20000, crc: 0x058D3659 },
            Op::Load { kind: Load::Byte16, file: "psh_27.12e", offset: 0x40000, size: 0x20000, crc: 0x579F4FD3 },
            Op::Load { kind: Load::Byte16, file: "psh_31.12f", offset: 0x40001, size: 0x20000, crc: 0x2C9F70B5 },
            Op::Load { kind: Load::Byte16, file: "psh_24.9e", offset: 0x80000, size: 0x20000, crc: 0xFAA14841 },
            Op::Load { kind: Load::Byte16, file: "psh_28.9f", offset: 0x80001, size: 0x20000, crc: 0x5C5B1F20 },
            Op::Load { kind: Load::Byte16, file: "psh_25.10e", offset: 0xc0000, size: 0x20000, crc: 0x724FDFDA },
            Op::Load { kind: Load::Byte16, file: "psh_29.10f", offset: 0xc0001, size: 0x20000, crc: 0x779CF901 },
            Op::Load { kind: Load::Word16Swap, file: "ps_21.6f", offset: 0x100000, size: 0x80000, crc: 0x8AFFA5A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ps-1m.3a", offset: 0x0, size: 0x80000, crc: 0x77B7CCAB },
            Op::Load { kind: Load::Word64, file: "ps-3m.5a", offset: 0x2, size: 0x80000, crc: 0x0122720B },
            Op::Load { kind: Load::Word64, file: "ps-2m.4a", offset: 0x4, size: 0x80000, crc: 0x64FA58D4 },
            Op::Load { kind: Load::Word64, file: "ps-4m.6a", offset: 0x6, size: 0x80000, crc: 0x60DA42C8 },
            Op::Load { kind: Load::Word64, file: "ps-5m.7a", offset: 0x200000, size: 0x80000, crc: 0xC54EA839 },
            Op::Load { kind: Load::Word64, file: "ps-7m.9a", offset: 0x200002, size: 0x80000, crc: 0x04C5ACBD },
            Op::Load { kind: Load::Word64, file: "ps-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xA544F4CC },
            Op::Load { kind: Load::Word64, file: "ps-8m.10a", offset: 0x200006, size: 0x80000, crc: 0x8F02F436 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps_q.5k", offset: 0x0, size: 0x8000, crc: 0x49FF4446 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps-q1.1k", offset: 0x0, size: 0x80000, crc: 0x31FD8726 },
            Op::Load { kind: Load::Byte, file: "ps-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x980A9EEF },
            Op::Load { kind: Load::Byte, file: "ps-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x0DD44491 },
            Op::Load { kind: Load::Byte, file: "ps-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xBED42F03 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_PUNISHERJ: RomSetDef = RomSetDef { name: "punisherj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "psj_23.8f", offset: 0x0, size: 0x80000, crc: 0x6B2FDA52 },
            Op::Load { kind: Load::Word16Swap, file: "psj_22.7f", offset: 0x80000, size: 0x80000, crc: 0xE01036BC },
            Op::Load { kind: Load::Word16Swap, file: "psj_21.6f", offset: 0x100000, size: 0x80000, crc: 0x8AFFA5A9 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "ps_01.3a", offset: 0x0, size: 0x80000, crc: 0x77B7CCAB },
            Op::Load { kind: Load::Word64, file: "ps_02.4a", offset: 0x2, size: 0x80000, crc: 0x0122720B },
            Op::Load { kind: Load::Word64, file: "ps_03.5a", offset: 0x4, size: 0x80000, crc: 0x64FA58D4 },
            Op::Load { kind: Load::Word64, file: "ps_04.6a", offset: 0x6, size: 0x80000, crc: 0x60DA42C8 },
            Op::Load { kind: Load::Word64, file: "ps_05.7a", offset: 0x200000, size: 0x80000, crc: 0xC54EA839 },
            Op::Load { kind: Load::Word64, file: "ps_06.8a", offset: 0x200002, size: 0x80000, crc: 0x04C5ACBD },
            Op::Load { kind: Load::Word64, file: "ps_07.9a", offset: 0x200004, size: 0x80000, crc: 0xA544F4CC },
            Op::Load { kind: Load::Word64, file: "ps_08.10a", offset: 0x200006, size: 0x80000, crc: 0x8F02F436 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps_q.5k", offset: 0x0, size: 0x8000, crc: 0x49FF4446 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ps-q1.1k", offset: 0x0, size: 0x80000, crc: 0x31FD8726 },
            Op::Load { kind: Load::Byte, file: "ps-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x980A9EEF },
            Op::Load { kind: Load::Byte, file: "ps-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x0DD44491 },
            Op::Load { kind: Load::Byte, file: "ps-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xBED42F03 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_PUNISHERBZ: RomSetDef = RomSetDef { name: "punisherbz", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "23.096", offset: 0x0, size: 0x80000, crc: 0xBFA45D23 },
            Op::Load { kind: Load::Word16Swap, file: "22.096", offset: 0x80000, size: 0x80000, crc: 0x092578A4 },
            Op::Load { kind: Load::Word16Swap, file: "21.096", offset: 0x100000, size: 0x80000, crc: 0xD21CCDDB },
            Op::Load { kind: Load::Word16Swap, file: "20.096", offset: 0x180000, size: 0x80000, crc: 0xF9F334CE },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1.096", offset: 0x0, size: 0x80000, crc: 0xAD468E07 },
            Op::Load { kind: Load::Word64, file: "2.096", offset: 0x2, size: 0x80000, crc: 0xB9FDB6B5 },
            Op::Load { kind: Load::Word64, file: "3.096", offset: 0x4, size: 0x80000, crc: 0xBE0B1A78 },
            Op::Load { kind: Load::Word64, file: "4.096", offset: 0x6, size: 0x80000, crc: 0xBBA67A43 },
            Op::Load { kind: Load::Word64, file: "ps_gfx5.rom", offset: 0x200000, size: 0x80000, crc: 0xC54EA839 },
            Op::Load { kind: Load::Word64, file: "ps_gfx7.rom", offset: 0x200002, size: 0x80000, crc: 0x04C5ACBD },
            Op::Load { kind: Load::Word64, file: "ps_gfx6.rom", offset: 0x200004, size: 0x80000, crc: 0xA544F4CC },
            Op::Load { kind: Load::Word64, file: "ps_gfx8.rom", offset: 0x200006, size: 0x80000, crc: 0x8F02F436 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "9.512", offset: 0x0, size: 0x8000, crc: 0xB8367EB5 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "18.010", offset: 0x0, size: 0x20000, crc: 0x375C66E7 },
            Op::Load { kind: Load::Byte, file: "19.010", offset: 0x20000, size: 0x20000, crc: 0xEB5CA884 },
        ] },
] };
/// Régions ignorées : aboardplds, audiorom_raw, bboardplds, cboardplds, dboardplds
pub const RS_SLAMMAST: RomSetDef = RomSetDef { name: "slammast", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mbe_23e.8f", offset: 0x0, size: 0x80000, crc: 0x5394057A },
            Op::Load { kind: Load::Byte16, file: "mbe_24b.9e", offset: 0x80000, size: 0x20000, crc: 0x95D5E729 },
            Op::Load { kind: Load::Byte16, file: "mbe_28b.9f", offset: 0x80001, size: 0x20000, crc: 0xB1C7CBCB },
            Op::Load { kind: Load::Byte16, file: "mbe_25b.10e", offset: 0xc0000, size: 0x20000, crc: 0xA50D3FD4 },
            Op::Load { kind: Load::Byte16, file: "mbe_29b.10f", offset: 0xc0001, size: 0x20000, crc: 0x08E32E56 },
            Op::Load { kind: Load::Word16Swap, file: "mbe_21a.6f", offset: 0x100000, size: 0x80000, crc: 0xD5007B05 },
            Op::Load { kind: Load::Word16Swap, file: "mbe_20a.5f", offset: 0x180000, size: 0x80000, crc: 0xAEB557B0 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mb-1m.3a", offset: 0x0, size: 0x80000, crc: 0x41468E06 },
            Op::Load { kind: Load::Word64, file: "mb-3m.5a", offset: 0x2, size: 0x80000, crc: 0xF453AA9E },
            Op::Load { kind: Load::Word64, file: "mb-2m.4a", offset: 0x4, size: 0x80000, crc: 0x2FFBFEA8 },
            Op::Load { kind: Load::Word64, file: "mb-4m.6a", offset: 0x6, size: 0x80000, crc: 0x1EB9841D },
            Op::Load { kind: Load::Word64, file: "mb-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x506B9DC9 },
            Op::Load { kind: Load::Word64, file: "mb-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xAFF8C2FB },
            Op::Load { kind: Load::Word64, file: "mb-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xB76C70E9 },
            Op::Load { kind: Load::Word64, file: "mb-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xE60C9556 },
            Op::Load { kind: Load::Word64, file: "mb-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x97976FF5 },
            Op::Load { kind: Load::Word64, file: "mb-12m.5c", offset: 0x400002, size: 0x80000, crc: 0xB350A840 },
            Op::Load { kind: Load::Word64, file: "mb-11m.4c", offset: 0x400004, size: 0x80000, crc: 0x8FB94743 },
            Op::Load { kind: Load::Word64, file: "mb-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xDA810D5F },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb_qa.5k", offset: 0x0, size: 0x8000, crc: 0xE21A03C4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb-q1.1k", offset: 0x0, size: 0x80000, crc: 0x0630C3CE },
            Op::Load { kind: Load::Byte, file: "mb-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x354F9C21 },
            Op::Load { kind: Load::Byte, file: "mb-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x7838487C },
            Op::Load { kind: Load::Byte, file: "mb-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xAB66E087 },
            Op::Load { kind: Load::Byte, file: "mb-q5.1m", offset: 0x200000, size: 0x80000, crc: 0xC789FEF2 },
            Op::Load { kind: Load::Byte, file: "mb-q6.2m", offset: 0x280000, size: 0x80000, crc: 0xECB81B61 },
            Op::Load { kind: Load::Byte, file: "mb-q7.3m", offset: 0x300000, size: 0x80000, crc: 0x041E49BA },
            Op::Load { kind: Load::Byte, file: "mb-q8.4m", offset: 0x380000, size: 0x80000, crc: 0x59FE702A },
        ] },
] };
/// Régions ignorées : aboardplds, audiorom_raw, bboardplds, cboardplds, dboardplds
pub const RS_SLAMMASTU: RomSetDef = RomSetDef { name: "slammastu", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mbu_23e.8f", offset: 0x0, size: 0x80000, crc: 0x224F0062 },
            Op::Load { kind: Load::Byte16, file: "mbu_24b.9e", offset: 0x80000, size: 0x20000, crc: 0x95D5E729 },
            Op::Load { kind: Load::Byte16, file: "mbu_28b.9f", offset: 0x80001, size: 0x20000, crc: 0xB1C7CBCB },
            Op::Load { kind: Load::Byte16, file: "mbu_25b.10e", offset: 0xc0000, size: 0x20000, crc: 0xA50D3FD4 },
            Op::Load { kind: Load::Byte16, file: "mbu_29b.10f", offset: 0xc0001, size: 0x20000, crc: 0x08E32E56 },
            Op::Load { kind: Load::Word16Swap, file: "mbu_21a.6f", offset: 0x100000, size: 0x80000, crc: 0xD5007B05 },
            Op::Load { kind: Load::Word16Swap, file: "mbu_20a.5f", offset: 0x180000, size: 0x80000, crc: 0xFC848AF5 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mb-1m.3a", offset: 0x0, size: 0x80000, crc: 0x41468E06 },
            Op::Load { kind: Load::Word64, file: "mb-3m.5a", offset: 0x2, size: 0x80000, crc: 0xF453AA9E },
            Op::Load { kind: Load::Word64, file: "mb-2m.4a", offset: 0x4, size: 0x80000, crc: 0x2FFBFEA8 },
            Op::Load { kind: Load::Word64, file: "mb-4m.6a", offset: 0x6, size: 0x80000, crc: 0x1EB9841D },
            Op::Load { kind: Load::Word64, file: "mb-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x506B9DC9 },
            Op::Load { kind: Load::Word64, file: "mb-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xAFF8C2FB },
            Op::Load { kind: Load::Word64, file: "mb-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xB76C70E9 },
            Op::Load { kind: Load::Word64, file: "mb-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xE60C9556 },
            Op::Load { kind: Load::Word64, file: "mb-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x97976FF5 },
            Op::Load { kind: Load::Word64, file: "mb-12m.5c", offset: 0x400002, size: 0x80000, crc: 0xB350A840 },
            Op::Load { kind: Load::Word64, file: "mb-11m.4c", offset: 0x400004, size: 0x80000, crc: 0x8FB94743 },
            Op::Load { kind: Load::Word64, file: "mb-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xDA810D5F },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb_qa.5k", offset: 0x0, size: 0x8000, crc: 0xE21A03C4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb-q1.1k", offset: 0x0, size: 0x80000, crc: 0x0630C3CE },
            Op::Load { kind: Load::Byte, file: "mb-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x354F9C21 },
            Op::Load { kind: Load::Byte, file: "mb-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x7838487C },
            Op::Load { kind: Load::Byte, file: "mb-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xAB66E087 },
            Op::Load { kind: Load::Byte, file: "mb-q5.1m", offset: 0x200000, size: 0x80000, crc: 0xC789FEF2 },
            Op::Load { kind: Load::Byte, file: "mb-q6.2m", offset: 0x280000, size: 0x80000, crc: 0xECB81B61 },
            Op::Load { kind: Load::Byte, file: "mb-q7.3m", offset: 0x300000, size: 0x80000, crc: 0x041E49BA },
            Op::Load { kind: Load::Byte, file: "mb-q8.4m", offset: 0x380000, size: 0x80000, crc: 0x59FE702A },
        ] },
] };
/// Régions ignorées : aboardplds, audiorom_raw, bboardplds, cboardplds, dboardplds
pub const RS_MBOMBERJ: RomSetDef = RomSetDef { name: "mbomberj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mbj_23e.8f", offset: 0x0, size: 0x80000, crc: 0x0D06036A },
            Op::Load { kind: Load::Word16Swap, file: "mbj_22b.7f", offset: 0x80000, size: 0x80000, crc: 0xACD38478 },
            Op::Load { kind: Load::Word16Swap, file: "mbj_21a.6f", offset: 0x100000, size: 0x80000, crc: 0xD5007B05 },
            Op::Load { kind: Load::Word16Swap, file: "mbj_20a.5f", offset: 0x180000, size: 0x80000, crc: 0xAEB557B0 },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mb_01.3a", offset: 0x0, size: 0x80000, crc: 0xA53B1C81 },
            Op::Load { kind: Load::Word64, file: "mb_02.4a", offset: 0x2, size: 0x80000, crc: 0x23FE10F6 },
            Op::Load { kind: Load::Word64, file: "mb_03.5a", offset: 0x4, size: 0x80000, crc: 0xCB866C2F },
            Op::Load { kind: Load::Word64, file: "mb_04.6a", offset: 0x6, size: 0x80000, crc: 0xC9143E75 },
            Op::Load { kind: Load::Word64, file: "mb_05.7a", offset: 0x200000, size: 0x80000, crc: 0x506B9DC9 },
            Op::Load { kind: Load::Word64, file: "mb_06.8a", offset: 0x200002, size: 0x80000, crc: 0xAFF8C2FB },
            Op::Load { kind: Load::Word64, file: "mb_07.9a", offset: 0x200004, size: 0x80000, crc: 0xB76C70E9 },
            Op::Load { kind: Load::Word64, file: "mb_08.10a", offset: 0x200006, size: 0x80000, crc: 0xE60C9556 },
            Op::Load { kind: Load::Word64, file: "mb_10.3c", offset: 0x400000, size: 0x80000, crc: 0x97976FF5 },
            Op::Load { kind: Load::Word64, file: "mb_11.4c", offset: 0x400002, size: 0x80000, crc: 0xB350A840 },
            Op::Load { kind: Load::Word64, file: "mb_12.5c", offset: 0x400004, size: 0x80000, crc: 0x8FB94743 },
            Op::Load { kind: Load::Word64, file: "mb_13.6c", offset: 0x400006, size: 0x80000, crc: 0xDA810D5F },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb_qa.5k", offset: 0x0, size: 0x8000, crc: 0xE21A03C4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb-q1.1k", offset: 0x0, size: 0x80000, crc: 0x0630C3CE },
            Op::Load { kind: Load::Byte, file: "mb-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x354F9C21 },
            Op::Load { kind: Load::Byte, file: "mb-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x7838487C },
            Op::Load { kind: Load::Byte, file: "mb-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xAB66E087 },
            Op::Load { kind: Load::Byte, file: "mb-q5.1m", offset: 0x200000, size: 0x80000, crc: 0xC789FEF2 },
            Op::Load { kind: Load::Byte, file: "mb-q6.2m", offset: 0x280000, size: 0x80000, crc: 0xECB81B61 },
            Op::Load { kind: Load::Byte, file: "mb-q7.3m", offset: 0x300000, size: 0x80000, crc: 0x041E49BA },
            Op::Load { kind: Load::Byte, file: "mb-q8.4m", offset: 0x380000, size: 0x80000, crc: 0x59FE702A },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_MBOMBRD: RomSetDef = RomSetDef { name: "mbombrd", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "mbde_26.11e", offset: 0x0, size: 0x20000, crc: 0x72B7451C },
            Op::Load { kind: Load::Byte16, file: "mbde_30.11f", offset: 0x1, size: 0x20000, crc: 0xA036DC16 },
            Op::Load { kind: Load::Byte16, file: "mbde_27.12e", offset: 0x40000, size: 0x20000, crc: 0x4086F534 },
            Op::Load { kind: Load::Byte16, file: "mbde_31.12f", offset: 0x40001, size: 0x20000, crc: 0x085F47F0 },
            Op::Load { kind: Load::Byte16, file: "mbde_24.9e", offset: 0x80000, size: 0x20000, crc: 0xC20895A5 },
            Op::Load { kind: Load::Byte16, file: "mbde_28.9f", offset: 0x80001, size: 0x20000, crc: 0x2618D5E1 },
            Op::Load { kind: Load::Byte16, file: "mbde_25.10e", offset: 0xc0000, size: 0x20000, crc: 0x9BDB6B11 },
            Op::Load { kind: Load::Byte16, file: "mbde_29.10f", offset: 0xc0001, size: 0x20000, crc: 0x3F52D5E5 },
            Op::Load { kind: Load::Word16Swap, file: "mbde_21.6f", offset: 0x100000, size: 0x80000, crc: 0x690C026A },
            Op::Load { kind: Load::Word16Swap, file: "mbde_20.5f", offset: 0x180000, size: 0x80000, crc: 0xB8B2139B },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mb-1m.3a", offset: 0x0, size: 0x80000, crc: 0x41468E06 },
            Op::Load { kind: Load::Word64, file: "mb-3m.5a", offset: 0x2, size: 0x80000, crc: 0xF453AA9E },
            Op::Load { kind: Load::Word64, file: "mb-2m.4a", offset: 0x4, size: 0x80000, crc: 0x2FFBFEA8 },
            Op::Load { kind: Load::Word64, file: "mb-4m.6a", offset: 0x6, size: 0x80000, crc: 0x1EB9841D },
            Op::Load { kind: Load::Word64, file: "mb-5m.7a", offset: 0x200000, size: 0x80000, crc: 0x506B9DC9 },
            Op::Load { kind: Load::Word64, file: "mb-7m.9a", offset: 0x200002, size: 0x80000, crc: 0xAFF8C2FB },
            Op::Load { kind: Load::Word64, file: "mb-6m.8a", offset: 0x200004, size: 0x80000, crc: 0xB76C70E9 },
            Op::Load { kind: Load::Word64, file: "mb-8m.10a", offset: 0x200006, size: 0x80000, crc: 0xE60C9556 },
            Op::Load { kind: Load::Word64, file: "mb-10m.3c", offset: 0x400000, size: 0x80000, crc: 0x97976FF5 },
            Op::Load { kind: Load::Word64, file: "mb-12m.5c", offset: 0x400002, size: 0x80000, crc: 0xB350A840 },
            Op::Load { kind: Load::Word64, file: "mb-11m.4c", offset: 0x400004, size: 0x80000, crc: 0x8FB94743 },
            Op::Load { kind: Load::Word64, file: "mb-13m.6c", offset: 0x400006, size: 0x80000, crc: 0xDA810D5F },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb_q.5k", offset: 0x0, size: 0x8000, crc: 0xD6FA76D1 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb-q1.1k", offset: 0x0, size: 0x80000, crc: 0x0630C3CE },
            Op::Load { kind: Load::Byte, file: "mb-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x354F9C21 },
            Op::Load { kind: Load::Byte, file: "mb-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x7838487C },
            Op::Load { kind: Load::Byte, file: "mb-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xAB66E087 },
            Op::Load { kind: Load::Byte, file: "mb-q5.1m", offset: 0x200000, size: 0x80000, crc: 0xC789FEF2 },
            Op::Load { kind: Load::Byte, file: "mb-q6.2m", offset: 0x280000, size: 0x80000, crc: 0xECB81B61 },
            Op::Load { kind: Load::Byte, file: "mb-q7.3m", offset: 0x300000, size: 0x80000, crc: 0x041E49BA },
            Op::Load { kind: Load::Byte, file: "mb-q8.4m", offset: 0x380000, size: 0x80000, crc: 0x59FE702A },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_MBOMBRDJ: RomSetDef = RomSetDef { name: "mbombrdj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "mbdj_26.11e", offset: 0x0, size: 0x20000, crc: 0x72B7451C },
            Op::Load { kind: Load::Byte16, file: "mbdj_30.11f", offset: 0x1, size: 0x20000, crc: 0xBEFF31CF },
            Op::Load { kind: Load::Byte16, file: "mbdj_27.12e", offset: 0x40000, size: 0x20000, crc: 0x4086F534 },
            Op::Load { kind: Load::Byte16, file: "mbdj_31.12f", offset: 0x40001, size: 0x20000, crc: 0x085F47F0 },
            Op::Load { kind: Load::Byte16, file: "mbdj_24.9e", offset: 0x80000, size: 0x20000, crc: 0xC20895A5 },
            Op::Load { kind: Load::Byte16, file: "mbdj_28.9f", offset: 0x80001, size: 0x20000, crc: 0x2618D5E1 },
            Op::Load { kind: Load::Byte16, file: "mbdj_25.10e", offset: 0xc0000, size: 0x20000, crc: 0x9BDB6B11 },
            Op::Load { kind: Load::Byte16, file: "mbdj_29.10f", offset: 0xc0001, size: 0x20000, crc: 0x3F52D5E5 },
            Op::Load { kind: Load::Word16Swap, file: "mbdj_21.6f", offset: 0x100000, size: 0x80000, crc: 0x690C026A },
            Op::Load { kind: Load::Word16Swap, file: "mbdj_20.5f", offset: 0x180000, size: 0x80000, crc: 0xB8B2139B },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mb_01.3a", offset: 0x0, size: 0x80000, crc: 0xA53B1C81 },
            Op::Load { kind: Load::Word64, file: "mb_02.4a", offset: 0x2, size: 0x80000, crc: 0x23FE10F6 },
            Op::Load { kind: Load::Word64, file: "mb_03.5a", offset: 0x4, size: 0x80000, crc: 0xCB866C2F },
            Op::Load { kind: Load::Word64, file: "mb_04.6a", offset: 0x6, size: 0x80000, crc: 0xC9143E75 },
            Op::Load { kind: Load::Word64, file: "mb_05.7a", offset: 0x200000, size: 0x80000, crc: 0x506B9DC9 },
            Op::Load { kind: Load::Word64, file: "mb_06.8a", offset: 0x200002, size: 0x80000, crc: 0xAFF8C2FB },
            Op::Load { kind: Load::Word64, file: "mb_07.9a", offset: 0x200004, size: 0x80000, crc: 0xB76C70E9 },
            Op::Load { kind: Load::Word64, file: "mb_08.10a", offset: 0x200006, size: 0x80000, crc: 0xE60C9556 },
            Op::Load { kind: Load::Word64, file: "mb_10.3c", offset: 0x400000, size: 0x80000, crc: 0x97976FF5 },
            Op::Load { kind: Load::Word64, file: "mb_11.4c", offset: 0x400002, size: 0x80000, crc: 0xB350A840 },
            Op::Load { kind: Load::Word64, file: "mb_12.5c", offset: 0x400004, size: 0x80000, crc: 0x8FB94743 },
            Op::Load { kind: Load::Word64, file: "mb_13.6c", offset: 0x400006, size: 0x80000, crc: 0xDA810D5F },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb_qa.5k", offset: 0x0, size: 0x8000, crc: 0xE21A03C4 },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mb-q1.1k", offset: 0x0, size: 0x80000, crc: 0x0630C3CE },
            Op::Load { kind: Load::Byte, file: "mb-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x354F9C21 },
            Op::Load { kind: Load::Byte, file: "mb-q3.3k", offset: 0x100000, size: 0x80000, crc: 0x7838487C },
            Op::Load { kind: Load::Byte, file: "mb-q4.4k", offset: 0x180000, size: 0x80000, crc: 0xAB66E087 },
            Op::Load { kind: Load::Byte, file: "mb-q5.1m", offset: 0x200000, size: 0x80000, crc: 0xC789FEF2 },
            Op::Load { kind: Load::Byte, file: "mb-q6.2m", offset: 0x280000, size: 0x80000, crc: 0xECB81B61 },
            Op::Load { kind: Load::Byte, file: "mb-q7.3m", offset: 0x300000, size: 0x80000, crc: 0x041E49BA },
            Op::Load { kind: Load::Byte, file: "mb-q8.4m", offset: 0x380000, size: 0x80000, crc: 0x59FE702A },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_PNICKJ: RomSetDef = RomSetDef { name: "pnickj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "pnij_36.12f", offset: 0x0, size: 0x20000, crc: 0x2D4FFB2B },
            Op::Load { kind: Load::Byte16, file: "pnij_42.12h", offset: 0x1, size: 0x20000, crc: 0xC085DFAF },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "pnij_09.4b", offset: 0x0, size: 0x20000, crc: 0x48177B0A },
            Op::Load { kind: Load::Byte64, file: "pnij_01.4a", offset: 0x1, size: 0x20000, crc: 0x01A0F311 },
            Op::Load { kind: Load::Byte64, file: "pnij_13.9b", offset: 0x2, size: 0x20000, crc: 0x406451B0 },
            Op::Load { kind: Load::Byte64, file: "pnij_05.9a", offset: 0x3, size: 0x20000, crc: 0x8C515DC0 },
            Op::Load { kind: Load::Byte64, file: "pnij_26.5e", offset: 0x4, size: 0x20000, crc: 0xE2AF981E },
            Op::Load { kind: Load::Byte64, file: "pnij_18.5c", offset: 0x5, size: 0x20000, crc: 0xF17A0E56 },
            Op::Load { kind: Load::Byte64, file: "pnij_38.8h", offset: 0x6, size: 0x20000, crc: 0xEB75BD8C },
            Op::Load { kind: Load::Byte64, file: "pnij_32.8f", offset: 0x7, size: 0x20000, crc: 0x84560BEF },
            Op::Load { kind: Load::Byte64, file: "pnij_10.5b", offset: 0x100000, size: 0x20000, crc: 0xC2ACC171 },
            Op::Load { kind: Load::Byte64, file: "pnij_02.5a", offset: 0x100001, size: 0x20000, crc: 0x0E21FC33 },
            Op::Load { kind: Load::Byte64, file: "pnij_14.10b", offset: 0x100002, size: 0x20000, crc: 0x7FE59B19 },
            Op::Load { kind: Load::Byte64, file: "pnij_06.10a", offset: 0x100003, size: 0x20000, crc: 0x79F4BFE3 },
            Op::Load { kind: Load::Byte64, file: "pnij_27.7e", offset: 0x100004, size: 0x20000, crc: 0x83D5CB0E },
            Op::Load { kind: Load::Byte64, file: "pnij_19.7c", offset: 0x100005, size: 0x20000, crc: 0xAF08B230 },
            Op::Load { kind: Load::Byte64, file: "pnij_39.9h", offset: 0x100006, size: 0x20000, crc: 0x70FBE579 },
            Op::Load { kind: Load::Byte64, file: "pnij_33.9f", offset: 0x100007, size: 0x20000, crc: 0x3ED2C680 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pnij_17.13b", offset: 0x0, size: 0x8000, crc: 0xE86F787A },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pnij_24.12c", offset: 0x0, size: 0x20000, crc: 0x5092257D },
            Op::Load { kind: Load::Byte, file: "pnij_25.13c", offset: 0x20000, size: 0x20000, crc: 0x22109AAA },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_QTONO2J: RomSetDef = RomSetDef { name: "qtono2j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "tn2j_30.11e", offset: 0x0, size: 0x20000, crc: 0x9226EB5E },
            Op::Load { kind: Load::Byte16, file: "tn2j_37.11f", offset: 0x1, size: 0x20000, crc: 0xD1D30DA1 },
            Op::Load { kind: Load::Byte16, file: "tn2j_31.12e", offset: 0x40000, size: 0x20000, crc: 0x015E6A8A },
            Op::Load { kind: Load::Byte16, file: "tn2j_38.12f", offset: 0x40001, size: 0x20000, crc: 0x1F139BCC },
            Op::Load { kind: Load::Byte16, file: "tn2j_28.9e", offset: 0x80000, size: 0x20000, crc: 0x86D27F71 },
            Op::Load { kind: Load::Byte16, file: "tn2j_35.9f", offset: 0x80001, size: 0x20000, crc: 0x7A1AB87D },
            Op::Load { kind: Load::Byte16, file: "tn2j_29.10e", offset: 0xc0000, size: 0x20000, crc: 0x9C384E99 },
            Op::Load { kind: Load::Byte16, file: "tn2j_36.10f", offset: 0xc0001, size: 0x20000, crc: 0x4C4B2A0A },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tn2-02m.4a", offset: 0x0, size: 0x80000, crc: 0xF2016A34 },
            Op::Load { kind: Load::Word64, file: "tn2-04m.6a", offset: 0x2, size: 0x80000, crc: 0x094E0FB1 },
            Op::Load { kind: Load::Word64, file: "tn2-01m.3a", offset: 0x4, size: 0x80000, crc: 0xCB950CF9 },
            Op::Load { kind: Load::Word64, file: "tn2-03m.5a", offset: 0x6, size: 0x80000, crc: 0x18A5BF59 },
            Op::Load { kind: Load::Word64, file: "tn2-11m.4c", offset: 0x200000, size: 0x80000, crc: 0xD0EDD30B },
            Op::Load { kind: Load::Word64, file: "tn2-13m.6c", offset: 0x200002, size: 0x80000, crc: 0x426621C3 },
            Op::Load { kind: Load::Word64, file: "tn2-10m.3c", offset: 0x200004, size: 0x80000, crc: 0xA34ECE70 },
            Op::Load { kind: Load::Word64, file: "tn2-12m.5c", offset: 0x200006, size: 0x80000, crc: 0xE04FF2F4 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tn2j_09.12a", offset: 0x0, size: 0x8000, crc: 0xE464B969 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tn2j_18.11c", offset: 0x0, size: 0x20000, crc: 0xA40BF9A7 },
            Op::Load { kind: Load::Byte, file: "tn2j_19.12c", offset: 0x20000, size: 0x20000, crc: 0x5B3B931E },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_PANG3: RomSetDef = RomSetDef { name: "pang3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pa3e_17a.11l", offset: 0x0, size: 0x80000, crc: 0xA213FA80 },
            Op::Load { kind: Load::Word16Swap, file: "pa3e_16a.10l", offset: 0x80000, size: 0x80000, crc: 0x7169EA67 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pa3-01m.2c", offset: 0x0, size: 0x100000, crc: 0x068A152C },
            Op::Continue { offset: 0x4, size: 0x100000 },
            Op::Load { kind: Load::Word64, file: "pa3-07m.2f", offset: 0x2, size: 0x100000, crc: 0x3A4A619D },
            Op::Continue { offset: 0x6, size: 0x100000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_11.11f", offset: 0x0, size: 0x8000, crc: 0xCB1423A2 },
            Op::Ignore { size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_PANG3R1: RomSetDef = RomSetDef { name: "pang3r1", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pa3e_17.11l", offset: 0x0, size: 0x80000, crc: 0xD7041D32 },
            Op::Load { kind: Load::Word16Swap, file: "pa3e_16.10l", offset: 0x80000, size: 0x80000, crc: 0x1BE9A483 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pa3-01m.2c", offset: 0x0, size: 0x100000, crc: 0x068A152C },
            Op::Continue { offset: 0x4, size: 0x100000 },
            Op::Load { kind: Load::Word64, file: "pa3-07m.2f", offset: 0x2, size: 0x100000, crc: 0x3A4A619D },
            Op::Continue { offset: 0x6, size: 0x100000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_11.11f", offset: 0x0, size: 0x8000, crc: 0xCB1423A2 },
            Op::Ignore { size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_PANG3J: RomSetDef = RomSetDef { name: "pang3j", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pa3j_17.11l", offset: 0x0, size: 0x80000, crc: 0x21F6E51F },
            Op::Load { kind: Load::Word16Swap, file: "pa3j_16.10l", offset: 0x80000, size: 0x80000, crc: 0xCA1D7897 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pa3-01m.2c", offset: 0x0, size: 0x100000, crc: 0x068A152C },
            Op::Continue { offset: 0x4, size: 0x100000 },
            Op::Load { kind: Load::Word64, file: "pa3-07m.2f", offset: 0x2, size: 0x100000, crc: 0x3A4A619D },
            Op::Continue { offset: 0x6, size: 0x100000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_11.11f", offset: 0x0, size: 0x8000, crc: 0xCB1423A2 },
            Op::Ignore { size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_PANG3B: RomSetDef = RomSetDef { name: "pang3b", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pa3w_17.11l", offset: 0x0, size: 0x80000, crc: 0x12138234 },
            Op::Load { kind: Load::Word16Swap, file: "pa3w_16.10l", offset: 0x80000, size: 0x80000, crc: 0xD1BA585C },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pa3-01m.2c", offset: 0x0, size: 0x100000, crc: 0x068A152C },
            Op::Continue { offset: 0x4, size: 0x100000 },
            Op::Load { kind: Load::Word64, file: "pa3-07m.2f", offset: 0x2, size: 0x100000, crc: 0x3A4A619D },
            Op::Continue { offset: 0x6, size: 0x100000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_11.11f", offset: 0x0, size: 0x8000, crc: 0x90A08C46 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds
pub const RS_PANG3B2: RomSetDef = RomSetDef { name: "pang3b2", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "16.u70", offset: 0x0, size: 0x40000, crc: 0x8D3FD82C },
            Op::Load { kind: Load::Byte16, file: "14.u72", offset: 0x1, size: 0x40000, crc: 0x654904C8 },
            Op::Load { kind: Load::Byte16, file: "15.u71", offset: 0x80000, size: 0x40000, crc: 0x42774E37 },
            Op::Load { kind: Load::Byte16, file: "13.u73", offset: 0x80001, size: 0x40000, crc: 0x531EA745 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "4.u68", offset: 0x0, size: 0x80000, crc: 0x45FC0A81 },
            Op::Load { kind: Load::Byte64, file: "2.u66", offset: 0x1, size: 0x80000, crc: 0x27668828 },
            Op::Load { kind: Load::Byte64, file: "10.u69", offset: 0x2, size: 0x80000, crc: 0xE051B2E9 },
            Op::Load { kind: Load::Byte64, file: "8.u63", offset: 0x3, size: 0x80000, crc: 0x4A68B194 },
            Op::Load { kind: Load::Byte64, file: "3.u67", offset: 0x4, size: 0x80000, crc: 0x91C8D782 },
            Op::Load { kind: Load::Byte64, file: "1.u65", offset: 0x5, size: 0x80000, crc: 0xFE32AF5D },
            Op::Load { kind: Load::Byte64, file: "9.u62", offset: 0x6, size: 0x80000, crc: 0xF0BBA5C7 },
            Op::Load { kind: Load::Byte64, file: "7.u64", offset: 0x7, size: 0x80000, crc: 0xC60C5E75 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "11.u32", offset: 0x0, size: 0x8000, crc: 0xCB1423A2 },
            Op::Ignore { size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "5.u34", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "6.u33", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds
pub const RS_PANG3B3: RomSetDef = RomSetDef { name: "pang3b3", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "u11l1_17.bin", offset: 0x0, size: 0x80000, crc: 0xDD3B95C0 },
            Op::Load { kind: Load::Word16Swap, file: "u10l1_16.bin", offset: 0x80000, size: 0x80000, crc: 0x1BE9A483 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "pa3-01m.2c", offset: 0x0, size: 0x100000, crc: 0x068A152C },
            Op::Continue { offset: 0x4, size: 0x100000 },
            Op::Load { kind: Load::Word64, file: "pa3-07m.2f", offset: 0x2, size: 0x100000, crc: 0x3A4A619D },
            Op::Continue { offset: 0x6, size: 0x100000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "u11f1.bin", offset: 0x0, size: 0x8000, crc: 0x278D786C },
            Op::Ignore { size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : bboardplds
pub const RS_PANG3B4: RomSetDef = RomSetDef { name: "pang3b4", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "22.u30", offset: 0x0, size: 0x20000, crc: 0xBF3EBE68 },
            Op::Load { kind: Load::Byte16, file: "26.u35", offset: 0x1, size: 0x20000, crc: 0xD20DB83C },
            Op::Load { kind: Load::Byte16, file: "23.u31", offset: 0x40000, size: 0x20000, crc: 0x94D494C2 },
            Op::Load { kind: Load::Byte16, file: "27.u36", offset: 0x40001, size: 0x20000, crc: 0x38E43390 },
            Op::Load { kind: Load::Byte16, file: "20.u28", offset: 0x80000, size: 0x20000, crc: 0x8DAF3814 },
            Op::Load { kind: Load::Byte16, file: "24.u33", offset: 0x80001, size: 0x20000, crc: 0xBB34E444 },
            Op::Load { kind: Load::Byte16, file: "21.u29", offset: 0xc0000, size: 0x20000, crc: 0x54D0B680 },
            Op::Load { kind: Load::Byte16, file: "25.u34", offset: 0xc0001, size: 0x20000, crc: 0xD666EC70 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "14.u25", offset: 0x0, size: 0x40000, crc: 0xD2B764A9 },
            Op::Load { kind: Load::Byte64, file: "06.u15", offset: 0x1, size: 0x40000, crc: 0xFB68BE4E },
            Op::Load { kind: Load::Byte64, file: "13.u24", offset: 0x200000, size: 0x40000, crc: 0x0C73A1C7 },
            Op::Load { kind: Load::Byte64, file: "05.u14", offset: 0x200001, size: 0x40000, crc: 0x475A5EF1 },
            Op::Load { kind: Load::Byte64, file: "10.u21", offset: 0x4, size: 0x40000, crc: 0x79673708 },
            Op::Load { kind: Load::Byte64, file: "02.u11", offset: 0x5, size: 0x40000, crc: 0xF706B466 },
            Op::Load { kind: Load::Byte64, file: "09.u20", offset: 0x200004, size: 0x40000, crc: 0x6E36E963 },
            Op::Load { kind: Load::Byte64, file: "01.u10", offset: 0x200005, size: 0x40000, crc: 0x7D15B9D7 },
            Op::Load { kind: Load::Byte64, file: "16.u27", offset: 0x2, size: 0x40000, crc: 0x3E293482 },
            Op::Load { kind: Load::Byte64, file: "08.u17", offset: 0x3, size: 0x40000, crc: 0x7E0CA927 },
            Op::Load { kind: Load::Byte64, file: "15.u26", offset: 0x200002, size: 0x40000, crc: 0xA933434F },
            Op::Load { kind: Load::Byte64, file: "07.u16", offset: 0x200003, size: 0x40000, crc: 0x83B3FA5E },
            Op::Load { kind: Load::Byte64, file: "12.u23", offset: 0x6, size: 0x40000, crc: 0x3C4DFB4F },
            Op::Load { kind: Load::Byte64, file: "04.u13", offset: 0x7, size: 0x40000, crc: 0x44CD5E95 },
            Op::Load { kind: Load::Byte64, file: "11.u22", offset: 0x200006, size: 0x40000, crc: 0x29194B90 },
            Op::Load { kind: Load::Byte64, file: "03.u12", offset: 0x200007, size: 0x40000, crc: 0x7E28974E },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_11.11f", offset: 0x0, size: 0x8000, crc: 0xCB1423A2 },
            Op::Ignore { size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_PANG3B5: RomSetDef = RomSetDef { name: "pang3b5", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "pa3e_17.11l", offset: 0x0, size: 0x80000, crc: 0xD7041D32 },
            Op::Load { kind: Load::Word16Swap, file: "pa3e_16.10l", offset: 0x80000, size: 0x80000, crc: 0x1BE9A483 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1.2c", offset: 0x0, size: 0x80000, crc: 0x22C934C4 },
            Op::Load { kind: Load::Word64, file: "7.2f", offset: 0x2, size: 0x80000, crc: 0x51031180 },
            Op::Load { kind: Load::Word64, file: "3.4c", offset: 0x4, size: 0x80000, crc: 0xAC119A46 },
            Op::Load { kind: Load::Word64, file: "9.4f", offset: 0x6, size: 0x80000, crc: 0x2E1D35F2 },
            Op::Load { kind: Load::Word64, file: "2.3c", offset: 0x200000, size: 0x80000, crc: 0x07C85E9B },
            Op::Load { kind: Load::Word64, file: "8.3f", offset: 0x200002, size: 0x80000, crc: 0x325CC0B7 },
            Op::Load { kind: Load::Word64, file: "4.5c", offset: 0x200004, size: 0x80000, crc: 0x4AD13297 },
            Op::Load { kind: Load::Word64, file: "10.5f", offset: 0x200006, size: 0x80000, crc: 0x026D0CD2 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_11.11f", offset: 0x0, size: 0x8000, crc: 0xCB1423A2 },
            Op::Ignore { size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "pa3_05.10d", offset: 0x0, size: 0x20000, crc: 0x73A10D5D },
            Op::Load { kind: Load::Byte, file: "pa3_06.11d", offset: 0x20000, size: 0x20000, crc: 0xAFFA4F82 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MEGAMAN: RomSetDef = RomSetDef { name: "megaman", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcmu_23b.8f", offset: 0x0, size: 0x80000, crc: 0x1CD33C7A },
            Op::Load { kind: Load::Word16Swap, file: "rcmu_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x708268C4 },
            Op::Load { kind: Load::Word16Swap, file: "rcmu_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x4376EA95 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "rcm_01.3a", offset: 0x0, size: 0x80000, crc: 0x6ECDF13F },
            Op::Load { kind: Load::Word64, file: "rcm_02.4a", offset: 0x2, size: 0x80000, crc: 0x944D4F0F },
            Op::Load { kind: Load::Word64, file: "rcm_03.5a", offset: 0x4, size: 0x80000, crc: 0x36F3073C },
            Op::Load { kind: Load::Word64, file: "rcm_04.6a", offset: 0x6, size: 0x80000, crc: 0x54E622FF },
            Op::Load { kind: Load::Word64, file: "rcm_05.7a", offset: 0x200000, size: 0x80000, crc: 0x5DD131FD },
            Op::Load { kind: Load::Word64, file: "rcm_06.8a", offset: 0x200002, size: 0x80000, crc: 0xF0FAF813 },
            Op::Load { kind: Load::Word64, file: "rcm_07.9a", offset: 0x200004, size: 0x80000, crc: 0x826DE013 },
            Op::Load { kind: Load::Word64, file: "rcm_08.10a", offset: 0x200006, size: 0x80000, crc: 0xFBFF64CF },
            Op::Load { kind: Load::Word64, file: "rcm_10.3c", offset: 0x400000, size: 0x80000, crc: 0x4DC8ADA9 },
            Op::Load { kind: Load::Word64, file: "rcm_11.4c", offset: 0x400002, size: 0x80000, crc: 0xF2B9EE06 },
            Op::Load { kind: Load::Word64, file: "rcm_12.5c", offset: 0x400004, size: 0x80000, crc: 0xFED5F203 },
            Op::Load { kind: Load::Word64, file: "rcm_13.6c", offset: 0x400006, size: 0x80000, crc: 0x5069D4A9 },
            Op::Load { kind: Load::Word64, file: "rcm_14.7c", offset: 0x600000, size: 0x80000, crc: 0x303BE3BD },
            Op::Load { kind: Load::Word64, file: "rcm_15.8c", offset: 0x600002, size: 0x80000, crc: 0x4F2D372F },
            Op::Load { kind: Load::Word64, file: "rcm_16.9c", offset: 0x600004, size: 0x80000, crc: 0x93D97FDE },
            Op::Load { kind: Load::Word64, file: "rcm_17.10c", offset: 0x600006, size: 0x80000, crc: 0x92371042 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm_09.11a", offset: 0x0, size: 0x8000, crc: 0x22AC8F5F },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm_18.11c", offset: 0x0, size: 0x20000, crc: 0x80F1F8AA },
            Op::Load { kind: Load::Byte, file: "rcm_19.12c", offset: 0x20000, size: 0x20000, crc: 0xF257DBE1 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MEGAMANA: RomSetDef = RomSetDef { name: "megamana", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcma_23b.8f", offset: 0x0, size: 0x80000, crc: 0x61E4A397 },
            Op::Load { kind: Load::Word16Swap, file: "rcma_22b.7f", offset: 0x80000, size: 0x80000, crc: 0x708268C4 },
            Op::Load { kind: Load::Word16Swap, file: "rcma_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x4376EA95 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "rcm_01.3a", offset: 0x0, size: 0x80000, crc: 0x6ECDF13F },
            Op::Load { kind: Load::Word64, file: "rcm_02.4a", offset: 0x2, size: 0x80000, crc: 0x944D4F0F },
            Op::Load { kind: Load::Word64, file: "rcm_03.5a", offset: 0x4, size: 0x80000, crc: 0x36F3073C },
            Op::Load { kind: Load::Word64, file: "rcm_04.6a", offset: 0x6, size: 0x80000, crc: 0x54E622FF },
            Op::Load { kind: Load::Word64, file: "rcm_05.7a", offset: 0x200000, size: 0x80000, crc: 0x5DD131FD },
            Op::Load { kind: Load::Word64, file: "rcm_06.8a", offset: 0x200002, size: 0x80000, crc: 0xF0FAF813 },
            Op::Load { kind: Load::Word64, file: "rcm_07.9a", offset: 0x200004, size: 0x80000, crc: 0x826DE013 },
            Op::Load { kind: Load::Word64, file: "rcm_08.10a", offset: 0x200006, size: 0x80000, crc: 0xFBFF64CF },
            Op::Load { kind: Load::Word64, file: "rcm_10.3c", offset: 0x400000, size: 0x80000, crc: 0x4DC8ADA9 },
            Op::Load { kind: Load::Word64, file: "rcm_11.4c", offset: 0x400002, size: 0x80000, crc: 0xF2B9EE06 },
            Op::Load { kind: Load::Word64, file: "rcm_12.5c", offset: 0x400004, size: 0x80000, crc: 0xFED5F203 },
            Op::Load { kind: Load::Word64, file: "rcm_13.6c", offset: 0x400006, size: 0x80000, crc: 0x5069D4A9 },
            Op::Load { kind: Load::Word64, file: "rcm_14.7c", offset: 0x600000, size: 0x80000, crc: 0x303BE3BD },
            Op::Load { kind: Load::Word64, file: "rcm_15.8c", offset: 0x600002, size: 0x80000, crc: 0x4F2D372F },
            Op::Load { kind: Load::Word64, file: "rcm_16.9c", offset: 0x600004, size: 0x80000, crc: 0x93D97FDE },
            Op::Load { kind: Load::Word64, file: "rcm_17.10c", offset: 0x600006, size: 0x80000, crc: 0x92371042 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm_09.12a", offset: 0x0, size: 0x8000, crc: 0x9632D6EF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm_18.11c", offset: 0x0, size: 0x20000, crc: 0x80F1F8AA },
            Op::Load { kind: Load::Byte, file: "rcm_19.12c", offset: 0x20000, size: 0x20000, crc: 0xF257DBE1 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_ROCKMANJ: RomSetDef = RomSetDef { name: "rockmanj", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "rcm_23a.8f", offset: 0x0, size: 0x80000, crc: 0xEFD96CB2 },
            Op::Load { kind: Load::Word16Swap, file: "rcm_22a.7f", offset: 0x80000, size: 0x80000, crc: 0x8729A689 },
            Op::Load { kind: Load::Word16Swap, file: "rcm_21a.6f", offset: 0x100000, size: 0x80000, crc: 0x517CCDE2 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "rcm_01.3a", offset: 0x0, size: 0x80000, crc: 0x6ECDF13F },
            Op::Load { kind: Load::Word64, file: "rcm_02.4a", offset: 0x2, size: 0x80000, crc: 0x944D4F0F },
            Op::Load { kind: Load::Word64, file: "rcm_03.5a", offset: 0x4, size: 0x80000, crc: 0x36F3073C },
            Op::Load { kind: Load::Word64, file: "rcm_04.6a", offset: 0x6, size: 0x80000, crc: 0x54E622FF },
            Op::Load { kind: Load::Word64, file: "rcm_05.7a", offset: 0x200000, size: 0x80000, crc: 0x5DD131FD },
            Op::Load { kind: Load::Word64, file: "rcm_06.8a", offset: 0x200002, size: 0x80000, crc: 0xF0FAF813 },
            Op::Load { kind: Load::Word64, file: "rcm_07.9a", offset: 0x200004, size: 0x80000, crc: 0x826DE013 },
            Op::Load { kind: Load::Word64, file: "rcm_08.10a", offset: 0x200006, size: 0x80000, crc: 0xFBFF64CF },
            Op::Load { kind: Load::Word64, file: "rcm_10.3c", offset: 0x400000, size: 0x80000, crc: 0x4DC8ADA9 },
            Op::Load { kind: Load::Word64, file: "rcm_11.4c", offset: 0x400002, size: 0x80000, crc: 0xF2B9EE06 },
            Op::Load { kind: Load::Word64, file: "rcm_12.5c", offset: 0x400004, size: 0x80000, crc: 0xFED5F203 },
            Op::Load { kind: Load::Word64, file: "rcm_13.6c", offset: 0x400006, size: 0x80000, crc: 0x5069D4A9 },
            Op::Load { kind: Load::Word64, file: "rcm_14.7c", offset: 0x600000, size: 0x80000, crc: 0x303BE3BD },
            Op::Load { kind: Load::Word64, file: "rcm_15.8c", offset: 0x600002, size: 0x80000, crc: 0x4F2D372F },
            Op::Load { kind: Load::Word64, file: "rcm_16.9c", offset: 0x600004, size: 0x80000, crc: 0x93D97FDE },
            Op::Load { kind: Load::Word64, file: "rcm_17.10c", offset: 0x600006, size: 0x80000, crc: 0x92371042 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm_09.12a", offset: 0x0, size: 0x8000, crc: 0x9632D6EF },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "rcm_18.11c", offset: 0x0, size: 0x20000, crc: 0x80F1F8AA },
            Op::Load { kind: Load::Byte, file: "rcm_19.12c", offset: 0x20000, size: 0x20000, crc: 0xF257DBE1 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, timekeeper
pub const RS_GANBARE: RomSetDef = RomSetDef { name: "ganbare", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mrnj_23d.8f", offset: 0x0, size: 0x80000, crc: 0xF929BE72 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mrnj_01.3a", offset: 0x0, size: 0x80000, crc: 0x3F878020 },
            Op::Load { kind: Load::Word64, file: "mrnj_02.4a", offset: 0x2, size: 0x80000, crc: 0x3E5624D8 },
            Op::Load { kind: Load::Word64, file: "mrnj_03.5a", offset: 0x4, size: 0x80000, crc: 0xD1E61F96 },
            Op::Load { kind: Load::Word64, file: "mrnj_04.6a", offset: 0x6, size: 0x80000, crc: 0xD241971B },
            Op::Load { kind: Load::Word64, file: "mrnj_05.7a", offset: 0x200000, size: 0x80000, crc: 0xC0A14562 },
            Op::Load { kind: Load::Word64, file: "mrnj_06.8a", offset: 0x200002, size: 0x80000, crc: 0xE6A71DFC },
            Op::Load { kind: Load::Word64, file: "mrnj_07.9a", offset: 0x200004, size: 0x80000, crc: 0x99AFB6C7 },
            Op::Load { kind: Load::Word64, file: "mrnj_08.10a", offset: 0x200006, size: 0x80000, crc: 0x52882C20 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mrnj_09.12a", offset: 0x0, size: 0x8000, crc: 0x62470D72 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mrnj_18.11c", offset: 0x0, size: 0x20000, crc: 0x08E13940 },
            Op::Load { kind: Load::Byte, file: "mrnj_19.12c", offset: 0x20000, size: 0x20000, crc: 0x5FA59927 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, timekeeper
pub const RS_PMONSTER: RomSetDef = RomSetDef { name: "pmonster", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "gbpj_23a.8f", offset: 0x0, size: 0x80000, crc: 0x52EF2D85 },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "gbpj_01.3a", offset: 0x0, size: 0x80000, crc: 0xA7BEA5BB },
            Op::Load { kind: Load::Word64, file: "gbpj_02.4a", offset: 0x2, size: 0x80000, crc: 0x357B76EC },
            Op::Load { kind: Load::Word64, file: "gbpj_03.5a", offset: 0x4, size: 0x80000, crc: 0xBCBC1881 },
            Op::Load { kind: Load::Word64, file: "gbpj_04.6a", offset: 0x6, size: 0x80000, crc: 0xB1126FDE },
            Op::Load { kind: Load::Word64, file: "gbpj_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBB5BE4B0 },
            Op::Load { kind: Load::Word64, file: "gbpj_06.8a", offset: 0x200002, size: 0x80000, crc: 0x1BE8FD86 },
            Op::Load { kind: Load::Word64, file: "gbpj_07.9a", offset: 0x200004, size: 0x80000, crc: 0xDEB8EF02 },
            Op::Load { kind: Load::Word64, file: "gbpj_08.10a", offset: 0x200006, size: 0x80000, crc: 0x9F90359D },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gbpj_09.12a", offset: 0x0, size: 0x8000, crc: 0xFC431024 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "gbpj_18.11c", offset: 0x0, size: 0x20000, crc: 0xCC54778D },
            Op::Load { kind: Load::Byte, file: "gbpj_19.12c", offset: 0x20000, size: 0x20000, crc: 0xEA8B56D8 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_POKONYAN: RomSetDef = RomSetDef { name: "pokonyan", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "xmqq-12f.bin", offset: 0x0, size: 0x20000, crc: 0x196297BF },
            Op::Load { kind: Load::Byte16, file: "xmqq-12h.bin", offset: 0x1, size: 0x20000, crc: 0x2D7EE2E9 },
            Op::Load { kind: Load::Byte16, file: "xmqq-13f.bin", offset: 0x40000, size: 0x20000, crc: 0x8F6ABF26 },
            Op::Load { kind: Load::Byte16, file: "xmqq-13h.bin", offset: 0x40001, size: 0x20000, crc: 0x3FEFE432 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte64, file: "xmqq-4b.bin", offset: 0x0, size: 0x20000, crc: 0x933AB76D },
            Op::Load { kind: Load::Byte64, file: "xmqq-4a.bin", offset: 0x1, size: 0x20000, crc: 0xB098E7A9 },
            Op::Load { kind: Load::Byte64, file: "xmqq-9b.bin", offset: 0x2, size: 0x20000, crc: 0xB66D62D4 },
            Op::Load { kind: Load::Byte64, file: "xmqq-9a.bin", offset: 0x3, size: 0x20000, crc: 0x9C23E40B },
            Op::Load { kind: Load::Byte64, file: "xmqq-5e.bin", offset: 0x4, size: 0x20000, crc: 0x63D06D6F },
            Op::Load { kind: Load::Byte64, file: "xmqq-5c.bin", offset: 0x5, size: 0x20000, crc: 0xE2169BB5 },
            Op::Load { kind: Load::Byte64, file: "xmqq-8h.bin", offset: 0x6, size: 0x20000, crc: 0x113121F5 },
            Op::Load { kind: Load::Byte64, file: "xmqq-8f.bin", offset: 0x7, size: 0x20000, crc: 0xBEB00E07 },
            Op::Load { kind: Load::Byte64, file: "xmqq-5b.bin", offset: 0x100000, size: 0x20000, crc: 0x05354905 },
            Op::Load { kind: Load::Byte64, file: "xmqq-5a.bin", offset: 0x100001, size: 0x20000, crc: 0xBD40215E },
            Op::Load { kind: Load::Byte64, file: "xmqq-10b.bin", offset: 0x100002, size: 0x20000, crc: 0x9FA773EF },
            Op::Load { kind: Load::Byte64, file: "xmqq-10a.bin", offset: 0x100003, size: 0x20000, crc: 0x638D4BC7 },
            Op::Load { kind: Load::Byte64, file: "xmqq-7e.bin", offset: 0x100004, size: 0x20000, crc: 0x72C45858 },
            Op::Load { kind: Load::Byte64, file: "xmqq-7c.bin", offset: 0x100005, size: 0x20000, crc: 0xD91CDA18 },
            Op::Load { kind: Load::Byte64, file: "xmqq-9h.bin", offset: 0x100006, size: 0x20000, crc: 0x3CD8594B },
            Op::Load { kind: Load::Byte64, file: "xmqq-9f.bin", offset: 0x100007, size: 0x20000, crc: 0x1EC10BED },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmqq-13b.bin", offset: 0x0, size: 0x8000, crc: 0x4E8B81A8 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "xmqq-12c.bin", offset: 0x0, size: 0x20000, crc: 0x71AC69AD },
            Op::Load { kind: Load::Byte, file: "xmqq-13c.bin", offset: 0x20000, size: 0x20000, crc: 0x71E29699 },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds
pub const RS_MPUMPKIN: RomSetDef = RomSetDef { name: "mpumpkin", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "mpa_23.8f", offset: 0x0, size: 0x80000, crc: 0x38B9883A },
        ] },
        RegionDef { name: "gfx", size: 0x600000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "mpa_01.3a", offset: 0x0, size: 0x80000, crc: 0x7C8C0C22 },
            Op::Load { kind: Load::Word64, file: "mpa_02.4a", offset: 0x2, size: 0x80000, crc: 0x23F95339 },
            Op::Load { kind: Load::Word64, file: "mpa_03.5a", offset: 0x4, size: 0x80000, crc: 0x107842A6 },
            Op::Load { kind: Load::Word64, file: "mpa_04.6a", offset: 0x6, size: 0x80000, crc: 0xFCE457AE },
            Op::Load { kind: Load::Word64, file: "mpa_05.7a", offset: 0x200000, size: 0x80000, crc: 0xBA8F3585 },
            Op::Load { kind: Load::Word64, file: "mpa_06.8a", offset: 0x200002, size: 0x80000, crc: 0x037F20CC },
            Op::Load { kind: Load::Word64, file: "mpa_07.9a", offset: 0x200004, size: 0x80000, crc: 0xBA8F3585 },
            Op::Load { kind: Load::Word64, file: "mpa_08.10a", offset: 0x200006, size: 0x80000, crc: 0x037F20CC },
            Op::Load { kind: Load::Word64, file: "mpa_10.3c", offset: 0x400000, size: 0x80000, crc: 0x870F3A2A },
            Op::Load { kind: Load::Word64, file: "mpa_11.4c", offset: 0x400002, size: 0x80000, crc: 0x8923FC3A },
            Op::Load { kind: Load::Word64, file: "mpa_12.5c", offset: 0x400004, size: 0x80000, crc: 0x87B88629 },
            Op::Load { kind: Load::Word64, file: "mpa_13.6c", offset: 0x400006, size: 0x80000, crc: 0xA09A6ACF },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpa_09.12a", offset: 0x0, size: 0x8000, crc: 0x0B5B1B72 },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mpa_18.11c", offset: 0x0, size: 0x20000, crc: 0xCEF6D39E },
            Op::Load { kind: Load::Byte, file: "mpa_19.12c", offset: 0x20000, size: 0x20000, crc: 0x24947F8E },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_GULUNPA: RomSetDef = RomSetDef { name: "gulunpa", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte16, file: "26", offset: 0x0, size: 0x20000, crc: 0xF30FFA29 },
            Op::Load { kind: Load::Byte16, file: "30", offset: 0x1, size: 0x20000, crc: 0x5D35F737 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "9", offset: 0x0, size: 0x8000, crc: 0x15AFD06F },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "gfx", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "1", offset: 0x0, size: 0x80000, crc: 0xB55E648F },
            Op::Load { kind: Load::Word64, file: "2", offset: 0x2, size: 0x80000, crc: 0xAD033BCE },
            Op::Load { kind: Load::Word64, file: "3", offset: 0x4, size: 0x80000, crc: 0x36C3951A },
            Op::Load { kind: Load::Word64, file: "4", offset: 0x6, size: 0x80000, crc: 0xFF0CB826 },
            Op::Fill { offset: 0x6, size: 0x1, value: 0xff },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0xff, ops: &[
            Op::Load { kind: Load::Byte, file: "18", offset: 0x0, size: 0x20000, crc: 0x9997A34F },
            Op::Load { kind: Load::Byte, file: "19", offset: 0x20000, size: 0x20000, crc: 0xE95270AC },
        ] },
] };
/// Régions ignorées : aboardplds, bboardplds, cboardplds, dboardplds
pub const RS_WOFCH: RomSetDef = RomSetDef { name: "wofch", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "tk2=ch=_23.8f", offset: 0x0, size: 0x80000, crc: 0x4E0B8DEE },
            Op::Load { kind: Load::Word16Swap, file: "tk2=ch=_22.7f", offset: 0x80000, size: 0x80000, crc: 0xD0937A8D },
        ] },
        RegionDef { name: "gfx", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "tk2-1m.3a", offset: 0x0, size: 0x80000, crc: 0x0D9CB9BF },
            Op::Load { kind: Load::Word64, file: "tk2-3m.5a", offset: 0x2, size: 0x80000, crc: 0x45227027 },
            Op::Load { kind: Load::Word64, file: "tk2-2m.4a", offset: 0x4, size: 0x80000, crc: 0xC5CA2460 },
            Op::Load { kind: Load::Word64, file: "tk2-4m.6a", offset: 0x6, size: 0x80000, crc: 0xE349551C },
            Op::Load { kind: Load::Word64, file: "tk2=ch=_05.7a", offset: 0x200000, size: 0x80000, crc: 0xE4A44D53 },
            Op::Load { kind: Load::Word64, file: "tk2=ch=_06.8a", offset: 0x200002, size: 0x80000, crc: 0x58066BA8 },
            Op::Load { kind: Load::Word64, file: "tk2=ch=_07.9a", offset: 0x200004, size: 0x80000, crc: 0xCC9006C9 },
            Op::Load { kind: Load::Word64, file: "tk2=ch=_08.10a", offset: 0x200006, size: 0x80000, crc: 0xD4A19A02 },
        ] },
        RegionDef { name: "audiocpu", size: 0x28000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2_qa.5k", offset: 0x0, size: 0x8000, crc: 0xC9183A0D },
            Op::Continue { offset: 0x10000, size: 0x18000 },
        ] },
        RegionDef { name: "qsound", size: 0x200000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "tk2-q1.1k", offset: 0x0, size: 0x80000, crc: 0x611268CF },
            Op::Load { kind: Load::Byte, file: "tk2-q2.2k", offset: 0x80000, size: 0x80000, crc: 0x20F55CA9 },
            Op::Load { kind: Load::Byte, file: "tk2-q3.3k", offset: 0x100000, size: 0x80000, crc: 0xBFCF6F52 },
            Op::Load { kind: Load::Byte, file: "tk2-q4.4k", offset: 0x180000, size: 0x80000, crc: 0x36642E88 },
        ] },
] };
/// Régions ignorées : bboardplds
pub const RS_SFACH: RomSetDef = RomSetDef { name: "sfach", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfach23", offset: 0x0, size: 0x80000, crc: 0x02A1A853 },
            Op::Load { kind: Load::Word16Swap, file: "sfza22", offset: 0x80000, size: 0x80000, crc: 0x8D9B2480 },
            Op::Load { kind: Load::Word16Swap, file: "sfzch21", offset: 0x100000, size: 0x80000, crc: 0x5435225D },
            Op::Load { kind: Load::Word16Swap, file: "sfza20", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfz01", offset: 0x0, size: 0x80000, crc: 0x0DD53E62 },
            Op::Load { kind: Load::Word64, file: "sfz02", offset: 0x2, size: 0x80000, crc: 0x94C31E3F },
            Op::Load { kind: Load::Word64, file: "sfz03", offset: 0x4, size: 0x80000, crc: 0x9584AC85 },
            Op::Load { kind: Load::Word64, file: "sfz04", offset: 0x6, size: 0x80000, crc: 0xB983624C },
            Op::Load { kind: Load::Word64, file: "sfz05", offset: 0x200000, size: 0x80000, crc: 0x2B47B645 },
            Op::Load { kind: Load::Word64, file: "sfz06", offset: 0x200002, size: 0x80000, crc: 0x74FD9FB1 },
            Op::Load { kind: Load::Word64, file: "sfz07", offset: 0x200004, size: 0x80000, crc: 0xBB2C734D },
            Op::Load { kind: Load::Word64, file: "sfz08", offset: 0x200006, size: 0x80000, crc: 0x454F7868 },
            Op::Load { kind: Load::Word64, file: "sfz10", offset: 0x400000, size: 0x80000, crc: 0x2A7D675E },
            Op::Load { kind: Load::Word64, file: "sfz11", offset: 0x400002, size: 0x80000, crc: 0xE35546C8 },
            Op::Load { kind: Load::Word64, file: "sfz12", offset: 0x400004, size: 0x80000, crc: 0xF122693A },
            Op::Load { kind: Load::Word64, file: "sfz13", offset: 0x400006, size: 0x80000, crc: 0x7CF942C8 },
            Op::Load { kind: Load::Word64, file: "sfz14", offset: 0x600000, size: 0x80000, crc: 0x09038C81 },
            Op::Load { kind: Load::Word64, file: "sfz15", offset: 0x600002, size: 0x80000, crc: 0x1AA17391 },
            Op::Load { kind: Load::Word64, file: "sfz16", offset: 0x600004, size: 0x80000, crc: 0x19A5ABD6 },
            Op::Load { kind: Load::Word64, file: "sfz17", offset: 0x600006, size: 0x80000, crc: 0x248B3B73 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz09", offset: 0x0, size: 0x8000, crc: 0xC772628B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz18", offset: 0x0, size: 0x20000, crc: 0x61022B2D },
            Op::Load { kind: Load::Byte, file: "sfz19", offset: 0x20000, size: 0x20000, crc: 0x3B5886D5 },
        ] },
] };
/// Régions ignorées : bboardplds
pub const RS_SFZCH: RomSetDef = RomSetDef { name: "sfzch", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzch23", offset: 0x0, size: 0x80000, crc: 0x1140743F },
            Op::Load { kind: Load::Word16Swap, file: "sfza22", offset: 0x80000, size: 0x80000, crc: 0x8D9B2480 },
            Op::Load { kind: Load::Word16Swap, file: "sfzch21", offset: 0x100000, size: 0x80000, crc: 0x5435225D },
            Op::Load { kind: Load::Word16Swap, file: "sfza20", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfz_01.3a", offset: 0x0, size: 0x80000, crc: 0x0DD53E62 },
            Op::Load { kind: Load::Word64, file: "sfz_02.4a", offset: 0x2, size: 0x80000, crc: 0x94C31E3F },
            Op::Load { kind: Load::Word64, file: "sfz_03.5a", offset: 0x4, size: 0x80000, crc: 0x9584AC85 },
            Op::Load { kind: Load::Word64, file: "sfz_04.6a", offset: 0x6, size: 0x80000, crc: 0xB983624C },
            Op::Load { kind: Load::Word64, file: "sfz_05.7a", offset: 0x200000, size: 0x80000, crc: 0x2B47B645 },
            Op::Load { kind: Load::Word64, file: "sfz_06.8a", offset: 0x200002, size: 0x80000, crc: 0x74FD9FB1 },
            Op::Load { kind: Load::Word64, file: "sfz_07.9a", offset: 0x200004, size: 0x80000, crc: 0xBB2C734D },
            Op::Load { kind: Load::Word64, file: "sfz_08.10a", offset: 0x200006, size: 0x80000, crc: 0x454F7868 },
            Op::Load { kind: Load::Word64, file: "sfz_10.3c", offset: 0x400000, size: 0x80000, crc: 0x2A7D675E },
            Op::Load { kind: Load::Word64, file: "sfz_11.4c", offset: 0x400002, size: 0x80000, crc: 0xE35546C8 },
            Op::Load { kind: Load::Word64, file: "sfz_12.5c", offset: 0x400004, size: 0x80000, crc: 0xF122693A },
            Op::Load { kind: Load::Word64, file: "sfz_13.6c", offset: 0x400006, size: 0x80000, crc: 0x7CF942C8 },
            Op::Load { kind: Load::Word64, file: "sfz_14.7c", offset: 0x600000, size: 0x80000, crc: 0x09038C81 },
            Op::Load { kind: Load::Word64, file: "sfz_15.8c", offset: 0x600002, size: 0x80000, crc: 0x1AA17391 },
            Op::Load { kind: Load::Word64, file: "sfz_16.9c", offset: 0x600004, size: 0x80000, crc: 0x19A5ABD6 },
            Op::Load { kind: Load::Word64, file: "sfz_17.10c", offset: 0x600006, size: 0x80000, crc: 0x248B3B73 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz_09.12a", offset: 0x0, size: 0x8000, crc: 0xC772628B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz_18.11c", offset: 0x0, size: 0x20000, crc: 0x61022B2D },
            Op::Load { kind: Load::Byte, file: "sfz_19.12c", offset: 0x20000, size: 0x20000, crc: 0x3B5886D5 },
        ] },
] };
/// Régions ignorées : bboardplds
pub const RS_SFZBCH: RomSetDef = RomSetDef { name: "sfzbch", regions: &[
        RegionDef { name: "maincpu", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "sfzbch23", offset: 0x0, size: 0x80000, crc: 0x53699F68 },
            Op::Load { kind: Load::Word16Swap, file: "sfza22", offset: 0x80000, size: 0x80000, crc: 0x8D9B2480 },
            Op::Load { kind: Load::Word16Swap, file: "sfzch21", offset: 0x100000, size: 0x80000, crc: 0x5435225D },
            Op::Load { kind: Load::Word16Swap, file: "sfza20", offset: 0x180000, size: 0x80000, crc: 0x806E8F38 },
        ] },
        RegionDef { name: "gfx", size: 0x800000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word64, file: "sfz_01.3a", offset: 0x0, size: 0x80000, crc: 0x0DD53E62 },
            Op::Load { kind: Load::Word64, file: "sfz_02.4a", offset: 0x2, size: 0x80000, crc: 0x94C31E3F },
            Op::Load { kind: Load::Word64, file: "sfz_03.5a", offset: 0x4, size: 0x80000, crc: 0x9584AC85 },
            Op::Load { kind: Load::Word64, file: "sfz_04.6a", offset: 0x6, size: 0x80000, crc: 0xB983624C },
            Op::Load { kind: Load::Word64, file: "sfz_05.7a", offset: 0x200000, size: 0x80000, crc: 0x2B47B645 },
            Op::Load { kind: Load::Word64, file: "sfz_06.8a", offset: 0x200002, size: 0x80000, crc: 0x74FD9FB1 },
            Op::Load { kind: Load::Word64, file: "sfz_07.9a", offset: 0x200004, size: 0x80000, crc: 0xBB2C734D },
            Op::Load { kind: Load::Word64, file: "sfz_08.10a", offset: 0x200006, size: 0x80000, crc: 0x454F7868 },
            Op::Load { kind: Load::Word64, file: "sfz_10.3c", offset: 0x400000, size: 0x80000, crc: 0x2A7D675E },
            Op::Load { kind: Load::Word64, file: "sfz_11.4c", offset: 0x400002, size: 0x80000, crc: 0xE35546C8 },
            Op::Load { kind: Load::Word64, file: "sfz_12.5c", offset: 0x400004, size: 0x80000, crc: 0xF122693A },
            Op::Load { kind: Load::Word64, file: "sfz_13.6c", offset: 0x400006, size: 0x80000, crc: 0x7CF942C8 },
            Op::Load { kind: Load::Word64, file: "sfz_14.7c", offset: 0x600000, size: 0x80000, crc: 0x09038C81 },
            Op::Load { kind: Load::Word64, file: "sfz_15.8c", offset: 0x600002, size: 0x80000, crc: 0x1AA17391 },
            Op::Load { kind: Load::Word64, file: "sfz_16.9c", offset: 0x600004, size: 0x80000, crc: 0x19A5ABD6 },
            Op::Load { kind: Load::Word64, file: "sfz_17.10c", offset: 0x600006, size: 0x80000, crc: 0x248B3B73 },
        ] },
        RegionDef { name: "stars", size: 0x8000, fill: 0x0, ops: &[
            Op::Copy { src: "gfx", src_offset: 0x0, offset: 0x0, size: 0x8000 },
        ] },
        RegionDef { name: "audiocpu", size: 0x18000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz_09.12a", offset: 0x0, size: 0x8000, crc: 0xC772628B },
            Op::Continue { offset: 0x10000, size: 0x8000 },
        ] },
        RegionDef { name: "oki", size: 0x40000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "sfz_18.11c", offset: 0x0, size: 0x20000, crc: 0x61022B2D },
            Op::Load { kind: Load::Byte, file: "sfz_19.12c", offset: 0x20000, size: 0x20000, crc: 0x3B5886D5 },
        ] },
] };
/// Régions ignorées : (aucune)
pub const RS_CPS1MULT: RomSetDef = RomSetDef { name: "cps1mult", regions: &[
        RegionDef { name: "maincpu", size: 0x4000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Word16Swap, file: "prg_0.bin", offset: 0x0, size: 0x800000, crc: 0xCBB4062C },
            Op::Load { kind: Load::Word16Swap, file: "prg_1.bin", offset: 0x800000, size: 0x800000, crc: 0xE434B882 },
            Op::Load { kind: Load::Word16Swap, file: "prg_2.bin", offset: 0x1000000, size: 0x800000, crc: 0x2CE2FC75 },
            Op::Load { kind: Load::Word16Swap, file: "prg_3.bin", offset: 0x1800000, size: 0x800000, crc: 0x0A93B43E },
            Op::Load { kind: Load::Word16Swap, file: "prg_4.bin", offset: 0x2000000, size: 0x800000, crc: 0x80B6DFB3 },
            Op::Load { kind: Load::Word16Swap, file: "prg_5.bin", offset: 0x2800000, size: 0x800000, crc: 0xBFA503E7 },
            Op::Load { kind: Load::Word16Swap, file: "prg_6.bin", offset: 0x3000000, size: 0x800000, crc: 0x977D2D34 },
            Op::Load { kind: Load::Word16Swap, file: "prg_7.bin", offset: 0x3800000, size: 0x800000, crc: 0x65B0D8FD },
        ] },
        RegionDef { name: "gfx", size: 0x8000000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "ca_0.bin", offset: 0x0, size: 0x800000, crc: 0x6819F572 },
            Op::Load { kind: Load::Byte, file: "ca_1.bin", offset: 0x800000, size: 0x800000, crc: 0xF18256D2 },
            Op::Load { kind: Load::Byte, file: "ca_2.bin", offset: 0x1000000, size: 0x800000, crc: 0x1F474165 },
            Op::Load { kind: Load::Byte, file: "ca_3.bin", offset: 0x1800000, size: 0x800000, crc: 0x1F7F36A7 },
            Op::Load { kind: Load::Byte, file: "ca_4.bin", offset: 0x2000000, size: 0x800000, crc: 0xB6C2AE5C },
            Op::Load { kind: Load::Byte, file: "ca_5.bin", offset: 0x2800000, size: 0x800000, crc: 0x68E7B97E },
            Op::Load { kind: Load::Byte, file: "ca_6.bin", offset: 0x3000000, size: 0x800000, crc: 0xD7DF2079 },
            Op::Load { kind: Load::Byte, file: "ca_7.bin", offset: 0x3800000, size: 0x800000, crc: 0x76C35A1F },
            Op::Load { kind: Load::Byte, file: "cb_0.bin", offset: 0x4000000, size: 0x800000, crc: 0xC9CA6EFC },
            Op::Load { kind: Load::Byte, file: "cb_1.bin", offset: 0x4800000, size: 0x800000, crc: 0xC53DE0E5 },
            Op::Load { kind: Load::Byte, file: "cb_2.bin", offset: 0x5000000, size: 0x800000, crc: 0x2934A28C },
            Op::Load { kind: Load::Byte, file: "cb_3.bin", offset: 0x5800000, size: 0x800000, crc: 0xCA41AE19 },
            Op::Load { kind: Load::Byte, file: "cb_4.bin", offset: 0x6000000, size: 0x800000, crc: 0xB9EA6163 },
            Op::Load { kind: Load::Byte, file: "cb_5.bin", offset: 0x6800000, size: 0x800000, crc: 0x74A402F2 },
            Op::Load { kind: Load::Byte, file: "cb_6.bin", offset: 0x7000000, size: 0x800000, crc: 0x73F471E1 },
            Op::Load { kind: Load::Byte, file: "cb_7.bin", offset: 0x7800000, size: 0x800000, crc: 0x7570E1F0 },
        ] },
        RegionDef { name: "audiocpu", size: 0x100000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "m1_074733.bin", offset: 0x0, size: 0x100000, crc: 0xFBB43B64 },
        ] },
        RegionDef { name: "oki", size: 0x400000, fill: 0x0, ops: &[
            Op::Load { kind: Load::Byte, file: "mx29f1610mcpsop44.u18", offset: 0x0, size: 0x200000, crc: 0x69CD2A53 },
            Op::Load { kind: Load::Byte, file: "mx29f1610mcpsop44.u19a", offset: 0x200000, size: 0x200000, crc: 0x4A0CEBAA },
        ] },
] };

/// Lignes GAME de cps1.cpp, dans l'ordre : (nom, parent, config machine, ports d'entrée, init, rotation, constructeur, titre, drapeaux).
pub static GAMES: &[GameInfo] = &[
    GameInfo { name: "forgottn", parent: "", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (World, newer)", not_working: false, romset: &RS_FORGOTTN },
    GameInfo { name: "forgottna", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (World)", not_working: false, romset: &RS_FORGOTTNA },
    GameInfo { name: "forgottnu", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (USA, B-Board 88621B-2, Rev. C)", not_working: false, romset: &RS_FORGOTTNU },
    GameInfo { name: "forgottnue", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (USA, B-Board 88618B-2, Rev. E)", not_working: false, romset: &RS_FORGOTTNUE },
    GameInfo { name: "forgottnuc", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (USA, B-Board 88618B-2, Rev. C)", not_working: false, romset: &RS_FORGOTTNUC },
    GameInfo { name: "forgottnua", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (USA, B-Board 88618B-2, Rev. A)", not_working: false, romset: &RS_FORGOTTNUA },
    GameInfo { name: "forgottnuaa", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (USA, B-Board 88618B-2, Rev. AA)", not_working: false, romset: &RS_FORGOTTNUAA },
    GameInfo { name: "forgottnj", parent: "forgottn", machine: "forgottn", input: "forgottnj", init: "empty_init", rotation: 0, maker: "Capcom", title: "Forgotten Worlds (Japan) (English prototype)", not_working: false, romset: &RS_FORGOTTNJ },
    GameInfo { name: "lostwrld", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Lost Worlds (Japan)", not_working: false, romset: &RS_LOSTWRLD },
    GameInfo { name: "lostwrldo", parent: "forgottn", machine: "forgottn", input: "forgottn", init: "empty_init", rotation: 0, maker: "Capcom", title: "Lost Worlds (Japan Old Ver.)", not_working: false, romset: &RS_LOSTWRLDO },
    GameInfo { name: "ghouls", parent: "", machine: "cps1_10MHz", input: "ghouls", init: "empty_init", rotation: 0, maker: "Capcom", title: "Ghouls'n Ghosts (World)", not_working: false, romset: &RS_GHOULS },
    GameInfo { name: "ghoulsu", parent: "ghouls", machine: "cps1_10MHz", input: "ghoulsu", init: "empty_init", rotation: 0, maker: "Capcom", title: "Ghouls'n Ghosts (USA)", not_working: false, romset: &RS_GHOULSU },
    GameInfo { name: "daimakai", parent: "ghouls", machine: "cps1_10MHz", input: "daimakai", init: "empty_init", rotation: 0, maker: "Capcom", title: "Daimakaimura (Japan)", not_working: false, romset: &RS_DAIMAKAI },
    GameInfo { name: "daimakair", parent: "ghouls", machine: "cps1_12MHz", input: "daimakair", init: "empty_init", rotation: 0, maker: "Capcom", title: "Daimakaimura (Japan Resale Ver.)", not_working: false, romset: &RS_DAIMAKAIR },
    GameInfo { name: "strider", parent: "", machine: "cps1_10MHz", input: "strider", init: "empty_init", rotation: 0, maker: "Capcom", title: "Strider (USA, B-Board 89624B-2)", not_working: false, romset: &RS_STRIDER },
    GameInfo { name: "striderua", parent: "strider", machine: "cps1_10MHz", input: "stridrua", init: "empty_init", rotation: 0, maker: "Capcom", title: "Strider (USA, B-Board 89624B-3)", not_working: false, romset: &RS_STRIDERUA },
    GameInfo { name: "strideruc", parent: "strider", machine: "cps1_10MHz", input: "stridrua", init: "empty_init", rotation: 0, maker: "bootleg (Capcom)", title: "Strider (USA, B-Board 90629B-3, buggy Street Fighter II conversion)", not_working: false, romset: &RS_STRIDERUC },
    GameInfo { name: "striderj", parent: "strider", machine: "cps1_10MHz", input: "strider", init: "empty_init", rotation: 0, maker: "Capcom", title: "Strider Hiryu (Japan)", not_working: false, romset: &RS_STRIDERJ },
    GameInfo { name: "striderjr", parent: "strider", machine: "cps1_12MHz", input: "strider", init: "empty_init", rotation: 0, maker: "Capcom", title: "Strider Hiryu (Japan Resale Ver.)", not_working: false, romset: &RS_STRIDERJR },
    GameInfo { name: "dynwar", parent: "", machine: "cps1_10MHz", input: "dynwar", init: "empty_init", rotation: 0, maker: "Capcom", title: "Dynasty Wars (USA, B-Board 89624B-?)", not_working: false, romset: &RS_DYNWAR },
    GameInfo { name: "dynwara", parent: "dynwar", machine: "cps1_10MHz", input: "dynwar", init: "empty_init", rotation: 0, maker: "Capcom", title: "Dynasty Wars (USA, B-Board 88622B-3)", not_working: false, romset: &RS_DYNWARA },
    GameInfo { name: "dynwarj", parent: "dynwar", machine: "cps1_10MHz", input: "dynwar", init: "empty_init", rotation: 0, maker: "Capcom", title: "Tenchi wo Kurau (Japan)", not_working: false, romset: &RS_DYNWARJ },
    GameInfo { name: "dynwarjr", parent: "dynwar", machine: "cps1_12MHz", input: "dynwar", init: "empty_init", rotation: 0, maker: "Capcom", title: "Tenchi wo Kurau (Japan Resale Ver.)", not_working: false, romset: &RS_DYNWARJR },
    GameInfo { name: "willow", parent: "", machine: "cps1_10MHz", input: "willow", init: "empty_init", rotation: 0, maker: "Capcom", title: "Willow (World)", not_working: false, romset: &RS_WILLOW },
    GameInfo { name: "willowu", parent: "willow", machine: "cps1_10MHz", input: "willow", init: "empty_init", rotation: 0, maker: "Capcom", title: "Willow (USA)", not_working: false, romset: &RS_WILLOWU },
    GameInfo { name: "willowuo", parent: "willow", machine: "cps1_10MHz", input: "willow", init: "empty_init", rotation: 0, maker: "Capcom", title: "Willow (USA Old Ver.)", not_working: false, romset: &RS_WILLOWUO },
    GameInfo { name: "willowj", parent: "willow", machine: "cps1_10MHz", input: "willow", init: "empty_init", rotation: 0, maker: "Capcom", title: "Willow (Japan)", not_working: false, romset: &RS_WILLOWJ },
    GameInfo { name: "unsquad", parent: "", machine: "cps1_10MHz", input: "unsquad", init: "empty_init", rotation: 0, maker: "Capcom", title: "U.N. Squadron (USA)", not_working: false, romset: &RS_UNSQUAD },
    GameInfo { name: "area88", parent: "unsquad", machine: "cps1_10MHz", input: "unsquad", init: "empty_init", rotation: 0, maker: "Capcom", title: "Area 88 (Japan)", not_working: false, romset: &RS_AREA88 },
    GameInfo { name: "area88r", parent: "unsquad", machine: "cps1_12MHz", input: "unsquad", init: "empty_init", rotation: 0, maker: "Capcom", title: "Area 88 (Japan Resale Ver.)", not_working: false, romset: &RS_AREA88R },
    GameInfo { name: "ffight", parent: "", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (World, set 1)", not_working: false, romset: &RS_FFIGHT },
    GameInfo { name: "ffighta", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (World, set 2)", not_working: false, romset: &RS_FFIGHTA },
    GameInfo { name: "ffightu", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (USA, set 1)", not_working: false, romset: &RS_FFIGHTU },
    GameInfo { name: "ffightu1", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (USA, set 2)", not_working: false, romset: &RS_FFIGHTU1 },
    GameInfo { name: "ffightua", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (USA 900112)", not_working: false, romset: &RS_FFIGHTUA },
    GameInfo { name: "ffightub", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (USA 900424)", not_working: false, romset: &RS_FFIGHTUB },
    GameInfo { name: "ffightuc", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (USA 900613)", not_working: false, romset: &RS_FFIGHTUC },
    GameInfo { name: "ffightj", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (Japan)", not_working: false, romset: &RS_FFIGHTJ },
    GameInfo { name: "ffightj1", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (Japan 900112)", not_working: false, romset: &RS_FFIGHTJ1 },
    GameInfo { name: "ffightj2", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (Japan 900305)", not_working: false, romset: &RS_FFIGHTJ2 },
    GameInfo { name: "ffightj3", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (Japan 900405)", not_working: false, romset: &RS_FFIGHTJ3 },
    GameInfo { name: "ffightj4", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "Capcom", title: "Final Fight (Japan 900613)", not_working: false, romset: &RS_FFIGHTJ4 },
    GameInfo { name: "ffightjh", parent: "ffight", machine: "cps1_10MHz", input: "ffight", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Smart / Final Fight (Japan, hack)", not_working: false, romset: &RS_FFIGHTJH },
    GameInfo { name: "ffightae", parent: "ffight", machine: "cps1_12MHz", input: "ffightae", init: "empty_init", rotation: 0, maker: "hack", title: "Final Fight 30th Anniversary Edition (World, hack)", not_working: false, romset: &RS_FFIGHTAE },
    GameInfo { name: "1941", parent: "", machine: "cps1_10MHz", input: "1941", init: "empty_init", rotation: 270, maker: "Capcom", title: "1941: Counter Attack (World 900227)", not_working: false, romset: &RS_1941 },
    GameInfo { name: "1941r1", parent: "1941", machine: "cps1_10MHz", input: "1941", init: "empty_init", rotation: 270, maker: "Capcom", title: "1941: Counter Attack (World)", not_working: false, romset: &RS_1941R1 },
    GameInfo { name: "1941u", parent: "1941", machine: "cps1_10MHz", input: "1941", init: "empty_init", rotation: 270, maker: "Capcom", title: "1941: Counter Attack (USA 900227)", not_working: false, romset: &RS_1941U },
    GameInfo { name: "1941j", parent: "1941", machine: "cps1_10MHz", input: "1941", init: "empty_init", rotation: 270, maker: "Capcom", title: "1941: Counter Attack (Japan)", not_working: false, romset: &RS_1941J },
    GameInfo { name: "mercs", parent: "", machine: "cps1_10MHz", input: "mercs", init: "empty_init", rotation: 270, maker: "Capcom", title: "Mercs (World 900302)", not_working: false, romset: &RS_MERCS },
    GameInfo { name: "mercsu", parent: "mercs", machine: "cps1_10MHz", input: "mercs", init: "empty_init", rotation: 270, maker: "Capcom", title: "Mercs (USA 900608)", not_working: false, romset: &RS_MERCSU },
    GameInfo { name: "mercsur1", parent: "mercs", machine: "cps1_10MHz", input: "mercs", init: "empty_init", rotation: 270, maker: "Capcom", title: "Mercs (USA 900302)", not_working: false, romset: &RS_MERCSUR1 },
    GameInfo { name: "mercsj", parent: "mercs", machine: "cps1_10MHz", input: "mercs", init: "empty_init", rotation: 270, maker: "Capcom", title: "Senjou no Ookami II (Japan 900302)", not_working: false, romset: &RS_MERCSJ },
    GameInfo { name: "mtwins", parent: "", machine: "cps1_10MHz", input: "mtwins", init: "empty_init", rotation: 0, maker: "Capcom", title: "Mega Twins (World 900619)", not_working: false, romset: &RS_MTWINS },
    GameInfo { name: "chikij", parent: "mtwins", machine: "cps1_10MHz", input: "mtwins", init: "empty_init", rotation: 0, maker: "Capcom", title: "Chiki Chiki Boys (Japan 900619)", not_working: false, romset: &RS_CHIKIJ },
    GameInfo { name: "msword", parent: "", machine: "cps1_10MHz", input: "msword", init: "empty_init", rotation: 0, maker: "Capcom", title: "Magic Sword: Heroic Fantasy (World 900725)", not_working: false, romset: &RS_MSWORD },
    GameInfo { name: "mswordr1", parent: "msword", machine: "cps1_10MHz", input: "msword", init: "empty_init", rotation: 0, maker: "Capcom", title: "Magic Sword: Heroic Fantasy (World 900623)", not_working: false, romset: &RS_MSWORDR1 },
    GameInfo { name: "mswordu", parent: "msword", machine: "cps1_10MHz", input: "msword", init: "empty_init", rotation: 0, maker: "Capcom", title: "Magic Sword: Heroic Fantasy (USA 900725)", not_working: false, romset: &RS_MSWORDU },
    GameInfo { name: "mswordj", parent: "msword", machine: "cps1_10MHz", input: "msword", init: "empty_init", rotation: 0, maker: "Capcom", title: "Magic Sword: Heroic Fantasy (Japan 900623)", not_working: false, romset: &RS_MSWORDJ },
    GameInfo { name: "cawing", parent: "", machine: "cps1_10MHz", input: "cawing", init: "empty_init", rotation: 0, maker: "Capcom", title: "Carrier Air Wing (World 901012)", not_working: false, romset: &RS_CAWING },
    GameInfo { name: "cawingr1", parent: "cawing", machine: "cps1_10MHz", input: "cawing", init: "empty_init", rotation: 0, maker: "Capcom", title: "Carrier Air Wing (World 901009)", not_working: false, romset: &RS_CAWINGR1 },
    GameInfo { name: "cawingu", parent: "cawing", machine: "cps1_10MHz", input: "cawing", init: "empty_init", rotation: 0, maker: "Capcom", title: "Carrier Air Wing (USA 901130)", not_working: false, romset: &RS_CAWINGU },
    GameInfo { name: "cawingur1", parent: "cawing", machine: "cps1_10MHz", input: "cawing", init: "empty_init", rotation: 0, maker: "Capcom", title: "Carrier Air Wing (USA 901012)", not_working: false, romset: &RS_CAWINGUR1 },
    GameInfo { name: "cawingj", parent: "cawing", machine: "cps1_10MHz", input: "cawing", init: "empty_init", rotation: 0, maker: "Capcom", title: "U.S. Navy (Japan 901012)", not_working: false, romset: &RS_CAWINGJ },
    GameInfo { name: "cawingjr", parent: "cawing", machine: "cps1_12MHz", input: "cawing", init: "empty_init", rotation: 0, maker: "Capcom", title: "U.S. Navy (Japan Resale Ver. 901130)", not_working: false, romset: &RS_CAWINGJR },
    GameInfo { name: "nemo", parent: "", machine: "cps1_10MHz", input: "nemo", init: "empty_init", rotation: 0, maker: "Capcom", title: "Nemo (World 901130)", not_working: false, romset: &RS_NEMO },
    GameInfo { name: "nemor1", parent: "nemo", machine: "cps1_10MHz", input: "nemo", init: "empty_init", rotation: 0, maker: "Capcom", title: "Nemo (World 901109)", not_working: false, romset: &RS_NEMOR1 },
    GameInfo { name: "nemoj", parent: "nemo", machine: "cps1_10MHz", input: "nemo", init: "empty_init", rotation: 0, maker: "Capcom", title: "Nemo (Japan 901120, 88622B-3 ROM board)", not_working: false, romset: &RS_NEMOJ },
    GameInfo { name: "nemoja", parent: "nemo", machine: "cps1_10MHz", input: "nemo", init: "empty_init", rotation: 0, maker: "Capcom", title: "Nemo (Japan 901120, 89625B-1 ROM board)", not_working: false, romset: &RS_NEMOJA },
    GameInfo { name: "sf2", parent: "", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910522)", not_working: false, romset: &RS_SF2 },
    GameInfo { name: "sf2ea", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910204)", not_working: false, romset: &RS_SF2EA },
    GameInfo { name: "sf2eb", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910214)", not_working: false, romset: &RS_SF2EB },
    GameInfo { name: "sf2ed", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910318)", not_working: false, romset: &RS_SF2ED },
    GameInfo { name: "sf2ee", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "init_sf2ee", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910228)", not_working: false, romset: &RS_SF2EE },
    GameInfo { name: "sf2ef", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910411)", not_working: false, romset: &RS_SF2EF },
    GameInfo { name: "sf2em", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910129)", not_working: false, romset: &RS_SF2EM },
    GameInfo { name: "sf2en", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (World 910204, conversion)", not_working: false, romset: &RS_SF2EN },
    GameInfo { name: "sf2ua", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910206)", not_working: false, romset: &RS_SF2UA },
    GameInfo { name: "sf2ub", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910214)", not_working: false, romset: &RS_SF2UB },
    GameInfo { name: "sf2uc", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910306)", not_working: false, romset: &RS_SF2UC },
    GameInfo { name: "sf2ud", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910318)", not_working: false, romset: &RS_SF2UD },
    GameInfo { name: "sf2ue", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "init_sf2ee", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910228)", not_working: false, romset: &RS_SF2UE },
    GameInfo { name: "sf2uf", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910411)", not_working: false, romset: &RS_SF2UF },
    GameInfo { name: "sf2ug", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910522, Rev. G)", not_working: false, romset: &RS_SF2UG },
    GameInfo { name: "sf2uh", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910522, Rev. H)", not_working: false, romset: &RS_SF2UH },
    GameInfo { name: "sf2ui", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 910522, Rev. I)", not_working: false, romset: &RS_SF2UI },
    GameInfo { name: "sf2uk", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 911101)", not_working: false, romset: &RS_SF2UK },
    GameInfo { name: "sf2um", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (USA 920312)", not_working: false, romset: &RS_SF2UM },
    GameInfo { name: "sf2j", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 911210, CPS-B-13)", not_working: false, romset: &RS_SF2J },
    GameInfo { name: "sf2j17", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 911210, CPS-B-17)", not_working: false, romset: &RS_SF2J17 },
    GameInfo { name: "sf2ja", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 910214)", not_working: false, romset: &RS_SF2JA },
    GameInfo { name: "sf2jc", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 910306)", not_working: false, romset: &RS_SF2JC },
    GameInfo { name: "sf2jf", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 910411)", not_working: false, romset: &RS_SF2JF },
    GameInfo { name: "sf2jh", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 910522)", not_working: false, romset: &RS_SF2JH },
    GameInfo { name: "sf2jl", parent: "sf2", machine: "cps1_10MHz", input: "sf2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II: The World Warrior (Japan 920312)", not_working: false, romset: &RS_SF2JL },
    GameInfo { name: "sf2ebbl", parent: "sf2", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (TAB Austria, bootleg, set 1)", not_working: false, romset: &RS_SF2EBBL },
    GameInfo { name: "sf2ebbl2", parent: "sf2", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (TAB Austria, bootleg, set 3)", not_working: false, romset: &RS_SF2EBBL2 },
    GameInfo { name: "sf2ebbl3", parent: "sf2", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (TAB Austria, bootleg, set 4)", not_working: false, romset: &RS_SF2EBBL3 },
    GameInfo { name: "sf2stt", parent: "sf2", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (TAB Austria, bootleg, set 2)", not_working: true, romset: &RS_SF2STT },
    GameInfo { name: "sf2rk", parent: "sf2", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2rk", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (RK, bootleg)", not_working: false, romset: &RS_SF2RK },
    GameInfo { name: "sf2qp1", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (Quicken Pt-I, bootleg)", not_working: false, romset: &RS_SF2QP1 },
    GameInfo { name: "sf2qp2", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (Quicken, bootleg)", not_working: false, romset: &RS_SF2QP2 },
    GameInfo { name: "sf2thndr", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "init_sf2thndr", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (Thunder Edition, bootleg, set 1)", not_working: false, romset: &RS_SF2THNDR },
    GameInfo { name: "sf2thndr2", parent: "sf2", machine: "cps1_10MHz", input: "sf2", init: "init_sf2thndr", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (Thunder Edition, bootleg, set 2)", not_working: false, romset: &RS_SF2THNDR2 },
    GameInfo { name: "sf2rules", parent: "sf2", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II: The World Warrior (bootleg with rules screen)", not_working: false, romset: &RS_SF2RULES },
    GameInfo { name: "3wonders", parent: "", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "Capcom", title: "Three Wonders (World 910520)", not_working: false, romset: &RS_3WONDERS },
    GameInfo { name: "3wondersr1", parent: "3wonders", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "Capcom", title: "Three Wonders (World 910513)", not_working: false, romset: &RS_3WONDERSR1 },
    GameInfo { name: "3wondersu", parent: "3wonders", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "Capcom", title: "Three Wonders (USA 910520)", not_working: false, romset: &RS_3WONDERSU },
    GameInfo { name: "wonder3", parent: "3wonders", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "Capcom", title: "Wonder 3 (Japan 910520)", not_working: false, romset: &RS_WONDER3 },
    GameInfo { name: "3wondersb", parent: "3wonders", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "bootleg", title: "Three Wonders (bootleg)", not_working: false, romset: &RS_3WONDERSB },
    GameInfo { name: "3wondersbi", parent: "3wonders", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "bootleg (Electronic Devices)", title: "Three Wonders (Italian bootleg)", not_working: true, romset: &RS_3WONDERSBI },
    GameInfo { name: "3wondersh", parent: "3wonders", machine: "cps1_10MHz", input: "3wonders", init: "empty_init", rotation: 0, maker: "bootleg", title: "Three Wonders (hack)", not_working: false, romset: &RS_3WONDERSH },
    GameInfo { name: "kod", parent: "", machine: "cps1_10MHz", input: "kod", init: "empty_init", rotation: 0, maker: "Capcom", title: "The King of Dragons (World 910805)", not_working: false, romset: &RS_KOD },
    GameInfo { name: "kodr1", parent: "kod", machine: "cps1_10MHz", input: "kodr1", init: "empty_init", rotation: 0, maker: "Capcom", title: "The King of Dragons (World 910711)", not_working: false, romset: &RS_KODR1 },
    GameInfo { name: "kodr2", parent: "kod", machine: "cps1_10MHz", input: "kodr1", init: "empty_init", rotation: 0, maker: "Capcom", title: "The King of Dragons (World 910731)", not_working: false, romset: &RS_KODR2 },
    GameInfo { name: "kodu", parent: "kod", machine: "cps1_10MHz", input: "kod", init: "empty_init", rotation: 0, maker: "Capcom", title: "The King of Dragons (USA 910910)", not_working: false, romset: &RS_KODU },
    GameInfo { name: "kodj", parent: "kod", machine: "cps1_10MHz", input: "kod", init: "empty_init", rotation: 0, maker: "Capcom", title: "The King of Dragons (Japan 910805, B-Board 90629B-3)", not_working: false, romset: &RS_KODJ },
    GameInfo { name: "kodja", parent: "kod", machine: "cps1_10MHz", input: "kod", init: "empty_init", rotation: 0, maker: "Capcom", title: "The King of Dragons (Japan 910805, B-Board 89625B-1)", not_working: false, romset: &RS_KODJA },
    GameInfo { name: "captcomm", parent: "", machine: "cps1_10MHz", input: "captcomm", init: "empty_init", rotation: 0, maker: "Capcom", title: "Captain Commando (World 911202)", not_working: false, romset: &RS_CAPTCOMM },
    GameInfo { name: "captcommr1", parent: "captcomm", machine: "cps1_10MHz", input: "captcomm", init: "empty_init", rotation: 0, maker: "Capcom", title: "Captain Commando (World 911014)", not_working: false, romset: &RS_CAPTCOMMR1 },
    GameInfo { name: "captcommu", parent: "captcomm", machine: "cps1_10MHz", input: "captcomm", init: "empty_init", rotation: 0, maker: "Capcom", title: "Captain Commando (USA 910928)", not_working: false, romset: &RS_CAPTCOMMU },
    GameInfo { name: "captcommj", parent: "captcomm", machine: "cps1_10MHz", input: "captcomm", init: "empty_init", rotation: 0, maker: "Capcom", title: "Captain Commando (Japan 911202)", not_working: false, romset: &RS_CAPTCOMMJ },
    GameInfo { name: "captcommjr1", parent: "captcomm", machine: "cps1_10MHz", input: "captcomm", init: "empty_init", rotation: 0, maker: "Capcom", title: "Captain Commando (Japan 910928)", not_working: false, romset: &RS_CAPTCOMMJR1 },
    GameInfo { name: "captcommb", parent: "captcomm", machine: "cps1_10MHz", input: "captcomm", init: "empty_init", rotation: 0, maker: "bootleg", title: "Captain Commando (bootleg)", not_working: false, romset: &RS_CAPTCOMMB },
    GameInfo { name: "knights", parent: "", machine: "cps1_10MHz", input: "knights", init: "empty_init", rotation: 0, maker: "Capcom", title: "Knights of the Round (World 911127)", not_working: false, romset: &RS_KNIGHTS },
    GameInfo { name: "knightsu", parent: "knights", machine: "cps1_10MHz", input: "knights", init: "empty_init", rotation: 0, maker: "Capcom", title: "Knights of the Round (USA 911127)", not_working: false, romset: &RS_KNIGHTSU },
    GameInfo { name: "knightsj", parent: "knights", machine: "cps1_10MHz", input: "knights", init: "empty_init", rotation: 0, maker: "Capcom", title: "Knights of the Round (Japan 911127, B-Board 91634B-2)", not_working: false, romset: &RS_KNIGHTSJ },
    GameInfo { name: "knightsja", parent: "knights", machine: "cps1_10MHz", input: "knights", init: "empty_init", rotation: 0, maker: "Capcom", title: "Knights of the Round (Japan 911127, B-Board 89625B-1)", not_working: false, romset: &RS_KNIGHTSJA },
    GameInfo { name: "knightsb2", parent: "knights", machine: "cps1_10MHz", input: "knights", init: "empty_init", rotation: 0, maker: "bootleg", title: "Knights of the Round (bootleg, World 911127)", not_working: false, romset: &RS_KNIGHTSB2 },
    GameInfo { name: "sf2ce", parent: "", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (World 920513)", not_working: false, romset: &RS_SF2CE },
    GameInfo { name: "sf2ceea", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (World 920313)", not_working: false, romset: &RS_SF2CEEA },
    GameInfo { name: "sf2ceec", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (World 920803)", not_working: false, romset: &RS_SF2CEEC },
    GameInfo { name: "sf2ceua", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (USA 920313)", not_working: false, romset: &RS_SF2CEUA },
    GameInfo { name: "sf2ceub", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (USA 920513)", not_working: false, romset: &RS_SF2CEUB },
    GameInfo { name: "sf2ceuc", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (USA 920803)", not_working: false, romset: &RS_SF2CEUC },
    GameInfo { name: "sf2cet", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (Taiwan 920313)", not_working: false, romset: &RS_SF2CET },
    GameInfo { name: "sf2ceja", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2cej", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (Japan 920322)", not_working: false, romset: &RS_SF2CEJA },
    GameInfo { name: "sf2cejb", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2cej", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (Japan 920513)", not_working: false, romset: &RS_SF2CEJB },
    GameInfo { name: "sf2cejc", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2cej", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Champion Edition (Japan 920803)", not_working: false, romset: &RS_SF2CEJC },
    GameInfo { name: "sf2bhh", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2bhh", init: "init_sf2rb", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Hung Hsi, bootleg)", not_working: false, romset: &RS_SF2BHH },
    GameInfo { name: "sf2cebltw", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2bhh", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition ('Taiwan' bootleg with PAL)", not_working: false, romset: &RS_SF2CEBLTW },
    GameInfo { name: "sf2rb", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2rb", init: "init_sf2rb", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Rainbow, bootleg, set 1)", not_working: false, romset: &RS_SF2RB },
    GameInfo { name: "sf2rb2", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2rb", init: "init_sf2rb2", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Rainbow, bootleg, set 2)", not_working: false, romset: &RS_SF2RB2 },
    GameInfo { name: "sf2rb3", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2rb", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Rainbow, bootleg, set 3)", not_working: false, romset: &RS_SF2RB3 },
    GameInfo { name: "sf2red", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Red Wave, bootleg, set 1)", not_working: false, romset: &RS_SF2RED },
    GameInfo { name: "sf2reda", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Red Wave, bootleg, set 2)", not_working: false, romset: &RS_SF2REDA },
    GameInfo { name: "sf2redp2", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Red Wave PtII, bootleg)", not_working: false, romset: &RS_SF2REDP2 },
    GameInfo { name: "sf2v004", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (V004, bootleg)", not_working: false, romset: &RS_SF2V004 },
    GameInfo { name: "sf2acc", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Accelerator!, bootleg, set 1)", not_working: false, romset: &RS_SF2ACC },
    GameInfo { name: "sf2acca", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Accelerator!, bootleg, set 2)", not_working: false, romset: &RS_SF2ACCA },
    GameInfo { name: "sf2accp2", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2accp2", init: "empty_init", rotation: 0, maker: "bootleg (Testron)", title: "Street Fighter II': Champion Edition (Accelerator Pt.II, bootleg)", not_working: false, romset: &RS_SF2ACCP2 },
    GameInfo { name: "sf2amf", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2amf", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Alpha Magic-F, bootleg)", not_working: true, romset: &RS_SF2AMF },
    GameInfo { name: "sf2amf2", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2amfx", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (L735 Test Rom, bootleg, set 1)", not_working: false, romset: &RS_SF2AMF2 },
    GameInfo { name: "sf2amf3", parent: "sf2ce", machine: "cps1_10MHz", input: "sf2amfx", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (L735 Test Rom, bootleg, set 2)", not_working: false, romset: &RS_SF2AMF3 },
    GameInfo { name: "sf2dkot2", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Double K.O. Turbo II, bootleg)", not_working: false, romset: &RS_SF2DKOT2 },
    GameInfo { name: "sf2level", parent: "sf2ce", machine: "sf2m3", input: "sf2level", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (bootleg with level selection)", not_working: false, romset: &RS_SF2LEVEL },
    GameInfo { name: "sf2ceblp", parent: "sf2ce", machine: "cps1_10MHz", input: "sf2", init: "init_sf2ceblp", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (protected bootleg on non-dash board)", not_working: false, romset: &RS_SF2CEBLP },
    GameInfo { name: "sf2m2", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2m2", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M2, bootleg)", not_working: false, romset: &RS_SF2M2 },
    GameInfo { name: "sf2m3", parent: "sf2ce", machine: "sf2m3", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M3, bootleg)", not_working: false, romset: &RS_SF2M3 },
    GameInfo { name: "sf2m4", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2m4", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M4, bootleg)", not_working: false, romset: &RS_SF2M4 },
    GameInfo { name: "sf2m5", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M5, bootleg)", not_working: false, romset: &RS_SF2M5 },
    GameInfo { name: "sf2m6", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M6, bootleg)", not_working: false, romset: &RS_SF2M6 },
    GameInfo { name: "sf2m7", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M7, bootleg)", not_working: false, romset: &RS_SF2M7 },
    GameInfo { name: "sf2m8", parent: "sf2ce", machine: "sf2m3", input: "sf2", init: "init_sf2m8", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M8, bootleg)", not_working: false, romset: &RS_SF2M8 },
    GameInfo { name: "sf2m10", parent: "sf2ce", machine: "sf2m10", input: "sf2hack", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (M10, bootleg)", not_working: false, romset: &RS_SF2M10 },
    GameInfo { name: "sf2yyc", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (YYC, bootleg)", not_working: false, romset: &RS_SF2YYC },
    GameInfo { name: "sf2koryu", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Xiang Long, Chinese bootleg)", not_working: false, romset: &RS_SF2KORYU },
    GameInfo { name: "sf2dongb", parent: "sf2ce", machine: "cps1_12MHz", input: "sf2", init: "init_sf2dongb", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Dongfang Bubai protection, bootleg)", not_working: false, romset: &RS_SF2DONGB },
    GameInfo { name: "sf2ceupl", parent: "sf2ce", machine: "sf2m10", input: "sf2hack", init: "empty_init", rotation: 0, maker: "bootleg (UPL?)", title: "Street Fighter II': Champion Edition (Japan 920322, UPL bootleg?)", not_working: false, romset: &RS_SF2CEUPL },
    GameInfo { name: "sf2cems6a", parent: "sf2ce", machine: "sf2cems6", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Mstreet-6, bootleg, set 1)", not_working: false, romset: &RS_SF2CEMS6A },
    GameInfo { name: "sf2cems6b", parent: "sf2ce", machine: "sf2cems6", input: "sf2bhh", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Mstreet-6, bootleg, set 2)", not_working: false, romset: &RS_SF2CEMS6B },
    GameInfo { name: "sf2cems6c", parent: "sf2ce", machine: "sf2cems6", input: "sf2bhh", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Mstreet-6, bootleg, set 3)", not_working: false, romset: &RS_SF2CEMS6C },
    GameInfo { name: "sf2ceds6", parent: "sf2ce", machine: "sf2cems6", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (Dstreet-6, bootleg)", not_working: false, romset: &RS_SF2CEDS6 },
    GameInfo { name: "sf2re", parent: "sf2ce", machine: "sf2m3", input: "sf2", init: "empty_init", rotation: 0, maker: "bootleg", title: "Street Fighter II': Champion Edition (RE, bootleg)", not_working: false, romset: &RS_SF2RE },
    GameInfo { name: "sf2mkot", parent: "sf2ce", machine: "cps1_10MHz", input: "sf2hack", init: "init_sf2hack", rotation: 0, maker: "bootleg", title: "Street Fighter II': Magic KO Turbo!! - Nightmare Crack", not_working: false, romset: &RS_SF2MKOT },
    GameInfo { name: "cworld2j", parent: "", machine: "cps1_12MHz", input: "cworld2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Adventure Quiz Capcom World 2 (Japan 920611)", not_working: false, romset: &RS_CWORLD2J },
    GameInfo { name: "cworld2ja", parent: "cworld2j", machine: "cps1_12MHz", input: "cworld2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Adventure Quiz Capcom World 2 (Japan 920611, B-Board 90629B-3, no battery)", not_working: false, romset: &RS_CWORLD2JA },
    GameInfo { name: "cworld2jb", parent: "cworld2j", machine: "cps1_12MHz", input: "cworld2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Adventure Quiz Capcom World 2 (Japan 920611, B-Board 91634B-2)", not_working: false, romset: &RS_CWORLD2JB },
    GameInfo { name: "varth", parent: "", machine: "cps1_12MHz", input: "varth", init: "empty_init", rotation: 270, maker: "Capcom", title: "Varth: Operation Thunderstorm (World 920714)", not_working: false, romset: &RS_VARTH },
    GameInfo { name: "varthr1", parent: "varth", machine: "cps1_12MHz", input: "varth", init: "empty_init", rotation: 270, maker: "Capcom", title: "Varth: Operation Thunderstorm (World 920612)", not_working: false, romset: &RS_VARTHR1 },
    GameInfo { name: "varthu", parent: "varth", machine: "cps1_12MHz", input: "varth", init: "empty_init", rotation: 270, maker: "Capcom (Romstar license)", title: "Varth: Operation Thunderstorm (USA 920612)", not_working: false, romset: &RS_VARTHU },
    GameInfo { name: "varthj", parent: "varth", machine: "cps1_12MHz", input: "varth", init: "empty_init", rotation: 270, maker: "Capcom", title: "Varth: Operation Thunderstorm (Japan 920714)", not_working: false, romset: &RS_VARTHJ },
    GameInfo { name: "varthjr", parent: "varth", machine: "cps1_12MHz", input: "varth", init: "empty_init", rotation: 270, maker: "Capcom", title: "Varth: Operation Thunderstorm (Japan Resale Ver. 920714)", not_working: false, romset: &RS_VARTHJR },
    GameInfo { name: "varthb2", parent: "varth", machine: "varthb2", input: "varth", init: "empty_init", rotation: 270, maker: "bootleg", title: "Varth: Operation Thunderstorm (bootleg, set 2)", not_working: false, romset: &RS_VARTHB2 },
    GameInfo { name: "varthb3", parent: "varth", machine: "varthb3", input: "varth", init: "empty_init", rotation: 270, maker: "bootleg", title: "Varth: Operation Thunderstorm (bootleg, set 3)", not_working: true, romset: &RS_VARTHB3 },
    GameInfo { name: "qad", parent: "", machine: "cps1_12MHz", input: "qad", init: "empty_init", rotation: 0, maker: "Capcom", title: "Quiz & Dragons: Capcom Quiz Game (USA 920701)", not_working: false, romset: &RS_QAD },
    GameInfo { name: "qadjr", parent: "qad", machine: "cps1_12MHz", input: "qadjr", init: "empty_init", rotation: 0, maker: "Capcom", title: "Quiz & Dragons: Capcom Quiz Game (Japan Resale Ver. 940921)", not_working: false, romset: &RS_QADJR },
    GameInfo { name: "wof", parent: "", machine: "qsound", input: "wof", init: "init_wof", rotation: 0, maker: "Capcom", title: "Warriors of Fate (World 921031)", not_working: false, romset: &RS_WOF },
    GameInfo { name: "wofr1", parent: "wof", machine: "qsound", input: "wof", init: "init_wof", rotation: 0, maker: "Capcom", title: "Warriors of Fate (World 921002)", not_working: false, romset: &RS_WOFR1 },
    GameInfo { name: "wofu", parent: "wof", machine: "qsound", input: "wof", init: "init_wof", rotation: 0, maker: "Capcom", title: "Warriors of Fate (USA 921031)", not_working: false, romset: &RS_WOFU },
    GameInfo { name: "wofa", parent: "wof", machine: "qsound", input: "wof", init: "init_wof", rotation: 0, maker: "Capcom", title: "Sangokushi II (Asia 921005)", not_working: false, romset: &RS_WOFA },
    GameInfo { name: "wofj", parent: "wof", machine: "qsound", input: "wof", init: "init_wof", rotation: 0, maker: "Capcom", title: "Tenchi wo Kurau II: Sekiheki no Tatakai (Japan 921031)", not_working: false, romset: &RS_WOFJ },
    GameInfo { name: "wofhfh", parent: "wof", machine: "wofhfh", input: "wofhfh", init: "empty_init", rotation: 0, maker: "bootleg", title: "Huo Feng Huang (Chinese bootleg of Sangokushi II)", not_working: false, romset: &RS_WOFHFH },
    GameInfo { name: "sf2hf", parent: "", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Hyper Fighting (World 921209)", not_working: false, romset: &RS_SF2HF },
    GameInfo { name: "sf2hfu", parent: "sf2hf", machine: "cps1_12MHz", input: "sf2", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II': Hyper Fighting (USA 921209)", not_working: false, romset: &RS_SF2HFU },
    GameInfo { name: "sf2hfj", parent: "sf2hf", machine: "cps1_12MHz", input: "sf2cej", init: "empty_init", rotation: 0, maker: "Capcom", title: "Street Fighter II' Turbo: Hyper Fighting (Japan 921209)", not_working: false, romset: &RS_SF2HFJ },
    GameInfo { name: "dino", parent: "", machine: "qsound", input: "dino", init: "init_dino", rotation: 0, maker: "Capcom", title: "Cadillacs and Dinosaurs (World 930201)", not_working: false, romset: &RS_DINO },
    GameInfo { name: "dinou", parent: "dino", machine: "qsound", input: "dino", init: "init_dino", rotation: 0, maker: "Capcom", title: "Cadillacs and Dinosaurs (USA 930201)", not_working: false, romset: &RS_DINOU },
    GameInfo { name: "dinoa", parent: "dino", machine: "qsound", input: "dino", init: "init_dino", rotation: 0, maker: "Capcom", title: "Cadillacs and Dinosaurs (Asia TW 930223)", not_working: false, romset: &RS_DINOA },
    GameInfo { name: "dinoj", parent: "dino", machine: "qsound", input: "dino", init: "init_dino", rotation: 0, maker: "Capcom", title: "Cadillacs: Kyouryuu Shin Seiki (Japan 930201)", not_working: false, romset: &RS_DINOJ },
    GameInfo { name: "dinohunt", parent: "dino", machine: "wofhfh", input: "dinoh", init: "init_dinohunt", rotation: 0, maker: "bootleg", title: "Dinosaur Hunter (Chinese bootleg of Cadillacs and Dinosaurs)", not_working: false, romset: &RS_DINOHUNT },
    GameInfo { name: "punisher", parent: "", machine: "qsound", input: "punisher", init: "init_punisher", rotation: 0, maker: "Capcom", title: "The Punisher (World 930422)", not_working: false, romset: &RS_PUNISHER },
    GameInfo { name: "punisheru", parent: "punisher", machine: "qsound", input: "punisher", init: "init_punisher", rotation: 0, maker: "Capcom", title: "The Punisher (USA 930422)", not_working: false, romset: &RS_PUNISHERU },
    GameInfo { name: "punisherh", parent: "punisher", machine: "qsound", input: "punisher", init: "init_punisher", rotation: 0, maker: "Capcom", title: "The Punisher (Hispanic 930422)", not_working: false, romset: &RS_PUNISHERH },
    GameInfo { name: "punisherj", parent: "punisher", machine: "qsound", input: "punisher", init: "init_punisher", rotation: 0, maker: "Capcom", title: "The Punisher (Japan 930422)", not_working: false, romset: &RS_PUNISHERJ },
    GameInfo { name: "punisherbz", parent: "punisher", machine: "wofhfh", input: "punisherbz", init: "empty_init", rotation: 0, maker: "bootleg", title: "Biaofeng Zhanjing (Chinese bootleg of The Punisher)", not_working: false, romset: &RS_PUNISHERBZ },
    GameInfo { name: "slammast", parent: "", machine: "qsound", input: "slammast", init: "init_slammast", rotation: 0, maker: "Capcom", title: "Saturday Night Slam Masters (World 930713)", not_working: false, romset: &RS_SLAMMAST },
    GameInfo { name: "slammastu", parent: "slammast", machine: "qsound", input: "slammast", init: "init_slammast", rotation: 0, maker: "Capcom", title: "Saturday Night Slam Masters (USA 930713)", not_working: false, romset: &RS_SLAMMASTU },
    GameInfo { name: "mbomberj", parent: "slammast", machine: "qsound", input: "slammast", init: "init_slammast", rotation: 0, maker: "Capcom", title: "Muscle Bomber: The Body Explosion (Japan 930713)", not_working: false, romset: &RS_MBOMBERJ },
    GameInfo { name: "mbombrd", parent: "", machine: "qsound", input: "slammast", init: "init_slammast", rotation: 0, maker: "Capcom", title: "Muscle Bomber Duo: Ultimate Team Battle (World 931206)", not_working: false, romset: &RS_MBOMBRD },
    GameInfo { name: "mbombrdj", parent: "mbombrd", machine: "qsound", input: "slammast", init: "init_slammast", rotation: 0, maker: "Capcom", title: "Muscle Bomber Duo: Heat Up Warriors (Japan 931206)", not_working: false, romset: &RS_MBOMBRDJ },
    GameInfo { name: "gulunpa", parent: "", machine: "cps1_10MHz", input: "gulunpa", init: "empty_init", rotation: 0, maker: "Capcom", title: "Gulun.Pa! (Japan 931220 L) (prototype)", not_working: false, romset: &RS_GULUNPA },
    GameInfo { name: "pnickj", parent: "", machine: "cps1_12MHz", input: "pnickj", init: "empty_init", rotation: 0, maker: "Capcom", title: "Pnickies (Japan 940608)", not_working: false, romset: &RS_PNICKJ },
    GameInfo { name: "qtono2j", parent: "", machine: "cps1_12MHz", input: "qtono2j", init: "empty_init", rotation: 0, maker: "Capcom", title: "Quiz Tonosama no Yabou 2: Zenkoku-ban (Japan 950123)", not_working: false, romset: &RS_QTONO2J },
    GameInfo { name: "megaman", parent: "", machine: "cps1_12MHz", input: "megaman", init: "empty_init", rotation: 0, maker: "Capcom", title: "Mega Man: The Power Battle (CPS1, USA 951006)", not_working: false, romset: &RS_MEGAMAN },
    GameInfo { name: "megamana", parent: "megaman", machine: "cps1_12MHz", input: "megaman", init: "empty_init", rotation: 0, maker: "Capcom", title: "Mega Man: The Power Battle (CPS1, Asia 951006)", not_working: false, romset: &RS_MEGAMANA },
    GameInfo { name: "rockmanj", parent: "megaman", machine: "cps1_12MHz", input: "rockmanj", init: "empty_init", rotation: 0, maker: "Capcom", title: "Rockman: The Power Battle (CPS1, Japan 950922)", not_working: false, romset: &RS_ROCKMANJ },
    GameInfo { name: "pmonster", parent: "", machine: "ganbare", input: "pmonster", init: "init_ganbare", rotation: 0, maker: "Capcom", title: "Gamushara Battle! Puchi Monster (Japan 990519)", not_working: true, romset: &RS_PMONSTER },
    GameInfo { name: "ganbare", parent: "", machine: "ganbare", input: "ganbare", init: "init_ganbare", rotation: 0, maker: "Capcom", title: "Ganbare! Marine Kun (Japan 2K0411)", not_working: false, romset: &RS_GANBARE },
    GameInfo { name: "pokonyan", parent: "", machine: "cps1_10MHz", input: "pokonyan", init: "empty_init", rotation: 0, maker: "Capcom", title: "Pokonyan! Balloon (Japan 940322)", not_working: false, romset: &RS_POKONYAN },
    GameInfo { name: "mpumpkin", parent: "", machine: "cps1_10MHz", input: "mpumpkin", init: "empty_init", rotation: 0, maker: "Capcom", title: "Magical Pumpkin: Puroland de Daibouken (Japan 960712)", not_working: false, romset: &RS_MPUMPKIN },
    GameInfo { name: "pang3", parent: "", machine: "pang3", input: "pang3", init: "init_pang3", rotation: 0, maker: "Mitchell", title: "Pang! 3 (Europe 950601)", not_working: false, romset: &RS_PANG3 },
    GameInfo { name: "pang3r1", parent: "pang3", machine: "pang3", input: "pang3", init: "init_pang3", rotation: 0, maker: "Mitchell", title: "Pang! 3 (Europe 950511)", not_working: false, romset: &RS_PANG3R1 },
    GameInfo { name: "pang3j", parent: "pang3", machine: "pang3", input: "pang3", init: "init_pang3", rotation: 0, maker: "Mitchell", title: "Pang! 3: Kaitou Tachi no Karei na Gogo (Japan 950511)", not_working: false, romset: &RS_PANG3J },
    GameInfo { name: "pang3b", parent: "pang3", machine: "pang3", input: "pang3b", init: "init_pang3b", rotation: 0, maker: "bootleg", title: "Pang! 3 (bootleg, set 1)", not_working: false, romset: &RS_PANG3B },
    GameInfo { name: "pang3b2", parent: "pang3", machine: "pang3", input: "pang3", init: "init_pang3", rotation: 0, maker: "bootleg", title: "Pang! 3 (bootleg, set 2)", not_working: false, romset: &RS_PANG3B2 },
    GameInfo { name: "pang3b3", parent: "pang3", machine: "pang3", input: "pang3", init: "init_pang3", rotation: 0, maker: "bootleg", title: "Pang! 3 (bootleg, set 3)", not_working: false, romset: &RS_PANG3B3 },
    GameInfo { name: "pang3b4", parent: "pang3", machine: "cps1_12MHz", input: "pang3b4", init: "init_pang3b4", rotation: 0, maker: "bootleg", title: "Pang! 3 (bootleg, set 4)", not_working: false, romset: &RS_PANG3B4 },
    GameInfo { name: "pang3b5", parent: "pang3", machine: "pang3", input: "pang3", init: "init_pang3", rotation: 0, maker: "bootleg", title: "Pang! 3 (bootleg, set 5)", not_working: false, romset: &RS_PANG3B5 },
    GameInfo { name: "cps1mult", parent: "", machine: "cps1_12MHz", input: "sf2", init: "init_cps1mult", rotation: 0, maker: "bootleg", title: "CPS1 Multi Game", not_working: true, romset: &RS_CPS1MULT },
];
