use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::events::ListItemSelect;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::ButtonState;
use crate::components::avatar::{Avatar, AvatarIconStyle, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::PelicanUI;
use uuid::Uuid;


#[derive(Debug, Component)]
pub struct ListItem(Stack, Rectangle, ListItemContent, #[skip] ButtonState, #[skip] fn(&mut Context), #[skip] Uuid);

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
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.background.primary;
        let content = ListItemContent::new(
            ctx, caret, title, flair, subtitle, description, right_title, 
            right_subtitle, radio_button, circle_icon
        );
        let layout = Stack(
            Offset::Start, Offset::Center, 
            Size::custom(|widths: Vec<(u32, u32)>| (widths[1].0, u32::MAX)), 
            Size::custom(|heights: Vec<(u32, u32)>| heights[1]), 
            Padding(0, 16, 0, 16)
        );

        ListItem(layout, Rectangle::new(color), content, ButtonState::Default, on_click, Uuid::new_v4())
    }
}

impl Events for ListItem {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                self.2.1.as_mut().map(|mut radio| {radio.select(ctx); ctx.trigger_event(ListItemSelect(self.5));});
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
                self.2.1.as_mut().map(|mut radio| radio.deselect(ctx));
            }
        }
        false
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
            Row(16, Offset::Center, Size::Fit, Padding::default()),
            radio_button.map(|enabled| RadioButton::new(ctx, enabled)), 
            circle_icon.map(|data| Avatar::new(ctx, data, None, false, 48.0)),
            ListItemData::new(ctx, title, flair, subtitle, description, right_title, right_subtitle),
            caret.then(|| Icon::new(ctx, "forward", color, 16)),
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
        RadioButton(Row::center(0), Icon::new(ctx, icon, color, 32))
    }

    pub fn select(&mut self, ctx: &mut Context) {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio_filled", color, 32);
    }

    pub fn deselect(&mut self, ctx: &mut Context) {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        self.1 =  Icon::new(ctx, "radio", color, 32);
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
        ListItemData (
            Row(8, Offset::Start, Size::Fit, Padding::default()),
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
            Row(8, Offset::Start, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size),
            flair.map(|(name, color)| Icon::new(ctx, name, color, 20)),
        )
    }
}

#[derive(Debug, Component)]
struct LeftData(pub Column, pub TitleRow, pub Option<BasicText>, pub Option<BasicText>);
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
            Column(4, Offset::Start, Size::custom(|widths: Vec<(u32, u32)>| (widths[1].0, u32::MAX)), Padding::default()),
            TitleRow::new(ctx, title, flair),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size)),
            description.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size)),
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
            Column(4, Offset::End, Size::Fit, Padding::default()),
            Text::new(ctx, title, TextStyle::Heading, font_size.h5),
            subtitle.map(|text| Text::new(ctx, text, TextStyle::Secondary, font_size.xs)),
        )
    }
}

#[derive(Debug, Component)]
pub struct ListItemGroup(Column, Vec<ListItem>);
impl Events for ListItemGroup {}

impl ListItemGroup {
    pub fn new(ctx: &mut Context, items: Vec<ListItem>) -> Self {
        ListItemGroup(Column::center(0), items)
    }
}

impl ListItem {
    pub fn contact(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        nym: &'static str,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        ListItem::new(ctx, true, name, None, Some(nym), None, None, None, None, Some(data), on_click)
    }

    pub fn direct_message(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        recent: &'static str,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        ListItem::new(ctx, true, name, None, Some(recent), None, None, None, None, Some(data), on_click)
    }

    pub fn group_message(
        ctx: &mut Context,
        names: Vec<&'static str>,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        let description = Box::leak(names.join(", ").into_boxed_str());
        let avatar = AvatarContent::Icon("group", AvatarIconStyle::Secondary);
        ListItem::new(ctx, true, "Group Message", None, None, Some(description), None, None, None, Some(avatar), on_click)
    }

    pub fn room(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        members: &'static str,
        description: &'static str,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        ListItem::new(ctx, true, name, None, Some(members), Some(description), None, None, None, Some(data), on_click)
    }

    pub fn bitcoin(
        ctx: &mut Context,
        is_received: bool,
        usd: f32,
        date: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let title = if is_received { "Received Bitcoin" } else { "Sent Bitcoin" };
        let usd = Box::leak(format!("{:.2}", usd).into_boxed_str());
        ListItem::new(ctx, true, title, None, Some(date), None, Some(usd), Some("Details"), None, None, on_click)
    }

    pub fn bitcoin_sending(
        ctx: &mut Context,
        usd: f32,
        btc: f32,
        date: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.status.warning;
        let flair = ("warning", color);
        let usd =  Box::leak(format!("${:.2}", usd).into_boxed_str());
        let btc =  Box::leak(format!("${:.8} BTC", btc).into_boxed_str());
        ListItem::new(ctx, true, "Sending Bitcoin", Some(flair), Some(date), None, Some(usd), Some(btc), None, None, on_click)
    }

    pub fn selection(
        ctx: &mut Context,
        selected: bool,
        title: &'static str,
        subtitle: &'static str,
        description: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        ListItem::new(ctx, false, title, None, Some(subtitle), Some(description), None, None, Some(selected), None, on_click)
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
        ListItemSelector(Column::center(0), 
            ListItem::selection(ctx, true, first.0, first.1, first.2, |_: &mut Context| ()),
            ListItem::selection(ctx, false, second.0, second.1, second.2, |_: &mut Context| ()),
            third.map(|third| ListItem::selection(ctx, false, third.0, third.1, third.2, |_: &mut Context| ())),
            fourth.map(|fourth| ListItem::selection(ctx, false, fourth.0, fourth.1, fourth.2, |_: &mut Context| ())),
        )
    }
}