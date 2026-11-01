# Journal

## 2026-11-01 — Initialisation (session locale, Mac)

- Rust 1.97.1 et cargo sont installés. Cibles disponibles : `aarch64-apple-darwin` et `x86_64-apple-darwin`. La cible `wasm32` n'est pas encore installée.
- git 2.50.1 et GitHub CLI sont disponibles ; `gh` est authentifié comme `hallya`.
- Repo créé : public, sous licence MIT.

## 2026-11-01 — Environnement de travail (sessions cloud)

<!-- ✅ réussite · ❌ échec · 💡 leçon -->

- ✅ Accès GitHub vérifié : 8 repos `hallya`, tous en écriture.
- ❌ Conversation du projet claude.ai (environnement `cloud_default`) : le proxy refuse (403) `index.crates.io`, `static.rust-lang.org` et `api.mistral.ai`. Le CLI `gh` a un jeton refusé.
- ✅ L'utilisateur règle l'environnement cloud : réseau Trusted ou Custom, et clé Mistral en « Network secret » sur `api.mistral.ai`.
- ❌ 14:42 et 14:45 : mêmes refus dans la conversation de mise en place, puis dans une nouvelle conversation du projet. Le réglage ne s'applique pas aux conversations du projet.
- ✅ 14:45 : cette nouvelle conversation a trouvé la note d'entrée du projet et y a consigné son résultat seule. Premier signe positif pour la reprise du fil (T3).
- ✅ ~15:00, session Claude Code cloud (Ubuntu 24.04, Rust 1.97.0) : `serde` téléchargé et compilé, cible `wasm32-unknown-unknown` installée avec un build de test réussi.
- ❌→✅ ~15:00 : Mistral répond d'abord 401 (« Invalid API Key »). Après correction de la clé par l'utilisateur : 200 sur `/v1/models` et sur un vrai appel à `mistral-small-latest`.
- ❌ ~15:00 : cette session Claude Code n'a pas pu lire la note du projet claude.ai. Elle n'a accès ni aux documents du projet, ni à un repo non attaché.
- ✅ 15:19 : repo `hallya/secretaire` rattaché à la conversation du projet et cloné.
- ❌ 15:19 : push impossible, l'application GitHub de Claude n'est pas installée sur ce repo.
- ✅ 15:20 : licence passée à MIT OR Apache-2.0 (commit local, en attente de push).
- 💡 Le repo est le seul endroit lisible par tous les espaces de travail : les notes y vivent (D-011).
- 💡 Plan B si le réseau manque : compilations et appels à Mistral passent par la CI GitHub Actions, avec la clé en secret GitHub.
- 💡 Les tests du micro, du haut-parleur et de l'écriture dans une autre application ne peuvent se faire que sur le Mac (D-015).
- ✅ 15:30 : l'utilisateur installe l'application GitHub de Claude sur le repo. Push réussi : `f7f1096`, `b455a4e`, `3a37773` visibles sur `origin/main` (T0 prouvé).
- ✅ 15:30 : depuis la conversation du projet, l'API GitHub répond 200 (repo et liste des runs de CI). Claude peut donc lire lui-même les preuves de CI.
- 💡 Avec le push et la lecture de la CI, tout peut se faire depuis le projet claude.ai : D-014 révisée en « un seul endroit pour travailler ».
- 💡 15:35 : analyse des outils de collaboration (D-013). Aucun outil auto-hébergé dans l'annuaire des connecteurs de Claude ; recommandation : Obsidian ouvert sur le repo, Notion en plan B.
- ✅ 15:40 : **T3 prouvé.** Dans une nouvelle conversation du projet, « Où en est-on ? » a rattaché le repo, lu `CLAUDE.md` et les notes sans aide, puis donné l'état, les décisions en attente et la prochaine action, conformes aux notes. Validé par l'utilisateur.
- ✅ 15:40 : D-014 acceptée (un seul endroit : le projet claude.ai). D-012 remplacée, `CLAUDE.md` mis à jour.
- 💡 15:40 : l'utilisateur dispose d'un NAS Synology DS1621xs+, à prendre en compte pour l'auto-hébergement et les tâches déléguables. D-013 rouverte : avis sur Anytype et recherche élargie demandés avant décision.
- 💡 15:45 : seconde recherche D-013 (Anytype, outils auto-hébergés, NAS). Fait nouveau : avec le NAS, un outil auto-hébergé est joignable via un connecteur personnalisé claude.ai. Mais Anytype n'a pas de MCP distant officiel, et Outline (MCP officiel intégré) n'a pas de kanban. La recommandation Obsidian tient. Rôle du NAS consigné en D-016 (proposée).
