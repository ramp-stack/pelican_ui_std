use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align };
use crate::theme::fonts::{Text, TextSize};
use crate::components::UserIcon;

#[derive(Clone)]
pub enum MessageBubble {
    You(&'static str),
    ContactGroup(&'static str),
    Rooms(&'static str)
}

impl ComponentBuilder for MessageBubble {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); // Move this to theme startup
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap(); // GET TEXT FONT

        let (background, text_color, text, pad) = match self {
            MessageBubble::You(a) 
                => (COLORS.brand.primary, COLORS.text.heading, *a, 24),
            MessageBubble::ContactGroup(a) 
                => (COLORS.background.secondary, COLORS.text.primary, *a, 24),
            MessageBubble::Rooms(a) 
                => (COLORS.background.primary, COLORS.text.primary, *a, 0)
        };

        let bound = Rect::new(0, 0, max_size.x, max_size.y);
        let mut content = Text::new(text, text_color, 16, font.clone()).build(ctx, bound);

        let (width, height) = (content.size(ctx).x + pad, content.size(ctx).y + pad);

        content.1 = Rect::new((width - content.size(ctx).x) / 2, (width - content.size(ctx).x) / 2, max_size.x, max_size.y);

        vec![
            Box::new(Shape(ShapeType::Rectangle(width, height), background, None).build(ctx, bound)),
            Box::new(content)
        ]
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}


#[derive(PartialEq)]
pub enum MessageType {
    You,
    Contact,
    Group,
    Rooms,
}

pub struct TextMessage(pub MessageType, pub Vec<&'static str>);

impl ComponentBuilder for TextMessage {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let (sender, align) = if self.0 == MessageType::You { ("You", Align::Right)} else { ("Ella Couch", Align::Left) };

        let mut row: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut col: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];

        let boxed_messages: Vec<(Box<dyn ComponentBuilder>, bool)> = self.1.iter()
            .map(|msg| {
                let bubble = match self.0 {
                    MessageType::You => MessageBubble::You(msg),
                    MessageType::Contact | MessageType::Group => MessageBubble::ContactGroup(msg),
                    MessageType::Rooms => MessageBubble::Rooms(msg),
                };

                (Child!(bubble), false)
            }).collect();

        // Add profile picture.
        if self.0 == MessageType::Rooms || self.0 == MessageType::Group {
            row.push(( Child!(UserIcon("profile.png", 24, None)), false ));
        }

        // Data at top if room message.
        if self.0 == MessageType::Rooms {
            col.push((
                Child!(Row {
                    spacing: 4, padding: ZERO, align: Align::Bottom,
                    children: vec![
                        (Child!(Text::heading(ctx, sender, TextSize::h5())), false),
                        (Child!(Text::secondary(ctx, "11:45 PM", TextSize::sm())), false)
                    ]
                }),
                false
            ));
        } 

        // Add message bubbles.
        col.push((
            Child!(Column { spacing: 8, padding: ZERO, align, children: boxed_messages }),
            false
        ));
        
        // Data at bottom if not rooms.
        if self.0 != MessageType::Rooms {
            col.push((
                Child!(Row {
                    spacing: 4, padding: ZERO, align: Align::Bottom,
                    children: vec![
                        (Child!(Text::secondary(ctx, sender, TextSize::sm())), false),
                        (Child!(Text::secondary(ctx, "·", TextSize::sm())), false),
                        (Child!(Text::secondary(ctx, "11:45 PM", TextSize::sm())), false)
                    ]
                }),
                false
            ));
        }
        
        // Add column to row
        row.push((
            Child!(Column { spacing: 8, padding: ZERO, align, children: col }),
            false
        ));

        // Build row
        ConstrainedBox!(300,
            Row { spacing: 8, padding: ZERO, align: Align::Top, children: row }
        ).build_children(ctx, max_size)

    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}