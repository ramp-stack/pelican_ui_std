use rust_on_rails::prelude::*;
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


#[derive(Clone)]
pub struct Circle(pub u32, pub Color);

impl Component for Circle {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        Container(Offset::default(), Size::Static(self.0, self.0), vec![
            Box::new(Shape(ShapeType::Ellipse(0), self.1))
        ])
    }
}

#[derive(Clone)]
pub struct RoundedRectangle(pub Color, pub Color, pub u32, pub u32);
// background color, stroke color, stroke width, corner radius

impl Component for RoundedRectangle {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        Container(Offset::default(), Size::Fill, vec![
            Box::new(Shape(ShapeType::RoundedRectangle(0, self.3), self.0)),
            Box::new(Shape(ShapeType::RoundedRectangle(self.2, self.3), self.1))
        ])
    }
}


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
