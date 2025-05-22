use rust_on_rails::prelude::*;
use crate::elements::shapes::{Rectangle};
use crate::elements::images::Brand;
use crate::events::{NavigateEvent, NavigatorSelect};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::components::avatar::AvatarContent;
use crate::components::button::{Button, ButtonState};
use crate::{PelicanUI, Callback};
use crate::AppPage;
use crate::ElementID;
use std::fmt::Debug;
use super::NavigationButton;

/// Main layout for desktop views with optional navigation.
#[derive(Debug, Component)]
pub struct DesktopInterface (Row, Option<DesktopNavigator>, Bin<Stack, Rectangle>, Box<dyn AppPage>);

impl DesktopInterface {
    /// Creates a new `DesktopInterface` with optional navigation and profile sections.
    ///
    /// - `start_page`: the main content page to show.
    /// - `start_index`: optional selected nav index.
    /// - `navigation`: optional list of nav button labels/icons and their callbacks.
    /// - `profile`: optional nav button to user profile.
    pub fn new(
        ctx: &mut Context, 
        start_page: impl AppPage, 
        start_index: Option<usize>,
        navigation: Option<Vec<(&'static str, &'static str, Callback)>>,
        profile: Option<(&'static str, AvatarContent, Callback)>,
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.outline.secondary;
        let navigator = navigation.zip(profile).zip(start_index).map(|((nav, p), i)| DesktopNavigator::new(ctx, i, nav, p));

        DesktopInterface(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            navigator,
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
            let (page, _) = page.get_page(ctx);
            self.3 = page;
        }
        true
    }
}

/// `DesktopNavigator` is a component that displays a set of navigation buttons along with a profile button.
/// It allows users to navigate between different sections of the app. Each navigation button can trigger a callback
/// when clicked, and the profile button opens a profile page or performs some action.
///
/// Example:
/// ```rust
/// let navigation_items = vec![
///     ("Home", "home_icon", Box::new(|ctx: &mut Context| { println!("Home clicked!"); })),
///     ("Settings", "settings_icon", Box::new(|ctx: &mut Context| { println!("Settings clicked!"); })),
///     ("Profile", "profile_icon", Box::new(|ctx: &mut Context| { println!("Profile clicked!"); })),
/// ];
///
/// let profile_data = (
///     "Profile", 
///     AvatarContent::new("profile_image"), // Assuming AvatarContent::new() sets up an avatar image
///     Box::new(|ctx: &mut Context| { println!("Profile button clicked!"); })
/// );
///
/// let desktop_navigator = DesktopNavigator::new(
///     &mut ctx,            // Context passed for UI initialization
///     0,                   // Start with "Home" tab selected (index 0)
///     navigation_items,    // List of navigation items
///     profile_data         // Profile button data
/// );
/// ```
#[derive(Debug, Component)]
pub struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, NavigationButton);

impl DesktopNavigator {
    /// Creates a new `DesktopNavigator` with navigation tabs and a profile button.
    ///
    /// - `start_index`: initially selected navigation index.
    /// - `navigation`: list of label/icon/callback tuples for nav items.
    /// - `profile`: label, avatar, and callback for the profile button.
    ///
    /// # Panics
    /// Panics if the `navigation` list is empty.
    pub fn new(
        ctx: &mut Context, 
        start_index: usize,
        navigation: Vec<(&'static str, &'static str, Callback)>,
        mut profile: (&'static str, AvatarContent, Callback)
    ) -> Self {
        if navigation.is_empty() {panic!("DesktopNavigator: Parameter 1 was empty. Navigator has no data.")}

        let theme = &ctx.get::<PelicanUI>().theme;
        let (wordmark, color) = (theme.brand.wordmark.clone(), theme.colors.shades.transparent);
        let profile_id = ElementID::new();

        let tabs: Vec<NavigationButton> = navigation.into_iter().enumerate().map(|(y, (i, n, mut c))| {
            let id = ElementID::new();
            let nb = Button::navigation(ctx, i, n, y == start_index, move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                (c)(ctx);
            });
            NavigationButton::new(id, Some(nb), None)
        }).collect();

        let pb = Button::photo(ctx, profile.0, profile.1, false, move |ctx: &mut Context| {
            ctx.trigger_event(NavigatorSelect(profile_id));
            (profile.2)(ctx);
        });

        DesktopNavigator(
            Column::new(32.0, Offset::Center, Size::Fill(100.0, 200.0), Padding(16.0, 32.0, 16.0, 32.0)),
            Brand::new(wordmark, (80.0, 44.0)),
            ButtonColumn::new(tabs),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100.0, 200.0), Size::Fill(100.0, f32::MAX), Padding::default()), 
                Rectangle::new(color)
            ),
            NavigationButton::new(profile_id, Some(pb), None)
        )
    }
}

impl OnEvent for DesktopNavigator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            println!("Navigator selected");
            let mut buttons: Vec<&mut NavigationButton> = self.2.buttons().iter_mut().collect();                buttons.push(&mut self.4);
            buttons.iter_mut().for_each(|button| {
                *button.button().as_mut().unwrap().status() = if button.id() == *id {ButtonState::Selected} else {ButtonState::Default};
                button.button().as_mut().unwrap().color(ctx);
            });
        }
        true
    }
}

#[derive(Debug, Component)]
struct ButtonColumn(Column, Vec<NavigationButton>);
impl OnEvent for ButtonColumn {}

impl ButtonColumn {
    fn new(buttons: Vec<NavigationButton>) -> Self {
        ButtonColumn(Column::center(8.0), buttons)
    }

    fn buttons(&mut self) -> &mut Vec<NavigationButton> {&mut self.1}
}