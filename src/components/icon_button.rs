use rust_on_rails::prelude::*;
use crate::{ Child, Row, COLORS, ZERO, Align, Stack };
use crate::theme::fonts::{Text, TextSize};

#[derive(Clone, Copy)]
pub struct IconButton(Icon, IconButtonStyle, Size);

impl ComponentBuilder for Button {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let (size, icon_size) = match (self.2, self.1) {
            (IconButtonStyle::Secondary, Size::Large) => (48, 32),
            (IconButtonStyle::Secondary, Size::Medium) => (32, 20),
            (IconButtonStyle::Ghost, Size::Large) => (48, 48),
            (IconButtonStyle::Ghost, Size::Medium) => (32, 32)
        }   

        let colors = COLORS.icon_button.colors_from(self.1, ButtonState::Default);

        Stack(ZERO, Align::Center, vec![
            RoundedRectangle(size, size, colors.background, Some((colors.outline, 1))),
            self.icon.build(icon_size, colors.icon)
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum IconButtonStyle {
    Secondary,
    Ghost
}