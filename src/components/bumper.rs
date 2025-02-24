use rust_on_rails::prelude::*;
use crate::{ Align, ConstrainedBox, Row, Child };
use crate::components::Button;

#[derive(Clone, Copy)]
pub enum Bumper {
    Single(Button),
    Double(Button, Button),
    // Input(TextInput)
}

impl ComponentBuilder for Bumper {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let content: Vec<(Box<dyn ComponentBuilder>, bool)> = match self {
            Bumper::Single(button) => vec![(Child!(*button), true)],
            Bumper::Double(first, second) => vec![(Child!(*first), true), (Child!(*second), true)],
            // Bumper::Input(input) => vec![(Child!(input), true)]
        };

        ConstrainedBox!(300, Row { 
            spacing: 16, align: Align::Center, padding: Vec2::new(16, 8), 
            children: content
        }).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}