use rust_on_rails::prelude::*;
use crate::elements::shapes::{Rectangle};
use crate::elements::text::TextStyle;
use crate::layout::{Column, Stack, Row, Padding, Offset, Size};
use crate::components::avatar::{AvatarContent, AvatarRow};
use crate::components::button::{IconButton, Button};
use crate::components::text_input::TextInput;
use crate::elements::text::Text;
use crate::{PelicanUI, Callback};
use crate::AppPage;
use crate::ElementID;
use std::fmt::Debug;

use super::{DesktopInterface, MobileInterface};

/// Root interface component with both mobile and desktop layouts options.
#[derive(Debug, Component)]
pub struct Interface (Stack, Rectangle, Option<MobileInterface>, Option<DesktopInterface>);
impl OnEvent for Interface {}

impl Interface {
    /// Creates a new `Interface` component, which is used to initialize the main interface for the application.
    /// The `Interface` is tailored based on the device type (mobile or desktop) and contains information like 
    /// the start page, navigation, and user profile. Depending on the platform (mobile or desktop), it will create 
    /// either a `MobileInterface` or a `DesktopInterface` to manage the layout and navigation of the application.
    ///
    /// # Parameters:
    /// - **`ctx`**: The [`Context`] for accessing the app's theme.
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
        start_index: Option<usize>, 
        navigation: Option<Vec<(&'static str, &'static str, Box<Callback>)>>,
        profile: Option<(&'static str, AvatarContent, Box<Callback>)>,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.background.primary;
        let (mobile, desktop) = match crate::config::IS_MOBILE {
            true => (Some(MobileInterface::new(ctx, start_page, start_index, navigation, profile)), None),
            false => (None, Some(DesktopInterface::new(ctx, start_page, start_index, navigation, profile)))
        };
        Interface(Stack::default(), Rectangle::new(color), mobile, desktop)
    }
}

/// A top-level layout component representing a full screen or page.
///
/// Contains a [`Header`,] main [`Content`], optional [`Bumper`] (e.g., a bottom call-to-action).
#[derive(Debug, Component)]
pub struct Page(Column, Header, Content, Option<Bumper>, #[skip] bool);
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
pub struct Content (Stack, ContentChildren, #[skip] bool);

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
    pub fn new(offset: Offset, does_scroll: bool, content: Vec<Box<dyn Drawable>>) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0.min(375.0), 375.0));
        let height = Size::custom(move |_: Vec<(f32, f32)>|(0.0, f32::MAX));
        Content(
            Stack(Offset::Center, offset, width, height, Padding(24.0, 0.0, 24.0, 0.0)),
            ContentChildren::new(content), does_scroll // scrollable boolean
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
    /// Returns a mutable reference to the internal flag allowing scroll behavior.
    pub fn can_scroll(&mut self) -> &mut bool {&mut self.2}
}

impl OnEvent for Content {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent{state: MouseState::Scroll(_, y), ..}) = event.downcast_ref::<MouseEvent>() {
            *self.1.0.scroll() += y;
            *self.1.0.scroll() = self.1.0.scroll().clamp(0.0, 100.); // 100 = content height
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
/// `Header` is a component that displays a header section with customizable icons and content.
/// It allows different layouts, such as a home screen, a stack of buttons, or a chat interface.
///
/// Example:
/// ```rust
/// let home_header = Header::home(&mut ctx, "Welcome Home");
/// let stack_header = Header::stack(
///     &mut ctx, 
///     Some(IconButton::new("Left Button", Box::new(|ctx| { println!("Left button clicked"); }))),
///     "My Stack Header", 
///     Some(IconButton::new("Right Button", Box::new(|ctx| { println!("Right button clicked"); })))
/// );
/// let chat_header = Header::chat(
///     &mut ctx, 
///     Some(IconButton::new("Left Button", Box::new(|ctx| { println!("Left button clicked"); }))),
///     Some(IconButton::new("Right Button", Box::new(|ctx| { println!("Right button clicked"); }))),
///     vec![AvatarContent::new("avatar_image")]
/// );
/// ```
#[derive(Debug, Component)]
pub struct Header(Row, HeaderIcon, HeaderContent, HeaderIcon);
impl OnEvent for Header {}

impl Header {
    /// Creates a simple header with a home button and a title.
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        Header(
            Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)),
            HeaderIcon::new(None), 
            HeaderContent::home(ctx, title),
            HeaderIcon::new(None)
        )
    }

    /// Creates a header with customizable left and right icons, and a title.
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

    /// Creates a header for a chat interface with left and right icons, and avatars.
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
struct HeaderContent(Column, Text, Option<AvatarRow>);
impl OnEvent for HeaderContent {}

