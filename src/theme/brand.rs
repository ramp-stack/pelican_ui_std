use rust_on_rails::prelude::*;
use std::collections::HashMap;
/// Represents the brand resources used in the theme, including the logomark, wordmark, app icon, and illustrations.
///
/// `BrandResources` encapsulates various visual elements that represent the brand of the application.
/// This includes images for the logomark, wordmark, app icon, and a set of illustrations associated with the brand.
///
/// # Fields:
/// - `wordmark`: An image representing the wordmark of the application (typically the name or logo in text).
/// - `logomark`: An image representing the logomark (a graphical symbol or logo without text).
/// - `app_icon`: An image representing the app icon that will be used on device home screens or taskbars.
/// - `illustrations`: A collection of illustrations associated with the brand, accessible by name.
#[derive(Clone, Debug)]
pub struct BrandResources {
    /// The wordmark image for the brand.
    pub wordmark: resources::Image,
    /// The logomark image for the brand.
    pub logomark: resources::Image,
    /// The app icon image for the brand.
    pub app_icon: resources::Image,
    /// A collection of illustrations associated with the brand.
    pub illustrations: Illustrations
}

impl BrandResources {
    /// Creates a new instance of `BrandResources` with custom images for the logomark, wordmark, app icon, and illustrations.
    ///
    /// # Parameters:
    /// - `logomark`: The image to use for the logomark.
    /// - `wordmark`: The image to use for the wordmark.
    /// - `app_icon`: The image to use for the app icon.
    /// - `illustrations`: The set of illustrations to associate with the brand.
    ///
    /// # Returns
    /// A new `BrandResources` instance with the provided resources.
    ///
    /// # Example:
    /// ```rust
    /// let brand = BrandResources::new(logomark_image, wordmark_image, app_icon_image, illustrations);
    /// ```
    pub fn new(
        logomark: resources::Image, 
        wordmark: resources::Image,
        app_icon: resources::Image,
        illustrations: Illustrations
    ) -> Self {
        BrandResources { logomark, wordmark, app_icon, illustrations }
    }

    /// Loads the default brand resources, including SVG images for the logomark, wordmark, app icon, and default illustrations.
    ///
    /// This method automatically loads the SVG files from the application’s resource folder and initializes the `BrandResources`
    /// with default values. It also loads illustrations using the `Illustrations::default` method.
    ///
    /// # Parameters:
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    ///
    /// # Returns
    /// A `BrandResources` instance initialized with the default SVG images and illustrations.
    ///
    /// # Example:
    /// ```rust
    /// let mut ctx = Context::new();
    /// let default_brand = BrandResources::default(&mut ctx);
    /// ```
    pub fn default(ctx: &mut Context) -> Self {
        BrandResources {
            logomark: ctx.add_svg(&ctx.load_file("brand/logomark.svg").unwrap(), 8.0),
            wordmark: ctx.add_svg(&ctx.load_file("brand/wordmark.svg").unwrap(), 8.0),
            app_icon: ctx.add_svg(&ctx.load_file("brand/app_icon.svg").unwrap(), 8.0),
            illustrations: Illustrations::default(ctx),
        }
    }
}

/// Represents a collection of illustrations associated with the brand, where each illustration can be accessed by its name.
///
/// The `Illustrations` struct stores a mapping of illustration names to images. It allows for retrieving illustrations
/// by name and adding or updating illustrations in the collection.
///
/// # Fields:
/// - `0`: A `HashMap` that stores the mapping of illustration names to their respective `resources::Image`.
#[derive(Clone, Debug)]
pub struct Illustrations(HashMap<&'static str, resources::Image>);

impl Illustrations {
    /// Loads the default set of illustrations for the brand.
    ///
    /// This method loads a predefined set of illustrations from the application's resource folder. Illustrations can be
    /// accessed by their name using the `get` method.
    ///
    /// # Parameters:
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    ///
    /// # Returns
    /// A new `Illustrations` instance with the default illustrations.
    ///
    /// # Example:
    /// ```rust
    /// let mut ctx = Context::new();
    /// let default_illustrations = Illustrations::default(&mut ctx);
    /// ```
    pub fn default(ctx: &mut Context) -> Self {
        let mut illustrations = HashMap::new();

        illustrations.insert("dodo", ctx.add_svg(&ctx.load_file("brand/illustrations/dodo.svg").unwrap(), 8.0));
        illustrations.insert("hummingbird", ctx.add_svg(&ctx.load_file("brand/illustrations/hummingbird.svg").unwrap(), 8.0));
        illustrations.insert("toucan", ctx.add_svg(&ctx.load_file("brand/illustrations/toucan.svg").unwrap(), 8.0));
        illustrations.insert("emu", ctx.add_svg(&ctx.load_file("brand/illustrations/emu.svg").unwrap(), 8.0));

        Self(illustrations)
    }

    /// Retrieves an illustration by its name.
    ///
    /// This method looks up an illustration by its name and returns a clone of the associated `resources::Image`.
    /// If the illustration is not found, it will panic.
    ///
    /// # Parameters:
    /// - `name`: The name of the illustration to retrieve.
    ///
    /// # Returns
    /// A clone of the `resources::Image` associated with the given name.
    ///
    /// # Panics
    /// If the illustration is not found, it will panic with a message indicating the missing illustration name.
    ///
    /// # Example:
    /// ```rust
    /// let illustration = illustrations.get("dodo");
    /// ```
    pub fn get(&self, name: &'static str) -> resources::Image {
        self.0.get(name).unwrap_or_else(|| panic!("Could not find illustration {:?}", name)).clone()
    }

    /// Adds or updates an illustration in the collection.
    ///
    /// This method either inserts a new illustration or updates an existing one in the collection.
    ///
    /// # Parameters:
    /// - `name`: The name of the illustration.
    /// - `illustration`: The new or updated `resources::Image` to associate with the name.
    ///
    /// # Example:
    /// ```rust
    /// illustrations.add_illustration("new_icon", new_illustration);
    /// ```
    pub fn add_illustration(&mut self, name: &'static str, illustration: resources::Image) {
        if let Some(existing) = self.0.get_mut(&name) {
            *existing = illustration; 
        } else {
            self.0.insert(name, illustration);
        }
    }
}
