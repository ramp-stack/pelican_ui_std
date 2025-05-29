use rust_on_rails::prelude::*;
use crate::elements::images::Icon;
use crate::elements::shapes::{Outline, Circle};
use crate::layout::{Stack, Offset, Size, Row, Padding};
use crate::plugin::PelicanUI;
use crate::utils::Callback;

/// A UI component that represents a user avatar, which can be either an icon or an image.
///
/// The `Avatar` struct displays a user's avatar with support for both images and icons. 
/// It can optionally include an outline and a flair icon (such as an 'edit' or 'block' icon). 
/// The avatar can be customized with different sizes and styles.
///
/// The avatar is constructed from either an image or an icon, and it automatically handles 
/// the layout and appearance based on the provided configuration, including size and flair.
///
/// # Example
/// ```rust
/// let avatar = Avatar::new(ctx, AvatarContent::Icon("profile", AvatarIconStyle::Secondary), 
///     Some(("edit", AvatarIconStyle::Secondary)), true, 48.0);
/// ```
#[derive(Component)]
pub struct Avatar(Stack, MainAvatar, Option<Flair>, #[skip] pub Option<Callback>);

impl Avatar {
    /// Creates a new `Avatar` component.
    ///
    /// This function allows you to create an avatar using either an image or an icon. You can 
    /// customize the appearance with a flair, outline, and size.
    ///
    /// # Parameters:
    /// - **`ctx`**: The [`Context`] for accessing the app's theme.
    /// - **`content`**: The content of the avatar, which can either be an image or an icon.
    /// - **`flair`**: An optional tuple containing the name and style of the flair to be added to the avatar.
    /// - **`outline`**: A boolean flag to indicate whether the avatar should have a circular outline.
    /// - **`size`**: The size of the avatar.
    /// - **`on_click`**: Closure to execute on click.
    ///
    /// # Returns:
    /// A newly created `Avatar` component.
    pub fn new(
        ctx: &mut Context, 
        content: AvatarContent, 
        flair: Option<(&'static str, AvatarIconStyle)>, 
        outline: bool, 
        size: f32,
        on_click: Option<Callback>
    ) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;

        Avatar(
            Stack(Offset::End, Offset::End, Size::Fit, Size::Fit, Padding::default()),
            MainAvatar::new(ctx, content, outline, size),
            flair.map(|(name, style)| Flair::new(ctx, name, style, size / 3.0, black)),
            on_click
        )
    }

    pub fn set_content(&mut self, content: AvatarContent)  {self.1.set_content(content)}

    /// Gets a mutable reference to the optional [`Flair`] component.
    pub fn flair(&mut self) -> &mut Option<Flair> {&mut self.2}
    /// Gets a mutable reference to the optional [`Outline`] component.
    pub fn outline(&mut self) -> &mut Option<Shape> {&mut self.1.3}
}

impl OnEvent for Avatar {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent{state: MouseState::Pressed, position: Some(_)}) = event.as_any_mut().downcast_mut::<MouseEvent>() {
            if let Some(on_click) = &mut self.3 {
                #[cfg(target_os = "ios")]
                crate::vibrate();
                (on_click)(ctx)
            }
        }
        false
    }
}

impl std::fmt::Debug for Avatar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Avatar...")
    }
}

#[derive(Component, Debug)]
struct MainAvatar(Stack, Option<AvatarIcon>, Option<Image>, Option<Shape>);
impl OnEvent for MainAvatar {}

impl MainAvatar {
    fn new(
        ctx: &mut Context, 
        content: AvatarContent,
        outline: bool, 
        size: f32,
    ) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;

        let (circle_icon, image) = match content {
            AvatarContent::Image(image) => (None, Some(Image{shape: ShapeType::Ellipse(0.0, (size, size)), image, color: None})),
            AvatarContent::Icon(name, style) => (Some(AvatarIcon::new(ctx, name, style, size)), None)
        };

        MainAvatar(
            Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding::default()),
            circle_icon, image, outline.then(|| Outline::circle(size, black)),
        )
    }

    fn set_content(&mut self, content: AvatarContent) {
        match content {
            AvatarContent::Image(image) => {
                if let Some(avatar_image) = &mut self.2 {
                    avatar_image.image = image;
                } else {
                    let size = self.1.as_mut().unwrap().1.shape.size().0 + 2.0;
                    self.2 = Some(Image{shape: ShapeType::Ellipse(0.0, (size, size)), image, color: None});
                }

                self.1 = None;
            }
            AvatarContent::Icon(_name, _style) => {/* to do */}
        };
    }
}

