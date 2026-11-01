---
id: D-014
statut: 3. Acceptée
date: 2026-11-01
origine: "utilisateur"
remplace: D-012
---
# D-014 · Un seul endroit pour travailler

## Décision
- L'utilisateur parle à Claude **uniquement** dans le projet « Secretaire » (web, application de bureau ou mobile : c'est le même projet).
- Quand une conversation devient longue, on en ouvre une nouvelle dans le même projet. La reprise est automatique grâce aux instructions du projet et à `CLAUDE.md` (T3).
- Les notes vivent dans `notes/` du repo, lisibles dans Obsidian ([[D-013 - Outil de collaboration]]).
- Les preuves sont fournies par la CI GitHub Actions, que Claude lit lui-même.
- L'utilisateur n'ouvre plus de session Claude Code lui-même. Si Claude en a besoin, il s'en occupe ou dit exactement quoi faire.

## Pourquoi
Remplace la version précédente (« pilotage dans le projet, code dans les sessions Claude Code »), devenue inutile. Constat du 2026-11-01 vers 15:30 : depuis une conversation du projet, Claude peut pousser sur le repo et lire l'API GitHub, y compris les résultats de CI.

## Limite
Pas de compilation instantanée dans ces conversations (pas d'accès à crates.io). Chaque essai passe par la CI et prend quelques minutes. Si c'est trop lent, Claude pourra tester une délégation vers une session cloud qui a le réseau.
