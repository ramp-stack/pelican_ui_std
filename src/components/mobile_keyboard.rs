use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::shapes::{Rectangle, RoundedRectangle};
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::components::button::{Button, IconButton, ButtonState};
use crate::layout::{Stack, Bin, Column, Row, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct MobileKeyboard(Stack, Rectangle, KeyboardContent);
impl Events for MobileKeyboard {}

impl MobileKeyboard {
    pub fn new(ctx: &mut Context) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.background.secondary;
        MobileKeyboard(
            Stack(
                Offset::Start, Offset::Start, 
                Size::Fill(200, u32::MAX), Size::custom(|heights: Vec<(u32, u32)>| heights[1]), 
                Padding::default()
            ), 
            Rectangle::new(color),
            KeyboardContent::new(ctx)
        )
    }
}

#[derive(Debug, Component)]
pub struct KeyboardHeader(Column, IconButtonRow, Bin<Stack, Rectangle>);
impl Events for KeyboardHeader {}

impl KeyboardHeader {
    pub fn new(ctx: &mut Context) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.outline.secondary;
        KeyboardHeader(
            Column::center(0),
            IconButtonRow::new(ctx),
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Fit, Size::Static(1), Padding::default()), 
                Rectangle::new(color)
            )
        )
    }
}

#[derive(Debug, Component)]
pub struct IconButtonRow(Row, IconButton, IconButton, IconButton, IconButton);
impl Events for IconButtonRow {}

impl IconButtonRow {
    pub fn new(ctx: &mut Context) -> Self {
        IconButtonRow(
            Row(16, Offset::Start, Size::Fit, Padding(12, 12, 12, 12)), 
            IconButton::keyboard(ctx, "emoji", |ctx: &mut Context| ()),
            IconButton::keyboard(ctx, "gif", |ctx: &mut Context| ()),
            IconButton::keyboard(ctx, "photos", |ctx: &mut Context| ()),
            IconButton::keyboard(ctx, "camera", |ctx: &mut Context| ()),
        )
    }
}

#[derive(Debug, Component)]
pub struct KeyboardContent(Column, KeyboardHeader, KeyboardRow, KeyboardRow, KeyboardRow, KeyboardRow, #[skip] bool, #[skip] u8);

impl KeyboardContent {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, text_size) = (theme.colors, theme.fonts.size.xl);
        KeyboardContent(
            Column(0, Offset::Center, Size::Fit, Padding(8, 8, 8, 8)),
            KeyboardHeader::new(ctx),
            KeyboardRow::top(ctx),
            KeyboardRow::middle(ctx),
            KeyboardRow::bottom(ctx),
            KeyboardRow::modifier(ctx)
        )
    }

    pub fn capslock_state(&mut self) -> bool {
        if let KeyType::Capslock(on) = self.4.1.as_mut().unwrap().key_type() {*on} else {false}
    }

    pub fn paginator_page(&mut self) -> u32 {
        if let KeyType::Paginator(page) = self.5.1.as_mut().unwrap().key_type() {*page} else {0}
    }

    pub fn update(&mut self) {
        self.2.update(top_keys(self.7), self.6);
        self.3.update(mid_keys(self.7), self.6);
        self.4.update(bot_keys(self.7), self.6);
        self.5.update(vec![], self.6);
    }
}

impl Events for KeyboardContent {
    fn on_tick(&mut self, ctx: &mut Context) {
        let theme = &ctx.get::<PelicanUI>().theme;
        // match event.key {
        //     Key::Named(NamedKey::Delete | NamedKey::Backspace) => *t = if t.len()>0 {(&t[0..t.len() - 1]).to_string()} else {String::new()}, // delete char
        //     Key::Character(c) => *t += &c, // add character
        //     _ => {}
        // };
    }
}

#[derive(Debug, Component)]
pub struct KeyRow(Row, Vec<Key>);
impl Events for KeyRow {}

impl KeyRow {
    pub fn new(ctx: &mut Context, keys: Vec<&'static str>) -> Self {
        let keys = keys.iter().map(|k| Key::character(ctx, k)).collect();
        KeyRow(Row::center(0), keys)
    }

    pub fn keys(&mut self) -> &mut Vec<Key> {&mut self.1}
}

#[derive(Debug, Component)]
pub struct KeyboardRow(Row, Option<Capslock>, Option<Paginator>, Option<KeyRow>, Option<Key>, Option<Key>);
// Capslock, Paginator, Character Row, Spacebar, Return
impl Events for KeyboardRow {}

impl KeyboardRow {
    fn top(ctx: &mut Context) -> Self {
        let key_row = top_keys(0).map(|keys| KeyRow::new(ctx, keys));
        KeyboardRow(Row::center(0), None, None, key_row, None, None)
    }

    fn middle(ctx: &mut Context) -> Self {
        let key_row = mid_keys(0).map(|keys| KeyRow::new(ctx, keys));
        KeyboardRow(Row::center(0), None, None, key_row, None, None)
    }

