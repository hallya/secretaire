# CLAUDE.md

Toutes les notes du projet sont dans `notes/`. Lis-les au début de chaque session.

- `notes/backlog.md` : les tâches en cours, à faire et faites, avec leurs preuves. **Commence par lui** : la première tâche « en cours » ou « à faire » est la prochaine action.
- `notes/decisions.md` : les décisions prises, proposées et à prendre (D-001, D-002, …)
- `notes/journal.md` : le journal daté des réussites, des échecs et des leçons

Si l'utilisateur demande « Où en est-on ? », réponds à partir de ces trois fichiers : l'état actuel, les décisions en attente, et la prochaine action.

## Où l'on travaille (D-014)

- **Un seul endroit pour l'utilisateur : le projet claude.ai « Secretaire ».** Notes, planification, code, push, et lecture des preuves de CI se font depuis ses conversations.
- **Preuves** : CI GitHub Actions (ces conversations n'ont pas accès à crates.io ni à Mistral).
- **Mac en local** : uniquement les tests physiques (dictée au micro réelle, écriture dans une autre application).

## Façon de travailler

- **Scope clair avant de commencer.** Une tâche a une seule responsabilité et se prouve par une preuve avant et une preuve après : un test qui échoue avant le changement et passe après (test unitaire, d'intégration ou e2e).
- **Sinon, découper.** Proposer le découpage et le mode de preuve de chaque morceau, et les faire valider par l'utilisateur avant de coder.
- **Prioriser.** Marquer chaque tâche MVP ou nice-to-have, et avancer d'abord vers le MVP.
- **Dire quand une preuve est hors de portée.** Si tu ne peux pas vérifier une preuve toi-même, dis-le et cherche avec l'utilisateur une autre façon de la fournir.
- **Noter les preuves** dans le message de commit et dans le journal.

## Mise à jour des notes, à la fin de chaque tâche

1. `backlog.md` : changer le statut de la tâche et noter sa preuve après.
2. `journal.md` : ajouter une ligne datée (✅ réussite, ❌ échec avec sa cause, 💡 leçon).
3. `decisions.md` : ajouter toute décision prise ou à prendre. Une décision acceptée ne se modifie pas : on en crée une nouvelle qui la remplace.
4. Commit et push, pour que la prochaine session parte du bon état.

## Règles

- L'utilisateur modifie aussi les notes depuis Obsidian, qui synchronise toutes les minutes (D-013, guide : `notes/guides/obsidian.md`). Avant d'écrire dans `notes/` : `git pull`. Avant d'ajouter une ligne en fin de fichier : vérifier que le fichier se termine par un retour à la ligne.

- Les secrets (clé Mistral, etc.) ne passent jamais par le chat ni par le repo. En cloud, ils sont dans les « Network secrets » ; pour la CI, dans les secrets GitHub Actions.
