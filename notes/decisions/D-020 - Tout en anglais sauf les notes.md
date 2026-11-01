---
id: D-020
statut: 3. Acceptée
date: 2026-11-01
origine: utilisateur
remplace: D-018
---
# D-020 · Tout en anglais, sauf les notes

## Décision
- **En anglais :** tout ce qui est dans le repo hors de `notes/` : code, noms de tests, données de test, commentaires, messages d'erreur, commandes (`secretaire dictate`…), README, CI, **messages de commit**.
- **En français :** uniquement notre collaboration dans Obsidian : `notes/` (et `CLAUDE.md`, qui décrit cette collaboration).

## Pourquoi
Demande explicite de l'utilisateur : « la seule chose en français, c'est notre collaboration dans Obsidian ».

## Alternatives écartées
- [[D-018 - Langue du code]] : commentaires et interface en français. Remplacée.

## Mise en œuvre
- **Avant :** run `41340923533` rouge sur `f13f6df` : les tests attendent des messages d'erreur en anglais.
- **Après :** run `41340994052` vert sur `5ea0c0e` : messages traduits, plus aucun texte français hors de `notes/` et `CLAUDE.md` (vérifié par recherche des accents).
- Les anciens messages de commit en français restent dans l'historique : réécrire l'historique public n'en vaut pas la peine.
