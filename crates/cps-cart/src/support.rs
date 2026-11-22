//! Jeux jouables : machine et initialisation émulées (T602).
//! spec: docs/spec/04-romsets-games.md §Jeux cibles

use crate::romset::GameInfo;
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

const MACHINES: [&str; 4] = ["cps1_10MHz", "cps1_12MHz", "forgottn", "qsound"];
const INITS: [&str; 6] = [
    "empty_init",
    "init_sf2ee",
    "init_wof",
    "init_dino",
    "init_punisher",
    "init_slammast",
];

pub fn support_of(info: &GameInfo) -> Support {
    if !MACHINES.contains(&info.machine) {
        Support::UnsupportedMachine
    } else if !INITS.contains(&info.init) {
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
        assert_eq!(supported_games().len(), 178);
    }

    #[test]
    fn every_supported_game_has_timing() {
        assert!(supported_games()
            .iter()
            .all(|g| crate::ticks_per_68k(g.machine).is_ok()));
    }
}
