use rust_on_rails::prelude::*;
use crate::{theme, Text, Row, Column, Stack, COLORS};
use crate::layout::Align;

#[derive(Clone, Copy)]
pub struct Button(&'static str, Size, Width, Option<&'static str>, ButtonStyle, Align);

impl Button {
    pub fn Primary(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Primary, a)
    }
    pub fn Secondary(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Secondary, a)
    }
    pub fn Ghost(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Ghost, a)
    }
}

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let colors = COLORS.button.colors_from(self.4, ButtonState::Default);
        let font = ctx.load_font("fonts/outfit_bold.ttf").unwrap(); // GET LABEL FONT
        let image = ctx.load_image("icons/pfp.png").unwrap(); // GET DESIRED ICON

        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        let (text_size, height, icon_size, x_padding) = match self.1 {
            Size::Medium => (16, 32, 16, 12),
            Size::Large => (20, 48, 24, 24)
        };

        let mut content = Row!(3, Vec2::new(0, 0), Align::Center, false,
            // Image(ShapeType::Rectangle(icon_size, 8), image),
            (Text::new(self.0, colors.label, text_size, font.clone()), false)
        ).build(ctx, bound);

        let width = match self.2 {
            Width::Hug => content.size(ctx).x + (x_padding * 2),
            Width::Expand => max_size.x,
        };

        Stack!(Vec2::new(0, 0), self.5,
            (Shape(ShapeType::Rectangle(width, height), colors.background, None), Vec2::new(0, 0)),
            (Shape(ShapeType::Rectangle(width, height), colors.outline, Some(200)), Vec2::new(0, 0)),
            (Text::new(self.0, colors.label, text_size, font.clone()), Vec2::new(x_padding, 0))
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
