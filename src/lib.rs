//! SQL Stream - A production-grade CLI tool for querying CSV/JSON files with SQL
//!
//! This library provides a high-performance SQL query engine built on Apache DataFusion
//! and Apache Arrow for executing SQL queries against CSV and JSON files using a zero-copy,
//! streaming architecture.
//!
//! # Example
//!
//! ```no_run
//! use sql_stream::{QueryEngine, SqlStreamError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), SqlStreamError> {
//!     let mut engine = QueryEngine::new()?;
//!     engine.register_file("data.csv", "my_table").await?;
//!     
//!     let results = engine.execute_query("SELECT * FROM my_table").await?;
//!     engine.print_results(results).await?;
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod engine;
pub mod error;

// Re-export key types for library consumers
pub use cli::CliArgs;
pub use engine::QueryEngine;
pub use error::SqlStreamError;
