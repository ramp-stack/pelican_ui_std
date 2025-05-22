use rust_on_rails::prelude::*;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::RoundedRectangle;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::utils::Timestamp;
use crate::PelicanUI;

/// Represents the style or source of a message in the UI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MessageType {
    /// A message sent by the current user (you).
    You,
    /// A message received from a direct contact.
    Contact,
    /// A message in a group chat.
    Group,
    /// A message in a public room.
    Rooms,
}

/// Metadata for a user.
#[derive(Debug, Clone)]
pub struct Profile {
    /// The display name of the user.
    pub name: &'static str,
    /// The decentralized identity (did) of the user.
    pub nym: &'static str,
    /// A short description written by the user.
    pub about: &'static str,
    /// Avatar image or content representing the user.
    pub avatar: AvatarContent,
}

/// A UI component representing a chat message, including avatar and content.
#[derive(Debug, Component)]
pub struct Message(Row, Option<Avatar>, MessageContent);

impl OnEvent for Message {}

impl Message {
    /// Constructs a new [`Message`] component with appropriate layout based on style.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The UI context.
    /// * `style` - The [`MessageType`] indicating sender type and alignment.
    /// * `messages` - A list of message strings to display as bubbles.
    /// * `sender` - The [`Profile`] of the message sender.
    /// * `time` - A timestamp string to display with the message.
    ///
    /// # Behavior
    ///
    /// - `MessageType::You`: Right-aligned, no avatar.
    /// - `MessageType::Rooms`: Left-aligned, avatar shown.
    /// - Other types: Right-aligned, avatar shown.
    pub fn new(
        ctx: &mut Context,
        style: MessageType,
        message: String,
        sender: (String, AvatarContent), // name, biography, identifier, avatar
        time: Timestamp,
    ) -> Self {
        let name = Box::leak(sender.0.into_boxed_str());
        let message = Box::leak(message.into_boxed_str());

        let (offset, avatar) = match style {
            MessageType::You => (Offset::End, false),
            MessageType::Rooms => (Offset::Start, true),
            _ => (Offset::End, true),
        };

        Message (
            Row::new(8.0, offset, Size::Fit, Padding::default()),
            avatar.then(|| Avatar::new(ctx, sender.1, None, false, 24.0, None)),
            MessageContent::new(ctx, style, message, name, time)
        )
    }
}

#[derive(Debug, Component)]
struct MessageContent(Column, Option<MessageData>, MessageBubbles, Option<MessageData>);
impl OnEvent for MessageContent {}

impl MessageContent {
    fn new(
        ctx: &mut Context,
        style: MessageType,
        message: &'static str,
        name: &'static str,
        time: Timestamp,
    ) -> Self {
        let name = match style {
            MessageType::You => "You",
            _ => name,
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
            Column::new(8.0, offset, Size::custom(|widths: Vec<(f32, f32)>| (widths[1].0, f32::MAX)), Padding::default()),
            top, MessageBubbles::new(ctx, message, style), bottom
        )
    }
}

#[derive(Debug, Component)]
struct MessageData(Row, Text, Option<Text>, Text);
impl OnEvent for MessageData {}

impl MessageData {
    fn new(
        ctx: &mut Context,
        style: MessageType,
        name: &'static str,
        time: Timestamp,
    ) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size;
        let (title_style, title_size, divider) = match style {
            MessageType::Rooms => (TextStyle::Heading, text_size.h5, false),
            _ => (TextStyle::Secondary, text_size.sm, true),
        };
        MessageData(
            Row::new(4.0, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, name, title_style, title_size, Align::Left),
            divider.then(|| Text::new(ctx, "·", TextStyle::Secondary, text_size.sm, Align::Left)),
            Text::new(ctx, time.friendly(), TextStyle::Secondary, text_size.sm, Align::Left),
        )
    }
}


#[derive(Debug, Component)]
struct MessageBubbles(Column, Vec<MessageBubble>);
impl OnEvent for MessageBubbles {}

impl MessageBubbles {
    fn new(
        ctx: &mut Context,
        // messages: Vec<&'static str>,
        message: &'static str,
        style: MessageType,
    ) -> Self {
        // let messages = messages.iter().map(|m| MessageBubble::new(ctx, m, style)).collect();
        MessageBubbles(Column::new(8.0, Offset::Start, Size::Fit, Padding::default()), vec![MessageBubble::new(ctx, message, style)])
    }
}

#[derive(Debug, Component)]
struct MessageBubble(Stack, RoundedRectangle, Text);
impl OnEvent for MessageBubble {}

impl MessageBubble {
    fn new(
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

        let (hp, vp) = (12.0, 12.0);
        let max_w = 300.0-(hp*2.0);
        let background = RoundedRectangle::new(0.0, 16.0, bg_color);
        let mut content = Text::new(ctx, message, text_style, text_size, Align::Left);
        content.text().width = Some(max_w);
        let layout = Stack(
            Offset::Center, Offset::Center, 
            Size::custom(move |widths: Vec<(f32, f32)>| {
                let size = (widths[1].1+(hp*2.)).min(max_w+(hp*2.));
                (size, size)
            }),
            Size::custom(move |heights: Vec<(f32, f32)>| (heights[1].0+vp, heights[1].1+vp)), 
            Padding::default()
        );

        MessageBubble(layout, background, content)
    }
}