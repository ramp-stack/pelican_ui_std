use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::shapes::{RoundedRectangle, Outline};
use crate::elements::text::{Text, TextStyle, ExpandableText};
use crate::components::icon_button::IconButton;
use crate::layout::{Padding, Stack, Offset, Size, Row};
use crate::PelicanUI;

#[derive(Debug, Clone, Component)]
pub struct TextInput(Stack, InputBackground, InputContent, #[skip] InputState);

impl TextInput {
    pub fn new(
        ctx: &mut Context,
        label: Option<&'static str>,
        value: Option<&'static str>,
        placeholder: &'static str,
        help_text: Option<&'static str>,
        error: Option<&'static str>,
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>,
    ) -> Self {
        let height = 48;
        let padding = 8;
        
        let (background, outline) = InputColor::get(ctx, InputState::Default);
        let content = InputContent::new(ctx, placeholder, icon_button);
        let width = Size::Fill(content.size(ctx).min_width()+(padding*2), MaxSize::MAX);
        let background = InputBackground::new(background, outline, width, height);

        TextInput(
            Stack(Offset::Start, Offset::Center, Size::Fit, Size::Fit, Padding::default()),
            background, content, InputState::Default
        )
    }

    pub fn set_state(&mut self, ctx: &mut Context, state: InputState) {
        if self.3 != state {
            self.3 = state;
            let (background, outline) = InputColor::get(ctx, state);
            println!("State: {:?}", state);
            self.1.set_color(background, outline);
        }
    }
}

impl Events for TextInput {
    fn on_click(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        match (position.is_some(), self.3) {
            (true, InputState::Hover) => self.set_state(ctx, InputState::Focus),
            (false, InputState::Focus) => self.set_state(ctx, InputState::Default),
            _ => {}
        };
        true
    }
    fn on_move(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        match (position.is_some(), self.3) {
            (true, InputState::Default) => self.set_state(ctx, InputState::Hover),
            (false, InputState::Hover) => self.set_state(ctx, InputState::Default),
            _ => {}
        };
        false
    }
    fn on_press(&mut self, ctx: &mut Context, text: String) -> bool {
        match self.3 {
            InputState::Focus => {
                println!("Input field received character: {:?}", text);
                if let BasicText(t, _, _, _, _, _) = &mut self.2.1.1 {
                    *t = self.2.2.to_string()+&text;
                    println!("Input field text contains: {:?}", t);
                }
            },
            _ => println!("No input field is focused.")
        }
        true
    }
}

#[derive(Clone, Debug, Component)]
pub struct InputContent(Row, Option<BasicText>, ExpandableText, #[skip] &'static str, Option<IconButton>);
impl Events for InputContent {}

impl InputContent {
    fn new(
        ctx: &mut Context, 
        placeholder: &'static str,
        icon_button: Option<(&'static str, fn(&mut Context, (u32, u32)) -> ())>
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        let input_value = "";
        InputContent(
            Row(8, Offset::Center, Size::Fit, Padding(8, 0, 8, 0)),
            ExpandableText::new(ctx, placeholder, TextStyle::Secondary, font_size),
            input_value,
            icon_button.map(|(icon, on_click)| IconButton::input(ctx, icon, on_click))
        )
    }
}

#[derive(Clone, Debug, Component)]
pub struct InputBackground(Stack, Shape, Shape);

impl InputBackground {
    pub fn new(bg: Color, oc: Color, width: Size, height: u32) -> Self {
        InputBackground(
            Stack(Offset::Center, Offset::Center, width, Size::Fit, Padding::default()),
            RoundedRectangle::new(100, height, 8, bg),
            Outline::rounded_rectangle(100, height, 8, 1, oc)
        )
    }

    fn set_color(&mut self, bg: Color, oc: Color) {
        let Shape(_, c) = &mut self.1;
        *c = bg;
        let Shape(_, c) = &mut self.2;
        *c = oc;
    }
}

impl Events for InputBackground {
    fn on_resize(&mut self, _ctx: &mut Context, size: (u32, u32)) {
        if let Shape(ShapeType::RoundedRectangle(_, (w, _), _), _) = &mut self.1 {
            *w = size.0;
        }
        if let Shape(ShapeType::RoundedRectangle(_, (w, _), _), _) = &mut self.2 {
            *w = size.0;
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum InputState {
    Default,
    Hover,
    Focus,
    Error
}

#[derive(Default, Clone)]
pub struct InputColor;
impl InputColor {
    fn get(ctx: &mut Context, state: InputState) -> (Color, Color) { // background, outline
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match state {
            InputState::Default => (colors.shades.transparent, colors.outline.secondary),
            InputState::Hover => (colors.background.secondary, colors.outline.secondary),
            InputState::Focus => (colors.shades.transparent, colors.outline.primary),
            InputState::Error => (colors.shades.transparent, colors.status.danger)
        }
    }
}

// pub struct TextInput {
//     label: Option<&'static str>,
//     value: Option<&'static str>,
//     help_text: Option<&'static str>,
//     error: Option<&'static str>,
//     placeholder: &'static str,
//     icon: Option<Icon>
// }

// impl ComponentBuilder for TextInput {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];
//         let mut content: Vec<Box<dyn ComponentBuilder>> = vec![];

//         if let Some(label) = &self.label { 
//             children.push(Text::heading(ctx, label, TextSize::h5())); // Label
//         }

//         if let Some(value) = &self.value { 
//             content.push(Text::primary(ctx, value, TextSize::md())); // Value
//         }

//         if let Some(placeholder) = &self.placeholder { 
//             content.push(Text::secondary(ctx, placeholder, TextSize::md())); // Placeholder
//         }

//         if let Some(icon) = &self.icon { 
//             content.push(IconButton(icon, IconButtonStyle::Secondary, Size::Medium)); // Icon Button
//         }

//         children.push(Stack(ZERO, Align::Center, vec![
//             RoundedRectangle(AUTO, 48, 8, colors.text_input, Some(colors.outline, 1)), // Input Field
//             Row(ZERO, 8, Align::Bottom, content) // Build Content
//         ]));

//         if let Some(text) = &self.help_text {
//             children.push(Text::secondary(ctx, text, TextSize::sm())); // Help Text
//         }

//         if let Some(text) = &self.error {
//             children.push(Text::error(ctx, text, TextSize::sm())); // Erorr Text
//         }

//         Row(Column, 16, Align::TopLeft, children).build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

// Default
// Hover
// Focus Empty
// Focus Filled
// Filled
// Error