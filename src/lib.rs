#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui/main/logo.png")]

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

use rust_on_rails::prelude::*;
use crate::theme::Theme;

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
    /// Initializes the `PelicanUI` with the provided theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - A `Theme` structure that defines the appearance of the app's UI. (Theme::default())
    pub fn init(&mut self, theme: Theme) {
        println!("Updating theme:");
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

/// A trait representing a page in an application.
///
/// This trait extends `Drawable` and `Debug`, which means any type that implements `AppPage` must
/// be drawable and must support debugging output.
pub trait AppPage: Drawable + std::fmt::Debug + 'static {}

/// A trait representing a flow in an application.
///
/// This trait is used to navigate between pages, with the ability to get the current page and trigger page navigation.
pub trait AppFlow: std::fmt::Debug + Send + Sync + dyn_clone::DynClone + 'static {
    /// Returns the current page of the application flow.
    ///
    /// # Arguments
    ///
    /// * `ctx` -  The current context, used for accessing themes and UI elements.
    ///
    /// # Returns
    ///
    /// A boxed trait object implementing `AppPage`, which represents the current page.
    fn get_page(&self, ctx: &mut Context) -> Box<dyn crate::AppPage>;

    /// Navigates to a new page in the application flow.
    ///
    /// # Arguments
    ///
    /// * `self` - The flow object that defines the navigation action.
    ///
    /// This function triggers a `NavigateEvent` that updates the current flow of the app.
    fn navigate(self, ctx: &mut Context) where Self: Sized {
        ctx.trigger_event(crate::events::NavigateEvent(Box::new(self) as Box<dyn AppFlow>));
    }
}

dyn_clone::clone_trait_object!(AppFlow);

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

/// A prelude module for easier access to the key components of the PelicanUI system.
pub mod prelude {
    pub use crate::ElementID;
    pub use crate::AppFlow;
    pub use crate::AppPage;
    pub use crate::events::*;
    pub use crate::interface::*;
    pub use crate::layout::*;
    pub use crate::components::*;
    pub use crate::elements::*;
    pub use crate::theme::*;
    pub use crate::PelicanUI;
}
