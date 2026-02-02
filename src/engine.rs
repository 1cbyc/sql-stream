//! Query engine implementation using Apache DataFusion
//!
//! This module provides the core query execution engine built on Apache DataFusion,
//! with support for registering CSV and JSON files as tables and executing SQL queries
//! with streaming result processing.

use crate::error::{Result, SqlStreamError};
use datafusion::arrow::util::pretty::print_batches;
use datafusion::prelude::*;
use std::path::Path;
use tracing::{debug, info, instrument};

/// High-performance SQL query engine powered by Apache DataFusion
///
/// The `QueryEngine` manages a DataFusion `SessionContext` and provides
/// methods for registering data files and executing SQL queries with
/// zero-copy streaming.
pub struct QueryEngine {
    ctx: SessionContext,
}

impl QueryEngine {
    /// Create a new query engine with default configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the session context cannot be initialized
    #[instrument]
    pub fn new() -> Result<Self> {
        info!("Initializing query engine");
        let ctx = SessionContext::new();
        Ok(Self { ctx })
    }

    /// Register a CSV or JSON file as a table in the query engine
    ///
    /// The file format is automatically detected from the file extension.
    /// Supported formats: `.csv`, `.json`
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the data file
    /// * `table_name` - Name to use for the table in SQL queries
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file does not exist
    /// - The file format is unsupported
    /// - Schema inference fails
    /// - Table registration fails
    #[instrument(skip(self))]
    pub async fn register_file(&mut self, file_path: &str, table_name: &str) -> Result<()> {
        let path = Path::new(file_path);

        // Check if file exists
        if !path.exists() {
            return Err(SqlStreamError::FileNotFound(path.to_path_buf()));
        }

        info!("Registering file: {} as table: {}", file_path, table_name);

        // Detect file format from extension
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| {
                SqlStreamError::UnsupportedFormat(
                    path.to_string_lossy().to_string()
                )
            })?;

        match extension.to_lowercase().as_str() {
            "csv" => {
                debug!("Detected CSV format");
                self.ctx
                    .register_csv(table_name, file_path, CsvReadOptions::new())
                    .await
                    .map_err(|e| {
                        SqlStreamError::TableRegistration(
                            table_name.to_string(),
                            e.to_string(),
                        )
                    })?;
            }
            "json" => {
                debug!("Detected JSON format");
                self.ctx
                    .register_json(table_name, file_path, NdJsonReadOptions::default())
                    .await
                    .map_err(|e| {
                        SqlStreamError::TableRegistration(
                            table_name.to_string(),
                            e.to_string(),
                        )
                    })?;
            }
            _ => {
                return Err(SqlStreamError::UnsupportedFormat(
                    extension.to_string()
                ));
            }
        }

        info!("Successfully registered table: {}", table_name);
        Ok(())
    }

    /// Execute a SQL query and return the results as a DataFrame
    ///
    /// # Arguments
    ///
    /// * `sql` - SQL query string to execute
    ///
    /// # Errors
    ///
    /// Returns an error if query parsing or execution fails
    #[instrument(skip(self))]
    pub async fn execute_query(&self, sql: &str) -> Result<DataFrame> {
        info!("Executing SQL query");
        debug!("Query: {}", sql);

        let df = self.ctx.sql(sql).await.map_err(|e| {
            SqlStreamError::QueryExecution(e.to_string())
        })?;

        Ok(df)
    }

    /// Execute a SQL query and print the results to stdout
    ///
    /// Uses Arrow's pretty printer for formatted table output with
    /// streaming to handle large result sets efficiently.
    ///
    /// # Arguments
    ///
    /// * `dataframe` - The DataFrame to print
    ///
    /// # Errors
    ///
    /// Returns an error if result collection or printing fails
    #[instrument(skip(self, dataframe))]
    pub async fn print_results(&self, dataframe: DataFrame) -> Result<()> {
        info!("Collecting and printing results");

        // Collect results as RecordBatches
        let batches = dataframe.collect().await?;

        // Print using Arrow's pretty printer
        print_batches(&batches).map_err(|e| {
            SqlStreamError::QueryExecution(format!("Failed to print results: {}", e))
        })?;

        let total_rows: usize = batches.iter().map(|b| b.num_rows()).sum();
        info!("Query returned {} rows", total_rows);

        Ok(())
    }
}

impl Default for QueryEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default QueryEngine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let engine = QueryEngine::new();
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_file_not_found() {
        let mut engine = QueryEngine::new().unwrap();
        let result = engine.register_file("nonexistent.csv", "test").await;
        assert!(matches!(result, Err(SqlStreamError::FileNotFound(_))));
    }
}
