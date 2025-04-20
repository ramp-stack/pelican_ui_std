use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::RoundedRectangle;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageType {
    You,
    Contact,
    Group,
    Rooms,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: &'static str,
    pub nym: &'static str,
    pub about: &'static str,
    pub avatar: AvatarContent,
}

#[derive(Debug, Component)]
pub struct Message(Row, Option<Avatar>);
 impl OnEvent for Message {}

impl Message {
    pub fn new(
        ctx: &mut Context,
        style: MessageType,
        _messages: Vec<&'static str>,
        sender: Profile,
        _time: &'static str,
    ) -> Self {
        let (offset, avatar) = match style {
            MessageType::You => (Offset::End, false),
            MessageType::Rooms => (Offset::Start, true),
            _ => (Offset::End, true),
        };

        Message (
            Row(8.0, offset, Size::Fit, Padding::default()),
            avatar.then(|| Avatar::new(ctx, sender.avatar.clone(), None, false, 24.0)),
            // MessageContent::new(ctx, style, messages, sender, time)
        )
    }
}

#[derive(Debug, Component)]
pub struct MessageContent(Column, Option<MessageData>, MessageBubbles, Option<MessageData>);
 impl OnEvent for MessageContent {}

impl MessageContent {
    pub fn new(
        ctx: &mut Context,
        style: MessageType,
        messages: Vec<&'static str>,
        sender: Profile,
        time: &'static str,
    ) -> Self {
        let name = match style {
            MessageType::You => "You",
            _ => sender.name,
        };
        let data = MessageData::new(ctx, style, name, time);

        let offset = match style {
            MessageType::You => Offset::End,
            _ => Offset::Start,
        };

        let (top, bottom) = match style {
            MessageType::Rooms => (Some(data), None),
            _ => (None, Some(data))
        };

        MessageContent(
            Column(8.0, offset, Size::Fill(0.0, 300.0), Padding::default()),
            top, MessageBubbles::new(ctx, messages, style), bottom
        )
    }
}

#[derive(Debug, Component)]
pub struct MessageData(Row, BasicText, Option<BasicText>, BasicText);
 impl OnEvent for MessageData {}

impl MessageData {
    pub fn new(
        ctx: &mut Context,
        style: MessageType,
        name: &'static str,
        time: &'static str,
    ) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size;
        let (title_style, title_size, divider) = match style {
            MessageType::Rooms => (TextStyle::Heading, text_size.h5, true),
            _ => (TextStyle::Secondary, text_size.sm, false),
        };
        MessageData(
            Row(4.0, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, name, title_style, title_size, TextAlign::Left),
            divider.then(|| Text::new(ctx, "·", TextStyle::Secondary, text_size.sm, TextAlign::Left)),
            Text::new(ctx, time, TextStyle::Secondary, text_size.sm, TextAlign::Left),
        )
    }
}


#[derive(Debug, Component)]
pub struct MessageBubbles(Column, Vec<MessageBubble>);
 impl OnEvent for MessageBubbles {}

impl MessageBubbles {
    pub fn new(
        ctx: &mut Context,
        messages: Vec<&'static str>,
        style: MessageType,
    ) -> Self {
        let messages = messages.iter().map(|m| MessageBubble::new(ctx, m, style)).collect();
        MessageBubbles(Column::center(8.0), messages)
    }
}

#[derive(Debug, Component)]
pub struct MessageBubble(Stack, RoundedRectangle, BasicText);
 impl OnEvent for MessageBubble {}

impl MessageBubble {
    pub fn new(
        ctx: &mut Context,
        message: &'static str,
        style: MessageType,
    ) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, text_size) = (theme.colors, theme.fonts.size.md);
        let (bg_color, text_style) = match style {
            MessageType::You => (colors.brand.primary, TextStyle::White),
            MessageType::Rooms => (colors.background.primary, TextStyle::White),
            MessageType::Group => (colors.background.secondary, TextStyle::Primary),
            MessageType::Contact => (colors.background.secondary, TextStyle::Primary),
        };
        let background = RoundedRectangle::new(0.0, 16.0, bg_color);
        let content = Text::new(ctx, message, text_style, text_size, TextAlign::Left);
        let layout = Stack(
            Offset::Center, Offset::Center, 
            Size::custom(|widths: Vec<(f32, f32)>| (widths[1].0, 200.0)), 
            Size::custom(|heights: Vec<(f32, f32)>| heights[1]), 
            Padding::default()
        );

        MessageBubble(layout, background, content)
    }
}