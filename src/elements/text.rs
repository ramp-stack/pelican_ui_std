use pelican_ui::events::{MouseState, MouseEvent, OnEvent, Event, TickEvent, Key, NamedKey};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::drawable::{Drawable, Component, Shape, Color, Align, Span, Cursor};
use pelican_ui::drawable::Text as BasicText;
use pelican_ui::{Context, Component, resources};

use crate::layout::{Stack, Offset, Size, Padding, Opt, Row};
use crate::elements::shapes::{Rectangle, Circle};

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
        let theme = &ctx.theme;
        match self {
            TextStyle::Heading => (theme.colors.text.heading, theme.fonts.fonts.heading.clone()),
            TextStyle::Primary => (theme.colors.text.primary, theme.fonts.fonts.text.clone()),
            TextStyle::Secondary => (theme.colors.text.secondary, theme.fonts.fonts.text.clone()),
            TextStyle::Error => (theme.colors.status.danger, theme.fonts.fonts.text.clone()),
            TextStyle::White => (theme.colors.text.heading, theme.fonts.fonts.text.clone()),
            TextStyle::Keyboard => (theme.colors.text.heading, theme.fonts.fonts.keyboard.clone()),
            TextStyle::Label(color) => (*color, theme.fonts.fonts.label.clone()),
        }
    }
}

#[derive(Component, Debug)]
pub struct Text(Stack, BasicText);
impl OnEvent for Text {}

impl Text {
    pub fn new(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        let text = BasicText::new(vec![Span::new(text.to_string(), size, Some(size*1.25), font, color)], None, align);
        Text(Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()), text)
    }

    pub fn text(&mut self) -> &mut BasicText { &mut self.1 }
}

#[derive(Debug)]
pub struct ExpandableText(pub Text);
impl OnEvent for ExpandableText {}

impl ExpandableText {
    pub fn new(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align) -> Self {
        ExpandableText(Text::new(ctx, text, style, size, align))
    }

    pub fn text(&mut self) -> &mut BasicText { self.0.text() }
}

impl Component for ExpandableText {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![&mut self.0] }
    fn children(&self) -> Vec<&dyn Drawable> { vec![&self.0] }

    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::new(0.0, children[0].min_height(), f32::MAX, children[0].max_height())
    }

    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        self.0.text().width = Some(size.0);
        vec![Area{offset: (0.0, 0.0), size}]
    }
}

#[derive(Component, Debug)]
pub struct TextEditor(Stack, ExpandableText, TextCursor);

impl TextEditor {
    pub fn new(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align) -> Self {
        let mut text = ExpandableText::new(ctx, text, style, size, align);
        text.text().cursor = Some(Cursor::default());
        TextEditor(Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()), text, TextCursor::new(ctx, style, size))
    }

    pub fn text(&mut self) -> &mut BasicText { self.1.text() }

    pub fn apply_edit(&mut self, _ctx: &mut Context, key: &Key) {
        let index = self.text().cursor.unwrap();
        match key {
            Key::Named(NamedKey::Enter) => {
                match index >= self.text().spans[0].text.len() {
                    true => self.text().spans[0].text.push('\n'),
                    false => self.text().spans[0].text.insert(index, '\n'),
                };
                if let Some(c) = self.text().cursor.as_mut() {*c += 1};
            },
            Key::Named(NamedKey::Space) => {
                match index >= self.text().spans[0].text.len() {
                    true => self.text().spans[0].text.push(' '),
                    false => self.text().spans[0].text.insert(index, ' '),
                };
                if let Some(c) = self.text().cursor.as_mut() {*c += 1};
            },
            Key::Named(NamedKey::Delete | NamedKey::Backspace) => {
                self.text().spans[0].text = {
                    let mut chars: Vec<char> = self.text().spans[0].text.chars().collect();

                    match chars.len() {
                        1 => chars.clear(),
                        _ if index >= chars.len() => {chars.pop();},
                        _ => {chars.remove(index);}
                    }

                    chars.into_iter().collect()
                };
                if let Some(c) = self.text().cursor.as_mut() { *c = c.saturating_sub(1); }
            },
            Key::Character(c) => {
                match index >= self.text().spans[0].text.len() {
                    true => c.chars().next().map(|ch| self.text().spans[0].text.push(ch)),
                    false => c.chars().next().map(|ch| self.text().spans[0].text.insert(index, ch)),
                };
                if let Some(c) = self.text().cursor.as_mut() {*c += 1;}
            },
            _ => {}
        };
    }

    pub fn display_cursor(&mut self, display: bool) {
        self.2.1.display(display)
    }
}


impl OnEvent for TextEditor {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let cursor_pos = self.text().cursor_position();
            *self.2.x_offset() = Offset::Static(cursor_pos.0);
            *self.2.y_offset() = Offset::Static(cursor_pos.1+2.0);
        } else if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if event.state == MouseState::Pressed && event.position.is_some() {
                self.text().cursor_click(event.position.unwrap().0, event.position.unwrap().1)
            }
        }
        
        true
    }
}

#[derive(Component, Debug)]
pub struct TextCursor(Stack, Opt<Rectangle>);

impl OnEvent for TextCursor {}

impl TextCursor {
    pub fn new(ctx: &mut Context, style: TextStyle, size: f32) -> Self {
        let (color, _) = style.get(ctx);
        TextCursor(
            Stack(Offset::Start, Offset::End, Size::Static(2.0), Size::Static(size), Padding::default()), 
            Opt::new(Rectangle::new(color), false)
        )
    }

    pub fn x_offset(&mut self) -> &mut Offset { &mut self.0.0 }
    pub fn y_offset(&mut self) -> &mut Offset { &mut self.0.1 }
}

#[derive(Debug, Component)]
pub struct BulletedText(Row, Shape, ExpandableText);

impl OnEvent for BulletedText {}

impl BulletedText {
    pub fn new(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, _) = style.get(ctx);
        BulletedText(
            Row::new(size*0.75, Offset::Center, Size::Fit, Padding::default()), // change this offset to be line_height - circle size / 2
            Circle::new(size*0.5, color),
            ExpandableText::new(ctx, text, style, size, align)
        )
    }
    pub fn text(&mut self) -> &mut BasicText { self.2.text() }
}
