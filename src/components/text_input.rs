use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text,TextStyle, ExpandableText};
use crate::components::button::IconButton;
use crate::layout::{EitherOr, Padding, Column, Stack, Offset, Size, Row, Bin};
use crate::PelicanUI;

use std::sync::mpsc::{self, Receiver};

#[derive(Debug, Component)]
pub struct TextInput(Column, Option<BasicText>, InputField, EitherOr<BasicText, BasicText>);

impl TextInput {
    pub fn new(
        ctx: &mut Context,
        label: Option<&'static str>,
        placeholder: &'static str,
        help_text: Option<&'static str>,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        TextInput(
            Column(16, Offset::Start, Size::Fit, Padding::default()),
            label.map(|text| Text::new(ctx, text, TextStyle::Heading, font_size.h5)),
            InputField::new(ctx, placeholder, icon_button),
            EitherOr::new(
                Text::new(ctx, help_text.unwrap_or("NONE"), TextStyle::Secondary, font_size.sm),
                Text::new(ctx, "", TextStyle::Error, font_size.sm)
            )
        )
    }
     
    pub fn error(&mut self) -> &mut String { self.3.right().value() }
}

impl Events for TextInput {
    fn on_tick(&mut self, _ctx: &mut Context) {
        let error = !self.3.right().value().is_empty();
        self.3.display_left(!error);
        *self.2.error() = error;
    }
}

#[derive(Debug, Component)]
struct InputField(Stack, OutlinedRectangle, InputContent, #[skip] InputState, #[skip] bool);

impl InputField {
    pub fn new(
        ctx: &mut Context,
        placeholder: &'static str,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let (background, outline) = InputState::Default.get_color(ctx);
        let content = InputContent::new(ctx, placeholder, icon_button);
        let background = OutlinedRectangle::new(background, outline, 8, 1);

        InputField(Stack(
            Offset::Center, Offset::End, Size::fill(),
                Size::custom(|heights: Vec<(u32, u32)>| heights[1]),
            Padding::default()
        ), background, content, InputState::Default, false)
    }

    pub fn error(&mut self) -> &mut bool { &mut self.4 }
}

impl Events for InputField {
    fn on_tick(&mut self, ctx: &mut Context) {
        self.3 = match self.3 {
            InputState::Default if self.4 => Some(InputState::Error),
            InputState::Error if !self.4 => Some(InputState::Default),
            _ => None
        }.unwrap_or(self.3);

        let (background, outline) = self.3.get_color(ctx);
        *self.1.background() = background;
        *self.1.outline() = outline;
        *self.2.focus() = self.3 == InputState::Focus;
    }

    fn on_mouse(&mut self, _ctx: &mut Context, event: MouseEvent) -> bool {
        self.3 = match self.3 {
            InputState::Default => {
                match event {
                    MouseEvent{state: MouseState::Moved, position: Some(_)} => Some(InputState::Hover),
                    _ => None
                }
            },
            InputState::Hover => {
                match event {
                    MouseEvent{state: MouseState::Pressed, position: Some(_)} => Some(InputState::Focus),
                    MouseEvent{state: MouseState::Moved, position: None} if self.4 => Some(InputState::Error),
                    MouseEvent{state: MouseState::Moved, position: None} => Some(InputState::Default),
                    _ => None
                }
            },
            InputState::Focus => {
                match event {
                    MouseEvent{state: MouseState::Pressed, position: None} if self.4 => Some(InputState::Error),
                    MouseEvent{state: MouseState::Pressed, position: None} => Some(InputState::Default),
                    _ => None
                }
            },
            InputState::Error => {
                match event {
                    MouseEvent{state: MouseState::Pressed, position: Some(_)} => Some(InputState::Focus),
                    MouseEvent{state: MouseState::Moved, position: Some(_)} => Some(InputState::Hover),
                    _ => None
                }
            }
        }.unwrap_or(self.3);
        true
    }

    fn on_keyboard(&mut self, _ctx: &mut Context, event: KeyboardEvent) -> bool {
        if self.3 == InputState::Focus && event.state == KeyboardState::Pressed {
            let t = self.2.input();
            *t = match event.key.as_str() {
                "\u{7f}" => if t.len()>0 {(&t[0..t.len() - 1]).to_string()} else {String::new()}, // delete char
                c => t.clone().to_owned()+c // add character
            };
        }
        false
    }
}

#[derive(Component)]
struct InputContent(
    Row, Bin<Stack, EitherOr<ExpandableText, ExpandableText>>, Option<IconButton>,
    #[skip] bool, #[skip] Option<(Receiver<u8>, Box<dyn FnMut(&mut Context, &mut String)>)>
);

impl InputContent {
    pub fn new(
        ctx: &mut Context,
        placeholder: &'static str,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        let (icon_button, callback) = icon_button.map(|(icon, on_click)| {
            let (sender, receiver) = mpsc::channel();
            (
                Some(IconButton::input(ctx, icon, move |_| {sender.send(0).unwrap();})),
                Some((receiver, Box::new(on_click) as Box<dyn FnMut(&mut Context, &mut String)>)),
            )
        }).unwrap_or((None, None));

        InputContent(
            Row(16, Offset::End, Size::Fit, Padding(16, 8, 8, 8)),
            Bin(
                Stack(Offset::default(), Offset::End, Size::Fit, Size::Fit, Padding(8, 6, 8, 6)),
                EitherOr::new(
                    ExpandableText::new(ctx, "hello", TextStyle::Primary, font_size),
                    ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size)
                )
            ),
            icon_button,
            false,
            callback,
        )
    }

    pub fn input(&mut self) -> &mut String { self.1.inner().left().value() }
    pub fn focus(&mut self) -> &mut bool {&mut self.3}
}

impl Events for InputContent {
    fn on_tick(&mut self, ctx: &mut Context) {
        if let Some((receiver, on_submit)) = self.4.as_mut() {
            if receiver.try_recv().is_ok() {
                on_submit(ctx, self.1.inner().left().value())
            }
        }

        let input = !self.1.inner().left().value().is_empty();
        self.1.inner().display_left(input || self.3)
    }
}

impl std::fmt::Debug for InputContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InputContent(...)")
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum InputState {
    Default,
    Hover,
    Focus,
    Error
}

impl InputState {
    fn get_color(&self, ctx: &mut Context) -> (Color, Color) { // background, outline
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            InputState::Default => (colors.shades.transparent, colors.outline.secondary),
            InputState::Hover => (colors.background.secondary, colors.outline.secondary),
            InputState::Focus => (colors.shades.transparent, colors.outline.primary),
            InputState::Error => (colors.shades.transparent, colors.status.danger)
        }
    }
}
