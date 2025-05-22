use rust_on_rails::prelude::*;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::{OutlinedRectangle, Rectangle};
use crate::components::avatar::{Avatar, AvatarContent};
use crate::components::button::ButtonState;
use crate::layout::{Column, Stack, Bin, Padding, Offset, Size};
use crate::PelicanUI;

/// Represents a `Card` component, often used for marketing or promoting content.
/// 
/// A `Card` is a UI element commonly used to display information with visual hierarchy. It typically
/// includes elements like an avatar, title, subtitle, description, and a button to trigger actions
/// like clicks for more information. The card is styled with a background and an outline, and has
/// an on-click callback for interaction.
#[derive(Debug, Component)]
pub struct Card(Stack, OutlinedRectangle, CardContent, #[skip] ButtonState, #[skip] fn(&mut Context) -> ());
impl Card {
    /// Creates a new `Card` component.
    ///
    /// This method constructs a new card with an avatar, title, subtitle, description, and an
    /// `on_click` callback. The card is styled with a background color and an outline, and is laid out
    /// using a stack arrangement. This card can be used to promote or market content in a UI.
    ///
    /// # Parameters:
    /// - **`ctx`**: The [`Context`] for accessing the app's theme.
    /// - **`avatar`**: The avatar content to display in the card (can be an image or icon).
    /// - **`title`**: The main title for the card (usually displayed prominently).
    /// - **`subtitle`**: A secondary title or tagline that provides more context.
    /// - **`description`**: A longer description or information about the content being promoted.
    /// - **`on_click`**: A function that will be executed when the card is clicked.
    ///
    /// # Returns:
    /// - **`Card`**: The constructed `Card` component.
    ///
    /// # Example:
    /// ```rust
    /// let card = Card::new(ctx, AvatarContent::Image(image), "My Title", "Subtitle", "This is a description", on_click_fn);
    /// ```
    pub fn new(
        ctx: &mut Context,
        avatar: AvatarContent, 
        title: &str, 
        subtitle: &str, 
        description: &str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        let (bg, oc) = (colors.background.primary, colors.outline.secondary);
        let background = OutlinedRectangle::new(bg, oc, 16.0, 1.0);
        let content = CardContent::new(ctx, avatar, title, subtitle, description);
        let layout = Stack(
            Offset::Center, Offset::Center, 
            Size::custom(|widths: Vec<(f32, f32)>| (widths[1].0, f32::MAX)), 
            Size::custom(|heights: Vec<(f32, f32)>| heights[1]), 
            Padding::default()
        );

        Card(layout, background, content, ButtonState::Default, on_click)
    }
}

impl OnEvent for Card {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.3 {
                    ButtonState::Default | ButtonState::Hover => (self.4)(ctx),
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

#[derive(Debug, Component)]
struct CardContent(Column, Avatar, Text, Text, Bin<Stack, Rectangle>, Text);
impl OnEvent for CardContent {}

impl CardContent {
    fn new(
        ctx: &mut Context, 
        avatar: AvatarContent, 
        title: &str, 
        subtitle: &str, 
        description: &str
    ) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (font_size, color) = (theme.fonts.size, theme.colors.outline.secondary);
        CardContent(
            Column::new(8.0, Offset::Center, Size::Fit, Padding(16.0, 16.0, 16.0, 16.0)),
            Avatar::new(ctx, avatar, None, false, 64.0, None),
            Text::new(ctx, title, TextStyle::Heading, font_size.h3, Align::Left),
            Text::new(ctx, subtitle, TextStyle::Primary, font_size.xs, Align::Left),
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Fit, Size::Static(1.0), Padding(0.0, 6.0, 0.0, 6.0)), 
                Rectangle::new(color)
            ),
            Text::new(ctx, description, TextStyle::Primary, font_size.sm, Align::Left),
        )
    }
}

// let card = Card {
//     circle_icon: CircleIconData::Photo(Image("../photos/chicken_on_a_donkey.png")),
//     title: "Donkey Lovers",
//     subtitle: "101 members",
//     description: "A place for donkey lovers to converse.",
// }
