//! Savestates (T408) : `Machine::save_state` et `Machine::load_state`.
//! Modèle : ref/puce16-neogeo/ng-core/state.rs.
//!
//! Format : magic `CPS1`, version u32 (little-endian), puis une suite de blocs bincode :
//! nom du jeu, 68000, Z80, port de broches de la carte son, Bus68k (ram, entrées, DIP, coinctrl,
//! uPD4701 optionnel — version 2, T603 ; coinctrl2 et EEPROM — version 3, QSound), carte QSound
//! optionnelle (RAM 2, DSP sans ses ROM, IRQ, échéances) — version 3,
//! Video (sans `cfg`, relié par `relink()`), SoundBoard (ram, banque, latches, temps), Chips
//! (YM2151 cadencé, OKI sans sa ROM, échéance et dernier échantillon OKI, échantillons en attente),
//! puis l'horloge de la machine et le framebuffer.
//! Les ROM (68000, Z80, OKI, graphismes) ne sont pas sauvegardées.
//! Les deux CPU sont sauvés à une frontière d'instruction.
//! Limite connue (Q-M68KSTATE, comme Neo Geo) : l'état STOP du 68000 n'est pas sauvegardé.

use puce_m68k::state::CpuState;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::machine::Machine;

const MAGIC: &[u8; 4] = b"CPS1";
const VERSION: u32 = 3;

/// Registres du 68000 : d, a, usp, ssp, sr, pc, prefetch.
type M68 = ([u32; 8], [u32; 7], u32, u32, u16, u32, [u16; 2]);

fn put<T: Serialize + ?Sized>(buf: &mut Vec<u8>, v: &T) {
    bincode::serialize_into(buf, v).expect("écriture d'un savestate en mémoire");
}

fn get<T: DeserializeOwned>(rd: &mut &[u8]) -> Result<T, String> {
    bincode::deserialize_from(rd).map_err(|e| format!("savestate illisible : {e}"))
}

fn fill<T: Copy>(dst: &mut [T], src: Vec<T>) -> Result<(), String> {
    if dst.len() != src.len() {
        return Err(format!(
            "savestate : bloc de {} éléments, {} attendus",
            src.len(),
            dst.len()
        ));
    }
    dst.copy_from_slice(&src);
    Ok(())
}

impl Machine {
    /// Sérialise tout l'état. Termine d'abord l'instruction en cours des deux CPU.
    pub fn save_state(&mut self) -> Vec<u8> {
        self.finish_instruction();
        while !self.z80.at_instruction_boundary() {
            self.sound.tick(&mut self.z80);
        }

        let mut buf = Vec::with_capacity(0x50000);
        buf.extend_from_slice(MAGIC);
        buf.extend_from_slice(&VERSION.to_le_bytes());
        put(&mut buf, &self.game);

        let s = self.cpu.state();
        let m68: M68 = (s.d, s.a, s.usp, s.ssp, s.sr, s.pc, s.prefetch);
        put(&mut buf, &m68);
        let z80 = self
            .z80
            .save_state()
            .expect("Z80 à une frontière d'instruction");
        put(&mut buf, &z80);

        // Bus 68000 (la ROM n'est pas sauvegardée)
        put(&mut buf, &self.bus.ram[..]);
        put(
            &mut buf,
            &(
                self.bus.inputs,
                self.bus.dips,
                self.bus.coinctrl,
                &self.bus.dial,
                self.bus.coinctrl2,
                &self.bus.eeprom,
            ),
        );
        put(&mut buf, &self.bus.video);

        // Carte son (la ROM n'est pas sauvegardée)
        let snd = &self.sound;
        put(&mut buf, &snd.ram[..]);
        put(&mut buf, &(snd.bank, snd.latch, snd.time, snd.port_state()));
        put(&mut buf, &snd.chips);
        put(&mut buf, &snd.qsound);

        put(
            &mut buf,
            &(
                self.now,
                self.next_vblank,
                self.frame,
                self.ipl1,
                self.ipl2,
                self.frame_done,
                self.frame_start,
            ),
        );
        put(&mut buf, &self.fb.pixels);
        buf
    }

