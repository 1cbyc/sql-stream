//! Error types for the SQL Stream library
//!
//! This module defines all error types used throughout the library using `thiserror`
//! for ergonomic error handling and proper error propagation.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for SQL Stream operations
#[derive(Error, Debug)]
pub enum SqlStreamError {
    /// File-related errors
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    /// Invalid file format or extension
    #[error("Unsupported file format: {0}. Supported formats: .csv, .json")]
    UnsupportedFormat(String),

    /// DataFusion-related errors
    #[error("DataFusion error: {0}")]
    DataFusion(#[from] datafusion::error::DataFusionError),

    /// Arrow-related errors
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// SQL execution errors
    #[error("SQL execution failed: {0}")]
    QueryExecution(String),

    /// Table registration errors
    #[error("Failed to register table '{0}': {1}")]
    TableRegistration(String, String),

    /// Schema inference errors
    #[error("Failed to infer schema from file: {0}")]
    SchemaInference(String),
}

/// Type alias for Results using SqlStreamError
pub type Result<T> = std::result::Result<T, SqlStreamError>;
