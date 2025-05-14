#[allow(clippy::module_inception)]
/// Module for interface components.
pub mod interface;
pub use interface::*;
/// Module for mobile interface and navigation components.
pub mod mobile;
pub use mobile::*;
/// Module for desktop interface and navigation components.
pub mod desktop;
pub use desktop::*;
/// Module for the mobile keyboard component.
pub mod mobile_keyboard;
pub use mobile_keyboard::*;