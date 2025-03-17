use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Row, Column, ZERO, Align };
use crate::components::{UserIcon, CircleIcon};

pub struct TextInput {
    label: Option<&'static str>,
    value: Option<&'static str>,
    help_text: Option<&'static str>,
    error: Option<&'static str>,
    placeholder: &'static str,
    icon: Option<Icon>
}

impl ComponentBuilder for TextInput {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut content: Vec<Box<dyn ComponentBuilder>> = vec![];

        if let Some(label) = &self.label { 
            children.push(Text::heading(ctx, label, TextSize::h5())); // Label
        }

        if let Some(value) = &self.value { 
            content.push(Text::primary(ctx, value, TextSize::md())); // Value
        }

        if let Some(placeholder) = &self.placeholder { 
            content.push(Text::secondary(ctx, placeholder, TextSize::md())); // Placeholder
        }

        if let Some(icon) = &self.icon { 
            content.push(IconButton(icon, IconButtonStyle::Secondary, Size::Medium)); // Icon Button
        }

        children.push(Stack(ZERO, Align::Center, vec![
            RoundedRectangle(AUTO, 48, 8, colors.text_input, Some(colors.outline, 1)), // Input Field
            Row(ZERO, 8, Align::Bottom, content) // Build Content
        ]));

        if let Some(text) = &self.help_text {
            children.push(Text::secondary(ctx, text, TextSize::sm())); // Help Text
        }

        if let Some(text) = &self.error {
            children.push(Text::error(ctx, text, TextSize::sm())); // Erorr Text
        }

        Row(Column, 16, Align::TopLeft, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

// Default
// Hover
// Focus Empty
// Focus Filled
// Filled
// Error