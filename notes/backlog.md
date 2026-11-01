# Backlog

<!-- Une tâche = une seule responsabilité + une preuve avant + une preuve après. -->
<!-- Priorité : MVP (nécessaire au produit minimal) ou nice-to-have (amélioration, personnalisation). -->
<!-- Statut : à faire / en cours / bloqué (raison) / fait (date). -->

## En cours ou à faire

| ID | Tâche | Priorité | Statut | Preuve avant → après |
|---|---|---|---|---|
| T0 | Rattacher le repo aux sessions cloud, en lecture et en écriture | MVP | Bloqué : l'application GitHub de Claude n'est pas installée sur `hallya/secretaire` | Push refusé → commits visibles sur `origin/main` |
| T3 | Règle de reprise : toute nouvelle session retrouve l'état seule | MVP | En cours : `CLAUDE.md` complété le 2026-11-01 | Nouvelle session Claude Code : « Où en est-on ? » donne l'état et la prochaine action ⚠️ test fait par l'utilisateur |
| T4 | CI de preuve (GitHub Actions) : compilation et tests en natif et en `wasm32` | MVP | À faire, après T0 | Aucun run → Claude lit lui-même le statut d'un run |
| T6 | Découper le MVP fonctionnel (dictée → texte, texte → voix) en tâches unitaires avec leurs preuves | MVP | À faire, à valider avec l'utilisateur | Aucune tâche fonctionnelle → liste validée dans ce fichier |
| N1 | Vue tableau du backlog | Nice-to-have | À faire | — |
| N2 | Récapitulatif hebdomadaire planifié | Nice-to-have | À faire | — |

## Fait

| ID | Tâche | Priorité | Fait le | Preuve avant → après |
|---|---|---|---|---|
| T1 | Structure de notes dans le repo (`notes/`, `CLAUDE.md`) | MVP | 2026-11-01 | Repo vide → commit initial `e8b5335` |
| T2 | Migrer décisions, backlog et journal depuis la note du projet claude.ai | MVP | 2026-11-01 | « Voxtral » dans `notes/` : 0 → au moins 1 résultat ; décisions : 0 → 15 |
| T5 | Ouvrir le réseau des sessions Claude Code cloud (crates.io, Rust, Mistral) | Nice-to-have (la CI sert de plan B) | 2026-11-01 | 403 → `serde` compilé, `wasm32` installé, Mistral répond 200 |
| T7 | Licence MIT OR Apache-2.0 (D-010) | MVP | 2026-11-01 | Un seul fichier `LICENSE` → `LICENSE-MIT` et `LICENSE-APACHE` cités dans le README |
