use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component, Align, Image};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Offset, Padding, Row, Size};

#[derive(Debug, Component)]
pub struct Alert(Row, Image, Text);
impl OnEvent for Alert {}

impl Alert {
    pub fn new(ctx: &mut Context, message: &str) -> Self {
        let theme = &ctx.theme;
        let (color, font_size) = (theme.colors.status.warning, theme.fonts.size.md);

        Alert(
            Row::new(4.0, Offset::Center, Size::Fit, Padding::default()),
            Icon::new(ctx, "warning", color, 32.0),
            Text::new(ctx, message, TextStyle::Primary, font_size, Align::Left)
        )
    }
}
