use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align };
use crate::theme::fonts::{Text, TextSize};
use crate::components::UserIcon;

pub struct TextMessage(pub MessageType, pub Vec<&'static str>, Option<Profile>);

pub struct TextMessage {
    style: MessageType,
    messages: Vec<&'static str>,
    sender: Profile,
    time: DateTime,
}

impl ComponentBuilder for TextMessage {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut row: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut col: Vec<Box<dyn ComponentBuilder>> = vec![];

        let (sender, align) = if self.0 == MessageType::You { ("You", Align::Right)} else { (sender.name, Align::Left) };

        if self.style == MessageType::Rooms || self.0 == MessageType::Group {
            row.push(CircleIcon(sender.profile_photo, None, false, 24)); // Profile Photo
        }

        if self.0 == MessageType::Rooms {
            col.push(Row(ZERO, 4, Align::Bottom, vec![
                Text::heading(ctx, sender, TextSize::h5()), // Name
                Text::secondary(ctx, self.time, TextSize::sm()) // Time/Date
            ]));
        } 

        col.push(
            Column(ZERO, 8, align, self.1
                .iter()
                .map(|msg| {
                    MessageBubble(msg, self.style) // Message
                })
                .collect()
            )
        );

        if self.0 != MessageType::Rooms {
            col.push(Row(ZERO, 4, Align::Bottom, vec![
                Text::secondary(ctx, sender, TextSize::sm()), // Name
                Text::secondary(ctx, "·", TextSize::sm()),
                Text::secondary(ctx, self.time, TextSize::sm()) // Time/Date
            ]));
        } 

        row.push(Column(ZERO, 8, Align::Top, col));
        
        ConstrainedBox(300, Row(ZERO, 8, Align::Top, row)).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Profile {
    profile_photo: CircleIconData,
    name: &'static str,
    nym: &'static str,
}

pub struct MessageBubble(&'static str, MessageType);

impl ComponentBuilder for MessageBubble {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let (background, pad) = match self.1 {
            MessageType::You => (COLORS.brand.primary,  24),
            MessageType::Contact => (COLORS.background.secondary, 24),
            MessageType::Group => (COLORS.background.secondary, 24),
            MessageType::Rooms => (COLORS.background.primary, 0)
        };

        let bound = Rect::new(0, 0, max_size.x, max_size.y);
        let mut content = Text::new(text, text_color, 16, font.clone()).build(ctx, bound);
        let (width, height) = (content.size(ctx).x + pad, content.size(ctx).y + pad);

        Stack(ZERO, Align::Center, vec![
            RoundedRectangle(width, height, 8, background, None),
            Text::primary(ctx, self.0, TextSize::md())
        ]).build(ctx, max_size)
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
