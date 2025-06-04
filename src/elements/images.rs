use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{ShapeType, Image, Color};
use pelican_ui::{Context, resources};

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
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `name`: The name of the icon to be displayed.
    /// - `color`: The color applied to the icon.
    /// - `size`: The size of the icon.
    ///
    /// # Returns:
    /// A new `Image` component with the specified icon, color, and size.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: f32) -> Image {
        let icon = ctx.theme.icons.get(name);
        Image{shape: ShapeType::Rectangle(0.0, (size, size)), image: icon, color: Some(color)}
    }
}

/// A component representing a brand image that can be displayed in the UI.
#[derive(Clone, Debug)]
pub struct Brand;
impl OnEvent for Brand {}
impl Brand {
    /// Creates a new `Brand` instance with a specified image and size.
    ///
    /// # Parameters:
    /// - `image`: The image representing the brand.
    /// - `size`: A tuple specifying the width and height of the image.
    ///
    /// # Returns:
    /// A new `Image` component with the specified image and size.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(image: resources::Image, size: (f32, f32)) -> Image {
        Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image, color: None}
    }
}
