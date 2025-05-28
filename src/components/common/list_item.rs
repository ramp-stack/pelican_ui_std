use rust_on_rails::prelude::*;
use crate::events::ListItemSelect;
use crate::elements::images::Icon;
use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::ButtonState;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::utils::ElementID; 
use crate::plugin::PelicanUI;
/// A List Item with various customizable components, such as a title, subtitle, description, 
/// and other UI elements like a radio button or a circle icon. The item can be interacted with, triggering 
/// a callback when clicked.
#[derive(Component)]
pub struct ListItem(Stack, Rectangle, ListItemContent, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>, #[skip] Option<ElementID>);

impl ListItem {
    /// Creates a new `ListItem` with the specified attributes, including text content, a callback for 
    /// interactions, and an optional element ID.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `caret`: Whether to show a caret icon.
    /// - `title`: The title displayed on the list item.
    /// - `flair`: Optional flair text and associated color for the list item.
    /// - `subtitle`: Optional subtitle text for additional context.
    /// - `description`: Optional description displayed on the list item.
    /// - `right_title`: Optional title to be displayed on the right side of the item.
    /// - `right_subtitle`: Optional subtitle on the right side.
    /// - `radio_button`: Optional boolean indicating whether the item has a radio button.
    /// - `circle_icon`: Optional avatar content to be shown as a circle icon.
    /// - `element_id`: Optional unique identifier for the item.
    /// - `on_click`: A callback function that gets triggered when the item is clicked.
    ///
    /// # Returns
    /// A new instance of `ListItem` configured with the provided parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        caret: bool,
        title: &str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&str>,
        description: Option<&str>,
        right_title: Option<&str>,
        right_subtitle: Option<&str>,
        radio_button: Option<bool>,
        circle_icon: Option<AvatarContent>,
        element_id: Option<ElementID>,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.background.primary;
        let content = ListItemContent::new(
            ctx, caret, title, flair, subtitle, description, right_title, 
            right_subtitle, radio_button, circle_icon
        );
        let layout = Stack(
            Offset::Start, Offset::Center, 
            Size::custom(|widths: Vec<(f32, f32)>| (widths[1].0, f32::MAX)), 
            Size::custom(|heights: Vec<(f32, f32)>| heights[1]), 
            Padding(0.0, 16.0, 0.0, 16.0)
        );

        ListItem(layout, Rectangle::new(color), content, ButtonState::Default, Box::new(on_click), element_id)
    }

    /// Returns a boolean representing the state of the [`RadioButton`]. Will return false if no [`RadioButton`] exists.
    pub fn is_selected(&self) -> bool {
        self.2.1.as_ref().map(|r| r.2).unwrap_or(false)
    }
}

impl OnEvent for ListItem {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Released, position: Some(_)} = event {
                if let Some(radio) = self.2.1.as_mut() {
                    radio.select(ctx);
                    ctx.trigger_event(ListItemSelect(self.5.expect("Selectable List Items Require ElementIDs")));
                }
                match self.3 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => {
                        #[cfg(target_os = "ios")]
                        crate::vibrate();
                        (self.4)(ctx)
                    },
                    _ => {}
                }
            }
        } else if let Some(ListItemSelect(id)) = event.downcast_ref::<ListItemSelect>() {
            if let Some(self_id) = &self.5 {
                if *id != *self_id {
                    if let Some(radio) = self.2.1.as_mut() {
                        radio.deselect(ctx);
                    }
                }
            }
        }
        false
    }
}

impl std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListItem")
    }
}

#[derive(Debug, Component)]
struct ListItemContent(Row, Option<RadioButton>, Option<Avatar>, ListItemData, Option<Image>);
impl OnEvent for ListItemContent {}

impl ListItemContent {
    #[allow(clippy::too_many_arguments)]
    fn new(
        ctx: &mut Context,
        caret: bool,
        title: &str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&str>,
        description: Option<&str>,
        right_title: Option<&str>,
        right_subtitle: Option<&str>,
        radio_button: Option<bool>,
        circle_icon: Option<AvatarContent>,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.text.secondary;
        ListItemContent(
            Row::new(16.0, Offset::Center, Size::Fit, Padding::default()),
            radio_button.map(|enabled| RadioButton::new(ctx, enabled)), 
            circle_icon.map(|data| Avatar::new(ctx, data, None, false, 48.0, None)),
            ListItemData::new(ctx, title, flair, subtitle, description, right_title, right_subtitle),
            caret.then(|| Icon::new(ctx, "forward", color, 16.0)),
        )
    }
}

