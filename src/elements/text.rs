use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::layout::{Stack, Offset, Size, Bin, Padding, Opt};
use crate::events::CursorMovedEvent;
use crate::elements::shapes::Rectangle;
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

#[derive(Component, Debug)]
pub struct Text(Stack, BasicText, Option<Opt<Bin<Stack, Rectangle>>>);

impl Text {
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        let text = BasicText::new(vec![Span::new(text, size, size*1.25, font, color)], None, align, false);
        Text(Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()), text, None)
    }

    pub fn new_with_edit(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        let text = BasicText::new(vec![Span::new(text, size, size*1.25, font, color)], None, align, true);
        Text(
            Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()),
            text,
            Some(Opt::new(Bin(
                Stack(Offset::Start, Offset::Start, Size::Static(2.0), Size::Static(size), Padding::default()), 
                Rectangle::new(color)
            ), true)),
        )
    }

    pub fn text(&mut self) -> &mut BasicText {&mut self.1}
    pub fn cursor(&mut self) -> &mut Option<Opt<Bin<Stack, Rectangle>>> {&mut self.2}
}


impl OnEvent for Text {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<CursorMovedEvent>() {
            if let Some(c) = &mut self.2 {
                if let Some(cords) = self.1.cursor_position(ctx.as_canvas()) {
                    c.inner().0.0 = Offset::Static(cords.0);
                    c.inner().0.1 = Offset::Static(cords.1+(self.1.spans[0].font_size/2.0));
                }
            }
        } else if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if event.state == MouseState::Pressed && event.position.is_some() {
                self.1.set_cursor(ctx.as_canvas(), event.position.unwrap().0, event.position.unwrap().1);
                ctx.trigger_event(CursorMovedEvent);
            }
        }
        true
    }
}

#[derive(Debug)]
pub struct ExpandableText(pub Text);

impl ExpandableText {
    pub fn new(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        ExpandableText(Text::new(ctx, text, style, size, align))
    }

    pub fn new_with_edit(ctx: &mut Context, text: &'static str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        ExpandableText(Text::new_with_edit(ctx, text, style, size, align))
    }

    pub fn text(&mut self) -> &mut BasicText {self.0.text()}
    pub fn cursor(&mut self) -> &mut Option<Opt<Bin<Stack, Rectangle>>> {self.0.cursor()}
}
impl OnEvent for ExpandableText {}

impl Component for ExpandableText {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {vec![&mut self.0]}
    fn children(&self) -> Vec<&dyn Drawable> {vec![&self.0]}
    fn request_size(&self, ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        let max_height = self.0.1.size(ctx).1;
        SizeRequest::new(0.0, 0.0, f32::MAX, max_height)
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        self.0.text().width = Some(size.0);
        vec![Area{offset: (0.0, 0.0), size}]
    }
}
