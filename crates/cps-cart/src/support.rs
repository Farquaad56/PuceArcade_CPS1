//! Jeux jouables : machine et initialisation émulées (T602).
//! spec: docs/spec/04-romsets-games.md §Jeux cibles

use crate::romset::GameInfo;
use crate::romset::{system_of, System};
use crate::romsets2_data::GAMES2;
use crate::romsets_data::GAMES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    Ok,
    /// Machine non émulée (QSound, pang3, bootlegs à matériel modifié…).
    UnsupportedMachine,
    /// Fonction d'initialisation MAME non émulée (décryptage, patchs…).
    UnsupportedInit,
    /// Marqué MACHINE_NOT_WORKING par MAME.
    NotWorking,
}

const MACHINES: [&str; 5] = ["cps1_10MHz", "cps1_12MHz", "forgottn", "qsound", "wofhfh"];
const INITS: [&str; 10] = [
    "empty_init",
    "init_dinohunt",
    "init_sf2hack",
    "init_sf2rk",
    "init_sf2thndr",
    "init_sf2ee",
    "init_wof",
    "init_dino",
    "init_punisher",
    "init_slammast",
];

/// CPS2 : machines et inits pris en charge (spec: docs/spec/05 §Système et jeux pris en charge).
const MACHINES2: [&str; 2] = ["cps2", "dead_cps2"];
const INITS2: [&str; 2] = ["init_cps2", "init_singbrd"];

pub fn support_of(info: &GameInfo) -> Support {
    let (machines, inits): (&[&str], &[&str]) = match system_of(info) {
        System::Cps1 => (&MACHINES, &INITS),
        System::Cps2 => (&MACHINES2, &INITS2),
    };
    if !machines.contains(&info.machine) {
        Support::UnsupportedMachine
    } else if !inits.contains(&info.init) {
        Support::UnsupportedInit
    } else if info.not_working {
        Support::NotWorking
    } else {
        Support::Ok
    }
}

/// Jeux jouables, dans l'ordre de la table GAMES.
pub fn supported_games() -> Vec<&'static GameInfo> {
    GAMES
        .iter()
        .filter(|g| support_of(g) == Support::Ok)
        .collect()
}

/// Jeux CPS2 pris en charge, dans l'ordre de la table GAMES2.
pub fn supported_games2() -> Vec<&'static GameInfo> {
    GAMES2
        .iter()
        .filter(|g| support_of(g) == Support::Ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::find_game;

    #[test]
    fn card_values() {
        let s = |n: &str| support_of(find_game(n).unwrap());
        assert_eq!(s("ghouls"), Support::Ok);
        assert_eq!(s("wof"), Support::Ok);
        assert_eq!(s("pang3"), Support::UnsupportedMachine);
        assert_eq!(s("sf2ee"), Support::Ok);
        assert_eq!(s("forgottn"), Support::Ok);
        // 160 CPS1 + 18 QSound (wof, dino, punisher, slammast, mbombrd et leurs clones)
        // + 17 bootlegs SF2 (init_sf2hack, init_sf2rk, init_sf2thndr) + 3 bootlegs chinois (wofhfh)
        assert_eq!(supported_games().len(), 198);
    }

    #[test]
    fn cps2_support() {
        let s = |n: &str| support_of(find_game(n).unwrap());
        assert_eq!(s("ssf2ud"), Support::Ok);
        assert_eq!(s("ddtod"), Support::Ok);
        assert_eq!(s("mvscjsing"), Support::Ok);
        assert_eq!(s("ssf2tb"), Support::UnsupportedMachine);
        assert_eq!(s("gigaman2"), Support::UnsupportedMachine);
        assert_eq!(s("pzloop2"), Support::UnsupportedInit);
        assert_eq!(s("ecofghtr"), Support::UnsupportedInit);
        assert_eq!(supported_games2().len(), 303);
        assert_eq!(supported_games().len(), 198);
    }

    #[test]
    fn every_supported_game_has_timing() {
        assert!(supported_games()
            .iter()
            .all(|g| crate::ticks_per_68k(g.machine).is_ok()));
    }
}
