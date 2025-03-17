// use rust_on_rails::prelude::*;
// use crate::theme::fonts::{Text, TextSize};
// use crate::{ Child, Row, Column, COLORS, ZERO, Align, Stack, Expand, Padding };

// pub struct NativeKeyboard();

// impl ComponentBuilder for NativeKeyboard {
//     fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let page_index = 1;
//         let caps = false;

//         let (top_row, middle_row, bottom_row) = generate_rows(page_index, caps);

//         let paginator_key = |i: u32| if i == 0 { "•--" }  else if i == 1 { "-•-" } else { "--•" };
    
//         Stack {padding: ZERO, align: Align::Top, children: vec![
//             (Child!(Shape(ShapeType::Rectangle(393, 300), COLORS.background.secondary, None)), ZERO),
//             (Child!(Column { padding: ZERO, spacing: 0, align: Align::Center, children: vec![
//                 (Child!(Row { padding: Vec2::new(12, 12), align: Align::Left, spacing: 16, children: vec![
//                     (icon(36), false),
//                     (icon(36), false),
//                     (icon(36), false),
//                     (icon(36), false),
//                     (icon(36), false),
//                     (Child!(Expand(false, 1, COLORS.background.secondary)), true),
//                     (icon(36), false)
//                 ]}), false),
//                 (Child!(Expand(false, 1, COLORS.outline.secondary)), false),
//                 (Child!(Padding(Vec2::new(5, 5), COLORS.background.secondary)), false),
//                 (Child!(Column {spacing: 12, padding: Vec2::new(5, 5), align: Align::Center, children: vec![
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: top_row}), true),
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: middle_row}), true),
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: vec![
//                         (Child!(Key("capslock")), false),
//                         (Child!(Row{spacing: 6, padding: Vec2::new(8, 0), align: Align::Center, children: bottom_row}), true),
//                         (Child!(Key("backspace")), false)
//                     ]}), false),
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: vec![
//                         (Child!(Key(paginator_key(page_index))), false),
//                         (Child!(Key("space")), true),
//                         (Child!(Key("return")), false)
//                     ]}), false),
//                 ]}), false)
//             ]}), ZERO)
//         ]}.build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct Key(&'static str);

// impl ComponentBuilder for Key {
//     fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

//         let lg_key_text = |c: &'static str, ctx: &mut ComponentContext| 
//             Child!(Text::primary_white(ctx, c, TextSize::xl()));

//         let dk_key_text = |c: &'static str, ctx: &mut ComponentContext| 
//             Child!(Text::secondary(ctx, c, TextSize::xl()));

//         let sm_key_text = |c: &'static str, ctx: &mut ComponentContext| 
//             Child!(Text::primary_white(ctx, c, TextSize::md()));
    
//         let mut paginator_key = |main_index: usize| {
//             Child!(Row {spacing: 2, padding: ZERO, align: Align::Left, children: vec![
//                 (if main_index == 0 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }, false),
//                 (if main_index == 1 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }, false),
//                 (if main_index == 2 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }, false)
//             ]})
//         };

//         let key_background = |max: Option<u32>| {
//             match max.is_some() {
//                 true => Child!(Shape(ShapeType::Rectangle(max.unwrap(), 42), COLORS.outline.secondary, None)),
//                 false => Child!(Expand(false, 42, COLORS.outline.secondary))
//             }
//         };
        
//         let (content, offset, max_width) = match self.0 {
//             "•--" => (paginator_key(0), 6, Some(92)),
//             "-•-" => (paginator_key(1), 6, Some(92)),
//             "--•" => (paginator_key(2), 6, Some(92)),
//             "space" => (sm_key_text(self.0, ctx), 11, None),
//             "return" => (sm_key_text(self.0, ctx), 11, Some(92)),
//             _ => match self.0.len() <= 3 {
//                 true => (lg_key_text(self.0, ctx), 4, None),
//                 false =>(icon(32), 5, Some(44))
//             }
//         };

//         Child!(Stack {padding: ZERO, align: Align::Top, children: vec![
//             (key_background(max_width), Vec2::new(0, 0)),
//             (content, Vec2::new(0, offset))
//         ]}).build_children(ctx, max_size)
//     }
    

//     fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
// }


// fn generate_rows(page: u32, caps: bool) -> (
//     Vec<(Box<dyn ComponentBuilder>, bool)>, 
//     Vec<(Box<dyn ComponentBuilder>, bool)>, 
//     Vec<(Box<dyn ComponentBuilder>, bool)>
// ) {
//     let top_row = match page {
//         0 => vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
//         1 => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
//         _ => vec!["[", "]", "{", "}", "(", ")", "<", ">", "+", "="]
//     };

//     let mid_row = match page {
//         0 => vec!["a", "s", "d", "f", "g", "h", "j", "k", "l"],
//         1 => vec!["-", "/", ":", ";", "#", "%", "$", "&", "@", "\""],
//         _ => vec!["_", "\\", "|", "~", "^", "*", "€", "£", "¥", "•"]
//     };

//     let bot_row = match page {
//         0 => vec!["z", "x", "c", "v", "b", "n", "m"],
//         1 => vec![".", ",", "?", "!", "'"],
//         _ => vec![".", ",", "?", "!", "'"]
//     };

//     (
//         str_to_key(top_row, caps), 
//         str_to_key(mid_row, caps), 
//         str_to_key(bot_row, caps)
//     )
// }

// pub fn str_to_key(keys: Vec<&'static str>, caps: bool) -> Vec<(Box<dyn ComponentBuilder>, bool)> {
//     keys.iter().map(|key| {
//         let key = if caps { key.to_string().to_uppercase() } else { key.to_string() };
//         (Child!(Key(Box::leak(key.into_boxed_str()))), true)
//     }).collect()
// }

// fn icon(s: u32) -> Box<dyn ComponentBuilder> {
//     Child!(Shape(ShapeType::Rectangle(s, s), "ffffff", None))
// }