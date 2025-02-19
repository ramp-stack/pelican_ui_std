pub mod prelude;
pub mod components;
pub mod theme;
pub mod layout;

use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::layout::{Column, Row, Stack};

#[macro_export]
macro_rules! Column {
    ($s:expr, $p:expr, $a:expr, false, $($i:expr),*) => {{
        Column {
            children: vec![$(Box::new($i) as Box<dyn ComponentBuilder>),*],
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};
    
    ($s:expr, $p:expr, $a:expr, true, $children:expr) => {{
        Column {
            children: $children,
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};
}

#[macro_export]
macro_rules! Stack {
    ($p:expr, $a:expr, $(($comp:expr, $num:expr)),*) => {{
        Stack {
            children: vec![$( (Box::new($comp) as Box<dyn ComponentBuilder>, $num) ),*],
            align: $a,
            padding: $p,
        }
    }};
}

#[macro_export]
macro_rules! Row {
    ($s:expr, $p:expr, $a:expr, false, $($i:expr),*) => {{
        Row {
            children: vec![$(Box::new($i) as Box<dyn ComponentBuilder>),*],
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};
    
    ($s:expr, $p:expr, $a:expr, true, $children:expr) => {{
        Row {
            children: $children,
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};
}


pub struct Text(pub BasicText);
impl Text {
    pub fn new(text: &'static str, color: &'static str, size: u32, font: Handle) -> Self {
        Text(BasicText(text, color, size, (size as f32*1.25) as u32, font))
    }
}

impl ComponentBuilder for Text {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Padding(pub u32, pub u32);

impl ComponentBuilder for Padding {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Shape(ShapeType::Rectangle(self.0, self.1), theme::color::palette().background.primary, None).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct ConstrainedBox(pub u32, pub Box<dyn ComponentBuilder>);

#[macro_export]
macro_rules! ConstrainedBox {
    ($x:expr, $i:expr) => {
        ConstrainedBox($x, Box::new($i) as Box<dyn ComponentBuilder>)
    };
}

impl ComponentBuilder for ConstrainedBox {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.1.build_children(ctx, Vec2::new(self.0, max_size.y))
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}