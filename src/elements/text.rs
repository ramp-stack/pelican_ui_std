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
pub struct Text(pub TextStyle, pub &'static str, pub u32);

impl Component for Text {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Container {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, fonts) = (theme.colors.text, theme.fonts.fonts.clone());

        let (color, font) = match self.0 {
            TextStyle::Heading => (colors.heading, fonts.heading.clone()),
            TextStyle::Primary => (colors.primary, fonts.text.clone()),
            TextStyle::Secondary => (colors.secondary, fonts.text.clone()),
            TextStyle::Error => (colors.danger, fonts.text.clone()),
            TextStyle::White => (colors.heading, fonts.text.clone()),
            TextStyle::Label(label_color) => (label_color, fonts.label.clone())
        };

        Container(Offset::default(), Size::Fit, vec![
            Box::new(BasicText(self.1, color, self.2, (self.2 as f32*1.25) as u32, font, fonts.emoji.clone()))
        ])
    }
}