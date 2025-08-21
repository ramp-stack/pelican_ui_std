use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component, ShapeType, Shape, Color};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::layout::Stack;

#[derive(Debug, Component)]
pub struct OutlinedRectangle(Stack, RoundedRectangle, RoundedRectangle);

impl OnEvent for OutlinedRectangle {}

impl OutlinedRectangle {
    pub fn new(bg: Color, oc: Color, radius: f32, stroke: f32) -> Self {
        OutlinedRectangle(
            Stack::default(),
            RoundedRectangle::new(0.0, radius, bg),
            RoundedRectangle::new(stroke, radius, oc)
        )
    }

    pub fn background(&mut self) -> &mut Color {&mut self.1.shape().color}
    pub fn outline(&mut self) -> &mut Color {&mut self.2.shape().color}
    pub fn size(&self) -> (f32, f32) {self.2.0.shape.size()}
}

#[derive(Debug)]
pub struct RoundedRectangle(Shape);

impl RoundedRectangle {
    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }
    pub fn new(s: f32, r: f32, color: Color) -> Self {
        RoundedRectangle(Shape{shape: ShapeType::RoundedRectangle(s, (0.0, 0.0), r, 0.0), color})
    }
}

impl OnEvent for RoundedRectangle {}
impl Component for RoundedRectangle {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {vec![&mut self.0]}
    fn children(&self) -> Vec<&dyn Drawable> {vec![&self.0]}
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::fill()
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        if let ShapeType::RoundedRectangle(_, s, _, _) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}

#[derive(Debug)]
pub struct Rectangle(Shape);

impl Rectangle {
    pub fn new(color: Color) -> Self {
        Rectangle(Shape { shape: ShapeType::Rectangle(0.0, (0.0, 0.0), 0.0), color })
    }

    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }
}

impl OnEvent for Rectangle {}
impl Component for Rectangle {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![&mut self.0] }
    fn children(&self) -> Vec<&dyn Drawable> { vec![&self.0] }
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::fill()
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        if let ShapeType::Rectangle(_, s, _) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}


pub struct Outline;

impl Outline {
    pub fn circle(s: f32, color: Color) -> Shape {
        Shape { shape: ShapeType::Ellipse(s * 0.06, (s, s), 0.0), color }
    }
}

pub struct Circle;

impl Circle {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(s: f32, color: Color) -> Shape {
        Shape { shape: ShapeType::Ellipse(0.0, (s, s), 0.0), color }
    }
}
