use rust_on_rails::prelude::*;
use crate::elements::shapes::{Rectangle};
use crate::elements::images::Brand;
use crate::events::NavigatorSelect;
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::components::button::{Button, ButtonState};
use crate::plugin::PelicanUI;
use crate::plugin::AppPage;
use crate::utils::ElementID;
use std::fmt::Debug;
use super::{NavigationButton, NavigateInfo};

/// Main layout for desktop views with optional navigation.
#[derive(Debug, Component)]
pub struct DesktopInterface (Row, Option<DesktopNavigator>, Bin<Stack, Rectangle>, Box<dyn AppPage>);
impl OnEvent for DesktopInterface {}

impl DesktopInterface {
    /// Creates a new `DesktopInterface` with optional navigation and profile sections.
    pub fn new(
        ctx: &mut Context, 
        start_page: Box<dyn AppPage>,
        navigation: Option<(usize, Vec<NavigateInfo>)>, // the start index, each button's info
    ) -> Self {
        let color = ctx.get::<PelicanUI>().theme.colors.outline.secondary;
        let navigator = navigation.map(|n| DesktopNavigator::new(ctx, n));

        DesktopInterface(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            navigator,
            Bin(
                Stack(Offset::default(), Offset::default(), Size::Static(1.0), Size::Fit, Padding::default()), 
                Rectangle::new(color)
            ),
            start_page
        )
    }

    pub fn set_page(&mut self, page: Box<dyn AppPage>) {
        self.3 = page;
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
pub struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, ButtonColumn);

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
        navigation: (usize, Vec<NavigateInfo>),
    ) -> Self {
        // pub type NavigateInfo = (&'static str, &str, Option<AvatarContent>, Box<dyn AppPage>);
        // icon name, name, avatar, page link

        let mut top_col = Vec::new();
        let mut bot_col = Vec::new();

        for (i, (icon, name, avatar, mut on_navigate)) in navigation.1.into_iter().enumerate() {
            let id = ElementID::new();
            let closure = move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                (on_navigate)(ctx)
            };

            if let Some(avatar) = avatar {
                let profile = Button::photo(ctx, name, avatar, navigation.0 == i, closure);
                bot_col.push(NavigationButton::new(id, Some(profile), None))
            } else {
                let button = Button::navigation(ctx, icon, name, navigation.0 == i, closure);
                top_col.push(NavigationButton::new(id, Some(button), None))
            }
        }

        let theme = &ctx.get::<PelicanUI>().theme;
        let (wordmark, color) = (theme.brand.wordmark.clone(), theme.colors.shades.transparent);

        DesktopNavigator(
            Column::new(32.0, Offset::Center, Size::Fill(100.0, 200.0), Padding(16.0, 32.0, 16.0, 32.0)),
            Brand::new(wordmark, (80.0, 44.0)),
            ButtonColumn::new(top_col),
            Bin (
                Stack(Offset::Center, Offset::Center, Size::Fill(100.0, 200.0), Size::Fill(100.0, f32::MAX), Padding::default()), 
                Rectangle::new(color)
            ),
            ButtonColumn::new(bot_col)
        )
    }
}

impl OnEvent for DesktopNavigator {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            println!("Navigator selected");
            let mut buttons: Vec<&mut NavigationButton> = self.2.buttons().iter_mut().collect();
            buttons.extend(self.4.buttons().iter_mut());
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
