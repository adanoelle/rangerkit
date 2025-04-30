// crates/rangerkit-cli/src/tui/pulse.rs
use ratatui::style::{Style, Color, Modifier};

/// A simple pulse cycle for animated breathing effects in the TUI.
/// Each tick adjusts intensity in a soft sinusoidal wave.
#[derive(Debug)]
pub struct PulseCycle {
    pub period_ticks: usize,
}

impl PulseCycle {
    /// Create a new PulseCycle that completes a breathing cycle in `period_ticks` frames.
    pub fn new(period_ticks: usize) -> Self {
        Self { period_ticks }
    }

    /// Given the current tick and a base color, returns a softly pulsing style using a gradient.
    pub fn style_for_tick(&self, tick: usize, base_color: Color) -> Style {
        let phase = (tick % self.period_ticks) as f64 / self.period_ticks as f64;
        let intensity = (phase * std::f64::consts::PI).sin().abs();

        // Map the intensity to gradient steps
        let color = match intensity {
            x if x < 0.2 => Color::Red,
            x if x < 0.4 => Color::LightRed,
            x if x < 0.6 => base_color,
            x if x < 0.8 => Color::LightRed,
            _ => Color::LightRed,
        };

        Style::default()
            .fg(color)
            .add_modifier(Modifier::BOLD)
    }
}
