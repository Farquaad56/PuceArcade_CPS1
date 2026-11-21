//! CRC-32/ISO-HDLC (polynôme réfléchi 0xEDB88320, init/xorout 0xFFFFFFFF).
//! Table de calcul dans une OnceLock<[u32; 256]>.

use std::sync::OnceLock;

fn make_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for (i, slot) in table.iter_mut().enumerate() {
        let mut crc = i as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
        *slot = crc;
    }
    table
}

fn get_table() -> &'static [u32; 256] {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    TABLE.get_or_init(make_table)
}

pub fn crc32_bytes(data: &[u8]) -> u32 {
    let table = get_table();
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[index];
    }
    crc ^ 0xFFFFFFFF
}
