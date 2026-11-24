//! cps-gui — PuceArcade CPS1/CPS2 sur le frontal JAMMA (T502).
#![forbid(unsafe_code)]

use std::path::Path;
use std::process::ExitCode;

use cps_cart::Config;
use cps_gui::Cps1Driver;
use puce_jamma_gui::about::About;
use puce_jamma_gui::AppOptions;

/// Nom affiché dans la fenêtre et dans « À propos ».
const NAME: &str = "PuceArcade CPS1/CPS2";

fn main() -> ExitCode {
    let cfg = match Config::load(Path::new(Config::DEFAULT_PATH)) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration illisible ({}) : {e}", Config::DEFAULT_PATH);
            eprintln!("Copier cps1.example.toml en cps1.toml et y indiquer le dossier des ROM.");
            return ExitCode::FAILURE;
        }
    };
    let opts = AppOptions {
        window_title: NAME.into(),
        config_dir: ".".into(),
        bezels_dir: Some("bezels".into()),
        about: Some(About {
            name: NAME.into(),
            version: env!("CARGO_PKG_VERSION").into(),
            text: "Projet d'émulateur en Rust Par Le Garrec Sebastien".into(),
            license_name: "PuceArcade — usage non commercial, tous droits réservés".into(),
            license_text: include_str!("../../../LICENSE").into(),
        }),
    };
    match puce_jamma_gui::run(Box::new(Cps1Driver { cfg }), opts) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Erreur du frontal : {e}");
            ExitCode::FAILURE
        }
    }
}
