use rust_on_rails::prelude::*;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Offset, Padding, Row, Size};
use crate::PelicanUI;

/// A UI component that displays a warning message along with a warning icon.
#[derive(Debug, Component)]
pub struct Alert(Row, Image, Text);
impl OnEvent for Alert {}

impl Alert {
    /// Creates a new `Alert` with a warning message.
    ///
    /// # Parameters
    /// - `ctx`: Context.
    /// - `message`: The warning message to display.
    ///
    /// # Returns
    /// - A new `Alert` component with a warning icon and a message.
    pub fn new(ctx: &mut Context, message: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.status.warning, theme.fonts.size.md);

        Alert(
            Row(4.0, Offset::Center, Size::Fit, Padding::default()),
            Icon::new(ctx, "warning", color, 32.0),
            Text::new(ctx, message, TextStyle::Primary, font_size, Align::Left)
        )
    }

    /// Retrieves and allows modifying the message of the alert.
    ///
    /// # Returns
    /// - A mutable reference to the message text stored in the `Alert`.
    pub fn message(&mut self) -> &mut String { &mut self.2.text().spans[0].text }
}
