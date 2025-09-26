//! # Lilo-et-Stych
//!
//! A Rust library for interacting with Stych.fr driving school platform and Matrix bot functionality.
//!
//! ## Features
//!
//! - **Stych.fr Integration**: Login and retrieve driving lesson planning
//! - **Matrix Bot**: Send messages and notifications via Matrix protocol
//! - **Course Management**: Find nearest available driving courses
//! - **Configuration**: Flexible configuration management
//! - **Error Handling**: Comprehensive error types and handling
//! - **Logging**: Structured logging throughout the application
//!
//! ## Usage
//!
//! ```rust
//! use lilo_et_stych::{StychClient, AppConfig, AppResult};
//!
//! #[tokio::main]
//! async fn main() -> AppResult<()> {
//!     // Load configuration
//!     let config = AppConfig::load()?;
//!     
//!     // Create Stych client
//!     let client = StychClient::new(config.stych);
//!     
//!     // Get nearest course
//!     let nearest_course = client.get_nearest_course().await?;
//!     
//!     if let Some(course) = nearest_course {
//!         println!("Nearest course: {}", course.description());
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Modules
//!
//! - [`config`]: Configuration management
//! - [`error`]: Custom error types
//! - [`http`]: HTTP client utilities
//! - [`logger`]: Logging utilities
//! - [`models`]: Data models for API responses
//! - [`stych`]: Stych.fr API client
//! - [`matrix`]: Matrix bot functionality

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod config;
pub mod error;
pub mod http;
pub mod logger;
pub mod matrix;
pub mod models;
pub mod monitoring;
pub mod stych;

// Re-export commonly used types
pub use config::AppConfig;
pub use error::{AppError, AppResult};
pub use matrix::MatrixBot;
pub use models::{CourseProposal, Instructor, PlanningResponse};
pub use stych::StychClient;

/// Initialize the application logger
pub fn init_logger(log_level: Option<log::LevelFilter>) {
    logger::init_logger(log_level);
}

/// Convenience function to get the nearest course with default configuration
pub async fn get_nearest_course() -> AppResult<Option<CourseProposal>> {
    let config = AppConfig::load().map_err(|e| AppError::Config(e.to_string()))?;
    let client = StychClient::new(config.stych);
    client.get_nearest_course().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // This test ensures all main types are properly exported
        let _config: Option<AppConfig> = None;
        let _error: Option<AppError> = None;
        let _course: Option<CourseProposal> = None;
        let _client: Option<StychClient> = None;
        let _bot: Option<MatrixBot> = None;
    }

    #[test]
    fn test_logger_initialization() {
        // Test that logger initialization doesn't panic
        init_logger(Some(log::LevelFilter::Debug));
    }
}
