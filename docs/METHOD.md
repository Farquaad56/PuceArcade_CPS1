# Méthode de développement agentique (reprise de Puce16 Neo Geo, v5)

Objectif : permettre à un modèle local (≈27B, 135k tokens) de construire un émulateur CPS1 cycle-exact sans perdre le fil, sans régresser,
avec un score mesurable à chaque étape.

| Problème d'un petit modèle | Réponse |
|---|---|
| Contexte limité | **Carte = unité de contexte** : chaque carte liste exactement quoi lire (plages de lignes), budget ≤ 40k. |
| Hallucination matérielle | **Specs digérées** (`docs/spec`) écrites par Claude depuis MAME, chaque valeur sourcée. Pas de web pendant le dev. |
| « Ça compile donc c'est bon » | **Oracle d'abord** : chaque carte a une commande d'acceptation chiffrée. |
| Dérive silencieuse | **Gate unique** (`scripts/gate.ps1`) + `metrics/*.json` : un score ne peut pas baisser. |
| Oubli entre sessions | `docs/STATE.md` + cases à cocher ; session neuve à chaque carte. |
| Boucles infinies | Règle des 3 essais, puis bissection et `blocked`. |
| Auto-évaluation complaisante | Revue séparée (contexte vierge) à chaque fin de phase : `docs/REVIEW.md`. |

## Règle « Claude pense, le codeur applique »
Architecture, algorithmes, constantes, valeurs de test et commandes d'acceptation sont décidés dans les cartes. Le LLM local applique.
Cartes « port » = traduction 1:1 d'un fichier de `ref/` (MAME cps1*.cpp, okim6295/okiadpcm, ymfm opm). Tables : générées par `tools/*.py`.

## Oracles
- 68000 / Z80 : crates de l'utilisateur, déjà validées (SingleStepTests, ZEXALL…) ; ici test de fumée (T003).
- YM2151 : vecteurs ymfm `tests-data/ym2151/*.bin` (tools/ymfm_opm_golden.cpp). OKI : vecteurs MAME `tests-data/oki/*.bin` (tools/oki_golden.cpp).
- Vidéo : tests unitaires à valeurs calculées à la main (décodage, palette, priorité) + captures PNG de MAME (golden, `tools/mame_golden.ps1`, seuil 2 % pour les jeux).
- Déterminisme : chaque test d'intégration s'exécute deux fois et compare un hash d'état.

## Phases (chaîne linéaire, voir tasks/INDEX.md)
P0 fondations -> P1 ROMs et bus -> P2 vidéo -> P3 audio -> P4 intégration (Ghouls puis SF2) -> P5 GUI (frontal JAMMA, après J200 du dépôt PuceArcade_JammaGui_Crates) -> P6 autres jeux CPS1 -> P7 précision et outils.
Kanban Hermes : `tasks/board.json` (id, phase, deps, statut) importé par `scripts/kanban_import.ps1` (board « cps1 », profil `rust-dev`).
