use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::shapes::{Rectangle, RoundedRectangle};
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::components::button::{IconButton, ButtonState};
use crate::layout::{Stack, Bin, Column, Row, Offset, Size, Padding};
use crate::PelicanUI;

use std::sync::mpsc::{self, Receiver, Sender};

#[cfg(target_os = "ios")]
extern "C" {
    fn trigger_haptic();
    // fn get_application_support_dir() -> *const std::os::raw::c_char;
}
#[cfg(target_os = "ios")]
fn vibrate()  {
    unsafe {
        trigger_haptic();
    }
}

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
pub struct IconButtonRow(Row, IconButton, IconButton, IconButton, IconButton);
impl Events for IconButtonRow {}

impl IconButtonRow {
    pub fn new(ctx: &mut Context) -> Self {
        IconButtonRow(
            Row(16, Offset::Start, Size::Fit, Padding(12, 12, 12, 12)), 
            IconButton::keyboard(ctx, "emoji", |_ctx: &mut Context| ()),
            IconButton::keyboard(ctx, "gif", |_ctx: &mut Context| ()),
            IconButton::keyboard(ctx, "photos", |_ctx: &mut Context| ()),
            IconButton::keyboard(ctx, "camera", |_ctx: &mut Context| ()),
        )
    }
}

#[derive(Component, Debug)]
pub struct KeyboardContent(Column, KeyboardHeader, KeyboardRow, KeyboardRow, KeyboardRow, KeyboardRow, #[skip] Receiver<u8>);

impl KeyboardContent {
    pub fn new(ctx: &mut Context) -> Self {
        let (sender, receiver) = mpsc::channel();
        KeyboardContent(
            Column(0, Offset::Center, Size::Fit, Padding(8, 8, 8, 8)),
            KeyboardHeader::new(ctx),
            KeyboardRow::top(ctx),
            KeyboardRow::middle(ctx),
            KeyboardRow::bottom(ctx, sender.clone()),
            KeyboardRow::modifier(ctx, sender),
            receiver
        )
    }

    pub fn update(&mut self) {
        let caps = self.4.capslock().as_mut().unwrap().status().clone();
        let page = self.5.paginator().as_mut().unwrap().status().clone();
        self.2.update(top_keys(page), caps);
        self.3.update(mid_keys(page), caps);
        self.4.update(bot_keys(page), caps);
        self.5.update(vec![], caps);
    }
}

impl Events for KeyboardContent {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            match self.6.try_recv() {
                Ok(0) => {println!("CAPSLOCK"); self.update();},
                Ok(1) => {println!("PAGINATOR"); self.update();},
                _ => {}
            }
            
            true
        } else {true}
    }
}

#[derive(Component, Debug)]
pub struct KeyRow(Row, Vec<Key>);
impl Events for KeyRow {}

impl KeyRow {
    pub fn new(ctx: &mut Context, keys: Vec<&'static str>) -> Self {
        let keys = keys.iter().map(|k| Key::character(ctx, k)).collect();
        KeyRow(Row::center(0), keys)
    }

    pub fn keys(&mut self) -> &mut Vec<Key> {&mut self.1}
}

#[derive(Component, Debug)]
pub struct KeyboardRow(Row, Option<Capslock>, Option<Paginator>, Option<KeyRow>, Option<Key>, Option<Key>);
// Capslock, Paginator, Character Row, Spacebar, Return
impl Events for KeyboardRow {}

impl KeyboardRow {
    fn top(ctx: &mut Context) -> Self {
        let key_row = KeyRow::new(ctx, top_keys(0));
        KeyboardRow(Row::center(0), None, None, Some(key_row), None, None)
    }

    fn middle(ctx: &mut Context) -> Self {
        let key_row = KeyRow::new(ctx, mid_keys(0));
        KeyboardRow(Row::center(0), None, None, Some(key_row), None, None)
    }

    fn bottom(ctx: &mut Context, sender: Sender<u8>) -> Self {
        let capslock = Capslock::new(ctx, sender);
        let backspace = Key::backspace(ctx);
        let key_row = KeyRow::new(ctx, bot_keys(0));
        KeyboardRow(Row::center(6), Some(capslock), None, Some(key_row), None, Some(backspace))
    }

    fn modifier(ctx: &mut Context, sender: Sender<u8>) -> Self {
        let paginator = Paginator::new(ctx, sender);
        let spacebar = Key::spacebar(ctx);
        let newline = Key::newline(ctx);
        KeyboardRow(Row::center(6), None, Some(paginator), None, Some(spacebar), Some(newline))
    }

    fn update(&mut self, new: Vec<&'static str>, caps_on: bool) {
        let format_text = |text: &str| {
            match caps_on {
                true => text.to_uppercase(),
                false => text.to_lowercase(),
            }
        };
    
        if let Some(spacebar) = &mut self.4 {
            if let Some(text) = spacebar.1.character().get_text().as_mut() {
                text.text = format_text("space");
            }
        }
    
        if let Some(newline) = &mut self.5 {
            if let Some(text) = newline.1.character().get_text().as_mut() {
                text.text = format_text("return");
            }
        }

        if let Some(keys) = &mut self.3 {
            keys.keys().iter_mut().enumerate().for_each(|(i, k)| {
                if let Some(text) = k.1.character().get_text().as_mut() {
                    text.text = format_text(new[i]);
                }
            });
        }
    }

    fn capslock(&mut self) -> &mut Option<Capslock> {&mut self.1}
    fn paginator(&mut self) -> &mut Option<Paginator> {&mut self.2}
}

