//! SQL Stream - Binary entry point
//!
//! This is the main entry point for the sql-stream CLI application.
//! It handles initialization, signal handling, and orchestrates the query execution.

use anyhow::{Context, Result};
use sql_stream::{CliArgs, QueryEngine};
use tokio::signal;
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = CliArgs::parse();

    // Initialize tracing/logging based on verbosity
    init_tracing(args.verbose);

    info!("SQL Stream CLI starting");

    // Validate CLI arguments
    if let Err(e) = args.validate() {
        error!("Validation error: {}", e);
        anyhow::bail!("{}", e);
    }

    // Setup graceful shutdown handler
    let shutdown_handle = tokio::spawn(async {
        match signal::ctrl_c().await {
            Ok(()) => {
                warn!("Received shutdown signal (Ctrl+C), terminating gracefully...");
                std::process::exit(0);
            }
            Err(err) => {
                error!("Failed to listen for shutdown signal: {}", err);
            }
        }
    });

    // Run the query
    let result = run_query(&args).await;

    // Abort shutdown handler if query completes normally
    shutdown_handle.abort();

    match result {
        Ok(()) => {
            info!("Query executed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Query execution failed: {}", e);
            Err(e)
        }
    }
}

/// Execute the SQL query against the provided file
async fn run_query(args: &CliArgs) -> Result<()> {
    // Create query engine
    let mut engine = QueryEngine::new()
        .context("Failed to initialize query engine")?;

    // Register the file as a table
    engine
        .register_file(
            args.file.to_str().context("Invalid file path")?,
            &args.table_name,
        )
        .await
        .context("Failed to register file")?;

    info!(
        "Registered file '{}' as table '{}'",
        args.file.display(),
        args.table_name
    );

    // Execute the query
    let dataframe = engine
        .execute_query(&args.query)
        .await
        .context("Failed to execute query")?;

    // Print results
    engine
        .print_results(dataframe)
        .await
        .context("Failed to print results")?;

    Ok(())
}

/// Initialize tracing subscriber with appropriate log level
fn init_tracing(verbose: bool) {
    let filter = if verbose {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false))
        .with(filter)
        .init();
}
