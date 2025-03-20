use rust_on_rails::prelude::Text as BasicText;
use rust_on_rails::prelude::*;

// pub struct Text(pub BasicText);

// impl Text {
//     pub fn new(text: &'static str, color: Color, size: u32, font: resources::Font) -> Self {
//         Self(BasicText(text, color, None, size, (size as f32*1.25) as u32, font))
//     }

//     pub fn heading(ctx: &mut Context, text: &'static str, size: u32) -> Self {
//         Self::new(text, crate::COLORS.text.heading, size, ctx.theme.fonts.heading)
//     }

//     pub fn primary(ctx: &mut Context, text: &'static str, size: u32) -> Self {
//         Self::new(text, crate::COLORS.text.primary, size, ctx.theme.fonts.text)
//     }

//     pub fn primary_white(ctx: &mut Context, text: &'static str, size: u32) -> Self {
//         Self::new(text, crate::COLORS.text.heading, size, ctx.theme.fonts.text)
//     }

//     pub fn secondary(ctx: &mut Context, text: &'static str, size: u32) -> Self {
//         Self::new(text, crate::COLORS.text.secondary, size, ctx.theme.fonts.text)
//     }

//     pub fn error(ctx: &mut Context, text: &'static str, size: u32) -> Self {
//         Self::new(text, crate::COLORS.status.danger, size, ctx.theme.fonts.text)
//     }

//     pub fn label(ctx: &mut Context, text: &'static str, size: u32) -> Self {
//         Self::new(text, crate::COLORS.text.heading, size, ctx.theme.fonts.label)
//     }
// }

// impl ComponentBuilder for Text {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         self.0.build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }