use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::{Icon, Brand};
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::{Rectangle};
use crate::components::mobile_keyboard::MobileKeyboard;
use crate::components::button::{ButtonState, Button, IconButton};
use crate::components::avatar::{Avatar, AvatarIconStyle, AvatarContent, AvatarRow};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct Interface (Stack, Option<MobileInterface>, Option<DesktopInterface>);
impl Events for Interface {}

impl Interface {
    pub fn new(
        ctx: &mut Context,
        page: Page,
    ) -> Self {
        let (mobile, desktop) = match crate::config::IS_MOBILE {
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
        DesktopInterface(Row(0, Offset::Start, Size::Fit, Padding::default()), navigator, page)
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
struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, Button);
impl Events for DesktopNavigator {}

impl DesktopNavigator {
    pub fn new(ctx: &mut Context) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (wordmark, color) = (theme.brand.wordmark.clone(), theme.colors.shades.transparent);
        let bitcoin = Button::navigation(ctx, "wallet", "Bitcoin", true, |ctx: &mut Context| println!("Bitcoin"));
        let messages = Button::navigation(ctx, "messages", "Messages", false, |ctx: &mut Context| println!("Messaging"));
        let rooms = Button::navigation(ctx, "door", "Rooms", false, |ctx: &mut Context| println!("Rooms"));
        DesktopNavigator(
            Column(32, Offset::Center, Size::Fill(100, 200), Padding(16, 32, 16, 32)),
            Brand::new(ctx, wordmark, (81, 45)),
            ButtonColumn::new(ctx, vec![bitcoin, messages, rooms]),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100, 200), Size::Fill(100, u32::MAX),  Padding::default()), 
                Rectangle::new(color)
            ),
            Button::photo(ctx, "My Profile", AvatarContent::Icon("profile", AvatarIconStyle::Secondary), false, |ctx: &mut Context| println!("Profile"))
        )
    }
}

#[derive(Debug, Component)]
struct ButtonColumn(Column, Vec<Button>);
impl Events for ButtonColumn {}

impl ButtonColumn {
    pub fn new(ctx: &mut Context, buttons: Vec<Button>) -> Self {
        ButtonColumn(
            Column(8, Offset::Center, Size::Fit, Padding::default()),
            buttons
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

#[derive(Debug)]
pub struct Bumper (Row, Vec<Box<dyn Drawable>>);
impl Events for Bumper {}

impl Bumper {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        Bumper(Row(16, Offset::Center, Size::Fill(10, u32::MAX), Padding(24, 16, 24, 16)), content)
    }
}

impl Component for Bumper {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {
        self.1.iter_mut().map(|c| &mut **c as &mut dyn Drawable).collect()
    }
    fn children(&self) -> Vec<&dyn Drawable> {
        self.1.iter().map(|c| &**c as & dyn Drawable).collect()
    }
    fn request_size(&self, ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        self.0.request_size(ctx, children)
    }
    fn build(&mut self, ctx: &mut Context, size: (u32, u32), children: Vec<SizeRequest>) -> Vec<Area> {
        self.0.build(ctx, size, children)
    }
}

#[derive(Debug)]
pub struct Content (Column, Vec<Box<dyn Drawable>>);
impl Events for Content {}

impl Content {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[1].0, 512));
        Content(Column(24, Offset::Center, width, Padding::default()), content)
    }
}


impl Component for Content {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {
        self.1.iter_mut().map(|c| &mut **c as &mut dyn Drawable).collect()
    }
    fn children(&self) -> Vec<&dyn Drawable> {
        self.1.iter().map(|c| &**c as & dyn Drawable).collect()
    }
    fn request_size(&self, ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        self.0.request_size(ctx, children)
    }
    fn build(&mut self, ctx: &mut Context, size: (u32, u32), children: Vec<SizeRequest>) -> Vec<Area> {
        self.0.build(ctx, size, children)
    }
}
