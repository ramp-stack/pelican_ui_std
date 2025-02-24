use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Row, Column, ZERO, Align, Stack, COLORS };
use crate::components::{Button};

pub struct DataItem {
    number: Option<&'static str>,
    label: &'static str,
    text: &'static str,
    description: Option<&'static str>,
    buttons: (Option<Button>, Option<Button>, Option<Button>)
}

impl ComponentBuilder for DataItem {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut data_item: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut contents: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];
        let mut quick_actions: Vec<(Box<dyn ComponentBuilder>, bool)> = vec![];

        if let Some(number) = &self.number { 
            data_item.push((Child!(Stack { padding: ZERO, align: Align::Center, children: vec![
                (Child!(Shape(ShapeType::Circle(32 / 2), COLORS.background.secondary, None)), ZERO),
                (Child!(Text::heading(ctx, number, TextSize::h5())), ZERO)
            ]}), false)); 
        }

        contents.push((Child!(Text::heading(ctx, self.label, TextSize::h5())), false));
        contents.push((Child!(Text::primary(ctx, self.text, TextSize::md())), false));

        if let Some(desc) = &self.description { 
            contents.push((Child!(Text::secondary(ctx, desc, TextSize::sm())), false)); 
        }

        if let Some(button) = &self.buttons.0 { quick_actions.push((Child!(*button), false)); }
        if let Some(button) = &self.buttons.1 { quick_actions.push((Child!(*button), false)); }
        if let Some(button) = &self.buttons.2 { quick_actions.push((Child!(*button), false)); }

        contents.push((Child!(Row { children: quick_actions, align: Align::Left, spacing: 8, padding: ZERO }), false));
        data_item.push((Child!(Column {children: contents, align: Align::Left, spacing: 16, padding: ZERO }), false));

        Row { children: data_item, align: Align::Left, spacing: 16, padding: ZERO }.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}