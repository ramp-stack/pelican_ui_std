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
        let text = BasicText::new(vec![Span::new(text.to_string(), size, Some(size*1.25), font, color)], None, align, None);
        Text(Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()), text)
    }

    pub fn new_with_cursor(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align) -> Self {
        let (color, font) = style.get(ctx);
        let text = BasicText::new(vec![Span::new(text.to_string(), size, Some(size*1.25), font, color)], None, align, Some(Cursor::default()));
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

    pub fn new_with_cursor(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align) -> Self {
        ExpandableText(Text::new_with_cursor(ctx, text, style, size, align))
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
pub struct TextEditor(Stack, Option<Text>, Option<ExpandableText>);

impl TextEditor {
    pub fn new(ctx: &mut Context, text: &str, style: TextStyle, size: f32, align: Align, can_expand: bool) -> Self {
        let (t, et) = match can_expand {
            true => (None, Some(ExpandableText::new_with_cursor(ctx, text, style, size, align))),
            false => (Some(Text::new_with_cursor(ctx, text, style, size, align)), None)
        };

        TextEditor(
            Stack(Offset::Start, Offset::Start, Size::Fit, Size::Fit, Padding::default()),
            t, et, //TextCursor::new(ctx, style, size)
        )
    }
    
    pub fn text(&mut self) -> &mut BasicText {
        if let Some(text) = &mut self.1 {
            return text.text();
        }

        self.2.as_mut().unwrap().text()
    }

    // pub fn cursor(&mut self) -> &mut TextCursor { &mut self.3 }

    // pub fn display_cursor(&mut self, display: bool) {
    //     self.3.display(display);
    // }

    pub fn apply_edit(&mut self, ctx: &mut Context, key: &Key) {
        // if let Some((i, _)) = self.text().cursor_action(CursorAction::GetIndex) {
            let new_text = self.text().spans[0].text.clone();
            match key {
                Key::Named(NamedKey::Enter) => {
                    new_text.chars().collect::<Vec<char>>().push('\n');
                    self.text().spans[0].text = new_text;
                    // self.text().spans[0].text = Self::insert_char(new_text, '\n', i as usize);
                    // self.text().cursor_action(CursorAction::MoveNewline);
                },
                Key::Named(NamedKey::Space) => {
                    new_text.chars().collect::<Vec<char>>().push(' ');
                    self.text().spans[0].text = new_text;
                    // self.text().spans[0].text = Self::insert_char(new_text, ' ', i as usize);
                    // self.text().cursor_action(CursorAction::MoveRight);
                },
                Key::Named(NamedKey::Delete | NamedKey::Backspace) => {
                    // self.text().cursor_action(CursorAction::MoveLeft);
                    let i = new_text.len().saturating_sub(1);
                    self.text().spans[0].text = Self::remove_char(new_text, i);
                },
                Key::Character(c) => {
                    // self.2.text().text().spans[0].text.insert_str(i as usize , c);
                    let c = c.chars().next().unwrap();
                    new_text.chars().collect::<Vec<char>>().push(c);
                    self.text().spans[0].text = new_text;
                    // self.text().spans[0].text = Self::insert_char(new_text, c, i as usize);
                    // self.text().cursor_action(CursorAction::MoveRight);
                },
                _ => {}
            };
        // }
    }

    fn insert_char(text: String, new: char, index: usize) -> String {
        let mut chars: Vec<char> = text.chars().collect();
        match index >= chars.len() {
            true => chars.push(new),
            false => chars.insert(index, new)
        }
    
        chars.into_iter().collect()
    }
    
    fn remove_char(text: String, index: usize) -> String {
        let mut chars: Vec<char> = text.chars().collect();
        match chars.len() == 1 {
            true => {chars.clear();},
            false if index >= chars.len() => {chars.pop();},
            false => {chars.remove(index);},
        }
    
        chars.into_iter().collect()
    }
}


impl OnEvent for TextEditor {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        // let mut text = self.text().clone();
        if event.downcast_ref::<TickEvent>().is_some() {
            // if let Some(cords) = text.cursor_action(CursorAction::GetPosition) {
            //     *self.3.x_offset() = Offset::Static(cords.0);
            //     *self.3.y_offset() = Offset::Static(cords.1-(text.spans[0].line_height/1.2));
            // }
        } else if let Some(event) = event.downcast_ref::<MouseEvent>() {
            // if event.state == MouseState::Pressed && event.position.is_some() {
            //     text.set_cursor((event.position.unwrap().0, event.position.unwrap().1));
            //     text.cursor_action(CursorAction::GetPosition);
            // }
        }
        // *self.text() = text;
        
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

    pub fn display(&mut self, display: bool) { self.1.display(display) }
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
