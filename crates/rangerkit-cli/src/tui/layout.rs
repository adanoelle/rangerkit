// crates/rangerkit-cli/src/tui/layout.rs
use anyhow::{Context, Result};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture}, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};

/// Launch a TUI lifecycle with a passed-in render function
pub fn launch_tui<F: FnOnce(&mut Terminal<CrosstermBackend<Stdout>>) -> Result<()>>(f: F) -> Result<()> {
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    f(&mut terminal)?;
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor().context("failed to show cursor")?;
    Ok(())
}

/// Wait for a keypress to exit the TUI
pub fn wait_for_exit() -> Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            if matches!(key.code, KeyCode::Char('q') | KeyCode::Enter) {
                return Ok(());
            }
        }
    }
}
