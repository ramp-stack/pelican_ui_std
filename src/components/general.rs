//! # General UI Components
//!
//! This module provides core, reusable UI elements that are platform-agnostic and useful
//! across a wide variety of application contexts, including forms, lists, and user interface structure.

/// A circular avatar component for displaying user or contact images.
pub mod avatar;
pub use avatar::*;

/// A generic button component with support for themes and states.
pub mod button;
pub use button::*;

/// A minimalistic text input field with focus and keyboard integration.
pub mod text_input;
pub use text_input::*;

/// A customizable alert component for displaying warnings or messages.
pub mod alert;
pub use alert::*;

/// A card container for marketing or promoting content.
pub mod card;
pub use card::*;

/// A component for rendering data.
pub mod data_item;
pub use data_item::*;

/// A component for rendering selectable list entries.
pub mod list_item;
pub use list_item::*;
