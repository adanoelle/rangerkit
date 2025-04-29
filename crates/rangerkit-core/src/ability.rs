use serde::Serialize;

/// Defines the Ability struct, representing powers a Spirit can perform.

/// Represents a single ability a spirit can perform
#[derive(Debug, Clone, Serialize)]
pub struct Ability {
    pub name: String,
    pub description: String,
}

impl Ability {
    /// Creates a new Ability.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the ability.
    /// * `description` - A short description of the ability's effect or purpose.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }
}
