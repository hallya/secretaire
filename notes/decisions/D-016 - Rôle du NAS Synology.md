---
id: D-016
statut: 3. Acceptée
date: 2026-11-01
origine: "utilisateur"
---
# D-016 · Rôle du NAS Synology

## Matériel
Synology DS1621xs+ : Xeon D-1527 (4 cœurs, 2,2 GHz), 8 Go de RAM extensibles à 32 Go, 10 GbE, sans carte graphique.

## Décision
- **Oui, plus tard, si utile** : héberger un outil auto-hébergé (Container Manager) si D-013 évolue ; héberger la version web de Secretaire (alternative à GitHub Pages) ; sauvegarder le repo.
- **Non** : un runner GitHub Actions auto-hébergé. GitHub le déconseille pour un repo public, car le code de n'importe quelle pull request s'exécuterait sur le NAS, et les runners de GitHub sont gratuits pour les repos publics.
- **Non pour le MVP** : faire tourner les modèles Voxtral sur le NAS. Sans carte graphique, ce serait trop lent pour du temps réel, et D-006 prévoit une API distante.
