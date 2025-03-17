use rust_on_rails::prelude::*;
use crate::{COLORS, Align, Stack, ZERO};

// CircleIcon(Some(Icon::Door), None, Option<IconStyle::Secondary>, Some((Icon::Edit, IconStyle::Secondary)), false, 128);

// CircleIcon(CircleIconData::Photo(img), None, false, 32);

pub struct CircleIcon(CircleIconData, Option<(Icon, IconStyle)>, bool, u32);

pub enum CircleIconData { 
    Photo(RgbaImage),
    Icon(Icon, IconStyle)
}


impl ComponentBuilder for CircleIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];

        let outline = if self.3 { Some(((self.2 * 13 / 12).ceil() - self.2, COLORS.shades.black)) } else { None };

        children.push(match self.0 {
            CircleIconData::Photo(img) => Image::Circle(*img, outline, self.3),
            CircleIconData::Icon(i, s) => _CircleIcon(*i, *s, outline, self.3)
        });

        if let Some((icon, variant)) = self.2 { children.push(_CircleIcon(icon, variant, true, 14)); }  // Flair

        Stack(ZERO, Align::BottomRight, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

struct _CircleIcon(Icon, IconStyle, Option<(u8, &'static str)>, u32); // Icon, Style, Size, Outline (thickness, color)

impl ComponentBuilder for _CircleIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

        let (background, icon_color) = self.1.to_hex();

        let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];

        children.push(Circle(self.2, background, outline)); // Background Circle
        
        children.push(self.0.build(round(self.2 / 0.75), icon_color)); // Icon

        Icon(IconName::Profile, "ffffff", 38);

        Stack(ZERO, Align::Center, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub enum IconStyle { Primary, Secondary, Brand, Success, Warning, Danger }

impl IconStyle {
    pub fn to_hex(&self) -> (&'static str, &'static str) {
        match self {
            IconStyle::Primary => (COLORS.text.heading, COLORS.background.primary),
            IconStyle::Secondary => (COLORS.background.secondary, COLORS.text.secondary),
            IconStyle::Brand => (COLORS.brand.primary, COLORS.brand.secondary),
            IconStyle::Success => (COLORS.status.success, COLORS.text.heading),
            IconStyle::Warning => (COLORS.status.warning, COLORS.text.heading),
            IconStyle::Danger => (COLORS.status.danger, COLORS.text.heading),
        }
    }
}
