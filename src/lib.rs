pub mod prelude;
pub mod components;
pub mod theme;
pub mod layout;

use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::layout::{Column, Row, Stack};
use once_cell::sync::Lazy;
use crate::theme::colors::ColorResources;

static COLORS: Lazy<ColorResources> = Lazy::new(|| ColorResources::default());

#[macro_export]
macro_rules! Column {
    ($s:expr, $p:expr, $a:expr, false, $(($i:expr, $b:expr)),*) => {{
        Column {
            children: vec![$((Box::new($i) as Box<dyn ComponentBuilder>, $b)),*],
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};

    ($s:expr, $p:expr, $a:expr, false, $($i:expr),*) => {{
        Column {
            children: vec![$((Box::new($i) as Box<dyn ComponentBuilder>, false)),*],
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};
    
    ($s:expr, $p:expr, $a:expr, true, $children:expr) => {{
        Column {
            children: $children
                .into_iter()
                .map(|child| (child as Box<dyn ComponentBuilder>, false))
                .collect(),
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

    ($p:expr, $a:expr, true, $children:expr) => {{
        Stack {
            children: $children,
            align: $a,
            padding: $p,
        }
    }};
}

#[macro_export]
macro_rules! Row {

    ($s:expr, $p:expr, $a:expr, false, $(($i:expr, $b:expr)),*) => {{
        Row {
            children: vec![$((Box::new($i) as Box<dyn ComponentBuilder>, $b)),*],
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};

    ($s:expr, $p:expr, $a:expr, false, $($i:expr),*) => {{
        Row {
            children: vec![$((Box::new($i) as Box<dyn ComponentBuilder>, false)),*],
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};

    
    ($s:expr, $p:expr, $a:expr, true, $children:expr) => {{
        Row {
            children: $children
                .into_iter()
                .map(|child| (child as Box<dyn ComponentBuilder>, false))
                .collect(),
            spacing: $s,
            align: $a,
            padding: $p,
        }
    }};

    ($s:expr, $p:expr, $a:expr, true, true, $children:expr) => {{
        Row {
            children: $children
                .into_iter()
                .map(|child| (child as Box<dyn ComponentBuilder>, true))
                .collect(),
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

pub struct Padding(pub Vec2, pub &'static str);

impl ComponentBuilder for Padding {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Shape(ShapeType::Rectangle(self.0.x, self.0.y), self.1, None).build_children(ctx, max_size)
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

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Expand(pub bool, pub u32, pub &'static str);

impl ComponentBuilder for Expand {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        // println!("Max size: {}, {}", max_size.x, self.1);
        match self.0 {
            true => Shape(ShapeType::Rectangle(self.1, max_size.y), self.2, None).build_children(ctx, max_size),
            false => Shape(ShapeType::Rectangle(max_size.x, self.1), self.2, None).build_children(ctx, max_size)
        }
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}