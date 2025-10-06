use std::ops::Deref;
use std::ops::DerefMut;

use ansi_to_tui::IntoText;
use crossterm::event::Event;
use crossterm::event::EventStream;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseEventKind;
use miette::miette;
use miette::IntoDiagnostic;
use miette::WrapErr;
use ratatui::prelude::Buffer;
use ratatui::prelude::Constraint;
use ratatui::prelude::Layout;
use ratatui::prelude::Rect;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use ratatui::widgets::Wrap;
use saturating::Saturating;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::io::DuplexStream;
use tokio_stream::StreamExt;
use tracing::instrument;

mod terminal;

use crate::buffers::TUI_SCROLLBACK_CAPACITY;
use crate::cli::TuiAction;
use crate::ghci::manager::WatcherEvent;
use crate::ShutdownHandle;
use terminal::TerminalGuard;

/// Default amount to scroll on mouse wheel events.
const SCROLL_AMOUNT: usize = 3;

/// State data for drawing the TUI.
#[derive(Debug)]
struct TuiState {
    debug: bool,
    quit: bool,
    scrollback: Vec<u8>,
    line_count: Saturating<usize>,
    scroll_offset: Saturating<usize>,
    actions: Vec<TuiAction>,
    show_actions: bool,
}

impl TuiState {
    fn new(actions: Vec<TuiAction>) -> Self {
        Self {
            debug: false,
            quit: false,
            scrollback: Vec::with_capacity(TUI_SCROLLBACK_CAPACITY),
            line_count: Saturating(1),
            scroll_offset: Saturating(0),
            actions,
            show_actions: true,
        }
    }

    #[instrument(level = "trace", skip_all)]
    fn render_inner(&self, area: Rect, buffer: &mut Buffer) -> miette::Result<()> {
        if area.width == 0 || area.height == 0 {
            return Ok(());
        }

        let action_height = if self.show_actions && !self.actions.is_empty() {
            (self.actions.len() + 2) as u16 // +2 for title and separator
        } else {
            0
        };

        let areas = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(action_height),
            Constraint::Length(if self.debug { 1 } else { 0 }),
        ])
        .split(area);

        let text = self.scrollback.into_text().into_diagnostic()?;

        let scroll_offset = u16::try_from(self.scroll_offset.0)
            .into_diagnostic()
            .wrap_err("Scroll offset doesn't fit into 16 bits")?;

        Paragraph::new(text)
            .wrap(Wrap::default())
            .scroll((scroll_offset, 0))
            .render(areas[0], buffer);

        // Render actions sidebar
        if self.show_actions && !self.actions.is_empty() {
            let mut action_text = String::from("Actions (press number key):\n");
            for (i, action) in self.actions.iter().enumerate() {
                action_text.push_str(&format!("  {}: {}\n", i + 1, action.label));
            }
            Paragraph::new(action_text).render(areas[1], buffer);
        }

        if self.debug {
            let line_count = self.line_count;
            let scroll_offset = self.scroll_offset;
            Paragraph::new(format!(
                "(☞ ﾟ ヮﾟ )☞  line_count={line_count}, scroll_offset={scroll_offset}"
            ))
            .render(areas[2], buffer);
        }

        Ok(())
    }
}

struct Tui {
    terminal: TerminalGuard,
    /// The last terminal size seen. This is updated on every `render` call.
    size: Rect,
    state: TuiState,
    action_sender: tokio::sync::mpsc::Sender<WatcherEvent>,
}

