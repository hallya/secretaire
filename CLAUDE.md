# CLAUDE.md

Toutes les notes du projet sont dans `notes/`. Lis-les au début de chaque session.

- `notes/decisions.md` : les décisions d'architecture (D-001, D-002, …)
- `notes/journal.md` : le journal daté des tests et de leurs résultats

## Répartition des sessions

- **Cloud** (claude.ai/code) : notes, planification, code, et preuves par la CI (GitHub Actions).
- **Mac en local** : uniquement les tests physiques (dictée au micro réelle, écriture dans une autre application).

## Règles

- Les secrets (clé Mistral, etc.) ne passent jamais par le chat ni par le repo. En cloud, ils sont dans les « Network secrets » ; pour la CI, dans les secrets GitHub Actions.
