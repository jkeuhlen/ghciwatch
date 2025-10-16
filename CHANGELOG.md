# Changelog

All notable changes to ghciwatch will be documented in this file.

## [2.0.0-rc1] - 2025-10-16

### Major Features

- **Warning Tracking**: Comprehensive warning tracking system across GHCi reloads with structured display and per-module tracking
- **Cabal Multi-REPL**: Full support for `cabal multi-repl` to work with multi-component projects
- **Enhanced TUI**: User-configurable actions with key bindings (1-9), custom shell commands, and comprehensive TUI mode improvements
- **Stdout Redirection**: Ability to redirect or suppress GHCi stdout output

### Improvements

- **Performance**: Incremental reader optimizations and ANSI handling improvements
- **Error Handling**: Graceful handling of broken pipe errors when files are deleted
- **CLI**: Better support for decimal time values (e.g., `--debounce 0.1s`)
- **Benchmarking**: Comprehensive benchmarking infrastructure for performance testing

### Bug Fixes

- Fixed broken pipe errors causing crashes after file deletion
- Improved handling of modules that fail to start
- Enhanced ANSI escape sequence processing
