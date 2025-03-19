// use rust_on_rails::prelude::*;
// use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align, Expand };
// use crate::components::button::*;

// pub struct DesktopNav(Vec<(&'static str, &'static str)>, u16, bool);

// impl DesktopNav {
//     pub fn new(tabs: Vec<(&'static str, &'static str)>, default_i: u16, is_desktop: bool) -> Self {
//         Self(tabs, default_i, is_desktop)
//     }
// }

// impl ComponentBuilder for DesktopNav {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let image = ctx.load_image("images/logomark.png").unwrap(); // Default logomark

//         // Image(ShapeType::Rectangle(icon_size, 8), image),

//         let buttons: Vec<(Box<dyn ComponentBuilder>, bool)> = self.0.iter().enumerate().map(|(_index, (name, _))| {
//             // if index as u16 == self.1 { print!("selected") } else { print!{"not Selected"}}
//             (Child!(Button::ghost(*name, Size::Large, Width::Expand, None, Align::Left)), true)
//         }).collect();

//         ConstrainedBox!(300, Column {
//             padding: Vec2::new(16, 32), align: Align::Center, spacing: 32, children: vec![
//                 (Child!(Image(ShapeType::Rectangle(150, 24), image)), false),
//                 (Child!(Column{spacing: 8, padding: ZERO, align: Align::Center, children: buttons }), false),
//                 // Spacer
//                 (Child!(Button::ghost("My Profile", Size::Large, Width::Expand, None, Align::Left)), false)
//             ]
//         }).build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct MobileNav();

// impl ComponentBuilder for MobileNav {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let image = ctx.load_image("images/profile.png").unwrap(); // Default logomark

//         ConstrainedBox!(300, 
//             Row {spacing: 0, padding: Vec2::new(16, 32), align: Align::Center, children: vec![
//                 (Child!(Shape(ShapeType::Rectangle(32, 32), "ffffff", None)), false),
//                 (Child!(Expand(false, 1, COLORS.background.primary)), true),
//                 (Child!(Shape(ShapeType::Rectangle(32, 32), "ffffff", None)), false),
//                 (Child!(Expand(false, 1, COLORS.background.primary)), true),
//                 (Child!(Image(ShapeType::Circle(32 / 2), image.clone())), false)
//             ]}
//         ).build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }