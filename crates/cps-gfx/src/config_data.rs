// Fichier GÉNÉRÉ par tools/gen_cps_tables.py depuis ref/mame-cps1/cps1_v.cpp — NE PAS MODIFIER À LA MAIN.
// license:BSD-3-Clause (données MAME, copyright-holders: Paul Leaman)
//! Configurations CPS-B, mappers de banques ROM graphiques et table des jeux (spec: docs/spec/02 §Config).
#![allow(clippy::unreadable_literal)]

use crate::config::{CpsBConfig, GameConfig, GfxRange, Mapper};

pub const CPS_B_01: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: 0, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 38, priority: [40, 42, 44, 46], palette_control: 48, layer_enable_mask: [2, 4, 8, 48, 48] };
pub const CPS_B_02: CpsBConfig = CpsBConfig { cpsb_addr: 32, cpsb_value: 2, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 44, priority: [42, 40, 38, 36], palette_control: 34, layer_enable_mask: [2, 4, 8, 0, 0] };
pub const CPS_B_03: CpsBConfig = CpsBConfig { cpsb_addr: 36, cpsb_value: 3, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 48, priority: [46, 44, 42, 40], palette_control: 38, layer_enable_mask: [32, 16, 8, 0, 0] };
pub const CPS_B_04: CpsBConfig = CpsBConfig { cpsb_addr: 32, cpsb_value: 4, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 46, priority: [38, 48, 40, 50], palette_control: 42, layer_enable_mask: [2, 4, 8, 0, 0] };
pub const CPS_B_05: CpsBConfig = CpsBConfig { cpsb_addr: 32, cpsb_value: 5, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 40, priority: [42, 44, 46, 48], palette_control: 50, layer_enable_mask: [2, 8, 32, 20, 20] };
pub const CPS_B_11: CpsBConfig = CpsBConfig { cpsb_addr: 50, cpsb_value: 1025, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 38, priority: [40, 42, 44, 46], palette_control: 48, layer_enable_mask: [8, 16, 32, 0, 0] };
pub const CPS_B_12: CpsBConfig = CpsBConfig { cpsb_addr: 32, cpsb_value: 1026, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 44, priority: [42, 40, 38, 36], palette_control: 34, layer_enable_mask: [2, 4, 8, 0, 0] };
pub const CPS_B_13: CpsBConfig = CpsBConfig { cpsb_addr: 46, cpsb_value: 1027, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 34, priority: [36, 38, 40, 42], palette_control: 44, layer_enable_mask: [32, 2, 4, 0, 0] };
pub const CPS_B_14: CpsBConfig = CpsBConfig { cpsb_addr: 30, cpsb_value: 1028, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 18, priority: [20, 22, 24, 26], palette_control: 28, layer_enable_mask: [8, 32, 16, 0, 0] };
pub const CPS_B_15: CpsBConfig = CpsBConfig { cpsb_addr: 14, cpsb_value: 1029, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 2, priority: [4, 6, 8, 10], palette_control: 12, layer_enable_mask: [4, 2, 32, 0, 0] };
pub const CPS_B_16: CpsBConfig = CpsBConfig { cpsb_addr: 0, cpsb_value: 1030, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 12, priority: [10, 8, 6, 4], palette_control: 2, layer_enable_mask: [16, 10, 10, 0, 0] };
pub const CPS_B_17: CpsBConfig = CpsBConfig { cpsb_addr: 8, cpsb_value: 1031, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 20, priority: [18, 16, 14, 12], palette_control: 10, layer_enable_mask: [8, 20, 2, 0, 0] };
pub const CPS_B_18: CpsBConfig = CpsBConfig { cpsb_addr: 16, cpsb_value: 1032, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 28, priority: [26, 24, 22, 20], palette_control: 18, layer_enable_mask: [16, 8, 2, 0, 0] };
pub const CPS_B_21_DEF: CpsBConfig = CpsBConfig { cpsb_addr: 50, cpsb_value: -1, mult_factor1: 0, mult_factor2: 2, mult_result_lo: 4, mult_result_hi: 6, unknown1: 8, unknown2: -1, unknown3: -1, layer_control: 38, priority: [40, 42, 44, 46], palette_control: 48, layer_enable_mask: [2, 4, 8, 48, 48] };
pub const CPS_B_21_BT1: CpsBConfig = CpsBConfig { cpsb_addr: 50, cpsb_value: 2048, mult_factor1: 14, mult_factor2: 12, mult_result_lo: 10, mult_result_hi: 8, unknown1: 6, unknown2: 4, unknown3: 2, layer_control: 40, priority: [38, 36, 34, 32], palette_control: 48, layer_enable_mask: [32, 4, 8, 18, 18] };
pub const CPS_B_21_BT2: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: 30, mult_factor2: 28, mult_result_lo: 26, mult_result_hi: 24, unknown1: -1, unknown2: 12, unknown3: 10, layer_control: 32, priority: [46, 44, 42, 40], palette_control: 48, layer_enable_mask: [48, 8, 48, 0, 0] };
pub const CPS_B_21_BT3: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: 6, mult_factor2: 4, mult_result_lo: 2, mult_result_hi: 0, unknown1: 14, unknown2: 12, unknown3: 10, layer_control: 32, priority: [46, 44, 42, 40], palette_control: 48, layer_enable_mask: [32, 18, 18, 0, 0] };
pub const CPS_B_21_BT4: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: 6, mult_factor2: 4, mult_result_lo: 2, mult_result_hi: 0, unknown1: 30, unknown2: 28, unknown3: 26, layer_control: 40, priority: [38, 36, 34, 32], palette_control: 48, layer_enable_mask: [32, 16, 2, 0, 0] };
pub const CPS_B_21_BT5: CpsBConfig = CpsBConfig { cpsb_addr: 50, cpsb_value: -1, mult_factor1: 14, mult_factor2: 12, mult_result_lo: 10, mult_result_hi: 8, unknown1: 30, unknown2: 28, unknown3: 26, layer_control: 32, priority: [46, 44, 42, 40], palette_control: 48, layer_enable_mask: [32, 4, 2, 0, 0] };
pub const CPS_B_21_BT6: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 32, priority: [46, 44, 42, 40], palette_control: 48, layer_enable_mask: [32, 20, 20, 0, 0] };
pub const CPS_B_21_BT7: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 44, priority: [-1, -1, -1, -1], palette_control: 18, layer_enable_mask: [20, 2, 20, 0, 0] };
pub const CPS_B_21_QS1: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 34, priority: [36, 38, 40, 42], palette_control: 44, layer_enable_mask: [16, 8, 4, 0, 0] };
pub const CPS_B_21_QS2: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: 46, unknown3: 32, layer_control: 10, priority: [12, 14, 0, 2], palette_control: 4, layer_enable_mask: [22, 22, 22, 0, 0] };
pub const CPS_B_21_QS3: CpsBConfig = CpsBConfig { cpsb_addr: 14, cpsb_value: 3072, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: 44, unknown2: -1, unknown3: -1, layer_control: 18, priority: [20, 22, 8, 10], palette_control: 12, layer_enable_mask: [4, 2, 32, 0, 0] };
pub const CPS_B_21_QS4: CpsBConfig = CpsBConfig { cpsb_addr: 46, cpsb_value: 3073, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: 28, unknown2: 30, unknown3: 8, layer_control: 22, priority: [0, 2, 40, 42], palette_control: 44, layer_enable_mask: [4, 8, 16, 0, 0] };
pub const CPS_B_21_QS5: CpsBConfig = CpsBConfig { cpsb_addr: 30, cpsb_value: 3074, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: 12, unknown2: -1, unknown3: -1, layer_control: 42, priority: [44, 46, 48, 50], palette_control: 28, layer_enable_mask: [4, 8, 16, 0, 0] };
pub const HACK_B_1: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 20, priority: [18, 16, 14, 12], palette_control: 10, layer_enable_mask: [14, 14, 14, 48, 48] };
pub const HACK_B_2: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: 14, mult_factor2: 12, mult_result_lo: 10, mult_result_hi: 8, unknown1: 6, unknown2: 4, unknown3: 2, layer_control: 40, priority: [38, 36, 34, 32], palette_control: 34, layer_enable_mask: [32, 4, 8, 18, 18] };
pub const HACK_B_3: CpsBConfig = CpsBConfig { cpsb_addr: 32, cpsb_value: 4, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 48, priority: [38, -1, 40, 50], palette_control: 42, layer_enable_mask: [2, 4, 8, 0, 0] };
pub const HACK_B_4: CpsBConfig = CpsBConfig { cpsb_addr: -1, cpsb_value: -1, mult_factor1: -1, mult_factor2: -1, mult_result_lo: -1, mult_result_hi: -1, unknown1: -1, unknown2: -1, unknown3: -1, layer_control: 40, priority: [38, 36, 34, 32], palette_control: 48, layer_enable_mask: [64, 16, 2, 0, 0] };

pub const MAPPER_LWCHR: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 2, start: 0x0, end: 0x1ffff, bank: 0 }, GfxRange { kind: 16, start: 0x0, end: 0x1ffff, bank: 1 }, GfxRange { kind: 4, start: 0x0, end: 0x1ffff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x1ffff, bank: 1 }] };
pub const MAPPER_LW621: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 2, start: 0x0, end: 0x1ffff, bank: 0 }, GfxRange { kind: 16, start: 0x0, end: 0x1ffff, bank: 1 }, GfxRange { kind: 4, start: 0x0, end: 0x1ffff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x1ffff, bank: 1 }] };
pub const MAPPER_DM620: Mapper = Mapper { bank_sizes: [0x8000, 0x2000, 0x2000, 0x0], ranges: &[GfxRange { kind: 8, start: 0x8000, end: 0xbfff, bank: 1 }, GfxRange { kind: 1, start: 0x2000, end: 0x3fff, bank: 2 }, GfxRange { kind: 31, start: 0x0, end: 0x1ffff, bank: 0 }] };
pub const MAPPER_DM22A: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x2000, 0x2000], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x1fff, bank: 0 }, GfxRange { kind: 2, start: 0x2000, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x1ffff, bank: 2 }, GfxRange { kind: 1, start: 0x2000, end: 0x3fff, bank: 3 }] };
pub const MAPPER_DAM63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x1fff, bank: 0 }, GfxRange { kind: 2, start: 0x2000, end: 0x2fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 0 }, GfxRange { kind: 8, start: 0x0, end: 0x1ffff, bank: 1 }, GfxRange { kind: 1, start: 0x2000, end: 0x3fff, bank: 1 }] };
pub const MAPPER_ST24M1: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 16, start: 0x0, end: 0x3ff, bank: 0 }, GfxRange { kind: 1, start: 0x0, end: 0x4fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 0 }, GfxRange { kind: 8, start: 0x0, end: 0x7fff, bank: 1 }, GfxRange { kind: 2, start: 0x7000, end: 0x7fff, bank: 1 }] };
pub const MAPPER_ST22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x4000, 0x4000], ranges: &[GfxRange { kind: 16, start: 0x0, end: 0x1ffff, bank: 0 }, GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x4000, end: 0x4fff, bank: 1 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x3fff, bank: 2 }, GfxRange { kind: 8, start: 0x4000, end: 0x7fff, bank: 3 }, GfxRange { kind: 2, start: 0x7000, end: 0x7fff, bank: 3 }] };
pub const MAPPER_TK22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x4000, 0x4000], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x4000, end: 0x5fff, bank: 1 }, GfxRange { kind: 2, start: 0x6000, end: 0x7fff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x3fff, bank: 2 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 3 }] };
pub const MAPPER_TK24B1: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x5fff, bank: 0 }, GfxRange { kind: 2, start: 0x6000, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x3fff, bank: 1 }] };
pub const MAPPER_WL24B: Mapper = Mapper { bank_sizes: [0x8000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x4fff, bank: 0 }, GfxRange { kind: 8, start: 0x5000, end: 0x6fff, bank: 0 }, GfxRange { kind: 2, start: 0x7000, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x3fff, bank: 1 }] };
pub const MAPPER_WL22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x4000, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x4000, end: 0x4fff, bank: 1 }, GfxRange { kind: 8, start: 0x5000, end: 0x6fff, bank: 1 }, GfxRange { kind: 2, start: 0x7000, end: 0x7fff, bank: 1 }, GfxRange { kind: 4, start: 0x0, end: 0x3fff, bank: 2 }] };
pub const MAPPER_S224B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x43ff, bank: 0 }, GfxRange { kind: 2, start: 0x4400, end: 0x4bff, bank: 0 }, GfxRange { kind: 8, start: 0x4c00, end: 0x5fff, bank: 0 }, GfxRange { kind: 4, start: 0x6000, end: 0x7fff, bank: 0 }] };
pub const MAPPER_S222B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x4000, end: 0x43ff, bank: 1 }, GfxRange { kind: 2, start: 0x4400, end: 0x4bff, bank: 1 }, GfxRange { kind: 8, start: 0x4c00, end: 0x5fff, bank: 1 }, GfxRange { kind: 4, start: 0x6000, end: 0x7fff, bank: 1 }] };
pub const MAPPER_YI24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x1fff, bank: 0 }, GfxRange { kind: 8, start: 0x2000, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x4000, end: 0x47ff, bank: 0 }, GfxRange { kind: 4, start: 0x4800, end: 0x7fff, bank: 0 }] };
pub const MAPPER_YI22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x1fff, bank: 0 }, GfxRange { kind: 8, start: 0x2000, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x4000, end: 0x47ff, bank: 1 }, GfxRange { kind: 4, start: 0x4800, end: 0x7fff, bank: 1 }] };
pub const MAPPER_AR24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 2, start: 0x3000, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x5fff, bank: 0 }, GfxRange { kind: 8, start: 0x6000, end: 0x7fff, bank: 0 }] };
pub const MAPPER_AR22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 2, start: 0x3000, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x5fff, bank: 1 }, GfxRange { kind: 8, start: 0x6000, end: 0x7fff, bank: 1 }] };
pub const MAPPER_ARA63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }] };
pub const MAPPER_O224B: Mapper = Mapper { bank_sizes: [0x8000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 2, start: 0x0, end: 0xbff, bank: 0 }, GfxRange { kind: 4, start: 0xc00, end: 0x3bff, bank: 0 }, GfxRange { kind: 8, start: 0x3c00, end: 0x4bff, bank: 0 }, GfxRange { kind: 1, start: 0x4c00, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0xa7ff, bank: 1 }, GfxRange { kind: 4, start: 0xa800, end: 0xb7ff, bank: 1 }, GfxRange { kind: 8, start: 0xb800, end: 0xbfff, bank: 1 }] };
pub const MAPPER_MS24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x4000, end: 0x4fff, bank: 0 }, GfxRange { kind: 4, start: 0x5000, end: 0x6fff, bank: 0 }, GfxRange { kind: 8, start: 0x7000, end: 0x7fff, bank: 0 }] };
pub const MAPPER_MS22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x4000, end: 0x4fff, bank: 1 }, GfxRange { kind: 4, start: 0x5000, end: 0x6fff, bank: 1 }, GfxRange { kind: 8, start: 0x7000, end: 0x7fff, bank: 1 }] };
pub const MAPPER_CK24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 2, start: 0x3000, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x6fff, bank: 0 }, GfxRange { kind: 8, start: 0x7000, end: 0x7fff, bank: 0 }] };
pub const MAPPER_CK22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 2, start: 0x3000, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x6fff, bank: 1 }, GfxRange { kind: 8, start: 0x7000, end: 0x7fff, bank: 1 }] };
pub const MAPPER_NM24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x4000, end: 0x47ff, bank: 0 }, GfxRange { kind: 1, start: 0x4800, end: 0x67ff, bank: 0 }, GfxRange { kind: 4, start: 0x4800, end: 0x67ff, bank: 0 }, GfxRange { kind: 8, start: 0x6800, end: 0x7fff, bank: 0 }] };
pub const MAPPER_CA24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 8, start: 0x3000, end: 0x4fff, bank: 0 }, GfxRange { kind: 2, start: 0x5000, end: 0x57ff, bank: 0 }, GfxRange { kind: 1, start: 0x5800, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x5800, end: 0x7fff, bank: 0 }] };
pub const MAPPER_CA22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 8, start: 0x3000, end: 0x3fff, bank: 0 }, GfxRange { kind: 8, start: 0x4000, end: 0x4fff, bank: 1 }, GfxRange { kind: 2, start: 0x5000, end: 0x57ff, bank: 1 }, GfxRange { kind: 1, start: 0x5800, end: 0x7fff, bank: 1 }, GfxRange { kind: 4, start: 0x5800, end: 0x7fff, bank: 1 }] };
pub const MAPPER_STF29: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 1, start: 0x10000, end: 0x11fff, bank: 2 }, GfxRange { kind: 8, start: 0x2000, end: 0x3fff, bank: 2 }, GfxRange { kind: 2, start: 0x4000, end: 0x4fff, bank: 2 }, GfxRange { kind: 4, start: 0x5000, end: 0x7fff, bank: 2 }] };
pub const MAPPER_RT24B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x53ff, bank: 0 }, GfxRange { kind: 2, start: 0x5400, end: 0x6fff, bank: 0 }, GfxRange { kind: 8, start: 0x7000, end: 0x7fff, bank: 0 }, GfxRange { kind: 8, start: 0x0, end: 0x3fff, bank: 1 }, GfxRange { kind: 4, start: 0x2800, end: 0x7fff, bank: 1 }, GfxRange { kind: 1, start: 0x5400, end: 0x7fff, bank: 1 }] };
pub const MAPPER_RT22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x4000, 0x4000], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x4000, end: 0x53ff, bank: 1 }, GfxRange { kind: 2, start: 0x5400, end: 0x6fff, bank: 1 }, GfxRange { kind: 8, start: 0x7000, end: 0x7fff, bank: 1 }, GfxRange { kind: 8, start: 0x0, end: 0x3fff, bank: 2 }, GfxRange { kind: 4, start: 0x2800, end: 0x3fff, bank: 2 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 3 }, GfxRange { kind: 1, start: 0x5400, end: 0x7fff, bank: 3 }] };
pub const MAPPER_KD29B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0x8fff, bank: 1 }, GfxRange { kind: 4, start: 0x9000, end: 0xbfff, bank: 1 }, GfxRange { kind: 2, start: 0xc000, end: 0xd7ff, bank: 1 }, GfxRange { kind: 8, start: 0xd800, end: 0xffff, bank: 1 }] };
pub const MAPPER_KD22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x4000, 0x4000], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x4000, end: 0x7fff, bank: 1 }, GfxRange { kind: 1, start: 0x8000, end: 0x8fff, bank: 2 }, GfxRange { kind: 4, start: 0x9000, end: 0xbfff, bank: 2 }, GfxRange { kind: 2, start: 0xc000, end: 0xd7ff, bank: 3 }, GfxRange { kind: 8, start: 0xd800, end: 0xffff, bank: 3 }] };
pub const MAPPER_CC63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 2, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 4, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 8, start: 0x8000, end: 0xffff, bank: 1 }] };
pub const MAPPER_KR63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 2, start: 0x8000, end: 0x9fff, bank: 1 }, GfxRange { kind: 1, start: 0x8000, end: 0xcfff, bank: 1 }, GfxRange { kind: 4, start: 0x8000, end: 0xcfff, bank: 1 }, GfxRange { kind: 8, start: 0xd000, end: 0xffff, bank: 1 }] };
pub const MAPPER_KR22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x4000, 0x4000], ranges: &[GfxRange { kind: 5, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 5, start: 0x4000, end: 0x7fff, bank: 1 }, GfxRange { kind: 5, start: 0x8000, end: 0x87ff, bank: 2 }, GfxRange { kind: 2, start: 0x8800, end: 0x97ff, bank: 2 }, GfxRange { kind: 5, start: 0x9800, end: 0xbfff, bank: 2 }, GfxRange { kind: 5, start: 0xc000, end: 0xcfff, bank: 3 }, GfxRange { kind: 8, start: 0xd000, end: 0xffff, bank: 3 }] };
pub const MAPPER_S9263B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 1, start: 0x10000, end: 0x11fff, bank: 2 }, GfxRange { kind: 8, start: 0x2000, end: 0x3fff, bank: 2 }, GfxRange { kind: 2, start: 0x4000, end: 0x4fff, bank: 2 }, GfxRange { kind: 4, start: 0x5000, end: 0x7fff, bank: 2 }] };
pub const MAPPER_VA22B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 15, start: 0x4000, end: 0x7fff, bank: 1 }] };
pub const MAPPER_VA63B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }] };
pub const MAPPER_VA24B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }] };
pub const MAPPER_Q522B: Mapper = Mapper { bank_sizes: [0x4000, 0x4000, 0x0, 0x0], ranges: &[GfxRange { kind: 5, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 5, start: 0x4000, end: 0x6fff, bank: 1 }, GfxRange { kind: 8, start: 0x7000, end: 0x77ff, bank: 1 }, GfxRange { kind: 2, start: 0x7800, end: 0x7fff, bank: 1 }] };
pub const MAPPER_TK263B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }] };
pub const MAPPER_CD63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }] };
pub const MAPPER_PS63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }] };
pub const MAPPER_MB63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 15, start: 0x10000, end: 0x17fff, bank: 2 }] };
pub const MAPPER_QD22B: Mapper = Mapper { bank_sizes: [0x4000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 8, start: 0x0, end: 0x3fff, bank: 0 }] };
pub const MAPPER_QAD63B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }] };
pub const MAPPER_TN2292: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x0], ranges: &[GfxRange { kind: 31, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 31, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 31, start: 0x10000, end: 0x17fff, bank: 2 }] };
pub const MAPPER_RCM63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x8000], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 15, start: 0x10000, end: 0x17fff, bank: 2 }, GfxRange { kind: 15, start: 0x18000, end: 0x1ffff, bank: 3 }] };
pub const MAPPER_SFZ63B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x8000], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 15, start: 0x10000, end: 0x17fff, bank: 2 }, GfxRange { kind: 15, start: 0x18000, end: 0x1ffff, bank: 3 }] };
pub const MAPPER_GBPR2: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x3fff, bank: 0 }, GfxRange { kind: 2, start: 0x4000, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x8000, end: 0xbfff, bank: 1 }, GfxRange { kind: 8, start: 0xc000, end: 0xffff, bank: 1 }] };
pub const MAPPER_GULUNPA: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 2, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 8, start: 0x4000, end: 0x5fff, bank: 0 }, GfxRange { kind: 4, start: 0x2000, end: 0x3fff, bank: 0 }, GfxRange { kind: 1, start: 0x6000, end: 0x7fff, bank: 0 }] };
pub const MAPPER_PKB10B: Mapper = Mapper { bank_sizes: [0x8000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 2, start: 0x0, end: 0xfff, bank: 0 }, GfxRange { kind: 5, start: 0x1000, end: 0x5fff, bank: 0 }, GfxRange { kind: 8, start: 0x6000, end: 0x7fff, bank: 0 }] };
pub const MAPPER_CP1B1F: Mapper = Mapper { bank_sizes: [0x10000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0xffff, bank: 0 }] };
pub const MAPPER_CP1B1F_BOOT: Mapper = Mapper { bank_sizes: [0x10000, 0x10000, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 15, start: 0x10000, end: 0x17fff, bank: 0 }, GfxRange { kind: 15, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 15, start: 0x18000, end: 0x1ffff, bank: 1 }] };
pub const MAPPER_VARTHB2: Mapper = Mapper { bank_sizes: [0x20000, 0x0, 0x0, 0x0], ranges: &[GfxRange { kind: 15, start: 0x0, end: 0x1ffff, bank: 0 }] };
pub const MAPPER_CPS2: Mapper = Mapper { bank_sizes: [0x20000, 0x20000, 0x0, 0x0], ranges: &[GfxRange { kind: 14, start: 0x0, end: 0x1ffff, bank: 1 }] };
pub const MAPPER_KNM10B: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 1, start: 0x10000, end: 0x10fff, bank: 2 }, GfxRange { kind: 4, start: 0x4000, end: 0x7fff, bank: 2 }, GfxRange { kind: 2, start: 0x1000, end: 0x1fff, bank: 2 }, GfxRange { kind: 8, start: 0x2000, end: 0x3fff, bank: 2 }] };
pub const MAPPER_POKONYAN: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x8000, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x2fff, bank: 0 }, GfxRange { kind: 2, start: 0x7000, end: 0x7fff, bank: 0 }, GfxRange { kind: 8, start: 0x3000, end: 0x3fff, bank: 0 }, GfxRange { kind: 4, start: 0x4000, end: 0x6fff, bank: 0 }] };
pub const MAPPER_PANG3B4: Mapper = Mapper { bank_sizes: [0x8000, 0x8000, 0x0, 0x0], ranges: &[GfxRange { kind: 1, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 4, start: 0x0, end: 0x7fff, bank: 0 }, GfxRange { kind: 1, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 2, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 4, start: 0x8000, end: 0xffff, bank: 1 }, GfxRange { kind: 8, start: 0x8000, end: 0xffff, bank: 1 }] };

