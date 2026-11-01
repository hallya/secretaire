---
id: T12
statut: 1. À faire
priorite: MVP
---
# T12 · Transcrire un fichier depuis le terminal

`secretaire transcribe file.wav` affiche le texte, avec un vrai appel à Mistral. ⚠️ Utilise la clé Mistral enregistrée dans les secrets GitHub (`MISTRAL_API_KEY`).

## Preuve
- **Avant :** La commande n'existe pas.
- **Après :** En CI, un fichier audio au contenu connu donne un texte qui contient les mots attendus.

## Liens
- [[D-009 - Voxtral pour la reconnaissance]]
