# Performances (T700)

Commande : `cargo run -p cps-core --release --example bench -- <jeu> 1800` (Windows, machine de l'utilisateur, 2026-11-22, release, sans son ni affichage).

| Jeu | 68000 | Trames/s | × temps réel | Temps / trame | Rendu / trame | CRC final |
|---|---|---|---|---|---|---|
| ghouls | 10 MHz | 163.3 | 2.74 | 6.13 ms | 1.23 ms (20 %) | 5EB486BC |
| sf2 | 10 MHz | 166.4 | 2.79 | 6.01 ms | 1.62 ms (27 %) | 787A88AC |
| sf2ceea | 12 MHz | 141.3 | 2.37 | 7.08 ms | 1.69 ms (24 %) | D12BA66D |

Le CRC final est identique d'une exécution à l'autre et correspond aux valeurs de référence T700.

Objectif : 120 trames/s (deux fois le temps réel). Atteint pour les trois jeux après optimisation T703 (décodage des infos de tuile une fois par tuile au lieu d'une fois par pixel). Le rendu est passé de 39-50 % à 20-27 % du temps de trame.
