# Questions ouvertes

## Décisions prises à la conception (2026-11-21, validées par l'utilisateur)
- Jeux prioritaires : ghouls (World) puis sf2 (World 910522), sf2ce (World 920513), sf2hf (World 921209) ; ensuite les autres CPS1 non-QSound (P6).
- Bootlegs (cps1bl_5205.cpp, cps1bl_pic.cpp) et QSound/Kabuki : hors périmètre de ce plan (à replanifier plus tard).
- Référence vidéo : MAME 0.287 (`X:\Mame_0287`, comme Puce16 Neo Geo) ; captures par tools/mame_golden.ps1.

## Points ouverts
- [OUVERT] Q-VBL : ordre exact au début du VBLANK (IRQ2 / recalcul des bases / latch sprites / rendu). Retenu : celui de MAME (screen.cpp vblank_begin :
  callbacks puis frame_update). À confirmer par les golden (T404). Si les sprites sont décalés d'une trame : inverser latch et rendu (ADR).
- [OUVERT] Q-WAIT : wait states ROM/RAM/GFXRAM non émulés par MAME (« TODO: verify wait cycle »). 0 partout ; à revoir si un golden dérive (T701).
- [OUVERT] Q-RESET68K : la séquence de reset de puce-m68k (40 horloges) diffère peut-être de celle de MAME : décalage de quelques horloges au démarrage.
- [OUVERT] Q-AUDIOCLK : l'audio n'est pas l'horloge maître de l'émulation (cadence pilotée par la vidéo, 59,637 Hz) : gestion de dérive dans le frontal JAMMA (carte J106 de PuceArcade_JammaGui_Crates).
- [RÉSOLU] Q-PAR : rapport fourni au frontal JAMMA (BoardInfo.video.aspect = 4:3, 3:4 pour les jeux ROT270) ; choix carré/TV dans le frontal.
- [OUVERT] Q-FRAMENO : numéro de trame passé au rendu des étoiles (`screen.frame_number()` de MAME = trames terminées avant la mise à jour ?) : retenu `frame` avant incrément (T603), à vérifier sur forgottn.
- [OUVERT] Q-4P : joueurs 3-4 (captcomm, mercs 3 joueurs…) : in2/in3 de la carte C non câblés à JammaInputs.players[2..4] (carte à planifier en P6).
