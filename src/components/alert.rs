use rust_on_rails::prelude::*;
use crate::{ Child, Row, Column, COLORS, ZERO, Align, ConstrainedBox };
use crate::theme::fonts::{Text, TextSize};

pub struct Alert(pub &'static str); 

impl ComponentBuilder for AmountDisplay {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Row(None, 4, Align::Center, vec![
            Icon::Warning.build(32, COLORS.status.warning), // Warning Icon
            Text::primary(ctx, self.0, TextSize::md()) // Warning Text
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}