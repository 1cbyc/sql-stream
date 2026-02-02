# SQL Stream

A CLI tool for executing SQL queries against CSV/JSON files using a zero-copy, streaming architecture powered by Apache DataFusion and Apache Arrow.

## Installation

### From Source

```bash
git clone https://github.com/1cbyc/sql-stream.git
cd sql-stream
cargo build --release
```

The binary will be available at `target/release/sql-stream`.

### Using Cargo

```bash
cargo install sql-stream
```

## Usage

### Basic Query on CSV

```bash
sql-stream -f data.csv -q "SELECT * FROM data LIMIT 10"
```

### Custom Table Name

```bash
sql-stream -f employees.csv -t employees -q "SELECT name, salary FROM employees WHERE age > 30"
```

### JSON Files

```bash
sql-stream -f data.json -q "SELECT COUNT(*) as total FROM data"
```

### Aggregations and Group By

```bash
sql-stream -f sales.csv -q "SELECT region, SUM(revenue) as total_revenue FROM data GROUP BY region ORDER BY total_revenue DESC"
```

### Enable Verbose Logging

```bash
sql-stream -f data.csv -q "SELECT * FROM data" --verbose
```

## Command Line Options

```
sql-stream -f <FILE> -q <SQL> [OPTIONS]

Options:
  -f, --file <FILE>           Path to CSV or JSON file (required)
  -q, --query <SQL>           SQL query to execute (required)
  -t, --table-name <NAME>     Table name for SQL queries (default: "data")
  -v, --verbose               Enable verbose debug logging
  -h, --help                  Print help information
  -V, --version               Print version information
```

## Examples

### Data Analysis

```bash
# Find top 5 highest salaries
sql-stream -f employees.csv -q "SELECT name, salary FROM data ORDER BY salary DESC LIMIT 5"

# Calculate average age by city
sql-stream -f employees.csv -q "SELECT city, AVG(age) as avg_age FROM data GROUP BY city"

# Filter and count
sql-stream -f sales.json -q "SELECT product, COUNT(*) as count FROM data WHERE revenue > 1000 GROUP BY product"
```

### Complex Queries

```bash
# Multiple aggregations
sql-stream -f data.csv -q "
  SELECT 
    category,
    COUNT(*) as count,
    AVG(price) as avg_price,
    MAX(price) as max_price,
    MIN(price) as min_price
  FROM data
  GROUP BY category
  HAVING count > 10
  ORDER BY avg_price DESC
"
```

## Architecture

SQL Stream is built with a modular architecture:

- **Engine Module** (`src/engine.rs`): DataFusion SessionContext management and query execution
- **CLI Module** (`src/cli.rs`): Argument parsing with clap
- **Error Module** (`src/error.rs`): Type-safe error handling with thiserror
- **Main Binary** (`src/main.rs`): Async runtime and orchestration

### Technology Stack

- **Apache DataFusion** (45.x): SQL query engine
- **Apache Arrow** (53.x): In-memory columnar format
- **Tokio**: Async runtime
- **Clap** (v4): CLI parsing
- **Tracing**: Structured logging

## Performance Considerations

- **Zero-Copy Streaming**: DataFusion processes data using Arrow's columnar format without unnecessary copying
- **Lazy Evaluation**: Queries are optimized before execution
- **Parallel Processing**: Multi-threaded execution for CPU-intensive operations
- **Memory Efficiency**: Streaming results prevent loading entire datasets into memory

For performance profiling, see [docs/PROFILING.md](docs/PROFILING.md).

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_csv_simple_select
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Generate documentation
cargo doc --open
```

## CI/CD

The project includes a GitHub Actions workflow that runs on every push and pull request:

- Format checking (`cargo fmt`)
- Linting (`cargo clippy`)
- Cross-platform tests (Linux, macOS, Windows)
- Release builds

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No clippy warnings (`cargo clippy`)
4. Documentation is updated

## License

This project is dual-licensed under MIT OR Apache-2.0.

## Acknowledgments

Built with:
- [Apache DataFusion](https://github.com/apache/arrow-datafusion)
- [Apache Arrow](https://github.com/apache/arrow-rs)
- [Tokio](https://tokio.rs/)