impl Deref for Tui {
    type Target = TuiState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Tui {
    fn new(
        mut terminal: TerminalGuard,
        actions: Vec<TuiAction>,
        action_sender: tokio::sync::mpsc::Sender<WatcherEvent>,
    ) -> Self {
        let area = terminal.get_frame().size();
        Self {
            terminal,
            size: area,
            state: TuiState::new(actions),
            action_sender,
        }
    }

    fn half_height(&self) -> Saturating<usize> {
        Saturating((self.size.height / 2) as usize)
    }

    fn scroll_up(&mut self, amount: usize) {
        self.scroll_offset -= Saturating(amount);
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll_offset += Saturating(amount);
        self.scroll_offset = self.scroll_offset.min(self.scroll_max());
    }

    fn scroll_max(&self) -> Saturating<usize> {
        self.line_count - self.half_height()
    }

    fn scroll_to(&mut self, scroll_offset: usize) {
        self.scroll_offset = self.scroll_max().min(Saturating(scroll_offset));
    }

    fn maybe_follow(&mut self) {
        let height = self.size.height as usize;

        let scrolled_to_bottom =
            self.scroll_offset >= self.line_count - Saturating(height) - Saturating(1);

        let scrollback_exceeds_height = self.line_count > Saturating(height);

        if scrolled_to_bottom && scrollback_exceeds_height {
            self.scroll_offset += Saturating(1);
        }
    }

    fn push_line(&mut self, line: String) {
        self.scrollback.extend(line.into_bytes());
        self.scrollback.push(b'\n');
        self.line_count += Saturating(1);
        self.maybe_follow();
    }

    #[instrument(level = "trace", skip(self))]
    fn render(&mut self) -> miette::Result<()> {
        let mut render_result = Ok(());
        self.terminal
            .draw(|frame| {
                self.size = frame.size();
                let buffer = frame.buffer_mut();
                render_result = self.state.render_inner(self.size, buffer);
            })
            .into_diagnostic()
            .wrap_err("Failed to draw to terminal")?;

        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    async fn handle_event(&mut self, event: Event) -> miette::Result<()> {
        // TODO: Steal Evan's declarative key matching macros?
        // https://github.com/evanrelf/indigo/blob/7a5e8e47291585cae03cdf5a7c47ad3bcd8db3e6/crates/indigo-tui/src/key/macros.rs
        match event {
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => self.scroll_up(SCROLL_AMOUNT),
                MouseEventKind::ScrollDown => self.scroll_down(SCROLL_AMOUNT),
                _ => {}
            },
            Event::Key(key) => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('j')) => self.scroll_down(1),
                (KeyModifiers::NONE, KeyCode::Char('k')) => self.scroll_up(1),
                (KeyModifiers::NONE, KeyCode::Char('g')) => self.scroll_to(0),
                (KeyModifiers::SHIFT, KeyCode::Char('g' | 'G')) => self.scroll_to(usize::MAX),
                (KeyModifiers::CONTROL, KeyCode::Char('u')) => self.scroll_up(self.half_height().0),
                (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                    self.scroll_down(self.half_height().0)
                }
                (KeyModifiers::CONTROL, KeyCode::Char('e')) => self.scroll_down(1),
                (KeyModifiers::CONTROL, KeyCode::Char('y')) => self.scroll_up(1),
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => self.quit = true,
                (KeyModifiers::NONE, KeyCode::Char('`')) => self.debug = false,
                (KeyModifiers::SHIFT, KeyCode::Char('`' | '~')) => self.debug = true,
                (KeyModifiers::NONE, KeyCode::Char('a')) => {
                    self.show_actions = !self.show_actions;
                }
                (KeyModifiers::NONE, KeyCode::Char(c @ '1'..='9')) => {
                    let index = (c as usize) - ('1' as usize);
                    if let Some(action) = self.state.actions.get(index) {
                        self.trigger_action(action.command.clone()).await?;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    async fn trigger_action(&self, command: String) -> miette::Result<()> {
        use miette::IntoDiagnostic;
        tracing::info!(%command, "Triggering TUI action");
        self.action_sender
            .send(WatcherEvent::Action { command })
            .await
            .into_diagnostic()?;
        Ok(())
    }
}

/// Start the terminal event loop, reading output from the given readers.
#[instrument(level = "debug", skip_all)]
pub async fn run_tui(
    mut shutdown: ShutdownHandle,
    ghci_reader: DuplexStream,
    tracing_reader: DuplexStream,
    actions: Vec<TuiAction>,
    action_sender: tokio::sync::mpsc::Sender<WatcherEvent>,
) -> miette::Result<()> {
    let mut ghci_reader = BufReader::new(ghci_reader).lines();
    let mut tracing_reader = BufReader::new(tracing_reader).lines();

    let terminal = terminal::enter()?;
    let mut tui = Tui::new(terminal, actions, action_sender);

    let mut event_stream = EventStream::new();

    tracing::warn!("`--tui` mode is experimental and may contain bugs or change drastically in future releases.");

    while !tui.quit {
        tui.render()?;

        tokio::select! {
            _ = shutdown.on_shutdown_requested() => {
                tui.quit = true;
            }

            line = ghci_reader.next_line() => {
                let line = line.into_diagnostic().wrap_err("Failed to read line from GHCI")?;
                match line {
                    Some(line) => {
                        tui.push_line(line);
                    },
                    None => {
                        tui.quit = true;
                    },
                }
            }

            line = tracing_reader.next_line() => {
                let line = line.into_diagnostic().wrap_err("Failed to read line from tracing")?;
                if let Some(line) = line {
                    tui.push_line(line);
                }
            }

            output = event_stream.next() => {
                let event = output
                    .ok_or_else(|| miette!("No more crossterm events"))?
                    .into_diagnostic()
                    .wrap_err("Failed to get next crossterm event")?;
                // TODO: `get_frame` is an expensive call, delay if possible.
                // https://github.com/MercuryTechnologies/ghciwatch/pull/206#discussion_r1508364135
                tui.handle_event(event).await?;
            }
        }
    }

    let _ = shutdown.request_shutdown();

    Ok(())
}
