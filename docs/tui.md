# TUI Mode

Ghciwatch includes an experimental TUI (Terminal User Interface) mode that provides an interactive, terminal-based interface with user-configurable actions.

**Note:** TUI mode is experimental and may contain bugs or change drastically in future releases.

## Enabling TUI Mode

Start ghciwatch with the `--tui` flag:

```bash
ghciwatch --tui
```

Or set the `GHCIWATCH_TUI` environment variable:

```bash
export GHCIWATCH_TUI=true
ghciwatch
```

You can add this to your shell configuration (e.g., `~/.zshrc` or `~/.bashrc`) to enable TUI mode by default.

## Features

### Interactive Display

The TUI provides a clean, scrollable view of GHCi output with:
- Real-time compilation results
- Vim-style navigation (`j`/`k` to scroll, `g`/`G` to jump to top/bottom)
- Scroll with mouse wheel or `Ctrl+u`/`Ctrl+d`

### User-Configurable Actions

The TUI includes a customizable action bar at the bottom of the screen with keyboard shortcuts for common tasks.

#### Default Actions

The TUI includes three built-in actions:

1. **Reload All** (key `1`) - Reloads all files that have changed in git:
   ```bash
   cd "$(git rev-parse --show-toplevel)" && git diff --name-only | xargs -r touch
   ```
   - Changes to the git root directory to ensure paths are correct
   - Lists all modified files according to git
   - Touches them to trigger a reload (the `-r` flag prevents errors if there are no changes)

2. **Toggle Warnings** (key `2`) - Toggles the `--track-warnings` setting:
   - Uses the internal command `@toggle-track-warnings`
   - Triggers a full restart to apply the new setting
   - Displays a message showing the new state (enabled/disabled)

3. **Toggle No-Load** (key `3`) - Toggles the `--repl-no-load` flag:
   - Uses the internal command `@toggle-no-load`
   - Adds or removes `--repl-no-load` from the GHCi command
   - Triggers a full restart to apply the new setting
   - When enabled, modules are loaded on-demand; when disabled, all modules load at startup
   - See [no-load.md](no-load.md) for more information about this feature

#### Adding Custom Actions

Add custom actions with the `--tui-action` flag:

```bash
# Single custom action
ghciwatch --tui --tui-action "Run Tests:cabal test"

# Multiple custom actions
ghciwatch --tui \
  --tui-action "Run Tests:cabal test" \
  --tui-action "Format Code:fourmolu -i src/" \
```

### Action Format

Actions are defined as `LABEL:COMMAND` where COMMAND can be:

#### Shell Commands

```bash
--tui-action "Run Tests:cabal test"
```

- `LABEL`: Text shown in the TUI interface
- `COMMAND`: Any valid shell command to execute
- Commands are run through `sh -c`, so shell features like pipes and redirection work

#### Internal Commands

```bash
--tui-action "Toggle Warnings:@toggle-track-warnings"
```

- Prefix the command with `@` to make it an internal command
- Internal commands modify ghciwatch runtime settings
- Currently supported internal commands:
  - `@toggle-track-warnings` - Toggle warning tracking on/off (triggers restart)
  - `@toggle-no-load` - Toggle `--repl-no-load` flag (triggers restart)

### Keyboard Shortcuts

#### Navigation
- `j` - Scroll down one line
- `k` - Scroll up one line
- `g` - Jump to top
- `G` (Shift+g) - Jump to bottom
- `Ctrl+u` - Scroll up half a screen
- `Ctrl+d` - Scroll down half a screen
- `Ctrl+e` - Scroll down one line (alternative)
- `Ctrl+y` - Scroll up one line (alternative)
- Mouse wheel - Scroll up/down

#### Actions
- `1-9` - Trigger the corresponding action
- `a` - Toggle action bar visibility
- `q` - Quit (requires confirmation with `q` or `y`)
- `Ctrl+C` - Immediate quit (no confirmation)

#### Debug
- `` ` `` - Hide debug info
- `~` (Shift+`) - Show debug info

## UI Layout

The TUI uses minimal screen space:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  GHCi output and compilation results                │
│  (scrollable area)                                  │
│                                                     │
├─────────────────────────────────────────────────────┤
│ [1] Reload All | [2] Toggle Warnings | [a] hide |  │
│ [q] quit                                            │
└─────────────────────────────────────────────────────┘
```

The action bar uses only **1 line** at the bottom and can be hidden entirely with the `a` key.

## Limitations

- Maximum of 9 actions can be defined (including the 3 defaults)
- Actions are run sequentially, not in parallel
- Shell commands do not have access to the GHCi session state
- Internal commands are limited to the predefined set
- Toggling settings requires a full GHCi restart

## Example Workflows

### Development with Tests

```bash
ghciwatch --tui \
  --tui-action "Run Tests:cabal test" \
  --tui-action "Quick Test:cabal test --test-option=--match=/MyModule/"
```

Press `1` to reload changed files, `2` to toggle warnings, `3` to toggle no-load mode, `4` to run all tests, `5` to run focused tests.

### Code Quality

```bash
ghciwatch --tui \
  --tui-action "Format:fourmolu -i src/" \
  --tui-action "Lint:hlint src/"
```

Press `1` to reload, `2` to toggle warnings, `3` to toggle no-load mode, `4` to format code, `5` to run linter.

### Multi-Package Projects

```bash
ghciwatch --tui \
  --tui-action "Build Backend:cabal build backend" \
  --tui-action "Build Frontend:cabal build frontend" \
  --tui-action "Test All:cabal test all"
```

Quickly build different components or run the full test suite.
