//! Commandes JAMMA (actives hautes) -> ports d'entrée CPS1 (actifs bas).
//! spec: docs/spec/01 §Entrées

use cps_common::InputState;
use puce_jamma::inputs::{JammaInputs, PlayerControls};
use puce_jamma::util::active_low;

/// Octet d'un joueur dans IN1 : droite, gauche, bas, haut, boutons 1-3 ; bit 7 toujours à 1.
fn in1_byte(p: &PlayerControls) -> u8 {
    0x80 | active_low(p.right, 0x01)
        | active_low(p.left, 0x02)
        | active_low(p.down, 0x04)
        | active_low(p.up, 0x08)
        | active_low(p.buttons[0], 0x10)
        | active_low(p.buttons[1], 0x20)
        | active_low(p.buttons[2], 0x40)
}

/// Boutons 4 à 6 d'un joueur (kick harness CPS) sur 3 bits.
fn kick_bits(p: &PlayerControls) -> u8 {
    active_low(p.buttons[3], 0x01) | active_low(p.buttons[4], 0x02) | active_low(p.buttons[5], 0x04)
}

pub fn to_cps_inputs(j: &JammaInputs) -> InputState {
    let [p1, p2, ..] = &j.players;
    // spec: docs/spec/01 §Entrées — IN0 : pièce 1, pièce 2, service, (1), start 1, start 2, test, (1)
    let in0 = 0x88
        | active_low(j.coins[0], 0x01)
        | active_low(j.coins[1], 0x02)
        | active_low(j.service, 0x04)
        | active_low(p1.start, 0x10)
        | active_low(p2.start, 0x20)
        | active_low(j.test, 0x40);
    let in1 = u16::from(in1_byte(p2)) << 8 | u16::from(in1_byte(p1));
    // IN2 : boutons 4-6 du joueur 1 (bits 0-2) et du joueur 2 (bits 4-6), bits 3 et 7 à 1
    let in2 = 0x88 | kick_bits(p1) | kick_bits(p2) << 4;
    InputState {
        in0,
        in1,
        in2,
        in3: 0xFF,
    }
}

/// Octet d'un joueur 3 ou 4 (ports IN2 / IN3 de cps1_3players / cps1_4players) : direction,
/// boutons 1-2, pièce, start.
fn extra_player_byte(p: &PlayerControls, coin: bool) -> u8 {
    active_low(p.right, 0x01)
        | active_low(p.left, 0x02)
        | active_low(p.down, 0x04)
        | active_low(p.up, 0x08)
        | active_low(p.buttons[0], 0x10)
        | active_low(p.buttons[1], 0x20)
        | active_low(coin, 0x40)
        | active_low(p.start, 0x80)
}

/// Entrées selon le nombre de joueurs de la borne : à partir de 3 joueurs, IN2 (F1C000) est le
/// joueur 3 et IN3 (F1C002) le joueur 4 ; avec 4 joueurs, le bouton 3 des joueurs 3 et 4 est en
/// IN1 bit 7 et bit 15 (slammast). spec: cps1.cpp INPUT_PORTS cps1_3players / cps1_4players / slammast
pub fn to_cps_inputs_for(j: &JammaInputs, players: u8) -> InputState {
    let mut s = to_cps_inputs(j);
    if players >= 3 {
        s.in2 = extra_player_byte(&j.players[2], j.coins[2]);
        s.in3 = if players >= 4 {
            extra_player_byte(&j.players[3], j.coins[3])
        } else {
            0xFF
        };
    }
    if players >= 4 {
        s.in1 = (s.in1 & 0x7F7F)
            | u16::from(active_low(j.players[2].buttons[2], 0x80))
            | u16::from(active_low(j.players[3].buttons[2], 0x80)) << 8;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_from_the_card() {
        assert_eq!(
            to_cps_inputs(&JammaInputs::default()),
            InputState::default()
        );
        let mut j = JammaInputs::default();
        j.players[0].right = true;
        j.players[0].buttons[0] = true;
        assert_eq!(to_cps_inputs(&j).in1, 0xFFEE);
        let mut j = JammaInputs::default();
        j.players[1].up = true;
        j.players[1].buttons[2] = true;
        assert_eq!(to_cps_inputs(&j).in1, 0xB7FF);
        let mut j = JammaInputs::default();
        j.players[0].buttons[3] = true;
        j.players[1].buttons[5] = true;
        assert_eq!(to_cps_inputs(&j).in2, 0xBE);
        let mut j = JammaInputs::default();
        j.coins[0] = true;
        j.players[0].start = true;
        assert_eq!(to_cps_inputs(&j).in0, 0xEE);
        let j = JammaInputs {
            test: true,
            ..Default::default()
        };
        assert_eq!(to_cps_inputs(&j).in0, 0xBF);
    }

    #[test]
    fn three_and_four_players() {
        let mut j = JammaInputs::default();
        j.players[2].up = true;
        j.players[2].start = true;
        j.coins[3] = true;
        let s = to_cps_inputs_for(&j, 3);
        assert_eq!((s.in2, s.in3), (0x77, 0xFF));
        let s = to_cps_inputs_for(&j, 4);
        assert_eq!((s.in2, s.in3), (0x77, 0xBF));
        j.players[3].buttons[2] = true;
        assert_eq!(to_cps_inputs_for(&j, 4).in1, 0x7FFF);
        assert_eq!(
            to_cps_inputs_for(&j, 2),
            to_cps_inputs(&j),
            "2 joueurs : inchangé"
        );
    }

    #[test]
    fn extra_players_are_ignored() {
        let mut j = JammaInputs::default();
        j.players[2].buttons[0] = true;
        j.coins[2] = true;
        assert_eq!(to_cps_inputs(&j), InputState::default());
    }
}
