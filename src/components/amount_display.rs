use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Row, Column, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Clone, Debug, Component)]
pub struct AmountDisplay(Column, BasicText, Message);
impl Events for AmountDisplay {}

impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, err: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let font_size = match usd.len() {
            0..=4 => font_size.title,
            5..=7 => font_size.h1,
            _ => font_size.h2
        };

        AmountDisplay (
            Column(16, Offset::Center, Size::Fit, Padding(16, 64, 16, 64)),
            Text::new(ctx, usd, TextStyle::Heading, font_size),
            Message::new(ctx, btc, err)
        )
    }
}

#[derive(Clone, Debug, Component)]
struct Message(Row, Option<Icon>, BasicText);
impl Events for Message {}

impl Message {
    fn new(ctx: &mut Context, btc: &'static str, err: Option<&'static str>) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (font_size, color) = (theme.fonts.size.lg, theme.colors.status.danger);
        let (icon, style, text) = match err {
            Some(err) => (Some(Icon::new(ctx, "error", color, 24)), TextStyle::Error, err),
            None => (None, TextStyle::Secondary, btc)
        };

        Message(
            Row::center(8),
            icon, Text::new(ctx, text, style, font_size)
        )
    }
}