/// Enum representing the content of an avatar, which can either be an icon or an image.
///
/// The [`AvatarContent`] enum allows for two types of avatar content: an icon or an image. The icon is represented
/// by a string (typically a reference to an icon's name or identifier) and a style, while the image is represented
/// by an image resource.
///
/// # Variants
/// - `Icon(&str, [`AvatarIconStyle`])`: Represents an icon, with the icon's name and its style.
/// - `Image([`resources::Image`])`: Represents an image resource used as the avatar.
///
/// # Example
/// ```rust
/// let avatar = AvatarContent::Icon("profile", AvatarIconStyle::Secondary);
/// let avatar_image = AvatarContent::Image(image_resource);
/// ```
#[derive(Debug, Clone)]
pub enum AvatarContent {
    /// Represents an icon with a string identifier and its style.
    Icon(&'static str, AvatarIconStyle),
    
    /// Represents an image as the avatar content.
    Image(resources::Image)
}

/// Enum representing the style of an avatar icon, with predefined styles and a custom option.
///
/// The `AvatarIconStyle` enum allows selection of different styles for an avatar icon. The predefined styles
/// cover various common states (e.g., primary, secondary, success, etc.), and the `Custom` variant allows users
/// to define their own color scheme for the icon.
#[derive(Debug, Copy, Clone)]
pub enum AvatarIconStyle {
    /// Primary [`AvatarContent::Icon`] style. 
    Primary,
    /// Secondary [`AvatarContent::Icon`] style.
    Secondary,
    /// Brand [`AvatarContent::Icon`] style.
    Brand,
    /// Success [`AvatarContent::Icon`] style.
    Success,
    /// Warning [`AvatarContent::Icon`] style.
    Warning,
    /// Danger [`AvatarContent::Icon`] style.
    Danger,
    /// Custom [`AvatarContent::Icon`] style.
    Custom(Color, Color)
}

impl AvatarIconStyle {
    fn get(&self, ctx: &mut Context) -> (Color, Color) {
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            AvatarIconStyle::Primary => (colors.text.heading, colors.background.primary),
            AvatarIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
            AvatarIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
            AvatarIconStyle::Success => (colors.status.success, colors.shades.white),
            AvatarIconStyle::Warning => (colors.status.warning, colors.shades.white),
            AvatarIconStyle::Danger => (colors.status.danger, colors.shades.white),
            AvatarIconStyle::Custom(background, icon) => (*background, *icon)
        }
    }
}

/// A component that represents an avatar icon, combining a stack layout, circular shape, and an image icon.
///
/// The `AvatarIcon` component is used to display an avatar in the form of an icon, where the icon is placed
/// inside a circular background. The icon’s appearance can be customized using the `AvatarIconStyle`, which
/// controls its color scheme. The icon size is proportional to the provided `size` parameter.
///
/// # Fields
/// - `Stack`: A layout component used to arrange the icon and background in a stacked fashion.
/// - `Shape`: A circular background shape with a customizable color.
/// - `Image`: The actual icon image, which is styled and sized according to the given parameters.
///
/// # Example
/// ```rust
/// let ctx: &mut Context = ...;
/// let avatar_icon = AvatarIcon::new(ctx, "user_icon", AvatarIconStyle::Primary, 50.0);
/// ```
#[derive(Debug, Component)]
pub struct AvatarIcon(Stack, Shape, Image);

impl OnEvent for AvatarIcon {}