impl HeaderContent {
    pub fn home(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h3;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
            None,
        )
    }

    pub fn stack(ctx: &mut Context, title: &'static str) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h4;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()),  
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
            None,
        )
    }

    pub fn chat(ctx: &mut Context, avatars: Vec<AvatarContent>) -> Self {
        let text_size = ctx.get::<PelicanUI>().theme.fonts.size.h5;
        let title = if avatars.len() > 1 {"Ella Couch"} else {"Group Message"};
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        HeaderContent(
            Column::new(10.0, Offset::Center, width, Padding::default()), 
            Text::new(ctx, title, TextStyle::Heading, text_size, Align::Left),
            Some(AvatarRow::new(ctx, avatars)),
        )
    }
}

#[derive(Debug, Component)]
struct HeaderIcon(Stack, Option<IconButton>);
impl OnEvent for HeaderIcon {}

impl HeaderIcon {
    pub fn new(icon: Option<IconButton>) -> Self {
        HeaderIcon(
            Stack(Offset::Center, Offset::Center, Size::Static(48.0), Size::Static(48.0), Padding::default()),
            icon
        )
    }
}
/// `Bumper` is a component that can be used to display a collection of buttons, images, or other drawable content
/// at the bottom of the screen. It can be customized with a background color and various layout options.
///
/// Example:
/// ```rust
/// let button_a = Button::new("Button A", Box::new(|ctx| { println!("Button A clicked"); }));
/// let button_b = Button::new("Button B", Box::new(|ctx| { println!("Button B clicked"); }));
/// let bumper = Bumper::double_button(&mut ctx, button_a, button_b);
/// ```
#[derive(Debug, Component)]
pub struct Bumper (Stack, Rectangle, BumperContent);
impl OnEvent for Bumper {}

impl Bumper {
    /// Creates a `Bumper` with customizable content (a list of drawable elements).
    pub fn new(ctx: &mut Context, content: Vec<Box<dyn Drawable>>) -> Self {
        let background = ctx.get::<PelicanUI>().theme.colors.background.primary;
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, 375.0));
        let height = Size::custom(move |heights: Vec<(f32, f32)>|(heights[1].0, heights[1].1));
        Bumper(
            Stack(Offset::Center, Offset::Start, width, height, Padding::default()),
            Rectangle::new(background), BumperContent::new(content)
        )
    }

    /// Creates a `Bumper` with two buttons.
    pub fn double_button(ctx: &mut Context, a: Button, b: Button) -> Self {
        Self::new(ctx, vec![Box::new(a), Box::new(b)])
    }

    /// Creates a `Bumper` with a single button.
    pub fn single_button(ctx: &mut Context, a: Button) -> Self {
        Self::new(ctx, vec![Box::new(a)])
    }

    /// Creates a `Bumper` with a single text input.
    pub fn input(ctx: &mut Context, input: TextInput) -> Self {
        Self::new(ctx, vec![Box::new(input)])
    }

    /// Returns a mutable reference to the list of drawable items in the bumper.
    pub fn items(&mut self) -> &mut Vec<Box<dyn Drawable>> {
        &mut self.2.1
    }
}

#[derive(Debug, Component)]
struct BumperContent (Row, Vec<Box<dyn Drawable>>);
impl OnEvent for BumperContent {}

impl BumperContent {
    fn new(content: Vec<Box<dyn Drawable>>) -> Self {
        BumperContent(Row(16.0, Offset::Center, Size::Fit, Padding(24.0, 16.0, 24.0, 16.0)), content)
    }
}

/// `NavigationButton` represents a button or an icon button used for navigation within a user interface.
#[derive(Debug, Component)]
pub struct NavigationButton(Stack, Option<Button>, Option<IconButton>, #[skip] ElementID);

impl OnEvent for NavigationButton {}

impl NavigationButton {
    /// Creates a new `NavigationButton` with the provided `ElementID`, an optional `Button`, and an optional `IconButton`.
    ///
    /// - `id`: The `ElementID` of the navigation button.
    /// - `button`: An optional `Button` for the navigation button.
    /// - `icon_button`: An optional `IconButton` for the navigation button.
    ///
    /// Example usage:
    /// ```rust
    /// let button = Button::new("Navigate", move |ctx: &mut Context| { println!("Navigating!"); });
    /// let icon_button = IconButton::new("icon_name", move |ctx: &mut Context| { println!("Icon clicked!"); });
    /// let nav_button = NavigationButton::new(ElementID::new(), Some(button), Some(icon_button));
    /// ```
    pub fn new(id: ElementID, button: Option<Button>, icon_button: Option<IconButton>) -> Self {
        NavigationButton(Stack::default(), button, icon_button, id)
    }

    /// Returns the `ElementID` associated with this `NavigationButton`.
    pub fn id(&self) -> ElementID {
        self.3
    }

    /// Returns a mutable reference to the optional `Button` within this `NavigationButton`.
    pub fn button(&mut self) -> &mut Option<Button> {
        &mut self.1
    }

    /// Returns a mutable reference to the optional `IconButton` within this `NavigationButton`.
    pub fn icon_button(&mut self) -> &mut Option<IconButton> {
        &mut self.2
    }
}