#[derive(Debug, Component)]
struct RadioButton(Row, Image, #[skip] bool); // is selected
impl OnEvent for RadioButton {}

impl RadioButton {
    fn new(ctx: &mut Context, is_enabled: bool) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio"};
        RadioButton(Row::center(0.0), Icon::new(ctx, icon, color, 32.0), is_enabled)
    }

    fn select(&mut self, ctx: &mut Context) {
        self.2 = true;
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio_filled", color, 32.0);
    }

    fn deselect(&mut self, ctx: &mut Context) {
        self.2 = false;
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio", color, 32.0);
    }
}

#[derive(Debug, Component)]
struct ListItemData(Row, LeftData, Option<RightData>);
impl OnEvent for ListItemData {}

impl ListItemData {
    fn new(
        ctx: &mut Context,
        title: &str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&str>,
        description: Option<&str>,
        right_title: Option<&str>,
        right_subtitle: Option<&str>,
    ) -> Self {
        ListItemData(
            Row::new(8.0, Offset::Start, Size::Fit, Padding::default()),
            LeftData::new(ctx, title, flair, subtitle, description),
            right_title.map(|r_title| RightData::new(ctx, r_title, right_subtitle)), 
        )
    }
}
#[derive(Debug, Component)]
struct TitleRow(Row, Text, Option<Image>);
impl OnEvent for TitleRow {}

impl TitleRow {
    fn new(ctx: &mut Context, title: &str, flair: Option<(&'static str, Color)>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        TitleRow(
            Row::new(8.0, Offset::Start, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size, Align::Left),
            flair.map(|(name, color)| Icon::new(ctx, name, color, 20.0)),
        )
    }
}

#[derive(Debug, Component)]
struct LeftData(Column, TitleRow, Option<ExpandableText>, Option<ExpandableText>);
impl OnEvent for LeftData {}

impl LeftData {
    pub fn new(
        ctx: &mut Context,
        title: &str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&str>,
        description: Option<&str>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.xs;
        LeftData (
            Column::new(4.0, Offset::Start, Size::custom(|widths: Vec<(f32, f32)>| (widths[0].0, f32::MAX)), Padding::default()),
            TitleRow::new(ctx, title, flair),
            subtitle.map(|text| ExpandableText::new(ctx, text, TextStyle::Secondary, font_size, Align::Left)),
            description.map(|text| {
                let limit = if crate::config::IS_MOBILE {80} else {100};
                let trimmed = if text.len() > limit { format!("{}...", &text[..(limit-3)]) } else { text.to_string() };
                ExpandableText::new(ctx, &trimmed, TextStyle::Secondary, font_size, Align::Left)
            }),
        )
    }
}

#[derive(Debug, Component)]
struct RightData(Column, Text, Option<Text>);
impl OnEvent for RightData {}

impl RightData {
    pub fn new(ctx: &mut Context, title: &str, subtitle: Option<&str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        RightData (
            Column::new(4.0, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size.h5, Align::Left),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs, Align::Left)),
        )
    }
}

impl ListItem {  
    /// Creates a list item for a radio selection.
    /// Displays a title, subtitle, and description, and supports a selected state.
    pub fn selection(
        ctx: &mut Context,
        selected: bool,
        title: &str,
        subtitle: &str,
        description: Option<&str>,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        ListItem::new(ctx, false, title, None, Some(subtitle), description, None, None, Some(selected), None, Some(ElementID::new()), on_click)
    }
}

/// A component representing a radio-style list item selector with multiple options.
#[derive(Debug, Component)]
pub struct ListItemSelector(
    Column,       // The layout column for organizing the items vertically.
    ListItem,     // The first list item (selected).
    ListItem,     // The second list item (unselected).
    Option<ListItem>,  // The third list item (optional, unselected).
    Option<ListItem>,  // The fourth list item (optional, unselected).
);