    fn bottom(ctx: &mut Context) -> Self {
        let capslock = Capslock::new(ctx);
        let backspace = Key::backspace(ctx);
        let key_row = bot_keys(0).map(|keys| KeyRow::new(ctx, keys));
        KeyboardRow(Row::center(6), Some(capslock), None, key_row, Some(backspace))
    }

    fn modifier(ctx: &mut Context) -> Self {
        let paginator = Paginator(ctx);
        let spacebar = Key::spacebar(ctx);
        let newline = Key::newline(ctx);
        KeyboardRow(Row::center(6), Some(paginator), None, None, Some(backspace))
        Self::new(ctx, 6, None, Some(paginator), Some(spacebar), Some(newline))
    }

    fn update(&mut self, new: Vec<&'static str>, caps_on: bool) {
        let format_text = |text: &str| {
            match caps_on {
                true => text.to_uppercase(),
                false => text.to_lowercase(),
            }
        };
    
        if let Some(spacebar) = &mut self.2 {
            if let Some(text) = spacebar.1.character().get_text().as_mut() {
                *text.value() = format_text("space");
            }
        }
    
        if let Some(newline) = &mut self.4 {
            if let Some(text) = newline.1.character().get_text().as_mut() {
                *text.value() = format_text("return");
            }
        }

        if let Some(keys) = &mut self.3 {
            keys.keys().iter_mut().enumerate().for_each(|(i, k)| {
                if let Some(text) = k.1.character().get_text().as_mut() {
                    *text.value() = format_text(new[i]);
                }
            });
        }
    }

    fn keys(&mut self) -> &mut Option<KeyRow> {&mut self.3}
}

#[derive(Debug, Component)]
pub struct Key(Stack, KeyContent, #[skip] ButtonState, #[skip] KeyType);

impl Key {
    fn build(ctx: &mut Context, size: u32, offset: Offset, character: KeyCharacter, key_type: KeyType) -> Self {
        let content = KeyContent::new(ctx, size, offset, character);
        Key(Stack::default(), content, ButtonState::Default, key_type)
    }

    pub fn character(ctx: &mut Context, character: &'static str) -> Self {
        let content = KeyCharacter::char(ctx, character);
        Self::build(ctx, 33, Offset::End, content, KeyType::Character(character))
    }

    pub fn spacebar(ctx: &mut Context) -> Self {
        let content = KeyCharacter::text(ctx, "space");
        Self::build(ctx, u32::MAX, Offset::Center, content, KeyType::Character(" "))
    }

    pub fn backspace(ctx: &mut Context) -> Self {
        let content = KeyCharacter::icon(ctx, "backspace");
        Self::build(ctx, 42, Offset::Center, content, KeyType::Backspace)
    }

    pub fn newline(ctx: &mut Context) -> Self {
        let content = KeyCharacter::text(ctx, "return");
        Self::build(ctx, 92, Offset::Center, content, KeyType::Newline)
    }

    pub fn key_type(&mut self) -> &mut KeyType {&mut self.3}
    pub fn content(&mut self) -> &mut KeyContent {&mut self.1}
}

impl Events for Key {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        self.2 = handle_state(ctx, self.2, event);

        *self.1.background() = match self.2 {
            ButtonState::Default => colors.shades.lighten,
            _ => colors.shades.lighten2,
        };

        false
    }
}


#[derive(Debug, Component)]
pub struct Capslock(Stack, KeyContent, #[skip] ButtonState, #[skip] bool);

impl Capslock {
    fn new(ctx: &mut Context) -> Self {
        let character = KeyCharacter::icon(ctx, "capslock");
        let content = KeyContent::new(ctx, 42, Offset::Center, character);
        Key(Stack::default(), content, ButtonState::Default, false)
    }
    pub fn key_type(&mut self) -> &mut KeyType {&mut self.3}
    pub fn content(&mut self) -> &mut KeyContent {&mut self.1}
}

impl Events for Capslock {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        self.2 = handle_state(ctx, self.2, event);

        *self.1.background() = match self.2 {
            ButtonState::Default => colors.shades.lighten,
            _ => colors.shades.lighten2,
        };

        if event.state == MouseState::Pressed && event.position.is_some() {
            self.3 = !self.3;
            let icon = if self.3 { "capslock_on" } else { "capslock" };
            *self.1.character() = KeyCharacter::icon(ctx, icon);
        }

        false
    }
}


#[derive(Debug, Component)]
pub struct Paginator(Stack, KeyContent, #[skip] ButtonState, #[skip] u8);

impl Paginator {
    fn new(ctx: &mut Context) -> Self {
        let character = KeyCharacter::paginator(ctx, 0);
        let content = KeyContent::new(ctx, 92, Offset::Center, character);
        Key(Stack::default(), content, ButtonState::Default, 0)
    }

    pub fn key_type(&mut self) -> &mut KeyType {&mut self.3}
    pub fn content(&mut self) -> &mut KeyContent {&mut self.1}
}

impl Events for Paginator {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        self.2 = handle_state(ctx, self.2, event);

        *self.1.background() = match self.2 {
            ButtonState::Default => colors.shades.lighten,
            _ => colors.shades.lighten2,
        };

        if event.state == MouseState::Pressed && event.position.is_some() {
            let (highlight, dim) = (colors.text.heading, colors.text.secondary);
            let next = if p == 2 { 0 } else { p + 1 };

            let styles = match next {
                0 => (highlight, dim, dim),
                1 => (dim, highlight, dim),
                _ => (dim, dim, highlight),
            };

            self.3 = KeyType::Paginator(next);
            *self.1.character().2.as_mut().unwrap().color() = styles.0;
            *self.1.character().3.as_mut().unwrap().color() = styles.1;
            *self.1.character().4.as_mut().unwrap().color() = styles.2;
        }

        false
    }
}