    /// Restaure un état produit par `save_state` pour le même jeu.
    ///
    /// L'en-tête et le nom du jeu sont vérifiés avant toute modification ; si la suite est
    /// corrompue ou tronquée, une erreur est renvoyée et la machine peut être dans un état
    /// partiel (faire un `reset`).
    pub fn load_state(&mut self, data: &[u8]) -> Result<(), String> {
        if data.len() < 8 || &data[..4] != MAGIC {
            return Err("savestate : magic CPS1 absent".to_string());
        }
        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if version != VERSION {
            return Err(format!("savestate : version {version} non prise en charge"));
        }
        let mut rd = &data[8..];
        let game: String = get(&mut rd)?;
        if game != self.game {
            return Err(format!(
                "savestate : jeu {game}, machine {} (autre jeu)",
                self.game
            ));
        }

        let (d, a, usp, ssp, sr, pc, prefetch): M68 = get(&mut rd)?;
        let z80: puce_z80::CpuState = get(&mut rd)?;

        let ram: Vec<u8> = get(&mut rd)?;
        #[allow(clippy::type_complexity)]
        let (inputs, dips, coinctrl, dial, coinctrl2, eeprom): (
            _,
            _,
            _,
            Option<cps_bus::Upd4701>,
            u16,
            Option<cps_bus::Eeprom93c46>,
        ) = get(&mut rd)?;
        let mut video: cps_gfx::regs::Video = get(&mut rd)?;

        let sound_ram: Vec<u8> = get(&mut rd)?;
        let (bank, latch, time, port) = get(&mut rd)?;
        let mut chips: cps_bus::Chips = get(&mut rd)?;
        let mut qsound: Option<Box<cps_bus::QSoundSide>> = get(&mut rd)?;

        let timing: (u64, u64, u64, bool, bool, bool, u64) = get(&mut rd)?;
        let pixels: Vec<u32> = get(&mut rd)?;
        if !rd.is_empty() {
            return Err(format!("savestate : {} octets en trop à la fin", rd.len()));
        }

        // Tout est lu et valide : application.
        if video.game != self.game {
            return Err("savestate : état vidéo d'un autre jeu".to_string());
        }
        if qsound.is_some() != self.sound.qsound.is_some()
            || eeprom.is_some() != self.bus.eeprom.is_some()
        {
            return Err("savestate : carte son incohérente avec le jeu".to_string());
        }
        if dial.is_some() != self.bus.dial.is_some() {
            return Err("savestate : molettes incohérentes avec le jeu".to_string());
        }
        fill(&mut self.bus.ram[..], ram)?;
        fill(&mut self.sound.ram[..], sound_ram)?;
        fill(&mut self.fb.pixels[..], pixels)?;
        self.cpu.load_state(&CpuState {
            d,
            a,
            usp,
            ssp,
            sr,
            pc,
            prefetch,
        });
        self.z80.load_state(&z80);
        self.bus.inputs = inputs;
        self.bus.dips = dips;
        self.bus.coinctrl = coinctrl;
        self.bus.dial = dial;
        self.bus.coinctrl2 = coinctrl2;
        self.bus.eeprom = eeprom;
        video.relink();
        self.bus.video = video;

        self.sound.bank = bank;
        self.sound.latch = latch;
        self.sound.time = time;
        self.sound.set_port_state(port);
        chips.oki.rom = std::mem::take(&mut self.sound.chips.oki.rom);
        self.sound.chips = chips;
        if let (Some(new), Some(old)) = (qsound.as_mut(), self.sound.qsound.as_mut()) {
            new.opcodes = std::mem::take(&mut old.opcodes);
            new.dsp.take_roms_from(&mut old.dsp);
        }
        self.sound.qsound = qsound;

        (
            self.now,
            self.next_vblank,
            self.frame,
            self.ipl1,
            self.ipl2,
            self.frame_done,
            self.frame_start,
        ) = timing;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::{testutil, Machine};

    /// Programme de T402 : IRQ2 au VBLANK, le gestionnaire incrémente un mot en RAM (0xFF0000).
    fn irq_machine(game_name: &str) -> Machine {
        let program: Vec<(u32, &[u8])> = vec![
            (0x000000, &[0x00, 0xFF, 0x80, 0x00]), // SSP
            (0x000004, &[0x00, 0x00, 0x04, 0x00]), // PC = 0x400
            (0x68, &[0x00, 0x00, 0x05, 0x00]),     // vecteur 26 (IRQ2) = 0x500
            // 0x400 : MOVE #$2000,SR ; BRA.S *
            (0x400, &[0x46, 0xFC, 0x20, 0x00, 0x60, 0xFE]),
            // 0x500 : ADDQ.W #1,$FF0000 ; RTE
            (0x500, &[0x52, 0x79, 0x00, 0xFF, 0x00, 0x00, 0x4E, 0x73]),
        ];
        let mut game = testutil::synthetic_game(&program);
        if game_name != game.info.name {
            game.info = cps_cart::romset::find_game(game_name).expect("jeu connu");
        }
        Machine::new(game)
    }

    fn run(m: &mut Machine, n: usize) {
        for _ in 0..n {
            m.run_frame();
        }
    }

    #[test]
    fn save_load_save_is_identical() {
        let mut m = irq_machine("ghouls");
        run(&mut m, 10);
        let s1 = m.save_state();
        assert_eq!(&s1[..4], b"CPS1");
        m.load_state(&s1).unwrap();
        assert_eq!(m.save_state(), s1);
    }

    #[test]
    fn load_resumes_deterministically() {
        let mut m = irq_machine("ghouls");
        run(&mut m, 100);
        let saved = m.save_state();
        run(&mut m, 50);
        let crc_a = m.framebuffer().crc32();
        let ram_a = m.bus.ram.clone();
        let counter_a = m.bus.read16(0xFF0000, true, true);
        assert_eq!(counter_a, 150, "un incrément par trame");

        m.load_state(&saved).unwrap();
        assert_eq!(m.bus.read16(0xFF0000, true, true), 100);
        run(&mut m, 50);
        assert_eq!(m.framebuffer().crc32(), crc_a);
        assert_eq!(m.bus.ram, ram_a);
    }

    #[test]
    fn load_into_fresh_machine() {
        let mut m = irq_machine("ghouls");
        run(&mut m, 30);
        let saved = m.save_state();
        run(&mut m, 20);
        let end_a = m.save_state();

        let mut fresh = irq_machine("ghouls");
        fresh.load_state(&saved).unwrap();
        run(&mut fresh, 20);
        assert_eq!(fresh.save_state(), end_a);
    }

    #[test]
    fn other_game_is_refused() {
        let mut m = irq_machine("ghouls");
        run(&mut m, 2);
        let saved = m.save_state();
        let mut other = irq_machine("sf2");
        let err = other.load_state(&saved).unwrap_err();
        assert!(err.contains("autre jeu"), "{err}");
    }

    #[test]
    fn truncated_or_bad_data_is_refused() {
        let mut m = irq_machine("ghouls");
        run(&mut m, 2);
        let saved = m.save_state();
        for cut in [0, 7, 8, 20, saved.len() / 2, saved.len() - 1] {
            assert!(m.load_state(&saved[..cut]).is_err(), "coupé à {cut}");
        }
        let mut longer = saved.clone();
        longer.push(0);
        assert!(m.load_state(&longer).is_err());
        assert!(m.load_state(b"XXXX\x01\0\0\0").is_err());
        assert!(m.load_state(b"CPS1\x02\0\0\0").is_err());
        // Après ces refus, l'état d'origine se recharge toujours.
        m.load_state(&saved).unwrap();
    }
}
