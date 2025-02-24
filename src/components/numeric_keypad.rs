use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, ZERO, Align };
use crate::components::button::*;

pub struct NumericKeypad();

impl ComponentBuilder for NumericKeypad {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let key = |a: &'static str| {
            (Child!(Button::ghost(a, Size::Large, Width::Expand, None, Align::Center)), true)
        };

        let row = |a: &'static str, b: &'static str, c: &'static str| {
            (Child!(Row { spacing: 16, padding: ZERO, align: Align::Center, children: vec![key(a), key(b), key(c)] }), true)
        };

        ConstrainedBox!(300, Column {
            spacing: 16, padding: ZERO, align: Align::Center, 
            children: vec![ row("1", "2", "3"), row("4", "5", "6"), row("7", "8", "9"), row(".", "0", "<") ]
        } ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}