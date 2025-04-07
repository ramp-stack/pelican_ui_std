use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::ButtonState;
use crate::components::avatar::{Avatar, AvatarIconStyle, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::PelicanUI;


#[derive(Debug, Component)]
pub struct ListItem(Stack, Rectangle, ListItemContent, #[skip] ButtonState, #[skip] fn(&mut Context));

impl ListItem {
    pub fn new(
        ctx: &mut Context,
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
            ctx, title, flair, subtitle, description, right_title, 
            right_subtitle, radio_button, circle_icon
        );
        let layout = Stack(
            Offset::Start, Offset::Center, 
            Size::custom(|widths: Vec<(u32, u32)>| (widths[1].0, u32::MAX)), 
            Size::custom(|heights: Vec<(u32, u32)>| heights[1]), 
            Padding::default()
        );

        ListItem(layout, Rectangle::new(color), content, ButtonState::Default, on_click)
    }
}

impl Events for ListItem {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        println!("event: {:?}", event);
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            println!("mouse: {:?}", event);
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.3 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => (self.4)(ctx),
                    _ => {}
                }
            }
            false
        } else {true}
    }
}


#[derive(Debug, Component)]
pub struct ListItemContent(Row, Option<RadioButton>, Option<Avatar>, ListItemData);
impl Events for ListItemContent {}

impl ListItemContent {
    pub fn new(
        ctx: &mut Context,
        title: &'static str,
        flair: Option<(&'static str, Color)>,
        subtitle: Option<&'static str>,
        description: Option<&'static str>,
        right_title: Option<&'static str>,
        right_subtitle: Option<&'static str>,
        radio_button: Option<bool>,
        circle_icon: Option<AvatarContent>,
    ) -> Self {
        ListItemContent(
            Row(16, Offset::Center, Size::Fit, Padding::default()),
            radio_button.map(|enabled| RadioButton::new(ctx, enabled)), 
            circle_icon.map(|data| Avatar::new(ctx, data, None, false, 48)),
            ListItemData::new(ctx, title, flair, subtitle, description, right_title, right_subtitle),
        )
    }
}

#[derive(Debug, Component)]
struct RadioButton(Row, Image);
impl Events for RadioButton {}

impl RadioButton {
    pub fn new(ctx: &mut Context, is_enabled: bool) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.text.heading;
        let icon = if is_enabled { "radio_filled" } else { "radio "};
        RadioButton(Row::center(0), Icon::new(ctx, icon, color, 32))
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

impl ListItem {
    pub fn contact(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        nym: &'static str,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        ListItem::new(ctx, name, None, Some(nym), None, None, None, None, Some(data), on_click)
    }

    pub fn direct_message(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        recent: &'static str,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        ListItem::new(ctx, name, None, Some(recent), None, None, None, None, Some(data), on_click)
    }

    pub fn group_message(
        ctx: &mut Context,
        names: Vec<&'static str>,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        let description = Box::leak(names.join(", ").into_boxed_str());
        let avatar = AvatarContent::Icon("group", AvatarIconStyle::Secondary);
        ListItem::new(ctx, "Group Message", None, None, Some(description), None, None, None, Some(avatar), on_click)
    }

    pub fn room(
        ctx: &mut Context,
        data: AvatarContent,
        name: &'static str,
        members: &'static str,
        description: &'static str,
        on_click: fn(&mut Context) -> ()
    ) -> Self {
        ListItem::new(ctx, name, None, Some(members), Some(description), None, None, None, Some(data), on_click)
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
        ListItem::new(ctx, title, None, Some(date), None, Some(usd), Some("Details"), None, None, on_click)
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
        ListItem::new(ctx, "Sending Bitcoin", Some(flair), Some(date), None, Some(usd), Some(btc), None, None, on_click)
    }

    pub fn selection(
        ctx: &mut Context,
        selected: bool,
        title: &'static str,
        subtitle: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        ListItem::new(ctx, title, None, None, Some(subtitle), None, None, Some(selected), None, on_click)
    }
}
