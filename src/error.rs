//! Custom error types for the lilo-et-stych application

use std::error::Error as StdError;
use std::fmt;

/// Main error type for the application
#[derive(Debug)]
pub enum AppError {
    /// Configuration-related errors
    Config(String),
    /// Network/HTTP-related errors
    Network(String),
    /// Authentication errors
    Auth(String),
    /// JSON parsing errors
    Parse(String),
    /// Matrix SDK errors
    Matrix(String),
    /// General application errors
    General(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AppError::Matrix(msg) => write!(f, "Matrix error: {}", msg),
            AppError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl StdError for AppError {}

impl From<config::ConfigError> for AppError {
    fn from(error: config::ConfigError) -> Self {
        AppError::Config(error.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::Network(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::Parse(error.to_string())
    }
}

impl From<matrix_sdk::Error> for AppError {
    fn from(error: matrix_sdk::Error) -> Self {
        AppError::Matrix(error.to_string())
    }
}

/// Result type alias for the application
pub type AppResult<T> = Result<T, AppError>;
