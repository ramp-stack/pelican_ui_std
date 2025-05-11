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
/// The `Theme` struct provides a unified way to manage the core resources of an application's user interface. This includes resources for colors, fonts, icons, and branding. It can be initialized either with default values or with custom resources. The `Theme` is used to apply a consistent visual style across the application.
///
/// # Modules:
/// - `colors`: Defines and manages color resources used in the theme.
/// - `fonts`: Defines and manages font resources used in the theme.
/// - `icons`: Defines and manages icon resources used in the theme.
/// - `brand`: Defines and manages branding resources such as logos or trademarks.
pub struct Theme {
    pub colors: ColorResources,   // Color resources for the theme.
    pub fonts: FontResources,     // Font resources for the theme.
    pub icons: IconResources,     // Icon resources for the theme.
    pub brand: BrandResources,    // Brand resources for the theme.
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
    /// # Example
    /// ```
    /// let custom_colors = ColorResources::new(/* ... */);
    /// let custom_fonts = FontResources::new(/* ... */);
    /// let custom_icons = IconResources::new(/* ... */);
    /// let custom_brand = BrandResources::new(/* ... */);
    /// 
    /// let custom_theme = Theme::new(custom_colors, custom_fonts, custom_icons, custom_brand);
    /// ```
    pub fn new(
        colors: ColorResources, 
        fonts: FontResources, 
        icons: IconResources,
        brand: BrandResources,
    ) -> Self { 
        Theme { colors, fonts, icons, brand } 
    }
}
