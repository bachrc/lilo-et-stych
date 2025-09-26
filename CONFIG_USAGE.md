# Configuration System Usage Guide

This document explains how to configure the lilo-et-stych Matrix bot using the three available methods:

1. Configuration file (`config.toml`)
2. Command-line arguments
3. Environment variables

## Configuration Precedence

The configuration system follows this precedence order (highest to lowest):
1. **Command-line arguments** (highest priority)
2. **Environment variables**
3. **Configuration file** (lowest priority)

This means CLI arguments will override environment variables, which will override config file values.

## Available Configuration Options

| Option                | Config File Path        | CLI Argument              | Environment Variable    | Description                                  |
| --------------------- | ----------------------- | ------------------------- | ----------------------- | -------------------------------------------- |
| Matrix Homeserver URL | `matrix.homeserver.url` | `--matrix-homeserver-url` | `MATRIX_HOMESERVER_URL` | Matrix server URL                            |
| Matrix Bot Username   | `matrix.bot.username`   | `--matrix-bot-username`   | `MATRIX_BOT_USERNAME`   | Bot Matrix ID                                |
| Matrix Bot Password   | `matrix.bot.password`   | `--matrix-bot-password`   | `MATRIX_BOT_PASSWORD`   | Bot password                                 |
| Stych Email           | `stych.email`           | `--stych-email`           | `STYCH_EMAIL`           | Stych.fr login email                         |
| Stych Password        | `stych.mdp`             | `--stych-password`        | `STYCH_PASSWORD`        | Stych.fr login password                      |
| Target User           | `target_user`           | `--target-user`           | `TARGET_USER`           | Notification target user                     |
| Config File Path      | N/A                     | `--config`                | N/A                     | Path to config file (default: `config.toml`) |

## Usage Examples

### 1. Configuration File Only

Create a `config.toml` file:

```toml
[matrix.homeserver]
url = "https://cyberendroit.net"

[matrix.bot]
username = "@stych:cyberendroit.net"
password = "your-bot-password"

[stych]
email = "your-email@example.com"
mdp = "your-stych-password"

# Optional: override default target user
target_user = "@your-user:cyberendroit.net"
```

Run the bot:
```bash
cargo run
```

### 2. Command-Line Arguments

Override specific values:
```bash
cargo run -- \
  --matrix-homeserver-url "https://custom-server.com" \
  --matrix-bot-username "@custom-bot:server.com" \
  --stych-email "custom@example.com" \
  --target-user "@target:example.com"
```

Use a different config file:
```bash
cargo run -- --config /path/to/custom-config.toml
```

### 3. Environment Variables

Set environment variables:
```bash
export MATRIX_HOMESERVER_URL="https://env-server.com"
export MATRIX_BOT_USERNAME="@env-bot:server.com"
export STYCH_EMAIL="env@example.com"
export TARGET_USER="@env-target:example.com"
```

Run the bot:
```bash
cargo run
```

### 4. Mixed Configuration (Demonstrating Precedence)

Config file (`config.toml`):
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

Environment variables:
```bash
export MATRIX_HOMESERVER_URL="https://env-server.com"
export STYCH_EMAIL="env@example.com"
```

Command-line arguments:
```bash
cargo run -- --matrix-homeserver-url "https://cli-server.com"
```

**Result**: The bot will use:
- Matrix homeserver: `https://cli-server.com` (CLI overrides env)
- Matrix bot username: `@file-bot:server.com` (from config file)
- Stych email: `env@example.com` (env overrides config file)
- Matrix bot password: `file-password` (from config file)
- Stych password: `file-stych-password` (from config file)

## Help and Version Information

View all available options:
```bash
cargo run -- --help
```

View version information:
```bash
cargo run -- --version
```

## Security Notes

- **Never commit passwords** to version control
- Use environment variables or command-line arguments for sensitive data in production
- Consider using a secrets management system for production deployments
- The bot password and Stych password are masked in logs and output

## Troubleshooting

If you encounter configuration errors:

1. Check that your TOML syntax is correct
2. Verify all required fields are present in your config file
3. Ensure environment variables are properly exported
4. Use `--help` to see all available options
5. Check that Matrix IDs are in the correct format (`@username:server.com`)