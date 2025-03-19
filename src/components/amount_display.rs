use rust_on_rails::prelude::*;
use crate::{ Child, Row, Column, COLORS, ZERO, Align, ConstrainedBox };
use crate::theme::fonts::{Text, TextSize};

pub struct AmountDisplay(pub &'static str, pub &'static str, pub Option<&'static str>); // USD BTC ERR

impl ComponentBuilder for AmountDisplay {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>>  = vec![];

        children.push(Text::heading(ctx, self.0, TextSize::h1())); // US Dollar

        if let Some(error) = self.2 {
            children.push(Row(ZERO, 8, Align::Left, vec![
                Icon::Error.build(32, COLORS.status.danger), // Error Icon
                Text::error(ctx, self.1.unwrap(), TextSize::lg()) // Error Text
            ]));
        } else {
            children.push(Text::secondary(ctx, self.1, TextSize::lg())) // Bitcoin 
        }

        Column(Vec2::new(16, 32), 32, Align::Center, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}