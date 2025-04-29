use crate::ability::Ability;

/// Defines the Spirit struct, representing a companion entity with abilities.

/// Represents a Spirit companion.
///
/// Each Spirit has a name, a glyph (symbol), and a set of abilities.
#[derive(Debug, Clone)]
pub struct Spirit {
    /// The name of the spirit
    pub name: String,
    /// The visual glyph symbolizing the spirit.
    pub glyph: String,
    /// A collection of abilities this Spirit can perform.
    pub abilities: Vec<Ability>,
}

impl Spirit {
    /// Creates a new Spirit witha given name and glyph.
    ///
    /// The new Spirit will initially have no abilities.
    ///
    /// # Arguments
    ///
    /// * `name` - The Spirit's name.
    /// * `glyph` - The Spirit's symbolic representation. 
    pub fn new(name: impl Into<String>, glyph: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            glyph: glyph.into(),
            abilities: Vec::new(),
        }
    }

    /// Adds an ability to the Spirit and returns the modified Spirit.
    ///
    /// This method supports chaining to easily compose spirits with multiple abilities.
    ///
    /// # Arguments
    ///
    /// * `ability` - The ability to add to the Spirit.
    pub fn with_ability(mut self, ability: Ability) -> Self {
        self.abilities.push(ability);
        self
    }
}

