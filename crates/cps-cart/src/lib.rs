//! cps-cart — voir docs/ARCHITECTURE.md
#![forbid(unsafe_code)]

pub mod config;
pub mod error;

#[cfg(test)]
mod smoke {
    #[test]
    fn it_builds() {}
}
