//! Enregistreur des voies d'une puce son pour l'oscilloscope du débogueur : un tampon circulaire
//! par voie, alimenté à chaque échantillon de la puce, seulement quand le débogueur le demande.

/// Échantillons gardés par voie.
pub const SCOPE_LEN: usize = 4096;

#[derive(Clone, Debug)]
pub struct VoiceScope {
    voices: usize,
    /// `SCOPE_LEN` échantillons par voie, voie après voie, valeurs déjà ramenées dans [-1, 1].
    buf: Vec<f32>,
    /// Prochaine position d'écriture (commune à toutes les voies).
    pos: usize,
    /// Facteur appliqué aux valeurs brutes de la puce.
    scale: f32,
}

impl VoiceScope {
    /// `full` : valeur brute qui correspond à la pleine échelle.
    pub fn new(voices: usize, full: f32) -> Self {
        VoiceScope {
            voices,
            buf: vec![0.0; voices * SCOPE_LEN],
            pos: 0,
            scale: 1.0 / full,
        }
    }

    /// Un échantillon de chaque voie (les voies manquantes restent à 0).
    pub fn push<T: Copy + Into<f64>>(&mut self, values: &[T]) {
        for v in 0..self.voices {
            let x = values.get(v).map_or(0.0, |x| (*x).into() as f32);
            self.buf[v * SCOPE_LEN + self.pos] = (x * self.scale).clamp(-1.0, 1.0);
        }
        self.pos = (self.pos + 1) % SCOPE_LEN;
    }

    /// Les `SCOPE_LEN` derniers échantillons de la voie `v`, du plus ancien au plus récent.
    pub fn voice(&self, v: usize) -> Vec<f32> {
        if v >= self.voices {
            return Vec::new();
        }
        let ring = &self.buf[v * SCOPE_LEN..(v + 1) * SCOPE_LEN];
        let mut out = Vec::with_capacity(SCOPE_LEN);
        out.extend_from_slice(&ring[self.pos..]);
        out.extend_from_slice(&ring[..self.pos]);
        out
    }

    pub fn voices(&self) -> usize {
        self.voices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_keeps_order_and_scale() {
        let mut s = VoiceScope::new(2, 100.0);
        for i in 0..(SCOPE_LEN + 3) as i32 {
            s.push(&[i % 100, -50]);
        }
        let v0 = s.voice(0);
        assert_eq!(v0.len(), SCOPE_LEN);
        // plus récent en dernier : i = SCOPE_LEN + 2
        let last = ((SCOPE_LEN + 2) % 100) as f32 / 100.0;
        assert!((*v0.last().unwrap() - last).abs() < 1e-6);
        assert!((v0[SCOPE_LEN - 2] - ((SCOPE_LEN + 1) % 100) as f32 / 100.0).abs() < 1e-6);
        assert!(s.voice(1).iter().all(|x| *x == -0.5));
        assert!(s.voice(2).is_empty());
    }

    #[test]
    fn clamps_and_pads_missing_voices() {
        let mut s = VoiceScope::new(3, 10.0);
        s.push(&[20i16, -30]);
        assert_eq!(*s.voice(0).last().unwrap(), 1.0);
        assert_eq!(*s.voice(1).last().unwrap(), -1.0);
        assert_eq!(*s.voice(2).last().unwrap(), 0.0);
    }
}
