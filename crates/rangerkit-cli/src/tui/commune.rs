// crates/rangerkit-cli/src/tui/commune.rs
use crate::tui::pulse::PulseCycle;
use crate::tui::widgets::build_spirit_list;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    prelude::*,
};
use rangerkit_core::SpiritManifest;

pub fn run_commune_tui() -> anyhow::Result<()> {
    use std::{time::{Duration, Instant}, io};
    use crossterm::event::{self, Event, KeyCode};
    use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute};
    use ratatui::backend::CrosstermBackend;
    use ratatui::Terminal;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let manifest = SpiritManifest::default();
    let pulse = PulseCycle::new(20);
    let tick_rate = Duration::from_millis(200);
    let mut tick = 0;

    loop {
        let now = Instant::now();

        terminal.draw(|f| {
            draw_commune_frame(f, &manifest, &pulse, tick);
        })?;

        let timeout = tick_rate.saturating_sub(now.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Enter) {
                    break;
                }
            }
        }

        tick = tick.wrapping_add(1);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn draw_commune_frame(
    f: &mut Frame,
    manifest: &SpiritManifest,
    pulse: &PulseCycle,
    tick: usize,
) {
    let full_area = f.area();

    let outer = Block::default()
        .title("🌿 The Spirits Gather")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left);
    f.render_widget(outer, full_area);

    let box_width = 40;
    let box_height = 10;

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((full_area.height.saturating_sub(box_height)) / 2),
            Constraint::Length(box_height),
            Constraint::Min(0),
        ])
        .split(full_area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((full_area.width.saturating_sub(box_width)) / 2),
            Constraint::Length(box_width),
            Constraint::Min(0),
        ])
        .split(vertical_chunks[1]);

    let spirit_area = horizontal_chunks[1];

    let inner = Block::default()
        .title("🕯️ Spirits ")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);

    let spirit_list = build_spirit_list(manifest, pulse, tick).block(inner);
    f.render_widget(spirit_list, spirit_area);

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(full_area);

    let footer = Paragraph::new("🌿 Press 'q' or 'Enter' to leave the circle")
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center);

    f.render_widget(footer, footer_chunks[1]);
}

