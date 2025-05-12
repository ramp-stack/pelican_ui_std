use rust_on_rails::prelude::*;
use crate::PelicanUI;

/// A component representing an icon that can be displayed in the UI.
///
/// The `Icon` struct allows you to create an icon with a specified name, color, and size.
/// The icon's appearance is defined by the provided name and color, which are used to
/// load and display the appropriate image.
#[derive(Clone, Debug)]
pub struct Icon;
impl OnEvent for Icon {}
impl Icon {
    /// Creates a new `Icon` instance with a specified name, color, and size.
    ///
    /// # Parameters:
    /// - `ctx`: The context to access resources like the theme.
    /// - `name`: The name of the icon to be displayed.
    /// - `color`: The color applied to the icon.
    /// - `size`: The size of the icon.
    ///
    /// # Returns:
    /// A new `Image` component with the specified icon, color, and size.
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: f32) -> Image {
        let icon = ctx.get::<PelicanUI>().theme.icons.get(name);
        Image{shape: ShapeType::Rectangle(0.0, (size, size)), image: icon, color: Some(color)}
    }
}

/// Creates a new `Brand` instance with a specified image and size.
///
/// # Parameters:
/// - `image`: The image representing the brand.
/// - `size`: A tuple specifying the width and height of the image.
///
/// # Returns:
/// A new `Image` component with the specified image and size.
#[derive(Clone, Debug)]
pub struct Brand;
impl OnEvent for Brand {}
impl Brand {
    pub fn new(image: resources::Image, size: (f32, f32)) -> Image {
        Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image, color: None}
    }
}
