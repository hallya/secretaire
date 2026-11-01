---
id: D-008
statut: 3. Acceptée
date: 2026-11-01
origine: "utilisateur (validée le 2026-11-01)"
---
# D-008 · Architecture core cli web

## Décision
- `core` : Rust pur, sans accès au système (configuration, choix du fournisseur, construction des requêtes, lecture des réponses). Compilé en natif **et** en `wasm32`, vérifié par la CI à chaque changement.
- `cli` : exécutable natif, un fichier par OS, rien à installer. Gère l'audio et l'écriture dans les autres applications.
- `web` (plus tard) : le `core` en WASM, avec les API audio du navigateur.

## Pourquoi
Il n'existe pas d'API audio standard en WASI (la proposition wasi-media n'est qu'une issue ouverte), et écrire dans une autre application demande des API du système, hors de portée d'un bac à sable WASM. Avec des modèles distants (D-006), le travail lourd est fait côté serveur.

## Alternative écartée
Tout en WASM (WASI) dans le terminal. Elle exigerait d'installer un runtime et d'inventer nos propres fonctions audio.