impl OnEvent for ListItemSelector {}
impl ListItemSelector {
    /// Creates a new `ListItemSelector` with four selectable list items, where the first item is selected by default.
    ///
    /// # Parameters:
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `first`: A tuple containing the title, subtitle, and an optional description for the first list item (selected).
    /// - `second`: A tuple containing the title, subtitle, and an optional description for the second list item (unselected).
    /// - `third`: An optional tuple containing the title, subtitle, and an optional description for the third list item (unselected).
    /// - `fourth`: An optional tuple containing the title, subtitle, and an optional description for the fourth list item (unselected).
    ///
    /// # Returns:
    /// A new `ListItemSelector` component, containing the provided list items in a vertical column.
    ///
    /// # Example:
    /// ```
    /// let selector = ListItemSelector::new(
    ///     ctx,
    ///     ("Option 1", "Description 1", Some("This is the first option")),
    ///     ("Option 2", "Description 2", Some("This is the second option")),
    ///     Some(("Option 3", "Description 3", Some("This is the third option"))),
    ///     Some(("Option 4", "Description 4", Some("This is the fourth option")))
    /// );
    /// ```
    pub fn new(
        ctx: &mut Context, 
        first: (&str, &str, Option<&str>), // title, subtitle, description
        second: (&str, &str, Option<&str>), 
        third: Option<(&str, &str, Option<&str>)>, 
        fourth: Option<(&str, &str, Option<&str>)>
    ) -> Self {
        ListItemSelector(
            Column::center(0.0), 
            ListItem::selection(ctx, true, first.0, first.1, first.2, |_: &mut Context| ()),
            ListItem::selection(ctx, false, second.0, second.1, second.2, |_: &mut Context| ()),
            third.map(|third| ListItem::selection(ctx, false, third.0, third.1, third.2, |_: &mut Context| ())),
            fourth.map(|fourth| ListItem::selection(ctx, false, fourth.0, fourth.1, fourth.2, |_: &mut Context| ())),
        )
    }

    /// Returns the index of currently selected [`ListItem`].
    pub fn index(&self) -> Option<u8> {
        if self.1.is_selected() { return Some(0) }
        if self.2.is_selected() { return Some(1) }
        if self.3.as_ref().map(|s| s.is_selected()).unwrap_or(false) { return Some(2) }
        if self.4.as_ref().map(|s| s.is_selected()).unwrap_or(false) { return Some(3) }
        None
    }
}


/// A component representing a group of list items, arranged vertically in a column.
///
/// [`ListItemGroup`] is used to display multiple [`ListItem`] components stacked vertically.
/// It provides a container that arranges the list items in a column layout, making it
/// ideal for organizing and presenting lists of items within a user interface.
///
/// # Components
/// - `Column`: A layout component that arranges its children in a vertical column.
/// - `Vec<ListItem>`: A vector of [`ListItem`] components to be displayed within the column.
///
/// # Example
/// ```rust
/// let list_item_group = ListItemGroup::new(vec![
///     ListItem::new("Item 1"),
///     ListItem::new("Item 2"),
///     ListItem::new("Item 3")
/// ]);
/// ```
#[derive(Debug, Component)]
pub struct ListItemGroup(Column, Vec<ListItem>);
impl OnEvent for ListItemGroup {}

impl ListItemGroup {
    /// Creates a new [`ListItemGroup`] with a vector of list items.
    ///
    /// This function initializes a [`ListItemGroup`] component by arranging the provided
    /// list items in a vertical column layout. The `Column` component ensures the items
    /// are stacked properly with default alignment.
    ///
    /// # Parameters
    /// - `list_items`: A vector of [`ListItem`] components to be displayed in the group.
    ///
    /// # Returns
    /// A new [`ListItemGroup`] component containing the provided list items arranged in a column.
    ///
    /// # Example
    /// ```rust
    /// let list_item_group = ListItemGroup::new(vec![
    ///     ListItem::new("Item A"),
    ///     ListItem::new("Item B"),
    /// ]);
    /// ```
    pub fn new(list_items: Vec<ListItem>) -> Self {
        ListItemGroup(Column::center(0.0), list_items)
    }
}
