//! Framebuffer 384×224 (0x00RRGGBB), ligne par ligne.
//! spec: docs/spec/00 §Trame — zone visible 384×224

use std::io::Write;

use crate::crc::crc32_bytes;

pub const SCREEN_W: usize = 384;
pub const SCREEN_H: usize = 224;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Framebuffer {
    pub pixels: Vec<u32>,
}

impl Framebuffer {
    /// Crée un framebuffer vide (tous les pixels à 0).
    pub fn new() -> Self {
        let size = SCREEN_W * SCREEN_H;
        Framebuffer {
            pixels: vec![0u32; size],
        }
    }

    /// CRC-32 sur les pixels sérialisés en u32 little-endian.
    pub fn crc32(&self) -> u32 {
        let mut bytes = Vec::with_capacity(self.pixels.len() * 4);
        for &p in &self.pixels {
            bytes.extend_from_slice(&p.to_le_bytes());
        }
        crc32_bytes(&bytes)
    }

    /// Écrit le framebuffer en fichier PPM binaire (P6).
    pub fn write_ppm(&self, path: &std::path::Path) -> std::io::Result<()> {
        let mut f = std::fs::File::create(path)?;
        // En-tête : "P6\n384 224\n255\n" (15 octets)
        write!(f, "P6\n{} {}\n255\n", SCREEN_W, SCREEN_H)?;

        for &p in &self.pixels {
            // R = bits 16-23, G = bits 8-15, B = bits 0-7
            let r = ((p >> 16) & 0xFF) as u8;
            let g = ((p >> 8) & 0xFF) as u8;
            let b = (p & 0xFF) as u8;
            f.write_all(&[r, g, b])?;
        }

        Ok(())
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::new()
    }
}
