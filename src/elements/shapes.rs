use rust_on_rails::prelude::*;
use crate::layout::{Offset, Padding, Size, Stack};

pub struct Circle;

impl Circle {
    pub fn new(s: u32, color: Color) -> Shape {
        Shape(ShapeType::Ellipse(0, (s, s)), color)
    }
}

#[derive(Debug, Component)]
pub struct OutlinedRectangle(Stack, RoundedRectangle, RoundedRectangle);
impl Events for OutlinedRectangle {}

impl OutlinedRectangle {
    pub fn new(bg: Color, oc: Color, radius: u32, stroke: u32) -> Self {
        OutlinedRectangle(
            Stack::default(),
            RoundedRectangle::new(0, radius, bg),
            RoundedRectangle::new(stroke, radius, oc)
        )
    }

    pub fn background(&mut self) -> &mut Color {self.1.shape().color()}
    pub fn outline(&mut self) -> &mut Color {self.2.shape().color()}
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
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(ResizeEvent(size)) = event.downcast_ref() {
            if let Shape(ShapeType::RoundedRectangle(_, (w, h), _), _) = &mut self.1 {
                *w = size.0;
                *h = size.1;
            }
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct Rectangle(Stack, Shape);

impl Rectangle {
    pub fn new(c: Color) -> Self {
        Rectangle(
            Stack(Offset::default(), Offset::default(), Self::get_size(None), Self::get_size(None), Padding::default()),
            Shape(ShapeType::Rectangle(0, (0, 0)), c)
        )
    }

    pub fn shape(&mut self) -> &mut Shape { &mut self.1 }

    fn get_size(s: Option<u32>) -> Size {
        s.map(|s| Size::Static(s)).unwrap_or(Size::Fill(0, u32::MAX))
    }
}

impl Events for Rectangle {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(ResizeEvent(size)) = event.downcast_ref() {
            if let Shape(ShapeType::Rectangle(_, (w, h)), _) = &mut self.1 {
                *w = size.0;
                *h = size.1;
            }
        }
        true
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
