use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::shapes::RoundedRectangle;
use crate::elements::text::{Text,TextStyle, ExpandableText};
use crate::components::button::IconButton;
use crate::layout::{Padding, Column, Stack, Offset, Size, Row};
use crate::PelicanUI;

#[derive(Debug, Clone, Component)]
pub struct TextInput(Column, Opt<BasicText>, InputField, Opt<BasicText>);
impl Events for TextInput {}

impl TextInput {
    pub fn new(
        ctx: &mut Context,
        label: Option<&'static str>,
        placeholder: &'static str,
        help_text: Option<&'static str>,
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let subtext = if let Some(err) = error {
            Some(Text::new(ctx, err, TextStyle::Error, font_size.sm))
        } else if let Some(help) = help_text {
            Some(Text::new(ctx, help, TextStyle::Secondary, font_size.sm))
        } else { None };

        let state = match error.is_some() {
            true => InputState::Error,
            false => InputState::Default,
        }; // Error status goes away on hover, on focus (shouldn't)

        TextInput(
            Column(16, Offset::Start, Size::Fit, Padding::default()),
            label.map(|text| Text::new(ctx, text, TextStyle::Heading, font_size.h5)),
            InputField::new(ctx, state, placeholder, icon_button), 
            subtext
        )
    }
}

#[derive(Clone, Debug, Component)]
struct InputField(Stack, InputBackground, InputContent, #[skip] InputState, #[skip] bool);

impl InputField {
    pub fn new(
        ctx: &mut Context,
        state: InputState,
        placeholder: &'static str,
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>,
    ) -> Self {
        let (background, outline) = state.get_color(ctx);
        let content = InputContent::new(ctx, placeholder, icon_button);
        let width = Size::Fill(content.size(ctx).min_width(), MaxSize::MAX);
        let height = content.size(ctx).min_height().0;
        let background = InputBackground::new(background, outline, width, height);

        InputField(Stack::center(), background, content, state, false)
    }

    pub fn set_error(&mut self, error: bool) {
        self.4 = error;
        if let Some(state) = self.3.toggle_error(self.4) {self.set_state(state);}
    } 

    fn set_state(&mut self, state: InputState) {
        let (background, outline) = state.get_color(ctx);
        *self.1.background() = background;
        *self.1.outline() = outline;
        *self.2.focus() = state == InputState::Focus;
    }
}

impl Events for InputField {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        if let Some(state) = self.3.handle(event, self.4) {self.set_state(state);}
        true
    }
    
    fn on_keyboard(&mut self, _ctx: &mut Context, event: KeyboardEvent) -> bool {
        if self.3 == InputState::Focus {
            let t = self.2.input();
            *t = match event.text.as_str() {
                "\u{7f}" => if t.len()>0 {(&t[0..t.len() - 1]).to_string()} else {String::new()}, // delete char
                c => t.clone().to_owned()+c // add character
            };
        }
        false
    }
}

#[derive(Clone, Debug, Component)]
struct InputBackground(Stack, RoundedRectangle, RoundedRectangle);
impl Events for InputBackground {}

impl InputBackground {
    fn new(bg: Color, oc: Color, width: Size, height: u32) -> Self {
        InputBackground(//TODO: Change RoundedRectangle to no longer accept width or height but always expand, Change size::Fit to Static(height)
            Stack(Offset::Center, Offset::Center, width, Size::Fit, Padding::default()),
            RoundedRectangle::new(0, None, Some(height), 8, bg),
            RoundedRectangle::new(1, None, Some(height), 8, oc)
        )
    }

    fn background(&mut self) -> &mut Color {self.1.shape().color()}
    fn outline(&mut self) -> &mut Color {self.2.shape().color()}
}

#[derive(Clone, Debug, Component)]
struct InputContent(Row, Eth<ExpandableText, ExpandableText>, Option<IconButton>, #[skip] bool);

impl InputContent {
    fn new(
        ctx: &mut Context, 
        placeholder: &'static str, 
        icon_button: Option<IconButton>
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;

        InputContent(
            Row(16, Offset::Center, Size::Fit, Padding(16, 8, 8, 8)),
            Eth::new(
                ExpandableText::new(ctx, "", TextStyle::Primary, font_size),
                ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size),
            ),
            icon_button
        )
    }

    fn input(&mut self) -> &mut String { &mut self.1.right().value() }
    fn focus(&mut self) -> &mut bool {&mut self.3}
}

impl Events for InputContent {
    fn on_tick(&mut self, ctx: &mut Context) {
        self.1.displa_yleft(!self.1.left().value().is_empty() || self.3)
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
    fn toggle_error(&mut self, error: bool) -> Option<Self> {
        let state = match self {
            InputState::Default if error => Some(InputState::Error),
            InputState::Error if !error => Some(InputState::Default),
            _ => None
        };
        if let Some(state) = state { *self = state; }
        state
    }

    fn handle(&mut self, event: MouseEvent, error: bool) -> Option<Self> {
        let state = match self {
            InputState::Default => {
                match mouse {
                    MouseEvent{state: MouseState::Moved, position: Some(_)} => Some(InputState::Hover),
                    _ => None
                }
            },
            InputState::Hover => {
                match event.state {
                    MouseEvent{state: MouseState::Pressed, position: Some(_)} => Some(InputState::Focus),
                    MouseEvent{state: MouseState::Moved, position: None} if error => Some(InputState::Error),
                    MouseEvent{state: MouseState::Moved, position: None} => Some(InputState::Default),
                    _ => None
                }
            },
            InputState::Focus => {
                match event.state {
                    MouseEvent{state: MouseState::Pressed, position: None} if error => Some(InputState::Error),
                    MouseEvent{state: MouseState::Pressed, position: None} => Some(InputState::Default),
                    _ => None
                }
            },
            InputState::Error => {
                match event.state {
                    MouseEvent{state: MouseState::Pressed, position: Some(_)} => Some(InputState::Focus),
                    MouseEvent{state: MouseState::Moved, position: Some(_)} => Some(InputState::Hover),
                    _ => None
                }
            }
            _ => None
        };
        if let Some(state) = state { *self = state; }
        state
    }

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
