use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Row, Stack, Column, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct AmountDisplay(Column, BasicText, SubText);
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
            SubText::new(ctx, btc, err)
        )
    }
}

#[derive(Debug, Component)]
struct SubText(Row, Option<Image>, BasicText);
impl Events for SubText {}

impl SubText {
    fn new(ctx: &mut Context, btc: &'static str, err: Option<&'static str>) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (font_size, color) = (theme.fonts.size.lg, theme.colors.status.danger);
        let (icon, style, text) = match err {
            Some(err) => (Some(Icon::new(ctx, "error", color, 24)), TextStyle::Error, err),
            None => (None, TextStyle::Secondary, btc)
        };

        SubText(
            Row::center(8),
            icon, Text::new(ctx, text, style, font_size)
        )
    }
}
