use rust_on_rails::prelude::*;
use crate::elements::shapes::{Rectangle};
use crate::events::{KeyboardActiveEvent, NavigateEvent};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size, Opt};
use crate::components::avatar::AvatarContent;
use crate::PelicanUI;
use crate::AppPage;
use std::fmt::Debug;

use super::mobile_keyboard::MobileKeyboard;
use super::navigation::{MobileNavigator, DesktopNavigator, Header, Bumper};

#[derive(Debug, Component)]
pub struct Interface (Stack, Option<MobileInterface>, Option<DesktopInterface>);
impl OnEvent for Interface {}

impl Interface {
    /// Creates a new `Interface` component, which is used to initialize the main interface for the application.
    /// The `Interface` is tailored based on the device type (mobile or desktop) and contains information like 
    /// the start page, navigation, and user profile. Depending on the platform (mobile or desktop), it will create 
    /// either a `MobileInterface` or a `DesktopInterface` to manage the layout and navigation of the application.
    ///
    /// # Parameters:
    /// - **`ctx`**: A mutable reference to the context, used to retrieve resources like themes, configurations, and other application state.
    /// - **`start_page`**: The starting page of the application, which should implement the `AppPage` trait. This will be the page shown when the app starts.
    /// - **`navigation`**: A tuple containing:
    ///   - An index `usize` representing the selected page for navigation.
    ///   - A vector of tuples, where each tuple contains:
    ///     - A static string for the label (e.g., the name of the page).
    ///     - A static string for the description (e.g., additional info about the page).
    ///     - A closure that takes a mutable reference to the context and performs actions when the navigation item is clicked.
    /// - **`profile`**: A tuple containing:
    ///   - A static string representing the username or profile identifier.
    ///   - The content for the avatar, which is of type `AvatarContent` (this will be used for displaying the user's avatar).
    ///   - A closure that takes a mutable reference to the context and allows modification of the profile information.
    ///
    /// # Returns:
    /// - **`Interface`**: The constructed `Interface` component, containing the appropriate platform-specific interface (mobile or desktop).
    ///
    /// # Example:
    /// ```rust
    /// let navigation = (0, vec![
    ///     ("Home", "Main page", Box::new(|ctx| { /* Navigate to home page */ })),
    ///     ("Profile", "User profile", Box::new(|ctx| { /* Navigate to profile page */ }))
    /// ]);
    /// let profile = ("Ella Mae", AvatarContent::Icon("profile", AvatarIconStyle::Primary), Box::new(|ctx| { /* View profile */ }));
    /// let interface = Interface::new(ctx, HomePage, navigation, profile);
    /// ```
    pub fn new(
        ctx: &mut Context, 
        start_page: impl AppPage, 
        navigation: (usize, Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)>),
        profile: (&'static str, AvatarContent, Box<dyn FnMut(&mut Context)>),
    ) -> Self {
        let (mobile, desktop) = match crate::config::IS_MOBILE {
            true => (Some(MobileInterface::new(ctx, start_page, navigation, profile)), None),
            false => (None, Some(DesktopInterface::new(ctx, start_page, navigation, profile)))
        };
        Interface(Stack::default(), mobile, desktop)
    }
}

#[derive(Debug, Component)]
struct MobileInterface (Column, Box<dyn AppPage>, Opt<MobileNavigator>, Option<MobileKeyboard>);

impl MobileInterface {
    fn new(
        ctx: &mut Context, 
        start_page: impl AppPage,
        navigation: (usize, Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)>),
        profile: (&'static str, AvatarContent, Box<dyn FnMut(&mut Context)>),
    ) -> Self {
        let navigator = MobileNavigator::new(ctx, navigation, profile);
        #[cfg(target_os = "ios")] // move to rust_on_rails layer
        let insets = safe_area_insets();
        #[cfg(not(target_os = "ios"))]
        let insets = (0., 0., 0., 0.);
        MobileInterface(
            Column::new(0.0, Offset::Center, Size::Fit, Padding(0.0, insets.0, 0.0, insets.1)), 
            Box::new(start_page), Opt::new(navigator, false), None,
        )
    }
}

impl OnEvent for MobileInterface {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(_event) = event.downcast_ref::<TickEvent>() {
            // self.2.display(self.1.navigator_status());
        } else if let Some(KeyboardActiveEvent(enabled)) = event.downcast_ref::<KeyboardActiveEvent>() {
            self.3 = match enabled {
                true => Some(MobileKeyboard::new(ctx)),
                false => None
            };
        } else if let Some(NavigateEvent(page)) = event.downcast_ref::<NavigateEvent>() {
            self.1 = page.get_page(ctx);
        }
        true
    }
}

#[derive(Debug, Component)]
struct DesktopInterface (Row, DesktopNavigator, Bin<Stack, Rectangle>, Box<dyn AppPage>);

