#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui/main/logo.png")]
// #![deny(missing_docs)]

//! A UI system for managing minimalistic components, themes, pages, and navigation in a Rust-based application.
//!
//! This system supports preset UI components, theme management, page navigation, and interaction with UI components,
//! and it includes platform-specific functionality for iOS haptic feedback.

/// Modular for Pelican UI specific events
pub mod events;
/// Modular for config
pub mod config;
/// Modular for UI layouts (rows, columns, stacks, etc.)
pub mod layout;
/// Modular for UI primitive elements (text, images, shapes, etc.)
pub mod elements;
/// Modular for UI components
pub mod components;
/// Modular for helper functions and objects
pub mod utils;

pub mod pages;

pub use crate::utils::AppPage; 
pub use crate::utils::macros::AppPage;
pub use crate::pages::*;
pub use crate::events::*;
pub use crate::layout::*;
pub use crate::components::*;
pub use crate::elements::*;
pub use crate::utils::*;
pub use crate::config::*;