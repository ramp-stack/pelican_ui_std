use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::Circle;
// use crate::elements::image::Image;
use crate::PelicanUI;

#[derive(Clone)]
pub enum ProfileImage {
    Icon(CircleIconData),
    Image(resources::Image)
}

#[derive(Clone)]
pub struct CircleIcon(pub ProfileImage, pub Option<CircleIconData>, pub bool);

impl Component for CircleIcon {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Vec<((u32, u32), Box<dyn Component>)> {
        let color = ctx.get::<PelicanUI>().theme.colors.shades.black;
        let size = max_size.0.min(max_size.1);
        let s = (size as f32 * 0.3).round() as u32;

        vec![
            match &self.0 {
                ProfileImage::Icon(circle_icon) => circle_icon.clone().stack(),
                ProfileImage::Image(profile) => Image(ShapeType::Ellipse(0, (size, size)), profile.clone(), None).stack()
            },
            match self.2 {
                true => Some(Shape(ShapeType::Ellipse((size as f32 * 0.06).round() as u32, (size, size)), color)),
                false => None
            }.stack(),
            self.1.as_ref().map(|circle_icon| {
                container![
                    (s, s),
                    circle_icon.clone().stack(),
                    Shape(ShapeType::Ellipse((s as f32 * 0.06).round() as u32, (s, s)), color).stack(),
                ]
            }).align(ctx, max_size, Align::BottomRight)
        ]
    }
}



#[derive(Clone)]
pub struct CircleIconData(pub &'static str, pub CircleIconStyle);

impl Component for CircleIconData {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Vec<((u32, u32), Box<dyn Component>)> {
        let size = max_size.0.min(max_size.1);
        let (background, icon_color) = self.1.get(ctx);
        vec![
            Circle(size, background).stack(),
            Icon(self.0, icon_color, size).stack()
        ]
    }
}

#[derive(Clone)]
pub enum CircleIconStyle {
    Primary,
    Secondary,
    Brand,
    Success,
    Warning,
    Danger
}

impl CircleIconStyle {
    pub fn get(&self, ctx: &mut Context) -> (Color, Color) {
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            CircleIconStyle::Primary => (colors.text.heading, colors.background.primary),
            CircleIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
            CircleIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
            CircleIconStyle::Success => (colors.status.success, colors.text.heading),
            CircleIconStyle::Warning => (colors.status.warning, colors.text.heading),
            CircleIconStyle::Danger => (colors.status.danger, colors.text.heading),
        }
    }
}
