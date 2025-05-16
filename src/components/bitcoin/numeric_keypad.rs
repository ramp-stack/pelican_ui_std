use rust_on_rails::prelude::*;
use crate::components::button::Button;
use crate::layout::{Column, Row, Offset, Size, Padding};

/// A numeric keypad component with four rows of buttons.
#[derive(Debug, Component)]
pub struct NumericKeypad(Column, ButtonRow, ButtonRow, ButtonRow, ButtonRow);
impl OnEvent for NumericKeypad {}

impl NumericKeypad {
    /// Creates a new [`NumericKeypad`] component with 4 rows of buttons.
    ///
    /// # Example
    /// ```
    /// let mut keypad = NumericKeypad::new(ctx);
    /// ```
    pub fn new(ctx: &mut Context) -> Self {
        NumericKeypad(
            Column::new(16.0, Offset::Center, Size::Fit, Padding::default()), 
            ButtonRow::new(ctx, Some("1"), Some("2"), Some("3")),
            ButtonRow::new(ctx, Some("4"), Some("5"), Some("6")),
            ButtonRow::new(ctx, Some("7"), Some("8"), Some("9")),
            ButtonRow::new(ctx, Some("."), Some("0"), None),
        )
    }
}

#[derive(Debug, Component)]
struct ButtonRow(Row, Button, Button, Button);
impl OnEvent for ButtonRow {}

impl ButtonRow {
    fn new(ctx: &mut Context, a: Option<&'static str>, b: Option<&'static str>, c: Option<&'static str>) -> Self {
        let key = |ctx: &mut Context, a: Option<&'static str>| {
            match a {
                Some(txt) => Button::keypad(ctx, Some(txt), None, move |ctx: &mut Context| on_click(ctx, Key::Character(SmolStr::new_static(txt)))),
                None => Button::keypad(ctx, None, Some("back"), |ctx: &mut Context| on_click(ctx, Key::Named(NamedKey::Backspace)))
            }
        };
        
        ButtonRow(Row::center(16.0), key(ctx, a), key(ctx, b), key(ctx, c))        
    }
}

fn on_click(ctx: &mut Context, key: Key) {
    ctx.trigger_event(KeyboardEvent{state: KeyboardState::Pressed, key})
}