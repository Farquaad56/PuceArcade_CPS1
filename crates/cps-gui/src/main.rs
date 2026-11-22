//! cps-gui — PuceArcade CPS1 sur le frontal JAMMA (T502).
#![forbid(unsafe_code)]

use std::path::Path;
use std::process::ExitCode;

use cps_cart::Config;
use cps_gui::Cps1Driver;
use puce_jamma_gui::AppOptions;

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
        window_title: "PuceArcade CPS1".into(),
        config_dir: ".".into(),
    };
    match puce_jamma_gui::run(Box::new(Cps1Driver { cfg }), opts) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Erreur du frontal : {e}");
            ExitCode::FAILURE
        }
    }
}
