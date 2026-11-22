// license:BSD-3-Clause
// copyright-holders:Aaron Giles
//! Registres FM OPM (YM2151) : `opm_registers` de ymfm.
//! Sources : ref/ymfm/src/ymfm_opm.h l.104-250, ymfm_opm.cpp l.40-346, ymfm_fm.h l.63-160.
//!
//! PORT-NOTE : `log_keyon` et `save_restore` ne sont pas portés (savestates par serde).
//! `opdata_cache::waveform` (pointeur vers la table de sinus) est remplacé par l'accesseur
//! `OpmRegisters::waveform()`.

use crate::common::{bitfield, EG_STATES};
use crate::fm_tables::abs_sin_attenuation;

/// Données calculées une fois au début de l'horloge et constantes pendant la génération.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpdataCache {
    /// Pas de phase, ou `PHASE_STEP_DYNAMIC` si la PM est active.
    pub phase_step: u32,
    /// Total level * 8 + KSL.
    pub total_level: u32,
    /// Valeur brute block/fréquence (sert à calculer phase_step).
    pub block_freq: u32,
    /// Détune (sert à calculer phase_step).
    pub detune: i32,
    /// Multiple (x.1, sert à calculer phase_step).
    pub multiple: u32,
    /// Niveau de sustain, décalé vers les valeurs d'enveloppe.
    pub eg_sustain: u32,
    /// Taux d'enveloppe, KSR inclus.
    pub eg_rate: [u8; EG_STATES],
    /// Décalage d'enveloppe.
    pub eg_shift: u8,
}

impl OpdataCache {
    /// Valeur de `phase_step` demandant un recalcul à chaque échantillon (changements de PM LFO).
    pub const PHASE_STEP_DYNAMIC: u32 = 1;
}

/// Taille d'une forme d'onde sinus complète.
pub const WAVEFORM_LENGTH: usize = 0x400;
/// Longueur des formes d'onde LFO.
const LFO_WAVEFORM_LENGTH: usize = 256;

const fn operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    (o1 as u32) | ((o2 as u32) << 8) | ((o3 as u32) << 16) | ((o4 as u32) << 24)
}

/// Mapping des opérateurs pour chaque canal OPM.
const FIXED_MAP: [u32; 8] = [
    operator_list(0, 1, 2, 3),
    operator_list(4, 5, 6, 7),
    operator_list(8, 9, 10, 11),
    operator_list(12, 13, 14, 15),
    operator_list(16, 17, 18, 19),
    operator_list(20, 21, 22, 23),
    operator_list(24, 25, 26, 27),
    operator_list(28, 29, 30, 31),
];

/// Registres OPM (YM2151).
#[derive(Clone, Debug)]
pub struct OpmRegisters {
    pub(crate) lfo_counter: u32,
    pub(crate) noise_lfsr: u32,
    pub(crate) noise_counter: u8,
    pub(crate) noise_state: u8,
    pub(crate) noise_lfo: u8,
    pub(crate) lfo_am: i16,
    pub(crate) regdata: [u8; OpmRegisters::REGISTERS],
    /// Formes d'onde LFO : AM dans les 8 bits bas, PM dans les 8 bits hauts.
    #[allow(dead_code)]
    pub(crate) lfo_waveform: [[i16; LFO_WAVEFORM_LENGTH]; 4],
    /// Table de sinus (une seule forme d'onde).
    pub(crate) waveform: [u16; WAVEFORM_LENGTH],
}

#[cfg(feature = "serde")]
impl serde::Serialize for OpmRegisters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Sérialise uniquement l'état dynamique ; les tables d'onde sont régénérées à la désérialisation.
        let state = OpmRegistersState {
            lfo_counter: self.lfo_counter,
            noise_lfsr: self.noise_lfsr,
            noise_counter: self.noise_counter,
            noise_state: self.noise_state,
            noise_lfo: self.noise_lfo,
            lfo_am: self.lfo_am,
            regdata: self.regdata,
        };
        state.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OpmRegisters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let state = OpmRegistersState::deserialize(deserializer)?;
        // Régénère les tables d'onde constantes.
        let mut regs = Self::new();
        regs.lfo_counter = state.lfo_counter;
        regs.noise_lfsr = state.noise_lfsr;
        regs.noise_counter = state.noise_counter;
        regs.noise_state = state.noise_state;
        regs.noise_lfo = state.noise_lfo;
        regs.lfo_am = state.lfo_am;
        regs.regdata = state.regdata;
        Ok(regs)
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct OpmRegistersState {
    lfo_counter: u32,
    noise_lfsr: u32,
    noise_counter: u8,
    noise_state: u8,
    noise_lfo: u8,
    lfo_am: i16,
    #[serde(with = "crate::serde_arr")]
    regdata: [u8; OpmRegisters::REGISTERS],
}

