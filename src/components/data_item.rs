use rust_on_rails::prelude::*;
use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::shapes::{Circle, Rectangle};
use crate::components::button::Button;
use crate::layout::{Column, Bin, Row, Stack, Padding, Offset, Size};
use crate::PelicanUI;

/// A `DataItem` component. Used to organize and display information.
#[derive(Debug, Component)]
pub struct DataItem(Row, Option<Number>, DataItemContent);
impl OnEvent for DataItem {}

impl DataItem {
    /// Creates a new `DataItem` component.
    ///
    /// This method constructs a new `DataItem` with a number, label, optional text, secondary text,
    /// a table of key-value pairs, and quick actions that can be performed (buttons).
    ///
    /// # Parameters:
    /// - **`ctx`**: The [`Context`] for accessing the app's theme.
    /// - **`number`**: An optional static string representing a number associated with the item (e.g., a count or value).
    /// - **`label`**: The main label that represents the data item (e.g., the name of the item).
    /// - **`text`**: An optional string representing additional text to display alongside the label.
    /// - **`secondary`**: An optional string for secondary text to display under the label or text.
    /// - **`table`**: An optional vector of tuples, each containing a pair of static strings, which can represent a table of key-value pairs.
    /// - **`quick_actions`**: An optional vector of `Button` components representing actions that can be performed on the data item (e.g., buttons for editing, deleting, etc.).
    ///
    /// # Returns:
    /// - **`DataItem`**: The constructed `DataItem` component, ready for display.
    ///
    /// # Example:
    /// ```rust
    /// let quick_actions = vec![Button::secondary(ctx, Some("edit"), "Edit", None)];
    /// let table = vec![("Key1", "Value1"), ("Key2", "Value2")];
    /// let data_item = DataItem::new(
    ///     ctx, 
    ///     Some("1"),
    ///     "Item Label",
    ///     Some("Some additional text"), 
    ///     Some("Secondary Text"),
    ///     Some(table),
    ///     Some(quick_actions)
    /// );
    /// ```
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
        let theme = &ctx.get::<PelicanUI>().theme;
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
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
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
        let theme = &ctx.get::<PelicanUI>().theme;
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