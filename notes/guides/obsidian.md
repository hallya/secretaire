# Guide : Obsidian synchronisé avec le repo (D-013)

Configuration qui fonctionne sur le Mac de l'utilisateur, validée le 2026-11-01 (T8).
À suivre pour réinstaller, ou pour configurer une autre machine.

## 1. Copie locale du repo

```
gh repo clone hallya/secretaire ~/code/secretaire   # si elle n'existe pas
```

Dans Obsidian : **Open folder as vault** → dossier `secretaire`.
La configuration locale d'Obsidian (`.obsidian/`) est ignorée par git.

## 2. Plugin Git (auteur : Vinzent03)

**Settings → Community plugins → Browse** → « Git » → **Install** → **Enable**, puis dans ses options :

- **Split timers for automatic commit and sync** : désactivé
- **Auto commit-and-sync interval (minutes)** : 1
- **Auto pull interval (minutes)** : 1, et pull au démarrage activé

Piège rencontré : avec le seul pull automatique réglé, les modifications restent sur le Mac sans jamais être commitées.

## 3. Signature des commits avec l'agent SSH Bitwarden

Les commits sont signés avec une clé SSH gérée par Bitwarden. Une application lancée depuis le Dock ne voit pas le réglage du Terminal qui désigne l'agent Bitwarden. Erreur obtenue : « Couldn't find key in agent? ».

Solution : un petit programme de signature qui va toujours chercher l'agent Bitwarden.

```
mkdir -p ~/.local/bin
cat > ~/.local/bin/ssh-keygen-bitwarden <<'EOF'
#!/bin/sh
for s in "$HOME/.bitwarden-ssh-agent.sock" "$HOME/Library/Containers/com.bitwarden.desktop/Data/.bitwarden-ssh-agent.sock"; do
  [ -S "$s" ] && { SSH_AUTH_SOCK="$s"; export SSH_AUTH_SOCK; break; }
done
exec /usr/bin/ssh-keygen "$@"
EOF
chmod +x ~/.local/bin/ssh-keygen-bitwarden
git -C ~/code/secretaire config --local gpg.ssh.program "$HOME/.local/bin/ssh-keygen-bitwarden"
```

Bitwarden doit être ouvert et déverrouillé.

## 4. Authentification pour pousser sur GitHub

GitHub refuse les mots de passe pour git. Erreur obtenue : « Password authentication is not supported ». Depuis Obsidian, git n'arrivait pas non plus à utiliser la connexion de la CLI `gh`.

Solution : copier le jeton de `gh` dans le trousseau macOS, que toutes les applications savent lire.

```
printf 'protocol=https\nhost=github.com\nusername=hallya\npassword=%s\n\n' "$(gh auth token)" | git credential-osxkeychain store
git -C ~/code/secretaire config --local credential.helper osxkeychain
```

Test qui imite les conditions d'Obsidian :

```
env -i HOME="$HOME" PATH=/usr/bin:/bin GIT_TERMINAL_PROMPT=0 git -C ~/code/secretaire push
```

## Reste à faire

- **T8c :** synchronisation sur l'iPhone. Le plugin Git est déconseillé sur mobile par son auteur.
- **T8d :** badge « Verified » sur GitHub. La clé Bitwarden doit être déclarée comme **Signing Key** dans les réglages GitHub.
