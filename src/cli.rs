//! Command-line argument parser and argument access.
use std::time::Duration;

use camino::Utf8PathBuf;
use clap::builder::ValueParserFactory;
use clap::Parser;
use clap_complete::Shell;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::clap::FmtSpanParserFactory;
use crate::clap::RustBacktrace;
use crate::clonable_command::ClonableCommand;
use crate::ignore::GlobMatcher;
use crate::normal_path::NormalPath;

/// Ghciwatch loads a GHCi session for a Haskell project and reloads it
/// when source files change.
///
/// ## Examples
///
/// Load `cabal v2-repl` and watch for changes in `src`:
///
///     ghciwatch
///
/// Load a custom GHCi session and watch for changes in multiple locations:
///
///     ghciwatch --command "cabal v2-repl lib:test-dev" \
///               --watch src --watch test
///
/// Run tests after reloads:
///
///     ghciwatch --test-ghci TestMain.testMain \
///               --after-startup-ghci ':set args "--match=/OnlyRunSomeTests/"'
///
/// Use `hpack` to regenerate `.cabal` files:
///
///     ghciwatch --before-startup-shell hpack \
///               --restart-glob '**/package.yaml'
///
/// Also reload the session when `.persistentmodels` change:
///
///     ghciwatch --watch config/modelsFiles \
///               --reload-glob '**/*.persistentmodels'
///
/// Don't reload for `README.md` files:
///
///     ghciwatch --reload-glob '!src/**/README.md'
#[allow(rustdoc::invalid_rust_codeblocks)]
#[derive(Debug, Clone, Parser)]
#[command(
    version,
    author,
    verbatim_doc_comment,
    max_term_width = 100,
    override_usage = "ghciwatch [--command SHELL_COMMAND] [--watch PATH] [OPTIONS ...]"
)]
pub struct Opts {
    /// A shell command which starts a GHCi REPL, e.g. `ghci` or `cabal v2-repl` or similar.
    ///
    /// This is used to launch the underlying GHCi session that `ghciwatch` controls.
    ///
    /// May contain quoted arguments which will be parsed in a `sh`-like manner.
    #[arg(long, value_name = "SHELL_COMMAND")]
    pub command: Option<ClonableCommand>,

    /// A Haskell source file to load into a GHCi REPL.
    ///
    /// Shortcut for `--command 'ghci PATH'`. Conflicts with `--command`.
    #[arg(value_name = "FILE", conflicts_with = "command")]
    pub file: Option<NormalPath>,

    /// A file to write compilation errors to.
    ///
    /// The output format is compatible with `ghcid`'s `--outputfile` option.
    #[arg(long, alias = "outputfile", alias = "errors")]
    pub error_file: Option<Utf8PathBuf>,

    /// Evaluate Haskell code in comments.
    ///
    /// This parses line commands starting with `-- $>` or multiline commands delimited by `{- $>`
    /// and `<$ -}` and evaluates them after reloads.
    #[arg(long, alias = "allow-eval")]
    pub enable_eval: bool,

    /// Clear the screen before reloads and restarts.
    #[arg(long)]
    pub clear: bool,

    /// Don't interrupt reloads when files change.
    ///
    /// Depending on your workflow, `ghciwatch` may feel more responsive with this set.
    #[arg(long)]
    pub no_interrupt_reloads: bool,

    /// Enable TUI mode (experimental).
    #[arg(long, hide = true)]
    pub tui: bool,

    /// Options for TUI mode.
    #[command(flatten)]
    pub tui_opts: TuiOpts,

    /// Track warnings across recompilations.
    ///
    /// When enabled, warnings will be preserved in memory even when files are recompiled
    /// due to dependency changes, helping prevent "ephemeral warnings" from being missed.
    #[arg(long, env = "GHCIWATCH_TRACK_WARNINGS")]
    pub track_warnings: bool,

    /// Generate Markdown CLI documentation.
    #[cfg(feature = "clap-markdown")]
    #[arg(long, hide = true)]
    pub generate_markdown_help: bool,

    /// Generate `man` pages in the given directory.
    #[cfg(feature = "clap_mangen")]
    #[arg(long, hide = true)]
    pub generate_man_pages: Option<Utf8PathBuf>,

