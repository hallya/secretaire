---
id: D-013
statut: 3. Acceptée
date: 2026-11-01
origine: utilisateur
---
# D-013 · Outil de collaboration : Obsidian sur le repo

## Décision
Les notes restent des fichiers Markdown dans le repo. L'utilisateur les lit et les modifie dans **Obsidian**, synchronisé avec GitHub par le plugin Git.

## Pourquoi
- Une vraie application native (Mac, iPhone, Android), qui fonctionne hors ligne.
- Une seule source de vérité : le repo, que Claude lit et écrit de partout.
- Rien à héberger, et le NAS n'a pas à être exposé sur internet.
- L'historique git garde la trace de chaque changement.

## Alternatives écartées
- **Notion** : la meilleure ergonomie, mais les notes quitteraient le repo.
- **Anytype** : excellente pour l'utilisateur, mais Claude n'y accède que par un connecteur communautaire fragile.
- **Outline sur le NAS** : connecteur officiel, mais pas de kanban, et le NAS doit être exposé.
- **Linear, Plane, AFFiNE et autres** : trop partiels, ou difficiles d'accès pour Claude.

## Liens
- Mise en œuvre : [[T8 - Obsidian synchronisé avec le repo]]
