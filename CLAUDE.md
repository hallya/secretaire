# CLAUDE.md

Toutes les notes du projet sont dans `notes/`, au format Obsidian (D-017). Lis-les au début de chaque session.

- `notes/Accueil.md` : où on en est et la prochaine action. **Commence par elle.**
- `notes/taches/` : une note par tâche (`T6 - Découper le MVP.md`…), avec ses preuves.
- `notes/decisions/` : une note par décision (`D-008 - Architecture core cli web.md`…).
- `notes/journal/` : une note par jour (`2026-11-01.md`), avec les réussites, les échecs et les leçons.
- `notes/Backlog.base`, `notes/Décisions.base` : vues Obsidian (kanban, tableaux) calculées à partir des propriétés des notes. Ne les modifie que pour changer une vue.

Si l'utilisateur demande « Où en est-on ? », réponds à partir de ces notes : l'état actuel, les décisions en attente (statut `1. À prendre` ou `2. Proposée`), et la prochaine action.
- La prochaine action est la tâche MVP en `2. En cours`.
- S'il n'y en a pas, c'est la première tâche MVP en `1. À faire`, ou celle désignée dans `Accueil.md`.
- Pour lister les statuts : `grep -H "^statut:" notes/taches/*.md`. Pour compter (tâches faites, etc.), utilise une commande comme `grep -l "^statut: 4. Fait" notes/taches/*.md | wc -l`, jamais de tête.

## Format des notes

**Tâche** (`notes/taches/<ID> - <Titre court>.md`) :

```markdown
---
id: T10
statut: 1. À faire          # 1. À faire · 2. En cours · 3. Bloqué · 4. Fait
priorite: MVP               # MVP · nice-to-have
fait_le: 2026-11-02         # seulement quand le statut passe à 4. Fait
---
# T10 · Titre court

Une ou deux phrases : ce que la tâche change, et pour qui.

## Preuve
- **Avant :** …
- **Après :** …

## Liens
- [[D-008 - Architecture core cli web]]
```

**Décision** (`notes/decisions/<ID> - <Titre court>.md`) : propriétés `id`, `statut` (`1. À prendre` · `2. Proposée` · `3. Acceptée` · `4. Remplacée`), `date`, `origine`, et `remplace` ou `remplacee_par` si besoin. Sections : `Décision`, `Pourquoi`, `Alternatives écartées`.

**Journal** (`notes/journal/AAAA-MM-JJ.md`) : une note par jour, des sections par sujet, des lignes courtes qui commencent par ✅, ❌ (avec la cause) ou 💡. Les détails techniques (commits, runs de CI) vont plutôt dans la preuve de la tâche concernée.

## Où l'on travaille (D-014)

- **Un seul endroit pour l'utilisateur : le projet claude.ai « Secretaire ».** Notes, planification, code, push, et lecture des preuves de CI se font depuis ses conversations.
- **Preuves** : CI GitHub Actions (ces conversations n'ont pas accès à crates.io ni à Mistral).
- **Mac en local** : uniquement les tests physiques (dictée au micro réelle, écriture dans une autre application).

## Façon de travailler

- **Scope clair avant de commencer.** Une tâche a une seule responsabilité et se prouve par une preuve avant et une preuve après : un test qui échoue avant le changement et passe après (test unitaire, d'intégration ou e2e).
- **Sinon, découper.** Proposer le découpage et le mode de preuve de chaque morceau, et les faire valider par l'utilisateur avant de coder.
- **Prioriser.** Marquer chaque tâche MVP ou nice-to-have, et avancer d'abord vers le MVP.
- **Dire quand une preuve est hors de portée.** Si tu ne peux pas vérifier une preuve toi-même, dis-le et cherche avec l'utilisateur une autre façon de la fournir.
- **Noter les preuves** dans le message de commit et dans la note de la tâche.

## Mise à jour des notes, à la fin de chaque tâche

1. **Note de la tâche** : changer `statut`, ajouter `fait_le` si elle est faite, compléter la preuve après.
2. **Journal du jour** : ajouter une ligne (✅, ❌ avec sa cause, 💡). Créer la note du jour si elle n'existe pas, et mettre à jour le lien « Dernier jour » dans `Accueil.md`.
3. **Décisions** : créer une note pour toute décision prise ou à prendre. Une décision acceptée ne se modifie pas : on en crée une nouvelle qui la remplace (propriétés `remplace` / `remplacee_par`, statut `4. Remplacée` pour l'ancienne).
4. **Accueil** : mettre à jour le résumé et la prochaine action si elles changent.
5. Commit et push, pour que la prochaine session parte du bon état.

## Règles

- L'utilisateur modifie aussi les notes depuis Obsidian, qui synchronise toutes les minutes (D-013, guide : `notes/guides/obsidian.md`). Avant d'écrire dans `notes/` : `git pull`. Avant d'ajouter une ligne en fin de fichier : vérifier que le fichier se termine par un retour à la ligne. Obsidian peut aussi réenregistrer les fichiers `.base` à sa façon : c'est normal.
- Les secrets (clé Mistral, etc.) ne passent jamais par le chat ni par le repo. En cloud, ils sont dans les « Network secrets » ; pour la CI, dans les secrets GitHub Actions.
