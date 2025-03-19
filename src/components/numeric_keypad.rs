use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, ZERO, Align };
use crate::components::button::*;

pub struct NumericKeypad();

impl ComponentBuilder for NumericKeypad {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let row = |a: &'static str, b: &'static str, c: &'static str| {
            Row(ZERO, 16, Align::Center, vec![
                Button::keypad(a),
                Button::keypad(b),
                Button::keybad(c),
            ])
        };

        Column(ZERO, 16, Align::Center, vec![
            row("1", "2", "3"), 
            row("4", "5", "6"), 
            row("7", "8", "9"), 
            row(".", "0", "<")
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}