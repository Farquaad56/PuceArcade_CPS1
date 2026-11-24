//! Voies sonores du débogueur (onglet Son) : 8 canaux FM du YM2151 + 4 voix OKI sur CPS1,
//! 16 voix PCM + 3 voix ADPCM du QSound sur CPS2 et sur les cartes CPS1 QSound.
//! L'ordre des voies est celui des bits du masque de muets (`Chips::set_mute`, `QSound::set_mute`).

use cps_common::clock::{
    TICKS_PER_OKI, TICKS_PER_QSOUND_SAMPLE, TICKS_PER_SECOND, TICKS_PER_YM_SAMPLE,
};
use cps_core::Machine;
use puce_jamma::debug::AudioVoice;

/// Nom des 16 valeurs de note du registre KC (OPM) : 3, 7, 11 et 15 n'existent pas.
const KC_NOTES: [&str; 16] = [
    "Do#", "Ré", "Ré#", "", "Mi", "Fa", "Fa#", "", "Sol", "Sol#", "La", "", "La#", "Si", "Do", "",
];

/// Note du code KC (octave dans les bits 6-4, note dans 3-0 ; « Do » appartient à l'octave suivante).
pub fn kc_note(kc: u32) -> String {
    let note = (kc & 0xF) as usize;
    let octave = (kc >> 4) & 7;
    match KC_NOTES[note] {
        "" => "—".into(),
        "Do" => format!("Do {}", octave + 1),
        n => format!("{n} {octave}"),
    }
}

/// Sortie gauche / droite d'un canal FM.
fn pan(l: bool, r: bool) -> &'static str {
    match (l, r) {
        (true, true) => "G+D",
        (true, false) => "G",
        (false, true) => "D",
        (false, false) => "coupé",
    }
}

/// Une voie au moins non nulle sur ses 512 derniers échantillons.
fn sounding(samples: &[f32]) -> bool {
    samples.iter().rev().take(512).any(|s| *s != 0.0)
}

/// Les voies de la machine (vide si l'enregistrement n'est pas actif).
pub fn voices(m: &Machine) -> Vec<AudioVoice> {
    match m.sound.qsound.as_ref() {
        Some(q) => q
            .scope
            .as_ref()
            .map_or_else(Vec::new, |sc| qsound_voices(q, sc)),
        None => cps1_voices(m),
    }
}

fn cps1_voices(m: &Machine) -> Vec<AudioVoice> {
    let chips = &m.sound.chips;
    let Some(sc) = chips.scopes.as_ref() else {
        return Vec::new();
    };
    let fm = chips.ym.chip().fm();
    let regs = fm.regs();
    let ym_rate = (TICKS_PER_SECOND / TICKS_PER_YM_SAMPLE) as u32;
    let mut v: Vec<AudioVoice> = (0..8u32)
        .map(|ch| {
            let detail = format!(
                "{} · algo {} · {}",
                kc_note(regs.ch_block_freq(ch) >> 6),
                regs.ch_algorithm(ch),
                pan(regs.ch_output_0(ch) != 0, regs.ch_output_1(ch) != 0)
            );
            AudioVoice {
                name: format!("FM {}", ch + 1),
                chip: "YM2151".into(),
                sample_rate: ym_rate,
                samples: sc.0.voice(ch as usize),
                active: fm.active_channels() & (1 << ch) != 0,
                detail,
            }
        })
        .collect();
    let oki_rate = (TICKS_PER_SECOND / (chips.oki.divisor() * TICKS_PER_OKI)) as u32;
    v.extend(chips.oki.voices.iter().enumerate().map(|(i, voice)| {
        let detail = if voice.playing {
            format!("vol {} · {}/{}", voice.volume, voice.sample, voice.count)
        } else {
            String::new()
        };
        AudioVoice {
            name: format!("ADPCM {}", i + 1),
            chip: "OKI MSM6295".into(),
            sample_rate: oki_rate,
            samples: sc.1.voice(i),
            active: voice.playing,
            detail,
        }
    }));
    v
}

fn qsound_voices(q: &cps_bus::QSoundSide, sc: &cps_bus::VoiceScope) -> Vec<AudioVoice> {
    let rate = (TICKS_PER_SECOND / TICKS_PER_QSOUND_SAMPLE) as u32;
    (0..sc.voices())
        .map(|i| {
            let samples = sc.voice(i);
            let (name, detail) = if i < 16 {
                let (vol, step, pan) = q.dsp.voice_regs(i);
                (
                    format!("PCM {}", i + 1),
                    format!("vol {:04X} · pas {step:04X} · pan {pan:02X}", vol as u16),
                )
            } else {
                (format!("ADPCM {}", i - 15), String::new())
            };
            AudioVoice {
                name,
                chip: "QSound".into(),
                sample_rate: rate,
                active: sounding(&samples),
                samples,
                detail,
            }
        })
        .collect()
}

/// Active l'enregistrement des voies et applique les muets (à refaire après un chargement d'état
/// ou une remise à zéro, qui remplacent les puces).
pub fn apply(m: &mut Machine, scope: bool, mute: u64) {
    match m.sound.qsound.as_mut() {
        Some(q) => {
            if scope != q.scope.is_some() {
                q.set_scope(scope);
            }
            q.dsp.set_mute((mute & 0x7_FFFF) as u32);
        }
        None => {
            let chips = &mut m.sound.chips;
            if scope != chips.scopes.is_some() {
                chips.set_scope(scope);
            }
            chips.set_mute(mute);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kc_notes() {
        assert_eq!(kc_note(0x4A), "La 4");
        assert_eq!(kc_note(0x4E), "Do 5");
        assert_eq!(kc_note(0x40), "Do# 4");
        assert_eq!(kc_note(0x43), "—");
    }

    #[test]
    fn pan_labels() {
        assert_eq!(pan(true, true), "G+D");
        assert_eq!(pan(false, true), "D");
    }
}
