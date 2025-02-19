// use rust_on_rails::prelude::*;
// use crate::theme;
// use crate::{Row, Column, Stack, Text, Alignment};
// use crate::components::abc::CircleIcon;

// #[derive(Clone)]
// pub enum MessageBubble {
//     You(&'static str),
//     ContactGroup(&'static str),
//     Rooms(&'static str)
// }

// impl ComponentBuilder for MessageBubble {
//     fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         ctx.include_assets(include_assets!("./resources")); // Move this to theme startup
//         let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap(); // GET TEXT FONT
//         let colors = theme::color::palette();

//         let (background, text_color, text, pad) = match self {
//             MessageBubble::You(a) 
//                 => (colors.brand.primary, colors.text.heading, *a, 24),
//             MessageBubble::ContactGroup(a) 
//                 => (colors.background.secondary, colors.text.primary, *a, 24),
//             MessageBubble::Rooms(a) 
//                 => (colors.background.primary, colors.text.primary, *a, 0)
//         };

//         let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
//         let mut content = Text::new(text, text_color, 16, font.clone()).build(ctx, bound);

//         let (width, height) = (content.size(ctx).x + pad, content.size(ctx).y + pad);

//         content.1 = Rect::new((width - content.size(ctx).x) / 2, (width - content.size(ctx).x) / 2, max_size.x, max_size.y);

//         vec![
//             Box::new(Shape(ShapeType::Rectangle(width, height), background, None).build(ctx, bound)),
//             Box::new(content)
//         ]
//     }

//     fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct TextMessage(pub MessageType, pub Vec<&'static str>);

// pub enum MessageType {
//     You,
//     Contact,
//     Group,
//     Rooms,
// }

// impl ComponentBuilder for TextMessage {
//     fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
//         let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
//         let colors = theme::color::palette();

//         let boxed_messages = self.1.iter()
//             .map(|msg| {
//                 let bubble = match self.0 {
//                     MessageType::You => MessageBubble::You(msg),
//                     MessageType::Contact | MessageType::Group => MessageBubble::ContactGroup(msg),
//                     MessageType::Rooms => MessageBubble::Rooms(msg),
//                 };

//                 Box::new(bubble) as Box<dyn ComponentBuilder>
//             }).collect();
    
//         match self.0 {
//             MessageType::You => {
//                 Column::new(8, Alignment::Right,
//                     Column(boxed_messages, 8, Alignment::Left),
//                     Text::new("11:48 AM", colors.text.secondary, 14, font.clone())
//                 ).build_children(ctx, max_size)
//             },
//             MessageType::Contact => {
//                 Column!(8, Alignment::Left,
//                     Column(boxed_messages, 8, Alignment::Left),
//                     Text::new("11:48 AM", colors.text.secondary, 14, font.clone())
//                 ).build_children(ctx, max_size)
//             },
//             MessageType::Group => {
//                 Row!(8, Alignment::Bottom,
//                     CircleIcon::Photo("profile.png", 24),
//                     Column!(8, Alignment::Left,
//                         Column(boxed_messages, 8, Alignment::Left),
//                         Row!(4, Alignment::Bottom,
//                             Text::new("Ella Couch", colors.text.secondary, 14, font.clone()),
//                             Text::new("·", colors.text.secondary, 14, font.clone()),
//                             Text::new("11:48 AM", colors.text.secondary, 14, font.clone())
//                         )
//                     )
//                 ).build_children(ctx, max_size)
//             },
//             MessageType::Rooms => {
//                 Row!(8, Alignment::Top,
//                     CircleIcon::Photo("profile.png", 24),
//                     Column!(8, Alignment::Left,
//                         Row!(4, Alignment::Bottom,
//                             Text::new("Ella Couch", colors.text.heading, 16, heading.clone()),
//                             Text::new("·", colors.text.secondary, 14, font.clone()),
//                             Text::new("11:48 AM", colors.text.secondary, 14, font.clone())
//                         ),
//                         Column(boxed_messages, 8, Alignment::Left)
//                     )
//                 ).build_children(ctx, max_size)
//             }
//         }

//     }

//     fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
// }