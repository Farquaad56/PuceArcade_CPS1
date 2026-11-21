//! Horloge du projet : 1 tick = 1/5_040_000_000 s (ADR-001).
//! PPCM entier de 10 MHz, 12 MHz, 16 MHz et 315/88 MHz.

pub type Tick = u64;

// spec: docs/spec/00 §Horloges du matériel
pub const TICKS_PER_SECOND: u64 = 5_040_000_000;

// spec: docs/spec/00 §Horloge du projet (table)
pub const TICKS_PER_68K_10MHZ: u64 = 504;
pub const TICKS_PER_68K_12MHZ: u64 = 420;

// spec: docs/spec/00 §Horloge du projet (table) — Z80 et YM2151 même quartz
pub const TICKS_PER_Z80: u64 = 1408;
pub const TICKS_PER_YM: u64 = 1408;

// spec: docs/spec/00 §Horloge du projet (table) — prescale YM2151
pub const YM_CLOCKS_PER_SAMPLE: u64 = 64;
pub const TICKS_PER_YM_SAMPLE: u64 = 90_112;

// spec: docs/spec/00 §Horloges du matériel — OKI MSM6295
pub const TICKS_PER_OKI: u64 = 5040;
pub const OKI_DIVISOR_PIN7_HIGH: u64 = 132;
pub const OKI_DIVISOR_PIN7_LOW: u64 = 165;

// spec: docs/spec/00 §Horloges du matériel — Pixel (CPS-A/CPS-B)
pub const TICKS_PER_PIXEL: u64 = 630;

// spec: docs/spec/00 §Trame (raw params MAME)
pub const PIXELS_PER_LINE: u64 = 512;
pub const LINES_PER_FRAME: u64 = 262;
pub const TICKS_PER_LINE: u64 = 322_560;
pub const TICKS_PER_FRAME: u64 = 84_510_720;

// spec: docs/spec/00 §Trame — VBLANK à la ligne 240
pub const VBLANK_START_LINE: u64 = 240;
pub const TICKS_VBLANK_START: u64 = 77_414_400;

// spec: docs/spec/00 §Trame — zone visible
pub const VISIBLE_X0: usize = 64;
pub const VISIBLE_Y0: usize = 16;
