//! Sources de fichiers ROM (dossier, zip, chaîne).
//! spec: docs/spec/04 §Recherche des fichiers

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::CartError;
use crate::romset::GameInfo;

/// Source de fichiers ROM.
pub trait FileSource: std::fmt::Debug {
    fn fetch(&self, file: &str, crc: u32) -> Result<Vec<u8>, CartError>;
}

/// Source : dossier local (recherche insensible à la casse).
#[derive(Debug)]
pub struct DirSource {
    pub dir: PathBuf,
}

impl FileSource for DirSource {
    fn fetch(&self, file: &str, _crc: u32) -> Result<Vec<u8>, CartError> {
        let path = self.dir.join(file);
        if path.exists() {
            return fs::read(&path).map_err(|e| CartError::Io(e.to_string()));
        }

        // Recherche insensible à la casse dans le dossier.
        let entries = fs::read_dir(&self.dir)
            .map_err(|e| CartError::Io(format!("lecture de {} : {}", self.dir.display(), e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| CartError::Io(e.to_string()))?;
            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.eq_ignore_ascii_case(file) {
                return fs::read(entry.path()).map_err(|e| {
                    CartError::Io(format!("lecture de {} : {}", entry.path().display(), e))
                });
            }
        }

        Err(CartError::MissingFile {
            file: file.to_string(),
        })
    }
}

/// Source : fichier zip (recherche par nom insensible à la casse, puis par CRC).
#[derive(Debug)]
pub struct ZipSource {
    pub path: PathBuf,
}

impl FileSource for ZipSource {
    fn fetch(&self, file: &str, crc: u32) -> Result<Vec<u8>, CartError> {
        let mut archive =
            zip::ZipArchive::new(fs::File::open(&self.path).map_err(|e| {
                CartError::Io(format!("ouverture de {} : {}", self.path.display(), e))
            })?)
            .map_err(|e| {
                CartError::Io(format!(
                    "archive zip {} invalide : {}",
                    self.path.display(),
                    e
                ))
            })?;

        // Recherche par nom insensible à la casse.
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| CartError::Io(e.to_string()))?;
            let name = entry.name().map_err(|e| CartError::Io(e.to_string()))?;
            if name.eq_ignore_ascii_case(file) && !entry.is_dir() {
                let mut data = Vec::new();
                entry
                    .read_to_end(&mut data)
                    .map_err(|e| CartError::Io(e.to_string()))?;
                return Ok(data);
            }
        }

        // Recherche par CRC32.
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| CartError::Io(e.to_string()))?;
            if !entry.is_dir() && entry.crc32() == crc {
                let mut data = Vec::new();
                entry
                    .read_to_end(&mut data)
                    .map_err(|e| CartError::Io(e.to_string()))?;
                return Ok(data);
            }
        }

        Err(CartError::MissingFile {
            file: file.to_string(),
        })
    }
}

/// Source en chaîne : essaie chaque source dans l'ordre.
#[derive(Debug)]
pub struct ChainSource {
    pub sources: Vec<Box<dyn FileSource>>,
}

impl FileSource for ChainSource {
    fn fetch(&self, file: &str, crc: u32) -> Result<Vec<u8>, CartError> {
        let mut last_err = None;
        for source in &self.sources {
            match source.fetch(file, crc) {
                Ok(data) => return Ok(data),
                Err(CartError::MissingFile { .. }) => continue,
                Err(e) => {
                    last_err = Some(e);
                    break;
                }
            }
        }

        if let Some(err) = last_err {
            Err(err)
        } else {
            Err(CartError::MissingFile {
                file: file.to_string(),
            })
        }
    }
}

