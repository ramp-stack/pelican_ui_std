use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::{Icon, Brand};
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::mobile_keyboard::MobileKeyboard;
use crate::components::button::{ButtonState, Button, IconButton};
use crate::components::avatar::{Avatar, AvatarIconStyle, AvatarContent};
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::PelicanUI;

#[cfg(any(target_os = "ios", target_os = "android"))]
const IS_MOBILE: bool = true;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
const IS_MOBILE: bool = false;


#[derive(Debug, Component)]
pub struct Interface (Stack, Option<MobileInterface>, Option<DesktopInterface>);
impl Events for Interface {}

impl Interface {
    pub fn new(
        ctx: &mut Context,
        page: Page,
    ) -> Self {
        let (mobile, desktop) = match IS_MOBILE {
            true => (Some(MobileInterface::new(ctx, page)), None),
            false => (None, Some(DesktopInterface::new(ctx, page)))
        };
        Interface(Stack::default(), mobile, desktop)
    }
}

#[derive(Debug, Component)]
struct MobileInterface(Column, Page, MobileNavigator);
impl Events for MobileInterface {}

impl MobileInterface {
    pub fn new(ctx: &mut Context, page: Page) -> Self {
        let navigator = MobileNavigator::new(ctx);
        MobileInterface(Column::center(0), page, navigator)
    }
}

#[derive(Debug, Component)]
struct DesktopInterface(Row, DesktopNavigator, Page);
impl Events for DesktopInterface {}

impl DesktopInterface {
    pub fn new(ctx: &mut Context, page: Page) -> Self {
        let navigator = DesktopNavigator::new(ctx);
        DesktopInterface(Row::center(0), navigator, page)
    }
}

#[derive(Debug, Component)]
struct MobileNavigator(Row, Vec<IconButton>);
impl Events for MobileNavigator {}

impl MobileNavigator {
    pub fn new(ctx: &mut Context) -> Self {
        MobileNavigator(Row::center(64), vec![
            IconButton::navigation(ctx, "wallet", true, |ctx: &mut Context| println!("Bitcoin")),
            IconButton::navigation(ctx, "messages", false, |ctx: &mut Context| println!("Messaging")),
            IconButton::navigation(ctx, "door", false, |ctx: &mut Context| println!("Rooms")),
            IconButton::navigation(ctx, "profile", false, |ctx: &mut Context| println!("Profile"))
        ])
    }
}

#[derive(Debug, Component)]
struct DesktopNavigator(Column, Image, Vec<Button>, Button);
impl Events for DesktopNavigator {}

impl DesktopNavigator {
    pub fn new(ctx: &mut Context) -> Self {
        let wordmark = ctx.get::<PelicanUI>().theme.brand.wordmark.clone();
        DesktopNavigator(
            Column(32, Offset::Center, Size::Fill(100, 200), Padding(16, 32, 16, 32)),
            Brand::new(ctx, wordmark, (81, 45)),
            vec![
                Button::navigation(ctx, "wallet", "Bitcoin", true, |ctx: &mut Context| println!("Bitcoin")),
                Button::navigation(ctx, "messages", "Messages", false, |ctx: &mut Context| println!("Messaging")),
                Button::navigation(ctx, "door", "Rooms", false, |ctx: &mut Context| println!("Rooms")),
            ],
            Button::photo(ctx, "My Profile", AvatarContent::Icon("profile", AvatarIconStyle::Secondary), false, |ctx: &mut Context| println!("Profile"))
        )
    }
}

#[derive(Debug, Component)]
pub struct Page (Column, Header, Content, Option<Bumper>, Option<MobileKeyboard>);
impl Events for Page {}

impl Page {
    pub fn new(
        ctx: &mut Context,
        header: Header,
        content: Content,
        bumper: Option<Bumper>,
        keyboard: Option<MobileKeyboard>,
    ) -> Self {
        Page(
            Column(0, Offset::Center, Size::Fill(0, u32::MAX), Padding::default()),
            header,
            content,
            bumper,
            keyboard,
        )
    }
}

