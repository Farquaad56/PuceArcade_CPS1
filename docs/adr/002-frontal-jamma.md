# ADR-002 — GUI = frontal JAMMA réutilisable (puce-jamma-gui)
Statut : accepté (2026-11-21, demande de l'utilisateur)
## Décision
Le GUI du CPS1 n'est plus écrit dans ce dépôt : `cps-gui` implémente le contrat `puce_jamma::JammaBoard` / `BoardDriver`
(https://github.com/Farquaad56/PuceArcade_JammaGui_Crates, copie du contrat : `ref/puce-jamma/00-contrat-jamma.md`) et appelle `puce_jamma_gui::run`.
Dépendances git épinglées (`rev` = SHA du tag v0.1.0, carte J200 de ce dépôt). eframe/cpal/gilrs/rfd/crossbeam ne sont plus des dépendances directes.
## Conséquences
Cartes T500-T505 : adaptateur d'entrées (actives hautes -> registres actifs bas), BoardInfo (384×224, 59,637 Hz, rotation ROT0/ROT270),
DIP décrits (générés par tools/extract_dips.py), savestates, débogage CPU/vidéo. Filtres, overlay, son, manettes, menus : fournis par le frontal.
`cps-gfx::Video::debug_layer_mask` permet au débogueur de masquer des couches.
