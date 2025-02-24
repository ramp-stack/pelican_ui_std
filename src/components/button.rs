use rust_on_rails::prelude::*;
use crate::{ Child, Row, COLORS, ZERO, Align, Stack };
use crate::theme::fonts::{Text, TextSize};

#[derive(Clone, Copy)]
pub struct Button(&'static str, Size, Width, Option<&'static str>, ButtonStyle, Align);

impl Button {
    pub fn primary(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Primary, a)
    }
    pub fn secondary(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Secondary, a)
    }
    pub fn ghost(n: &'static str, s: Size, w: Width, ip: Option<&'static str>, a: Align) -> Self {
        Self(n, s, w, ip, ButtonStyle::Ghost, a)
    }
}

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let colors = COLORS.button.colors_from(self.4, ButtonState::Default);
        // let image = ctx.load_image("icons/pfp.png").unwrap(); // GET DESIRED ICON

        let bound = Rect::new(0, 0, max_size.x, max_size.y);

        let (text_size, height, _icon_size, x_padding) = match self.1 {
            Size::Medium => (TextSize::md(), 32, 16, 12),
            Size::Large => (TextSize::lg(), 48, 24, 24)
        };

        let content = Row {spacing: 3, padding: ZERO, align: Align::Center, children: vec![
            (Child!(Text::label(ctx, self.0, text_size)), false)
        ]}.build(ctx, bound);

        let width = match self.2 {
            Width::Hug => content.size(ctx).x + (x_padding * 2),
            Width::Expand => max_size.x,
        };

        Stack {padding: ZERO, align: self.5, children: vec![
            (Child!(Shape(ShapeType::Rectangle(width, height), colors.background, None)), ZERO),
            (Child!(Shape(ShapeType::Rectangle(width, height), colors.outline, Some(200))), ZERO),
            (Child!(Text::label(ctx, self.0, text_size)), Vec2::new(x_padding, 0))
        ]}.build_children(ctx, max_size)
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
