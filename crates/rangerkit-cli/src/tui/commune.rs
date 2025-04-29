use std::io;
use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use rangerkit_core::SpiritManifest;

/// Runs the spirit commune TUI.
pub fn run_commune_tui() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let manifest = SpiritManifest::default();


    terminal.draw(|f| {
        let size = f.area();
        let items: Vec<ListItem> = manifest.spirits.iter().map(|spirit| {
            let content = format!("{}  {}", spirit.glyph, spirit.name);
            ListItem::new(content)
        }).collect();

        let list = List::new(items)
            .block(Block::default()
                .title("🌿 The Spirits Gather")
                .borders(Borders::ALL)
            );

        f.render_widget(list, size);
    }).context("failed to draw spirits")?;

    // Wait for a keypress to exit
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Enter {
                break;
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}
