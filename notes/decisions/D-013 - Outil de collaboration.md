---
id: D-013
statut: 3. Acceptée
date: 2026-11-01
origine: utilisateur
---
# D-013 · Outil de collaboration : Obsidian sur le repo

## Décision
Les notes restent des fichiers Markdown dans le repo. L'utilisateur les lit et les modifie dans **Obsidian**, synchronisé avec GitHub par le plugin Git.

## Pourquoi
- Une vraie application native (Mac, iPhone, Android), qui fonctionne hors ligne.
- Une seule source de vérité : le repo, que Claude lit et écrit de partout.
- Rien à héberger, et le NAS n'a pas à être exposé sur internet.
- L'historique git garde la trace de chaque changement.

## Alternatives écartées
- **Notion** : la meilleure ergonomie, mais les notes quitteraient le repo.
- **Anytype** : excellente pour l'utilisateur, mais Claude n'y accède que par un connecteur communautaire fragile.
- **Outline sur le NAS** : connecteur officiel, mais pas de kanban, et le NAS doit être exposé.
- **Linear, Plane, AFFiNE et autres** : trop partiels, ou difficiles d'accès pour Claude.

## Détail de l'analyse

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

## Liens
- Mise en œuvre : [[T8 - Obsidian synchronisé avec le repo]]