#[derive(Debug, Component)]
pub struct KeyContent(Stack, RoundedRectangle, KeyCharacter);
impl Events for KeyContent {}

impl KeyContent {
    pub fn new(ctx: &mut Context, size: u32, offset: Offset, content: KeyCharacter) -> Self {
        KeyContent(
            Stack(Offset::Center, offset, Size::Fill(20, size), Size::Static(48), Padding(3, 6, 3, 6)),
            RoundedRectangle::new(0, 4, ctx.get::<PelicanUI>().theme.colors.shades.lighten),
            content
        )
    }

    pub fn background(&mut self) -> &mut Color {self.1.shape().color()}
    pub fn character(&mut self) -> &mut KeyCharacter {&mut self.2}
}

#[derive(Debug, Component)]
pub struct KeyCharacter(Row, Option<Image>, Option<BasicText>, Option<BasicText>, Option<BasicText>);
impl Events for KeyCharacter {}

impl KeyCharacter {
    pub fn char(ctx: &mut Context, key: &'static str) -> Self {
        let size = ctx.get::<PelicanUI>().theme.fonts.size.xl;
        KeyCharacter(
            Row(0, Offset::Center, Size::Fit, Padding(0, 0, 0, 10)),
            None,
            Some(Text::new(ctx, key, TextStyle::Keyboard, size)),
            None, None
        )
    }

    pub fn text(ctx: &mut Context, key: &'static str) -> Self {
        let size = ctx.get::<PelicanUI>().theme.fonts.size.md;
        KeyCharacter(Row::center(0), None, Some(Text::new(ctx, key, TextStyle::Keyboard, size)), None, None)
    }

    pub fn icon(ctx: &mut Context, i: &'static str) -> Self {
        let c = ctx.get::<PelicanUI>().theme.colors.text.heading;
        KeyCharacter(Row::center(0), Some(Icon::new(ctx, i, c, 36)), None, None, None)
    }

    pub fn paginator(ctx: &mut Context, page: u32) -> Self {
        let size = ctx.get::<PelicanUI>().theme.fonts.size.h2;
        let (highlight, dim) = (TextStyle::White, TextStyle::Secondary);

        let styles = match page {
            0 => (highlight, dim, dim),
            1 => (dim, highlight, dim),
            _ => (dim, dim, highlight),
        };

        KeyCharacter(
            Row::center(1),
            None,
            Some(Text::new(ctx, "•", styles.0, size)),
            Some(Text::new(ctx, "•", styles.1, size)),
            Some(Text::new(ctx, "•", styles.2, size)),
        )
    }

    pub fn get_text(&mut self) -> &mut Option<BasicText> {&mut self.2}
}


pub fn handle_state(ctx: &mut Context, state: ButtonState, event: MouseEvent) -> ButtonState {
    match state {
        ButtonState::Default if event.position.is_some() => {
            match event.state {
                MouseState::Pressed => Some(ButtonState::Selected),
                MouseState::Moved => Some(ButtonState::Selected),
                MouseState::Released => Some(ButtonState::Default)
            }
        },
        ButtonState::Selected => {
            match event.state {
                MouseState::Released => Some(ButtonState::Default),
                MouseState::Moved if event.position.is_some() => Some(ButtonState::Selected),
                MouseState::Moved if event.position.is_none() => Some(ButtonState::Default),
                MouseState::Pressed if event.position.is_some() => Some(ButtonState::Selected),
                MouseState::Pressed => Some(ButtonState::Default),
                _ => None,
            }
        },
        _ => None
    }.unwrap_or(state)
}


fn top_keys(page: u32) -> Vec<&'static str> {
    match page {
        0 => vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
        1 => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
        _ => vec!["[", "]", "{", "}", "(", ")", "<", ">", "+", "="]
    }
}

fn mid_keys(page: u32) -> Vec<&'static str> {
    match page {
        0 => vec!["a", "s", "d", "f", "g", "h", "j", "k", "l"],
        1 => vec!["/", "\\", "\"", "'", "~", ".", ",", "?", "!"],
        _ => vec!["-", ":", ";", "#", "%", "$", "&", "^", "*",]
    }  
}

fn bot_keys(page: u32) -> Vec<&'static str> {
    match page {
        0 => vec!["z", "x", "c", "v", "b", "n", "m"],
        1 => vec!["@", "|", "`", "˚", "€", "£", "¥"],
        _ => vec!["™", "©", "•", "¶", "€", "£", "¥"]
    }  
}