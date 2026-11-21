//! COMBINE_DATA de MAME : fusion d'octets pair/impair en mot.
//! spec: docs/spec/01 §Conventions — `COMBINE_DATA(&r, data, mask)`

pub fn combine_data(old: u16, data: u16, uds: bool, lds: bool) -> u16 {
    let mut mask = 0u16;
    if uds {
        mask |= 0xFF00;
    }
    if lds {
        mask |= 0x00FF;
    }
    (old & !mask) | (data & mask)
}
