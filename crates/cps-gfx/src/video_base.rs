//! Port de cps1_get_video_base (ref/mame-cps1/cps1_v.cpp l.2246-2366)
// license:BSD-3-Clause
// copyright-holders:Barry Rodowick, Nicola Salmoria, Aaron Giles
// source: ref/mame-cps1/cps1_v.cpp l.2246-2366

use crate::regs::{
    Video, OBJ_BASE, OBJ_SIZE, OTHER_BASE, OTHER_SIZE, SCROLL1_BASE, SCROLL1_X, SCROLL1_Y,
    SCROLL2_BASE, SCROLL2_X, SCROLL2_Y, SCROLL3_BASE, SCROLL3_X, SCROLL3_Y, SCROLL_SIZE, STARS1_X,
    STARS1_Y, STARS2_X, STARS2_Y, VIDEOCONTROL,
};

impl Video {
    /// Calcule les bases vidéo et l'activation des couches (l.2246-2366).
    pub fn get_video_base(&mut self) {
        // PORT-NOTE: MAME utilise des pointeurs ; ici on calcule les index de mots.
        let scroll1xoff = 0;
        let scroll2xoff = 0;
        let scroll3xoff = 0;

        // Recalculer les bases (marquer dirty dans MAME -> rien à faire ici)
        self.bases.scroll[0] = self.cps1_base(SCROLL1_BASE, SCROLL_SIZE);
        self.bases.scroll[1] = self.cps1_base(SCROLL2_BASE, SCROLL_SIZE);
        self.bases.scroll[2] = self.cps1_base(SCROLL3_BASE, SCROLL_SIZE);

        // Bootleg kludges (l.2267-2311)
        let kludge = self.cfg.bootleg_kludge & 15;
        if kludge == 0x01 {
            self.regs_a[OBJ_BASE] = 0x9100;
            // scroll offsets would be applied in MAME via pointer arithmetic
        } else if kludge == 2 {
            self.regs_a[OBJ_BASE] = 0x9100;
            // scroll offsets
        } else if kludge == 3 {
            // scroll offsets
        } else if self.cfg.bootleg_kludge == 0x88 {
            // 3wondersb specific overrides
            self.regs_b[0x30 / 2] = 0x3f;
            self.regs_a[VIDEOCONTROL] = 0x3e;
            self.regs_a[SCROLL2_BASE] = 0x90c0;
            self.regs_a[SCROLL3_BASE] = 0x9100;
            self.regs_a[5] = 0x9140; // PALETTE_BASE
        }

        // Bases obj et other (l.2313-2314)
        self.bases.obj = self.cps1_base(OBJ_BASE, OBJ_SIZE);
        self.bases.other = self.cps1_base(OTHER_BASE, OTHER_SIZE);

        // Valeurs de défilement (l.2317-2326)
        self.bases.scrollx[0] = self.regs_a[SCROLL1_X] as i32 + scroll1xoff;
        self.bases.scrolly[0] = self.regs_a[SCROLL1_Y] as i32;
        self.bases.scrollx[1] = self.regs_a[SCROLL2_X] as i32 + scroll2xoff;
        self.bases.scrolly[1] = self.regs_a[SCROLL2_Y] as i32;
        self.bases.scrollx[2] = self.regs_a[SCROLL3_X] as i32 + scroll3xoff;
        self.bases.scrolly[2] = self.regs_a[SCROLL3_Y] as i32;
        self.bases.starsx[0] = self.regs_a[STARS1_X] as i32;
        self.bases.starsy[0] = self.regs_a[STARS1_Y] as i32;
        self.bases.starsx[1] = self.regs_a[STARS2_X] as i32;
        self.bases.starsy[1] = self.regs_a[STARS2_Y] as i32;

        // Activation des couches (l.2328-2335)
        let layercontrol = self.regs_b[(self.cfg.cpsb.layer_control / 2) as usize];
        let videocontrol = self.regs_a[VIDEOCONTROL];
        let masks = &self.cfg.cpsb.layer_enable_mask;

        self.bases.layer_enabled[0] = (layercontrol & (masks[0] as u16)) != 0;
        self.bases.layer_enabled[1] =
            (layercontrol & (masks[1] as u16)) != 0 && ((videocontrol >> 2) & 1) == 1;
        self.bases.layer_enabled[2] =
            (layercontrol & (masks[2] as u16)) != 0 && ((videocontrol >> 3) & 1) == 1;
        self.bases.stars_enabled[0] = (layercontrol & (masks[3] as u16)) != 0;
        self.bases.stars_enabled[1] = (layercontrol & (masks[4] as u16)) != 0;
    }
}
