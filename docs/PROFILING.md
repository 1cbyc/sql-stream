# Performance Profiling Guide

This guide covers how to profile and optimize the performance of SQL Stream.

## Prerequisites

Install the flamegraph tool:

```bash
cargo install flamegraph
```

On Linux, you may need to install perf:

```bash
# Ubuntu/Debian
sudo apt-get install linux-tools-common linux-tools-generic

# Fedora/RHEL
sudo dnf install perf
```

On Windows, use the built-in profiling tools or WSL2.

## Generating Flamegraphs

### Basic Flamegraph

```bash
# On Linux (requires sudo for perf access)
sudo cargo flamegraph -- -f large_dataset.csv -q "SELECT * FROM data"

# Output: flamegraph.svg
```

### Profiling Specific Queries

```bash
# Profile aggregation query
sudo cargo flamegraph -- -f data.csv -q "SELECT category, COUNT(*), AVG(price) FROM data GROUP BY category"

# Profile sorting
sudo cargo flamegraph -- -f data.csv -q "SELECT * FROM data ORDER BY timestamp DESC LIMIT 1000"

# Profile joins (requires registering multiple tables in code)
```

### Interpreting Results

The flamegraph shows:
- **Width**: Time spent in each function
- **Height**: Call stack depth
- **Color**: Different modules/crates

Look for:
1. **Wide bars at the top**: Hot paths in your code
2. **DataFusion internals**: Normal to see significant time here
3. **I/O operations**: File reading bottlenecks
4. **Serialization**: Arrow formatting overhead

## Benchmarking

### Create Test Datasets

Generate large CSV files for realistic testing:

```bash
# Python script to generate test data
python3 << 'EOF'
import csv
import random

with open('large_test.csv', 'w', newline='') as f:
    writer = csv.writer(f)
    writer.writerow(['id', 'name', 'value', 'category', 'timestamp'])
    
    for i in range(1000000):  # 1 million rows
        writer.writerow([
            i,
            f'Item_{i}',
            random.randint(1, 10000),
            f'Category_{random.randint(1, 100)}',
            f'2024-{random.randint(1,12):02d}-{random.randint(1,28):02d}'
        ])
EOF
```

### Timing Queries

Use the built-in time command:

```bash
# Linux/macOS
time cargo run --release -- -f large_test.csv -q "SELECT category, COUNT(*), AVG(value) FROM data GROUP BY category"

# PowerShell (Windows)
Measure-Command { cargo run --release -- -f large_test.csv -q "SELECT category, COUNT(*), AVG(value) FROM data GROUP BY category" }
```

## Performance Tips

### 1. File Format Selection

- **CSV**: Better for wide tables with many columns
- **JSON**: Better for nested or semi-structured data
- **Parquet** (future): Best for analytical workloads (consider adding support)

### 2. Query Optimization

```sql
-- Good: Filter early
SELECT name FROM data WHERE category = 'A' ORDER BY value LIMIT 10

-- Bad: Filter late  
SELECT * FROM data ORDER BY value LIMIT 1000 -- then filter in app
```

### 3. Memory Considerations

DataFusion streams results, but aggregations require materialization:

```sql
-- Low memory: Streaming
SELECT * FROM data WHERE value > 100

-- Higher memory: Full aggregation
SELECT category, SUM(value) FROM data GROUP BY category
```

### 4. Build Configuration

Always use release mode for production:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## Bottleneck Checklist

If queries are slow, check:

1. **File I/O**: Is disk reading the bottleneck?
   - Use SSD instead of HDD
   - Consider file compression
   
2. **CPU**: Is computation the bottleneck?
   - Check multi-core utilization
   - Optimize SQL query
   
3. **Memory**: Is swapping occurring?
   - Reduce batch size in DataFusion config
   - Use LIMIT clauses for exploratory queries

4. **Network**: For remote files
   - Consider local caching
   - Use faster network connection

## Continuous Performance Testing

Add benchmark tests:

```rust
// benches/benchmark.rs (requires criterion crate)
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_query(c: &mut Criterion) {
    c.bench_function("csv_aggregation", |b| {
        b.iter(|| {
            // Your query logic here
        });
    });
}

criterion_group!(benches, benchmark_query);
criterion_main!(benches);
```

## Resources

- [DataFusion Performance Guide](https://arrow.apache.org/datafusion/user-guide/performance.html)
- [Arrow Memory Model](https://arrow.apache.org/docs/format/Columnar.html)
- [Flamegraph Documentation](https://github.com/flamegraph-rs/flamegraph)
