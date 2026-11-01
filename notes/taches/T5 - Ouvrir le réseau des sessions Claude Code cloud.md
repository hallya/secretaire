---
id: T5
statut: 4. Fait
priorite: nice-to-have
fait_le: 2026-11-01
---
# T5 · Ouvrir le réseau des sessions Claude Code cloud

Accès à crates.io, aux outils Rust et à Mistral depuis les sessions Claude Code cloud. Nice-to-have : la CI sert de plan B.

## Preuve
- **Avant :** 403 sur `index.crates.io`, `static.rust-lang.org` et `api.mistral.ai`.
- **Après :** `serde` compilé, `wasm32` installé, Mistral répond 200.
