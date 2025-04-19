use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Row, Stack, Column, Offset, Size, Padding};
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
            Column(16.0, Offset::Center, Size::Fit, Padding(16.0, 64.0, 16.0, 64.0)),
            Text::new(ctx, usd, TextStyle::Heading, font_size, TextAlign::Left),
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
            Some(err) => (Some(Icon::new(ctx, "error", color, 24.0)), TextStyle::Error, err),
            None => (None, TextStyle::Secondary, btc)
        };

        SubText(
            Row::center(8.0),
            icon, Text::new(ctx, text, style, font_size, TextAlign::Left)
        )
    }
}


#[derive(Debug, Component)]
pub struct AmountInput(Stack, AmountInputContent);
impl Events for AmountInput {}

impl AmountInput {
    pub fn new(ctx: &mut Context) -> Self {
        AmountInput (
            Stack(Offset::Center, Offset::Center, Size::Fit, Size::fill(), Padding::default()),
            AmountInputContent::new(ctx)
        )
    }
}

#[derive(Debug, Component)]
pub struct AmountInputContent(Column, Display, SubText);

impl AmountInputContent {
    pub fn new(ctx: &mut Context) -> Self {
        let subtext = if !crate::config::IS_MOBILE {"Type dollar amount."} else {"0.00001234 BTC"};
        AmountInputContent (
            Column(16.0, Offset::Center, Size::Fit, Padding(16.0, 64.0, 16.0, 64.0)),
            Display::new(ctx),
            SubText::new(ctx, subtext, None), 
        )
    }
}

impl Events for AmountInputContent {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(KeyboardEvent{state: KeyboardState::Pressed, key}) = event.downcast_ref() {
            let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
            let mut t = self.1.amount().text.replace(",", ""); 
            let mut digit_count = t.chars().filter(|ch| *ch != '.' && *ch != '$' && *ch != ',').count();

            match key {
                Key::Named(NamedKey::Delete | NamedKey::Backspace) if !t.is_empty() => {
                    match t.as_str() {
                        "$0" => {},
                        _ if t.len() <= 2 => t = "$0".to_string(),
                        _ => { t = t[..t.len() - 1].to_string(); }
                    }
                },
                Key::Character(c) => {
                    let character = c.to_string().chars().next().unwrap();

                    if digit_count < 8 {
                        if character.is_ascii_digit() {
                            match t.as_str() {
                                "$0" => t = format!("${}", character),
                                _ => match t.find('.') {
                                    Some(i) if t[i + 1..].len() < 2 => t.push(character),
                                    None => t.push(character),
                                    _ => {}
                                }
                            }
                        }

                        if character == '.' && !t.contains('.') {
                            let projected_len = digit_count + 2;
                            if projected_len <= 8 {
                                t.push('.');
                            }
                        }
                    }
                },
                _ => {}
            }

            self.1.zeros().text = match t.find('.') {
                Some(i) => match t[i + 1..].len() {
                    0 => "00",
                    1 => "0",
                    _ => "",
                }.to_string(),
                None => "".to_string(),
            };

            digit_count = t.chars().filter(|ch| *ch != '.' && *ch != '$' && *ch != ',').count();

            let (dollars, cents) = match t.find('.') {
                Some(i) => (&t[1..i], Some(&t[i..])), // skip '$'
                None => (&t[1..], None),
            };

            let dollars_chars: Vec<char> = dollars.chars().rev().collect();
            let mut formatted = String::new();

            for (i, ch) in dollars_chars.iter().enumerate() {
                if i > 0 && i % 3 == 0 {
                    formatted.push(',');
                }
                formatted.push(*ch);
            }

            let formatted_dollars: String = formatted.chars().rev().collect();
            let t_formatted = format!("${}{}", formatted_dollars, cents.unwrap_or(""));

            let size = match digit_count + self.1.zeros().text.len() {
                0..=5 => font_size.title,
                _ => font_size.h1
            };

            self.1.amount().font_size = size;
            self.1.amount().line_height = size * 1.25;
            self.1.zeros().font_size = size;
            self.1.zeros().line_height = size * 1.25;
            self.1.amount().text = t_formatted.clone();

            self.2.2.text = match t_formatted.as_str() {
                "$0" | "$0." | "$0.0" | "$0.00" if !crate::config::IS_MOBILE => "Type dollar amount.".to_string(),
                _ => "0.00001234 BTC".to_string()
            };

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
            Row::center(0.0),
            Text::new(ctx, "$0", TextStyle::Label(mc), font_size, TextAlign::Left),
            Text::new(ctx, "", TextStyle::Label(dc), font_size, TextAlign::Left),
        )
    }

    pub fn amount(&mut self) -> &mut BasicText {&mut self.1}
    pub fn zeros(&mut self) -> &mut BasicText {&mut self.2}
}