    /// Generate shell completions for the given shell.
    #[arg(long)]
    pub completions: Option<Shell>,

    /// Lifecycle hooks and commands to run at various points.
    #[command(flatten)]
    pub hooks: crate::hooks::HookOpts,

    /// Options to modify file watching.
    #[command(flatten)]
    pub watch: WatchOpts,

    /// Options to modify logging and error-handling behavior.
    #[command(flatten)]
    pub logging: LoggingOpts,
}

/// Options for watching files.
#[derive(Debug, Clone, clap::Args)]
#[clap(next_help_heading = "File watching options")]
pub struct WatchOpts {
    /// Use polling with the given interval rather than notification-based file watching.
    ///
    /// Polling tends to be more reliable and less performant. In particular, notification-based
    /// watching often misses updates on macOS.
    #[arg(long, value_name = "DURATION", value_parser = crate::clap::DurationValueParser::default())]
    pub poll: Option<Duration>,

    /// Debounce file events; wait this duration after receiving an event before attempting to
    /// reload.
    ///
    /// Defaults to 0.5 seconds.
    // Why do we need to use `value_parser` with this argument but not with the `Utf8PathBuf`
    // arguments? I have no clue!
    #[arg(
        long,
        default_value = "500ms",
        value_name = "DURATION",
        value_parser = crate::clap::DurationValueParser::default(),
    )]
    pub debounce: Duration,

    /// A path to watch for changes.
    ///
    /// Directories are watched recursively. Can be given multiple times.
    #[arg(long = "watch", value_name = "PATH")]
    pub paths: Vec<NormalPath>,

    /// Reload the GHCi session when paths matching this glob change.
    ///
    /// By default, only changes to Haskell source files trigger reloads. If you'd like to exclude
    /// some files from that, you can add an ignore glob here, like `!src/my-special-dir/**/*.hs`.
    ///
    /// Globs provided here have precisely the same semantics as a single line in a `gitignore`
    /// file (`man gitignore`), where the meaning of `!` is inverted: namely, `!` at the beginning
    /// of a glob will ignore a file.
    ///
    /// The last matching glob will determine if a reload is triggered.
    ///
    /// Can be given multiple times.
    #[arg(long = "reload-glob")]
    pub reload_globs: Vec<String>,

    /// Restart the GHCi session when paths matching this glob change.
    ///
    /// By default, only changes to `.cabal` or `.ghci` files will trigger restarts.
    ///
    /// See `--reload-globs` for more details.
    ///
    /// Can be given multiple times.
    ///
    /// [1]: https://gitlab.haskell.org/ghc/ghc/-/issues/11596
    #[arg(long = "restart-glob")]
    pub restart_globs: Vec<String>,
}

impl WatchOpts {
    /// Build the specified globs into a matcher.
    pub fn reload_globs(&self) -> miette::Result<GlobMatcher> {
        GlobMatcher::from_globs(self.reload_globs.iter())
    }

    /// Build the specified globs into a matcher.
    pub fn restart_globs(&self) -> miette::Result<GlobMatcher> {
        GlobMatcher::from_globs(self.restart_globs.iter())
    }
}

// TODO: Possibly set `RUST_LIB_BACKTRACE` from `RUST_BACKTRACE` as well, so that `full`
// enables source snippets for spantraces?
// https://docs.rs/color-eyre/latest/color_eyre/#multiple-report-format-verbosity-levels

/// Options to modify logging and error-handling behavior.
#[derive(Debug, Clone, clap::Args)]
#[clap(next_help_heading = "Logging options")]
pub struct LoggingOpts {
    /// Log message filter.
    ///
    /// Can be any of "error", "warn", "info", "debug", or "trace". Supports more granular
    /// filtering, as well.
    ///
    /// The grammar is: `target[span{field=value}]=level`, where `target` is a module path, `span`
    /// is a span name, and `level` is one of the levels listed above.
    ///
    /// See [documentation in `tracing-subscriber`][1].
    ///
    /// A nice value is `ghciwatch=debug`.
    ///
    /// [1]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
    #[arg(long, env = "GHCIWATCH_LOG", default_value = "ghciwatch=info")]
    pub log_filter: String,

    /// How to display backtraces in error messages.
    #[arg(long, env = "RUST_BACKTRACE", default_value = "0")]
    pub backtrace: RustBacktrace,

