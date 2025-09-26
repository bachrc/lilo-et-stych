# Configuration du Bot Matrix

## Méthodes de Configuration

Le bot supporte trois méthodes de configuration (par ordre de priorité) :

1. **Arguments en ligne de commande** (priorité la plus haute)
2. **Variables d'environnement**
3. **Fichier de configuration** (priorité la plus basse)

Pour plus de détails sur l'utilisation de ces méthodes, consultez le guide complet : [CONFIG_USAGE.md](CONFIG_USAGE.md)

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

Puis lancez le bot :
```bash
cargo run
```

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

## Utilisation de Base

1. Configurez votre fichier `config.toml` avec les informations de connexion
2. Lancez le bot : `cargo run`
3. Le bot enverra un message "bonjour" à l'utilisateur configuré et récupérera les informations de cours de conduite

## Exemples d'Utilisation Avancés

Pour voir tous les exemples d'utilisation et la priorité des configurations, consultez le guide complet : [CONFIG_USAGE.md](CONFIG_USAGE.md)

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
- Les mots de passe sont masqués dans les journaux et la sortie