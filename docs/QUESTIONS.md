# Questions ouvertes

## Décisions prises à la conception (2026-11-21, validées par l'utilisateur)
- Jeux prioritaires : ghouls (World) puis sf2 (World 910522), sf2ce (World 920513), sf2hf (World 921209) ; ensuite les autres CPS1 non-QSound (P6).
- Bootlegs (cps1bl_5205.cpp, cps1bl_pic.cpp) et QSound/Kabuki : hors périmètre de ce plan (à replanifier plus tard).
- Référence vidéo : MAME 0.287 (`X:\Mame_0287`, comme Puce16 Neo Geo) ; captures par tools/mame_golden.ps1.

## Points ouverts
- [RÉSOLU, ADR-004] Q-VBL : MAME rend AVANT les callbacks de VBLANK (IRQ2, bases, copie des sprites) ; ordre corrigé le 2026-11-22 (ghouls identique au pixel près, catalogue 41 -> 43 OK). Texte d'origine : ordre exact au début du VBLANK (IRQ2 / recalcul des bases / latch sprites / rendu). Retenu : celui de MAME (screen.cpp vblank_begin :
  callbacks puis frame_update). À confirmer par les golden (T404). Si les sprites sont décalés d'une trame : inverser latch et rendu (ADR).
- [OUVERT] Q-WAIT : wait states ROM/RAM/GFXRAM non émulés par MAME (« TODO: verify wait cycle »). 0 partout ; à revoir si un golden dérive (T701).
- [OUVERT] Q-RESET68K : la séquence de reset de puce-m68k (40 horloges) diffère peut-être de celle de MAME : décalage de quelques horloges au démarrage.
- [OUVERT] Q-AUDIOCLK : l'audio n'est pas l'horloge maître de l'émulation (cadence pilotée par la vidéo, 59,637 Hz) : gestion de dérive dans le frontal JAMMA (carte J106 de PuceArcade_JammaGui_Crates).
- [RÉSOLU] Q-PAR : rapport fourni au frontal JAMMA (BoardInfo.video.aspect = 4:3, 3:4 pour les jeux ROT270) ; choix carré/TV dans le frontal.
- [OUVERT] Q-FRAMENO : numéro de trame passé au rendu des étoiles (`screen.frame_number()` de MAME = trames terminées avant la mise à jour ?) : retenu `frame` avant incrément (T603), à vérifier sur forgottn.
- [OUVERT] Q-4P : joueurs 3-4 (captcomm, mercs 3 joueurs…) : in2/in3 de la carte C non câblés à JammaInputs.players[2..4] (carte à planifier en P6).
- [RÉSOLU] Q-ROMSETS : `roms/` (137 zips, anciens noms ; 76 romsets CPS1 complets, vérification CRC du 2026-11-21). sf2ce.zip contient sf2ceea (920313), cible Champion Edition (T406).
  sf2hf : jeu parent autonome dans MAME 0.287 ; `roms/sf2hf.zip` reconstruit le 2026-11-22 (ROM 68000 de l'ancien zip, gardé en sf2hf_ancien.zip, + ROM partagées de sf2ce.zip).
- [OUVERT] Q-SF2HF-3000 : sf2hf a ~2 trames d'avance sur MAME à la trame 3000 (trames 120/600/1500 identiques) : à traiter avec Q-VBL / Q-RESET68K / Q-WAIT en T701. Mesure du 2026-11-22 : MAME 2993 == nous 2990 au pixel près, puis avance constante de 2 trames : un seul événement (trame de latence chez MAME vers 2990), pas une dérive.
- [OUVERT] Q-JAMMA-SHA : le crate puce-jamma épinglé (rev 916eace, tag v0.1.0 de J200) est vide (lib.rs = `#![forbid(unsafe_code)]` uniquement). Les types JammaInputs et util::active_low n'existent pas dans ce commit. T500 bloquée jusqu'à ce que le dépôt PuceArcade_JammaGui_Crates publie une révision contenant l'implémentation du contrat (modules inputs, util, etc.).
  Réponse (Claude, 2026-11-22) : le frontal JAMMA n'avait jamais été importé dans Hermes. Board « jamma » importé (J000-J200) ; T500 reprendra quand J200 sera done, avec le SHA noté dans le STATE.md de PuceArcade_JammaGui_Crates. En attendant, T600 dépend de T408 (P6/P7 en parallèle). Travail inachevé de T500 mis de côté : `git stash list` (« T500 inacheve »).

- T500b (résolu par l'orchestrateur) : la carte attendait 859-860 échantillons après la 1re trame (0 -> début VBLANK). Le cœur termine chaque `run_frame` sur un multiple de TICKS_PER_FRAME (now = 84 510 720 après la 1re trame, Z80 synchrone à 1 période près) et ses tests attendent 937/938. Test aligné sur le cœur : 937..=938.
- T505 (même cause que T500b) : 60 trames donnent 60 × 84 510 720 / 90 112 = 56 268-56 269 échantillons, pas 56 191-56 192. Test aligné sur le cœur.
- T603 (résolu par l'orchestrateur) : la carte lisait l'id CPS-B de sf2ee en 0x8001F2 (offset 0x32 des CPS-B-21). sf2ee utilise CPS-B-18, id en offset 0x10 (cps1_v.cpp, CPS_B_18) : test en 0x8001D0 == 0x0408 et 0x800150 == 0.
- T604 (résolu par l'orchestrateur) : libtest n'accepte pas `-- --list-games` ; la liste est écrite par le test ignoré `list_games` (`cargo test -p cps-core --release --test catalogue -- --ignored list_games`).
- QSound (résolu par l'orchestrateur, ADR-005) : la référence ref/mame-cps1/cps1.cpp nomme `audiorom_raw` la région que MAME 0.287 nomme `user1` (ROM Z80 chiffrée, ROM_COPY avant kabuki_setup) ; les deux noms sont acceptés. La ROM interne du DSP (dl-1425.bin) est fournie par l'utilisateur dans qsound.zip, jamais dans le dépôt.
