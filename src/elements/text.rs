use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::PelicanUI;

#[derive(Clone)]
pub enum TextStyle {
    Heading,
    Primary,
    Secondary,
    Error,
    White,
    Label(Color),
}

#[derive(Clone)]
pub struct Text;

impl Text {
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: u32) -> BasicText {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, fonts) = (theme.colors.text, theme.fonts.fonts.clone());

        let (color, font) = match style {
            TextStyle::Heading => (colors.heading, fonts.heading.clone()),
            TextStyle::Primary => (colors.primary, fonts.text.clone()),
            TextStyle::Secondary => (colors.secondary, fonts.text.clone()),
            TextStyle::Error => (colors.danger, fonts.text.clone()),
            TextStyle::White => (colors.heading, fonts.text.clone()),
            TextStyle::Label(label_color) => (label_color, fonts.label.clone())
        };

        BasicText::new(text, color, None, size, (size as f32*1.25) as u32, font)
    }
}