# Oracles (générés par Claude, versionnés dans tests-data/)
Régénération : `powershell tools/oracles.ps1` (g++ requis). `git diff tests-data/` doit rester vide.

## YM2151 — tests-data/ym2151/*.bin
Pilote `tools/ymfm_opm_golden.cpp` sur ymfm (`ref/ymfm`, BSD-3). Format : 1000 échantillons × 2 voies i16 LE [gauche, droite] = 4000 octets.
Scénarios : silence, fm_algo7, fm_algo4_keyoff, lfo, noise, timers (+ timers_status.bin = `read(1)` après chaque échantillon,
timers_irq.bin = état IRQ 0/1 après chaque échantillon), mix. Les écritures de chaque scénario sont dans `main()` du .cpp
(fonction `r(w, avant, registre, valeur)` = écriture adresse (offset 0) puis donnée (offset 1) « avant l'échantillon `avant` » ;
`voice(...)` = programmation d'une voix complète).
Conventions (identiques à l'adaptateur Rust, cartes T306/T307) : 1 échantillon / 64 horloges YM ; l'échantillon n (n ≥ 1) est produit à ym_time = n×64 ;
avant lui : (1) écritures planifiées « avant n » à ym_time = (n-1)×64, (2) timers dont l'échéance ≤ n×64 dans l'ordre chronologique
(`engine_timer_expired`, ym_time = échéance), (3) ym_time = n×64 puis `generate(1)`. `set_timer(t,d)` : échéance = ym_time + d (d < 0 : arrêt).
`set_busy_end(c)` : busy jusqu'à ym_time + c. Sorties écrêtées à [-32768, 32767].

## OKI MSM6295 — tests-data/oki/*.bin
Pilote `tools/oki_golden.cpp` : `okiadpcm.cpp` de MAME compilé tel quel (shim `tools/oki_shim/emu.h`), logique des voix recopiée de okim6295.cpp.
Format : 2000 échantillons i32 LE (somme des 4 voix de `clock(nibble) × numérateur_volume`) + `<nom>_status.bin` (2000 octets, `read()` après chaque échantillon).
Avant l'échantillon n (n ≥ 1) : commandes planifiées « avant n », puis génération. ROM : `rom(a) = (h ^ (h >> 15)) & 0xFF`, h = (a & 0x3FFFF) × 0x9E3779B1 (u32),
sauf la table d'échantillons (a < 0x400) : échantillon s : start = 0x400 + s×0x100, stop = start + 0x7F + s (s = 5 : stop = start, invalide),
octets 6-7 de chaque entrée = 0. Scénarios : silence, voice1, two_voices, stop, busy_ignore, invalid, all4 (détail dans `main()`).

## Captures MAME (golden, NON versionnées)
`powershell tools/mame_golden.ps1 -Game <jeu> -Frames 120,600,1500,3000 -Out tests-data/golden/<jeu>` (MAME 0.287, `X:\Mame_0287\mame.exe`,
une exécution par trame, configuration vierge, son coupé). Sortie : `f<N>.png` (384×224).
