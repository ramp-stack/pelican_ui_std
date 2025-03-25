use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::circle_icon::{CircleIcon, CircleIconContent, CircleIconStyle};
use crate::layout::{Row, RowOffset, Column, ColumnOffset, Stack, Offset, Size};
use crate::PelicanUI;

// Rules:
// Exported structs and enums prefixed with name of the "top-layer" component.
// If a struct or enum isn’t exported, start its name with _.
// First item in a file should be top-layer component struct or enum
// 'User' should never touch the struct, only new functions

struct _DataItem(pub Row, pub Option<_Number>, pub _DataItemContent);

impl DataItem {
    pub fn new(
        number: Option<u32>,
        label: &'static str,
        text: Option<&'static str>,
        secondary: Option<&'static str>,
        table: Option<Vec<(&'static str, &'static str)>>,
        quick_actions: Option<Vec<(&'static str, &'static str)>>,
    ) -> Self {
        DataItem (
            Row(32, Offset::Start, Size::Fill),
            number.map(|n| _Number::new(ctx, n)),
            _DataItemContent(label, text, secondary, table, quick_actions)
        )
    }
}

struct _Number(pub Stack, pub BasicText, pub Shape);

impl _Number {
    pub fn new(ctx: &mut Context, num: u32) -> Self {
        let theme = ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.background.secondary, theme.fonts.size.h5);
        _Number (
            Stack((Offset::Center, Offset::Center), (Size::Fit, Size::Fit)),
            Text::new(ctx, &num.to_string(), TextStyle::Heading, font_size),
            Circle::new(32, color), 
        )
    }
}

struct _DataItemContent(pub BasicText, pub BasicText, pub BasicText, pub _Table, pub _QuickActions);

impl _DataItemContent {
    pub fn new() -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        _DataItemContent (
            Text::new(ctx, label, TextStyle::Heading, font_size.h5),
            text.map(|t| Text::new(ctx, t, TextStyle::Primary, font_size.md)),
            secondary.map(|t| Text::new(ctx, t, TextStyle::Secondary, font_size.sm)),
        )
    }
}

struct _Table(pub Stack, pub Vec<_Tabular>);

struct _Tabular(pub Row, pub BasicText, pub BasicText);

struct _QuickActions(pub Wrap, pub Vec<Button>);

impl Component for Card {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
    fn layout(&self) -> &dyn Layout {&self.0}
}
pub struct DataItem {
    number: Option<&'static str>,
    label: &'static str,
    table: Option<Vec<(&'static str, &'static str)>>,
    text: Option<&'static str>,
    secondary_text: Option<&'static str>,
    quick_actions: Option<Vec<(&'static str, &'static str)>>,
}

impl ComponentBuilder for DataItem {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];
        let mut contents: Vec<Box<dyn ComponentBuilder>> = vec![];

        contents.push(Text::heading(ctx, self.label, TextSize::h5()));

        if let Some(text) = &self.text { 
            contents.push(Text::primary(ctx, self.text, TextSize::md())); 
        }

        if let Some(secondary_text) = &self.secondary_text { 
            contents.push(Text::secondary(ctx, self.secondary_text, TextSize::sm())); 
        }

        if !self.table.is_empty() { 
            let tabulars = &self.table
                .into_iter()
                .map(|row| {
                    Row(ZERO, AUTO, Align::Center, vec![
                        Text::primary(ctx, row.0, TextSize::sm()),
                        Text::primary(ctx, row.1, TextSize::sm())
                    ])
                }).collect();

            contents.push(Column(ZERO, 0, Align::Left, tabulars));
        }

        if !self.quick_actions.is_empty() { 
            let buttons = &self.quick_actions
                .into_iter()
                .map(|label| {
                    Button::secondary(label, Icon::Edit, Size::Medium, Width::Hug)
                }).collect();

            contents.push(Row(ZERO, 8, Align::Left, buttons));
        }

        if let Some(num) = self.number { 
            children.push(
                Stack(ZERO, Align::Center, vec![
                    Circle(32, COLORS.background.secondary, None),
                    Text::heading(ctx, num, TextSize::h5())
                ])
            ); 
        }

        children.push(Column(ZERO, 16, Align::Left, contents));

        Row(ZERO, 16, Align::Left, children).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}

// let confirm_amount = DataItem {
//     number: Some("2"),
//     label: "Confirm amount",
//     table: vec![
//         ("date", "12/25/20"),
//         ("time", "11:45 PM")
//     ],
//     text: None,
//     secondary_text: None,
//     quick_actions: vec!["Edit amount", "Edit speed"]
// }

// let confirm_address = DataItem {
//     number: Some("1"),
//     label: "Confirm adress",
//     table: Vec::new(),
//     text: Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
//     secondary_text: Some("Bitcoin sent to the wrong address can never be recovered."),
//     quick_actions: vec!["Edit address"]
// }