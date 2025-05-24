use rust_on_rails::prelude::*;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{ExpandableText, Text, TextStyle};
use crate::components::button::IconButton;
use crate::events::{KeyboardActiveEvent, SetActiveInput};
use crate::layout::{EitherOr, Padding, Column, Stack, Offset, Size, Row, Bin};
use crate::PelicanUI;

use std::sync::mpsc::{self, Receiver};
/// A labeled text input with optional help or error messages and an optional icon button.
#[derive(Debug, Component)]
pub struct TextInput(Column, Option<Text>, InputField, Option<Text>, Option<Text>);

impl TextInput {
    /// Creates a new [`TextInput`] component.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The UI context.
    /// * `value` - An optional initial value for the input field.
    /// * `label` - An optional label displayed above the input field.
    /// * `placeholder` - Placeholder text displayed inside the input field.
    /// * `help_text` - Optional help text shown below the input.
    /// * `icon_button` - An optional icon button with label and callback function.
    ///
    /// If `help_text` is provided, it is shown by default. Use [`set_error`] to override it with an error.
    pub fn new(
        ctx: &mut Context,
        value: Option<&str>,
        label: Option<&str>,
        placeholder: &str,
        help_text: Option<&str>,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        TextInput(
            Column::new(16.0, Offset::Start, Size::fill(), Padding::default()),
            label.map(|text| Text::new(ctx, text, TextStyle::Heading, font_size.h5, Align::Left)),
            InputField::new(ctx, value, placeholder, icon_button),
            help_text.map(|t| Text::new(ctx, t, TextStyle::Secondary, font_size.sm, Align::Left)),
            None
        )
    }

    /// Sets an error message to be displayed below the input field,
    /// replacing any existing help text.
    pub fn set_error(&mut self, ctx: &mut Context, error: &str) {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        self.4 = Some(Text::new(ctx, error, TextStyle::Error, font_size, Align::Left));
        self.3 = None;
    }

    /// Sets help text to be displayed below the input field,
    /// removing any currently displayed error message.
    pub fn set_help(&mut self, ctx: &mut Context, help: &str) {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        self.3 = Some(Text::new(ctx, help, TextStyle::Secondary, font_size, Align::Left));
        self.4 = None;
    }

    /// Returns a mutable reference to the input field's error flag.
    pub fn error(&mut self) -> &mut bool {
        self.2.error()
    }

    /// Returns the input field's string value.
    pub fn get_value(&mut self) -> String {
        self.2.input().replace('\u{200C}', "")
    }

    /// Sets the input's text to the provided string. 
    pub fn set_value(&mut self, new: String) {
        *self.2.input() = new;
    }
}

impl OnEvent for TextInput {
    /// Updates the error state during the UI's tick event.
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            *self.2.error() = self.4.is_some();
        }
        true
    }
}


