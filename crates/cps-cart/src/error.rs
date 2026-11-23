//! Erreurs de chargement et configuration des cartouches.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CartError {
    Io(String),
    Config(String),
    MissingFile {
        file: String,
    },
    BadSize {
        file: String,
        expected: usize,
        got: usize,
    },
    BadCrc {
        file: String,
        expected: u32,
        got: u32,
    },
    UnknownGame(String),
    UnsupportedMachine(String),
    /// CPS2 : fichier .key absent du romset (région « key »).
    MissingKey(String),
    OutOfRegion {
        region: String,
        offset: usize,
    },
}

impl std::fmt::Display for CartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CartError::Io(msg) => write!(f, "erreur E/S : {}", msg),
            CartError::Config(msg) => write!(f, "configuration invalide : {}", msg),
            CartError::MissingFile { file } => write!(f, "fichier manquant : {}", file),
            CartError::BadSize {
                file,
                expected,
                got,
            } => {
                write!(
                    f,
                    "taille incorrecte pour {} : attendu {} octets, obtenu {}",
                    file, expected, got
                )
            }
            CartError::BadCrc {
                file,
                expected,
                got,
            } => {
                write!(
                    f,
                    "CRC incorrect pour {} : attendu {:08X}, obtenu {:08X}",
                    file, expected, got
                )
            }
            CartError::UnknownGame(name) => write!(f, "jeu inconnu : {}", name),
            CartError::UnsupportedMachine(name) => write!(f, "machine non supportée : {}", name),
            CartError::MissingKey(name) => {
                write!(f, "clé CPS2 (.key) absente du romset de {}", name)
            }
            CartError::OutOfRegion { region, offset } => {
                write!(
                    f,
                    "déplacement hors de la région {} (offset {:X})",
                    region, offset
                )
            }
        }
    }
}

impl std::error::Error for CartError {}
