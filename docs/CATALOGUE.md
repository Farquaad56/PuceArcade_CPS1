# Catalogue CPS1 — validation contre MAME

Généré par `cargo test -p cps-core --release --test catalogue -- --ignored catalogue`.
Trames 600 et 1500 comparées aux golden MAME (seuil 2 % de pixels différents).

| Jeu | Titre | Résultat |
|---|---|---|
| forgottn | Forgotten Worlds (World, newer) | ROM refusées (fichier manquant : lw40.12f) |
| forgottnua | Forgotten Worlds (USA, B-Board 88618B-2, Rev. A) | OK (écart max 0.000 %) |
| lostwrld | Lost Worlds (Japan) | OK (écart max 0.000 %) |
| ghouls | Ghouls'n Ghosts (World) | OK (écart max 0.000 %) |
| ghoulsu | Ghouls'n Ghosts (USA) | OK (écart max 0.000 %) |
| daimakai | Daimakaimura (Japan) | OK (écart max 0.000 %) |
| strider | Strider (USA, B-Board 89624B-2) | OK (écart max 0.000 %) |
| striderj | Strider Hiryu (Japan) | ROM refusées (fichier manquant : sth_36.12f) |
| dynwar | Dynasty Wars (USA, B-Board 89624B-?) | OK (écart max 0.000 %) |
| dynwarj | Tenchi wo Kurau (Japan) | OK (écart max 0.000 %) |
| willow | Willow (World) | ROM refusées (fichier manquant : wle_30.11f) |
| willowu | Willow (USA) | OK (écart max 1.330 %) |
| willowj | Willow (Japan) | ROM refusées (fichier manquant : wl_34.10f) |
| unsquad | U.N. Squadron (USA) | OK (écart max 0.000 %) |
| area88 | Area 88 (Japan) | OK (écart max 0.000 %) |
| ffight | Final Fight (World, set 1) | OK (écart max 0.000 %) |
| ffightu | Final Fight (USA, set 1) | OK (écart max 0.000 %) |
| ffightua | Final Fight (USA 900112) | OK (écart max 0.000 %) |
| ffightub | Final Fight (USA 900424) | ROM refusées (fichier manquant : ffu_30_3.11f) |
| ffightj | Final Fight (Japan) | OK (écart max 0.000 %) |
| ffightj1 | Final Fight (Japan 900112) | ROM refusées (fichier manquant : ffj_36.12f) |
| ffightj2 | Final Fight (Japan 900305) | ROM refusées (fichier manquant : ffj_36a.12f) |
| 1941 | 1941: Counter Attack (World 900227) | OK (écart max 0.000 %) |
| 1941r1 | 1941: Counter Attack (World) | OK (écart max 0.000 %) |
| 1941u | 1941: Counter Attack (USA 900227) | OK (écart max 0.000 %) |
| 1941j | 1941: Counter Attack (Japan) | OK (écart max 0.000 %) |
| mercs | Mercs (World 900302) | OK (écart max 0.000 %) |
| mercsu | Mercs (USA 900608) | ROM refusées (fichier manquant : so2_30a.11f) |
| mercsj | Senjou no Ookami II (Japan 900302) | OK (écart max 0.000 %) |
| mtwins | Mega Twins (World 900619) | OK (écart max 0.000 %) |
| chikij | Chiki Chiki Boys (Japan 900619) | OK (écart max 0.000 %) |
| msword | Magic Sword: Heroic Fantasy (World 900725) | OK (écart max 0.000 %) |
| mswordr1 | Magic Sword: Heroic Fantasy (World 900623) | OK (écart max 0.000 %) |
| mswordu | Magic Sword: Heroic Fantasy (USA 900725) | OK (écart max 0.000 %) |
| mswordj | Magic Sword: Heroic Fantasy (Japan 900623) | OK (écart max 0.000 %) |
| cawing | Carrier Air Wing (World 901012) | OK (écart max 0.000 %) |
| cawingr1 | Carrier Air Wing (World 901009) | OK (écart max 0.000 %) |
| cawingu | Carrier Air Wing (USA 901130) | ROM refusées (fichier manquant : cau_36.12f) |
| cawingj | U.S. Navy (Japan 901012) | OK (écart max 0.000 %) |
| nemo | Nemo (World 901130) | OK (écart max 0.000 %) |
| nemoj | Nemo (Japan 901120, 88622B-3 ROM board) | OK (écart max 0.000 %) |
| sf2 | Street Fighter II: The World Warrior (World 910522) | OK (écart max 0.037 %) |
| sf2eb | Street Fighter II: The World Warrior (World 910214) | OK (écart max 0.037 %) |
| sf2ua | Street Fighter II: The World Warrior (USA 910206) | OK (écart max 0.038 %) |
| sf2ub | Street Fighter II: The World Warrior (USA 910214) | OK (écart max 0.038 %) |
| sf2ud | Street Fighter II: The World Warrior (USA 910318) | OK (écart max 0.038 %) |
| sf2ue | Street Fighter II: The World Warrior (USA 910228) | OK (écart max 0.038 %) |
| sf2uf | Street Fighter II: The World Warrior (USA 910411) | OK (écart max 0.038 %) |
| sf2ui | Street Fighter II: The World Warrior (USA 910522, Rev. I) | OK (écart max 0.038 %) |
| sf2uk | Street Fighter II: The World Warrior (USA 911101) | OK (écart max 0.038 %) |
| sf2j | Street Fighter II: The World Warrior (Japan 911210, CPS-B-13) | OK (écart max 0.023 %) |
| sf2ja | Street Fighter II: The World Warrior (Japan 910214) | OK (écart max 0.023 %) |
| sf2jc | Street Fighter II: The World Warrior (Japan 910306) | OK (écart max 0.023 %) |
| 3wonders | Three Wonders (World 910520) | OK (écart max 0.000 %) |
| wonder3 | Wonder 3 (Japan 910520) | OK (écart max 0.000 %) |
| kod | The King of Dragons (World 910805) | ROM refusées (fichier manquant : kde_30a.11e) |
| kodr1 | The King of Dragons (World 910711) | OK (écart max 0.000 %) |
| kodu | The King of Dragons (USA 910910) | OK (écart max 0.000 %) |
| kodj | The King of Dragons (Japan 910805, B-Board 90629B-3) | OK (écart max 0.000 %) |
| captcomm | Captain Commando (World 911202) | ROM refusées (fichier manquant : cce_23f.8f) |
| captcommr1 | Captain Commando (World 911014) | OK (écart max 0.000 %) |
| knights | Knights of the Round (World 911127) | OK (écart max 0.000 %) |
| knightsu | Knights of the Round (USA 911127) | OK (écart max 0.000 %) |
| knightsj | Knights of the Round (Japan 911127, B-Board 91634B-2) | ROM refusées (fichier manquant : kr_23j.8f) |
| sf2ce | Street Fighter II': Champion Edition (World 920513) | ROM refusées (fichier manquant : s92e_23b.8f) |
| sf2ceea | Street Fighter II': Champion Edition (World 920313) | OK (écart max 0.000 %) |
| sf2ceua | Street Fighter II': Champion Edition (USA 920313) | OK (écart max 0.000 %) |
| sf2ceub | Street Fighter II': Champion Edition (USA 920513) | OK (écart max 0.000 %) |
| sf2ceuc | Street Fighter II': Champion Edition (USA 920803) | OK (écart max 0.000 %) |
| sf2rb3 | Street Fighter II': Champion Edition (Rainbow, bootleg, set 3) | OK (écart max 0.000 %) |
| sf2red | Street Fighter II': Champion Edition (Red Wave, bootleg, set 1) | écart 94.81 % à la trame 1500 |
| sf2v004 | Street Fighter II': Champion Edition (V004, bootleg) | écart 38.02 % à la trame 600 |
| sf2accp2 | Street Fighter II': Champion Edition (Accelerator Pt.II, bootleg) | OK (écart max 0.000 %) |
| cworld2j | Adventure Quiz Capcom World 2 (Japan 920611) | OK (écart max 0.000 %) |
| varth | Varth: Operation Thunderstorm (World 920714) | OK (écart max 0.000 %) |
| varthr1 | Varth: Operation Thunderstorm (World 920612) | OK (écart max 0.000 %) |
| varthu | Varth: Operation Thunderstorm (USA 920612) | OK (écart max 0.000 %) |
| varthj | Varth: Operation Thunderstorm (Japan 920714) | OK (écart max 0.000 %) |
| qad | Quiz & Dragons: Capcom Quiz Game (USA 920701) | OK (écart max 0.000 %) |
| wof | Warriors of Fate (World 921031) | ROM refusées (fichier manquant : tk2e_23c.8f) |
| wofr1 | Warriors of Fate (World 921002) | OK (écart max 0.000 %) |
| wofu | Warriors of Fate (USA 921031) | OK (écart max 0.000 %) |
| wofa | Sangokushi II (Asia 921005) | OK (écart max 0.000 %) |
| wofj | Tenchi wo Kurau II: Sekiheki no Tatakai (Japan 921031) | OK (écart max 0.000 %) |
| sf2hf | Street Fighter II': Hyper Fighting (World 921209) | OK (écart max 0.000 %) |
| dino | Cadillacs and Dinosaurs (World 930201) | OK (écart max 0.000 %) |
| dinou | Cadillacs and Dinosaurs (USA 930201) | ROM refusées (fichier manquant : cdu_22a.7f) |
| dinoj | Cadillacs: Kyouryuu Shin Seiki (Japan 930201) | OK (écart max 0.000 %) |
| punisher | The Punisher (World 930422) | OK (écart max 0.000 %) |
| slammast | Saturday Night Slam Masters (World 930713) | OK (écart max 0.000 %) |
| mbomberj | Muscle Bomber: The Body Explosion (Japan 930713) | ROM refusées (fichier manquant : mbj_22b.7f) |
| mbombrd | Muscle Bomber Duo: Ultimate Team Battle (World 931206) | OK (écart max 0.000 %) |
| mbombrdj | Muscle Bomber Duo: Heat Up Warriors (Japan 931206) | ROM refusées (fichier manquant : mb_qa.5k) |
| pnickj | Pnickies (Japan 940608) | OK (écart max 0.000 %) |
| megaman | Mega Man: The Power Battle (CPS1, USA 951006) | OK (écart max 0.000 %) |
| megamana | Mega Man: The Power Battle (CPS1, Asia 951006) | OK (écart max 0.000 %) |
| rockmanj | Rockman: The Power Battle (CPS1, Japan 950922) | OK (écart max 0.000 %) |

**78 OK**, 2 en échec, 17 non comparés (ROM ou golden indisponibles) sur 97 jeux présents.
