use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::events::{ListItemSelect, RemoveContactEvent, AddContactEvent};
use crate::elements::images::Icon;
use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::{ButtonState, QuickDeselectButton};
use crate::components::avatar::{Avatar, AvatarIconStyle, AvatarContent};
use crate::layout::{Column, Stack, Row, Wrap, Padding, Offset, Size};
use crate::PelicanUI;
use uuid::Uuid;


#[derive(Component)]
pub struct ListItem(Stack, Rectangle, ListItemContent, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>, #[skip] Uuid);

impl ListItem {
    pub fn new(
        ctx: &mut Context,
        caret: bool,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
        right_title: Option<&'static str>,
        right_subtitle: Option<&'static str>,
        radio_button: Option<bool>,
        circle_icon: Option<AvatarContent>,
        id: Uuid,
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

        ListItem(layout, Rectangle::new(color), content, ButtonState::Default, Box::new(on_click), id)
    }
}

impl Events for ListItem {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                self.2.1.as_mut().map(|radio| {radio.select(ctx); ctx.trigger_event(ListItemSelect(self.5));});
                match self.3 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => {
                        #[cfg(target_os = "ios")]
                        crate::vibrate();
                        (self.4)(ctx)
                    },
                    _ => {}
                }
            }
        }  if let Some(ListItemSelect(id)) = event.downcast_ref::<ListItemSelect>() {
            if *id != self.5 {
                self.2.1.as_mut().map(|radio| radio.deselect(ctx));
            }
        }
        false
    }
}

impl std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListItem(...)")
    }
}

#[derive(Debug, Component)]
pub struct ListItemContent(Row, Option<RadioButton>, Option<Avatar>, ListItemData, Option<Image>);
impl Events for ListItemContent {}

impl ListItemContent {
    pub fn new(
        ctx: &mut Context,
        caret: bool,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
        right_title: Option<&'static str>,
        right_subtitle: Option<&'static str>,
        radio_button: Option<bool>,
        circle_icon: Option<AvatarContent>,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.text.secondary;
        ListItemContent(
            Row(16.0, Offset::Center, Size::Fit, Padding::default()),
            radio_button.map(|enabled| RadioButton::new(ctx, enabled)), 
            circle_icon.map(|data| Avatar::new(ctx, data, None, false, 48.0)),
            ListItemData::new(ctx, title, flair, subtitle, description, right_title, right_subtitle),
            caret.then(|| Icon::new(ctx, "forward", color, 16.0)),
        )
    }
}

#[derive(Debug, Component)]
struct RadioButton(Row, Image);
impl Events for RadioButton {}

impl RadioButton {
    pub fn new(ctx: &mut Context, is_enabled: bool) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio"};
        RadioButton(Row::center(0.0), Icon::new(ctx, icon, color, 32.0))
    }

    pub fn select(&mut self, ctx: &mut Context) {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio_filled", color, 32.0);
    }

    pub fn deselect(&mut self, ctx: &mut Context) {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio", color, 32.0);
    }
}

#[derive(Debug, Component)]
struct ListItemData(pub Row, pub LeftData, pub Option<RightData>);
impl Events for ListItemData {}

impl ListItemData {
    pub fn new(
        ctx: &mut Context,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
        right_title: Option<&'static str>,
        right_subtitle: Option<&'static str>,
    ) -> Self {
        ListItemData(
            Row(8.0, Offset::Start, Size::Fit, Padding::default()),
            LeftData::new(ctx, title, flair, subtitle, description),
            right_title.map(|r_title| RightData::new(ctx, r_title, right_subtitle)), 
        )
    }
}
#[derive(Debug, Component)]
struct TitleRow(Row, BasicText, Option<Image>);
impl Events for TitleRow {}

impl TitleRow {
    pub fn new(ctx: &mut Context, title: &'static str, flair: Option<(&'static str, Color)>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        TitleRow(
            Row(8.0, Offset::Start, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size),
            flair.map(|(name, color)| Icon::new(ctx, name, color, 20.0)),
        )
    }
}

#[derive(Debug, Component)]
struct LeftData(pub Column, pub TitleRow, pub Option<ExpandableText>, pub Option<ExpandableText>);
impl Events for LeftData {}

impl LeftData {
    pub fn new(
        ctx: &mut Context,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
    ) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size.xs;
        LeftData (
            Column(4.0, Offset::Start, Size::custom(|widths: Vec<(f32, f32)>| (widths[0].0, f32::MAX)), Padding::default()),
            TitleRow::new(ctx, title, flair),
            subtitle.map(|text| ExpandableText::new(ctx, text, TextStyle::Secondary, font_size)),
            description.map(|text| {
                text.len()
                    .checked_sub(70 + 1)
                    .map(|_| format!("{}...", &text[..70_usize.saturating_sub(3)]))
                    .unwrap_or_else(|| text.to_string());
                ExpandableText::new(ctx, text, TextStyle::Secondary, font_size)
            }),
        )
    }
}

#[derive(Debug, Component)]
struct RightData(Column, BasicText, Option<BasicText>);
impl Events for RightData {}

impl RightData {
    pub fn new(ctx: &mut Context, title: &'static str, subtitle: Option<&'static str>) -> Self {
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        RightData (
            Column(4.0, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size.h5),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
        )
    }
}

