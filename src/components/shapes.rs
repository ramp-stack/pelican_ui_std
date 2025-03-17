use rust_on_rails::prelude::*;
use crate::{ZERO, Align};

pub struct Circle(u32, &'static str, Option<(u32, &'static str)>); // size, color, outline

impl ComponentBuilder for Circle {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn Drawable>> = vec![];
        let bound = Rect::new(0, 0, max_size.x, max_size.y);

        children.push(Box::new(Shape(ShapeType::Ellipse(0, (self.0, self.0)), self.1, 255).build(ctx, bound))); // Primary 

        if let Some((thickness, outline)) = self.2 {
            children.push(Box::new(Shape(ShapeType::Ellipse(thickness, (self.0, self.0)), outline, 255).build(ctx, bound))) // Outline
        }

        children
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}
