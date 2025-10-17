# Changelog

All notable changes to ghciwatch will be documented in this file.

## [2.0.0-rc1] - 2025-10-16

### Major Features

- **Warning Tracking**: Comprehensive warning tracking system across GHCi reloads with structured display and per-module tracking
- **Cabal Multi-REPL**: Full support for `cabal multi-repl` to work with multi-component projects
- **Enhanced TUI**: User-configurable actions with key bindings (1-9), custom shell commands, and comprehensive TUI mode improvements
  - Added "Toggle Quiet Mode" as default TUI action (key 4)
- **Stdout Redirection**: Ability to redirect or suppress GHCi stdout output
- **Quiet Mode**: New `--quiet-stdout` flag and `GHCIWATCH_QUIET_STDOUT` environment variable to suppress compilation progress output for large projects (16000+ modules) while preserving diagnostics and status messages

### Improvements

- **Performance**: Incremental reader optimizations and ANSI handling improvements
  - Quiet mode can improve performance by 10-30% on large projects by eliminating terminal I/O overhead
- **Error Handling**: Graceful handling of broken pipe errors when files are deleted
- **CLI**: Better support for decimal time values (e.g., `--debounce 0.1s`)
- **Benchmarking**: Comprehensive benchmarking infrastructure for performance testing
- **User Feedback**: Status messages in quiet mode keep users informed of compilation progress

### Bug Fixes

- Fixed broken pipe errors causing crashes after file deletion
- Improved handling of modules that fail to start
- Enhanced ANSI escape sequence processing
