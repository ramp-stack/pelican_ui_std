use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Column, COLORS, Align, ConstrainedBox, Row, ZERO, Expand };
use crate::components::ProfilePictures;

#[derive(Clone, Copy)]
pub enum Header {
    Home(Option<&'static str>, &'static str, Option<&'static str>),
    Stack(Option<&'static str>, &'static str, Option<&'static str>),
    Chat(Option<&'static str>, &'static str, Option<&'static str>)
}

impl ComponentBuilder for Header {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let center = match self {
            Header::Home(_, t, _) => (Child!(Text::heading(ctx, t, TextSize::h3())), false),
            Header::Stack(_, t, _) => (Child!(Text::heading(ctx, t, TextSize::h4())), false),
            Header::Chat(_, t, _) => (Child!(Column { 
                padding: ZERO, spacing: 10, align: Align::Center, children: vec![
                    (Child!(ProfilePictures(vec![])), false),
                    (Child!(Text::heading(ctx, t, TextSize::h4())), false)
                ]
            }), false),
        };

        let (_left, _right) = match self {
            Header::Home(l, _, r) => (l, r),
            Header::Stack(l, _, r) => (l, r),
            Header::Chat(l, _, r) => (l, r),
        };

        ConstrainedBox!(300, Row { spacing: 16, align: Align::Center, padding: ZERO, children: vec![
            (Child!(Shape(ShapeType::Rectangle(32, 32), "ffffff", None)), false),
            (Child!(Expand(false, 1, COLORS.background.primary)), true),
            center,
            (Child!(Expand(false, 1, COLORS.background.primary)), true),
            (Child!(Shape(ShapeType::Rectangle(32, 32), "ffffff", None)), false)
        ]}).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}