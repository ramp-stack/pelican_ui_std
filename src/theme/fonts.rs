use rust_on_rails::prelude::*;

/// Represents a collection of font resources, including fonts and font sizes.
#[derive(Clone)]
pub struct FontResources {
    /// The fonts used for various UI elements (e.g., headings, text, labels, etc.).
    pub fonts: Fonts,
    /// The font sizes used throughout the UI.
    pub size: FontSize,
}

impl FontResources {
    /// Creates a new `FontResources` instance with the specified fonts and font sizes.
    ///
    /// # Parameters
    /// - `fonts`: The fonts used in the UI.
    /// - `size`: The font sizes used in the UI.
    ///
    /// # Returns
    /// A `FontResources` struct with the provided fonts and sizes.
    pub fn new(fonts: Fonts, size: FontSize) -> Self {
        Self { fonts, size }
    }

    /// Loads the default font resources using the context to load fonts.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    ///
    /// # Returns
    /// A `FontResources` struct with default fonts and sizes.
    pub fn default(ctx: &mut Context) -> Self {
        FontResources {
            fonts: Fonts::default(ctx),
            size: FontSize::default(),
        }
    }
}

/// Defines a collection of fonts used throughout the application for various elements (headings, text, labels, etc.).
#[derive(Clone)]
pub struct Fonts {
    /// The font used for headings.
    pub heading: resources::Font,
    /// The font used for regular text.
    pub text: resources::Font,
    /// The font used for labels.
    pub label: resources::Font,
    /// The font used for keyboard elements.
    pub keyboard: resources::Font,
    /// The font used for emoji characters.
    pub emoji: resources::Font,
}

impl Fonts {
    /// Creates a new `Fonts` struct with the specified fonts.
    ///
    /// # Parameters
    /// - `heading`: The font used for headings.
    /// - `text`: The font used for regular text.
    /// - `label`: The font used for labels.
    /// - `keyboard`: The font used for keyboard elements.
    /// - `emoji`: The font used for emoji characters.
    ///
    /// # Returns
    /// A `Fonts` struct with the provided fonts.
    pub fn new(
        heading: resources::Font, 
        text: resources::Font, 
        label: resources::Font, 
        keyboard: resources::Font, 
        emoji: resources::Font
    ) -> Self {
        Self { heading, text, label, keyboard, emoji }
    }

    /// Loads the default fonts using the provided context.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    ///
    /// # Returns
    /// A `Fonts` struct with default fonts loaded from the specified paths.
    pub fn default(ctx: &mut Context) -> Self {
        let bold = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let medium = ctx.load_font("fonts/outfit_medium.ttf").unwrap();
        let regular = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        let emoji = ctx.load_font("fonts/noto_color_emoji.ttf").unwrap();
        
        Self {
            heading: bold.clone(),
            text: regular,
            label: bold,
            keyboard: medium,
            emoji,
        }
    }
}

/// Defines a struct that holds font sizes for various UI elements.
#[derive(Copy, Clone)]
pub struct FontSize {
    /// The font size used for title text.
    pub title: f32,
    /// The font size used for h1 (primary) headers.
    pub h1: f32,
    /// The font size used for h2 (secondary) headers.
    pub h2: f32,
    /// The font size used for h3 headers.
    pub h3: f32,
    /// The font size used for h4 headers.
    pub h4: f32,
    /// The font size used for h5 headers.
    pub h5: f32,
    /// The font size used for h6 headers.
    pub h6: f32,
    /// The font size used for extra-large text.
    pub xl: f32,
    /// The font size used for large text.
    pub lg: f32,
    /// The font size used for medium text.
    pub md: f32,
    /// The font size used for small text.
    pub sm: f32,
    /// The font size used for extra-small text.
    pub xs: f32,
}

impl Default for FontSize {
    /// Returns the default font sizes used throughout the application.
    ///
    /// # Returns
    /// A `FontSize` struct with default font sizes for various elements.
    fn default() -> Self {
        FontSize {
            title: 72.0,
            h1: 48.0,
            h2: 32.0,
            h3: 24.0,
            h4: 20.0,
            h5: 16.0,
            h6: 14.0,
            xl: 24.0,
            lg: 20.0,
            md: 16.0,
            sm: 14.0,
            xs: 12.0,
        }
    }
}