impl DesktopInterface {
    fn new(
        ctx: &mut Context, 
        start_page: impl AppPage, 
        navigation: (usize, Vec<(&'static str, &'static str, Box<dyn FnMut(&mut Context)>)>),
        profile: (&'static str, AvatarContent, Box<dyn FnMut(&mut Context)>),
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.outline.secondary;
        DesktopInterface(
            Row(0.0, Offset::Start, Size::Fit, Padding::default()),
            DesktopNavigator::new(ctx, navigation, profile), 
            Bin (
                Stack(Offset::default(), Offset::default(), Size::Static(1.0),  Size::Fit, Padding::default()), 
                Rectangle::new(color)
            ),
            Box::new(start_page)
        )
    }
}

impl OnEvent for DesktopInterface {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigateEvent(page)) = event.downcast_ref::<NavigateEvent>() {
            self.3 = page.get_page(ctx);
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct Page (Column, Header, Content, Option<Bumper>, #[skip] bool);
impl OnEvent for Page {}

impl Page {
    /// Creates a new `Page` component that initializes the page layout with a header, content, optional bumper, and navigation status.
    /// The page is structured with a flexible width and a vertical layout that contains the header, content, and optional bumper.
    /// It also supports a flag to manage whether navigation is enabled or not.
    ///
    /// # Parameters:
    /// - **`header`**: The header of the page, which is of type `Header`. It typically contains the title or navigation elements for the page.
    /// - **`content`**: The main content of the page, which is of type `Content`. This part holds the primary information displayed on the page.
    /// - **`bumper`**: An optional `Bumper` component, which could be used for additional UI elements or effects at the bottom of the page.
    /// - **`has_nav`**: A boolean flag that determines whether the page includes a navigation interface (e.g., a bottom navigation bar).
    ///
    /// # Returns:
    /// - **`Page`**: The constructed `Page` component, which consists of a `Column` layout, header, content, optional bumper, and navigation status.
    ///
    /// # Example:
    /// ```rust
    /// let page = Page::new(header, content, Some(bumper), true);
    /// ```
    /// This creates a new page with a header, content, a bumper at the bottom, and navigation enabled.
    ///
    pub fn new(header: Header, content: Content, bumper: Option<Bumper>, has_nav: bool) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[1].0, f32::MAX));
        Page(
            Column::new(12.0, Offset::Center, width, Padding::default()),
            header,
            content,
            bumper,
            has_nav
        )
    }

    /// Returns a mutable reference to the `Header` component of the page.
    pub fn header(&mut self) -> &mut Header {&mut self.1}
    /// Returns a mutable reference to the `Content` component of the page.
    pub fn content(&mut self) -> &mut Content {&mut self.2}
    /// Returns a mutable reference to the `Bumper` component of the page.
    pub fn bumper(&mut self) -> &mut Option<Bumper> {&mut self.3}
    /// Returns the navigation status of the page as a boolean.
    pub fn navigator_status(&self) -> bool {self.4}
}

/// A component that holds and arranges its child elements in a column.
/// The `Content` component is used to display and manage the content area of a page,
/// with the ability to dynamically modify the list of drawable items.
#[derive(Debug, Component)]
pub struct Content (Stack, ContentChildren);

impl Content {
    /// Creates a new `Content` component with the specified layout and child items.
    ///
    /// # Parameters:
    /// - `offset`: The offset for positioning the content relative to its parent.
    /// - `content`: A vector of `Box<dyn Drawable>` elements representing the items to display in the content area.
    ///
    /// # Returns:
    /// - **`Content`**: A new `Content` component that contains the stack layout and the provided items.
    ///
    pub fn new(offset: Offset, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0.min(375.0), 375.0));
        let height = Size::custom(move |_: Vec<(f32, f32)>|(0.0, f32::MAX));
        Content(
            Stack(Offset::Center, offset, width, height, Padding(24.0, 0.0, 24.0, 0.0)),
            ContentChildren::new(content),
        )
    }
    
    /// Returns a mutable reference to the list of items contained within the `Content` component.
    ///
    /// This allows the user to dynamically add, remove, or modify the content items after the
    /// `Content` component has been created.
    ///
    /// # Returns:
    /// - **`&mut Vec<Box<dyn Drawable>>`**: A mutable reference to the vector of drawable items.
    pub fn items(&mut self) -> &mut Vec<Box<dyn Drawable>> {&mut self.1.1}
}

impl OnEvent for Content {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let MouseEvent{state: MouseState::Scroll(_, y), ..} = event {
                *self.1.0.scroll() += y;
                *self.1.0.scroll() = self.1.0.scroll().clamp(0.0, 100.); // 100 = content height
            }
        }
        true
    }
}

#[derive(Debug, Component)]
struct ContentChildren (Column, Vec<Box<dyn Drawable>>);
impl OnEvent for ContentChildren {}

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
pub fn safe_area_insets() -> (f32, f32, f32, f32) {
    unsafe {
        let ptr = get_safe_area_insets();
        (
            *ptr.add(0) as f32, // top
            *ptr.add(1) as f32, // bottom
            *ptr.add(2) as f32, // left
            *ptr.add(3) as f32, // right
        )
    }
}
