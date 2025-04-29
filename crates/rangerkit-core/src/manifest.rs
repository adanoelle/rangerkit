use crate::spirit::Spirit;
use crate::ability::Ability;
use serde::Serialize;

/// Defines the SpiritManifest struct, a living registry of all known Spirits.

/// Manifest containing all known spirits.
///
/// The SpiritManifest allows RangerKit to keep track of active companions
/// and their abilities for CLI invocation and TUI mapping.
#[derive(Debug, Serialize)]
pub struct SpiritManifest {
    /// The collection of all registered spirits.
    pub spirits: Vec<Spirit>,
}

impl SpiritManifest {
    /// Creates a new, empty SpiritManifest.
    pub fn new() -> Self {
        Self {
            spirits: Vec::new(),
        }
    }

    /// Adds a Spirit to the manifest and returns the modified manifest.
    ///
    /// This method supports chaining for building manifest trees dynamically.
    ///
    /// # Arguments
    ///
    /// * `spirit` - The Spirit to add.
    pub fn with_spirit(mut self, spirit: Spirit) -> Self {
        self.spirits.push(spirit);
        self
    }
}

impl Default for SpiritManifest {
    /// Provides a default SpiritManifest containing the core starting Spirits.
    /// - Lantern Spirit
    /// - Waystone Spirit
    fn default() -> Self {
        SpiritManifest::new()
          .with_spirit(
              Spirit::new("Lantern Spirit", "(*)")
                .with_ability(Ability::new(
                    "Light Dark Paths",
                    "Illuminate unseen trails and infrastructure",
                )),
          )
        .with_spirit(
            Spirit::new("Waystone Spirit", "•")
              .with_ability(Ability::new(
                  "Map the Stars",
                  "Draw a living starmap of your resources",
              )),
        )
    }
}
