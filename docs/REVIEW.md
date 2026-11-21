# Checklist de revue (session séparée, contexte vierge)
- [ ] Gate verte, metrics non régressés.
- [ ] Aucun « cycle ajouté en bloc » ; tous les accès passent par `Bus` ; temps en ticks (ADR-001).
- [ ] Aucune constante matérielle sans référence `spec:`.
- [ ] Aucun test affaibli, aucun oracle modifié sans `--bless`.
- [ ] Fichiers ≤ 400 lignes (hors fichiers générés), pas de `unsafe`, pas de `unwrap` hors tests.
- [ ] Fichiers générés à jour (`python tools/gen_cps_tables.py --check`, `extract_romsets.py --check`, `gen_oki_tables.py --check`).
- [ ] STATE.md et board.json cohérents avec le code.
Sortie : liste de défauts classés (bloquant / mineur). Un bloquant recrée une carte.
