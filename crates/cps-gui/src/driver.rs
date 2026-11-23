//! `Cps1Driver` : catalogue et chargement des jeux CPS1 pour le frontal JAMMA (T501).

use std::path::PathBuf;

use cps_cart::{load_game, Config, GameInfo};
use puce_jamma::board::JammaBoard;
use puce_jamma::driver::{BoardDriver, GameEntry};

use crate::board::Cps1Board;

/// Jeux prioritaires, affichés en tête de liste.
const FIRST: [&str; 5] = ["ghouls", "sf2", "sf2ce", "sf2ceea", "sf2hf"];

pub struct Cps1Driver {
    pub cfg: Config,
}

impl Cps1Driver {
    /// ROM présentes : `<roms>/<nom>.zip` ou `<roms>/<nom>/`, ou celles du parent.
    fn available(&self, g: &GameInfo) -> bool {
        let present = |n: &str| {
            !n.is_empty()
                && (self.cfg.roms_dir.join(format!("{n}.zip")).is_file()
                    || self.cfg.roms_dir.join(n).is_dir())
        };
        present(g.name) || present(g.parent)
    }
}

/// Jeux jouables (`cps_cart::support_of`), les jeux prioritaires en tête.
pub fn supported_games() -> Vec<&'static GameInfo> {
    let all = cps_cart::supported_games();
    let mut out: Vec<&GameInfo> = FIRST
        .iter()
        .filter_map(|n| all.iter().copied().find(|g| g.name == *n))
        .collect();
    out.extend(all.into_iter().filter(|g| !FIRST.contains(&g.name)));
    out
}

impl BoardDriver for Cps1Driver {
    fn system_name(&self) -> &str {
        "Capcom CPS1 / CPS2"
    }

    /// Jeux CPS1 (prioritaires en tête) puis jeux CPS2 dans l'ordre de la table (ADR-006 §5).
    fn games(&self) -> Vec<GameEntry> {
        supported_games()
            .into_iter()
            .chain(cps_cart::supported_games2())
            .map(|g| GameEntry {
                name: g.name.into(),
                title: g.title.into(),
                parent: g.parent.into(),
                available: self.available(g),
                supported: true,
            })
            .collect()
    }

    fn default_game(&self) -> Option<String> {
        Some(self.cfg.game.clone()).filter(|g| !g.is_empty())
    }

    fn load(&self, name: &str) -> Result<Box<dyn JammaBoard>, String> {
        let game = load_game(&self.cfg, name).map_err(|e| format!("{name} : {e}"))?;
        let nv = self.cfg.saves_dir.join(format!("{name}.nv"));
        Ok(Box::new(Cps1Board::new(game).with_nvram(nv)))
    }

    fn saves_dir(&self) -> PathBuf {
        self.cfg.saves_dir.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn driver(tag: &str) -> (Cps1Driver, PathBuf) {
        let dir = std::env::temp_dir().join(format!("cps1_driver_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("ghouls")).unwrap();
        std::fs::write(dir.join("sf2.zip"), b"").unwrap();
        std::fs::write(dir.join("ssf2.zip"), b"").unwrap();
        let cfg = Config::parse(&format!(
            "[paths]\nroms_dir = \"{}\"\nsaves_dir = \"{}\"\n",
            dir.display().to_string().replace('\\', "/"),
            dir.join("saves").display().to_string().replace('\\', "/")
        ))
        .unwrap();
        (Cps1Driver { cfg }, dir)
    }

    #[test]
    fn catalogue_order_and_availability() {
        let (d, dir) = driver("cat");
        let g = d.games();
        let names: Vec<&str> = g.iter().take(5).map(|e| e.name.as_str()).collect();
        assert_eq!(names, FIRST);
        let avail = |n: &str| g.iter().find(|e| e.name == n).unwrap().available;
        assert!(avail("ghouls") && avail("sf2"));
        // sf2ce n'a pas de parent : son zip est absent
        assert!(!avail("sf2ce"));
        // sf2ua est un clone de sf2 : le zip du parent suffit
        assert!(avail("sf2ua"));
        assert!(g.iter().all(|e| e.supported));
        // CPS2 après le dernier jeu CPS1 ; ssf2.zip couvre ssf2 et son clone ssf2ud
        assert_eq!(g.len(), 178 + 303);
        let pos = |n: &str| g.iter().position(|e| e.name == n).unwrap();
        assert!(pos("ssf2") >= 178 && pos("ssf2ud") >= 178);
        assert!(avail("ssf2") && avail("ssf2ud"));
        let err = d.load("zzz").err().unwrap();
        assert!(err.contains("zzz"), "{err}");
        assert_eq!(d.saves_dir(), dir.join("saves"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unsupported_hardware_is_excluded() {
        let names: Vec<&str> = supported_games().iter().map(|g| g.name).collect();
        assert!(
            names.contains(&"dino") && names.contains(&"punisher"),
            "QSound pris en charge"
        );
        assert!(!names.contains(&"pang3"));
        assert!(names.contains(&"1941"));
        assert_eq!(names.len(), 178);
    }

    #[test]
    fn driver_is_send_and_sync() {
        fn check<T: Send + Sync>() {}
        check::<Cps1Driver>();
    }
}
