//! Bilan du dossier ROM : pour chaque zip (ou dossier) de `roms_dir`, les jeux CPS1 / CPS2 qu'il
//! contient et s'ils se chargent (fichiers, tailles, CRC, clé CPS2). ROM requises :
//! `cargo test -p cps-cart --release --test rom_check -- --ignored --nocapture`.
//! Rapport écrit dans `target/roms_rapport.md` (hors dépôt).

use cps_cart::game::DL1425_CRC;
use cps_cart::romsets2_data::GAMES2;
use cps_cart::romsets_data::GAMES;
use cps_cart::source::{DirSource, FileSource, ZipSource};
use cps_cart::{load_game, support_of, system_of, Config, GameInfo, Support, System};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

fn system_name(g: &GameInfo) -> &'static str {
    match system_of(g) {
        System::Cps1 => "CPS1",
        System::Cps2 => "CPS2",
    }
}

/// ROM système : (set, fichier, CRC, rôle). Pas des jeux, mais requises par certains jeux.
const SYSTEM_ROMS: [(&str, &str, u32, &str); 1] = [(
    "qsound",
    "dl-1425.bin",
    DL1425_CRC,
    "programme de la puce son QSound",
)];

/// Jeux pris en charge qui ont besoin du set système `set`.
fn needs(g: &GameInfo, set: &str) -> bool {
    set == "qsound" && (g.machine == "qsound" || system_of(g) == System::Cps2)
}

/// État d'une ROM système : zip `set.zip` ou dossier `set/` dans roms_dir.
fn system_rom_status(cfg: &Config, set: &str, file: &str, crc: u32) -> Result<(), String> {
    let zip = cfg.roms_dir.join(format!("{set}.zip"));
    let dir = cfg.roms_dir.join(set);
    if zip.exists() {
        ZipSource { path: zip }
            .fetch(file, crc)
            .map(|_| ())
            .map_err(|e| e.to_string())
    } else if dir.is_dir() {
        DirSource { dir }
            .fetch(file, crc)
            .map(|_| ())
            .map_err(|e| e.to_string())
    } else {
        Err(format!("{set}.zip absent"))
    }
}

#[test]
#[ignore] // ROM requises
fn rom_check() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let Ok(cfg) = Config::load(&root.join("cps1.toml")) else {
        println!("cps1.toml absent : bilan ignoré");
        return;
    };
    // jeux du dossier : zip ou dossier portant le nom du jeu
    let mut sets: BTreeMap<String, bool> = BTreeMap::new();
    for e in std::fs::read_dir(&cfg.roms_dir)
        .expect("roms_dir")
        .flatten()
    {
        let n = e.file_name().to_string_lossy().to_string();
        if let Some(s) = n.strip_suffix(".zip") {
            sets.insert(s.to_string(), true);
        } else if e.path().is_dir() {
            sets.insert(n, false);
        }
    }
    let all: Vec<&GameInfo> = GAMES.iter().chain(GAMES2.iter()).collect();
    let (mut ok, mut bad, mut unsupported) = (Vec::new(), Vec::new(), Vec::new());
    let mut unknown = Vec::new();
    for set in sets.keys() {
        if SYSTEM_ROMS.iter().any(|r| r.0 == set) {
            continue;
        }
        // jeux portés par ce set : lui-même s'il est parent, ou ses clones
        let games: Vec<&&GameInfo> = all
            .iter()
            .filter(|g| g.name == set || g.parent == set)
            .collect();
        if games.is_empty() {
            unknown.push(set.clone());
            continue;
        }
        for g in games {
            // un clone dont le set propre existe est jugé avec son propre set
            if g.name != set && sets.contains_key(g.name) {
                continue;
            }
            let line = format!("{} ({}, {})", g.name, system_name(g), g.title);
            let why = match support_of(g) {
                Support::Ok => None,
                Support::UnsupportedMachine => Some(format!("machine {} non émulée", g.machine)),
                Support::UnsupportedInit => Some(format!("{} non émulée", g.init)),
                Support::NotWorking => Some("marqué non fonctionnel par MAME".to_string()),
            };
            if let Some(why) = why {
                unsupported.push(format!("{line} : {why}"));
                continue;
            }
            match load_game(&cfg, g.name) {
                Ok(_) => ok.push(line),
                Err(e) => bad.push(format!("{line} : {e}")),
            }
        }
    }
    let mut r = String::new();
    let _ = writeln!(r, "# Bilan du dossier ROM ({})\n", cfg.roms_dir.display());
    let _ = writeln!(r, "## ROM système\n");
    for (set, file, crc, role) in SYSTEM_ROMS {
        let n = all
            .iter()
            .filter(|g| support_of(g) == Support::Ok && needs(g, set))
            .count();
        match system_rom_status(&cfg, set, file, crc) {
            Ok(()) => {
                let _ = writeln!(
                    r,
                    "- ✅ {set} ({file}, {role}) : bonne, utilisée par {n} jeux"
                );
            }
            Err(e) => {
                let _ = writeln!(
                    r,
                    "- ❌ {set} ({file}, {role}) : ROM système manquante ou mauvaise ({e}) ; \
                     nécessaire pour {n} jeux (cartes QSound du CPS1 et tous les jeux CPS2)"
                );
            }
        }
    }
    let _ = writeln!(r);
    let _ = writeln!(
        r,
        "{} jeux bons, {} jeux en erreur, {} non pris en charge, {} sets inconnus\n",
        ok.len(),
        bad.len(),
        unsupported.len(),
        unknown.len()
    );
    for (title, list) in [
        ("Bons (se chargent)", &ok),
        ("En erreur", &bad),
        ("Non pris en charge par l'émulateur", &unsupported),
        ("Sets inconnus (ni CPS1 ni CPS2 de MAME 0.287)", &unknown),
    ] {
        let _ = writeln!(r, "## {title} ({})\n", list.len());
        for l in list {
            let _ = writeln!(r, "- {l}");
        }
        let _ = writeln!(r);
    }
    let out = root.join("target/roms_rapport.md");
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    std::fs::write(&out, &r).unwrap();
    println!("{r}");
}
