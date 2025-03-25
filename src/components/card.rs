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

pub struct Card(pub Stack, pub _Card, pub _CardBackground);

impl Card {
    pub fn new(
        ctx: &mut Context,
        circle_icon: CircleIconContent, 
        title: &'static str, 
        subtitle: &'static str, 
        description: &'static str,
    ) -> Self {
        Card (
            Column(32, Offset::Center, Size::Fill),
            _Card::new(ctx, circle_icon, title, subtitle, description),
            _CardBackground::new(max_width.0),
        )
    }
}

impl Component for Card {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
    fn layout(&self) -> &dyn Layout {&self.0}
}

struct _Card(pub Column, pub CircleIcon, pub BasicText, pub BasicText, pub _SeparationLine, pub BasicText);

impl _Card {
    pub fn new(
        ctx: &mut Context,
        circle_icon: CircleIconContent, 
        title: &'static str, 
        subtitle: &'static str, 
        description: &'static str,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        _Card (
            Column(8, Offset::Center, Size::Fill),
            CircleIcon::new(ctx, circle_icon, None, false, 64),
            Text::new(ctx, title, TextStyle::Heading, font_size.h3),
            Text::new(ctx, subtitle, TextStyle::Primary, font_size.xs),
            _SeparationLine(max_width.0),
            Text::new(ctx, description, TextStyle::Primary, font_size.sm),
        )
    }
}

impl Component for Card {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2, &self.3, &self.4, self.5]}
    fn layout(&self) -> &dyn Layout {&self.0}
}


pub struct _CardBackground(pub Stack, pub Shape, pub Shape);

impl _CardBackground {
    pub fn new(bg: Color, oc: Color, r: u32, h: u32, w: u32) -> Self {
        // Height hug, width fill
        let colors = ctx.get::<PelicanUI>().theme.colors;
        _CardBackground(
            Stack((Offset::Center, Offset::Center), (Size::Fit, Size::Fit)),
            RoundedRectangle::new(w, h, 16, colors.background.primary),
            Outline::rounded_rectangle(w, h, 16, 1, colors.outline.secondary)
        )
    }
}

impl Component for _CardBackground {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
    fn layout(&self) -> &dyn Layout {&self.0}
}

struct _SeparationLine(pub Column, pub Shape, pub Shape, pub Shape);

impl _SeparationLine {
    pub fn new(bg: Color, oc: Color, r: u32, h: u32, w: u32) -> Self {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        _SeparationLine(
            Column(0, Offset::Start, Size::Fit),
            RoundedRectangle::new(w, 6, 0, colors.shades.transparent),
            RoundedRectangle::new(w, 1, 0, colors.outline.secondary),
            RoundedRectangle::new(w, 6, 0, colors.shades.transparent),
        )
    }
}

impl Component for _SeparationLine {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2, &mut self.3]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2, &self.3]}
    fn layout(&self) -> &dyn Layout {&self.0}
}

// let card = Card {
//     circle_icon: CircleIconData::Photo(Image("../photos/chicken_on_a_donkey.png")),
//     title: "Donkey Lovers",
//     subtitle: "101 members",
//     description: "A place for donkey lovers to converse.",
// }