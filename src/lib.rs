pub mod prelude;
pub mod components;
pub mod theme;
pub mod layout;

use rust_on_rails::prelude::*;

use crate::layout::{Column, Row, Stack};
use once_cell::sync::Lazy;
use crate::theme::colors::ColorResources;

const ZERO: Vec2 = Vec2{x: 0, y:0};
static COLORS: Lazy<ColorResources> = Lazy::new(|| ColorResources::default());

#[macro_export]
macro_rules! Child {
    ($x:expr) => {{
        (Box::new($x) as Box<dyn ComponentBuilder>)
    }};
}

fn icon(ctx: &mut ComponentContext) -> Handle {
    ctx.load_image("images/profile.png").unwrap()
}

pub fn pelican_startup(ctx: &mut ComponentContext) {
    ctx.include_assets(include_assets!("./resources")); 
}

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Right,
    Center,
    Bottom,
    Top,
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