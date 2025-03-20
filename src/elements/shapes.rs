use rust_on_rails::prelude::*;
use crate::{ZERO, Stack};
use crate::layout::Align;
use crate::elements::text::{Text, TextStyle};

// pub struct Circle(pub u32, pub Color);

// impl Circle {
//     pub fn new(size: u32, color: &'static str) -> Self {
//         Circle(size, Color::from_hex(color, 255))
//     }
// }

// impl ComponentBuilder for Circle {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         Shape(ShapeType::Ellipse(0, (self.0, self.0)), self.1).build_children(ctx, max_size)
//     }
// }

// pub struct Outline(pub Color);

// impl ComponentBuilder for Outline {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let s = max_size.x.min(max_size.y);
//         let o = (s as f32 * 0.06).round() as u32;
//         vec![Box::new(Shape(ShapeType::Ellipse(o, (s, s)), self.0))]
//     }
// }

pub struct Circle(Box<dyn ComponentBuilder>);

impl Circle {
    pub fn new(s: u32, c: Color) -> Self {
        Self(Box::new(Shape(ShapeType::Ellipse(0, (s, s)), c)))
    }

    pub fn outlined(s: u32, c: Color, oc: Color) -> Self {
        let o = (s as f32 * 0.06).round() as u32;
        Self(Box::new(
            Stack!(Align::Center, 
                Shape(ShapeType::Ellipse(0, (s, s)), c),
                Shape(ShapeType::Ellipse(o, (s + o, s + o)), oc)
            )
        ))
    }
}

impl ComponentBuilder for Circle {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)
    }
}

pub struct RoundedRectangle(Box<dyn ComponentBuilder>);

impl RoundedRectangle {
    pub fn new(s: (u32, u32), r: u32, c: Color) -> Self {
        Self(Box::new(Shape(ShapeType::RoundedRectangle(0, s, r), c)))
    }

    pub fn outlined(s: (u32, u32), r: u32, c: Color, o: u32, oc: Color) -> Self {
        Self(Box::new(
            Stack!(Align::Center, 
                Shape(ShapeType::RoundedRectangle(0, s, r), c),
                Shape(ShapeType::RoundedRectangle(o, s, r), oc)
            )
        ))
    }
}

impl ComponentBuilder for RoundedRectangle {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)
    }
}