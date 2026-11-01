---
id: T4
statut: 4. Fait
priorite: MVP
fait_le: 2026-11-01
---
# T4 · CI de preuve

GitHub Actions : tests en natif, compilation en `wasm32`. `notes/` et les fichiers `.md` ne déclenchent pas la CI.

## Preuve
- **Avant :** 0 run ; puis run `41319231807` rouge sur `c25efd2` (test natif en échec, `wasm32` compile).
- **Après :** Run `41319297636` vert sur `c2a03bf` (2 jobs en succès). Statuts lus par Claude via l'API GitHub.

## Liens
- Suites : [[T4b - Formatage et lints en CI]], [[T4c - Tests exécutés en wasm32]], [[T4d - CI multi-OS]]
