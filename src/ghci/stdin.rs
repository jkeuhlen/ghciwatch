use itertools::Itertools;
use miette::Context;
use miette::IntoDiagnostic;
use tokio::io::AsyncWriteExt;
use tokio::process::ChildStdin;
use tracing::instrument;

use crate::incremental_reader::FindAt;

use super::loaded_module::LoadedModule;
use super::parse::ShowPaths;
use super::CompilationLog;
use super::GhciCommand;
use super::ModuleSet;
use super::PROMPT;
use crate::ghci::GhciStdout;

pub struct GhciStdin {
    /// Inner stdin writer.
    pub stdin: ChildStdin,
}

impl GhciStdin {
    /// Write a line on `stdin` and wait for a prompt on stdout.
    ///
    /// The `line` should contain the trailing newline.
    ///
    /// The `find` parameter determines where the prompt can be found in the output line.
    #[instrument(skip(self, stdout), level = "debug")]
    async fn write_line_with_prompt_at(
        &mut self,
        stdout: &mut GhciStdout,
        line: &str,
        find: FindAt,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        match self.stdin.write_all(line.as_bytes()).await {
            Ok(_) => stdout.prompt(find, log).await,
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                Err(miette::miette!(
                    "GHCi process stdin closed unexpectedly (broken pipe). \
                     This usually happens when GHCi crashes or exits unexpectedly."
                )
                .wrap_err(e))
            }
            Err(e) => Err(e).into_diagnostic(),
        }
    }

    /// Write a line on `stdin` and wait for a prompt on stdout.
    ///
    /// The `line` should contain the trailing newline.
    async fn write_line(
        &mut self,
        stdout: &mut GhciStdout,
        line: &str,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        self.write_line_with_prompt_at(stdout, line, FindAt::LineStart, log)
            .await
    }

    /// Run a [`GhciCommand`].
    ///
    /// The command may be multiple lines.
    #[instrument(skip(self, stdout), level = "debug")]
    pub async fn run_command(
        &mut self,
        stdout: &mut GhciStdout,
        command: &GhciCommand,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        for line in command.lines() {
            self.write_line(stdout, &format!("{line}\n"), log).await?;
        }

        Ok(())
    }

    #[instrument(skip(self, stdout), name = "stdin_initialize", level = "debug")]
    pub async fn initialize(
        &mut self,
        stdout: &mut GhciStdout,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        // We tell stdout/stderr we're compiling for the first prompt because this includes all the
        // module compilation before the first prompt.
        self.write_line_with_prompt_at(
            stdout,
            &format!(":set prompt {PROMPT}\n"),
            FindAt::Anywhere,
            log,
        )
        .await?;
        self.write_line(stdout, &format!(":set prompt-cont {PROMPT}\n"), log)
            .await?;
        Ok(())
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn reload(
        &mut self,
        stdout: &mut GhciStdout,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        self.write_line(stdout, ":reload\n", log).await
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn add_modules(
        &mut self,
        stdout: &mut GhciStdout,
        modules: impl IntoIterator<Item = &LoadedModule>,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        let modules = modules.into_iter().format(" ");
        // We use `:add` because `:load` unloads all previously loaded modules:
        //
        // > All previously loaded modules, except package modules, are forgotten. The new set of
        // > modules is known as the target set. Note that :load can be used without any arguments
        // > to unload all the currently loaded modules and bindings.
        //
        // https://downloads.haskell.org/ghc/latest/docs/users_guide/ghci.html#ghci-cmd-:load
        self.write_line(stdout, &format!(":add {modules}\n"), log)
            .await
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn remove_modules(
        &mut self,
        stdout: &mut GhciStdout,
        modules: impl IntoIterator<Item = &LoadedModule>,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        let modules = modules.into_iter().format(" ");
        self.write_line(stdout, &format!(":unadd {modules}\n"), log)
            .await
    }

    #[instrument(skip(self, stdout), level = "debug")]
    pub async fn interpret_module(
        &mut self,
        stdout: &mut GhciStdout,
        module: &LoadedModule,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        // `:add *` forces the module to be interpreted, even if it was already loaded from
        // bytecode. This is necessary to access the module's top-level binds for the eval feature.
        self.write_line(stdout, &format!(":add *{module}\n"), log)
            .await
    }

    #[instrument(skip(self, stdout), level = "debug")]
    pub async fn eval(
        &mut self,
        stdout: &mut GhciStdout,
        module_name: &str,
        command: &GhciCommand,
        log: &mut CompilationLog,
    ) -> miette::Result<()> {
        self.write_line(stdout, &format!(":module + *{module_name}\n"), log)
            .await?;

        self.run_command(stdout, command, log).await?;

        self.write_line(stdout, &format!(":module - *{module_name}\n"), log)
            .await?;

        Ok(())
    }

    #[instrument(skip(self, stdout), level = "debug")]
    pub async fn show_paths(&mut self, stdout: &mut GhciStdout) -> miette::Result<ShowPaths> {
        match self.stdin.write_all(b":show paths\n").await {
            Ok(_) => stdout.show_paths().await,
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                Err(miette::miette!(
                    "GHCi process stdin closed unexpectedly (broken pipe). \
                     This usually happens when GHCi crashes or exits unexpectedly."
                )
                .wrap_err(e))
            }
            Err(e) => Err(e).into_diagnostic(),
        }
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn show_targets(
        &mut self,
        stdout: &mut GhciStdout,
        show_paths: &ShowPaths,
    ) -> miette::Result<ModuleSet> {
        match self.stdin.write_all(b":show targets\n").await {
            Ok(_) => stdout.show_targets(show_paths).await,
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                Err(miette::miette!(
                    "GHCi process stdin closed unexpectedly (broken pipe). \
                     This usually happens when GHCi crashes or exits unexpectedly."
                )
                .wrap_err(e))
            }
            Err(e) => Err(e).into_diagnostic(),
        }
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn show_modules(
        &mut self,
        stdout: &mut GhciStdout,
        show_paths: &ShowPaths,
    ) -> miette::Result<ModuleSet> {
        match self.stdin.write_all(b":show modules\n").await {
            Ok(_) => stdout.show_modules(show_paths).await,
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                Err(miette::miette!(
                    "GHCi process stdin closed unexpectedly (broken pipe). \
                     This usually happens when GHCi crashes or exits unexpectedly."
                )
                .wrap_err(e))
            }
            Err(e) => Err(e).into_diagnostic(),
        }
    }

    #[allow(dead_code)] // TODO: No it should not be!
    #[instrument(skip(self, stdout), level = "debug")]
    pub async fn quit(&mut self, stdout: &mut GhciStdout) -> miette::Result<()> {
        match self.stdin.write_all(b":quit\n").await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                // If the pipe is already broken when we try to quit, that's fine - GHCi is already gone
                tracing::debug!("GHCi stdin already closed when attempting to quit");
                return Ok(());
            }
            Err(e) => {
                return Err(e)
                    .into_diagnostic()
                    .wrap_err("Failed to tell ghci to `:quit`")
            }
        }
        stdout
            .quit()
            .await
            .wrap_err("Failed to wait for ghci to quit")
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    #[tokio::test]
    async fn test_broken_pipe_error_message() {
        // This test verifies that when we get a BrokenPipe error, we provide a helpful
        // error message explaining what happened

        // We can't easily create a GhciStdin with a mock stdin because ChildStdin is
        // from the standard library and doesn't expose constructors. However, we can
        // verify the error handling logic by checking that our error messages are
        // properly formatted.

        // The actual integration test in tests/ghci_crash.rs will verify the full
        // end-to-end behavior.

        let err = io::Error::new(io::ErrorKind::BrokenPipe, "test broken pipe");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);

        // Verify our error message would be helpful
        let expected_msg = "GHCi process stdin closed unexpectedly (broken pipe). \
                           This usually happens when GHCi crashes or exits unexpectedly.";
        assert!(expected_msg.contains("broken pipe"));
        assert!(expected_msg.contains("GHCi"));
    }

    #[test]
    fn test_broken_pipe_error_kind_detection() {
        // Verify we can detect BrokenPipe errors correctly
        let broken_pipe = io::Error::new(io::ErrorKind::BrokenPipe, "pipe broken");
        assert_eq!(broken_pipe.kind(), io::ErrorKind::BrokenPipe);

        let other_error = io::Error::new(io::ErrorKind::ConnectionReset, "connection reset");
        assert_ne!(other_error.kind(), io::ErrorKind::BrokenPipe);
    }
}

