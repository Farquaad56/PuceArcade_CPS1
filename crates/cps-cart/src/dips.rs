//! DIP switches : valeurs par défaut et description des champs.
//! spec: docs/spec/01 §DIP, docs/spec/04 §Jeux cibles

use crate::dips_data;

/// Définition d'un champ DIP (un groupe de bits dans un port).
#[derive(Debug)]
pub struct DipFieldDef {
    pub port: u8, // 0 = DSWA, 1 = DSWB, 2 = DSWC
    pub mask: u8,
    pub default: u8, // valeur par défaut dans le masque
    pub label: &'static str,
    pub settings: &'static [(&'static str, u8)],
}

/// Récupère les champs DIP pour un bloc INPUT_PORTS donné.
pub fn dip_fields(input: &str) -> &'static [DipFieldDef] {
    for (name, fields) in dips_data::DIP_FIELDS {
        if *name == input {
            return fields;
        }
    }
    &[]
}

/// Récupère la disposition des entrées (joueurs, boutons du joueur 1).
pub fn input_layout(input: &str) -> (u8, u8) {
    for (name, players, buttons) in dips_data::INPUTS {
        if *name == input {
            return (*players, *buttons);
        }
    }
    // Absent : 2 joueurs, 3 boutons.
    (2, 3)
}

/// Valeurs par défaut des DIP switches pour un bloc INPUT_PORTS donné.
pub fn default_dips(input: &str) -> cps_common::Dips {
    for (name, dswa, dswb, dswc) in dips_data::DIPS {
        if *name == input {
            return cps_common::Dips {
                dswa: *dswa,
                dswb: *dswb,
                dswc: *dswc,
            };
        }
    }
    // Absent : tous les bits à 1 (repose).
    cps_common::Dips::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_dips_ghouls() {
        let dips = default_dips("ghouls");
        assert_eq!(dips.dswa, 0xFF);
        assert_eq!(dips.dswb, 0xFD);
        assert_eq!(dips.dswc, 0xFF);
    }

    #[test]
    fn default_dips_sf2() {
        let dips = default_dips("sf2");
        assert_eq!(dips.dswa, 0xFF);
        assert_eq!(dips.dswb, 0xFC);
        assert_eq!(dips.dswc, 0x9F);
    }

    #[test]
    fn default_dips_unknown() {
        let dips = default_dips("inconnu");
        assert_eq!(dips, cps_common::Dips::default());
    }

    #[test]
    fn dip_fields_ghouls_coin_a() {
        let fields = dip_fields("ghouls");
        // Premier champ : Coin A.
        let coin_a = &fields[0];
        assert_eq!(coin_a.label, "Coin A");
        assert_eq!(coin_a.port, 0);
        assert_eq!(coin_a.mask, 0x07);
        assert_eq!(coin_a.default, 0x07);
        // Vérifie qu'il y a bien un réglage "1 Coin/1 Credit".
        let has_1c1cr = coin_a
            .settings
            .iter()
            .any(|(label, val)| *label == "1 Coin/1 Credit" && *val == 0x07);
        assert!(has_1c1cr);
    }

    #[test]
    fn dip_fields_ghouls_difficulty() {
        let fields = dip_fields("ghouls");
        // Trouve le champ Difficulty.
        let difficulty = fields
            .iter()
            .find(|f| f.label == "Difficulty")
            .expect("champ Difficulty");
        assert_eq!(difficulty.port, 1);
        assert_eq!(difficulty.default, 0x05);
    }

    #[test]
    fn dip_fields_unknown() {
        let fields = dip_fields("inconnu");
        assert!(fields.is_empty());
    }

    #[test]
    fn input_layout_ghouls() {
        let (players, buttons) = input_layout("ghouls");
        assert_eq!(players, 2);
        assert_eq!(buttons, 2);
    }

    #[test]
    fn input_layout_sf2() {
        let (players, buttons) = input_layout("sf2");
        assert_eq!(players, 2);
        assert_eq!(buttons, 6);
    }

    #[test]
    fn input_layout_captcomm() {
        let (players, buttons) = input_layout("captcomm");
        assert_eq!(players, 4);
        assert_eq!(buttons, 2);
    }
}
