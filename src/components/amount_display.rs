use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::events::{SetActiveEvent, SetInactiveEvent};
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Row, Stack, Column, Offset, Size, Padding};
use crate::{PelicanUI, ElementID};

#[derive(Debug, Component)]
pub struct AmountDisplay(Column, BasicText, SubText);
 impl OnEvent for AmountDisplay {}
impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, _err: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;

        let font_size = match usd.len() {
            0..=4 => font_size.title,
            5..=7 => font_size.h1,
            _ => font_size.h2
        };

        AmountDisplay (
            Column(16.0, Offset::Center, Size::Fit, Padding(16.0, 64.0, 16.0, 64.0)),
            Text::new(ctx, usd, TextStyle::Heading, font_size, TextAlign::Left),
            SubText::new(ctx, btc)
        )
    }
}

#[derive(Debug, Component)]
struct SubText(Row, Option<Image>, BasicText, #[skip] bool);
 impl OnEvent for SubText {}

impl SubText {
    fn new(ctx: &mut Context, btc: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.lg;
        SubText(Row::center(8.0), None, Text::new(ctx, btc, TextStyle::Secondary, text_size, TextAlign::Left), true)
    }

    fn set_error(&mut self, ctx: &mut Context, err: &'static str) {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, text_size) = (theme.colors.status.danger, theme.fonts.size.lg);
        self.1 = Some(Icon::new(ctx, "error", color, 24.0));
        self.2 = Text::new(ctx, err, TextStyle::Error, text_size, TextAlign::Left);
    }

    fn set_subtext(&mut self, ctx: &mut Context, txt: &'static str) {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.lg;
        self.1 = None;
        self.2 = Text::new(ctx, txt, TextStyle::Secondary, text_size, TextAlign::Left);
    }
}


#[derive(Debug, Component)]
pub struct AmountInput(Stack, AmountInputContent, #[skip] Option<Vec<ElementID>>);

impl AmountInput {
    pub fn new(ctx: &mut Context, to_disable: Option<Vec<ElementID>>) -> Self {
        AmountInput (
            Stack(Offset::Center, Offset::Center, Size::Fit, Size::fill(), Padding::default()),
            AmountInputContent::new(ctx), to_disable
        )
    }
}

 impl OnEvent for AmountInput {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            if let Some(ids) = &self.2 {
                match self.1.2.3 {
                    false => ids.into_iter().for_each(|id| ctx.trigger_event(SetActiveEvent(*id))),
                    true => ids.into_iter().for_each(|id| ctx.trigger_event(SetInactiveEvent(*id)))
                }
            }
        }
        true
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
            SubText::new(ctx, subtext), 
        )
    }
}

 impl OnEvent for AmountInputContent {
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

            let min = 2.14; // MINIMUM EXAMPLE
            let max = 80.03; // MAXIMUM EXAMPLE

            match t_formatted.as_str() {
                "$0" | "$0." | "$0.0" | "$0.00" if !crate::config::IS_MOBILE => {
                    println!("IS ZERO< CANNOT CONTINUE");
                    self.2.set_subtext(ctx, "Type dollar amount.");
                    self.2.3 = true;
                },
                _ => {
                    if t_formatted.trim_start_matches('$').parse::<f64>().unwrap_or(0.0) < min {
                        println!("IS MIN< CANNOCT ONTINUE");
                        self.2.set_error(ctx, "$2.18 Minimum.");
                        self.2.3 = true;
                    } else if t_formatted.trim_start_matches('$').parse::<f64>().unwrap_or(0.0) > max {
                        println!("IS MAX< CANNOT CONTINUE");
                        self.2.set_error(ctx, "$80.14 Maximum.");
                        self.2.3 = true;
                    } else {
                        println!("IS GUT< CAN WILL CONTINUE");
                        self.2.set_subtext(ctx, "0.00001234 BTC");
                        self.2.3 = false;
                    }
                }
            }
            
        }  
        true
    }
}

#[derive(Debug, Component)]
struct Display(Row, BasicText, BasicText);
 impl OnEvent for Display {}
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