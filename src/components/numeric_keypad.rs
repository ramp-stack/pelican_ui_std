use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::components::button::Button;
use crate::layout::{Column, Row, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Clone, Debug, Component)]
pub struct NumericKeypad(Column, ButtonRow, ButtonRow, ButtonRow, ButtonRow);
impl Events for NumericKeypad {}

impl NumericKeypad {
    pub fn new(ctx: &mut Context) -> Self {
        NumericKeypad(
            Column(16, Offset::Center, Size::Fit, Padding(0, 16, 0, 16)), 
            ButtonRow::new(ctx, Some("1"), Some("2"), Some("3")),
            ButtonRow::new(ctx, Some("4"), Some("5"), Some("6")),
            ButtonRow::new(ctx, Some("7"), Some("8"), Some("9")),
            ButtonRow::new(ctx, Some("."), Some("0"), None),
        )
    }
}

#[derive(Clone, Debug, Component)]
struct ButtonRow(Row, Button, Button, Button);
impl Events for ButtonRow {}

impl ButtonRow {
    fn new(ctx: &mut Context, a: Option<&'static str>, b: Option<&'static str>, c: Option<&'static str>) -> Self {
        let key = |ctx: &mut Context, a: Option<&'static str>| {
            Button::keypad(ctx, a, a.is_none().then_some("back"), |ctx: &mut Context| ())
        };
        
        ButtonRow(Row::center(16), key(ctx, a), key(ctx, b), key(ctx, c))        
    }
}