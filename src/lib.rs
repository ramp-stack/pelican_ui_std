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
/// Modular for UI theme (fonts, icons, colors, etc.)
pub mod theme;
/// Modular for UI layouts (rows, columns, stacks, etc.)
pub mod layout;
/// Modular for UI primitive elements (text, images, shapes, etc.)
pub mod elements;
/// Modular for UI components
pub mod components;
/// Modular for interface
pub mod interface;
/// Modular for helper functions and objects
pub mod utils;

use rust_on_rails::prelude::*;
use crate::theme::Theme;

/// A boxed, mutable callback function that takes a mutable reference to a [`Context`].
///
/// This type is typically used to store closures that modify or respond to the state
/// of the `Context` during event handling, rendering, or other application logic.
///
/// # Example
/// ```rust
/// fn make_callback() -> Callback {
///     Box::new(|ctx: &mut Context| {
///         ctx.do_something();
///     })
/// }
/// ```
pub type Callback = Box<dyn FnMut(&mut Context)>;

#[cfg(target_os = "ios")]
extern "C" {
    fn trigger_haptic();
}

#[cfg(target_os = "ios")]
fn vibrate()  {
    unsafe {
        trigger_haptic();
    }
}

/// Plugin structure holding the theme.
pub struct PelicanUI {
    /// The theme used to style the UI.
    pub theme: Theme,
}

impl PelicanUI {
    /// Updates `PelicanUI` with the provided theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - A `Theme` structure that defines the appearance of the app's UI. (Theme::default())
    pub fn update_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}

impl Plugin for PelicanUI {
    async fn background_tasks(_ctx: &mut HeadlessContext) -> Tasks {
        vec![]
    }

    async fn new(ctx: &mut Context, _h_ctx: &mut HeadlessContext) -> (Self, Tasks) {
        ctx.include_assets(include_assets!("./resources"));
        (PelicanUI { theme: Theme::default(ctx) }, vec![])
    }
}


/// Represents a unique identifier for an element in the user interface.
///
/// This struct wraps a `Uuid` to ensure each UI element has a unique identifier, which is useful
/// for tracking and referencing elements throughout the UI system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementID(uuid::Uuid);

impl ElementID {
    /// Creates a new `ElementID` with a randomly generated UUID.
    ///
    /// # Returns
    ///
    /// A new `ElementID` with a random UUID.
    pub fn new() -> Self {
        ElementID(uuid::Uuid::new_v4())
    }

    /// Returns the underlying UUID of the `ElementID`.
    ///
    /// # Returns
    ///
    /// A `uuid::Uuid` representing the unique identifier of the element.
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for ElementID {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AppPage: Drawable + std::fmt::Debug + 'static {
    fn into_boxed(self) -> Box<dyn AppPage> where Self: Sized {
        Box::new(self) as Box<dyn AppPage>
    }
}

// dyn_clone::clone_trait_object!(AppPage);

pub use pelican_macro::AppPage as derive_AppPage;

pub mod macros {
    pub use pelican_macro::AppPage;
}

/// A prelude module for easier access to the key components of the PelicanUI system.
pub mod prelude {
    pub use crate::AppPage; 
    pub use crate::macros::AppPage;

    pub use crate::ElementID;
    pub use crate::events::*;
    pub use crate::interface::*;
    pub use crate::layout::*;
    pub use crate::components::*;
    pub use crate::elements::*;
    pub use crate::theme::*;
    pub use crate::utils::*;
    pub use crate::config::*;
    pub use crate::PelicanUI;
}

