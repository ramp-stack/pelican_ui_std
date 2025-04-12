use rust_on_rails::prelude::*;
use crate::elements::shapes::{Rectangle};
use crate::events::{SummonKeyboardEvent, HideKeyboardEvent};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::PelicanUI;

use super::mobile_keyboard::MobileKeyboard;
use super::navigation::{MobileNavigator, DesktopNavigator, Header};

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
struct MobileInterface(Column, Page, Option<MobileNavigator>, Option<MobileKeyboard>);

impl MobileInterface {
    pub fn new(ctx: &mut Context, page: Page) -> Self {
        let _navigator = MobileNavigator::new(ctx);
        #[cfg(target_os = "ios")]
        let insets = safe_area_insets();
        #[cfg(not(target_os = "ios"))]
        let insets = (0., 0., 0., 0.);
        MobileInterface(
            Column(0.0, Offset::Center, Size::Fit, Padding(0.0, insets.0, 0.0, insets.1)), 
            page, None, None
        )
    }
}

impl Events for MobileInterface {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(_event) = event.downcast_ref::<SummonKeyboardEvent>() {
            self.3 = Some(MobileKeyboard::new(ctx));
        } else if let Some(_event) = event.downcast_ref::<HideKeyboardEvent>() {
            self.3 = None;
        }
        true
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
            Row(0.0, Offset::Start, Size::Fit, Padding::default()),
            navigator, 
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Static(1.0),  Size::Fit, Padding::default()), 
                Rectangle::new(color)
            ),
           page
        )
    }
}

#[derive(Debug, Component)]
pub struct Page (Column, Header, Content, Option<Bumper>);
impl Events for Page {}

impl Page {
    pub fn new(header: Header, content: Content, bumper: Option<Bumper>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[1].0, f32::MAX));
        Page(
            Column(12.0, Offset::Center, width, Padding::default()),
            header,
            content,
            bumper,
        )
    }
}

#[derive(Debug, Component)]
pub struct Bumper (Stack, BumperContent);
impl Events for Bumper {}

impl Bumper {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, 375.0));
        Bumper(
            Stack(Offset::Center, Offset::Start, width, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            BumperContent::new(content)
        )
    }
}

#[derive(Debug, Component)]
pub struct BumperContent (Row, Vec<Box<dyn Drawable>>);
impl Events for BumperContent {}

impl BumperContent {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        BumperContent(Row::center(16.0), content)
    }
}

#[derive(Debug, Component)]
pub struct Content (Stack, ContentChildren);
impl Events for Content {}

impl Content {
    pub fn new(offset: Offset, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, 375.0));
        let height = Size::custom(move |heights: Vec<(f32, f32)>|(heights[0].0, f32::MAX));
        Content(
            Stack(Offset::Center, offset, width, height, Padding(24.0, 0.0, 24.0, 0.0)),
            ContentChildren::new(content),
        )
    }
}

#[derive(Debug, Component)]
struct ContentChildren (Column, Vec<Box<dyn Drawable>>);
impl Events for ContentChildren {}

impl ContentChildren {
    pub fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        ContentChildren(Column::center(24.0), content)
    }
}


#[cfg(target_os = "ios")]
extern "C" {
    fn get_safe_area_insets() -> *const f64;
}

#[cfg(target_os = "ios")]
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
