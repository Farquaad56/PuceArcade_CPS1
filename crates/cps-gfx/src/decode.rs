//! Décodage des tuiles CPS1 (layouts 8x8, 8x8_2, 16x16, 32x32).
//! spec: docs/spec/02-video-cps-a-b.md §Décodage des tuiles

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    L8x8,
    L8x8b,
    L16x16,
    L32x32,
}

impl Layout {
    pub fn size(self) -> u32 {
        match self {
            Layout::L8x8 | Layout::L8x8b => 8,
            Layout::L16x16 => 16,
            Layout::L32x32 => 32,
        }
    }

    pub fn bytes_per_tile(self) -> usize {
        match self {
            Layout::L8x8 | Layout::L8x8b => 64,
            Layout::L16x16 => 128,
            Layout::L32x32 => 512,
        }
    }
}

/// Décode un pixel d'une tuile CPS1. Retourne le pen brut (0..15).
/// spec: docs/spec/02-video-cps-a-b.md §Décodage des tuiles
pub fn tile_pixel(gfx: &[u8], layout: Layout, code: u32, x: u32, y: u32) -> u8 {
    let bytes_per_tile = layout.bytes_per_tile();
    let elements = gfx.len() / bytes_per_tile;
    if elements == 0 {
        return 15;
    }

    let code = code % elements as u32;
    let size = layout.size();
    assert!(x < size && y < size, "pixel out of tile bounds");

    // Calcul de la base de ligne selon le layout
    let line_base: usize = match layout {
        Layout::L8x8 | Layout::L8x8b => (code * 64 + y * 8) as usize,
        Layout::L16x16 => (code * 128 + y * 8) as usize,
        Layout::L32x32 => (code * 512 + y * 16) as usize,
    };

    // Groupe de 4 octets selon le layout et la position x
    let group: usize = match layout {
        Layout::L8x8 => 0,
        Layout::L8x8b => 4,
        Layout::L16x16 => {
            if x < 8 {
                0
            } else {
                4
            }
        }
        Layout::L32x32 => (x / 8) as usize * 4,
    };

    let s = 7 - (x & 7);
    let g = line_base + group;

    // 4 plans : b[g+0]=bit0, b[g+1]=bit1, b[g+2]=bit2, b[g+3]=bit3
    ((gfx[g + 3] >> s) & 1) << 3
        | ((gfx[g + 2] >> s) & 1) << 2
        | ((gfx[g + 1] >> s) & 1) << 1
        | ((gfx[g] >> s) & 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l16x16_basic() {
        let mut gfx = vec![0u8; 1024]; // 8 tuiles 16x16
                                       // code 1 -> base 128
        gfx[131] = 0x80; // plan bit3, pixel (0,0)
        assert_eq!(tile_pixel(&gfx, Layout::L16x16, 1, 0, 0), 8);

        gfx[128] = 0x01; // plan bit0, pixel (7,0)
        assert_eq!(tile_pixel(&gfx, Layout::L16x16, 1, 7, 0), 1);

        gfx[134] = 0x80; // plan bit2, pixel (8,0) -> groupe x>=8
        assert_eq!(tile_pixel(&gfx, Layout::L16x16, 1, 8, 0), 4);

        gfx[128 + 8 * 15 + 7] = 0x01; // pixel (15,15)
        assert_eq!(tile_pixel(&gfx, Layout::L16x16, 1, 15, 15), 8);
    }

    #[test]
    fn l8x8_basic() {
        let mut gfx = vec![0u8; 1024];
        // L8x8 code 0 : octet +2 = plan d'offset 16 = bit 2
        gfx[2] = 0x40;
        assert_eq!(tile_pixel(&gfx, Layout::L8x8, 0, 1, 0), 4);

        // L8x8b code 0 : octet +6 (g=4) plan bit2
        gfx[6] = 0x40;
        assert_eq!(tile_pixel(&gfx, Layout::L8x8b, 0, 1, 0), 4);

        // L8x8b pixel (7,0) : octet g+1 avec g=4 -> gfx[5]
        gfx[5] = 0x01;
        assert_eq!(tile_pixel(&gfx, Layout::L8x8b, 0, 7, 0), 2);

        // L8x8 pixel (7,0) reste 0 (groupe différent)
        assert_eq!(tile_pixel(&gfx, Layout::L8x8, 0, 7, 0), 0);
    }

    #[test]
    fn l32x32_basic() {
        let mut gfx = vec![0u8; 1024];
        // L32x32 code 0 : pixel (31,0) -> groupe x/8=3, g=12
        gfx[12 + 1] = 0x01; // plan bit1
        assert_eq!(tile_pixel(&gfx, Layout::L32x32, 0, 31, 0), 2);

        // pixel (0,2) -> ligne y=2, base = 512 + 2*16 = 544
        gfx[16 * 2 + 0] = 0x80; // plan bit0
        assert_eq!(tile_pixel(&gfx, Layout::L32x32, 0, 0, 2), 1);
    }

    #[test]
    fn code_modulo() {
        let mut gfx = vec![0u8; 1024]; // 8 tuiles 16x16
        gfx[131] = 0x80; // code 1, pixel (0,0) pen 8

        // code 9 % 8 = 1 -> même pixels que code 1
        assert_eq!(tile_pixel(&gfx, Layout::L16x16, 9, 0, 0), 8);

        // gfx vide -> 15
        let empty: Vec<u8> = vec![];
        assert_eq!(tile_pixel(&empty, Layout::L16x16, 0, 0, 0), 15);
    }
}
