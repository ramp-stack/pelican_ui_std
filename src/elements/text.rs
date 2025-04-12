use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
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
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32) -> BasicText {
        let (color, font) = style.get(ctx);
        BasicText::new(text, color, None, size, size *1.25, font)
    }
}

#[derive(Debug)]
pub struct ExpandableText(pub BasicText);

impl ExpandableText {
    pub fn text(&mut self) -> &mut String {&mut self.0.text}

    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32) -> Self {
        let (color, font) = style.get(ctx);
        ExpandableText(BasicText::new(text, color, None, size, size*1.25, font))
    }
}

impl Events for ExpandableText {}
impl Component for ExpandableText {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {vec![&mut self.0]}
    fn children(&self) -> Vec<&dyn Drawable> {vec![&self.0]}
    fn request_size(&self, ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        let request = self.0.request_size(ctx);
        SizeRequest::new(0.0, request.min_height(), f32::MAX, request.max_height())
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        let BasicText{max_width, ..} = &mut self.0;
        *max_width = Some(size.0);
        vec![Area{offset: (0, 0), size}]
    }
}
