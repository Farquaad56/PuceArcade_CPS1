//! NEC µPD4701A : compteur incrémental 2 axes (molettes de Forgotten Worlds).
// license:BSD-3-Clause
// copyright-holders:smf,AJR
// source: ref/mame-machine/upd4701.cpp l.1-407 (sous-ensemble utilisé par forgottn)

/// MASK_COUNTER (l.12)
const MASK_COUNTER: i16 = 0xfff;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Upd4701 {
    cs: bool,
    xy: bool,
    ul: bool,
    resetx: bool,
    resety: bool,
    latchx: i16,
    latchy: i16,
    startx: i16,
    starty: i16,
    x: i16,
    y: i16,
    switches: u8,
    latchswitches: u8,
    cf: bool,
}

impl Default for Upd4701 {
    fn default() -> Self {
        Self::new()
    }
}

impl Upd4701 {
    /// Constructeur (l.14-39).
    pub fn new() -> Self {
        Upd4701 {
            cs: true,
            xy: false,
            ul: false,
            resetx: false,
            resety: false,
            latchx: 0,
            latchy: 0,
            startx: 0,
            starty: 0,
            x: 0,
            y: 0,
            switches: 0,
            latchswitches: 0,
            cf: true,
        }
    }

    /// ul_w (l.75)
    fn ul_w(&mut self, state: bool) {
        self.ul = state;
    }

    /// xy_w (l.84)
    fn xy_w(&mut self, state: bool) {
        self.xy = state;
    }

    /// cs_w (l.93) — PORT-NOTE: m_cf_cb non câblé sur CPS1 (callback omis).
    fn cs_w(&mut self, state: bool) {
        if self.cs != state {
            self.cs = state;
            if !self.cs {
                self.latchx = self.x.wrapping_sub(self.startx) & MASK_COUNTER;
                self.latchy = self.y.wrapping_sub(self.starty) & MASK_COUNTER;
                self.latchswitches = self.switches;
                if self.switches != 0 {
                    self.latchswitches |= 8;
                }
                if !self.cf {
                    self.cf = true;
                }
            }
        }
    }

    /// resetx_w (l.122)
    fn resetx_w(&mut self, state: bool) {
        if self.resetx != state {
            self.resetx = state;
            if self.resetx {
                self.startx = self.x;
            }
        }
    }

    /// resety_w (l.137)
    fn resety_w(&mut self, state: bool) {
        if self.resety != state {
            self.resety = state;
            if self.resety {
                self.starty = self.y;
            }
        }
    }

    /// reset_x_w (l.162)
    pub fn reset_x_w(&mut self) {
        self.resetx_w(true);
        self.resetx_w(false);
    }

    /// reset_y_w (l.182)
    pub fn reset_y_w(&mut self) {
        self.resety_w(true);
        self.resety_w(false);
    }

    /// x_add (l.249)
    pub fn x_add(&mut self, data: i16) {
        if !self.resetx && data != 0 {
            self.x = self.x.wrapping_add(data);
            if self.cs && self.cf {
                self.cf = false;
            }
        }
    }

    /// y_add (l.267)
    pub fn y_add(&mut self, data: i16) {
        if !self.resety && data != 0 {
            self.y = self.y.wrapping_add(data);
            if self.cs && self.cf {
                self.cf = false;
            }
        }
    }

    /// d_r (l.338) — PORT-NOTE: lecture CS inactif = bus ouvert (0) ; impossible via read_xy.
    fn d_r(&self) -> u8 {
        if self.cs {
            return 0;
        }
        let mut data = (if self.xy { self.latchy } else { self.latchx }) as u16;
        data |= u16::from(self.latchswitches) << 12;
        if self.ul {
            (data >> 8) as u8
        } else {
            (data & 0xff) as u8
        }
    }

    /// read_x (l.359)
    pub fn read_x(&mut self, offset: u32) -> u8 {
        self.read_xy(offset & 1)
    }

    /// read_y (l.368)
    pub fn read_y(&mut self, offset: u32) -> u8 {
        self.read_xy((offset & 1) | 2)
    }

    /// read_xy (l.377)
    pub fn read_xy(&mut self, offset: u32) -> u8 {
        let old_cs = self.cs;
        self.cs_w(false);
        self.xy_w(offset & 2 != 0);
        self.ul_w(offset & 1 != 0);
        let result = self.d_r();
        self.cs_w(old_cs);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x(u: &mut Upd4701) -> u16 {
        u16::from(u.read_x(1)) << 8 | u16::from(u.read_x(0))
    }
    fn y(u: &mut Upd4701) -> u16 {
        u16::from(u.read_y(1)) << 8 | u16::from(u.read_y(0))
    }

    #[test]
    fn card_values() {
        let mut u = Upd4701::new();
        u.x_add(5);
        u.y_add(-3);
        assert_eq!(x(&mut u), 5);
        assert_eq!(y(&mut u), 0xFFD);
        u.reset_x_w();
        assert_eq!(x(&mut u), 0);
        assert_eq!(y(&mut u), 0xFFD);
        u.x_add(-1);
        assert_eq!(x(&mut u), 0xFFF);
        u.reset_y_w();
        assert_eq!(y(&mut u), 0);
    }
}
