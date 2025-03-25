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

pub struct AmountDisplay(pub Column, pub BasicText, pub _Message);


impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, err: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let font_size = match usd.len() {
            0..=4 => font_size.title,
            5..=7 => font_size.h1,
            _ => font_size.h2
        };

        AmountDisplay (
            Column(32, Offset::Center, Size::Fill),
            Text::new(ctx, title, TextStyle::Heading, font_size),
            _Message::new(btc, err)
        )
    }
}

impl Component for AmountDisplay {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
    fn layout(&self) -> &dyn Layout {&self.0}
}

struct _Message(pub Row, pub Option<Icon>, pub BasicText);

impl _Message {
    pub fn new(btc: &'static str, err: Option<&'static str>) -> Self {
        let row = Row(8, Offset::Center, Size::Fit);
        match err {
            Some(err) => _Message(row, Some(Icon::new(ctx, "error", 24)), Text::new(ctx, err, TextStyle::Error, font_size.lg)),
            None => _Message(row, None, Text::new(ctx, btc, TextStyle::Secondary, font_size.lg))
        }
    }
}

impl Component for _Message {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![];
        if let Some(icon) = &mut self.1 { children.push(icon); }
        children.push(&mut self.2);
        children
    }
    fn children_mut(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![];
        if let Some(icon) = &self.1 { children.push(icon); }
        children.push(&self.2);
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}