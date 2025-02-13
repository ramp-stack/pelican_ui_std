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


pub struct Column(pub Vec<Box<dyn ComponentBuilder>>, pub u32);

#[macro_export]
macro_rules! Column {
    ($x:literal, $($i:expr),*) => {{
        Column(vec![
            $(Box::new($i) as Box<dyn ComponentBuilder>),*
        ], $x)
    }}
}

impl ComponentBuilder for Column {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        self.0.iter().map(|builder| {
            let child = builder.build(ctx, bound);
            let height = child.size(ctx).y;
            bound.h -= self.1 + height;
            bound.y += self.1 + height;
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


pub struct Row(pub Vec<Box<dyn ComponentBuilder>>, pub u32);

#[macro_export]
macro_rules! Row {
    ($x:literal, $($i:expr),*) => {{
        Row(vec![
            $(Box::new($i) as Box<dyn ComponentBuilder>),*
        ], $x)
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

        self.0.iter().map(|builder| {
            let mut child = builder.build(ctx, bound);
            let width = child.size(ctx).x;
            let height = child.size(ctx).y;
            let offset = (max_height - height) / 2;

            bound.w -= self.1 + width;
            bound.x += self.1 + width;
            child.1.h -= offset;
            child.1.y += offset;

            Box::new(child) as Box<dyn Drawable>
        }).collect()
    }

    fn on_click(&mut self, ctx: &mut ComponentContext, max_size: Vec2, position: Vec2) {
        // let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
        // for builder in &mut self.0 {
        //     let child = builder.build(ctx, bound);
        //     let size = child.size(ctx);
        //     if size.x > position.x && bound.y+size.y > position.y {
        //         builder.on_click(
        //             ctx,
        //             Vec2::new(max_size.x, bound.y+self.1),
        //             Vec2::new(max_size.x, position.y-bound.y)
        //         );
        //         break;
        //     }
        //     if bound.y+size.y+self.1 > position.y {
        //         break;//Clicked to the side or inbetween objects
        //     }
        //     bound.h -= self.1 + size.y;
        //     bound.y += self.1 + size.y;
        // }
    }

    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }
}




pub struct FText(Text, Handle);
impl FText {
    pub fn new(text: &'static str, color: &'static str, size: u32, font: Handle, ofont: Handle) -> Self {
        FText(Text::new(text, color, size, font), ofont)
    }
}

impl ComponentBuilder for FText {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)//Pipe children
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
        let ohandle = self.0.0.4.clone();
        self.0.0.4 = self.1.clone();
        self.1 = ohandle;
    }


    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }
}

pub struct FShape(Shape);
impl FShape {
    pub fn new(shape: ShapeType, color: &'static str, stroke_width: Option<u16>) -> Self {
        FShape(Shape(shape, color, stroke_width))
    }
}

impl ComponentBuilder for FShape {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)//Pipe children
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
        if self.0.1 == "ff0000" {self.0.1 = "00ff00"}
        else if self.0.1 == "00ff00" {self.0.1 = "0000ff"}
        else {self.0.1 = "ff0000"}
    }


    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {
    }
}
