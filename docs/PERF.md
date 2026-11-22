# Performances (T700)

Commande : `cargo run -p cps-core --release --example bench -- <jeu> 1800` (Windows, machine de l'utilisateur, 2026-11-22, release, sans son ni affichage).

| Jeu | 68000 | Trames/s | × temps réel | Temps / trame | Rendu / trame | CRC final |
|---|---|---|---|---|---|---|
| ghouls | 10 MHz | 137,5 | 2,31 | 7,27 ms | 2,84 ms (39 %) | 95438C3C |
| sf2 | 10 MHz | 126,6 | 2,12 | 7,90 ms | 3,92 ms (50 %) | 787A88AC |
| sf2ceea | 12 MHz | 115,6 | 1,94 | 8,65 ms | 3,80 ms (44 %) | D12BA66D |

Le CRC final est identique d'une exécution à l'autre (ghouls : 95438C3C deux fois).

Objectif : 120 trames/s (deux fois le temps réel). Atteint pour les jeux à 10 MHz, pas pour sf2ceea (115,6). Le temps réel (59,64 trames/s) est tenu partout avec de la marge.

Profil : le rendu pèse 39 à 50 % d'une trame. `draw_tilemap` appelle `tilemap_pixel` pour chaque pixel visible de chaque couche (86 016 appels par couche, deux fois pour une couche de priorité haute) : lecture GFXRAM, mapper de banque et choix du layout sont refaits pour chaque pixel alors qu'ils ne changent qu'à chaque tuile. Optimisation proposée, sans changement de résultat : carte T703.
