use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Brand;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::{Button, IconButton, ButtonColumn};
use crate::components::avatar::{AvatarIconStyle, AvatarContent, AvatarRow};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct MobileNavigator(Row, Vec<IconButton>);
impl Events for MobileNavigator {}

impl MobileNavigator {
    pub fn new(ctx: &mut Context) -> Self {
        MobileNavigator(Row(48.0, Offset::Center, Size::Fit, Padding(0.0, 8.0, 0.0, 8.0)), vec![
            IconButton::tab_nav(ctx, "wallet", true, |_ctx: &mut Context| println!("Bitcoin")),
            IconButton::tab_nav(ctx, "messages", false, |_ctx: &mut Context| println!("Messaging")),
            IconButton::tab_nav(ctx, "door", false, |_ctx: &mut Context| println!("Rooms")),
            IconButton::tab_nav(ctx, "profile", false, |_ctx: &mut Context| println!("Profile"))
        ])
    }
}

#[derive(Debug, Component)]
pub struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, Button);
impl Events for DesktopNavigator {}

impl DesktopNavigator {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (wordmark, color) = (theme.brand.wordmark.clone(), theme.colors.shades.transparent);
        let bitcoin = Button::navigation(ctx, "wallet", "Bitcoin", true, |_ctx: &mut Context| println!("Bitcoin"));
        let messages = Button::navigation(ctx, "messages", "Messages", false, |_ctx: &mut Context| println!("Messaging"));
        let rooms = Button::navigation(ctx, "door", "Rooms", false, |_ctx: &mut Context| println!("Rooms"));
        DesktopNavigator(
            Column(32.0, Offset::Center, Size::Fill(100.0, 200.0), Padding(16.0, 32.0, 16.0, 32.0)),
            Brand::new(wordmark, (80.0, 44.0)),
            ButtonColumn::new(vec![bitcoin, messages, rooms]),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100.0, 200.0), Size::Fill(100.0, f32::MAX),  Padding::default()), 
                Rectangle::new(color)
            ),
            Button::photo(ctx, "My Profile", AvatarContent::Icon("profile", AvatarIconStyle::Secondary), false, |_ctx: &mut Context| println!("Profile"))
        )
    }
}


#[derive(Debug, Component)]
pub struct Header(Row, HeaderIcon, HeaderContent, HeaderIcon);
impl Events for Header {}

impl Header {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(None), 
            HeaderContent::home(ctx, title),
            HeaderIcon::new(None)
        )
    }

    pub fn stack(
        ctx: &mut Context, 
        left: Option<IconButton>, 
        title: &'static str, 
        right: Option<IconButton>
    ) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(left), 
            HeaderContent::stack(ctx, title), 
            HeaderIcon::new(right)
        )
    }

    pub fn chat(
        ctx: &mut Context, 
        left: Option<IconButton>,
        right: Option<IconButton>,
        avatars: Vec<AvatarContent>,
    ) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(left), 
            HeaderContent::chat(ctx, avatars), 
            HeaderIcon::new(right)
        )
    }
}

#[derive(Debug, Component)]
struct HeaderContent(Column, BasicText, Option<AvatarRow>);
impl Events for HeaderContent {}

impl HeaderContent {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h3;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size),
            None,
        )
    }

    pub fn stack(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h4;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column(10.0, Offset::Center, width, Padding::default()),  
            Text::new(ctx, title, TextStyle::Heading, text_size),
            None,
        )
    }

    pub fn chat(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let title = if avatars.len() > 1 {"Ella Couch"} else {"Group Message"};
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size),
            Some(AvatarRow::new(ctx, avatars)),
        )
    }
}

#[derive(Debug, Component)]
struct HeaderIcon(Stack, Option<IconButton>);
impl Events for HeaderIcon {}

impl HeaderIcon {
    pub fn new(icon: Option<IconButton>) -> Self {
        HeaderIcon(
            Stack(Offset::Center, Offset::Center, Size::Static(48.0), Size::Static(48.0), Padding::default()),
            icon
        )
    }
}
