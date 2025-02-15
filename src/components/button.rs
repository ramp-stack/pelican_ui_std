use rust_on_rails::prelude::*;
use crate::{theme, Text, Row, Column, Stack, Alignment};

#[derive(Clone, Copy)]
pub enum Button {
    Primary(&'static str, Size, Width, Option<&'static str>),
    Secondary(&'static str, Size, Width, Option<&'static str>),
    Ghost(&'static str, Size, Width, Option<&'static str>)
}

impl Button {
    fn destructure(&self) -> (&'static str, Size, Width, Option<&'static str>, ButtonStyle) {
        match self {
            Button::Primary(a, b, c, d) => (*a, *b, *c, *d, ButtonStyle::Primary),
            Button::Secondary(a, b, c, d) => (*a, *b, *c, *d, ButtonStyle::Secondary),
            Button::Ghost(a, b, c, d) => (*a, *b, *c, *d, ButtonStyle::Ghost),
        }
    }
}

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let (label, size, width, icon, style) = self.destructure();
        let colors = theme::color::palette().button.colors_from(style, ButtonState::Default);
        let font = ctx.load_font("fonts/outfit_bold.ttf").unwrap(); // GET LABEL FONT
        let image = ctx.load_image("icons/pfp.png").unwrap(); // GET DESIRED ICON

        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        let (text_size, height, icon_size) = match size {
            Size::Medium => (16, 32, 16),
            Size::Large => (20, 48, 24)
        };

        let mut content = Row!(3, Alignment::Center, 
            // Image(ShapeType::Rectangle(icon_size, 8), image),
            Text::new(label, colors.label, text_size, font.clone())
        ).build(ctx, bound);

        let width = match width {
            Width::Hug => content.size(ctx).x + 48,
            Width::Expand => max_size.x,
        };

        Stack!(
            Shape(ShapeType::Rectangle(width, height), colors.background, None),
            Shape(ShapeType::Rectangle(width, height), colors.outline, Some(200)),
            Text::new(label, colors.label, text_size, font.clone())
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Expand,
    Hug,
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Large,
    Medium,
}
