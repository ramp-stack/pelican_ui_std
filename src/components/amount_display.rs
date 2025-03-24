use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::circle_icon::{CircleIcon, CircleIconContent, CircleIconStyle};
use crate::layout::{Row, RowOffset, Column, ColumnOffset, Stack, Offset, Size};
use crate::PelicanUI;

// Rules:
// Exported structs and enums prefixed with name of the "top-layer" component.
// If a struct or enum isn’t exported, start its name with _.
// First item in a file should be top-layer component struct or enum
// 'User' should never touch the struct, only new functions

pub struct AmountDisplay(pub BasicText, pub _Message);

impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, err: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let font_size = match usd.len() {
            0..=4 => font_size.title,
            5..=7 => font_size.h1,
            _ => font_size.h2
        };

        AmountDisplay (
            Text::new(ctx, title, TextStyle::Heading, font_size),
            _Message::new(btc, err)
        )
    }
}

impl Component for AmountDisplay {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        Container::new(Column(32, ColumnOffset::Center), vec![&mut self.0, &mut self.1])
    }
}

struct _Message(pub Option<Icon>, pub BasicText);

impl _Message {
    pub fn new(btc: &'static str, err: Option<&'static str>) -> Self {
        match err {
            Some(err) => _Message(Some(Icon::new(ctx, "error", 24)), Text::new(ctx, err, TextStyle::Error, font_size.lg)),
            None => _Message(None, Text::new(ctx, btc, TextStyle::Secondary, font_size.lg))
        }
    }
}

impl Component for _Message {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![];
        if let Some(icon) = &mut self.0 { children.push(icon); }
        children.push(&mut self.1);
        Container::new(Row(8, RowOffset::Center), children)
    }
}