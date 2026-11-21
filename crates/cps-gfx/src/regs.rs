//! Registres vidéo CPS-A et état global (spec: docs/spec/02 §Registres CPS-A)
// license:BSD-3-Clause
// copyright-holders:Barry Rodowick, Nicola Salmoria, Aaron Giles
// source: ref/mame-cps1/cps1.h l.176-193

use crate::config::{find_config, GameConfig};

/// Index de registre CPS-A : OBJ_BASE (base RAM sprites / 256)
pub const OBJ_BASE: usize = 0;
/// Index de registre CPS-A : SCROLL1_BASE (8×8)
pub const SCROLL1_BASE: usize = 1;
/// Index de registre CPS-A : SCROLL2_BASE (16×16)
pub const SCROLL2_BASE: usize = 2;
/// Index de registre CPS-A : SCROLL3_BASE (32×32)
pub const SCROLL3_BASE: usize = 3;
/// Index de registre CPS-A : OTHER_BASE (table de rowscroll)
pub const OTHER_BASE: usize = 4;
/// Index de registre CPS-A : PALETTE_BASE (déclenche copie palette à l'écriture)
pub const PALETTE_BASE: usize = 5;
/// Index de registre CPS-A : SCROLL1 X
pub const SCROLL1_X: usize = 6;
/// Index de registre CPS-A : SCROLL1 Y
pub const SCROLL1_Y: usize = 7;
/// Index de registre CPS-A : SCROLL2 X
pub const SCROLL2_X: usize = 8;
/// Index de registre CPS-A : SCROLL2 Y
pub const SCROLL2_Y: usize = 9;
/// Index de registre CPS-A : SCROLL3 X
pub const SCROLL3_X: usize = 10;
/// Index de registre CPS-A : SCROLL3 Y
pub const SCROLL3_Y: usize = 11;
/// Index de registre CPS-A : STARS1 X
pub const STARS1_X: usize = 12;
/// Index de registre CPS-A : STARS1 Y
pub const STARS1_Y: usize = 13;
/// Index de registre CPS-A : SCROLL2 X (stars)
pub const STARS2_X: usize = 14;
/// Index de registre CPS-A : STARS2 Y
pub const STARS2_Y: usize = 15;
/// Index de registre CPS-A : ROWSCROLL_OFFS
pub const ROWSCROLL_OFFS: usize = 16;
/// Index de registre CPS-A : VIDEOCONTROL (bit0 rowscroll, bit2 scroll2, bit3 scroll3, bit15 flip)
pub const VIDEOCONTROL: usize = 17;

/// Taille de la RAM de défilement (scroll) en mots
pub const SCROLL_SIZE: u32 = 0x4000;
/// Taille de la RAM sprites (obj) en mots
pub const OBJ_SIZE: u32 = 0x800;
/// Taille de la RAM "other" (rowscroll table) en mots
pub const OTHER_SIZE: u32 = 0x800;
/// Alignement minimum de la palette (une page = 512 couleurs)
pub const PALETTE_ALIGN: u32 = 0x400;
/// Taille totale de la GFXRAM en mots (0x3FFFF / 2 + 1)
pub const GFXRAM_WORDS: usize = 0x18000;

/// Bases vidéo calculées par get_video_base()
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VideoBases {
    pub scroll: [usize; 3],
    pub obj: usize,
    pub other: usize,
    pub scrollx: [i32; 3],
    pub scrolly: [i32; 3],
    pub starsx: [i32; 2],
    pub starsy: [i32; 2],
    pub layer_enabled: [bool; 3],
    pub stars_enabled: [bool; 2],
}

