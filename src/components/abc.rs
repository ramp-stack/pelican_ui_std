use rust_on_rails::prelude::*;
use crate::theme;
use crate::{Row, Column, Stack, Text, Padding, ConstrainedBox};
use crate::layout::Align;
use crate::components::button::*;

#[derive(Clone, Copy)]
pub enum CircleIcon {
    Icon(&'static str, u32),
    Photo(&'static str, u32),
    Brand(&'static str, u32)
}

impl ComponentBuilder for CircleIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); // Move this to theme startup
        let colors = theme::color::palette();
        let image = ctx.load_image("images/profile.png").unwrap(); // Get individual path images

        match self {
            CircleIcon::Photo(p, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), colors.background.primary, Some(100)), Vec2::new(0, 0)),
                    (Image(ShapeType::Circle(*s / 2), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            },
            CircleIcon::Icon(p, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), colors.background.secondary, None), Vec2::new(0, 0)),
                    (Image(ShapeType::Rectangle((*s as f32 * 0.75).round() as u32, (*s as f32 * 0.75).round() as u32), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            },
            CircleIcon::Brand(p, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), colors.brand.primary, None), Vec2::new(0, 0)),
                    (Image(ShapeType::Rectangle((*s as f32 * 0.75).round() as u32, (*s as f32 * 0.75).round() as u32), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            }
        }
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Card {
    circle_icon: CircleIcon,
    title: &'static str,
    subtitle: &'static str,
    description: &'static str,
    button: Button
}

impl Card {
    pub fn room(n: &'static str, st: &'static str, d: &'static str) -> Self {
        Self {
            circle_icon: CircleIcon::Photo("profile", 64), // get user pfp
            title: n,
            subtitle: st,
            description: d,
            button: Button::Secondary("Join Room", Size::Medium, Width::Hug, None, Align::Center),
        }
    }
}

impl ComponentBuilder for Card {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        let colors = theme::color::palette();
        Column!(8, Vec2::new(0, 0), Align::Center, false,
            self.circle_icon,
            Text::new(self.title, colors.text.heading, 24, heading.clone()),
            Text::new(self.subtitle, colors.text.primary, 12, font.clone()),
            Padding(1, 6),
            Shape(ShapeType::Rectangle(230, 1), colors.outline.secondary, None),
            Padding(1, 6),
            Text::new(self.description, colors.text.primary, 14, font.clone()),
            self.button
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Navigator (Vec<(&'static str, &'static str)>, u16, bool);

impl Navigator {
    pub fn new(tabs: Vec<(&'static str, &'static str)>, default_i: u16, is_desktop: bool) -> Self {
        Self(tabs, default_i, is_desktop)
    }
}

impl ComponentBuilder for Navigator {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        let colors = theme::color::palette();

        let image = ctx.load_image("images/logomark.png").unwrap(); // Default logomark

        // Image(ShapeType::Rectangle(icon_size, 8), image),

        let buttons = self.0.iter().enumerate().map(|(index, (name, _))| {
            Box::new(if index as u16 == self.1 {
                Button::Secondary(*name, Size::Large, Width::Expand, None, Align::Left) // Hover not Secondary
            } else {
                Button::Ghost(*name, Size::Large, Width::Expand, None, Align::Left)
            }) as Box<dyn ComponentBuilder>
        }).collect();

        ConstrainedBox!(300, 
            Column!(32, Vec2::new(0, 0), Align::Center, false,
                Image(ShapeType::Rectangle(150, 24), image),
                Column!(8, Vec2::new(0, 0), Align::Center, true, buttons)
            )
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}
