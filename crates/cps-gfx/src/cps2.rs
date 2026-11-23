// license:BSD-3-Clause
// copyright-holders:Paul Leaman, Andreas Naive, Nicola Salmoria,Charles MacDonald
//! État vidéo propre au CPS2 : deux RAM objets commutées par une banque, registres « output »,
//! tampon d'objets latché au VBLANK et recherche du dernier sprite. spec: docs/spec/05 §Vidéo
//! source: ref/mame-cps2/cps2.cpp l.756-763 (registres output), l.799-869 (video_start, objram),
//!         l.872-894 (find_last_sprite), l.1105-1112 (cps2_objram_latch)

/// m_cps2_obj_size / 2 : 0x2000 octets = 0x1000 mots.
pub const OBJ_WORDS: usize = 0x1000;

/// CPS2_OBJ_PRI / 2 : mot des priorités de couches.
pub const OBJ_PRI: usize = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cps2Video {
    pub objram1: Vec<u16>,
    pub objram2: Vec<u16>,
    pub objram_bank: u16,
    /// Registres output (400000-40000B) : OBJ_BASE, UNK1, OBJ_PRI, UNK2, OBJ_XOFFS, OBJ_YOFFS.
    pub output: [u16; 6],
    pub buffered_obj: Vec<u16>,
    pub last_sprite: i32,
    pub pri_ctrl: u16,
}

impl Default for Cps2Video {
    /// video_start (l.799-813) : RAM objets et tampon à 0.
    fn default() -> Self {
        Cps2Video {
            objram1: vec![0; OBJ_WORDS],
            objram2: vec![0; OBJ_WORDS],
            objram_bank: 0,
            output: [0; 6],
            buffered_obj: vec![0; OBJ_WORDS],
            last_sprite: 0,
            pri_ctrl: 0,
        }
    }
}

/// COMBINE_DATA : uds = octet fort, lds = octet faible.
fn combine(old: &mut u16, data: u16, uds: bool, lds: bool) {
    let mask = if uds { 0xFF00 } else { 0 } | if lds { 0x00FF } else { 0 };
    *old = (*old & !mask) | (data & mask);
}

impl Cps2Video {
    /// cps2_objram_bank_w (l.817-821) : ACCESSING_BITS_0_7 = lds.
    pub fn objram_bank_w(&mut self, data: u16, lds: bool) {
        if lds {
            self.objram_bank = data & 1;
        }
    }

    /// cps2_objram1_r (l.823-829).
    pub fn objram1_r(&self, offset: usize) -> u16 {
        let o = offset % OBJ_WORDS;
        if self.objram_bank & 1 != 0 {
            self.objram2[o]
        } else {
            self.objram1[o]
        }
    }

    /// cps2_objram2_r (l.831-837).
    pub fn objram2_r(&self, offset: usize) -> u16 {
        let o = offset % OBJ_WORDS;
        if self.objram_bank & 1 != 0 {
            self.objram1[o]
        } else {
            self.objram2[o]
        }
    }

    /// cps2_objram1_w (l.839-845).
    pub fn objram1_w(&mut self, offset: usize, data: u16, uds: bool, lds: bool) {
        let o = offset % OBJ_WORDS;
        if self.objram_bank & 1 != 0 {
            combine(&mut self.objram2[o], data, uds, lds);
        } else {
            combine(&mut self.objram1[o], data, uds, lds);
        }
    }

    /// cps2_objram2_w (l.847-853).
    pub fn objram2_w(&mut self, offset: usize, data: u16, uds: bool, lds: bool) {
        let o = offset % OBJ_WORDS;
        if self.objram_bank & 1 != 0 {
            combine(&mut self.objram1[o], data, uds, lds);
        } else {
            combine(&mut self.objram2[o], data, uds, lds);
        }
    }

    /// cps2_objbase (l.855-869) : banque 0 -> objram1, banque 1 -> objram2.
    pub fn objbase(&self) -> &[u16] {
        let mut baseptr = 0x7000;
        if self.objram_bank & 1 != 0 {
            baseptr ^= 0x0080;
        }
        if baseptr == 0x7000 {
            &self.objram1
        } else {
            &self.objram2
        }
    }

    /// Lecture d'un registre output (RAM, 6 mots).
    pub fn output_r(&self, word: usize) -> u16 {
        self.output[word % 6]
    }

    /// Écriture d'un registre output (COMBINE_DATA).
    pub fn output_w(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        combine(&mut self.output[word % 6], data, uds, lds);
    }

    /// cps2_objram_latch (l.1105-1112), front montant du VBLANK.
    pub fn objram_latch(&mut self) {
        self.pri_ctrl = self.output[OBJ_PRI];
        let base = self.objbase().to_vec();
        self.buffered_obj.copy_from_slice(&base);
    }

    /// find_last_sprite (l.872-894), partie CPS2 : marque de fin de table.
    pub fn find_last_sprite(&mut self) {
        let base = &self.buffered_obj;
        let mut offset = 0;
        while offset < OBJ_WORDS {
            if base[offset + 1] >= 0x8000 || base[offset + 3] >= 0xff00 {
                // Marker found. This is the last sprite.
                self.last_sprite = offset as i32 - 4;
                return;
            }
            offset += 4;
        }
        // Sprites must use full sprite RAM
        self.last_sprite = OBJ_WORDS as i32 - 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_switching() {
        let mut v = Cps2Video::default();
        v.objram2_w(3, 0xABCD, true, true);
        assert_eq!((v.objram2[3], v.objram1[3]), (0xABCD, 0));
        v.objram_bank_w(1, true);
        v.objram2_w(3, 0x1234, true, true);
        assert_eq!(v.objram1[3], 0x1234);
        assert_eq!(v.objram2_r(3), 0x1234);
        assert_eq!(v.objram1_r(3), 0xABCD);
        v.objram_bank_w(0, false);
        assert_eq!(v.objram_bank, 1, "octet faible non écrit");
    }

    #[test]
    fn combine_bytes() {
        let mut v = Cps2Video::default();
        v.objram1_w(0x1005, 0xAABB, true, false);
        v.objram1_w(5, 0x11CC, false, true);
        assert_eq!(v.objram1[5], 0xAACC);
    }

    #[test]
    fn objbase_and_latch() {
        let mut v = Cps2Video::default();
        v.objram1[0] = 1;
        v.objram2[0] = 2;
        assert_eq!(v.objbase()[0], 1);
        v.objram_bank_w(1, true);
        assert_eq!(v.objbase()[0], 2);
        v.output_w(2, 0x1234, true, true);
        v.objram_latch();
        assert_eq!(v.pri_ctrl, 0x1234);
        assert_eq!(v.buffered_obj, v.objbase());
    }

    #[test]
    fn last_sprite() {
        let mut v = Cps2Video::default();
        v.find_last_sprite();
        assert_eq!(v.last_sprite, 0xFFC);
        v.buffered_obj[9] = 0x8000;
        v.find_last_sprite();
        assert_eq!(v.last_sprite, 4);
        v.buffered_obj[3] = 0xFF00;
        v.find_last_sprite();
        assert_eq!(v.last_sprite, -4);
    }
}