#[derive(Component, Debug)]
pub struct Key(Stack, KeyContent, #[skip] ButtonState);

impl Key {
    pub fn character(ctx: &mut Context, character: &'static str) -> Self {
        let character = KeyCharacter::char(ctx, character);
        let content = KeyContent::new(ctx, 33, Offset::End, character);
        Key(Stack::default(), content, ButtonState::Default)
    }

    pub fn spacebar(ctx: &mut Context) -> Self {
        let character = KeyCharacter::text(ctx, "space");
        let content = KeyContent::new(ctx, u32::MAX, Offset::Center, character);
        Key(Stack::default(), content, ButtonState::Default)
    }

    pub fn backspace(ctx: &mut Context) -> Self {
        let character = KeyCharacter::icon(ctx, "backspace");
        let content = KeyContent::new(ctx, 42, Offset::Center, character);
        Key(Stack::default(), content, ButtonState::Default)
    }

    pub fn newline(ctx: &mut Context) -> Self {
        let character = KeyCharacter::text(ctx, "return");
        let content = KeyContent::new(ctx, 92, Offset::Center, character);
        Key(Stack::default(), content, ButtonState::Default)
    }

    pub fn content(&mut self) -> &mut KeyContent {&mut self.1}
}

impl Events for Key {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            let colors = ctx.get::<PelicanUI>().theme.colors;
            self.2 = handle_state(ctx, self.2, *event);

            *self.1.background() = match self.2 {
                ButtonState::Default => colors.shades.lighten,
                ButtonState::Pressed => colors.shades.lighten2,
                _ => colors.shades.lighten,
            };

            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.2 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => {
                        #[cfg(target_os = "ios")]
                        vibrate();
                        // TRIGGER KEYBOARD EVENTS
                        // if let Some(on_click) = self.3 {
                        //     (on_click)(ctx);
                        // }
                    },
                    _ => {}
                }
            }

            false
        } else {true}

    }
}



#[derive(Component)]
pub struct Capslock(Stack, KeyContent, #[skip] ButtonState, #[skip] bool, #[skip] Sender<u8>);

impl Capslock {
    fn new(ctx: &mut Context, sender: Sender<u8>) -> Self {
        let character = KeyCharacter::icon(ctx, "capslock");
        let content = KeyContent::new(ctx, 42, Offset::Center, character);
        Capslock(Stack::default(), content, ButtonState::Default, false, sender)
    }
    pub fn content(&mut self) -> &mut KeyContent {&mut self.1}
    pub fn status(&mut self) -> &mut bool {&mut self.3}
}

impl std::fmt::Debug for Capslock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capslock(...)")
    }
}

impl Events for Capslock {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            let colors = ctx.get::<PelicanUI>().theme.colors;
            self.2 = handle_state(ctx, self.2, *event);

            *self.1.background() = match self.2 {
                ButtonState::Default => colors.shades.lighten,
                ButtonState::Pressed => colors.shades.lighten2,
                _ => colors.shades.lighten,
            };

            if event.state == MouseState::Pressed && event.position.is_some() {
                self.3 = !self.3;
                let icon = if self.3 { "capslock_on" } else { "capslock" };
                *self.1.character() = KeyCharacter::icon(ctx, icon);
            }

            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.2 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => {
                        #[cfg(target_os = "ios")]
                        vibrate();
                        self.4.send(0).unwrap();
                    }
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

#[derive(Component)]
pub struct Paginator(Stack, KeyContent, #[skip] ButtonState, #[skip] u32, #[skip] Sender<u8>);

impl Paginator {
    fn new(ctx: &mut Context, sender: Sender<u8>) -> Self {
        let character = KeyCharacter::paginator(ctx, 0);
        let content = KeyContent::new(ctx, 92, Offset::Center, character);
        Paginator(Stack::default(), content, ButtonState::Default, 0, sender)
    }

    pub fn content(&mut self) -> &mut KeyContent {&mut self.1}
    pub fn status(&mut self) -> &mut u32 {&mut self.3}
}

impl std::fmt::Debug for Paginator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paginator(...)")
    }
}

impl Events for Paginator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            let colors = ctx.get::<PelicanUI>().theme.colors;
            self.2 = handle_state(ctx, self.2, *event);

            *self.1.background() = match self.2 {
                ButtonState::Default => colors.shades.lighten,
                ButtonState::Pressed => colors.shades.lighten2,
                _ => colors.shades.lighten,
            };

            if event.state == MouseState::Pressed && event.position.is_some() {
                #[cfg(target_os = "ios")]
                vibrate();

                let (highlight, dim) = (colors.text.heading, colors.text.secondary);
                let next = if self.3 == 2 { 0 } else { self.3 + 1 };
                self.3 = next;

                let styles = match next {
                    0 => (highlight, dim, dim),
                    1 => (dim, highlight, dim),
                    _ => (dim, dim, highlight),
                };

                self.1.character().2.as_mut().unwrap().color = styles.0;
                self.1.character().3.as_mut().unwrap().color = styles.1;
                self.1.character().4.as_mut().unwrap().color = styles.2;
            }

            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.2 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => self.4.send(1).unwrap(),
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

#[derive(Component, Debug)]
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

    pub fn background(&mut self) -> &mut Color {&mut self.1.shape().color}
    pub fn character(&mut self) -> &mut KeyCharacter {&mut self.2}
}

#[derive(Component, Debug)]
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


pub fn handle_state(_ctx: &mut Context, state: ButtonState, event: MouseEvent) -> ButtonState {
    match state {
        ButtonState::Default if event.position.is_some() => {
            match event.state {
                MouseState::Pressed => Some(ButtonState::Pressed),
                MouseState::Released => Some(ButtonState::Default),
                _ => None,
            }
        },
        ButtonState::Pressed => {
            match event.state {
                MouseState::Released => Some(ButtonState::Default),
                MouseState::Moved if event.position.is_none() => Some(ButtonState::Default),
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
