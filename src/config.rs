//! Configuration management for the lilo-et-stych application

use clap::Parser;
use serde::Deserialize;

/// Command-line arguments for the application
#[derive(Parser, Debug)]
#[command(name = "lilo-et-stych")]
#[command(about = "Matrix bot for Stych.fr driving school platform")]
#[command(version)]
pub struct CliArgs {
    /// Path to configuration file (default: config.toml)
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    /// Matrix homeserver URL
    #[arg(long, env = "MATRIX_HOMESERVER_URL")]
    pub matrix_homeserver_url: Option<String>,

    /// Matrix bot username
    #[arg(long, env = "MATRIX_BOT_USERNAME")]
    pub matrix_bot_username: Option<String>,

    /// Matrix bot password
    #[arg(long, env = "MATRIX_BOT_PASSWORD")]
    pub matrix_bot_password: Option<String>,

    /// Stych.fr email
    #[arg(long, env = "STYCH_EMAIL")]
    pub stych_email: Option<String>,

    /// Stych.fr password
    #[arg(long, env = "STYCH_PASSWORD")]
    pub stych_password: Option<String>,

    /// Target user for notifications (overrides default)
    #[arg(long, env = "TARGET_USER")]
    pub target_user: Option<String>,
}

/// Main application configuration
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Matrix homeserver and bot configuration
    pub matrix: MatrixConfig,
    /// Stych.fr platform configuration
    pub stych: StychConfig,
    /// Target user for notifications (optional, can be overridden by CLI)
    pub target_user: Option<String>,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
}

/// Matrix homeserver and bot configuration
#[derive(Debug, Deserialize, Clone)]
pub struct MatrixConfig {
    /// Matrix homeserver configuration
    pub homeserver: HomeserverConfig,
    /// Matrix bot configuration
    pub bot: BotConfig,
}

/// Matrix homeserver configuration
#[derive(Debug, Deserialize, Clone)]
pub struct HomeserverConfig {
    /// The URL of the Matrix homeserver
    pub url: String,
}

/// Matrix bot configuration
#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    /// The username of the Matrix bot
    pub username: String,
    /// The password of the Matrix bot
    pub password: String,
}

/// Stych.fr platform configuration
#[derive(Debug, Deserialize, Clone)]
pub struct StychConfig {
    /// Email address for Stych.fr login
    pub email: String,
    /// Password for Stych.fr login (mdp = mot de passe in French)
    pub mdp: String,
}

/// Monitoring configuration
#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringConfig {
    /// Check interval in seconds
    pub check_interval_seconds: u64,
}

impl AppConfig {
    /// Load configuration with unified precedence: CLI args > env vars > config file
    pub fn load() -> Result<Self, config::ConfigError> {
        let cli_args = CliArgs::parse();
        Self::load_with_args(cli_args)
    }

    /// Load configuration with provided CLI arguments
    pub fn load_with_args(cli_args: CliArgs) -> Result<Self, config::ConfigError> {
        // Build configuration with proper precedence:
        // 1. Config file as base
        // 2. Environment variables
        // 3. CLI arguments (highest precedence)
        let settings = config::Config::builder()
            .add_source(config::File::with_name(&cli_args.config))
            .add_source(
                config::Environment::with_prefix("LILO_ET_STYCH")
                    .separator("_")
                    .try_parsing(true),
            );

        // Build the base configuration
        let mut config: AppConfig = settings.build()?.try_deserialize()?;

        // Apply CLI argument overrides with highest precedence
        Self::apply_cli_overrides(&mut config, &cli_args);

        Ok(config)
    }

    /// Apply CLI argument overrides to the configuration
    fn apply_cli_overrides(config: &mut AppConfig, cli_args: &CliArgs) {
        if let Some(url) = &cli_args.matrix_homeserver_url {
            config.matrix.homeserver.url = url.clone();
        }
        if let Some(username) = &cli_args.matrix_bot_username {
            config.matrix.bot.username = username.clone();
        }
        if let Some(password) = &cli_args.matrix_bot_password {
            config.matrix.bot.password = password.clone();
        }
        if let Some(email) = &cli_args.stych_email {
            config.stych.email = email.clone();
        }
        if let Some(password) = &cli_args.stych_password {
            config.stych.mdp = password.clone();
        }
        if let Some(target_user) = &cli_args.target_user {
            config.target_user = Some(target_user.clone());
        }
    }

    /// Load configuration from a specific file path (for testing)
    pub fn load_from_file(path: &str) -> Result<Self, config::ConfigError> {
        let cli_args = CliArgs {
            config: path.to_string(),
            ..Default::default()
        };
        Self::load_with_args(cli_args)
    }

    /// Get the target user, using CLI override or default
    pub fn get_target_user(&self) -> &str {
        self.target_user
            .as_deref()
            .unwrap_or("@pacha:cyberendroit.net")
    }
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            config: "config.toml".to_string(),
            matrix_homeserver_url: None,
            matrix_bot_username: None,
            matrix_bot_password: None,
            stych_email: None,
            stych_password: None,
            target_user: None,
        }
    }
}
