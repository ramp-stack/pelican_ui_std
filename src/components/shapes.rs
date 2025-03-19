use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Image as RailsImage;
use crate::{ZERO, Align, Stack};

pub struct Outline(pub u32, pub Color);

impl Outline {
    pub fn new(size: u32, color: &'static str) -> Self {
        Outline(size, Color::from_hex(color, 255))
    }
}

impl ComponentBuilder for Outline {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let size = (self.0 as f32 * 0.3).round() as u32;
        vec![Box::new(Shape(ShapeType::Ellipse(size, (self.0 + size, self.0 + size)), self.2))]
    }
}

pub struct Circle(pub u32, pub Color);

impl Circle {
    pub fn new(size: u32, color: &'static str) -> Self {
        Circle(size, Color::from_hex(color, 255))
    }
}

impl ComponentBuilder for Circle {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        vec![Box::new(Shape(ShapeType::Ellipse(0, (self.0, self.0)), self.1))]
    }
}


pub enum Image {
    Circle(RailsImage, Option<Outline>, u32),
}

// Image::Circle(*img, outline, self.3),

impl ComponentBuilder for Image {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];

        let (img, outline, size) = match self {
            Image::Circle(i, o, s) => (i.clone(), *o, *s),
        };

        // children.push(Box::new(Image(ShapeType::Ellipse(0, (size, size)), ctx.add_image(img)))); // Image 

        if let Some(o) = outline {
            children.push(o.new(size)) // Outline
        }

        Stack(ZERO, Align::Center, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}
