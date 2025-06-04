use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component, Align, Shape};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::shapes::{Circle, Rectangle};
use crate::components::button::Button;
use crate::layout::{Column, Bin, Row, Stack, Padding, Offset, Size};

#[derive(Debug, Component)]
pub struct DataItem(Row, Option<Number>, DataItemContent);
impl OnEvent for DataItem {}

impl DataItem {
    pub fn new(
        ctx: &mut Context,
        number: Option<&str>,
        label: &str,
        text: Option<&str>,
        secondary: Option<&str>,
        table: Option<Vec<(&str, &str)>>,
        quick_actions: Option<Vec<Button>>,
    ) -> Self {
        DataItem (
            Row::new(32.0, Offset::Start, Size::Fit, Padding::default()),
            number.map(|n| Number::new(ctx, n)),
            DataItemContent::new(ctx, label, text, secondary, table, quick_actions)
        )
    }
}

#[derive(Debug, Component)]
struct Number(Stack, Shape, Text);
impl OnEvent for Number {}

impl Number {
    pub fn new(ctx: &mut Context, txt: &str) -> Self {
        let theme = &ctx.theme;
        let (color, font_size) = (theme.colors.background.secondary, theme.fonts.size.h5);
        Number(
            Stack::center(),
            Circle::new(32.0, color),
            Text::new(ctx, txt, TextStyle::Heading, font_size, Align::Left), 
        )
    }
}

#[derive(Debug, Component)]
struct DataItemContent(Column, Text, Option<ExpandableText>, Option<Text>, Option<Table>, Option<QuickActions>);
impl OnEvent for DataItemContent {}

impl DataItemContent {
    fn new(
        ctx: &mut Context,
        label: &str,
        text: Option<&str>,
        secondary: Option<&str>,
        table: Option<Vec<(&str, &str)>>,
        quick_actions: Option<Vec<Button>>,
    ) -> Self {
        let font_size = ctx.theme.fonts.size;
        DataItemContent(
            Column::new(16.0, Offset::Start, Size::fill(), Padding::default()),
            Text::new(ctx, label, TextStyle::Heading, font_size.h5, Align::Left),
            text.map(|t| ExpandableText::new(ctx, t, TextStyle::Primary, font_size.md, Align::Left)),
            secondary.map(|t|Text::new(ctx, t, TextStyle::Secondary, font_size.sm, Align::Left)),
            table.map(|tabulars| Table::new(ctx, tabulars)),
            quick_actions.map(QuickActions::new)
        )
    }
}

#[derive(Debug, Component)]
struct Table(pub Column, pub Vec<Tabular>);
impl OnEvent for Table {}

impl Table {
    pub fn new(ctx: &mut Context, items: Vec<(&str, &str)>) -> Self {
        Table (
            Column::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            items.iter().map(|(name, data)| Tabular::new(ctx, name, data)).collect()
        )
    }
}

#[derive(Debug, Component)]
struct Tabular(Row, Text, Bin<Stack, Rectangle>, Text);
impl OnEvent for Tabular {}

impl Tabular {
    fn new(ctx: &mut Context, name: &str, data: &str) -> Self {
        let theme = &ctx.theme;
        let (font_size, color) = (theme.fonts.size.sm, theme.colors.shades.transparent);
        Tabular (
            Row::new(8.0, Offset::Start, Size::Fit, Padding(0.0, 4.0, 0.0, 4.0)),
            Text::new(ctx, name, TextStyle::Primary, font_size, Align::Left),
            Bin(
                Stack(Offset::Center, Offset::Center, Size::Fit, Size::Static(1.0), Padding::default()),
                Rectangle::new(color),
            ),
            Text::new(ctx, data, TextStyle::Primary, font_size, Align::Left),
        )
    }
}

#[derive(Debug, Component)]
struct QuickActions(Row, Vec<Button>); // Row should be wrap
impl OnEvent for QuickActions {}

impl QuickActions {
    fn new(buttons: Vec<Button>) -> Self {
        QuickActions(Row::new(8.0, Offset::Start, Size::Fit, Padding::default()), buttons)
    }
}