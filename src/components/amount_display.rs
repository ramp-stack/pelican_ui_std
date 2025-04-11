use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Row, Column, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct AmountDisplay(Column, BasicText, SubText);
impl Events for AmountDisplay {}

impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, err: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let font_size = match usd.len() {
            0..=4 => font_size.title,
            5..=7 => font_size.h1,
            _ => font_size.h2
        };

        AmountDisplay (
            Column(16, Offset::Center, Size::Fit, Padding(16, 64, 16, 64)),
            Text::new(ctx, usd, TextStyle::Heading, font_size),
            SubText::new(ctx, btc, err)
        )
    }
}

#[derive(Debug, Component)]
struct SubText(Row, Option<Image>, BasicText);
impl Events for SubText {}

impl SubText {
    fn new(ctx: &mut Context, btc: &'static str, err: Option<&'static str>) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (font_size, color) = (theme.fonts.size.lg, theme.colors.status.danger);
        let (icon, style, text) = match err {
            Some(err) => (Some(Icon::new(ctx, "error", color, 24)), TextStyle::Error, err),
            None => (None, TextStyle::Secondary, btc)
        };

        SubText(
            Row::center(8),
            icon, Text::new(ctx, text, style, font_size)
        )
    }
}

#[derive(Debug, Component)]
pub struct AmountInput(Column, Display, SubText);

impl AmountInput {
    pub fn new(ctx: &mut Context) -> Self {
        AmountInput (
            Column(16, Offset::Center, Size::Fit, Padding(16, 64, 16, 64)),
            Display::new(ctx),
            SubText::new(ctx, "Type dollar amount.", None), 
        )
    }
}

impl Events for AmountInput {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
        } else if let Some(KeyboardEvent{state: KeyboardState::Pressed, key}) = event.downcast_ref() {
            let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
            let mut t = self.1.amount().text.clone();

            match key {
                Key::Named(NamedKey::Delete | NamedKey::Backspace) if !t.is_empty() => {
                    match t.as_str() {
                        "$0" => {},
                        _ if t.len() <= 2 => t = "$0".to_string(),
                        _ => t = t[..t.len() - 1].to_string(),
                    }
                },
                Key::Character(c) => {
                    let character = c.to_string().chars().next().unwrap();
                    let has_dec = t.contains('.');
                    let char_count = t.chars().filter(|ch| *ch != '.').count();
                    let zero_count = self.1.zeros().text.len();
                
                    if (has_dec && char_count < 10) || !has_dec && char_count < 8 {
                        if character.is_ascii_digit() {
                            match t.as_str() {
                                "$0" => t = "$".to_owned()+&character.to_string(),
                                _ => match t.find('.') {
                                    Some(i) if t[i + 1..].len() < 2 => t += &character.to_string(),
                                    None => t += &character.to_string(),
                                    _ => {}
                                }
                            }
                        }
                        if character == '.' && !t.contains('.') {
                            let projected_len = t.chars().filter(|ch| *ch != '.').count() + 2 + self.1.zeros().text.len();
                            if projected_len <= 8 {
                                t += ".";
                            }
                        }
                        
                    }
                },
                _ => {}
            };

            match t.find('.') {
                Some(i) => {
                    self.1.zeros().text = match &t[i + 1..].len() {
                        0 => "00",
                        1 => "0",
                        _ => "",
                    }.to_string();
                },
                None => self.1.zeros().text = "".to_string()
            }
            let size = match t.len() + self.1.zeros().text.len() {
                0..=5 => font_size.title,
                _ => font_size.h1,
            };
            self.1.amount().font_size = size;
            self.1.amount().line_height = (size as f32*1.25) as u32;
            self.1.zeros().font_size = size;
            self.1.zeros().line_height = (size as f32*1.25) as u32;
            self.1.amount().text = t;
        }
        true
    }
}

#[derive(Debug, Component)]
struct Display(Row, BasicText, BasicText);
impl Events for Display {}
impl Display {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let font_size = theme.fonts.size.title;
        let (mc, dc) = (theme.colors.text.heading, theme.colors.text.secondary);
        Display (
            Row::center(0),
            Text::new(ctx, "$0", TextStyle::Label(mc), font_size),
            Text::new(ctx, "", TextStyle::Label(dc), font_size),
        )
    }

    pub fn amount(&mut self) -> &mut BasicText {&mut self.1}
    pub fn zeros(&mut self) -> &mut BasicText {&mut self.2}
}