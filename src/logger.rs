//! Logging utilities for the lilo-et-stych application

use env_logger::{Builder, Target};
use log::{debug, error, info, warn, LevelFilter};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the logger with the specified log level
pub fn init_logger(log_level: Option<LevelFilter>) {
    INIT.call_once(|| {
        let level = log_level.unwrap_or(LevelFilter::Info);

        Builder::from_default_env()
            .target(Target::Stdout)
            .filter_level(level)
            .format_timestamp_secs()
            .init();

        info!("Logger initialized with level: {}", level);
    });
}

/// Log an informational message
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

/// Log a warning message
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*)
    };
}

/// Log an error message
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*)
    };
}

/// Log a debug message
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}

/// Log the start of an operation
pub fn log_operation_start(operation: &str) {
    info!("🚀 Starting operation: {}", operation);
}

/// Log the successful completion of an operation
pub fn log_operation_success(operation: &str) {
    info!("✅ Operation completed successfully: {}", operation);
}

/// Log the failure of an operation
pub fn log_operation_failure(operation: &str, error: &str) {
    error!("❌ Operation failed: {} - {}", operation, error);
}

/// Log authentication status
pub fn log_auth_status(success: bool, message: &str) {
    if success {
        info!("🔐 Authentication successful: {}", message);
    } else {
        error!("🔒 Authentication failed: {}", message);
    }
}

/// Log network request details
pub fn log_request(method: &str, url: &str) {
    debug!("📡 {} request to: {}", method, url);
}

/// Log response details
pub fn log_response(status: u16, message: &str) {
    if status < 400 {
        debug!("📨 Response: {} - {}", status, message);
    } else {
        warn!("📨 Response: {} - {}", status, message);
    }
}