#[derive(Debug, Component)]
struct InputField(Stack, OutlinedRectangle, InputContent, #[skip] InputState, #[skip] bool);

impl InputField {
    pub fn new(
        ctx: &mut Context,
        value: Option<&str>,
        placeholder: &str,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let (background, outline) = InputState::Default.get_color(ctx);
        let content = InputContent::new(ctx, value, placeholder, icon_button);
        let background = OutlinedRectangle::new(background, outline, 8.0, 1.0);

        InputField(Stack(
            Offset::Start, Offset::Start, Size::fill(),
            Size::custom(|heights: Vec<(f32, f32)>| (heights[1].0.max(48.0), heights[1].1.max(48.0))),
            Padding::default()
        ), background, content, InputState::Default, false)
    }

    pub fn error(&mut self) -> &mut bool { &mut self.4 }
    pub fn input(&mut self) -> &mut String { &mut self.2.text().text().spans[0].text }
}

impl OnEvent for InputField {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            if let Some(c) = self.2.text().cursor().as_mut() { c.display(self.3 == InputState::Focus) }
            self.3 = match self.3 {
                InputState::Default if self.4 => Some(InputState::Error),
                InputState::Error if !self.4 => Some(InputState::Default),
                _ => None
            }.unwrap_or(self.3);

            let (background, outline) = self.3.get_color(ctx);
            *self.1.background() = background;
            *self.1.outline() = outline;
            *self.2.focus() = self.3 == InputState::Focus;
        } else if let Some(SetActiveInput(s)) = event.downcast_ref::<SetActiveInput>() {
            *self.input() = s.to_string();
        } else if let Some(KeyboardActiveEvent(enabled)) = event.downcast_ref::<KeyboardActiveEvent>() {
            if !enabled && self.3 == InputState::Focus {
                if self.4 { self.3 = InputState::Error } else { self.3 = InputState::Default }
            }
        } else if let Some(event) = event.downcast_ref::<MouseEvent>() {
            self.3 = match self.3 {
                InputState::Default => {
                    match event {
                        MouseEvent{state: MouseState::Pressed, position: Some(_)} => {ctx.trigger_event(KeyboardActiveEvent(true)); Some(InputState::Focus)},
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
                        MouseEvent{state: MouseState::Pressed, position: None} if self.4 && !crate::config::IS_MOBILE => Some(InputState::Error),
                        MouseEvent{state: MouseState::Pressed, position: None} if !crate::config::IS_MOBILE => Some(InputState::Default),
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
        } else if let Some(KeyboardEvent{state: KeyboardState::Pressed, key}) = event.downcast_ref() {
            if self.3 == InputState::Focus {
                if let Some((i, _)) = self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::GetIndex) {
                    let mut new_text = self.2.text().text().spans[0].text.clone();
                    // ALL THIS SHOULD BE MOVED TO AN EDITABLE TEXT COMPONENT 
                    match key {
                        Key::Named(NamedKey::Enter) => {
                            self.2.text().text().spans[0].text.insert_str(i as usize, "\n");
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveNewline);
                        },
                        Key::Named(NamedKey::Space) => {
                            self.2.text().text().spans[0].text.insert_str(i as usize, " ");
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveRight);
                        },
                        Key::Named(NamedKey::Delete | NamedKey::Backspace) => {
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveLeft);
                            self.2.text().text().spans[0].text = remove_char(new_text, (i as usize).saturating_sub(1));
                        },
                        Key::Character(c) => {
                            self.2.text().text().spans[0].text.insert_str(i as usize , c);
                            self.2.text().text().cursor_action(ctx.as_canvas(), CursorAction::MoveRight);
                        },
                        _ => {}
                    };
                }
            }
        }
        true
    }
}


fn remove_char(text: String, index: usize) -> String {
    let mut chars: Vec<char> = text.chars().collect();
    match chars.len() == 1 {
        true => {chars.clear();},
        false if index >= chars.len() => {chars.pop();},
        false => {chars.remove(index);},
    }

    chars.into_iter().collect()
}


/// `SubmitCallback` is triggered when the optional icon button within the text input is pressed.
/// It has access to a mutable reference to the [`Context`] and the current input value as a `&mut &str`.
pub type SubmitCallback = Box<dyn FnMut(&mut Context, &mut String)>;

#[derive(Component)]
struct InputContent(
    Row, Bin<Stack, EitherOr<ExpandableText, ExpandableText>>, Option<IconButton>,
    #[skip] bool, #[skip] Option<(Receiver<u8>, SubmitCallback)>
);

impl InputContent {
    fn new(
        ctx: &mut Context,
        value: Option<&str>,
        placeholder: &str,
        icon_button: Option<(&'static str, impl FnMut(&mut Context, &mut String) + 'static)>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        let (icon_button, callback) = icon_button.map(|(icon, on_click)| {
            let (sender, receiver) = mpsc::channel();
            (
                Some(IconButton::input(ctx, icon, move |_| {sender.send(0).unwrap();})),
                Some((receiver, Box::new(on_click) as SubmitCallback)),
            )
        }).unwrap_or((None, None));

        InputContent(
            Row::new(0.0, Offset::End, Size::Fit, Padding(16.0, 8.0, 8.0, 8.0)),
            Bin(
                Stack(Offset::default(), Offset::End, Size::fill(), Size::Fit, Padding(8.0, 8.0, 8.0, 8.0)),
                EitherOr::new(
                    ExpandableText::new_with_cursor(ctx, value.unwrap_or(""), TextStyle::Primary, font_size, Align::Left),
                    ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size, Align::Left)
                )
            ),
            icon_button,
            false,
            callback,
        )
    }

    fn text(&mut self) -> &mut ExpandableText { self.1.inner().left() }
    fn focus(&mut self) -> &mut bool {&mut self.3}
}

impl OnEvent for InputContent {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            if let Some((receiver, on_submit)) = self.4.as_mut() {
                if receiver.try_recv().is_ok() {
                    on_submit(ctx, &mut self.1.inner().left().0.text().spans[0].text)
                }
            }

            let input = !self.1.inner().left().0.text().spans[0].text.is_empty();
            self.1.inner().display_left(input || self.3)
        }
        true
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
