use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;

use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Offset, Padding, Row, Size};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct Alert(Row, Image, Text);
impl OnEvent for Alert {}

impl Alert {
    pub fn new(ctx: &mut Context, message: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.status.warning, theme.fonts.size.md);

        Alert(
            Row(4.0, Offset::Center, Size::Fit, Padding::default()),
            Icon::new(ctx, "warning", color, 32.0),
            Text::new(ctx, message, TextStyle::Primary, font_size, Align::Left)
        )
    }

    pub fn message(&mut self) -> &mut String { &mut self.2.text().spans[0].text }
}
