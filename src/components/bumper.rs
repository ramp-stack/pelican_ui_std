use rust_on_rails::prelude::*;
use crate::{ Align, ConstrainedBox, Row, Child };
use crate::components::Button;

#[derive(Clone, Copy)]
pub enum Bumper {
    Single(Button),
    Double(ButtonRow),
    Input(TextInput)
}

impl ComponentBuilder for Bumper {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Column(ZERO, 16, Align::Center, vec![match self {
            Bumper::Single(button) => *button,
            Bumper::Double(row) => *row,
            Bumper::Input(input) => *input,
        }]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}