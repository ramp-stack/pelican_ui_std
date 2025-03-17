use rust_on_rails::prelude::*;
use crate::theme::fonts::{Text, TextSize};
use crate::{ Child, Column, Padding, COLORS, Align };
use crate::components::{UserIcon, Button, Size, Width};

pub struct Card {
    circle_icon: CircleIconData,
    title: &'static str,
    subtitle: &'static str,
    description: &'static str>
}

impl ComponentBuilder for Card {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let content = Column(Vec2::new(14, 16), 8, Align::Center, vec![
            CircleIcon(self.circle_icon, None, None, 64),
            Text::heading(ctx, self.title, TextSize::h3()),
            Text::primary(ctx, self.subtitle, TextSize::xs()),
            _SeparationLine(AUTO, 1, 6),
            Text::primary(ctx, self.description, TextSize::sm())
        ]);

        let bound = Rect::new(0, 0, max_size.x, max_size.y);
        let mut built_content = content.build(ctx, bound);
        let (width, height) = (built_content.size(ctx).x + pad, built_content.size(ctx).y + pad);

        Stack(ZERO, Align::Center, vec![
            RoundedRectangle(width, height, 8, background, None),
            content
        ]).build(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

struct _SeparationLine(u32, u32, u32); // width, height, vertical padding

let card = Card {
    circle_icon: CircleIconData::Photo(Image("../photos/chicken_on_a_donkey.png")),
    title: "Donkey Farmers",
    subtitle: "101 members",
    description: "A place for donkey farmers to converse.",
}