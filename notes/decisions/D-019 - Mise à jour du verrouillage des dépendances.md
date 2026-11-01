---
id: D-019
statut: 3. Acceptée
date: 2026-11-01
origine: Claude (obstacle rencontré sur T11)
---
# D-019 · Mise à jour du verrouillage des dépendances

## Problème
Dès T11, le code a besoin de bibliothèques externes (lecture du JSON, puis décodage de l'audio). Ajouter une dépendance exige de mettre à jour `Cargo.lock`, et la CI refuse un `Cargo.lock` périmé (`--locked`). Or ni les conversations du projet ni la machine virtuelle du Mac n'ont accès à crates.io (vérifié le 2026-11-01).

## Décision
Un job de CI dédié met à jour `Cargo.lock` et le commite lui-même sur `main` quand une dépendance change (`cargo update --workspace`). Ce job n'est autorisé à écrire que sur les pushs vers `main`, jamais depuis une pull request. Les tests tournent sans `--locked` pour ce seul push, puis avec le `Cargo.lock` à jour pour les suivants.

## Pourquoi
Claude reste autonome pour ajouter une dépendance, sans action de l'utilisateur, et `Cargo.lock` reste à jour dans le repo. C'est le même principe que les outils comme Renovate ou Dependabot.

## Alternatives
- **Retirer `--locked` de la CI** : plus simple, mais `Cargo.lock` ne serait plus jamais à jour dans le repo.
- **L'utilisateur met à jour `Cargo.lock` depuis son Mac** à chaque nouvelle dépendance : fiable, mais casse l'autonomie de Claude.
- **Écrire nous-mêmes la lecture du JSON et le décodage de l'audio** : réinventer des bibliothèques éprouvées, plus de risques d'erreur.

## Mise en œuvre (2026-11-01)
- **Avant :** `serde_json` ajouté sans mettre à jour `Cargo.lock` (`fbb35c6`) → run `41341074116` rouge : « cannot update the lock file … because --locked was passed ».
- **Après :** job `lockfile` ajouté (`56cad35`) → run `41341140364` vert (3 jobs), puis commit `bebc119` de github-actions[bot] : `Cargo.lock` contient `serde_json` 1.0.154.
- Conséquence : le code ne compile plus dans les conversations du projet dès qu'il a une dépendance. Toute compilation passe par la CI.
