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
            Stack(Offset::Start, Offset::Start, Size::Fit, Size::Static(326), Padding::default()), 
            Rectangle::new(color),
            KeyboardContent::new(ctx)
        )
    }
}

#[derive(Debug, Component)]
pub struct KeyboardContent(Column, KeyboardHeader, KeyRows);
impl Events for KeyboardContent {}

impl KeyboardContent {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (colors, text_size) = (theme.colors, theme.fonts.size.xl);
        KeyboardContent(
            Column::center(16),
            KeyboardHeader::new(ctx),
            KeyRows::new(ctx)
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
pub struct KeyRows(Column, KeyRow, KeyRow, BottomRow, ModifierRow);
impl Events for KeyRows {}

impl KeyRows{
    pub fn new(ctx: &mut Context) -> Self {
        let top = vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"];
        let middle = vec!["a", "s", "d", "f", "g", "h", "j", "k", "l"];
        let bottom = vec!["z", "x", "c", "v", "b", "n", "m"];
        KeyRows (
            Column::center(12), 
            KeyRow::new(ctx, top),
            KeyRow::new(ctx, middle),
            BottomRow::new(ctx, bottom),
            ModifierRow::new(ctx)
        )
    }
}


#[derive(Debug, Component)]
pub struct KeyRow(Row, Vec<Key>);
impl Events for KeyRow {}

impl KeyRow {
    pub fn new(ctx: &mut Context, keys: Vec<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.xl;
        let keys = keys.iter().map(|k| Key::character(ctx, k)).collect();
        KeyRow(Row(6, Offset::Center, Size::Fit, Padding(4, 0, 4, 0)), keys)
    }
}

#[derive(Debug, Component)]
pub struct BottomRow(Row, Key, KeyRow, Key);
impl Events for BottomRow {}

impl BottomRow {
    pub fn new(ctx: &mut Context, names: Vec<&'static str>) -> Self {
        BottomRow(
            Row(8, Offset::Center, Size::Fit, Padding(4, 0, 4, 0)),
            Key::capslock(ctx),
            KeyRow::new(ctx, names),
            Key::backspace(ctx)
        )
    }
}

#[derive(Debug, Component)]
pub struct ModifierRow(Row, Key, Key, Key);
impl Events for ModifierRow {}

impl ModifierRow {
    pub fn new(ctx: &mut Context) -> Self {
        ModifierRow(
            Row(12, Offset::Center, Size::Fit, Padding(4, 0, 4, 0)),
            Key::paginator(ctx),
            Key::spacebar(ctx),
            Key::newline(ctx),
        )
    }
}


#[derive(Debug, Component)]
pub struct Key(Stack, RoundedRectangle, KeyContent, #[skip] &'static str, #[skip] ButtonState);

impl Key {
    pub fn new(
        ctx: &mut Context,
        width: Size, 
        key: Option<&'static str>, 
        icon: Option<&'static str>,
        padding: Option<u32>,
        text_size: Option<u32>,
        on_click: &'static str,
    ) -> Self {
        let offset = if padding.is_some() {Offset::End} else {Offset::Center};
        Key(
            Stack(Offset::Center, offset, width, Size::Static(48), Padding::default()),
            RoundedRectangle::new(0, 4, ctx.get::<PelicanUI>().theme.colors.shades.lighten),
            KeyContent::new(ctx, key, icon, text_size, padding), on_click, ButtonState::Default,
        )
    }

    pub fn character(ctx: &mut Context, character: &'static str) -> Self {
        let font_size = Some(ctx.get::<PelicanUI>().theme.fonts.size.xl);
        Self::new(ctx, Size::Fit, Some(character), None, Some(12), font_size, "key")
    }

    pub fn backspace(ctx: &mut Context) -> Self {
        Self::new(ctx, Size::Static(42), None, Some("backspace"), None, None, "backspace")
    }

    pub fn capslock(ctx: &mut Context) -> Self {
        Self::new(ctx, Size::Static(42), None, Some("capslock"), None, None, "capslock")
    }

    pub fn newline(ctx: &mut Context) -> Self {
        let font_size = Some(ctx.get::<PelicanUI>().theme.fonts.size.md);
        Self::new(ctx, Size::Static(92), Some("return"), None, None, font_size, "newline")
    }

    pub fn paginator(ctx: &mut Context) -> Self {
        let font_size = Some(ctx.get::<PelicanUI>().theme.fonts.size.xl);
        Self::new(ctx, Size::Static(92), Some("•••"), None, None, font_size, "switched")
    }

    pub fn spacebar(ctx: &mut Context) -> Self {
        let font_size = Some(ctx.get::<PelicanUI>().theme.fonts.size.md);
        Self::new(ctx, Size::Fit, Some("space"), None, None, font_size, "space")
    }
}

impl Events for Key {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        if event.position.is_some() {println!("MOUSE EVENT: {:?}", self.4);}
        
        if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
            println!("Pressed: {:?}", self.3);
        }
        self.4 = match self.4 {
            ButtonState::Default if event.position.is_some() => ButtonState::Selected,
            ButtonState::Selected => ButtonState::Default,
            _ => self.4
        };
        *self.1.shape().color() = match self.4 {
            ButtonState::Default => colors.shades.lighten,
            ButtonState::Hover => colors.shades.lighten2,
            ButtonState::Selected => colors.shades.lighten2,
            ButtonState::Disabled => colors.shades.lighten,
        };
        false
    }
}

#[derive(Debug, Component)]
pub struct KeyContent(Stack, Option<BasicText>, Option<Icon>);
impl Events for KeyContent {}

impl KeyContent {
    pub fn new(
        ctx: &mut Context, 
        key: Option<&'static str>, 
        icon: Option<&'static str>, 
        text_size: Option<u32>,
        p: Option<u32>,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let p = if let Some(p) = p {p} else {0};
        KeyContent (
            Stack(Offset::Center, Offset::Center, Size::Fit, Size::Fit, Padding(0, 0, 0, p)),
            key.map(|k| Text::new(ctx, k, TextStyle::White, text_size.unwrap())),
            icon.map(|i| Icon::new(ctx, i, color, 36))
        )
    }
}



// let top = match page {
//     0 => vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
//     1 => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
//     _ => vec!["[", "]", "{", "}", "(", ")", "<", ">", "+", "="]
// };

// let middle = match page {
//     0 => vec!["a", "s", "d", "f", "g", "h", "j", "k", "l"],
//     1 => vec!["-", "/", ":", ";", "#", "%", "$", "&", "@", "\""],
//     _ => vec!["_", "\\", "|", "~", "^", "*", "€", "£", "¥", "•"]
// };

// let bottom = match page {
//     0 => vec!["z", "x", "c", "v", "b", "n", "m"],
//     1 => vec![".", ",", "?", "!", "'"],
//     _ => vec![".", ",", "?", "!", "'"]
// };


// pub struct NativeKeyboard();

// impl ComponentBuilder for NativeKeyboard {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let page_index = 1;
//         let caps = false;

//         let (top_row, middle_row, bottom_row) = generate_rows(page_index, caps);

//         let paginator_key = |i: u32| if i == 0 { "•--" }  else if i == 1 { "-•-" } else { "--•" };
    
//         Stack {padding: ZERO, align: Align::Top, children: vec![
//             (Child!(Shape(ShapeType::Rectangle(393, 300), COLORS.background.secondary, None)), ZERO),
//             (Child!(Column { padding: ZERO, spacing: 0, align: Align::Center, children: vec![
//                 (Child!(Row { padding: Vec2::new(12, 12), align: Align::Left, spacing: 16, children: vec![
//                     (icon(36), false),
//                     (icon(36), false),
//                     (icon(36), false),
//                     (icon(36), false),
//                     (icon(36), false),
//                     (Child!(Expand(false, 1, COLORS.background.secondary)), true),
//                     (icon(36), false)
//                 ]}), false),
//                 (Child!(Expand(false, 1, COLORS.outline.secondary)), false),
//                 (Child!(Padding(Vec2::new(5, 5), COLORS.background.secondary)), false),
//                 (Child!(Column {spacing: 12, padding: Vec2::new(5, 5), align: Align::Center, children: vec![
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: top_row}), true),
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: middle_row}), true),
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: vec![
//                         (Child!(Key("capslock")), false),
//                         (Child!(Row{spacing: 6, padding: Vec2::new(8, 0), align: Align::Center, children: bottom_row}), true),
//                         (Child!(Key("backspace")), false)
//                     ]}), false),
//                     (Child!(Row{spacing: 6, padding: Vec2::new(4, 0), align: Align::Left, children: vec![
//                         (Child!(Key(paginator_key(page_index))), false),
//                         (Child!(Key("space")), true),
//                         (Child!(Key("return")), false)
//                     ]}), false),
//                 ]}), false)
//             ]}), ZERO)
//         ]}.build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct Key(&'static str);

// impl ComponentBuilder for Key {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {

//         let lg_key_text = |c: &'static str, ctx: &mut Context| 
//             Child!(Text::primary_white(ctx, c, TextSize::xl()));

//         let dk_key_text = |c: &'static str, ctx: &mut Context| 
//             Child!(Text::secondary(ctx, c, TextSize::xl()));

//         let sm_key_text = |c: &'static str, ctx: &mut Context| 
//             Child!(Text::primary_white(ctx, c, TextSize::md()));
    
//         let mut paginator_key = |main_index: usize| {
//             Child!(Row {spacing: 2, padding: ZERO, align: Align::Left, children: vec![
//                 (if main_index == 0 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }, false),
//                 (if main_index == 1 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }, false),
//                 (if main_index == 2 { lg_key_text("•", ctx) } else { dk_key_text("•", ctx) }, false)
//             ]})
//         };

//         let key_background = |max: Option<u32>| {
//             match max.is_some() {
//                 true => Child!(Shape(ShapeType::Rectangle(max.unwrap(), 42), COLORS.outline.secondary, None)),
//                 false => Child!(Expand(false, 42, COLORS.outline.secondary))
//             }
//         };
        
//         let (content, offset, max_width) = match self.0 {
//             "•--" => (paginator_key(0), 6, Some(92)),
//             "-•-" => (paginator_key(1), 6, Some(92)),
//             "--•" => (paginator_key(2), 6, Some(92)),
//             "space" => (sm_key_text(self.0, ctx), 11, None),
//             "return" => (sm_key_text(self.0, ctx), 11, Some(92)),
//             _ => match self.0.len() <= 3 {
//                 true => (lg_key_text(self.0, ctx), 4, None),
//                 false =>(icon(32), 5, Some(44))
//             }
//         };

//         Child!(Stack {padding: ZERO, align: Align::Top, children: vec![
//             (key_background(max_width), Vec2::new(0, 0)),
//             (content, Vec2::new(0, offset))
//         ]}).build_children(ctx, max_size)
//     }
    

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }


