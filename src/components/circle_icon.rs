use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Image as RailsImage;
use crate::{COLORS, Align, ZERO, Stack};
use crate::theme::icons::{Icon, IconName};
use crate::components::shapes::{Circle, Outline, Image};
use crate::{StackOption, StackDirect};

// pub struct CircleIcon(pub CircleIconData, pub Option<(IconName, IconStyle)>, pub bool, pub u32);

pub struct CircleIcon(pub bool, pub u32, Box<dyn ComponentBuilder>, Option<Icon>);

// Image or icon, flair, outline, size

pub enum CircleIcon { 
    Photo(Image, Option<(IconName, CircleIconStyle)>, bool, u32),
    Icon(Icon, Option<(IconName, CircleIconStyle)>, bool, u32)
}

pub struct CircleIconData(IconName, CircleIconStyle);

impl CircleIconData {
    pub fn new_photo(image: Image) -> Self {
        Self::Photo(image)
    }

    pub fn new_icon(icon: IconName, style: CircleIconStyle) -> Self {
        Self::Icon(icon)
    }
}

impl CircleIcon {
    fn _new(data: Box<dyn ComponentBuilder>, flair: Option<Icon>, outline: bool, size: u32) -> Self {

    }

    pub fn new(icon: IconName, style: CircleIconStyle, flair: Option<(IconName, CircleIconStyle)>, outline: bool, size: u32) -> Self {
        
    }
}

impl ComponentBuilder for CircleIcon {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>>  = vec![];

        let outline = if self.2 { 
            Some(Outline((self.3 as f32 * 0.3).round() as u32, COLORS.shades.black)) // Calculate and create outline
        } else { None };

        children.push(match &self.0 {
            CircleIconData::Photo(img) => Box::new(Image::Circle(img.clone(), outline, self.3)), // Create circle image
            CircleIconData::Icon(i, s) => Box::new(_CircleIcon(*i, *s, outline, self.3)) // Create circle icon
        });

        if let Some((icon, variant)) = &self.1 { // Add FLAIR
            let size = (self.3 as f32 * 0.3).round() as u32;
            children.push(Box::new(
                _CircleIcon(
                    *icon, 
                    *variant,
                    Some(Outline((size as f32 * 0.06).ceil() as u32, COLORS.shades.black)), 
                    size
                )
            ));
        } 

        Stack(ZERO, Align::BottomRight, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}

struct _CircleIcon(IconName, CircleIconStyle, Option<Outline>, u32); // Icon, Style, Outline (thickness, color), Size

impl ComponentBuilder for _CircleIcon {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let (background, icon_color) = self.1.to_hex();

        Stack(ZERO, Align::Center, vec![
            Box::new(Circle(self.3, background, self.2)), // Circle
            Box::new(Icon::new(ctx, self.0, icon_color, (self.3 as f32 * 0.75).round() as u32)) // Icon
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}

#[derive(Clone, Copy)]
pub enum CircleIconStyle { 
    Primary, 
    Secondary,
    Brand, 
    Success, 
    Warning, 
    Danger 
}

impl CircleIconStyle {
    pub fn to_hex(&self) -> (&'static str, &'static str) {
        match self {
            CircleIconStyle::Primary => (COLORS.text.heading, COLORS.background.primary),
            CircleIconStyle::Secondary => (COLORS.background.secondary, COLORS.text.secondary),
            CircleIconStyle::Brand => (COLORS.brand.primary, COLORS.brand.secondary),
            CircleIconStyle::Success => (COLORS.status.success, COLORS.text.heading),
            CircleIconStyle::Warning => (COLORS.status.warning, COLORS.text.heading),
            CircleIconStyle::Danger => (COLORS.status.danger, COLORS.text.heading),
        }
    }
}
