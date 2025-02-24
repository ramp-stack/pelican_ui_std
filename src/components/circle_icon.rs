use rust_on_rails::prelude::*;
use crate::{ Child, COLORS, Align, Stack, ZERO, icon};


#[derive(Clone, Copy)]
pub enum CircleIcon {
    Default(&'static str, u32),
    Brand(&'static str, u32)
}

impl ComponentBuilder for CircleIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let (background, _icon_color, size) = match self {
            CircleIcon::Default(_, size) => (COLORS.background.secondary, COLORS.text.secondary, size),
            CircleIcon::Brand(_, size) => (COLORS.brand.primary, COLORS.brand.text, size)
        };

        let icon_s = (*size as f32 * 0.75).round() as u32;

        Stack {padding: ZERO, align: Align::Center, children: vec![
            (Child!(Shape(ShapeType::Circle(*size / 2), background, None)), ZERO),
            (Child!(Image(ShapeType::Rectangle(icon_s, icon_s), icon(ctx).clone())), ZERO)
        ]}.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Clone, Copy)]
pub struct UserIcon(pub &'static str, pub u32, pub Option<u16>);

impl ComponentBuilder for UserIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        Stack { padding: ZERO, align: Align::Center, children: vec![
            (Child!(Shape(ShapeType::Circle(self.1 / 2), COLORS.background.primary, self.2)), ZERO),
            (Child!(Image(ShapeType::Circle(self.1 / 2), icon(ctx).clone())), ZERO)
        ]}.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}


pub struct ProfilePictures(pub Vec<&'static str>);

impl ComponentBuilder for ProfilePictures {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let pfps: Vec<(Box<dyn ComponentBuilder>, Vec2)> = self.0
            .iter().take(5).enumerate()
            .map(|(index, _)| (
                Child!(UserIcon("profile", 32, Some(400))),
                Vec2::new(index as u32 * 20, 0)
            )).collect();

        Stack { padding: ZERO, align: Align::Left, children: pfps }.build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}
