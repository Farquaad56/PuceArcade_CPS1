//! Commandes JAMMA (actives hautes) -> ports d'entrée CPS2 IN0, IN1, IN2 (actifs bas).
//! spec: docs/spec/05 §Ports d'entrée (cps2.cpp INPUT_PORTS cps2_4p4b, cps2_2p6b, qndream)

use cps_bus::Cps2Inputs;
use puce_jamma::inputs::{JammaInputs, PlayerControls};

/// Bit `mask` à 0 si `pressed` (logique active basse).
fn low(pressed: bool, mask: u16) -> u16 {
    if pressed {
        0
    } else {
        mask
    }
}

/// Octet d'un joueur : droite, gauche, bas, haut, B1-B4.
fn player_byte(p: &PlayerControls) -> u16 {
    low(p.right, 0x01)
        | low(p.left, 0x02)
        | low(p.down, 0x04)
        | low(p.up, 0x08)
        | low(p.buttons[0], 0x10)
        | low(p.buttons[1], 0x20)
        | low(p.buttons[2], 0x40)
        | low(p.buttons[3], 0x80)
}

/// IN2 : DO (1, remplacé par l'hôte), test, service, bits 3-7 à 1, start J1-J4, pièces 1-4.
fn in2_port(j: &JammaInputs) -> u16 {
    let mut v = 0x00F9 | low(j.test, 0x0002) | low(j.service, 0x0004);
    for (i, p) in j.players.iter().enumerate() {
        v |= low(p.start, 0x0100 << i);
    }
    for (i, &c) in j.coins.iter().enumerate() {
        v |= low(c, 0x1000 << i);
    }
    v
}

/// Conversion selon la disposition d'entrées CPS2 du jeu (`GameInfo::input`).
pub fn to_cps2_inputs(j: &JammaInputs, input: &str) -> Cps2Inputs {
    let [p1, p2, p3, p4] = &j.players;
    let mut s = Cps2Inputs {
        in0: player_byte(p1) | player_byte(p2) << 8,
        in1: player_byte(p3) | player_byte(p4) << 8,
        in2: in2_port(j),
    };
    match input {
        "cps2_2p6b" | "cps2_2p6bt" | "cybots" => {
            s.in0 |= 0x8080;
            s.in1 = 0xFFC8
                | low(p1.buttons[3], 0x01)
                | low(p1.buttons[4], 0x02)
                | low(p1.buttons[5], 0x04)
                | low(p2.buttons[3], 0x10)
                | low(p2.buttons[4], 0x20);
            // bit 14 : B6 du joueur 2 (la pièce 3 n'est pas lue)
            s.in2 = (s.in2 & !0x4000) | low(p2.buttons[5], 0x4000);
        }
        "qndream" => {
            let quiz = |p: &PlayerControls| {
                low(p.buttons[3], 0x1)
                    | low(p.buttons[2], 0x2)
                    | low(p.buttons[1], 0x4)
                    | low(p.buttons[0], 0x8)
            };
            s.in0 = 0xF0F0 | quiz(p1) | quiz(p2) << 8;
            s.in1 = 0xFFFF;
        }
        _ => {}
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(f: impl Fn(&mut JammaInputs)) -> JammaInputs {
        let mut j = JammaInputs::default();
        f(&mut j);
        j
    }

    fn ports(j: &JammaInputs, input: &str) -> (u16, u16, u16) {
        let s = to_cps2_inputs(j, input);
        (s.in0, s.in1, s.in2)
    }

    #[test]
    fn four_players() {
        let l = "cps2_4p4b";
        assert_eq!(ports(&JammaInputs::default(), l), (0xFFFF, 0xFFFF, 0xFFFF));
        let j = with(|j| {
            j.players[0].right = true;
            j.players[0].buttons[0] = true;
        });
        assert_eq!(ports(&j, l).0, 0xFFEE);
        assert_eq!(ports(&with(|j| j.players[3].up = true), l).1, 0xF7FF);
        assert_eq!(ports(&with(|j| j.coins[0] = true), l).2, 0xEFFF);
        assert_eq!(ports(&with(|j| j.players[1].start = true), l).2, 0xFDFF);
        assert_eq!(ports(&with(|j| j.test = true), l).2, 0xFFFD);
    }

    #[test]
    fn six_buttons() {
        let l = "cps2_2p6b";
        assert_eq!(ports(&JammaInputs::default(), l), (0xFFFF, 0xFFFF, 0xFFFF));
        let p = ports(&with(|j| j.players[0].buttons[3] = true), l);
        assert_eq!((p.0, p.1), (0xFFFF, 0xFFFE));
        assert_eq!(
            ports(&with(|j| j.players[1].buttons[5] = true), l).2,
            0xBFFF
        );
        assert_eq!(
            ports(&with(|j| j.players[1].buttons[4] = true), l).1,
            0xFFDF
        );
        assert_eq!(
            ports(&with(|j| j.coins[2] = true), l).2,
            0xFFFF,
            "pièce 3 non lue"
        );
    }

    #[test]
    fn quiz() {
        let l = "qndream";
        assert_eq!(
            ports(&with(|j| j.players[0].buttons[0] = true), l).0,
            0xFFF7
        );
        assert_eq!(
            ports(&with(|j| j.players[1].buttons[3] = true), l).0,
            0xFEFF
        );
        assert_eq!(ports(&with(|j| j.players[0].up = true), l).0, 0xFFFF);
    }
}