    /// When to log span events, which loosely correspond to tasks being run in the async runtime.
    ///
    /// Allows multiple values, comma-separated.
    #[arg(
        long,
        default_value = "new,close",
        value_delimiter = ',',
        value_parser = FmtSpanParserFactory::value_parser()
    )]
    pub trace_spans: Vec<FmtSpan>,

    /// Path to write JSON logs to.
    ///
    /// JSON logs are not yet stable and the format may change on any release.
    #[arg(long, value_name = "PATH")]
    pub log_json: Option<Utf8PathBuf>,
}

/// Options for TUI mode.
#[derive(Debug, Clone, clap::Args)]
#[clap(next_help_heading = "TUI options")]
pub struct TuiOpts {
    /// Define custom actions for TUI mode.
    ///
    /// Format: `LABEL:SHELL_COMMAND`. The label will be shown in the TUI, and the shell command
    /// will be executed when the action is triggered using number keys (1-9).
    ///
    /// Example: `--tui-action "Reload All:git diff --name-only | xargs touch"`
    ///
    /// Can be given multiple times (up to 9 actions).
    #[arg(long = "tui-action", value_name = "LABEL:SHELL_COMMAND")]
    pub actions: Vec<TuiAction>,
}

impl TuiOpts {
    /// Get all actions, including the default ones.
    pub fn get_actions(&self) -> Vec<TuiAction> {
        let mut actions = vec![
            TuiAction::default_reload_all(),
            TuiAction::default_toggle_warnings(),
            TuiAction::default_toggle_no_load(),
        ];
        actions.extend(self.actions.clone());
        actions.truncate(9); // Maximum of 9 actions (keys 1-9)
        actions
    }
}

impl Default for TuiOpts {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

/// A user-configurable action in TUI mode.
#[derive(Debug, Clone)]
pub struct TuiAction {
    /// The label to display in the TUI.
    pub label: String,
    /// The command to execute (either shell or internal).
    pub command: TuiActionCommand,
}

/// The command to execute for a TUI action.
#[derive(Debug, Clone)]
pub enum TuiActionCommand {
    /// A shell command to execute.
    Shell(String),
    /// An internal ghciwatch command.
    Internal(String),
}

impl TuiAction {
    /// The default "Reload All" action.
    pub fn default_reload_all() -> Self {
        Self {
            label: "Reload All".to_string(),
            // Run from git root to ensure paths are correct
            command: TuiActionCommand::Shell(
                "cd \"$(git rev-parse --show-toplevel)\" && git diff --name-only | xargs -r touch"
                    .to_string(),
            ),
        }
    }

    /// The default "Toggle Warnings" action.
    pub fn default_toggle_warnings() -> Self {
        Self {
            label: "Toggle Warnings".to_string(),
            command: TuiActionCommand::Internal("toggle-track-warnings".to_string()),
        }
    }

    /// The default "Toggle No-Load" action.
    pub fn default_toggle_no_load() -> Self {
        Self {
            label: "Toggle No-Load".to_string(),
            command: TuiActionCommand::Internal("toggle-no-load".to_string()),
        }
    }
}

impl std::str::FromStr for TuiAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid TUI action format '{}'. Expected 'LABEL:COMMAND' or 'LABEL:@INTERNAL_COMMAND'",
                s
            ));
        }
        let label = parts[0].trim().to_string();
        let command_str = parts[1].trim();

        let command = if let Some(internal_cmd) = command_str.strip_prefix('@') {
            TuiActionCommand::Internal(internal_cmd.to_string())
        } else {
            TuiActionCommand::Shell(command_str.to_string())
        };

        Ok(Self { label, command })
    }
}

