# ADR 007 — Horloge du 68000 du CPS2 : 25 200 / 59 ticks par cycle
- Contexte : MAME 0.287 déclare le 68000 du CPS2 à 16 MHz puis applique `set_clock_scale(0.7375)` (cps2.cpp l.1804, l.11013),
  soit 11,8 MHz effectifs. 5 040 000 000 / 11 800 000 = 25 200 / 59 ≈ 427,12 : pas un nombre entier de ticks (ADR-001).
- Décision : la durée d'un cycle 68000 est une fraction `num / den` (CPS1 : 504/1 ou 420/1 ; CPS2 : 25 200/59). L'hôte garde un
  reste entier : à chaque horloge, `acc += num ; now += acc / den ; acc %= den`. Le reste fait partie de l'état sauvegardé.
  `cps_cart::Game` porte `ticks_per_68k` (numérateur) et `ticks_68k_den` (dénominateur, 1 pour le CPS1).
- Conséquences : aucune dérive (59 cycles = exactement 25 200 ticks), déterminisme conservé, CPS1 inchangé (den = 1 : même suite
  de ticks qu'avant). L'unité de temps de l'ADR-001 ne change pas.
- Cartes impactées : T802 (cps-cart), T830 (cps-core).
