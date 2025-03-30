use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::circle_icon::{CircleIcon, CircleIconContent, CircleIconStyle};
use crate::layout::{Row, RowOffset, Column, ColumnOffset, Stack, Offset, Size};
use crate::PelicanUI;

struct DataItem(pub Row, pub Option<_Number>, pub _DataItemContent);

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

impl Component for DataItem {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![];
        if let Some(number) = &mut self.1 { children.push(number); }
        children.push(&mut self.2)
        
        children
    }

    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&self.1];
        if let Some(number) = &self.1 { children.push(number); }
        children.push(&self.2)
        
        children
    }

    fn layout(&self) -> &dyn Layout {&self.0}
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

impl Component for _Number {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> { vec![&mut self.1, &mut self.2] }
    fn children(&self) -> Vec<&ComponentRef> { vec![&self.1, &self.2] }
    fn layout(&self) -> &dyn Layout {&self.0}
}

struct _DataItemContent(pub BasicText, pub BasicText, pub BasicText, pub _Table, pub _QuickActions);

impl _DataItemContent {
    pub fn new(
        label: &'static str,
        text: Option<&'static str>,
        secondary: Option<&'static str>,
        table: Option<Vec<(&'static str, &'static str)>>,
        quick_actions: Option<Vec<(&'static str, &'static str)>>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        _DataItemContent (
            Text::new(ctx, label, TextStyle::Heading, font_size.h5),
            text.map(|t| Text::new(ctx, t, TextStyle::Primary, font_size.md)),
            secondary.map(|t| Text::new(ctx, t, TextStyle::Secondary, font_size.sm)),
            table.map(|tabulars| _Table::new(ctx, tabulars)),
            quick_actions.map(|actions| _QuickActions::new(actions)),
        )
    }
}

impl Component for _DataItemContent {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&mut self.1];

        if let Some(text) = &mut self.2 { children.push(text); }
        if let Some(secondary) = &mut self.3 { children.push(secondary); }
        if let Some(table) = &mut self.4 { children.push(table); }
        if let Some(actions) = &mut self.5 { children.push(actions); }
        
        children
    }

    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![&self.1];

        if let Some(text) = &self.2 { children.push(text); }
        if let Some(secondary) = &self.3 { children.push(secondary); }
        if let Some(table) = &self.4 { children.push(table); }
        if let Some(actions) = &self.5 { children.push(actions); }
        
        children
    }

    fn layout(&self) -> &dyn Layout {&self.0}
}

struct _Table(pub Column, pub Vec<_Tabular>);

impl _Table {
    pub fn new(ctx: &mut Context, items: Vec<(&'static str, &'static str)>) -> Self {
        _Table (
            Column(0, Offset::Start, Size::Fit),
            items.iter().map(|(name, data)| _Tabular::new(ctx, name, data)).collect()
        )
    }
}

impl Component for _Table {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        self.1.iter().map(|a| &mut a).collect()
    }
    fn children_mut(&self) -> Vec<&ComponentRef> {
        self.1.iter().map(|a| &a).collect()
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}

struct _Tabular(pub Row, pub BasicText, pub Expand, pub BasicText);

impl _Tabular {
    pub fn new(ctx: &mut Context, name: &'static str, data: &'static str) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.sm;
        _Tabular (
            Row(0, Offset::Start, Size::Fill),
            Text::new(ctx, name, TextStyle::Primary, font_size),
            Expand(true, false),  // Expand width, not height
            Text::new(ctx, name, TextStyle::Primary, font_size),
        )
    }
}

impl Component for _Tabular {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2, &mut self.3]}
    fn children_mut(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2, &self.3]}
    fn layout(&self) -> &dyn Layout {&self.0}
}

struct Expand;

struct _QuickActions(pub Wrap, pub Vec<Button>);

impl _QuickActions {
    pub fn new(items: Vec<(&'static str, &'static str)>) -> Self {
        _QuickActions (
            Wrap(8, 8, Offset::Start, Size::Fill),
            items.iter().map(|(icon, label)| Button::secondary(ctx, Some(icon), label, None)).collect()
        )
    }
}

impl Component for _QuickActions {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        self.1.iter().map(|a| &mut a).collect()
    }
    fn children_mut(&self) -> Vec<&ComponentRef> {
        self.1.iter().map(|a| &a).collect()
    }
    fn layout(&self) -> &dyn Layout {&self.0}
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