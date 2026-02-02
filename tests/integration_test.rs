//! Integration tests for SQL Stream CLI
//!
//! These tests verify the end-to-end functionality of the query engine
//! with real CSV and JSON files.

use sql_stream::{QueryEngine, SqlStreamError};
use std::path::PathBuf;

/// Helper function to get the path to test fixtures
fn fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(filename)
}

#[tokio::test]
async fn test_csv_simple_select() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");

    engine
        .register_file(csv_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT * FROM employees LIMIT 5")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert!(!batches.is_empty());
    assert!(batches[0].num_rows() <= 5);
}

#[tokio::test]
async fn test_csv_aggregation() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");

    engine
        .register_file(csv_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT COUNT(*) as total, AVG(age) as avg_age FROM employees")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].num_rows(), 1);
}

#[tokio::test]
async fn test_csv_where_clause() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");

    engine
        .register_file(csv_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT name, salary FROM employees WHERE age > 30 ORDER BY salary DESC")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert!(!batches.is_empty());
}

#[tokio::test]
async fn test_csv_group_by() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");

    engine
        .register_file(csv_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT city, COUNT(*) as count FROM employees GROUP BY city ORDER BY count DESC")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert!(!batches.is_empty());
}

#[tokio::test]
async fn test_json_select() {
    let mut engine = QueryEngine::new().unwrap();
    let json_path = fixture_path("sample.json");

    engine
        .register_file(json_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT name, city FROM employees WHERE salary > 70000")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert!(!batches.is_empty());
}

#[tokio::test]
async fn test_json_aggregation() {
    let mut engine = QueryEngine::new().unwrap();
    let json_path = fixture_path("sample.json");

    engine
        .register_file(json_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT MAX(salary) as max_salary, MIN(salary) as min_salary FROM employees")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert_eq!(batches.len(), 1);
}

#[tokio::test]
async fn test_invalid_file() {
    let mut engine = QueryEngine::new().unwrap();

    let result = engine
        .register_file("nonexistent.csv", "test")
        .await;

    assert!(matches!(result, Err(SqlStreamError::FileNotFound(_))));
}

#[tokio::test]
async fn test_unsupported_format() {
    // Create a temporary file with unsupported extension
    use std::fs::File;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let mut engine = QueryEngine::new().unwrap();

    let result = engine
        .register_file(file_path.to_str().unwrap(), "test")
        .await;

    assert!(matches!(result, Err(SqlStreamError::UnsupportedFormat(_))));
}

#[tokio::test]
async fn test_invalid_sql() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");

    engine
        .register_file(csv_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let result = engine
        .execute_query("INVALID SQL QUERY")
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_multiple_tables() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");
    let json_path = fixture_path("sample.json");

    engine
        .register_file(csv_path.to_str().unwrap(), "csv_data")
        .await
        .unwrap();

    engine
        .register_file(json_path.to_str().unwrap(), "json_data")
        .await
        .unwrap();

    // Query both tables
    let df = engine
        .execute_query("SELECT COUNT(*) FROM csv_data")
        .await
        .unwrap();

    let batches = df.collect().await.unwrap();
    assert_eq!(batches.len(), 1);
}

#[tokio::test]
async fn test_print_results() {
    let mut engine = QueryEngine::new().unwrap();
    let csv_path = fixture_path("sample.csv");

    engine
        .register_file(csv_path.to_str().unwrap(), "employees")
        .await
        .unwrap();

    let df = engine
        .execute_query("SELECT * FROM employees LIMIT 3")
        .await
        .unwrap();

    // This should not panic
    let result = engine.print_results(df).await;
    assert!(result.is_ok());
}
