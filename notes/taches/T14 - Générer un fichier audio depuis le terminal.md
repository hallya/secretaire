---
id: T14
statut: 1. À faire
priorite: MVP
---
# T14 · Générer un fichier audio depuis le terminal

`secretaire lire "Bonjour" --sortie bonjour.mp3` crée un fichier audio, avec un vrai appel à Mistral. ⚠️ Utilise la clé Mistral enregistrée dans les secrets GitHub (`MISTRAL_API_KEY`).

## Preuve
- **Avant :** La commande n'existe pas.
- **Après :** Aller-retour en CI : l'audio généré, retranscrit par T12, redonne le texte.

## Liens
- [[D-007 - Voxtral TTS pour la lecture]]
- [[T12 - Transcrire un fichier depuis le terminal]]
