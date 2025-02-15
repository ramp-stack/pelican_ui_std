pub mod prelude;
pub mod components;
pub mod theme;

use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;

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

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
        self.0.0 = ("T".to_string()+self.0.0).leak()
    }

    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }
}


pub struct Square(Shape);
impl Square {
    pub fn new(s: u32, c: &'static str, sw: Option<u16>) -> Self {
        Square(Shape(ShapeType::Rectangle(s, s), c, sw))
    }
}

impl ComponentBuilder for Square {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)//Pipe children
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}

    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Padding(pub u32, pub u32);

impl ComponentBuilder for Padding {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let colors = theme::color::palette();
        Shape(ShapeType::Rectangle(self.0, self.1), colors.background.primary, None).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }

    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }
}



#[derive(PartialEq, Eq, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
    Bottom,
    Top,
}

pub struct Column(pub Vec<Box<dyn ComponentBuilder>>, pub u32, pub Alignment);

#[macro_export]
macro_rules! Column {
    ($x:literal, $a:expr, $($i:expr),*) => {{
        Column(vec![
            $(Box::new($i) as Box<dyn ComponentBuilder>),*
        ], $x, $a)
    }}
}

impl ComponentBuilder for Column {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        let max_width = self.0.iter()
            .map(|builder| {
                let child = builder.build(ctx, bound);
                child.size(ctx).x
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0);

        self.0.iter().map(|builder| {
            let mut child = builder.build(ctx, bound);
            let height = child.size(ctx).y;
            let width = child.size(ctx).x;

            bound.h -= self.1 + height;
            bound.y += self.1 + height;
            
            match self.2 {
                Alignment::Right => {
                    let offset = max_width - width;
                    child.1.w -= offset;
                    child.1.x += offset;
                },
                Alignment::Center => {
                    let offset = (max_width - width) / 2;
                    child.1.w -= offset;
                    child.1.x += offset;
                }
                _ => {}
            }
            

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        for builder in &mut self.0 {
            let child = builder.build(ctx, bound);
            let size = child.size(ctx);
            if size.x > position.x && bound.y+size.y > position.y {
                builder.on_click(
                    ctx,
                    Vec2::new(max_size.x, bound.y+self.1),
                    Vec2::new(max_size.x, position.y-bound.y)
                );
                break;
            }
            if bound.y+size.y+self.1 > position.y {
                break;//Clicked to the side or inbetween objects
            }
            bound.h -= self.1 + size.y;
            bound.y += self.1 + size.y;
        }
    }

    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }
}

pub struct Stack(pub Vec<Box<dyn ComponentBuilder>>);

#[macro_export]
macro_rules! Stack {
    ($($i:expr),*) => {{
        Stack(vec![
            $(Box::new($i) as Box<dyn ComponentBuilder>),*
        ])
    }}
}

impl ComponentBuilder for Stack {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        let max_height = self.0.iter()
            .map(|builder| {
                let child = builder.build(ctx, bound);
                child.size(ctx).y
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0);
        
        let max_width = self.0.iter()
            .map(|builder| {
                let child = builder.build(ctx, bound);
                child.size(ctx).x
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0);

        self.0.iter().map(|builder| {
            let mut child = builder.build(ctx, bound);
            let width = child.size(ctx).x;
            let height = child.size(ctx).y;
            let y_offset = (max_height - height) / 2;
            let x_offset = (max_width - width) / 2;
            
            child.1.h -= y_offset;
            child.1.y += y_offset;

            child.1.w -= x_offset;
            child.1.x += x_offset;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Row(pub Vec<Box<dyn ComponentBuilder>>, pub u32, pub Alignment);

#[macro_export]
macro_rules! Row {
    ($x:literal, $a:expr, $($i:expr),*) => {{
        Row(vec![
            $(Box::new($i) as Box<dyn ComponentBuilder>),*
        ], $x, $a)
    }}
}

impl ComponentBuilder for Row {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);

        let max_height = self.0.iter()
            .map(|builder| {
                let child = builder.build(ctx, bound);
                child.size(ctx).y
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0);

        let max_width = self.0.iter()
            .map(|builder| {
                let child = builder.build(ctx, bound);
                child.size(ctx).x
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0);

        self.0.iter().map(|builder| {
            let mut child = builder.build(ctx, bound);
            let width = child.size(ctx).x;
            let height = child.size(ctx).y;

            bound.w -= self.1 + width;
            bound.x += self.1 + width;

            match self.2 {
                Alignment::Bottom => {
                    let offset = max_height - height;
                    child.1.h -= offset;
                    child.1.y += offset;
                },
                Alignment::Center => {
                    let offset = (max_height - height) / 2;
                    child.1.h -= offset;
                    child.1.y += offset;
                },
                _ => {}
            }

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}