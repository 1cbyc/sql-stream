//! CLI argument parsing and validation
//!
//! This module defines the command-line interface using `clap` with derive macros
//! for a professional and user-friendly CLI experience.

use clap::Parser;
use std::path::PathBuf;

/// SQL Stream - Execute SQL queries against CSV/JSON files
///
/// A high-performance CLI tool powered by Apache DataFusion for running
/// SQL queries on CSV and JSON files using streaming architecture.
#[derive(Parser, Debug)]
#[command(
    name = "sql-stream",
    version,
    author,
    about = "Execute SQL queries against CSV/JSON files with streaming",
    long_about = "A production-grade CLI tool that executes SQL queries against CSV/JSON files \
                  using Apache DataFusion and Apache Arrow with zero-copy, streaming architecture."
)]
pub struct CliArgs {
    /// Path to the CSV or JSON file to query
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "Path to CSV or JSON file",
        required = true
    )]
    pub file: PathBuf,

    /// SQL query to execute
    #[arg(
        short = 'q',
        long = "query",
        value_name = "SQL",
        help = "SQL query string to execute",
        required = true
    )]
    pub query: String,

    /// Custom table name for the registered file
    #[arg(
        short = 't',
        long = "table-name",
        value_name = "NAME",
        help = "Table name to use in SQL queries",
        default_value = "data"
    )]
    pub table_name: String,

    /// Enable verbose debug logging
    #[arg(short = 'v', long = "verbose", help = "Enable verbose logging output")]
    pub verbose: bool,
}

impl CliArgs {
    /// Parse CLI arguments from command line
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sql_stream::CliArgs;
    /// let args = CliArgs::parse();
    /// ```
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Validate CLI arguments
    ///
    /// Performs additional validation beyond what clap provides
    ///
    /// # Errors
    ///
    /// Returns an error message if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Check if file exists
        if !self.file.exists() {
            return Err(format!("File not found: {}", self.file.display()));
        }

        // Check if file has a valid extension
        let extension = self
            .file
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| "File must have an extension (.csv or .json)".to_string())?;

        match extension.to_lowercase().as_str() {
            "csv" | "json" => Ok(()),
            _ => Err(format!(
                "Unsupported file extension: .{}. Supported: .csv, .json",
                extension
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_structure() {
        // This test ensures the CLI structure is valid
        // Actual parsing is tested via integration tests
        let args = CliArgs {
            file: PathBuf::from("test.csv"),
            query: "SELECT * FROM data".to_string(),
            table_name: "data".to_string(),
            verbose: false,
        };

        assert_eq!(args.table_name, "data");
        assert_eq!(args.query, "SELECT * FROM data");
    }
}