impl Default for OpmRegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl OpmRegisters {
    // constantes
    pub const OUTPUTS: u32 = 2;
    pub const CHANNELS: u32 = 8;
    pub const ALL_CHANNELS: u32 = (1 << Self::CHANNELS) - 1;
    pub const OPERATORS: u32 = Self::CHANNELS * 4;
    pub const WAVEFORMS: u32 = 1;
    pub const REGISTERS: usize = 0x100;
    pub const DEFAULT_PRESCALE: u32 = 2;
    pub const EG_CLOCK_DIVIDER: u32 = 3;
    pub const CSM_TRIGGER_MASK: u32 = Self::ALL_CHANNELS;
    pub const REG_MODE: u32 = 0x14;
    pub const STATUS_TIMERA: u8 = 0x01;
    pub const STATUS_TIMERB: u8 = 0x02;
    pub const STATUS_BUSY: u8 = 0x80;
    pub const STATUS_IRQ: u8 = 0;
    pub const DYNAMIC_OPS: bool = false;
    pub const EG_HAS_SSG: bool = false;
    pub const MODULATOR_DELAY: bool = false;

    /// Constructeur : crée les tables de formes d'onde LFO et sinus.
    #[allow(clippy::needless_range_loop)]
    pub fn new() -> Self {
        // Table de sinus (une seule forme d'onde)
        let mut waveform = [0u16; WAVEFORM_LENGTH];
        for index in 0..WAVEFORM_LENGTH {
            let index_u32 = index as u32;
            waveform[index] =
                (abs_sin_attenuation(index_u32) | (bitfield(index_u32, 9, 1) << 15)) as u16;
        }

        // Formes d'onde LFO : AM dans les 8 bits bas, PM dans les 8 bits hauts
        let mut lfo_waveform = [[[0i16; LFO_WAVEFORM_LENGTH]; 4]];
        for index in 0..LFO_WAVEFORM_LENGTH {
            let index_u32 = index as u32;

            // Forme d'onde 0 : dent de scie
            let am: i16 = (index ^ 0xff) as i16;
            let pm: i16 = index as i16;
            lfo_waveform[0][0][index] = am | (pm << 8);

            // Forme d'onde 1 : carrée
            let am2: i16 = if bitfield(index_u32, 7, 1) != 0 {
                0
            } else {
                0xff
            };
            let pm2: i16 = am2 ^ 0x80;
            lfo_waveform[0][1][index] = am2 | (pm2 << 8);

            // Forme d'onde 2 : triangulaire
            let am3: i16 = if bitfield(index_u32, 7, 1) != 0 {
                ((index as u8) << 1) as i16
            } else {
                (((index ^ 0xff) as u8) << 1) as i16
            };
            let pm3: i16 = if bitfield(index_u32, 6, 1) != 0 {
                am3
            } else {
                !am3
            };
            lfo_waveform[0][2][index] = pm3 | (pm3 << 8);

            // Forme d'onde 3 : bruit (remplie dynamiquement)
            lfo_waveform[0][3][index] = 0;
        }

        OpmRegisters {
            lfo_counter: 0,
            noise_lfsr: 1,
            noise_counter: 0,
            noise_state: 0,
            noise_lfo: 0,
            lfo_am: 0,
            regdata: [0; Self::REGISTERS],
            lfo_waveform: lfo_waveform[0],
            waveform,
        }
    }

    /// Retour à l'état initial.
    pub fn reset(&mut self) {
        self.regdata.fill(0);
        // Sortie activée sur les deux canaux par défaut pour tous les canaux
        for i in 0..8 {
            self.regdata[0x20 + i] = 0xc0;
        }
    }

    /// Retourne la table de sinus.
    pub fn waveform(&self) -> &[u16] {
        &self.waveform
    }

    /// Mappe un numéro de canal vers son offset dans les registres.
    pub fn channel_offset(chnum: u32) -> u32 {
        assert!(chnum < Self::CHANNELS);
        chnum
    }

    /// Mappe un numéro d'opérateur vers son offset dans les registres.
    pub fn operator_offset(opnum: u32) -> u32 {
        assert!(opnum < Self::OPERATORS);
        opnum
    }

    /// Retourne le mapping des opérateurs pour chaque canal.
    pub fn operator_map(&self, dest: &mut [u32; 8]) {
        for (i, &entry) in FIXED_MAP.iter().enumerate() {
            dest[i] = entry;
        }
    }

    // --- Accesseurs système ---

    /// Registre de test.
    pub fn test(&self) -> u32 {
        self.byte(0x01, 0, 8, 0)
    }

