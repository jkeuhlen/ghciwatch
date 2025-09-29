# ghciwatch Benchmarks

This directory contains performance benchmarks for critical paths in ghciwatch.

## Running Benchmarks

### Run all benchmarks
```bash
cargo bench
```

The benchmarks are pre-configured with stability settings:
- 2-3 second warm-up time (critical paths use 3s)
- 3-5 second measurement time (critical paths use 5s)
- 100-200 sample size (critical paths use 200)

### Override default settings
```bash
# Run with custom parameters (must specify which benchmark)
cargo bench --bench ghci_parsing -- --warm-up-time 5 --measurement-time 10 --sample-size 500

# Quick run for testing
cargo bench --bench file_events -- --quick
```

### Run specific benchmark suite
```bash
cargo bench --bench ghci_parsing
cargo bench --bench incremental_reader
cargo bench --bench file_events
cargo bench --bench eval_parsing
cargo bench --bench tui_rendering
```

### Run specific benchmark within a suite
```bash
cargo bench --bench ghci_parsing -- parse_ghc_messages
```

### Generate HTML reports
```bash
cargo bench -- --output-format bencher
```

Reports are saved to `target/criterion/` with interactive HTML graphs.

## Benchmark Suites

### 1. GHCi Output Parsing (`ghci_parsing`)
Tests the performance of parsing GHC compiler messages, warnings, and errors.

**Key metrics:**
- Parsing throughput for various message types
- ANSI escape sequence stripping performance
- Handling of large error cascades

### 2. Incremental Reader (`incremental_reader`)
Benchmarks the streaming reader that processes GHCi output.

**Key metrics:**
- Pattern matching with Aho-Corasick
- UTF-8 decoding performance
- Buffer management and reallocation

### 3. File Event Processing (`file_events`)
Measures file watching and event filtering performance.

**Key metrics:**
- Event classification speed
- Path canonicalization
- BTreeSet operations for deduplication

### 4. Eval Command Parsing (`eval_parsing`)
Tests parsing of eval comments in Haskell source files.

**Key metrics:**
- Winnow parser performance
- Line/column calculation
- Handling of various eval command styles

### 5. TUI Rendering (`tui_rendering`)
Benchmarks terminal UI rendering and scrolling operations.

**Key metrics:**
- ANSI to TUI conversion
- Scrollback buffer management
- Event handling responsiveness

## Profiling

### Using flamegraph
```bash
cargo install flamegraph
cargo flamegraph --bench ghci_parsing
```

### Using perf (Linux)
```bash
cargo bench --bench incremental_reader -- --profile-time 10
```

## Baseline Comparison

Save a baseline before making changes:
```bash
cargo bench -- --save-baseline main
```

Compare after changes:
```bash
cargo bench -- --baseline main
```

## CI Integration

Benchmarks run automatically on PRs to detect performance regressions.
See `.github/workflows/benchmarks.yml` for configuration.

## Adding New Benchmarks

1. Create a new file in `benches/` directory
2. Add benchmark configuration to `Cargo.toml`:
   ```toml
   [[bench]]
   name = "your_benchmark"
   harness = false
   ```
3. Use criterion macros to define benchmark groups
4. Include realistic test data in `benches/fixtures/` if needed

## Benchmark Stability Tips

To get consistent benchmark results:

1. **Close unnecessary applications** - Reduce CPU contention
2. **Disable system updates** - Prevent background downloads/installs
3. **Use consistent power settings** - Plug in laptops, disable power saving
4. **Run multiple times** - Look for patterns across runs
5. **Use longer warm-up times** - Allow CPU caches and branch predictors to stabilize
6. **Pin to specific CPU cores** (Linux) - Use `taskset -c 0` to reduce migration

The benchmarks are configured with:
- 3 second warm-up time for critical paths
- 5 second measurement time
- 200 sample size
- `iter_batched` for allocation-heavy benchmarks

## Performance Goals

- **GHCi parsing**: < 1ms for typical error messages
- **Incremental reader**: > 100 MB/s throughput
- **File events**: < 100μs per event
- **Eval parsing**: < 10ms for 1000 line file
- **TUI rendering**: 60+ FPS scrolling
