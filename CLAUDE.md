# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ghciwatch is a GHCi-based file watcher and recompiler for Haskell projects written in Rust. It loads a GHCi session and automatically reloads it when source files change, providing fast feedback during Haskell development.

## Common Development Commands

### Build
```bash
cargo build                    # Build debug version
cargo build --release          # Build optimized release version
```

### Test
```bash
cargo nextest run              # Run tests with nextest (preferred, faster)
cargo test                     # Alternative test runner
cargo llvm-cov nextest         # Run tests with coverage
```

### Lint & Format
```bash
cargo clippy                  # Run linter
cargo fmt                     # Format code
cargo check                   # Check compilation without building
```

### Documentation
```bash
cargo doc --document-private-items --no-deps --workspace  # Generate API docs
cargo run --features clap-markdown -- --generate-markdown-help > docs/cli.md  # Update CLI docs
mdbook build docs              # Build user manual
```

## High-Level Architecture

The codebase follows a modular architecture with clear separation of concerns:

### Core Components

1. **Main Entry Point** (`src/main.rs`, `src/lib.rs`): Initializes the application, parses CLI arguments, and orchestrates the three main subsystems below.

2. **GHCi Management** (`src/ghci/`): 
   - `manager.rs`: Handles lifecycle of the GHCi process including startup, shutdown, and reload coordination
   - `process.rs`: Manages the actual GHCi subprocess
   - `stdin.rs`, `stdout.rs`, `stderr.rs`: Handle I/O streams from GHCi
   - `parse/`: Contains parsers for GHCi output including error messages, module information, and evaluation comments
   - `warning_tracker.rs`: Tracks and manages GHC warnings across reloads

3. **File Watching** (`src/watcher.rs`): Monitors filesystem changes and triggers reload events. Uses the `notify` crate for cross-platform file watching.

4. **TUI (Terminal UI)** (`src/tui/`): Optional terminal interface for displaying compilation results in a structured format using ratatui.
   - Supports user-configurable actions accessible via number keys (1-9)
   - Actions are shell commands that can be triggered from the TUI interface
   - Default action includes "Reload All" which touches all changed git files

### Key Architectural Patterns

- **Async/Await**: Uses Tokio for concurrent operations, allowing GHCi process management, file watching, and UI updates to run simultaneously
- **Channel-based Communication**: Components communicate via mpsc channels (e.g., `WatcherEvent` sent from watcher to GHCi manager)
- **Graceful Shutdown**: `ShutdownManager` coordinates clean shutdown across all subsystems
- **Hook System** (`src/hooks.rs`): Extensible lifecycle hooks allow running custom commands at various points (startup, before/after reload, etc.)

### Data Flow

1. File watcher detects changes → sends `WatcherEvent` to GHCi manager
2. GHCi manager processes event → sends reload command to GHCi process  
3. GHCi output is parsed → compilation results displayed to user
4. Lifecycle hooks execute at appropriate points
5. TUI (if enabled) continuously updates display with latest compilation state

## TUI Custom Actions

The TUI mode supports user-configurable actions that can be triggered via keyboard shortcuts. This makes it easy to run common workspace-specific commands without leaving ghciwatch.

### Usage

Add custom actions with the `--tui-action` flag:

```bash
# Single custom action
ghciwatch --tui --tui-action "Run Tests:cabal test"

# Multiple custom actions
ghciwatch --tui \
  --tui-action "Run Tests:cabal test" \
  --tui-action "Format Code:fourmolu -i src/" \
  --tui-action "Check Types:cabal build --ghc-options=-fno-code"
```

### Default Actions

By default, the TUI includes a "Reload All" action (key `1`) that reloads all files that have changed in git:
```bash
cd "$(git rev-parse --show-toplevel)" && git diff --name-only | xargs -r touch
```

This command:
- Changes to the git root directory to ensure paths are correct
- Lists all modified files according to git
- Touches them to trigger a reload (the `-r` flag prevents errors if there are no changes)

### Using Actions

- Press `a` to toggle the action list visibility
- Press number keys `1-9` to trigger the corresponding action
- The action list shows at the bottom of the TUI display
- Maximum of 9 actions can be defined (including the default)

### Action Format

Actions are defined as `LABEL:SHELL_COMMAND`:
- `LABEL`: Text shown in the TUI interface
- `SHELL_COMMAND`: Any valid shell command to execute

The commands are run through `sh -c`, so shell features like pipes and redirection work as expected.

## Development Notes

- The project uses Nix for development environment setup (`nix develop`)
- Integration tests in `tests/` require a Haskell toolchain (GHC, cabal, hpack)
- Test harness is in `test-harness/` workspace member
- Minimum supported Rust version appears to be 1.72 based on dependency constraints