use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Row, Column, ZERO, Align, Stack, COLORS };
use crate::components::{Button};

pub struct DataItem {
    number: Option<&'static str>,
    label: &'static str,
    table: Option<Vec<(&'static str, &'static str)>>,
    text: Option<&'static str>,
    secondary_text: Option<&'static str>,
    quick_actions: Option<Vec<&'static str>>,
}

impl ComponentBuilder for DataItem {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut contents: Vec<Box<dyn ComponentBuilder>> = vec![];

        contents.push(Text::heading(ctx, self.label, TextSize::h5()));

        if let Some(text) = &self.text { 
            contents.push(Text::primary(ctx, self.text, TextSize::md())); 
        }

        if let Some(secondary_text) = &self.secondary_text { 
            contents.push(Text::secondary(ctx, self.secondary_text, TextSize::sm())); 
        }

        if !self.table.is_empty() { 
            let tabulars = &self.table
                .into_iter()
                .map(|row| {
                    Row(ZERO, AUTO, Align::Center, vec![
                        Text::primary(ctx, row.0, TextSize::sm()),
                        Text::primary(ctx, row.1, TextSize::sm())
                    ])
                }).collect();

            contents.push(Column(ZERO, 0, Align::Left, tabulars));
        }

        if !self.quick_actions.is_empty() { 
            let buttons = &self.quick_actions
                .into_iter()
                .map(|label| {
                    Button::secondary(label, Icon::Edit, Size::Medium, Width::Hug)
                }).collect();

            contents.push(Row(ZERO, 8, Align::Left, buttons));
        }

        if let Some(num) = self.number { 
            children.push(
                Stack(ZERO, Align::Center, vec![
                    Circle(32, COLORS.background.secondary, None),
                    Text::heading(ctx, num, TextSize::h5())
                ])
            ); 
        }

        children.push(Column(ZERO, 16, Align::Left, contents));

        Row(ZERO, 16, Align::Left, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

// let confirm_amount = DataItem {
//     number: Some("2"),
//     label: "Confirm amount",
//     table: vec![
//         ("date", "12/25/20"),
//         ("time", "11:45 PM")
//     ],
//     text: None,
//     secondary_text: None,
//     quick_actions: vec!["Edit amount", "Edit speed"]
// }

// let confirm_address = DataItem {
//     number: Some("1"),
//     label: "Confirm adress",
//     table: Vec::new(),
//     text: Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
//     secondary_text: Some("Bitcoin sent to the wrong address can never be recovered."),
//     quick_actions: vec!["Edit address"]
// }