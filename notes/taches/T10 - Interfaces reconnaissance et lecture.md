---
id: T10
statut: 2. En cours
priorite: MVP
---
# T10 · Interfaces reconnaissance et lecture

Dans `core`, une interface pour la reconnaissance et une pour la lecture, avec un fournisseur choisi par son nom, séparément pour chaque fonction.

## Preuve
- **Avant :** Test avec un faux fournisseur en échec (choix du fournisseur non implémenté).
- **Après :** Le même test passe en CI, en natif ; `core` compile toujours en `wasm32`.

## Liens
- [[D-004 - Modèle d'IA au choix]]
- [[D-008 - Architecture core cli web]]