#[derive(Debug, Component)]
pub struct Header(Row, HeaderIcon, HeaderContent, HeaderIcon);
impl Events for Header {}

impl Header {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        Header(
            Row(16, Offset::Center, Size::Fit, Padding(24, 24, 24, 24)),
            HeaderIcon::new(ctx, None), 
            HeaderContent::home(ctx, title),
            HeaderIcon::new(ctx, None)
        )
    }

    pub fn stack(
        ctx: &mut Context, 
        left: Option<IconButton>, 
        title: &'static str, 
        right: Option<IconButton>
    ) -> Self {
        Header(
            Row(16, Offset::Center, Size::Fit, Padding(24, 24, 24, 24)),
            HeaderIcon::new(ctx, left), 
            HeaderContent::stack(ctx, title), 
            HeaderIcon::new(ctx, right)
        )
    }

    pub fn chat(
        ctx: &mut Context, 
        left: Option<IconButton>, 
        title: &'static str, 
        right: Option<IconButton>,
        avatars: Vec<AvatarContent>,
    ) -> Self {
        Header(
            Row(16, Offset::Center, Size::Fit, Padding(24, 24, 24, 24)),
            HeaderIcon::new(ctx, left), 
            HeaderContent::chat(ctx, avatars), 
            HeaderIcon::new(ctx, right)
        )
    }
}

#[derive(Debug, Component)]
struct HeaderContent(Column, BasicText, Option<AvatarRow>);
impl Events for HeaderContent {}

impl HeaderContent {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h3;
        HeaderContent(
            Column::center(10), 
            Text::new(ctx, title, TextStyle::Heading, text_size),
            None,
        )
    }

    pub fn stack(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h4;
        HeaderContent(
            Column::center(10), 
            Text::new(ctx, title, TextStyle::Heading, text_size),
            None,
        )
    }

    pub fn chat(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let title = if avatars.len() > 1 {"Ella Couch"} else {"Group Message"};
        HeaderContent(
            Column::center(10), 
            Text::new(ctx, title, TextStyle::Heading, text_size),
            Some(AvatarRow::new(ctx, avatars)),
        )
    }
}

#[derive(Debug, Component)]
struct AvatarRow(Row, Vec<Avatar>);
impl Events for AvatarRow {}

impl AvatarRow {
    pub fn new(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = &ctx.get::<PelicanUI>().theme.fonts.size.h5;
        AvatarRow(
            Row::center(0), 
            avatars.into_iter().map(|avatar| Avatar::new(ctx, avatar, None, true, 32)).collect()
        )
    }
}



#[derive(Debug, Component)]
struct HeaderIcon(Stack, Option<IconButton>);
impl Events for HeaderIcon {}

impl HeaderIcon {
    pub fn new(ctx: &mut Context, icon: Option<IconButton>) -> Self {
        HeaderIcon(
            Stack(Offset::Center, Offset::Center, Size::Static(32), Size::Static(32), Padding::default()),
            icon
        )
    }
}

#[derive(Debug, Component)]
pub struct Bumper (Row, Vec<Child>);
impl Events for Bumper {}

impl Bumper {
    pub fn new(ctx: &mut Context, content: Vec<Child>) -> Self {
        Bumper(Row(16, Offset::Center, Size::Fill(10, u32::MAX), Padding(24, 16, 24, 16)), content)
    }
}

#[derive(Debug, Component)]
pub struct Content (Column, Vec<Child>);
impl Events for Content {}

impl Content {
    pub fn new(ctx: &mut Context, content: Vec<Child>) -> Self {
        Content(Column(24, Offset::Center, Size::Fill(10, u32::MAX), Padding::default()), content)
    }
}

#[derive(Debug, Component)]
pub struct Child (pub Stack, pub Box<dyn Drawable>);
impl Events for Child {}

impl Child {
    pub fn new(d: impl Drawable + 'static) -> Self {
        Child(Stack::default(), Box::new(d)) 
    }
    pub fn inner(&mut self) -> &mut D {&mut self.1}
}