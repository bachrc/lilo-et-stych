//! Main binary for the lilo-et-stych Matrix bot

use lilo_et_stych::{init_logger, log_info, monitoring::MonitoringService, AppConfig, AppResult};
use log::LevelFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Load configuration and initialize logger
    let config = AppConfig::load()?;
    init_logger(Some(LevelFilter::Info));

    log_info!("=== Lilo-et-Stych Matrix Bot ===");
    log_info!("Configuration loaded successfully");
    log_info!("- Matrix homeserver: {}", config.matrix.homeserver.url);
    log_info!("- Matrix bot: {}", config.matrix.bot.username);
    log_info!("- Stych email: {}", config.stych.email);
    log_info!("- Target user: {}", config.get_target_user());
    log_info!(
        "- Monitoring interval: {} seconds",
        config.monitoring.check_interval_seconds
    );

    // Create and start monitoring service
    log_info!("Initializing monitoring service...");
    let mut monitoring_service = MonitoringService::new(config).await?;

    log_info!("Starting monitoring service...");
    monitoring_service.start_monitoring().await
}
