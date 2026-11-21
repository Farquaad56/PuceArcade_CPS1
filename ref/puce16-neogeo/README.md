# Modèle de code : Puce16 Neo Geo (même auteur, licence MIT, NE PAS COMPILER ICI)
Copie en lecture seule de fichiers de https://github.com/Farquaad56/Puce16_NeoGeo (état du 2026-11-21) qui fonctionnent avec les mêmes
crates CPU (mêmes révisions). Les cartes indiquent quel fichier reprendre et quoi y changer. Les noms `ng-*` deviennent `cps-*`.
- ng-ym2610/ : moteur FM ymfm porté (fm_tables, fm_operator, fm_channel, fm_engine, common) — base du YM2151 (T300-T306).
- ng-bus/sound.rs : SoundBoard Z80 au T-state par l'API broches de puce-z80 — base de T105.
- ng-core/ : ordonnanceur (sched.rs, host68k.rs), adaptateur ymfm (ym_adapter.rs), mixage (audio_mix.rs), savestates (state.rs).
- ng-testkit/golden.rs : comparateur PNG (T004). ng-common/ : types communs. ng-gui/ : application egui (T500-T505).
