---
id: T10
statut: 4. Fait
priorite: MVP
fait_le: 2026-11-01
---
# T10 · Interfaces reconnaissance et lecture

Dans `core`, une interface pour la reconnaissance et une pour la lecture, avec un fournisseur choisi par son nom, séparément pour chaque fonction.

## Preuve
- **Avant :** run `41337631564` rouge sur `0b96385` : 4 tests en échec (choix par nom, choix indépendant par fonction, allers-retours requête/réponse), `wasm32` compile.
- **Après :** run `41337685567` vert sur `81345e1` : les 6 tests passent en natif, `core` compile en `wasm32`.

## Liens
- [[D-004 - Modèle d'IA au choix]]
- [[D-008 - Architecture core cli web]]

## Ce qui existe maintenant
- `SpeechToText` et `TextToSpeech` : un fournisseur décrit la requête à envoyer et lit la réponse, sans accès au réseau (D-008).
- `Registry` et `Selection` : on choisit un fournisseur par son nom, séparément pour la reconnaissance et la lecture.
- `ProviderError` : messages d'erreur en français, par exemple « aucun fournisseur « x » pour la reconnaissance ».