impl ListItem {
    pub fn contact(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        nym: &'static str,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        ListItem::new(ctx, true, name, None, Some(nym), None, None, None, None, Some(data), Uuid::new_v4(), on_click)
    }

    pub fn recipient(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        nym: &'static str,
    ) -> Self {
        let id = Uuid::new_v4();
        ListItem::new(ctx, true, name, None, Some(nym), None, None, None, None, Some(data), id, move |ctx: &mut Context| ctx.trigger_event(AddContactEvent(name, id)))
    }

    pub fn direct_message(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        recent: &'static str,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        ListItem::new(ctx, true, name, None, Some(recent), None, None, None, None, Some(data), Uuid::new_v4(), on_click)
    }

    pub fn group_message(
        ctx: &mut Context,
        names: Vec<&'static str>,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        let description = Box::leak(names.join(", ").into_boxed_str());
        let avatar = AvatarContent::Icon("group", AvatarIconStyle::Secondary);
        ListItem::new(ctx, true, "Group Message", None, None, Some(description), None, None, None, Some(avatar), Uuid::new_v4(), on_click)
    }

    pub fn room(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        members: &'static str,
        description: &'static str,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        ListItem::new(ctx, true, name, None, Some(members), Some(description), None, None, None, Some(data), Uuid::new_v4(), on_click)
    }

    pub fn bitcoin(
        ctx: &mut Context,
        is_received: bool,
        usd: f32,
        date: &'static str,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let title = if is_received { "Received Bitcoin" } else { "Sent Bitcoin" };
        let usd = Box::leak(format!("{:.2}", usd).into_boxed_str());
        ListItem::new(ctx, true, title, None, Some(date), None, Some(usd), Some("Details"), None, None, Uuid::new_v4(), on_click)
    }

    pub fn bitcoin_sending(
        ctx: &mut Context,
        usd: f32,
        btc: f32,
        date: &'static str,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.status.warning;
        let flair = ("warning", color);
        let usd =  Box::leak(format!("${:.2}", usd).into_boxed_str());
        let btc =  Box::leak(format!("${:.8} BTC", btc).into_boxed_str());
        ListItem::new(ctx, true, "Sending Bitcoin", Some(flair), Some(date), None, Some(usd), Some(btc), None, None, Uuid::new_v4(), on_click)
    }

    pub fn selection(
        ctx: &mut Context,
        selected: bool,
        title: &'static str,
        subtitle: &'static str,
        description: &'static str,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        ListItem::new(ctx, false, title, None, Some(subtitle), Some(description), None, None, Some(selected), None, Uuid::new_v4(), on_click)
    }
}

#[derive(Debug, Component)]
pub struct ListItemSelector(Column, ListItem, ListItem, Option<ListItem>, Option<ListItem>);
impl Events for ListItemSelector {}

impl ListItemSelector {
    pub fn new(
        ctx: &mut Context, 
        first: (&'static str, &'static str, &'static str), //title,subtitle,description
        second: (&'static str, &'static str, &'static str), 
        third: Option<(&'static str, &'static str, &'static str)>, 
        fourth: Option<(&'static str, &'static str, &'static str)>
    ) -> Self {
        ListItemSelector(Column::center(0.0), 
            ListItem::selection(ctx, true, first.0, first.1, first.2, |_: &mut Context| ()),
            ListItem::selection(ctx, false, second.0, second.1, second.2, |_: &mut Context| ()),
            third.map(|third| ListItem::selection(ctx, false, third.0, third.1, third.2, |_: &mut Context| ())),
            fourth.map(|fourth| ListItem::selection(ctx, false, fourth.0, fourth.1, fourth.2, |_: &mut Context| ())),
        )
    }
}

#[derive(Debug, Component)]
pub struct ListItemGroup(Column, Vec<ListItem>);
impl Events for ListItemGroup {}

impl ListItemGroup {
    pub fn new(items: Vec<ListItem>) -> Self {
        ListItemGroup(Column::center(0.0), items)
    }
}


#[derive(Debug, Component)]
pub struct QuickDeselect(Column, Option<QuickDeselectContent>, ListItemGroup);

impl QuickDeselect {
    pub fn new(list_items: Vec<ListItem>) -> Self {
        QuickDeselect(
            Column(24.0, Offset::Start, Size::Fit, Padding::default()), 
            None, ListItemGroup::new(list_items)
        )
    }
}

impl Events for QuickDeselect {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(AddContactEvent(name, id)) = event.downcast_ref::<AddContactEvent>() {
            let button = QuickDeselectButton::new(ctx, name, *id);
            match &mut self.1 {
                Some(select) => {
                    if !select.1.iter().any(|b| b.id() == *id) {select.1.push(button)}
                },
                None => self.1 = Some(QuickDeselectContent::new(button)),
            }
        } else if let Some(RemoveContactEvent(id)) = event.downcast_ref::<RemoveContactEvent>() {
            if let Some(select) = &mut self.1 {
                if select.1.len() == 1 {
                    self.1 = None;
                } else {
                    select.1.retain(|button| button.id() != *id);
                }
            }
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct QuickDeselectContent(Wrap, Vec<QuickDeselectButton>);
impl Events for QuickDeselectContent {}

impl QuickDeselectContent {
    pub fn new(first: QuickDeselectButton) -> Self {
        QuickDeselectContent(
            Wrap(8.0, 8.0, Offset::Start, Offset::Center, Padding::default()), 
            vec![first],
        )
    }
}
