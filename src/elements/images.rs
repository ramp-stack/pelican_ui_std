use rust_on_rails::prelude::*;
use crate::PelicanUI;
use crate::layout::{Stack, Offset, Size};
use crate::elements::shapes::Circle;

#[derive(Debug, Component)]
pub struct Icon(Stack, pub Image);

impl Icon {
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: u32) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        Icon(
            Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit),
            Image(ShapeType::Rectangle(0, (size, size)), theme.icons.get(name), Some(color))
        )
    }

    pub fn set_color(&mut self, color: Color) {
        let Image(_, _, c) = &mut self.1;
        *c = Some(color);
    }
}
impl Events for Icon {}

#[derive(Clone, Debug)]
pub enum CircleIconStyle {
    Primary,
    Secondary,
    Brand,
    Success,
    Warning,
    Danger
}

impl CircleIconStyle {
    fn get(&self, ctx: &mut Context) -> (Color, Color) {
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

#[derive(Debug, Component)]
pub struct CircleIcon(Stack, Shape, Icon);
impl Events for CircleIcon {}

impl CircleIcon {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        let icon_size = (size as f32 * 0.75).round() as u32;
        let (background, icon_color) = style.get(ctx);
        CircleIcon(
            Stack::center(),
            Circle::new(size - 2, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
}
