use rust_on_rails::prelude::*;
use crate::layout::Stack;
// use crate::{ZERO, Stack};
// use crate::layout::Align;
// use crate::elements::text::{Text, TextStyle};

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

pub struct Circle;

impl Circle {
    pub fn new(s: u32, color: Color) -> Shape {
        Shape(ShapeType::Ellipse(0, (s, s)), color)
    }
}

#[derive(Debug, Component)]
pub struct RoundedRectangle(Stack, Shape);

impl RoundedRectangle {
    pub fn new(s: u32, r: u32, c: Color) -> Self {
        RoundedRectangle(
            Stack::fill(),
            Shape(ShapeType::RoundedRectangle(s, (0, 0), r), c)
        )
    }

    pub fn shape(&mut self) -> &mut Shape { &mut self.1 }
}

impl Events for RoundedRectangle {
    fn on_resize(&mut self, _ctx: &mut Context, size: (u32, u32)) {
        if let Shape(ShapeType::RoundedRectangle(_, (w, h), _), _) = &mut self.1 {
            *w = size.0;
            *h = size.1;
        }
    }
}

pub struct Outline;

impl Outline {
    pub fn circle(s: u32, color: Color) -> Shape {
        Shape(ShapeType::Ellipse((s as f32 * 0.06).round() as u32, (s, s)), color)
    }
}



// #[derive(Clone)]
// pub struct RoundedRectangle(pub Color, pub Color, pub u32, pub u32);
// // background color, stroke color, stroke width, corner radius

// impl Component for RoundedRectangle {
//     fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
//         Container(Offset::default(), Size::Fill, vec![
//             Box::new(Shape(ShapeType::RoundedRectangle(0, self.3), self.0)),
//             Box::new(Shape(ShapeType::RoundedRectangle(self.2, self.3), self.1))
//         ])
//     }
// }


// pub struct RoundedRectangle(Box<dyn ComponentBuilder>);

// impl RoundedRectangle {
//     pub fn new(s: (u32, u32), r: u32, c: Color) -> Self {
//         Self(Box::new(Shape(ShapeType::RoundedRectangle(0, s, r), c)))
//     }

//     pub fn outlined(s: (u32, u32), r: u32, c: Color, o: u32, oc: Color) -> Self {
//         Self(Box::new(
//             Stack!(Align::Center, 
//                 Shape(ShapeType::RoundedRectangle(0, s, r), c),
//                 Shape(ShapeType::RoundedRectangle(o, s, r), oc)
//             )
//         ))
//     }
// }

// impl ComponentBuilder for RoundedRectangle {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         self.0.build_children(ctx, max_size)
//     }
// }
