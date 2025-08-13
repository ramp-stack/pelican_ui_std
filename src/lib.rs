#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui/main/logo.png")]

pub mod events;
pub mod config;
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