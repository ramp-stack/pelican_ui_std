use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align };
use crate::theme::fonts::{Text, TextSize};
use crate::components::UserIcon;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    You,
    Contact,
    Group,
    Rooms,
}

#[derive(Debug)]
pub struct Profile {
    name: &'static str,
    nym: &'static str,
    description: &'static str,
    avatar: AvatarContent,
}

#[derive(Debug, Component)]
pub struct Message(Row, Avatar, MessageContent);
impl Events for Message {}

impl Message {
    pub fn new(
        ctx: &mut Context,
        style: MessageType,
        messages: Vec<&'static str>,
        sender: Profile,
        time: &'static str,
    ) -> Self {
        let (name, offset, avatar) = match style {
            MessageType::You => ("You", Offset::End, None),
            MessageType::Rooms => (sender.name, Offset::Start, Some(sender.avatar)),
            MessageType::Group => (sender.name, Offset::End, Some(sender.avatar)),
            MessageType::Contact => (sender.name, Offset::End, Some(sender.avatar))
        };

        Message (
            Row(8, offset, Size::Fit),
            avatar.map(|data| Avatar::new(ctx, data, None, false, 24)),
            MessageContent::new(ctx, style, messages, sender, time)
        )
    }
}

#[derive(Debug, Component)]
pub struct MessageContent(Column, Option<MessageData>, MessageBubbles, Option<MessageData>);
impl Events for MessageContent {}

impl MessageContent {
    pub fn new(
        ctx: &mut Context,
        style: MessageType,
        messages: Vec<&'static str>,
        sender: Profile,
        time: &'static str,
    ) -> Self {

        let offset = match style {
            MessageType::You => Offset::End,
            _ => Offset::Start,
        };

        let (top, bottom) = match style {
            MessageType::Rooms => (Some(MessageData::new(ctx, style, sender.name, time)), None),
            _ => (None, Some(MessageData::new(ctx, style, sender.name, time)))
        }

        MessageContent(
            Column(8, offset, Size::Fill(0, 300), Padding::default()),
            top, MessageBubbles::new(ctx, style, messages), bottom
        )
    }
}

#[derive(Debug, Component)]
pub struct MessageData(Row, BasicText, Option<BasicText>, BasicText);
impl Events for MessageData {}

impl MessageData {
    pub fn new(
        ctx: &mut Context,
        style: MessageStyle,
        name: &'static str,
        time: &'static str,
    ) -> Self {
        let (title_style, title_size, divider) = match style {
            MessageType::Rooms => (TextStyle::Heading, text_size.h5, true),
            _ => (TextStyle::Secondary, text_size.sm, false),
        }
        MessageData(
            Row(4, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, name, title_style, title_size),
            divider.then(|| Text::new(ctx, "·", TextStyle::Secondary, text_size.sm)),
            Text::new(ctx, time, TextStyle::Secondary, text_size.sm),
        )
    }
}


#[derive(Debug, Component)]
pub struct MessageBubbles(Column, Vec<MessageBubble>);
impl Events for MessageBubbles {}

impl MessageBubbles {
    pub fn new(
        ctx: &mut Context,
        messages: Vec<&'static str>,
        style: MessageStyle,
    ) -> Self {
        let messages = message.iter().map(|m| MessageBubble::new(ctx, m, style));
        MessageBubbles(Column::default(8), messages)
    }
}

#[derive(Debug, Component)]
pub struct MessageBubble(Stack, Rectangle, BasicText);
impl Events for MessageBubble {}

impl MessageBubble {
    pub fn new(
        ctx: &mut Context,
        message: &'static str,
        style: MessageStyle,
    ) -> Self {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        let (bg_color, msg_color) = match style {
            MessageType::You => (colors.background.primary, colors.outline.secondary),
            MessageType::Rooms => (colors.background.primary, colors.outline.secondary),
            MessageType::Group => (colors.background.primary, colors.outline.secondary),
            MessageType::Content => (colors.background.primary, colors.outline.secondary),
        };
        let background = OutlinedRectangle::new(bg, oc, 16, 1);
        let content = CardContent::new(ctx, avatar, title, subtitle, description);
        let layout = Stack(
            Offset::Center, Offset::Center, 
            Size::custom(|widths: Vec<(u32, u32)>| (widths[1].0, u32::MAX)), 
            Size::custom(|heights: Vec<(u32, u32)>| heights[1]), 
            Padding::default()
        );

        MessageBubbles(
            Stack::center()
            Column(8, Offset::End, Size::Static(min_width), Padding::default()),
            messages,
        )
    }
}

pub struct MessageBubble(&'static str, MessageType);

impl ComponentBuilder for MessageBubble {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

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

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}




pub struct DateTime(date: &'static str, time: &'static str);