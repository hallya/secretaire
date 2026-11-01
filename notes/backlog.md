# Backlog

<!-- Une tâche = une seule responsabilité + une preuve avant + une preuve après. -->
<!-- Priorité : MVP (nécessaire au produit minimal) ou nice-to-have (amélioration, personnalisation). -->
<!-- Statut : à faire / en cours / bloqué (raison) / fait (date). -->

## En cours ou à faire

| ID | Tâche | Priorité | Statut | Preuve avant → après |
|---|---|---|---|---|
| T8 | Fenêtre ergonomique sur les notes : Obsidian ouvert sur le repo (D-013) | MVP | À faire : installation par l'utilisateur, étapes fournies le 2026-11-01 | Aucune modification de l'utilisateur dans l'historique → un commit de test fait depuis Obsidian, visible sur GitHub (vérifiable par Claude) |
| T4b | Contrôles de qualité en CI : formatage (`cargo fmt --check`) et lints (`clippy`) | Nice-to-have | À faire | Code mal formaté accepté → run rouge sur un défaut de formatage |
| T4c | Exécuter les tests aussi en `wasm32` (pas seulement compiler) | Nice-to-have | À faire | Tests non exécutés en `wasm32` → job de tests `wasm32` vert |
| T4d | CI multi-OS (macOS, Windows), dès que la crate `cli` existe | Nice-to-have | À faire, après la première tâche `cli` | Linux seulement → les trois OS verts |
| T6 | Découper le MVP fonctionnel (dictée → texte, texte → voix) en tâches unitaires avec leurs preuves | MVP | À faire, à valider avec l'utilisateur | Aucune tâche fonctionnelle → liste validée dans ce fichier |
| T8b | Vue tableau (kanban) du backlog dans Obsidian | Nice-to-have | À faire, après T8 | Backlog en tableau → colonnes « À faire / En cours / Fait » affichées |
| T8c | Synchronisation des notes sur l'iPhone | Nice-to-have | À faire, après T8 | Note absente du téléphone → modification visible en moins de 2 minutes |
| N2 | Récapitulatif hebdomadaire planifié | Nice-to-have | À faire | — |

## Fait

| ID | Tâche | Priorité | Fait le | Preuve avant → après |
|---|---|---|---|---|
| T4 | CI de preuve (GitHub Actions) : tests en natif, compilation en `wasm32` ; `notes/` et les `.md` ne déclenchent pas la CI | MVP | 2026-11-01 | 0 run → run `41319231807` rouge sur `c25efd2` (test natif en échec, `wasm32` compile) → run `41319297636` vert sur `c2a03bf` (2 jobs en succès), statuts lus par Claude via l'API GitHub |
| T3 | Règle de reprise : toute nouvelle conversation du projet retrouve l'état seule | MVP | 2026-11-01 | Aucune règle → nouvelle conversation du projet : « Où en est-on ? » a rattaché le repo, lu les notes et donné l'état, les décisions en attente et la prochaine action, conformes aux notes (validé par l'utilisateur) |
| T0 | Rattacher le repo à la conversation du projet, en lecture et en écriture | MVP | 2026-11-01 | Push refusé (403) → `f7f1096`, `b455a4e`, `3a37773` visibles sur `origin/main` |
| T1 | Structure de notes dans le repo (`notes/`, `CLAUDE.md`) | MVP | 2026-11-01 | Repo vide → commit initial `e8b5335` |
| T2 | Migrer décisions, backlog et journal depuis la note du projet claude.ai | MVP | 2026-11-01 | « Voxtral » dans `notes/` : 0 → au moins 1 résultat ; décisions : 0 → 15 |
| T5 | Ouvrir le réseau des sessions Claude Code cloud (crates.io, Rust, Mistral) | Nice-to-have (la CI sert de plan B) | 2026-11-01 | 403 → `serde` compilé, `wasm32` installé, Mistral répond 200 |
| T7 | Licence MIT OR Apache-2.0 (D-010) | MVP | 2026-11-01 | Un seul fichier `LICENSE` → `LICENSE-MIT` et `LICENSE-APACHE` cités dans le README |

<!-- N1 (vue tableau) est remplacée par T8b. -->
