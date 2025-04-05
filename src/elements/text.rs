use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::layout::{Stack, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Clone, Copy, Debug)]
pub enum TextStyle {
    Heading,
    Primary,
    Secondary,
    Error,
    White,
    Keyboard,
    Label(Color),
}

impl TextStyle {
    pub fn get(&self, ctx: &mut Context) -> (Color, resources::Font) {
        let theme = &ctx.get::<PelicanUI>().theme;
        match self {
            TextStyle::Heading => (theme.colors.text.heading, theme.fonts.fonts.heading.clone()),
            TextStyle::Primary => (theme.colors.text.primary, theme.fonts.fonts.text.clone()),
            TextStyle::Secondary => (theme.colors.text.secondary, theme.fonts.fonts.text.clone()),
            TextStyle::Error => (theme.colors.text.danger, theme.fonts.fonts.text.clone()),
            TextStyle::White => (theme.colors.text.heading, theme.fonts.fonts.text.clone()),
            TextStyle::Keyboard => (theme.colors.text.heading, theme.fonts.fonts.keyboard.clone()),
            TextStyle::Label(color) => (*color, theme.fonts.fonts.label.clone()),
        }
    }
}

pub struct Text;

impl Text {
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: u32) -> BasicText {
        let (color, font) = style.get(ctx);
        BasicText::new(text, color, None, size, (size as f32*1.25) as u32, font)
    }
}

#[derive(Debug, Component)]
pub struct ExpandableText(pub Stack, pub BasicText);

impl ExpandableText {
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: u32) -> Self {
        let (color, font) = style.get(ctx);
        ExpandableText(
            Stack(Offset::default(), Offset::default(), Size::fill(), Size::Fit, Padding::default()),
            BasicText::new(text, color, None, size, (size as f32*1.25) as u32, font)
        )
    }

    pub fn value(&mut self) -> &mut String {self.1.value()}
}

impl Events for ExpandableText {
    fn on_resize(&mut self, _ctx: &mut Context, size: (u32, u32)) {
        let BasicText(_, _, min_width, _, _, _) = &mut self.1; 
        *min_width = Some(size.0);
    }
}