/// État vidéo complet CPS1 (regs, GFXRAM, palette, bases)
#[derive(Clone, Debug)]
pub struct Video {
    /// Configuration du jeu courant (reliée via relink())
    // NOTE: serde(skip) will be added in T408 when serialization is implemented
    pub cfg: &'static GameConfig,
    /// Nom du jeu (pour relink())
    pub game: String,
    /// Registres CPS-A (32 mots)
    pub regs_a: [u16; 32],
    /// Registres CPS-B (32 mots)
    pub regs_b: [u16; 32],
    /// GFXRAM (0x18000 mots)
    pub gfxram: Vec<u16>,
    /// Palette (0xC00 entrées, u32 = 0x00RRGGBB)
    pub palette: Vec<u32>,
    /// Tampon sprites latché au VBLANK (0x400 mots)
    pub buffered_obj: Vec<u16>,
    /// Bases vidéo calculées
    pub bases: VideoBases,
    /// Entrée IN2 (bits 8-15 = 0)
    pub in2: u16,
    /// Entrée IN3 (bits 8-15 = 0)
    pub in3: u16,
    /// Offset du dernier sprite trouvé (-4 si aucun)
    pub last_sprite: i32,
    /// Masque de débogage des couches (bit0 sprites, bit1-3 scroll1-3, bit4 étoiles)
    pub debug_layer_mask: u32,
}

impl Video {
    /// Crée un état vidéo pour le jeu donné et initialise les valeurs par défaut.
    pub fn new(game: &str) -> Self {
        let cfg = find_config(game);
        let mut v = Self {
            cfg,
            game: game.to_string(),
            regs_a: [0; 32],
            regs_b: [0; 32],
            gfxram: vec![0; GFXRAM_WORDS],
            palette: vec![0; 0xC00],
            buffered_obj: vec![0; 0x400],
            bases: VideoBases::default(),
            in2: 0,
            in3: 0,
            last_sprite: -4,
            debug_layer_mask: 0xFFFF_FFFF,
        };
        v.reset();
        v
    }

    /// Relie la configuration au nom du jeu (après chargement d'un save).
    pub fn relink(&mut self) {
        self.cfg = find_config(&self.game);
    }

    /// Réinitialise l'état vidéo aux valeurs par défaut (video_start, l.2533-2596).
    pub fn reset(&mut self) {
        // Clear GFXRAM and registers
        for i in 0..GFXRAM_WORDS {
            self.gfxram[i] = 0;
        }
        for i in 0..32 {
            self.regs_a[i] = 0;
            self.regs_b[i] = 0;
        }

        // Default base addresses (video_start l.2564-2568)
        self.regs_a[OBJ_BASE] = 0x9200;
        self.regs_a[SCROLL1_BASE] = 0x9000;
        self.regs_a[SCROLL2_BASE] = 0x9040;
        self.regs_a[SCROLL3_BASE] = 0x9080;
        self.regs_a[OTHER_BASE] = 0x9100;

        // Palette noire (0xC00 entrées)
        for i in 0..0xC00 {
            self.palette[i] = 0;
        }

        // Tampon sprites à zéro
        for i in 0..0x400 {
            self.buffered_obj[i] = 0;
        }

        // Entrées par défaut (repos)
        self.in2 = 0x00FF;
        self.in3 = 0x00FF;

        // Calculer les bases vidéo initiales
        self.get_video_base();
    }

    /// Calcule la base GFXRAM pour un registre CPS-A donné (cps1_base, l.2099-2111).
    /// Retourne l'index de MOT dans gfxram.
    pub fn cps1_base(&self, reg: usize, boundary: u32) -> usize {
        let base = self.regs_a[reg] as i32 * 256;
        // Align to boundary (mask out lower bits)
        let aligned = base & !(boundary as i32 - 1);
        // Word index in GFXRAM (modulo 0x18000)
        ((aligned & 0x3FFFF) / 2) as usize % GFXRAM_WORDS
    }

    /// Écriture GFXRAM avec COMBINE_DATA (l.2368-2415).
    pub fn gfxram_write(&mut self, word: usize, data: u16, uds: bool, lds: bool) {
        if uds && lds {
            self.gfxram[word] = data;
        } else if uds {
            let old = self.gfxram[word];
            // Write upper byte of data to high byte position
            let upper_byte = data >> 8;
            self.gfxram[word] = (old & 0x00FF) | (upper_byte << 8);
        } else if lds {
            let old = self.gfxram[word];
            // Write lower byte of data to low byte position
            let lower_byte = data & 0xFF;
            self.gfxram[word] = (old & 0xFF00) | lower_byte;
        }
    }

