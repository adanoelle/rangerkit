// crates/rangerkit-cli/src/tui/widgets.rs
use crate::tui::pulse::PulseCycle;
use ratatui::{
    widgets::{List, ListItem},
    text::{Line, Span},
    style::{Color, Style},
};
use rangerkit_core::SpiritManifest;

/// Build the spirit list widget from a manifest, with pulsing glyphs.
pub fn build_spirit_list(
    manifest: &SpiritManifest,
    pulse: &PulseCycle,
    tick: usize,
) -> List<'static> {
    let items: Vec<ListItem> = manifest.spirits.iter().map(|spirit| {
        let base_color = match spirit.name.as_str() {
            "Lantern Spirit" => Color::Red,
            "Waystone Spirit" => Color::Red,
            _ => Color::Red,
        };

        let glyph_style = pulse.style_for_tick(tick, base_color);

        let glyph = Span::styled(
            spirit.glyph.to_string(),
            glyph_style,
        );

        let name = Span::styled(
            format!(" {}", spirit.name),
            Style::default().fg(Color::White),
        );

        ListItem::new(Line::from(vec![glyph, name]))
    }).collect();

    List::new(items)
}

