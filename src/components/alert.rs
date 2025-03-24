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

pub struct Alert(pub Icon, pub BasicText);

impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, err: Option<&'static str>) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.status.warning;

        AmountDisplay (
            Icon::new(ctx, "warning", color, 32),
            Text::new(ctx, title, TextStyle::Heading, font_size)
        )
    }
}

impl Component for AmountDisplay {
    fn build(&mut self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        Container::new(Row(4, RowOffset::Center), vec![&mut self.0, &mut self.1])
    }
}

pub struct Alert(pub &'static str); 

impl ComponentBuilder for AmountDisplay {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Row(None, 4, Align::Center, vec![
            Icon::Warning.build(32, COLORS.status.warning), // Warning Icon
            Text::primary(ctx, self.0, TextSize::md()) // Warning Text
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}