/// Table des jeux, dans l'ordre de cps1_config_table (la recherche prend la première entrée de ce nom).
pub static GAMES: &[GameConfig] = &[
    GameConfig { name: "forgottn", cpsb: CPS_B_01, mapper: MAPPER_LW621, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottna", cpsb: CPS_B_01, mapper: MAPPER_LW621, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottnu", cpsb: CPS_B_01, mapper: MAPPER_LW621, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottnue", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottnuc", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottnua", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottnuaa", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "forgottnj", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "lostwrld", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "lostwrldo", cpsb: CPS_B_01, mapper: MAPPER_LWCHR, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ghouls", cpsb: CPS_B_01, mapper: MAPPER_DM620, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ghoulsu", cpsb: CPS_B_01, mapper: MAPPER_DM620, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "daimakai", cpsb: CPS_B_01, mapper: MAPPER_DM22A, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "daimakair", cpsb: CPS_B_21_DEF, mapper: MAPPER_DAM63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "strider", cpsb: CPS_B_01, mapper: MAPPER_ST24M1, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "striderua", cpsb: CPS_B_01, mapper: MAPPER_ST24M1, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "strideruc", cpsb: CPS_B_17, mapper: MAPPER_ST24M1, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "striderj", cpsb: CPS_B_01, mapper: MAPPER_ST22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "striderjr", cpsb: CPS_B_21_DEF, mapper: MAPPER_ST24M1, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dynwar", cpsb: CPS_B_02, mapper: MAPPER_TK24B1, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dynwara", cpsb: CPS_B_02, mapper: MAPPER_TK22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dynwarj", cpsb: CPS_B_02, mapper: MAPPER_TK22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dynwarjr", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "willow", cpsb: CPS_B_03, mapper: MAPPER_WL24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "willowu", cpsb: CPS_B_03, mapper: MAPPER_WL24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "willowuo", cpsb: CPS_B_03, mapper: MAPPER_WL24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "willowj", cpsb: CPS_B_03, mapper: MAPPER_WL22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffight", cpsb: CPS_B_04, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffighta", cpsb: CPS_B_04, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightu", cpsb: CPS_B_04, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightu1", cpsb: CPS_B_04, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightua", cpsb: CPS_B_01, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightub", cpsb: CPS_B_03, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightuc", cpsb: CPS_B_05, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightj", cpsb: CPS_B_04, mapper: MAPPER_S222B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightj1", cpsb: CPS_B_01, mapper: MAPPER_S222B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightj2", cpsb: CPS_B_02, mapper: MAPPER_S222B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightj3", cpsb: CPS_B_03, mapper: MAPPER_S222B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightj4", cpsb: CPS_B_05, mapper: MAPPER_S222B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightjh", cpsb: CPS_B_01, mapper: MAPPER_S224B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ffightae", cpsb: CPS_B_21_DEF, mapper: MAPPER_S224B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "1941", cpsb: CPS_B_05, mapper: MAPPER_YI24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "1941r1", cpsb: CPS_B_05, mapper: MAPPER_YI24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "1941u", cpsb: CPS_B_05, mapper: MAPPER_YI24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "1941j", cpsb: CPS_B_05, mapper: MAPPER_YI22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "unsquad", cpsb: CPS_B_11, mapper: MAPPER_AR24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "area88", cpsb: CPS_B_11, mapper: MAPPER_AR22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "area88r", cpsb: CPS_B_21_DEF, mapper: MAPPER_ARA63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mercs", cpsb: CPS_B_12, mapper: MAPPER_O224B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "mercsu", cpsb: CPS_B_12, mapper: MAPPER_O224B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "mercsur1", cpsb: CPS_B_12, mapper: MAPPER_O224B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "mercsj", cpsb: CPS_B_12, mapper: MAPPER_O224B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "msword", cpsb: CPS_B_13, mapper: MAPPER_MS24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mswordr1", cpsb: CPS_B_13, mapper: MAPPER_MS24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mswordu", cpsb: CPS_B_13, mapper: MAPPER_MS24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mswordj", cpsb: CPS_B_13, mapper: MAPPER_MS22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mtwins", cpsb: CPS_B_14, mapper: MAPPER_CK24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "chikij", cpsb: CPS_B_14, mapper: MAPPER_CK22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "nemo", cpsb: CPS_B_15, mapper: MAPPER_NM24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "nemor1", cpsb: CPS_B_15, mapper: MAPPER_NM24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "nemoj", cpsb: CPS_B_15, mapper: MAPPER_NM24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "nemoja", cpsb: CPS_B_15, mapper: MAPPER_NM24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawing", cpsb: CPS_B_16, mapper: MAPPER_CA24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawingr1", cpsb: CPS_B_16, mapper: MAPPER_CA24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawingu", cpsb: CPS_B_05, mapper: MAPPER_CA22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawingur1", cpsb: CPS_B_16, mapper: MAPPER_CA24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawingj", cpsb: CPS_B_16, mapper: MAPPER_CA22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawingjr", cpsb: CPS_B_21_DEF, mapper: MAPPER_ARA63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cawingbl", cpsb: CPS_B_16, mapper: MAPPER_CA22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2", cpsb: CPS_B_11, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ea", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2eb", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ed", cpsb: CPS_B_05, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ee", cpsb: CPS_B_18, mapper: MAPPER_STF29, in2_addr: 60, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ef", cpsb: CPS_B_15, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2em", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2en", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ebbl", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ebbl2", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ebbl3", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2stt", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2rk", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ua", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ub", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2uc", cpsb: CPS_B_12, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ud", cpsb: CPS_B_05, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ue", cpsb: CPS_B_18, mapper: MAPPER_STF29, in2_addr: 60, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2uf", cpsb: CPS_B_15, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ug", cpsb: CPS_B_11, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2uh", cpsb: CPS_B_13, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ui", cpsb: CPS_B_14, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2uk", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2um", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2j", cpsb: CPS_B_13, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2j17", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ja", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2jc", cpsb: CPS_B_12, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2jf", cpsb: CPS_B_15, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2jh", cpsb: CPS_B_13, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2jl", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2qp1", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2qp2", cpsb: CPS_B_14, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2thndr", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2thndr2", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mpumpkin", cpsb: CPS_B_21_DEF, mapper: MAPPER_SFZ63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "3wonders", cpsb: CPS_B_21_BT1, mapper: MAPPER_RT24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "3wondersr1", cpsb: CPS_B_21_BT1, mapper: MAPPER_RT24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "3wondersu", cpsb: CPS_B_21_BT1, mapper: MAPPER_RT24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wonder3", cpsb: CPS_B_21_BT1, mapper: MAPPER_RT22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "3wondersb", cpsb: CPS_B_21_BT1, mapper: MAPPER_RT24B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 136 },
    GameConfig { name: "3wondersbi", cpsb: CPS_B_21_BT1, mapper: MAPPER_RT24B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 136 },
    GameConfig { name: "3wondersh", cpsb: HACK_B_2, mapper: MAPPER_RT24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "kod", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD29B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "kodr1", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD29B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "kodr2", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD29B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "kodu", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD29B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "kodj", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD29B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "kodja", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD22B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "kodb", cpsb: CPS_B_21_BT2, mapper: MAPPER_KD29B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "captcomm", cpsb: CPS_B_21_BT3, mapper: MAPPER_CC63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "captcommr1", cpsb: CPS_B_21_BT3, mapper: MAPPER_CC63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "captcommu", cpsb: CPS_B_21_BT3, mapper: MAPPER_CC63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "captcommj", cpsb: CPS_B_21_BT3, mapper: MAPPER_CC63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "captcommjr1", cpsb: CPS_B_21_BT3, mapper: MAPPER_CC63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "captcommb", cpsb: CPS_B_21_BT3, mapper: MAPPER_CC63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 67 },
    GameConfig { name: "captcommb2", cpsb: CPS_B_21_BT4, mapper: MAPPER_CC63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "knights", cpsb: CPS_B_21_BT4, mapper: MAPPER_KR63B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "knightsu", cpsb: CPS_B_21_BT4, mapper: MAPPER_KR63B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "knightsj", cpsb: CPS_B_21_BT4, mapper: MAPPER_KR63B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "knightsja", cpsb: CPS_B_21_BT4, mapper: MAPPER_KR22B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "knightsb2", cpsb: HACK_B_4, mapper: MAPPER_KR63B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 64 },
    GameConfig { name: "knightsb3", cpsb: CPS_B_21_BT4, mapper: MAPPER_KR63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pokonyan", cpsb: CPS_B_21_DEF, mapper: MAPPER_POKONYAN, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ce", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceea", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceec", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceua", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceub", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceuc", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2cet", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceja", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2cejb", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2cejc", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2bhh", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2rb", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2rb2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2rb3", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2red", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2reda", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2redp2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2v004", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2acc", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2ceblp", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2cebltw", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2acca", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2accp2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2amf", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2amf2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2amf3", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2dkot2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2level", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2m1", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2m2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2m3", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2m4", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2m5", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2m6", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2m7", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2m8", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2m9", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2m10", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2dongb", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2yyc", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2koryu", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2mdt", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2mdta", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2mdtb", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceb", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceb2", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceb3", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceb4", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceb5", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2b", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2b2", cpsb: CPS_B_17, mapper: MAPPER_STF29, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceupl", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2rules", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "sf2ceds6", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2cems6a", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2cems6b", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2cems6c", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2re", cpsb: HACK_B_1, mapper: MAPPER_S9263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 66 },
    GameConfig { name: "sf2mkot", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 65 },
    GameConfig { name: "varth", cpsb: CPS_B_04, mapper: MAPPER_VA24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "varthb", cpsb: CPS_B_04, mapper: MAPPER_VA63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 15 },
    GameConfig { name: "varthb2", cpsb: HACK_B_3, mapper: MAPPER_VARTHB2, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 193 },
    GameConfig { name: "varthb3", cpsb: CPS_B_04, mapper: MAPPER_VA63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 15 },
    GameConfig { name: "varthr1", cpsb: CPS_B_04, mapper: MAPPER_VA24B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "varthu", cpsb: CPS_B_04, mapper: MAPPER_VA63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "varthj", cpsb: CPS_B_21_BT5, mapper: MAPPER_VA22B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "varthjr", cpsb: CPS_B_21_BT5, mapper: MAPPER_VA63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cworld2j", cpsb: CPS_B_21_BT6, mapper: MAPPER_Q522B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "cworld2ja", cpsb: CPS_B_21_DEF, mapper: MAPPER_Q522B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cworld2jb", cpsb: CPS_B_21_BT6, mapper: MAPPER_Q522B, in2_addr: 54, in3_addr: 0, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "wof", cpsb: CPS_B_21_QS1, mapper: MAPPER_TK263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofr1", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofa", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofu", cpsb: CPS_B_21_QS1, mapper: MAPPER_TK263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofj", cpsb: CPS_B_21_QS1, mapper: MAPPER_TK263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofhfh", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofpic", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofr1bl", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dino", cpsb: CPS_B_21_QS2, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dinou", cpsb: CPS_B_21_QS2, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dinoj", cpsb: CPS_B_21_QS2, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dinoa", cpsb: CPS_B_21_QS2, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dinopic", cpsb: CPS_B_21_QS2, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dinopic2", cpsb: CPS_B_21_QS2, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "dinohunt", cpsb: CPS_B_21_DEF, mapper: MAPPER_CD63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punisher", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punisheru", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punisherh", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punisherj", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punipic", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punipic2", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punipic3", cpsb: CPS_B_21_QS3, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "punisherbz", cpsb: CPS_B_21_DEF, mapper: MAPPER_PS63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "slammast", cpsb: CPS_B_21_QS4, mapper: MAPPER_MB63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "slammastu", cpsb: CPS_B_21_QS4, mapper: MAPPER_MB63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "slampic", cpsb: CPS_B_21_QS4, mapper: MAPPER_MB63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "slampic2", cpsb: CPS_B_21_QS4, mapper: MAPPER_VARTHB2, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mbomberj", cpsb: CPS_B_21_QS4, mapper: MAPPER_MB63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mbombrd", cpsb: CPS_B_21_QS5, mapper: MAPPER_MB63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "mbombrdj", cpsb: CPS_B_21_QS5, mapper: MAPPER_MB63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2hf", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2hfu", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sf2hfj", cpsb: CPS_B_21_DEF, mapper: MAPPER_S9263B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "qad", cpsb: CPS_B_21_BT7, mapper: MAPPER_QD22B, in2_addr: 54, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "qadjr", cpsb: CPS_B_21_DEF, mapper: MAPPER_QAD63B, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "qtono2j", cpsb: CPS_B_21_DEF, mapper: MAPPER_TN2292, in2_addr: 54, in3_addr: 56, out2_addr: 52, bootleg_kludge: 0 },
    GameConfig { name: "megaman", cpsb: CPS_B_21_DEF, mapper: MAPPER_RCM63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "megamana", cpsb: CPS_B_21_DEF, mapper: MAPPER_RCM63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "rockmanj", cpsb: CPS_B_21_DEF, mapper: MAPPER_RCM63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pnickj", cpsb: CPS_B_21_DEF, mapper: MAPPER_PKB10B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3r1", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3j", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3b", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3b2", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3b3", cpsb: CPS_B_17, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3b4", cpsb: CPS_B_21_DEF, mapper: MAPPER_PANG3B4, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pang3b5", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F_BOOT, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "ganbare", cpsb: CPS_B_21_DEF, mapper: MAPPER_GBPR2, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "pmonster", cpsb: CPS_B_21_DEF, mapper: MAPPER_GBPR2, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "gulunpa", cpsb: CPS_B_21_DEF, mapper: MAPPER_GULUNPA, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sfach", cpsb: CPS_B_21_DEF, mapper: MAPPER_SFZ63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sfzbch", cpsb: CPS_B_21_DEF, mapper: MAPPER_SFZ63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "sfzch", cpsb: CPS_B_21_DEF, mapper: MAPPER_SFZ63B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "wofch", cpsb: CPS_B_21_DEF, mapper: MAPPER_TK263B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cps2", cpsb: CPS_B_21_DEF, mapper: MAPPER_CPS2, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "kenseim", cpsb: CPS_B_21_DEF, mapper: MAPPER_KNM10B, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
    GameConfig { name: "cps1mult", cpsb: CPS_B_21_DEF, mapper: MAPPER_CP1B1F, in2_addr: 0, in3_addr: 0, out2_addr: 0, bootleg_kludge: 0 },
];
