#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/logo.png")]

//! Pelican UI Standard provides a wide range of components and interface systems for building beautiful, consistently designed applications.
//!
//! Download the starter template here: <https://github.com/EllaCouch20/ramp_template>

/// ## Events
///
/// Events used by Pelican UI Standard components.
pub mod events;
/// ## Config
///
/// Variables for detecting which 
pub mod config;
/// ## Layout
///
/// Every structure implementing the [`Component`](pelican_ui::Component) trait must have its **first element implement [`Layout`](pelican_ui::layout::Layout)**.  
/// This ensures that the component can correctly manage positioning, sizing, and nested layouts within the UI hierarchy.
///
pub mod layout;
/// ## Elements
///
/// Contains the lowest-level UI primitives, such as text, images, and other  
/// fundamental building blocks that can be composed into larger components.
///
pub mod elements;
/// ## Components
///
/// Commonly used UI and interface components composed from [`elements`] and  
/// [`layouts`](crate::layout). 
///
/// Every structure implementing the [`Component`](pelican_ui::Component) trait must have its **first element implement [`Layout`](pelican_ui::layout::Layout)**.  
/// This ensures that the component can correctly manage positioning, sizing, and nested layouts within the UI hierarchy.
///
/// Every structure implementing the [`Component`](pelican_ui::Component) trait must also implement [`OnEvent`](pelican_ui::events::OnEvent).
///
/// Every structure implementing the [`Component`](pelican_ui::Component) trait must also implement [`Debug`].
///
/// ## Example
///
/// ```rust
#[doc = include_str!("examples/component.rs")]
/// ```

pub mod components;
/// ## Utilities
///
/// Helper structures like Timestamp, ElementID, etc.
pub mod utils;
/// ## Pages
///
/// A couple pages built using Pelican UI Standard components. These can be used as is or as examples when building new pages.
pub mod pages;

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