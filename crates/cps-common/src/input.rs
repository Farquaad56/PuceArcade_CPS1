//! Entrées JAMMA actives-BAS (1 = relâché).
//! spec: docs/spec/01 §Entrées

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputState {
    pub in0: u8,
    pub in1: u16,
    pub in2: u8,
    pub in3: u8,
}

impl Default for InputState {
    fn default() -> Self {
        // spec: docs/spec/01 §Entrées — repos 0xFF / 0xFFFF
        InputState {
            in0: 0xFF,
            in1: 0xFFFF,
            in2: 0xFF,
            in3: 0xFF,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dips {
    pub dswa: u8,
    pub dswb: u8,
    pub dswc: u8,
}

impl Default for Dips {
    fn default() -> Self {
        // spec: docs/spec/01 §DIP switches par défaut — repos 0xFF
        Dips {
            dswa: 0xFF,
            dswb: 0xFF,
            dswc: 0xFF,
        }
    }
}

pub trait InputProvider {
    fn inputs(&self) -> InputState;
}

// spec: docs/spec/01 §Entrées — IN0 (8 bits)
pub mod in0 {
    pub const COIN1: u8 = 0x01;
    pub const COIN2: u8 = 0x02;
    pub const SERVICE1: u8 = 0x04;
    pub const START1: u8 = 0x10;
    pub const START2: u8 = 0x20;
    pub const TEST: u8 = 0x40;
}

// spec: docs/spec/01 §Entrées — IN1 (joueur, par octet)
pub mod pad {
    pub const RIGHT: u8 = 0x01;
    pub const LEFT: u8 = 0x02;
    pub const DOWN: u8 = 0x04;
    pub const UP: u8 = 0x08;
    pub const B1: u8 = 0x10;
    pub const B2: u8 = 0x20;
    pub const B3: u8 = 0x40;
}

// spec: docs/spec/01 §Entrées — IN2 (SF2 uniquement)
pub mod kick {
    pub const P1_B4: u8 = 0x01;
    pub const P1_B5: u8 = 0x02;
    pub const P1_B6: u8 = 0x04;
    pub const P2_B4: u8 = 0x10;
    pub const P2_B5: u8 = 0x20;
    pub const P2_B6: u8 = 0x40;
}
