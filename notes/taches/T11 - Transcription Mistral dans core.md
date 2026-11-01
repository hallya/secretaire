---
id: T11
statut: 2. En cours
priorite: MVP
---
# T11 · Transcription Mistral dans core

Construire la requête de transcription Mistral et lire sa réponse, sans réseau.

> [!note] Débloquée par [[D-019 - Mise à jour du verrouillage des dépendances]] le 2026-11-01.

## Format de l'API (doc Mistral, 2026-11-01)
- `POST https://api.mistral.ai/v1/audio/transcriptions`, en-tête `Authorization: Bearer …`
- formulaire multipart : `model=voxtral-mini-transcribe-3`, `file=@audio.wav`, `language=fr` (facultatif)
- réponse JSON : `text`, `model`, `language`, `segments`, `usage`

## Preuve
- **Avant :** Tests sur des réponses enregistrées en échec.
- **Après :** Les mêmes tests passent en CI.

## Liens
- [[D-009 - Voxtral pour la reconnaissance]]
- [[T10 - Interfaces reconnaissance et lecture]]
