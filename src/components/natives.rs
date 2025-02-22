use rust_on_rails::prelude::*;
use crate::{
    Expand,
    ConstrainedBox,
    Row, 
    Padding,
    Column, 
    Stack, 
    Text, 
    COLORS
};
use crate::layout::Align;

pub struct MobileKeyboard();

impl ComponentBuilder for MobileKeyboard {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let page_index = 1;
        let caps = false;

        let (top_row, middle_row, bottom_row) = generate_rows(page_index, caps);

        let paginator_key = |i: u32| {
            if i == 0 { "•--" } 
            else if i == 1 { "-•-" }
            else { "--•" }
        };
    

        Stack!(Vec2::new(0, 0), Align::Top,
            (Shape(ShapeType::Rectangle(393, 300), COLORS.background.secondary, None), Vec2::new(0, 0)),
            (ConstrainedBox!(393,
                Column!(0, Vec2::new(0, 0), Align::Center, false,
                    Row!(16, Vec2::new(12, 12), Align::Left, false,
                        (icon(36), false),
                        (icon(36), false),
                        (icon(36), false),
                        (icon(36), false),
                        (icon(36), false),
                        (Expand(false, 1, COLORS.background.secondary), true),
                        (icon(36), false)
                    ),
                    Expand(false, 1, COLORS.outline.secondary),
                    Padding(Vec2::new(5, 5), COLORS.background.secondary),
                    Column!(12, Vec2::new(0, 0), Align::Center, false,
                        Row!(6, Vec2::new(4, 0), Align::Left, true, true, top_row),
                        Row!(6, Vec2::new(12, 0), Align::Left, true, true, middle_row),
                        Row!(6, Vec2::new(4, 0), Align::Left, false, 
                            (Key("capslock"), false),
                            (Row!(6, Vec2::new(8, 0), Align::Center, true, true, bottom_row), true),
                            (Key("backspace"), false)
                        ),
                        Row!(6, Vec2::new(4, 0), Align::Left, false, 
                            (Key(paginator_key(page_index)), false),
                            (Key("space"), true),
                            (Key("return"), false)
                        ),
                        Padding(Vec2::new(5, 12), COLORS.background.secondary)
                    )
                )
            ), Vec2::new(0, 0))
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Key(&'static str);

impl ComponentBuilder for Key {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
    
        let mut paginator_key = |main_index: usize| {
            Box::new(
                Row!(2, Vec2::new(0, 0), Align::Left, true, vec![
                    if main_index == 0 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) },
                    if main_index == 1 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) },
                    if main_index == 2 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }
                ])
            ) as Box<dyn ComponentBuilder>
        };
        
        let (content, offset, max_width) = match self.0 {
            "•--" => (paginator_key(0), 6, Some(92)),
            "-•-" => (paginator_key(1), 6, Some(92)),
            "--•" => (paginator_key(2), 6, Some(92)),
            "space" => (sm_key_text(self.0, ctx), 11, None),
            "return" => (sm_key_text(self.0, ctx), 11, Some(92)),
            _ => match self.0.len() <= 3 {
                true => (lg_key_text(self.0, ctx), 4, None),
                false =>(Box::new(icon(32)) as Box<dyn ComponentBuilder>, 5, Some(44))
            }
        };
    
        Stack!(Vec2::new(0, 0), Align::Top, true, vec![
            (key_background(max_width), Vec2::new(0, 0)),
            (content, Vec2::new(0, offset))
        ]).build_children(ctx, max_size)
    }
    

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}


// Helper Functions

fn generate_rows(page: u32, caps: bool) -> (
    Vec<Box<dyn ComponentBuilder>>, 
    Vec<Box<dyn ComponentBuilder>>, 
    Vec<Box<dyn ComponentBuilder>>
) {
    (
        str_to_key(top_row(page), caps), 
        str_to_key(mid_row(page), caps), 
        str_to_key(bot_row(page), caps)
    )
}

fn top_row(page: u32) -> Vec<&'static str> {
    match page {
        0 => vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
        1 => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
        _ => vec!["[", "]", "{", "}", "(", ")", "<", ">", "+", "="]
    }
}

fn mid_row(page: u32) -> Vec<&'static str> {
    match page {
        0 => vec!["a", "s", "d", "f", "g", "h", "j", "k", "l"],
        1 => vec!["-", "/", ":", ";", "#", "%", "$", "&", "@", "\""],
        _ => vec!["_", "\\", "|", "~", "^", "*", "€", "£", "¥", "•"]
    }
}

fn bot_row(page: u32) -> Vec<&'static str> {
    match page {
        0 => vec!["z", "x", "c", "v", "b", "n", "m"],
        1 => vec![".", ",", "?", "!", "'"],
        _ => vec![".", ",", "?", "!", "'"]
    }
}

pub fn str_to_key(keys: Vec<&'static str>, caps: bool) -> Vec<Box<dyn ComponentBuilder>> {
    keys.iter()
        .map(|key| {
            let key = if caps { key.to_string().to_uppercase() } else { key.to_string() };
            Box::new(Key(Box::leak(key.into_boxed_str()))) as Box<dyn ComponentBuilder>
        }).collect()
}


fn key_background(max: Option<u32>) -> Box<dyn ComponentBuilder> {
    if let Some(a) = max {
        Box::new(Shape(ShapeType::Rectangle(a, 42), COLORS.outline.secondary, None))
    } else {
        Box::new(Expand(false, 42, COLORS.outline.secondary))
    }
}

fn lg_key_text(c: &'static str, ctx: &mut ComponentContext) -> Box<dyn ComponentBuilder> {
    let regular_font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
    Box::new(Text::new(c, COLORS.text.heading, 24, regular_font.clone())) as Box<dyn ComponentBuilder>
}

fn dk_key_text(c: &'static str, ctx: &mut ComponentContext) -> Box<dyn ComponentBuilder> {
    let regular_font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
    Box::new(Text::new(c, COLORS.text.secondary, 24, regular_font.clone())) as Box<dyn ComponentBuilder>
}

fn sm_key_text(c: &'static str, ctx: &mut ComponentContext) -> Box<dyn ComponentBuilder> {
    let regular_font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
    Box::new(Text::new(c, COLORS.text.heading, 16, regular_font.clone())) as Box<dyn ComponentBuilder>
}

fn icon(s: u32) -> Shape {
    Shape(ShapeType::Rectangle(s, s), "ffffff", None)
}