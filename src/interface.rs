use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Brand;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::{Rectangle};
use crate::components::mobile_keyboard::MobileKeyboard;
use crate::components::button::{Button, IconButton, ButtonColumn};
use crate::components::avatar::{AvatarIconStyle, AvatarContent, AvatarRow};
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
struct MobileInterface(Column, Page, MobileNavigator, Option<MobileKeyboard>);
impl Events for MobileInterface {}

impl MobileInterface {
    pub fn new(ctx: &mut Context, page: Page) -> Self {
        let navigator = MobileNavigator::new(ctx);
        let insets = safe_area_insets();
        MobileInterface(
            Column(0, Offset::Center, Size::Fit, Padding(0, insets.0 as u32, 0, insets.1 as u32)), 
            page, navigator, None
        )
    }

    pub fn open_keyboard(&mut self, ctx: &mut Context) {
        self.3 = Some(MobileKeyboard::new(ctx));
    }
}

#[derive(Debug, Component)]
struct DesktopInterface(Row, DesktopNavigator, Bin<Stack, Rectangle>, Page);
impl Events for DesktopInterface {}

impl DesktopInterface {
    pub fn new(ctx: &mut Context, page: Page) -> Self {
        let navigator = DesktopNavigator::new(ctx);
        let color = ctx.get::<PelicanUI>().theme.colors.outline.secondary;
        DesktopInterface(
            Row(0, Offset::Start, Size::Fit, Padding::default()),
            navigator, 
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Static(1),  Size::Fit, Padding::default()), 
                Rectangle::new(color)
            ),
           page
        )
    }
}

#[derive(Debug, Component)]
struct MobileNavigator(Row, Vec<IconButton>);
impl Events for MobileNavigator {}

impl MobileNavigator {
    pub fn new(ctx: &mut Context) -> Self {
        MobileNavigator(Row(48, Offset::Center, Size::Fit, Padding(0, 8, 0, 8)), vec![
            IconButton::tab_nav(ctx, "wallet", true, |_ctx: &mut Context| println!("Bitcoin")),
            IconButton::tab_nav(ctx, "messages", false, |_ctx: &mut Context| println!("Messaging")),
            IconButton::tab_nav(ctx, "door", false, |_ctx: &mut Context| println!("Rooms")),
            IconButton::tab_nav(ctx, "profile", false, |_ctx: &mut Context| println!("Profile"))
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
        let bitcoin = Button::navigation(ctx, "wallet", "Bitcoin", true, |_ctx: &mut Context| println!("Bitcoin"));
        let messages = Button::navigation(ctx, "messages", "Messages", false, |_ctx: &mut Context| println!("Messaging"));
        let rooms = Button::navigation(ctx, "door", "Rooms", false, |_ctx: &mut Context| println!("Rooms"));
        DesktopNavigator(
            Column(32, Offset::Center, Size::Fill(100, 200), Padding(16, 32, 16, 32)),
            Brand::new(wordmark, (80, 44)),
            ButtonColumn::new(vec![bitcoin, messages, rooms]),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100, 200), Size::Fill(100, u32::MAX),  Padding::default()), 
                Rectangle::new(color)
            ),
            Button::photo(ctx, "My Profile", AvatarContent::Icon("profile", AvatarIconStyle::Secondary), false, |_ctx: &mut Context| println!("Profile"))
        )
    }
}




#[derive(Debug, Component)]
pub struct Page (Column, Header, Content, Option<Bumper>); // todo mobilekeyboard into interface
impl Events for Page {}

impl Page {
    pub fn new(header: Header, content: Content, bumper: Option<Bumper>) -> Self {
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[1].0, u32::MAX));
        Page(
            Column(12, Offset::Center, width, Padding::default()),
            header,
            content,
            bumper,
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
            Row(16, Offset::Center, Size::Fit, Padding(24, 24, 24, 24)),
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
            Row(16, Offset::Center, Size::Fit, Padding(24, 24, 24, 24)),
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
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[0].0, u32::MAX));
        HeaderContent(
            Column(10, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size),
            None,
        )
    }

    pub fn stack(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h4;
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[0].0, u32::MAX));
        HeaderContent(
            Column(10, Offset::Center, width, Padding::default()),  
            Text::new(ctx, title, TextStyle::Heading, text_size),
            None,
        )
    }

    pub fn chat(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let title = if avatars.len() > 1 {"Ella Couch"} else {"Group Message"};
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[0].0, u32::MAX));
        HeaderContent(
            Column(10, Offset::Center, width, Padding::default()), 
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
            Stack(Offset::Center, Offset::Center, Size::Static(32), Size::Static(32), Padding::default()),
            icon
        )
    }
}

#[derive(Debug, Component)]
pub struct Bumper (Stack, BumperContent);
impl Events for Bumper {}

impl Bumper {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[0].0, 375));
        Bumper(
            Stack(Offset::Center, Offset::Start, width, Size::Fit, Padding(24, 16, 24, 16)),
            BumperContent::new(ctx, content)
        )
    }
}

#[derive(Debug, Component)]
pub struct BumperContent (Row, Vec<Box<dyn Drawable>>);
impl Events for BumperContent {}

impl BumperContent {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        BumperContent(Row::center(16), content)
    }
}

#[derive(Debug, Component)]
pub struct Content (Stack, ContentChildren);
impl Events for Content {}

impl Content {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(u32, u32)>|(widths[0].0, 375));
        let height = Size::custom(move |heights: Vec<(u32, u32)>|(heights[0].0, u32::MAX));
        Content(
            Stack(Offset::Center, Offset::Start, width, height, Padding(24, 16, 24, 16)),
            ContentChildren::new(ctx, content),
        )
    }
}

#[derive(Debug, Component)]
pub struct ContentChildren (Column, Vec<Box<dyn Drawable>>);
impl Events for ContentChildren {}

impl ContentChildren {
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        ContentChildren(Column::center(24), content)
    }
}

extern "C" {
    fn get_safe_area_insets() -> *const f64;
}

pub fn safe_area_insets() -> (f64, f64, f64, f64) {
    unsafe {
        let ptr = get_safe_area_insets();
        (
            *ptr.add(0), // top
            *ptr.add(1), // bottom
            *ptr.add(2), // left
            *ptr.add(3), // right
        )
    }
}
