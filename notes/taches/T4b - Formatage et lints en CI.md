---
id: T4b
statut: 1. À faire
priorite: nice-to-have
---
# T4b · Formatage et lints en CI

Faire échouer la CI quand le code est mal formaté (`cargo fmt --check`) ou qu'il déclenche un avertissement de `clippy`.

## Preuve
- **Avant :** un code mal formaté passe la CI.
- **Après :** un run rouge sur un défaut de formatage, puis vert une fois corrigé.
