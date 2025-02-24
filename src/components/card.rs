use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Column, Padding, COLORS, Align };
use crate::components::{UserIcon, Button, Size, Width};

pub struct Card {
    user_icon: UserIcon,
    title: &'static str,
    subtitle: &'static str,
    description: &'static str,
    button: Button
}

impl Card {
    pub fn room(n: &'static str, st: &'static str, d: &'static str) -> Self {
        Self {
            user_icon: UserIcon("profile", 64, None), // get user pfp
            title: n,
            subtitle: st,
            description: d,
            button: Button::secondary("Join Room", Size::Medium, Width::Hug, None, Align::Center),
        }
    }
}

impl ComponentBuilder for Card {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Column {
            padding: Vec2::new(14, 16), spacing: 8, align: Align::Center,
            children: vec![
                (Child!(self.user_icon), false),
                (Child!(Text::heading(ctx, self.title, TextSize::h3())), false),
                (Child!(Text::primary(ctx, self.subtitle, TextSize::xs())), false),
                (Child!(Padding(Vec2::new(1, 6), COLORS.background.primary)), false),
                (Child!(Shape(ShapeType::Rectangle(230, 1), COLORS.outline.secondary, None)), false),
                (Child!(Padding(Vec2::new(1, 6), COLORS.background.primary)), false),
                (Child!(Text::primary(ctx, self.description, TextSize::sm())), false),
                (Child!(self.button), false)
            ]
        }.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}
