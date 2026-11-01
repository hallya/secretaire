---
id: T11
statut: 4. Fait
priorite: MVP
fait_le: 2026-11-01
---
# T11 · Transcription Mistral dans core

Construire la requête de transcription Mistral et lire sa réponse, sans réseau.

> [!note] Débloquée par [[D-019 - Mise à jour du verrouillage des dépendances]] le 2026-11-01.

## Format de l'API (doc Mistral, 2026-11-01)
- `POST https://api.mistral.ai/v1/audio/transcriptions`, en-tête `Authorization: Bearer …`
- formulaire multipart : `model=voxtral-mini-transcribe-3`, `file=@audio.wav`, `language=fr` (facultatif)
- réponse JSON : `text`, `model`, `language`, `segments`, `usage`

## Preuve
- **Avant :** run `41341419159` rouge sur `443c509` : 7 tests en échec (requête et réponse non implémentées).
- **Après :** run `41341481594` vert sur `c7c87b9` : 16 tests, dont 9 pour Mistral.
- ⚠️ La réponse de test reprend le format de la documentation Mistral. T12 le confrontera à la vraie API.

## Liens
- [[D-009 - Voxtral pour la reconnaissance]]
- [[T10 - Interfaces reconnaissance et lecture]]

## Ce qui existe maintenant
- `MistralSpeechToText` : nom `mistral`, modèle `voxtral-mini-transcribe-3` par défaut, langue et adresse de l'API réglables.
- Les erreurs HTTP reprennent le message de Mistral (`message` ou `detail`).
- La clé API n'apparaît jamais dans les journaux de débogage.