// fn generate_rows(page: u32, caps: bool) -> (
//     Vec<(Box<dyn ComponentBuilder>, bool)>, 
//     Vec<(Box<dyn ComponentBuilder>, bool)>, 
//     Vec<(Box<dyn ComponentBuilder>, bool)>
// ) {
//     let top_row = match page {
//         0 => vec!["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
//         1 => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
//         _ => vec!["[", "]", "{", "}", "(", ")", "<", ">", "+", "="]
//     };

//     let mid_row = match page {
//         0 => vec!["a", "s", "d", "f", "g", "h", "j", "k", "l"],
//         1 => vec!["-", "/", ":", ";", "#", "%", "$", "&", "@", "\""],
//         _ => vec!["_", "\\", "|", "~", "^", "*", "€", "£", "¥", "•"]
//     };

//     let bot_row = match page {
//         0 => vec!["z", "x", "c", "v", "b", "n", "m"],
//         1 => vec![".", ",", "?", "!", "'"],
//         _ => vec![".", ",", "?", "!", "'"]
//     };

//     (
//         str_to_key(top_row, caps), 
//         str_to_key(mid_row, caps), 
//         str_to_key(bot_row, caps)
//     )
// }

// pub fn str_to_key(keys: Vec<&'static str>, caps: bool) -> Vec<(Box<dyn ComponentBuilder>, bool)> {
//     keys.iter().map(|key| {
//         let key = if caps { key.to_string().to_uppercase() } else { key.to_string() };
//         (Child!(Key(Box::leak(key.into_boxed_str()))), true)
//     }).collect()
// }

// fn icon(s: u32) -> Box<dyn ComponentBuilder> {
//     Child!(Shape(ShapeType::Rectangle(s, s), "ffffff", None))
// }