    /// Définit les entrées IN2/IN3 (bits 8-15 toujours à zéro).
    pub fn set_inputs(&mut self, in2: u8, in3: u8) {
        self.in2 = in2 as u16;
        self.in3 = in3 as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_new_ghouls() {
        let v = Video::new("ghouls");
        assert_eq!(v.regs_a[OBJ_BASE], 0x9200);
        assert_eq!(v.regs_a[SCROLL1_BASE], 0x9000);
    }

    #[test]
    fn test_cps1_base_scroll() {
        let v = Video::new("ghouls");
        // SCROLL1_BASE=0x9000 -> base=0x9000*256=0x900000, aligned to 0x4000 -> 0x900000
        // (0x900000 & 0x3FFFF) / 2 = 0
        assert_eq!(v.cps1_base(SCROLL1_BASE, SCROLL_SIZE), 0);
    }

    #[test]
    fn test_cps1_base_obj() {
        let v = Video::new("ghouls");
        // OBJ_BASE=0x9200 -> base=0x9200*256=0x920000, aligned to 0x800 -> 0x920000
        // (0x920000 & 0x3FFFF) / 2 = 0x10000
        assert_eq!(v.cps1_base(OBJ_BASE, OBJ_SIZE), 0x10000);
    }

    #[test]
    fn test_cps1_base_other() {
        let v = Video::new("ghouls");
        // OTHER_BASE=0x9100 -> base=0x9100*256=0x910000, aligned to 0x800 -> 0x910000
        // (0x910000 & 0x3FFFF) / 2 = 0x8000
        assert_eq!(v.cps1_base(OTHER_BASE, OTHER_SIZE), 0x8000);
    }

    #[test]
    fn test_cps1_base_scroll_alignment() {
        let mut v = Video::new("ghouls");
        // Test alignment: set SCROLL1_BASE to non-aligned value
        v.regs_a[SCROLL1_BASE] = 0x9012;
        // base=0x9012*256=0x901200, aligned to 0x4000 -> 0x900000
        assert_eq!(v.cps1_base(SCROLL1_BASE, SCROLL_SIZE), 0);
    }

    #[test]
    fn test_gfxram_write_combine() {
        let mut v = Video::new("ghouls");
        // Write upper byte first (uds=true)
        v.gfxram_write(5, 0xAB12, true, false);
        // Then lower byte (lds=true)
        v.gfxram_write(5, 0x00CD, false, true);
        assert_eq!(v.gfxram[5], 0xABCD);
    }

    #[test]
    fn test_set_inputs() {
        let mut v = Video::new("ghouls");
        v.set_inputs(0xFE, 0x7F);
        assert_eq!(v.in2, 0x00FE);
        assert_eq!(v.in3, 0x007F);
    }

    #[test]
    fn test_get_video_base_ghouls() {
        let mut v = Video::new("ghouls");
        // ghouls: layer_control=0x26 -> regs_b[0x13], masks={0x02,0x04,0x08,0x30,0x30}
        v.regs_b[0x13] = 0x000E; // Enable layers 1,2,3 (bits 1,2,3)
        v.regs_a[VIDEOCONTROL] = 0x000E; // Enable scroll2 (bit2), scroll3 (bit3)
        v.get_video_base();

        assert!(v.bases.layer_enabled[0]);
        assert!(v.bases.layer_enabled[1]);
        assert!(v.bases.layer_enabled[2]);
    }

    #[test]
    fn test_get_video_base_scroll_disabled() {
        let mut v = Video::new("ghouls");
        v.regs_b[0x13] = 0x000E;
        v.regs_a[VIDEOCONTROL] = 0; // Disable scroll2 and scroll3
        v.get_video_base();

        assert!(v.bases.layer_enabled[0]);
        assert!(!v.bases.layer_enabled[1]);
        assert!(!v.bases.layer_enabled[2]);
    }

    #[test]
    fn test_scroll_values() {
        let mut v = Video::new("ghouls");
        v.regs_a[SCROLL2_X] = 0x0123;
        v.get_video_base();
        assert_eq!(v.bases.scrollx[1], 0x123);
    }

    #[test]
    fn test_obj_base() {
        let v = Video::new("ghouls");
        assert_eq!(v.bases.obj, 0x10000);
    }
}
