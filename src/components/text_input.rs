use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text,TextStyle, ExpandableText};
use crate::components::button::IconButton;
use crate::layout::{EitherOr, Padding, Column, Stack, Offset, Size, Row};
use crate::PelicanUI;

#[derive(Debug, Clone, Component)]
pub struct TextInput(Column, Option<BasicText>, InputField, EitherOr<BasicText, BasicText>);

impl TextInput {
    pub fn new(
        ctx: &mut Context,
        label: Option<&'static str>,
        placeholder: &'static str,
        help_text: Option<&'static str>,
        icon_button: Option<(&'static str, fn(&mut Context, &mut String) -> ())>,
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

#[derive(Clone, Debug, Component)]
struct InputField(Stack, OutlinedRectangle, InputContent, #[skip] InputState, #[skip] bool);

impl InputField {
    pub fn new(
        ctx: &mut Context,
        placeholder: &'static str,
        icon_button: Option<(&'static str, fn(&mut Context, &mut String) -> ())>,
    ) -> Self {
        let (background, outline) = InputState::Default.get_color(ctx);
        let content = InputContent::new(ctx, placeholder, icon_button);
        let width = Size::Fill(content.size(ctx).min_width(), MaxSize::MAX);
        let height = Size::Static(content.size(ctx).min_height().0);

        let background = OutlinedRectangle::new(background, outline, width, height, 8, 1);

        InputField(
            Stack(Offset::End, Offset::End, Size::default(), Size::default(), Padding::default()), 
            background, content, InputState::Default, false
        )
    }

    pub fn error(&mut self) -> &mut bool { &mut self.4 }
}

impl Events for InputField {
    fn on_resize(&mut self, ctx: &mut Context, _size: (u32, u32)) {
        *self.1.height() = Size::Static(self.2.size(ctx).min_height().0);
    }

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

#[derive(Clone, Debug, Component)]
struct InputContent(Row, InputText, Option<IconButton>, #[skip] Option<fn(&mut Context, &mut String) -> ()>);
impl Events for InputContent {}

impl InputContent {
    fn new(
        ctx: &mut Context, 
        placeholder: &'static str, 
        icon_button: Option<(&'static str, fn(&mut Context, &mut String) -> ())>
    ) -> Self {
        let (icon_button, on_click) = icon_button.map(|(icon, on_click)| (
            Some(IconButton::input(ctx, icon, |_| {println!("LOOSER!");})), 
            Some(on_click))).unwrap_or((None, None)
        );

        InputContent(
            Row(16, Offset::End, Size::Fit, Padding(8, 8, 8, 8)),
            InputText::new(ctx, placeholder),
            icon_button,
            on_click
        )
    }

    fn input(&mut self) -> &mut String { self.1.input() }
    fn focus(&mut self) -> &mut bool { self.1.focus() }
}

#[derive(Clone, Debug, Component)]
struct InputText(Row, EitherOr<ExpandableText, ExpandableText>, #[skip] bool);

impl InputText {
    fn new(ctx: &mut Context, placeholder: &'static str,) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;

        InputText(
            Row(16, Offset::End, Size::Fit, Padding(8, 6, 8, 6)),
            EitherOr::new(
                ExpandableText::new(ctx, "", TextStyle::Primary, font_size),
                ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size),
            ), false
        )
    }

    fn focus(&mut self) -> &mut bool { &mut self.2 }
    fn input(&mut self) -> &mut String { self.1.left().value() }
}

impl Events for InputText {
    fn on_tick(&mut self, _ctx: &mut Context) {
        let input = !self.1.left().value().is_empty();
        self.1.display_left(input || self.2)
    }

    // fn on_mouse(&mut self, ctx: &mut Context) -> bool {
    //     if let Some(button) = self.2.as_mut() {
    //         button.
    //     }
    //     true
    // }
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
