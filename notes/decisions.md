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

## D-012 — Répartition des sessions · 2026-11-01 · acceptée

Origine : `CLAUDE.md` du commit initial.
- Cloud (claude.ai/code) : notes, planification, code, preuves par la CI.
- Mac en local : uniquement les tests physiques.

## D-013 — Pas d'outil de notes type Notion pour l'instant · 2026-11-01 · proposée

Raisons : un outil auto-hébergé n'est pas joignable depuis les espaces cloud de Claude sans exposer un connecteur MCP sur internet, et il doublerait la source de vérité. Si un jour une interface plus riche est utile : Docmost (AGPL-3.0, PostgreSQL et Redis, authentification intégrée). Pour auto-héberger la forge elle-même : Forgejo, vers lequel `notes/` migrerait tel quel.

## D-014 — Les conversations du projet claude.ai servent au pilotage, pas au code · 2026-11-01 · proposée

Précise D-012. Constat : les conversations du projet tournent dans l'environnement `cloud_default`, sans accès à crates.io ni à Mistral. Les sessions Claude Code cloud ont ce réseau. Donc : piloter et décider dans les conversations du projet, coder et tester dans les sessions Claude Code.

## D-015 — Comment prouver les tests physiques · — · à prendre

Micro, haut-parleur et écriture dans une autre application ne peuvent être testés que sur le Mac de l'utilisateur. Options : check-list manuelle exécutée par l'utilisateur, ou session Claude Code locale. À décider au moment des tâches concernées.