impl AvatarIcon {
    /// Creates a new [`AvatarIcon`] component with a circular background and an icon.
    ///
    /// This function initializes the [`AvatarIcon`] by creating a stack layout with a circular background
    /// and an icon. The background color and icon color are determined by the provided [`AvatarIconStyle`].
    /// The icon’s size is scaled according to the provided `size` parameter.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `name`: The name or identifier of the icon to display.
    /// - `style`: The style of the avatar icon, which determines the colors.
    /// - `size`: The size of the avatar icon.
    ///
    /// # Returns
    /// A new [`AvatarIcon`] component with the specified icon and style.
    pub fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: f32) -> Self {
        let icon_size = size * 0.75;
        let (background, icon_color) = style.get(ctx);
        AvatarIcon(
            Stack::center(),
            Circle::new(size - 2.0, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }

    /// Returns a mutable reference to the [`Image`] component (icon) inside the [`AvatarIcon`].
    ///
    /// This method allows direct manipulation of the icon image within the [`AvatarIcon`].
    ///
    /// # Returns
    /// A mutable reference to the `Image` component representing the icon.
    pub fn icon(&mut self) -> &mut Image { &mut self.2 }
}

/// A component that represents an avatar flair, combining a stack layout, an avatar icon, and an outline shape.
///
/// The [`Flair`] component is used to display an avatar icon with an additional flair, typically in the form of
/// a circular outline surrounding the icon. The icon's appearance can be customized using the [`AvatarIconStyle`],
/// and the outline’s color can be specified. The icon size is proportional to the given `size` parameter.
///
/// # Fields
/// - `Stack`: A layout component that arranges the avatar icon and outline in a stacked fashion.
/// - `AvatarIcon`: The avatar icon component, which includes a circular background and a styled icon.
/// - `Shape`: An outline shape that surrounds the avatar icon, adding a flair effect.
///
/// # Example
/// ```rust
/// let ctx: &mut Context = ...;
/// let avatar_flair = Flair::new(ctx, "profile", AvatarIconStyle::Primary, 50.0, Color::BLACK);
/// ```
#[derive(Debug, Component)]
pub struct Flair(Stack, AvatarIcon, Shape);

impl OnEvent for Flair {}

impl Flair {
    /// Creates a new [`Flair`] component with an avatar icon and an outline flair.
    ///
    /// This function initializes the [`Flair`] component by creating a stack layout with the avatar icon and a circular
    /// outline surrounding the icon. The background color and icon color are determined by the provided `AvatarIconStyle`,
    /// and the outline color is set by the `color` parameter.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `name`: The name or identifier of the icon to display.
    /// - `style`: The style of the avatar icon, which determines the colors of the icon.
    /// - `size`: The size of the avatar icon and the outline.
    /// - `color`: The color of the outline surrounding the avatar icon.
    ///
    /// # Returns
    /// A new [`Flair`] component containing a stacked avatar icon with a circular outline.
    pub fn new(ctx: &mut Context, name: &'static str, style: AvatarIconStyle, size: f32, color: Color) -> Self {
        Flair(
            Stack::center(),
            AvatarIcon::new(ctx, name, style, size),
            Outline::circle(size, color)
        )
    }

    /// Returns a mutable reference to the [`Image`] component (icon) inside the [`AvatarIcon`] of [`Flair`].
    ///
    /// This method allows direct manipulation of the icon within the [`Flair`] component.
    ///
    /// # Returns
    /// A mutable reference to the [`Image`] component representing the avatar icon.
    pub fn icon(&mut self) -> &mut Image {self.1.icon()}
}

/// A component that represents a row of avatar icons, arranged in a horizontal layout.
///
/// The [`AvatarRow`] component is used to display a series of avatar icons in a row. Each avatar icon can
/// be customized using the [`AvatarContent`] enum, and the icons are displayed in a centered horizontal alignment.
/// The row layout and the avatars within it are flexible and can be adjusted by modifying the parameters.
///
/// # Fields
/// - `Row`: A layout component that arranges the avatars in a horizontal row with centered alignment.
/// - `Vec<Avatar>`: A vector of [`Avatar`] components, each representing a single avatar icon.
///
/// # Example
/// ```rust
/// let ctx: &mut Context = ...;
/// let avatar_row = AvatarRow::new(ctx, vec![AvatarContent::Icon("profile", AvatarIconStyle::Primary)]);
/// ```
#[derive(Debug, Component)]
pub struct AvatarRow(Row, Vec<Avatar>);

impl OnEvent for AvatarRow {}

impl AvatarRow {
    /// Creates a new [`AvatarRow`] component with a row of avatar icons.
    ///
    /// This function initializes the [`AvatarRow`] by creating a row layout and then generating avatar icons
    /// based on the provided `avatars` vector, which contains different types of avatar content (either an
    /// icon or an image). The avatars are displayed in a centered row with the default size of 32.0.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `avatars`: A vector of [`AvatarContent`] values that specify the content for each avatar (either an icon or an image).
    ///
    /// # Returns
    /// A new [`AvatarRow`] component containing a centered row of avatar icons.
    pub fn new(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        AvatarRow(
            Row::center(-16.0),
            avatars.into_iter().take(5).map(|avatar| Avatar::new(ctx, avatar, None, true, 32.0, None)).collect()
        )
    }
}
