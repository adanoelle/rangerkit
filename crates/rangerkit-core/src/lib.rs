//! # RangerKit Core
//!
//! This module defines the core types for RangerKit: Spirits, Abilities, and the Spirit Manifest.
//!
//! Spirits are companions that embody different tools and helpers.
//! Abilities are the powers each spirit can perform.
//! The Spirit Manifest is the living registry of known spirits.

pub mod spirit;
pub mod ability;
pub mod manifest;

// Re-export core types for readiness
pub use spirit::*;
pub use ability::*;
pub use manifest::*;
