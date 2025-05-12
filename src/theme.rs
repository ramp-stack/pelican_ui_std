use rust_on_rails::prelude::Context;

use colors::ColorResources;
use fonts::FontResources;
use icons::IconResources;
use brand::BrandResources;

pub mod colors;
pub mod fonts;
pub mod icons;
pub mod brand;

/// Represents the visual theme of the application, encapsulating color, font, icon, and brand resources.
///
/// # Modules:
/// - `colors`: Defines and manages color resources used in the theme.
/// - `fonts`: Defines and manages font resources used in the theme.
/// - `icons`: Defines and manages icon resources used in the theme.
/// - `brand`: Defines and manages branding resources such as logos or trademarks.
pub struct Theme {
    pub colors: ColorResources
    pub fonts: FontResources,
    pub icons: IconResources,
    pub brand: BrandResources,
}

impl Theme {
    /// Returns the default theme for the application, using default resources for colors, fonts, icons, and brand.
    ///
    /// This function initializes a `Theme` instance with default values for all resources, making it suitable for applications that don't require custom styling or resources.
    ///
    /// # Parameters
    /// - `ctx`: A mutable reference to the `Context` used to initialize `FontResources` and `IconResources`. These resources may require context-specific data, such as loaded font files or icon images.
    ///
    /// # Returns
    /// A `Theme` instance initialized with default color, font, icon, and brand resources.
    ///
    /// # Example
    /// ```
    /// let mut ctx = Context::new();
    /// let default_theme = Theme::default(&mut ctx);
    /// ```
    pub fn default(ctx: &mut Context) -> Self {
        Theme {
            colors: ColorResources::default(),
            fonts: FontResources::default(ctx),
            icons: IconResources::default(ctx),
            brand: BrandResources::default(ctx),
        }
    }

    /// Creates a new theme with custom resources for colors, fonts, icons, and branding.
    ///
    /// This function allows users to specify their own theme resources, which can be useful for applications with a unique or custom design.
    ///
    /// # Parameters
    /// - `colors`: Custom `ColorResources` to use in the theme.
    /// - `fonts`: Custom `FontResources` to use in the theme.
    /// - `icons`: Custom `IconResources` to use in the theme.
    /// - `brand`: Custom `BrandResources` to use in the theme.
    ///
    /// # Returns
    /// A `Theme` instance initialized with the provided custom resources.
    ///
    pub fn new(
        colors: ColorResources, 
        fonts: FontResources, 
        icons: IconResources,
        brand: BrandResources,
    ) -> Self { 
        Theme { colors, fonts, icons, brand } 
    }
}
