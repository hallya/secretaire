---
id: D-018
statut: 2. Proposée
date: 2026-11-01
origine: Claude
---
# D-018 · Langue du code

## Décision
- Noms de types et de fonctions en **anglais**, la convention de l'écosystème Rust pour un repo public.
- **Noms des tests en français**, rédigés comme des phrases de spécification (`un_fournisseur_inconnu_donne_une_erreur_explicite`) : ils se lisent comme la spec du produit.
- Commentaires de documentation en français.

## Pourquoi
Le code reste lisible par la communauté Rust, et les tests (la preuve de chaque tâche) restent lisibles par l'utilisateur.

## Alternatives écartées
- Tout en français : inhabituel pour une crate publique.
- Tout en anglais : les tests ne se liraient plus comme une spec pour l'utilisateur.
