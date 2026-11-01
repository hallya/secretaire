---
id: D-018
statut: 4. Remplacée
remplacee_par: D-020
date: 2026-11-01
origine: utilisateur (révision de la proposition de Claude)
---
# D-018 · Langue du code

## Décision
- Le code est **entièrement en anglais** : types, fonctions, **noms des tests** et données de test.
- Les commentaires de documentation restent en français.
- L'interface vue par l'utilisateur (commandes comme `secretaire dicter`, messages d'erreur) est en français.

## Pourquoi
C'est la convention de l'écosystème Rust pour un repo public. L'utilisateur a demandé que les noms de tests soient aussi en anglais.

## Alternatives écartées
- Noms des tests en français, rédigés comme une spec : proposition initiale de Claude, refusée par l'utilisateur.
- Tout en français : inhabituel pour une crate publique.

## Mise en œuvre
Tests renommés au commit `0f07df0`, avec les mêmes 6 tests verts (run `41339321630`).
