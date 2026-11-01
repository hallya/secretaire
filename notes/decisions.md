# Décisions

<!-- Format : ## D-XXX — titre · date · statut (à prendre / proposée / acceptée / remplacée) -->
<!-- Une décision acceptée ne se modifie pas : on en crée une nouvelle qui la remplace. -->

## D-001 — Rust comme langage de base, compilé en WASM si possible · — · acceptée

Origine : description du projet. Le WASM est visé partout où c'est possible. Voir D-008 pour la limite côté terminal.

## D-002 — Le terminal d'abord, le web ensuite · — · acceptée

Origine : description du projet. Première cible d'usage : dicter et écouter des prompts destinés à une IA.

## D-003 — Développement guidé par les specs, preuve avant et après · — · acceptée

Origine : description du projet et instructions. Chaque changement a un scope unitaire, une seule responsabilité, et deux preuves : un test qui échoue avant et passe après (test unitaire, d'intégration ou e2e). Chaque tâche est marquée MVP ou nice-to-have. Si Claude ne peut pas vérifier une preuve lui-même, il le dit.

## D-004 — Modèle d'IA au choix, séparément pour la reconnaissance et la lecture · — · acceptée

Origine : description du projet. Implique une interface commune par fonction (reconnaissance, lecture) avec un fournisseur interchangeable.

## D-005 — Code sur un repo GitHub public `hallya/secretaire` · 2026-11-01 · acceptée

Origine : l'utilisateur.

## D-006 — Modèles appelés via une API distante · 2026-11-01 · acceptée

Origine : l'utilisateur. Pas d'inférence locale pour le MVP.

## D-007 — Lecture vocale : Mistral Voxtral TTS · 2026-11-01 · acceptée

Origine : l'utilisateur. Sorti en mars 2026 : 9 langues dont le français, voix prêtes à l'emploi (anglais américain, britannique, français), clonage de voix à partir d'un court extrait, profils de voix réutilisables via l'API, réponse en flux. Prix annoncé : 0,016 $ les 1 000 caractères. La personnalisation des voix passe par la plateforme Mistral, donc par l'API (cohérent avec D-006).

## D-008 — Architecture : `core` / `cli` / `web` · 2026-11-01 · acceptée

- `core` : Rust pur, sans accès au système (configuration, choix du fournisseur, construction des requêtes, lecture des réponses). Compilé en natif **et** en `wasm32`, vérifié par la CI à chaque changement.
- `cli` : exécutable natif, un fichier par OS, rien à installer. Gère l'audio et l'écriture dans les autres applications.
- `web` (plus tard) : le `core` en WASM, avec les API audio du navigateur.

Raisons : il n'existe pas d'API audio standard en WASI (la proposition wasi-media n'est qu'une issue ouverte), et écrire dans une autre application demande des API du système, hors de portée d'un bac à sable WASM. Avec des modèles distants (D-006), le travail lourd est fait côté serveur.

Alternative écartée : tout en WASM (WASI) dans le terminal. Elle exigerait d'installer un runtime et d'inventer nos propres fonctions audio.

## D-009 — Reconnaissance vocale : Mistral Voxtral · 2026-11-01 · acceptée

Origine : l'utilisateur. Voxtral Mini Transcribe V2 (par fichier, 0,003 $/min) et Voxtral Realtime (en direct, latence réglable sous 200 ms, 0,006 $/min). 13 langues dont le français. Une seule clé API pour la reconnaissance et la lecture.

## D-010 — Licence MIT OR Apache-2.0 · 2026-11-01 · acceptée

Origine : l'utilisateur. Double licence, la convention de l'écosystème Rust. Remplace la licence MIT seule du commit initial.

## D-011 — Notes du projet dans `notes/` du repo, `CLAUDE.md` comme point d'entrée · 2026-11-01 · acceptée

Structure mise en place par l'utilisateur au commit initial. Raison constatée le 2026-11-01 : le repo est le seul endroit lisible par tous les espaces de travail. Une session Claude Code ne voit pas les documents du projet claude.ai.

Fichiers : `decisions.md` (décisions prises et à prendre), `backlog.md` (tâches faites et à venir), `journal.md` (réussites, échecs, leçons).

## D-012 — Répartition des sessions · 2026-11-01 · remplacée (par D-014)

Origine : `CLAUDE.md` du commit initial.
- Cloud (claude.ai/code) : notes, planification, code, preuves par la CI.
- Mac en local : uniquement les tests physiques.

## D-013 — Outil de collaboration sur les notes : Obsidian ouvert sur le repo · 2026-11-01 · acceptée

Besoin exprimé par l'utilisateur : GitHub est simple mais peu ergonomique pour lui. Il faut un outil gratuit (auto-hébergeable si possible) que Claude utilise par MCP ou API, et que l'utilisateur utilise par le web ou une application native.

Contrainte côté Claude, constatée le 2026-11-01 : depuis les conversations du projet, Claude n'atteint que GitHub et les connecteurs de l'annuaire de Claude. Aucun outil auto-hébergé n'y figure. Le joindre demanderait un connecteur personnalisé, donc un serveur MCP exposé en HTTPS sur internet.

Options évaluées :
- **Obsidian sur le repo** (recommandé) : application native gratuite (Mac, iOS, Android, Windows), hors ligne. Un plugin de synchronisation git récupère les changements toutes les 30 s sur ordinateur et 60 s sur mobile, et pousse après chaque modification. Claude continue d'écrire en Markdown via git. Une seule source de vérité, rien à héberger, historique git. Inconvénients : un jeton GitHub à créer par l'utilisateur, beaucoup de petits commits automatiques, conflits possibles si on modifie la même ligne en même temps.
- **Notion** (plan B) : la meilleure ergonomie (bases de données, vue kanban, commentaires, applications natives). Connecteur officiel dans l'annuaire de Claude. Gratuit pour un utilisateur seul : pages et blocs illimités, historique de 7 jours, fichiers de 5 Mo max. Inconvénients : pas auto-hébergeable, et les notes quitteraient le repo, ce qui fait deux endroits (notes dans Notion, code et preuves dans GitHub).
- **Linear** : excellent pour le backlog seul (lien automatique avec les commits GitHub, application native). Connecteur officiel. Plan gratuit limité à 250 tickets et 2 équipes. Pas auto-hébergeable. Ne couvre ni les décisions ni le journal.
- **Plane** (auto-hébergeable, équivalent libre de Linear) : serveur MCP officiel distant, mais l'authentification OAuth exige l'édition commerciale. L'édition communautaire gratuite ne fonctionne qu'en local, donc pas depuis le cloud de Claude. Demande aussi un hébergement en HTTPS public.
- **AFFiNE** (auto-hébergeable, équivalent libre de Notion, applications natives) : MCP officiel intégré et gratuit, mais pas utilisable comme connecteur web de claude.ai, et les outils d'écriture sont réservés aux versions de développement. Pas d'écriture dans les bases de données.
- **Outline, Docmost** (wikis auto-hébergeables) : pas de connecteur dans l'annuaire, hébergement à prévoir.
- **Claude Docs** (déjà inclus dans claude.ai) : documents modifiables et commentables dans l'application Claude, mais illisibles par la CI et hors du repo.

Raison du choix recommandé : c'est la seule option qui ajoute une vraie application native sans créer une deuxième source de vérité ni rien à héberger.

**Seconde recherche (2026-11-01, 15:45), demandée par l'utilisateur : Anytype, recherche élargie, NAS Synology.**

Fait nouveau : avec le NAS, un outil auto-hébergé devient joignable par Claude via un connecteur personnalisé (claude.ai : Customize > Connectors > Add custom connector). Conditions : serveur MCP distant joignable depuis internet par les serveurs d'Anthropic, en HTTPS, avec OAuth ou un en-tête d'authentification. Ces connecteurs fonctionnent sur le web, l'application de bureau et le mobile.

- **Anytype** : la meilleure application pour l'utilisateur (natives sur toutes les plateformes, local-first, chiffrée de bout en bout, vues kanban). Mais côté Claude :
  - le MCP officiel est local : il parle à l'application de bureau, donc il est injoignable depuis le cloud ;
  - le client sans interface officiel (`anytype-cli`) expose une API HTTP, mais aucun MCP ;
  - le seul connecteur distant trouvé est communautaire : 0 étoile, un seul mainteneur, et il repose sur une API v2 encore en pré-version ;
  - les notes sortiraient de git, donc deux sources de vérité ;
  - plan gratuit : 100 Mo de sauvegarde distante, ou un nœud de synchronisation auto-hébergé sur le NAS (Docker Compose), avec bascule manuelle de tous les appareils.

  Verdict : excellente pour l'utilisateur, fragile pour Claude. À revoir si Anytype publie un MCP distant officiel.
- **Outline** (wiki auto-hébergeable) : MCP officiel intégré depuis février 2026, en HTTP avec OAuth ou clé API, y compris en auto-hébergé. Devient viable sur le NAS. Limites : pas de kanban, un fournisseur d'identité externe à configurer, licence BSL (pas open source au sens strict), interface web.
- **Plane** (backlog, auto-hébergeable) : MCP officiel distant. Avec l'édition communautaire, seul l'accès par clé API pourrait convenir à un connecteur personnalisé, sans garantie. À tester avant de compter dessus.
- **AFFiNE** : MCP officiel en HTTP avec jeton, mais écriture réservée aux versions de développement. Inchangé.
- **AppFlowy, Nextcloud, SiYuan, Trilium, BookStack, Wiki.js, Joplin, Logseq** : au mieux des MCP communautaires, ou seulement une API. Rien de plus solide que les options ci-dessus.
- Coût commun à toute option auto-hébergée : exposer le NAS sur internet (proxy inverse et certificat HTTPS, idéalement limité aux adresses IP d'Anthropic), puis le maintenir.

Conclusion de la seconde recherche : la recommandation Obsidian tient. C'est la seule option qui ne demande ni d'exposer le NAS, ni une deuxième source de vérité, ni de dépendre d'un connecteur communautaire. Si l'utilisateur veut plus tard une vraie application auto-hébergée : Outline sur le NAS est l'option la plus solide côté Claude.

## D-016 — Rôle du NAS Synology DS1621xs+ · 2026-11-01 · acceptée

Matériel : Xeon D-1527 (4 cœurs, 2,2 GHz), 8 Go de RAM extensibles à 32 Go, 10 GbE, sans carte graphique.

- **Oui, plus tard, si utile** : héberger un outil auto-hébergé (Container Manager) si D-013 évolue ; héberger la version web de Secretaire (alternative à GitHub Pages) ; sauvegarder le repo.
- **Non** : un runner GitHub Actions auto-hébergé. GitHub le déconseille pour un repo public, car le code de n'importe quelle pull request s'exécuterait sur le NAS, et les runners de GitHub sont gratuits pour les repos publics.
- **Non pour le MVP** : faire tourner les modèles Voxtral sur le NAS. Sans carte graphique, ce serait trop lent pour du temps réel, et D-006 prévoit une API distante.

## D-014 — Un seul endroit pour travailler : le projet claude.ai « Secretaire » · 2026-11-01 · acceptée

Remplace la version précédente (« pilotage dans le projet, code dans les sessions Claude Code »), devenue inutile. Constat du 2026-11-01 vers 15:30 : depuis une conversation du projet, Claude peut pousser sur le repo et lire l'API GitHub, y compris les résultats de CI.

- L'utilisateur parle à Claude **uniquement** dans le projet « Secretaire » (web, application de bureau ou mobile : c'est le même projet).
- Quand une conversation devient longue, on en ouvre une nouvelle dans le même projet. La reprise est automatique grâce aux instructions du projet et à `CLAUDE.md` (T3).
- Les notes vivent dans `notes/` du repo (D-011), lisibles dans Obsidian (D-013).
- Les preuves sont fournies par la CI GitHub Actions, que Claude lit lui-même.
- L'utilisateur n'ouvre plus de session Claude Code lui-même. Si Claude en a besoin, il s'en occupe ou dit exactement quoi faire.
- Seule limite : pas de compilation instantanée dans ces conversations (pas d'accès à crates.io). Chaque essai passe par la CI et prend quelques minutes. Si c'est trop lent, Claude pourra tester une délégation vers une session cloud qui a le réseau.

## D-015 — Comment prouver les tests physiques · — · à prendre

Micro, haut-parleur et écriture dans une autre application ne peuvent être testés que sur le Mac de l'utilisateur. Options : check-list manuelle exécutée par l'utilisateur, ou session Claude Code locale. À décider au moment des tâches concernées.
