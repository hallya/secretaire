---
id: D-017
statut: 3. Acceptée
date: 2026-11-01
origine: "utilisateur (pilote T9 validé)"
remplace: D-011
---
# D-017 · Format des notes

## Décision
Les notes restent dans `notes/` du repo, avec `CLAUDE.md` comme point d'entrée, mais au format Obsidian :
- `notes/Accueil.md` : où on en est, prochaine action, kanban et décisions ;
- `notes/taches/` : une note par tâche, avec les propriétés `id`, `statut`, `priorite`, `fait_le` ;
- `notes/decisions/` : une note par décision, avec les propriétés `id`, `statut`, `date`, `origine` ;
- `notes/journal/` : une note par jour ;
- `notes/Backlog.base` et `notes/Décisions.base` : vues kanban et tableau.

## Pourquoi
L'utilisateur trouvait les grands tableaux et les longs fichiers uniques difficiles à lire dans Obsidian. Pilote jugé « beaucoup mieux » le 2026-11-01.

## Statuts
Numérotés pour que les colonnes s'affichent dans l'ordre. Tâches : `1. À faire`, `2. En cours`, `3. Bloqué`, `4. Fait`. Décisions : `1. À prendre`, `2. Proposée`, `3. Acceptée`, `4. Remplacée`.
