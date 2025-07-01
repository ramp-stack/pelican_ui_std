use pelican_ui::events::{OnEvent, MouseState, MouseEvent, Event};
use pelican_ui::drawable::{Drawable, Component, Align, Color, Image};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::events::ListItemSelect;
use crate::elements::images::Icon;
use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::ButtonState;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size, Opt};
use crate::utils::ElementID;

#[derive(Component)]
pub struct ListItem(Stack, Rectangle, ListItemContent, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>, #[skip] Option<ElementID>, #[skip] bool);

impl ListItem {
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
        let color = ctx.theme.colors.background.primary;
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

        ListItem(layout, Rectangle::new(color), content, ButtonState::Default, Box::new(on_click), element_id, false)
    }

    pub fn title(&mut self) -> &mut Text {&mut self.2.data().left().title().1}
    pub fn subtitle(&mut self) -> &mut Option<ExpandableText> {self.2.data().left().subtitle()}


    pub fn is_selected(&self) -> bool {
        self.2.1.as_ref().map(|r| r.2).unwrap_or(false)
    }
}

impl OnEvent for ListItem {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                self.6 = true;
            } else if let MouseEvent{state: MouseState::Released, position: Some(_)} = event {
                if self.6 {
                    if let Some(radio) = self.2.1.as_mut() {
                        radio.select(ctx);
                        ctx.trigger_event(ListItemSelect(self.5.expect("Selectable List Items Require ElementIDs")));
                    }
                    match self.3 {
                        ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => {
                            ctx.hardware.haptic();
                            (self.4)(ctx)
                        },
                        _ => {}
                    }
                    self.6 = false;
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
        let color = ctx.theme.colors.text.secondary;
        ListItemContent(
            Row::new(16.0, Offset::Center, Size::Fit, Padding::default()),
            radio_button.map(|enabled| RadioButton::new(ctx, enabled)), 
            circle_icon.map(|data| Avatar::new(ctx, data, None, false, 48.0, None)),
            ListItemData::new(ctx, title, flair, subtitle, description, right_title, right_subtitle),
            caret.then(|| Icon::new(ctx, "forward", color, 16.0)),
        )
    }

    fn data(&mut self) -> &mut ListItemData {&mut self.3}
}

#[derive(Debug, Component)]
struct RadioButton(Row, Image, #[skip] bool); // is selected
impl OnEvent for RadioButton {}

impl RadioButton {
    fn new(ctx: &mut Context, is_enabled: bool) -> Self {
        let color = ctx.theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio"};
        RadioButton(Row::center(0.0), Icon::new(ctx, icon, color, 32.0), is_enabled)
    }

    fn select(&mut self, ctx: &mut Context) {
        self.2 = true;
        let color = ctx.theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio_filled", color, 32.0);
    }

    fn deselect(&mut self, ctx: &mut Context) {
        self.2 = false;
        let color = ctx.theme.colors.text.heading;
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

    fn left(&mut self) -> &mut LeftData {&mut self.1}
}
#[derive(Debug, Component)]
struct TitleRow(Row, Text, Option<Image>);
impl OnEvent for TitleRow {}

impl TitleRow {
    fn new(ctx: &mut Context, title: &str, flair: Option<(&'static str, Color)>) -> Self {
        let font_size = ctx.theme.fonts.size.h5;
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
        let font_size = ctx.theme.fonts.size.xs;
        LeftData (
            Column::new(4.0, Offset::Start, Size::custom(|widths: Vec<(f32, f32)>| (widths[0].0, f32::MAX)), Padding::default()),
            TitleRow::new(ctx, title, flair),
            subtitle.map(|text| ExpandableText::new(ctx, text, TextStyle::Secondary, font_size, Align::Left, Some(2))),
            description.map(|text| ExpandableText::new(ctx, &text, TextStyle::Secondary, font_size, Align::Left, Some(2))),
        )
    }

    fn title(&mut self) -> &mut TitleRow {&mut self.1}
    fn subtitle(&mut self) -> &mut Option<ExpandableText> {&mut self.2}
}

#[derive(Debug, Component)]
struct RightData(Column, Text, Option<Text>);
impl OnEvent for RightData {}

impl RightData {
    pub fn new(ctx: &mut Context, title: &str, subtitle: Option<&str>) -> Self {
        let font_size = ctx.theme.fonts.size;
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

    pub fn index(&self) -> Option<u8> {
        if self.1.is_selected() { return Some(0) }
        if self.2.is_selected() { return Some(1) }
        if self.3.as_ref().map(|s| s.is_selected()).unwrap_or(false) { return Some(2) }
        if self.4.as_ref().map(|s| s.is_selected()).unwrap_or(false) { return Some(3) }
        None
    }
}

#[derive(Debug, Component)]
pub struct ListItemGroup(Column, Vec<Opt<ListItem>>);
impl OnEvent for ListItemGroup {}

impl ListItemGroup {
    pub fn new(list_items: Vec<ListItem>) -> Self {
        let list_items = list_items.into_iter().map(|item| Opt::new(item, true)).collect();
        ListItemGroup(Column::center(0.0), list_items)
    }

    pub fn items(&mut self) -> &mut Vec<Opt<ListItem>> {&mut self.1}
    pub fn hide(&mut self, hide: bool, i: usize) {
        self.items().get_mut(i).map(|item| item.display(!hide));
    }
}