impl Opts {
    /// Perform late initialization of the command-line arguments. If `init` isn't called before
    /// the arguments are used, the behavior is undefined.
    pub fn init(&mut self) -> miette::Result<()> {
        if let Some(file) = &self.file {
            self.watch.paths.push(file.clone());
        } else if self.watch.paths.is_empty() {
            self.watch.paths.push(NormalPath::from_cwd("src")?);
        }

        // These help our libraries (particularly `color-eyre`) see these options.
        // The options are provided mostly for documentation.
        std::env::set_var("RUST_BACKTRACE", self.logging.backtrace.to_string());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_action_from_str_valid() {
        let action: TuiAction = "Reload All:git diff --name-only | xargs touch"
            .parse()
            .unwrap();
        assert_eq!(action.label, "Reload All");
        assert!(matches!(
            action.command,
            TuiActionCommand::Shell(ref cmd) if cmd == "git diff --name-only | xargs touch"
        ));
    }

    #[test]
    fn test_tui_action_from_str_with_spaces() {
        let action: TuiAction = "My Action  :  echo hello  ".parse().unwrap();
        assert_eq!(action.label, "My Action");
        assert!(matches!(
            action.command,
            TuiActionCommand::Shell(ref cmd) if cmd == "echo hello"
        ));
    }

    #[test]
    fn test_tui_action_from_str_with_colon_in_command() {
        let action: TuiAction = "Run Test:cabal test --test-option=--match=/Foo/Bar:"
            .parse()
            .unwrap();
        assert_eq!(action.label, "Run Test");
        assert!(matches!(
            action.command,
            TuiActionCommand::Shell(ref cmd) if cmd == "cabal test --test-option=--match=/Foo/Bar:"
        ));
    }

    #[test]
    fn test_tui_action_from_str_invalid() {
        let result: Result<TuiAction, _> = "InvalidAction".parse();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Expected 'LABEL:COMMAND'"));
    }

    #[test]
    fn test_tui_action_from_str_internal() {
        let action: TuiAction = "Toggle Warnings:@toggle-track-warnings"
            .parse()
            .unwrap();
        assert_eq!(action.label, "Toggle Warnings");
        assert!(matches!(
            action.command,
            TuiActionCommand::Internal(ref cmd) if cmd == "toggle-track-warnings"
        ));
    }

    #[test]
    fn test_tui_action_default_reload_all() {
        let action = TuiAction::default_reload_all();
        assert_eq!(action.label, "Reload All");
        assert!(matches!(
            action.command,
            TuiActionCommand::Shell(ref cmd) if cmd == "cd \"$(git rev-parse --show-toplevel)\" && git diff --name-only | xargs -r touch"
        ));
    }

    #[test]
    fn test_tui_action_default_toggle_warnings() {
        let action = TuiAction::default_toggle_warnings();
        assert_eq!(action.label, "Toggle Warnings");
        assert!(matches!(
            action.command,
            TuiActionCommand::Internal(ref cmd) if cmd == "toggle-track-warnings"
        ));
    }

    #[test]
    fn test_tui_action_default_toggle_no_load() {
        let action = TuiAction::default_toggle_no_load();
        assert_eq!(action.label, "Toggle No-Load");
        assert!(matches!(
            action.command,
            TuiActionCommand::Internal(ref cmd) if cmd == "toggle-no-load"
        ));
    }

    #[test]
    fn test_tui_opts_get_actions_default_only() {
        let opts = TuiOpts::default();
        let actions = opts.get_actions();
        assert_eq!(actions.len(), 3);
        assert_eq!(actions[0].label, "Reload All");
        assert_eq!(actions[1].label, "Toggle Warnings");
        assert_eq!(actions[2].label, "Toggle No-Load");
    }

    #[test]
    fn test_tui_opts_get_actions_with_custom() {
        let opts = TuiOpts {
            actions: vec![
                "Custom 1:echo one".parse().unwrap(),
                "Custom 2:echo two".parse().unwrap(),
            ],
        };
        let actions = opts.get_actions();
        assert_eq!(actions.len(), 5);
        assert_eq!(actions[0].label, "Reload All");
        assert_eq!(actions[1].label, "Toggle Warnings");
        assert_eq!(actions[2].label, "Toggle No-Load");
        assert_eq!(actions[3].label, "Custom 1");
        assert_eq!(actions[4].label, "Custom 2");
    }

    #[test]
    fn test_tui_opts_get_actions_truncates_at_nine() {
        let opts = TuiOpts {
            actions: (1..=10)
                .map(|i| format!("Action {}:echo {}", i, i).parse().unwrap())
                .collect(),
        };
        let actions = opts.get_actions();
        assert_eq!(actions.len(), 9); // 3 default + 6 custom (truncated from 10)
    }
}