    /// Réinitialisation LFO.
    pub fn lfo_reset(&self) -> u32 {
        self.byte(0x01, 1, 1, 0)
    }

    /// Fréquence du bruit.
    pub fn noise_frequency(&self) -> u32 {
        self.byte(0x0f, 0, 5, 0) ^ 0x1f
    }

    /// Activation du bruit.
    pub fn noise_enable(&self) -> u32 {
        self.byte(0x0f, 7, 1, 0)
    }

    /// Valeur de la minuterie A.
    pub fn timer_a_value(&self) -> u32 {
        self.word(0x10, 0, 8, 0x11, 0, 2, 0)
    }

    /// Valeur de la minuterie B.
    pub fn timer_b_value(&self) -> u32 {
        self.byte(0x12, 0, 8, 0)
    }

    /// Mode CSM (Channel Select Mode).
    pub fn csm(&self) -> u32 {
        self.byte(0x14, 7, 1, 0)
    }

    /// Réinitialisation de la minuterie B.
    pub fn reset_timer_b(&self) -> u32 {
        self.byte(0x14, 5, 1, 0)
    }

    /// Réinitialisation de la minuterie A.
    pub fn reset_timer_a(&self) -> u32 {
        self.byte(0x14, 4, 1, 0)
    }

    /// Activation de la minuterie B.
    pub fn enable_timer_b(&self) -> u32 {
        self.byte(0x14, 3, 1, 0)
    }

    /// Activation de la minuterie A.
    pub fn enable_timer_a(&self) -> u32 {
        self.byte(0x14, 2, 1, 0)
    }

    /// Chargement de la minuterie B.
    pub fn load_timer_b(&self) -> u32 {
        self.byte(0x14, 1, 1, 0)
    }

    /// Chargement de la minuterie A.
    pub fn load_timer_a(&self) -> u32 {
        self.byte(0x14, 0, 1, 0)
    }

    /// Taux du LFO.
    pub fn lfo_rate(&self) -> u32 {
        self.byte(0x18, 0, 8, 0)
    }

    /// Profondeur AM du LFO.
    pub fn lfo_am_depth(&self) -> u32 {
        self.byte(0x19, 0, 7, 0)
    }

    /// Profondeur PM du LFO.
    pub fn lfo_pm_depth(&self) -> u32 {
        self.byte(0x1a, 0, 7, 0)
    }

    /// Nombre de bits de sortie.
    pub fn output_bits(&self) -> u32 {
        self.byte(0x1b, 6, 2, 0)
    }

    /// Forme d'onde du LFO.
    pub fn lfo_waveform_select(&self) -> u32 {
        self.byte(0x1b, 0, 2, 0)
    }

    // --- Accesseurs par canal ---

    /// Sortie sur n'importe quel canal (bits 6-7).
    pub fn ch_output_any(&self, choffs: u32) -> u32 {
        self.byte(0x20, 6, 2, choffs)
    }

    /// Sortie sur le canal 0.
    pub fn ch_output_0(&self, choffs: u32) -> u32 {
        self.byte(0x20, 6, 1, choffs)
    }

    /// Sortie sur le canal 1.
    pub fn ch_output_1(&self, choffs: u32) -> u32 {
        self.byte(0x20, 7, 1, choffs)
    }

    /// Feedback du canal.
    pub fn ch_feedback(&self, choffs: u32) -> u32 {
        self.byte(0x20, 3, 3, choffs)
    }

    /// Algorithme du canal.
    pub fn ch_algorithm(&self, choffs: u32) -> u32 {
        self.byte(0x20, 0, 3, choffs)
    }

    /// Block/fréquence du canal.
    pub fn ch_block_freq(&self, choffs: u32) -> u32 {
        self.word(0x28, 0, 7, 0x30, 2, 6, choffs)
    }

    /// Sensibilité PM LFO du canal.
    pub fn ch_lfo_pm_sens(&self, choffs: u32) -> u32 {
        self.byte(0x38, 4, 3, choffs)
    }

    /// Sensibilité AM LFO du canal.
    pub fn ch_lfo_am_sens(&self, choffs: u32) -> u32 {
        self.byte(0x38, 0, 2, choffs)
    }

    // --- Accesseurs par opérateur ---

    /// Détune de l'opérateur.
    pub fn op_detune(&self, opoffs: u32) -> u32 {
        self.byte(0x40, 4, 3, opoffs)
    }

    /// Multiple de l'opérateur.
    pub fn op_multiple(&self, opoffs: u32) -> u32 {
        self.byte(0x40, 0, 4, opoffs)
    }

    /// Niveau total de l'opérateur.
    pub fn op_total_level(&self, opoffs: u32) -> u32 {
        self.byte(0x60, 0, 7, opoffs)
    }

