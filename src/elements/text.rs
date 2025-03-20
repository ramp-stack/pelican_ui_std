use rust_on_rails::prelude::*;
use crate::PelicanUI;

pub enum TextStyle {
    Heading,
    Primary,
    Secondary,
    Error,
    White,
    Label,
}

pub struct Text(pub TextStyle, pub &'static str, pub u32);

impl ComponentBuilder for Text {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, fonts) = (theme.colors, theme.fonts.fonts.clone());

        let (color, font) = match self.0 {
            TextStyle::Heading => (colors.text.heading, fonts.heading.clone()),
            TextStyle::Primary => (colors.text.primary, fonts.text.clone()),
            TextStyle::Secondary => (colors.text.secondary, fonts.text.clone()),
            TextStyle::Error => (colors.status.danger, fonts.text.clone()),
            TextStyle::White => (colors.text.heading, fonts.text.clone()),
            TextStyle::Label => (colors.text.heading, fonts.label.clone())
        };

        BasicText(self.1, color, None, self.2, (self.2 as f32*1.25) as u32, font).build_children(ctx, max_size)
    }
}