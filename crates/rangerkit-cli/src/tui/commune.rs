use anyhow::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    prelude::*,
};
use crate::tui::widgets::build_spirit_list;
use rangerkit_core::SpiritManifest;
use crate::tui::layout::{launch_tui, wait_for_exit};


/// Entrypoint for the commune TUI experience
pub fn run_commune_tui() -> Result<()> {
    launch_tui(|terminal| {
        let manifest = SpiritManifest::default();
        terminal.draw(|f| draw_commune_frame(f, &manifest))?;
        wait_for_exit()
    })
}

pub fn draw_commune_frame(f: &mut Frame, manifest: &SpiritManifest) {
    let full_area = f.area();

    // Draw outer border first
    let outer = Block::default()
        .title("🌿 The Spirits Gather")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left);

    f.render_widget(outer, full_area);

    // Centered box for the spirits
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

    // Create inner frame for spirits with a border
    let inner = Block::default()
        .title("🕯️ Spirits")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);

    let spirit_list = build_spirit_list(manifest).block(inner);
    f.render_widget(spirit_list, spirit_area);

    // Footer hint
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