/// Construit la chaîne de sources pour un jeu donné.
/// Ordre : <roms>/<name>.zip, <roms>/<name>/, puis parent si non vide.
pub fn sources_for(cfg: &Config, info: &GameInfo) -> ChainSource {
    let mut sources: Vec<Box<dyn FileSource>> = Vec::new();

    // Zip du jeu.
    let game_zip = cfg.roms_dir.join(format!("{}.zip", info.name));
    if game_zip.exists() {
        sources.push(Box::new(ZipSource { path: game_zip }));
    }

    // Dossier du jeu.
    let game_dir = cfg.roms_dir.join(info.name);
    if game_dir.is_dir() {
        sources.push(Box::new(DirSource { dir: game_dir }));
    }

    // Parent (si non vide).
    if !info.parent.is_empty() {
        let parent_zip = cfg.roms_dir.join(format!("{}.zip", info.parent));
        if parent_zip.exists() {
            sources.push(Box::new(ZipSource { path: parent_zip }));
        }

        let parent_dir = cfg.roms_dir.join(info.parent);
        if parent_dir.is_dir() {
            sources.push(Box::new(DirSource { dir: parent_dir }));
        }
    }

    ChainSource { sources }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::romset::RomSetDef;
    use std::io::Write;
    use tempfile::TempDir;

    fn make_config(roms_dir: PathBuf) -> Config {
        Config {
            roms_dir,
            saves_dir: PathBuf::from("saves"),
            mame: None,
            game: "test".to_string(),
            allow_bad_crc: false,
            dips: std::collections::BTreeMap::new(),
        }
    }

    fn make_game(name: &'static str, parent: &'static str) -> GameInfo {
        // Romset minimal pour les tests.
        static ROMSET: RomSetDef = RomSetDef {
            name: "test",
            regions: &[],
        };
        GameInfo {
            name,
            parent,
            machine: "cps1_10MHz",
            input: "",
            init: "",
            rotation: 0,
            maker: "",
            title: "",
            not_working: false,
            romset: &ROMSET,
        }
    }

    #[test]
    fn dir_source_case_insensitive() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("AB.BIN");
        fs::write(&file_path, [1, 2, 3]).unwrap();

        let source = DirSource {
            dir: tmp.path().to_path_buf(),
        };
        let data = source.fetch("ab.bin", 0).unwrap();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn dir_source_missing_file() {
        let tmp = TempDir::new().unwrap();
        let source = DirSource {
            dir: tmp.path().to_path_buf(),
        };
        let result = source.fetch("missing.bin", 0);
        assert!(matches!(result, Err(CartError::MissingFile { .. })));
    }

    #[test]
    fn zip_source_case_insensitive() {
        let tmp = TempDir::new().unwrap();
        let zip_path = tmp.path().join("test.zip");

        // Créer un zip avec une entrée "x.rom".
        {
            let file = fs::File::create(&zip_path).unwrap();
            let mut writer = zip::ZipWriter::new(file);
            writer
                .start_file::<&str, ()>(
                    "x.rom",
                    zip::write::FileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored),
                )
                .unwrap();
            writer.write_all(&[1, 2, 3]).unwrap();
            writer.finish().unwrap();
        }

        let source = ZipSource { path: zip_path };
        let data = source.fetch("X.ROM", 0).unwrap();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn zip_source_by_crc() {
        use cps_common::crc32_bytes;

        let tmp = TempDir::new().unwrap();
        let zip_path = tmp.path().join("test.zip");

        // Créer un zip avec une entrée "x.rom".
        {
            let file = fs::File::create(&zip_path).unwrap();
            let mut writer = zip::ZipWriter::new(file);
            writer
                .start_file::<&str, ()>(
                    "x.rom",
                    zip::write::FileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored),
                )
                .unwrap();
            writer.write_all(&[1, 2, 3]).unwrap();
            writer.finish().unwrap();
        }

        let source = ZipSource { path: zip_path };
        // Recherche par CRC (nom différent).
        let crc = crc32_bytes(&[1, 2, 3]);
        let data = source.fetch("autre", crc).unwrap();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn chain_source_fallback() {
        let tmp1 = TempDir::new().unwrap();
        let tmp2 = TempDir::new().unwrap();

        // Fichier dans le deuxième dossier.
        fs::write(tmp2.path().join("file.bin"), [4, 5, 6]).unwrap();

        let chain = ChainSource {
            sources: vec![
                Box::new(DirSource {
                    dir: tmp1.path().to_path_buf(),
                }),
                Box::new(DirSource {
                    dir: tmp2.path().to_path_buf(),
                }),
            ],
        };

        let data = chain.fetch("file.bin", 0).unwrap();
        assert_eq!(data, vec![4, 5, 6]);
    }

    #[test]
    fn sources_for_builds_chain() {
        let tmp = TempDir::new().unwrap();
        let cfg = make_config(tmp.path().to_path_buf());

        // Créer un zip pour le jeu.
        let game_zip = tmp.path().join("mygame.zip");
        fs::write(&game_zip, []).unwrap();

        let info = make_game("mygame", "");
        let chain = sources_for(&cfg, &info);
        assert_eq!(chain.sources.len(), 1); // Seulement le zip du jeu.
    }

    #[test]
    fn sources_for_with_parent() {
        let tmp = TempDir::new().unwrap();
        let cfg = make_config(tmp.path().to_path_buf());

        // Créer un zip pour le jeu et un dossier pour le parent.
        fs::write(tmp.path().join("child.zip"), []).unwrap();
        fs::create_dir(tmp.path().join("parent")).unwrap();

        let info = make_game("child", "parent");
        let chain = sources_for(&cfg, &info);
        assert_eq!(chain.sources.len(), 2); // Zip enfant + dossier parent.
    }
}
