use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Row, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Clone, Debug, Component)]
pub struct Alert(Row, Icon, BasicText);
impl Events for Alert {}

impl Alert {
    pub fn new(ctx: &mut Context, title: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.status.warning, theme.fonts.size.md);

        Alert (
            Row(4, Offset::Center, Size::Fit, Padding::default()),
            Icon::new(ctx, "warning", color, 32),
            Text::new(ctx, title, TextStyle::Primary, font_size)
        )
    }
}