    /// KSR de l'opérateur.
    pub fn op_ksr(&self, opoffs: u32) -> u32 {
        self.byte(0x80, 6, 2, opoffs)
    }

    /// Taux d'attaque de l'opérateur.
    pub fn op_attack_rate(&self, opoffs: u32) -> u32 {
        self.byte(0x80, 0, 5, opoffs)
    }

    /// Activation AM LFO de l'opérateur.
    pub fn op_lfo_am_enable(&self, opoffs: u32) -> u32 {
        self.byte(0xa0, 7, 1, opoffs)
    }

    /// Taux de décroissance de l'opérateur.
    pub fn op_decay_rate(&self, opoffs: u32) -> u32 {
        self.byte(0xa0, 0, 5, opoffs)
    }

    /// Détune2 de l'opérateur.
    pub fn op_detune2(&self, opoffs: u32) -> u32 {
        self.byte(0xc0, 6, 2, opoffs)
    }

    /// Taux de sustain de l'opérateur.
    pub fn op_sustain_rate(&self, opoffs: u32) -> u32 {
        self.byte(0xc0, 0, 5, opoffs)
    }

    /// Niveau de sustain de l'opérateur.
    pub fn op_sustain_level(&self, opoffs: u32) -> u32 {
        self.byte(0xe0, 4, 4, opoffs)
    }

    /// Taux de relâchement de l'opérateur.
    pub fn op_release_rate(&self, opoffs: u32) -> u32 {
        self.byte(0xe0, 0, 4, opoffs)
    }

    // --- Méthodes par défaut (non implémentées pour OPM) ---

    /// Masque de statut (OPL seulement).
    pub fn status_mask(&self) -> u32 {
        0
    }

    /// Réinitialisation IRQ (OPL seulement).
    pub fn irq_reset(&self) -> u32 {
        0
    }

    /// Activation SSG EG de l'opérateur (OPN(A) seulement).
    pub fn op_ssg_eg_enable(&self, _opoffs: u32) -> u32 {
        0
    }

    /// Mode SSG EG de l'opérateur (OPN(A) seulement).
    pub fn op_ssg_eg_mode(&self, _opoffs: u32) -> u32 {
        0
    }

    // --- Helpers internes ---

    /// Extrait un champ de bits d'un octet.
    fn byte(&self, offset: u32, start: i32, count: i32, extra_offset: u32) -> u32 {
        bitfield(
            self.regdata[(offset + extra_offset) as usize] as u32,
            start,
            count,
        )
    }

    /// Extrait un champ de bits sur deux octets (MSB en premier).
    #[allow(clippy::too_many_arguments)]
    fn word(
        &self,
        offset1: u32,
        start1: i32,
        count1: i32,
        offset2: u32,
        start2: i32,
        count2: i32,
        extra_offset: u32,
    ) -> u32 {
        (self.byte(offset1, start1, count1, extra_offset) << count2)
            | self.byte(offset2, start2, count2, extra_offset)
    }

    // --- Méthodes implémentées dans opm_regs_ops.rs ---

    /// État du bruit.
    pub fn noise_state_get(&self) -> u32 {
        self.noise_state as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_map() {
        let regs = OpmRegisters::new();
        let mut map = [0u32; 8];
        regs.operator_map(&mut map);

        // Canal 0 : opérateurs 0,1,2,3 -> 0x00000000 | (1<<8) | (2<<16) | (3<<24) = 0x03020100
        assert_eq!(map[0], 0x03020100);

        // Canal 7 : opérateurs 28,29,30,31 -> 0x1F1E1D1C
        assert_eq!(map[7], 0x1f1e1d1c);
    }

    #[test]
    fn test_reset() {
        let mut regs = OpmRegisters::new();
        // Écrire une valeur dans un registre
        regs.regdata[0x20] = 0xff;
        regs.reset();
        // Après reset, tous les registres doivent être à 0 sauf les sorties (0xc0)
        assert_eq!(regs.regdata[0x20], 0xc0);
        for i in 1..8 {
            assert_eq!(regs.regdata[0x20 + i], 0xc0);
        }
    }

    #[test]
    fn test_accessors() {
        let mut regs = OpmRegisters::new();
        // Écrire directement dans le registre d'algorithme du canal 0 (0x20)
        regs.regdata[0x20] = 0xc7; // bits 6-7=11 (sortie sur les deux canaux), bits 3-5=0 (feedback), bits 0-2=7 (algorithme)

        assert_eq!(regs.ch_algorithm(0), 7);
        assert_eq!(regs.ch_feedback(0), 0);
        assert_eq!(regs.ch_output_any(0), 3); // bits 6-7 = 11b = 3
    }
}
