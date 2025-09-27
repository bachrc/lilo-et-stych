# Lilo-et-Stych 🚗

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-blue.svg)](https://www.rust-lang.org/)

A Rust library and Matrix bot for interacting with the Stych.fr driving school platform to monitor available driving courses and send notifications.

## Features

- **Stych.fr Integration**: Login and retrieve driving lesson planning
- **Matrix Bot**: Send real-time notifications via Matrix protocol
- **Course Monitoring**: Automatically check for new available driving courses
- **Flexible Configuration**: Multiple configuration methods (file, CLI, environment variables)
- **Comprehensive Logging**: Detailed logging for debugging and monitoring

## Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/bachrc/lilo-et-stych.git
   cd lilo-et-stych
   ```

2. Configure the application (see [Configuration](#configuration)):
   ```bash
   cp config.toml.example config.toml
   # Edit config.toml with your credentials
   ```

3. Run the bot:
   ```bash
   cargo run
   ```

## Configuration

The application supports three configuration methods with the following precedence:
1. Command-line arguments (highest priority)
2. Environment variables
3. Configuration file (lowest priority)

### Configuration File

Create a `config.toml` file:

```toml
[matrix.homeserver]
url = "https://your-matrix-server.com"

[matrix.bot]
username = "@your-bot:server.com"
password = "your-bot-password"

[stych]
email = "your-stych-email@example.com"
mdp = "your-stych-password"

# Optional: override default target user
target_user = "@target-user:server.com"

[monitoring]
check_interval_seconds = 60
```

### Environment Variables

```bash
export MATRIX_HOMESERVER_URL="https://your-matrix-server.com"
export MATRIX_BOT_USERNAME="@your-bot:server.com"
export MATRIX_BOT_PASSWORD="your-bot-password"
export STYCH_EMAIL="your-stych-email@example.com"
export STYCH_PASSWORD="your-stych-password"
export TARGET_USER="@target-user:server.com"
```

### Command-Line Arguments

```bash
cargo run -- \
  --matrix-homeserver-url "https://your-matrix-server.com" \
  --matrix-bot-username "@your-bot:server.com" \
  --stych-email "your-stych-email@example.com" \
  --target-user "@target-user:server.com"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Stych.fr platform for providing driving course information
- Built with [Rust](https://www.rust-lang.org/) and [Matrix SDK](https://github.com/matrix-org/matrix-rust-sdk)