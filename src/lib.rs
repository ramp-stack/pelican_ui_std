#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/logo.png")]

pub mod events;
pub mod config;
/// ## Layout
///
/// Every structure implementing the [`Component`](pelican_ui::Component) trait must have its **first element implement [`Layout`](pelican_ui::layout::Layout)**.  
/// This ensures that the component can correctly manage positioning, sizing, and nested layouts within the UI hierarchy.
///
pub mod layout;
pub mod elements;
pub mod components;
pub mod utils;
pub mod pages;

pub use crate::pages::*;
pub use crate::events::*;
pub use crate::layout::*;
pub use crate::components::*;
pub use crate::elements::*;
pub use crate::utils::*;
pub use crate::config::*;

/*
How to create an app with Ramp
How to create a page with Ramp
How to create a component with Ramp
How to create a layout with Ramp
How to create a plugin with Ramp
How to create a messaging app with Ramp
How to create a bitcoin wallet with Ramp
How to create a AIR profile with Ramp
*/