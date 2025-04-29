// crates/rangerkit-cli/src/tui/widgets.rs
use ratatui::{
    widgets::{List, ListItem, Block, Borders},
    text::{Line, Span},
    style::{Color, Modifier, Style}, // ← move Style here
};
use rangerkit_core::SpiritManifest;

/// Build the spirit list widget from a manifest.
pub fn build_spirit_list(manifest: &SpiritManifest) -> List<'_> {
    let items: Vec<ListItem> = manifest.spirits.iter().map(|spirit| {
        let glyph_color = match spirit.name.as_str() {
            "Lantern Spirit" => Color::LightYellow,
            "Waystone Spirit" => Color::Cyan,
            _ => Color::Gray,
        };

        let glyph = Span::styled(
            spirit.glyph.clone(),
            Style::default()
                .fg(glyph_color)
                .add_modifier(Modifier::SLOW_BLINK | Modifier::BOLD),
        );

        let name = Span::styled(
            format!(" {}",
            spirit.name),
            Style::default().fg(Color::White),
        );

        ListItem::new(Line::from(vec![glyph, name]))
    }).collect();

    List::new(items)
        .block(Block::default().title("🌿 The Spirits Gather").borders(Borders::ALL))
}

