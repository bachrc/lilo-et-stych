# Configuration du Bot Matrix

## Méthodes de Configuration

Le bot supporte trois méthodes de configuration (par ordre de priorité) :

1. **Arguments en ligne de commande** (priorité la plus haute)
2. **Variables d'environnement**
3. **Fichier de configuration** (priorité la plus basse)

Cela signifie que les arguments de ligne de commande remplaceront les variables d'environnement, qui remplaceront les valeurs du fichier de configuration.

## Configuration via Fichier

Créez un fichier `config.toml` avec la structure suivante :

```toml
# Fichier de configuration du bot Matrix
# Remplissez ces informations avec les identifiants de votre bot

[matrix.homeserver]
# URL du serveur Matrix (homeserver)
url = "https://matrix.example.com"

[matrix.bot]
# Identifiant Matrix du bot (format: @nom:serveur.com)
username = "@votrebot:exemple.com"

# Mot de passe du bot
password = "votre_mot_de_passe_securise"

[stych]
# Email pour la connexion à Stych.fr
email = "votre-email@example.com"

# Mot de passe pour la connexion à Stych.fr
mdp = "votre_mot_de_passe_stych"

# Optionnel : utilisateur cible pour les notifications (par défaut: @pacha:cyberendroit.net)
target_user = "@votre-utilisateur:exemple.com"
```

## Configuration via Arguments de Ligne de Commande

Vous pouvez surcharger n'importe quelle valeur du fichier de configuration :

```bash
cargo run -- \
  --matrix-homeserver-url "https://custom-server.com" \
  --matrix-bot-username "@custom-bot:server.com" \
  --stych-email "custom@example.com" \
  --target-user "@target:example.com"
```

Utiliser un fichier de configuration différent :
```bash
cargo run -- --config /path/to/custom-config.toml
```

## Configuration via Variables d'Environnement

Définissez les variables d'environnement :

```bash
export MATRIX_HOMESERVER_URL="https://env-server.com"
export MATRIX_BOT_USERNAME="@env-bot:server.com"
export MATRIX_BOT_PASSWORD="env-password"
export STYCH_EMAIL="env@example.com"
export STYCH_PASSWORD="env-stych-password"
export TARGET_USER="@env-target:example.com"
```

## Exemple de Configuration Mixte (Démonstration de la Priorité)

Fichier de configuration (`config.toml`) :
```toml
[matrix.homeserver]
url = "https://file-server.com"

[matrix.bot]
username = "@file-bot:server.com"
password = "file-password"

[stych]
email = "file@example.com"
mdp = "file-stych-password"
```

Variables d'environnement :
```bash
export MATRIX_HOMESERVER_URL="https://env-server.com"
export STYCH_EMAIL="env@example.com"
```

Arguments de ligne de commande :
```bash
cargo run -- --matrix-homeserver-url "https://cli-server.com"
```

**Résultat**: Le bot utilisera :
- Serveur Matrix : `https://cli-server.com` (CLI remplace les variables d'environnement)
- Nom d'utilisateur du bot : `@file-bot:server.com` (du fichier de configuration)
- Email Stych : `env@example.com` (variable d'environnement remplace le fichier de configuration)
- Mot de passe du bot : `file-password` (du fichier de configuration)
- Mot de passe Stych : `file-stych-password` (du fichier de configuration)

## Options de Configuration Disponibles

| Option                   | Chemin dans le fichier  | Argument CLI              | Variable d'environnement |
| ------------------------ | ----------------------- | ------------------------- | ------------------------ |
| URL du serveur Matrix    | `matrix.homeserver.url` | `--matrix-homeserver-url` | `MATRIX_HOMESERVER_URL`  |
| Nom d'utilisateur du bot | `matrix.bot.username`   | `--matrix-bot-username`   | `MATRIX_BOT_USERNAME`    |
| Mot de passe du bot      | `matrix.bot.password`   | `--matrix-bot-password`   | `MATRIX_BOT_PASSWORD`    |
| Email Stych              | `stych.email`           | `--stych-email`           | `STYCH_EMAIL`            |
| Mot de passe Stych       | `stych.mdp`             | `--stych-password`        | `STYCH_PASSWORD`         |
| Utilisateur cible        | `target_user`           | `--target-user`           | `TARGET_USER`            |
| Chemin du fichier config | N/A                     | `--config`                | N/A                      |


## Aide et Informations

Afficher l'aide :
```bash
cargo run -- --help
```

Afficher la version :
```bash
cargo run -- --version
```

## Notes de Sécurité

- **Ne commettez jamais de mots de passe** dans le contrôle de version
- Utilisez des variables d'environnement ou des arguments de ligne de commande pour les données sensibles en production
- Considérez l'utilisation d'un système de gestion des secrets pour les déploiements en production
- Les mots de passe sont masqués dans les journaux et la sortie

## Dépannage

Si vous rencontrez des erreurs de configuration :

1. Vérifiez que votre syntaxe TOML est correcte
2. Assurez-vous que tous les champs requis sont présents dans votre fichier de configuration
3. Vérifiez que les variables d'environnement sont correctement exportées
4. Utilisez `--help` pour voir toutes les options disponibles
5. Vérifiez que les identifiants Matrix sont au bon format (`@nom:serveur.com`)