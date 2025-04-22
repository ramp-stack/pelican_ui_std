use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;

use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::events::{SetActiveEvent, SetInactiveEvent};
use crate::layout::{Column, Offset, Padding, Row, Size, Stack};
use crate::{ElementID, PelicanUI};

#[derive(Debug, Component)]
pub struct AmountDisplay(Column, BasicText, SubText);
impl OnEvent for AmountDisplay {}

impl AmountDisplay {
    pub fn new(ctx: &mut Context, usd: &'static str, btc: &'static str, _err: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        let font_size = if usd.len() <= 5 { font_size.title } else { font_size.h1 };

        AmountDisplay (
            Column(16.0, Offset::Center, Size::Fit, Padding(16.0, 64.0, 16.0, 64.0)),
            Text::new(ctx, usd, TextStyle::Heading, font_size, Align::Left),
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
        SubText(Row::center(8.0), None, Text::new(ctx, btc, TextStyle::Secondary, text_size, Align::Left), true)
    }

    fn set_error(&mut self, ctx: &mut Context, err: &'static str) {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, text_size) = (theme.colors.status.danger, theme.fonts.size.lg);
        self.1 = Some(Icon::new(ctx, "error", color, 24.0));
        self.2 = Text::new(ctx, err, TextStyle::Error, text_size, Align::Left);
    }

    fn set_subtext(&mut self, ctx: &mut Context, txt: &'static str) {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.lg;
        self.1 = None;
        self.2 = Text::new(ctx, txt, TextStyle::Secondary, text_size, Align::Left);
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
            // Get font sizes from theme
            let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
            // Remove commas from input string
            let mut t = self.1.amount().text().replace(",", "");
            // Count digits (excluding dots and commas)
            let mut digit_count = t.chars().filter(|ch| ch.is_ascii_digit()).count();

            // Handle key input
            match key {
                // Handle delete or backspace when input isn't empty
                Key::Named(NamedKey::Delete | NamedKey::Backspace) if !t.is_empty() => match t.as_str() {
                    // Do nothing if the value is "$0"
                    "$0" => {},
                    // Reset to "0" if only one character remains
                    _ if t.len() == 1 => t = "0".to_string(),
                    // Remove the last character
                    _ => { t.pop(); }
                },
                // Handle regular character input
                Key::Character(c) => {
                    // Only process if digit count is under the 8-digit cap
                    if digit_count < 8 {
                        // Get the first character from input
                        let character = c.chars().next().unwrap();

                        // Only continue if character is a number
                        if character.is_ascii_digit() {
                            match t.find('.') {
                                // Allow up to 2 digits after the decimal
                                Some(i) if t[i + 1..].len() < 2 => t.push(character),
                                None => match t.as_str() {
                                    // Replace "0" with typed digit
                                    "0" => t = character.to_string(),
                                    // Append digit to current value
                                    _ => t.push(character),
                                },
                                // Do nothing if 2 decimal digits already present
                                _ => {}
                            }
                        } else if character == '.' && !t.contains('.') && digit_count + 2 <= 8 {
                            // Allow decimal if not already present and projected total digits is less than 8
                            t.push('.');
                        }
                    }
                }
                // Ignore other key types
                _ => {return true}
            }

            // Set placeholder zeroes after the decimal point
            *self.1.zeros().text() = match t.find('.') {
                Some(i) => match t[i + 1..].len() {
                    0 => "00",
                    1 => "0",
                    _ => "",
                },
                None => "",
            }.to_string();

            // Recalculate digit count (excluding dot/comma)
            digit_count = t.chars().filter(|ch| ch.is_ascii_digit()).count();

            // Split into dollar and cent portions
            let (dollars, cents) = match t.find('.') {
                Some(i) => (&t[..i], Some(&t[i..])),
                None => (t.as_str(), None),
            };

            // Format dollar portion with commas every 3 chars
            let formatted_dollars: String = dollars
                .chars().rev().enumerate()
                .map(|(i, ch)| if i > 0 && i % 3 == 0 { vec![',', ch] } else { vec![ch] })
                .flatten().collect::<Vec<_>>().into_iter()
                .rev().collect();

            // Combine formatted dollars and cents into final string
            let t_formatted = format!("{}{}", formatted_dollars, cents.unwrap_or(""));

            // Choose font size based on total digits (including decimal placeholders)
            let total_digits = digit_count + self.1.zeros().text().len();
            let size = if total_digits <= 5 { font_size.title } else { font_size.h1 };

            // Set final text
            *self.1.amount().text() = t_formatted.clone();

            // Apply font size and line height to amount and zeros
            *self.1.amount().font_size() = size;
            *self.1.amount().line_height() = size * 1.25;
            *self.1.zeros().font_size() = size;
            *self.1.zeros().line_height() = size * 1.25;

            // Set min and max allowed values
            let min = 2.14;
            let max = 80.03;

            // Parse final amount as f64 for validation
            let value = t_formatted.parse::<f64>().unwrap_or(0.0);

            // Display subtext or error message based on parsed value
            match t_formatted.as_str() {
                "0" | "0." | "0.0" | "0.00" if !crate::config::IS_MOBILE => {
                    self.2.set_subtext(ctx, "Type dollar amount."); // Prompt input
                    self.2.3 = true; // Disable buttons
                }
                _ if value < min => {
                    self.2.set_error(ctx, "$2.18 Minimum."); // Exceeds min -> show error
                    self.2.3 = true; // Disable buttons
                }
                _ if value > max => {
                    self.2.set_error(ctx, "$80.14 Maximum."); // Exceeds max -> show error
                    self.2.3 = true; // Disable buttons
                }
                _ => {
                    self.2.set_subtext(ctx, "0.00001234 BTC"); // Valid -> show BTC equivalent
                    self.2.3 = false; // Enable buttons
                }
            }
        }  
        true
    }
}

#[derive(Debug, Component)]
struct Display(Row, BasicText, BasicText, BasicText);
impl OnEvent for Display {}

impl Display {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let font_size = theme.fonts.size.title;
        let (mc, dc) = (theme.colors.text.heading, theme.colors.text.secondary);
        Display (
            Row::center(0.0),
            Text::new(ctx, "$", TextStyle::Label(mc), font_size, Align::Left),
            Text::new(ctx, "0", TextStyle::Label(mc), font_size, Align::Left),
            Text::new(ctx, "", TextStyle::Label(dc), font_size, Align::Left),
        )
    }

    pub fn amount(&mut self) -> &mut BasicText {&mut self.2}
    pub fn zeros(&mut self) -> &mut BasicText {&mut self.3}
}