use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::Circle;
use crate::discard_nonsies;
// use crate::elements::image::Image;
use crate::PelicanUI;

// CircleIcon
// CircleIconData
// CircleIconStyle

#[derive(Clone)]
pub enum CircleIcon {
    Icon(CircleIconData, Option<CircleIconData>, bool, u32),
    Image(resources::Image, Option<CircleIconData>, bool, u32)
}

impl Component for CircleIcon {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let (content, flair, outline, size) = match self {
            CircleIcon::Icon(d, f, o, s) => 
                (Box::new(d.clone()) as Box<dyn ComponentTag>, *f, *o, *s),
            CircleIcon::Image(i, f, o, s) => 
                (Box::new(Image(ShapeType::Ellipse(0), i.clone(), None)) as Box<dyn ComponentTag>, *f, *o, *s)
        };

        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        let flair_s = (size as f32 / 3.0).round() as u32;
        let stroke = |s| (s as f32 * 0.06).round() as u32;

        Container(Offset::BottomRight, Size::Static(size, size), discard_nonsies![
            Some(content),
            match outline {
                true => Some(Box::new(Shape(ShapeType::Ellipse(stroke(size)), black)) as Box<dyn ComponentTag>),
                false => None
            },
            match flair {
                Some(icon_data) => Some(Box::new(Container(Offset::Center, Size::Static(flair_s, flair_s), vec![
                    Box::new(icon_data.clone()) as Box<dyn ComponentTag>,
                    Box::new(Shape(ShapeType::Ellipse(stroke(flair_s)), black)) as Box<dyn ComponentTag>
                ]))),
                None => None
            }
        ])
    }
}

#[derive(Clone, Copy)]
pub struct CircleIconData(pub &'static str, pub CircleIconStyle);

impl Component for CircleIconData {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let size = max_size.0.min(max_size.1);
        let icon_size = (size as f32 * 0.75).round() as u32;
        let (background, icon_color) = self.1.get(ctx);
        Container(Offset::Center, Size::Fill, vec![
            Box::new(Circle(size - 2, background)),
            Box::new(Icon(self.0, icon_color, icon_size)),
        ])
    }
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