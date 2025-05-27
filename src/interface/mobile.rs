use rust_on_rails::prelude::*;
use crate::events::{KeyboardActiveEvent, NavigateEvent, NavigatorSelect};
use crate::layout::{Column, Row, Padding, Offset, Size, Opt, Stack, Bin};
use crate::components::avatar::AvatarContent;
use crate::components::button::{IconButton, ButtonState};
use crate::elements::shapes::Rectangle;
use crate::PelicanUI;
use crate::Callback;
use crate::AppPage;
use crate::ElementID;
use std::fmt::Debug;
use super::{NavigationButton, NavigateInfo, MobileKeyboard};

#[cfg(target_os = "ios")]
use crate::prelude::safe_area_insets;

/// `MobileInterface` is a component that represents the mobile user interface. It consists of a column layout, a main 
/// application page, an optional mobile navigator, and an optional mobile keyboard. This interface is ideal for 
/// structuring mobile app UIs that need navigation and interactivity, supporting elements like profiles and app pages.
///
/// Example:
/// ```rust
/// let navigation = vec![
///     ("home", "Home", Box::new(|ctx| { println!("Navigating to Home!"); })),
///     ("settings", "Settings", Box::new(|ctx| { println!("Navigating to Settings!"); }))
/// ];
/// let profile = ("Profile", AvatarContent::new(), Box::new(|ctx| { println!("Opening profile..."); }));
/// let mobile_interface = MobileInterface::new(&mut ctx, HomePage::new(), Some(0), Some(navigation), Some(profile));
/// ```
#[derive(Debug, Component)]
pub struct MobileInterface (Column, Bin<Stack, Rectangle>, Box<dyn AppPage>, Option<MobileKeyboard>, Option<Opt<MobileNavigator>>, Bin<Stack, Rectangle>);

impl MobileInterface {
    /// Creates a new `MobileInterface` with the specified starting page, navigation, and optional profile.
    ///
    /// - `start_page`: The starting page of the app, which should implement the `AppPage` trait.
    /// - `start_index`: The index of the starting navigation item (if provided).
    /// - `navigation`: An optional vector of navigation items (ID, label, and callback function) to be used in the mobile navigator.
    /// - `profile`: An optional tuple containing the profile label, avatar content, and callback function to open the profile view.
    ///
    /// Example usage:
    /// ```rust
    /// let navigation = vec![
    ///     ("home", "Home", Box::new(|ctx| { println!("Navigating to Home!"); })),
    ///     ("settings", "Settings", Box::new(|ctx| { println!("Navigating to Settings!"); }))
    /// ];
    /// let profile = ("Profile", AvatarContent::new(), Box::new(|ctx| { println!("Opening profile..."); }));
    /// let mobile_interface = MobileInterface::new(&mut ctx, HomePage::new(), Some(0), Some(navigation), Some(profile));
    /// ```
    pub fn new(
        ctx: &mut Context, 
        start_page: impl AppPage,
        navigation: Option<(usize, Vec<NavigateInfo>)>
    ) -> Self {
        let background = ctx.get::<PelicanUI>().theme.colors.background.primary;
        let navigator = navigation.map(|n| Opt::new(MobileNavigator::new(ctx, n), true));
        #[cfg(target_os = "ios")] // move to rust_on_rails layer
        let insets = safe_area_insets();
        #[cfg(not(target_os = "ios"))]
        let insets = (0., 0., 0., 0.);
        
        MobileInterface(
            Column::new(0.0, Offset::Center, Size::Fit, Padding::default()), 
            Bin(Stack(Offset::Center, Offset::Center, Size::fill(), Size::Static(insets.0), Padding::default()), Rectangle::new(background)),
            Box::new(start_page), 
            None,
            navigator,
            Bin(Stack(Offset::Center, Offset::Center, Size::fill(), Size::Static(insets.1), Padding::default()), Rectangle::new(background))
        )
    }

    pub fn set_page(&mut self, page: Box<dyn AppPage>, has_nav: bool) {
        if let Some(navigator) = &mut self.4 {navigator.display(has_nav);}
        self.2 = page;
    }
}

impl OnEvent for MobileInterface {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(_event) = event.downcast_ref::<TickEvent>() {
            // self.2.display(self.1.navigator_status());
        } else if let Some(KeyboardActiveEvent(enabled)) = event.downcast_ref::<KeyboardActiveEvent>() {
            match enabled {
                true if self.3.is_some() => {},
                true => self.3 = Some(MobileKeyboard::new(ctx)),
                false => self.3 = None
            }
        }
        true
    }
}

/// `MobileNavigator` is a component used for navigation in mobile interfaces. It displays a row of navigation buttons
/// (either icons or text) that the user can interact with. The navigation buttons can be configured with different
/// actions, such as switching pages or opening profile views.
///
/// Example:
/// ```rust
/// let navigation = vec![
///     ("home", "Home", Box::new(|ctx| { println!("Navigating to Home!"); })),
///     ("settings", "Settings", Box::new(|ctx| { println!("Navigating to Settings!"); }))
/// ];
/// let profile = ("Profile", AvatarContent::new(), Box::new(|ctx| { println!("Opening profile..."); }));
/// let mobile_nav = MobileNavigator::new(&mut ctx, 0, navigation, profile);
/// ```
#[derive(Debug, Component)]
pub struct MobileNavigator(Stack, Rectangle, MobileNavigatorContent);

impl MobileNavigator {
    pub fn new(
        ctx: &mut Context,
        navigation: (usize, Vec<NavigateInfo>)
    ) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        let height = Size::custom(move |heights: Vec<(f32, f32)>|(heights[1].0, heights[1].1));
        let background = ctx.get::<PelicanUI>().theme.colors.background.primary;

        MobileNavigator(
            Stack(Offset::Center, Offset::Start, width, height, Padding::default()), Rectangle::new(background),
            MobileNavigatorContent::new(ctx, navigation)
        )
    }
}

impl OnEvent for MobileNavigator {}

#[derive(Debug, Component)]
struct MobileNavigatorContent(Row, Vec<NavigationButton>);

impl MobileNavigatorContent {
    fn new(
        ctx: &mut Context,
        navigation: (usize, Vec<NavigateInfo>)
    ) -> Self {
        let mut tabs = Vec::new();
        for (i, (icon, _, _, mut on_navigate)) in navigation.1.into_iter().enumerate() {
            let id = ElementID::new();
            let closure = move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                (on_navigate)(ctx)
            };

            let button = IconButton::tab_nav(ctx, icon, navigation.0 == i, closure);
            tabs.push(NavigationButton::new(id, None, Some(button)));
        }

        MobileNavigatorContent(
            Row::new(48.0, Offset::Center, Size::Fit, Padding(0.0, 8.0, 0.0, 8.0)),
            tabs
        )
    }
}


impl OnEvent for MobileNavigatorContent {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigatorSelect(id)) = event.downcast_ref::<NavigatorSelect>() {
            self.1.iter_mut().for_each(|button| {
                let status = if button.id() == *id {ButtonState::Selected} else {ButtonState::UnSelected};
                *button.icon_button().as_mut().unwrap().status() = status;
                button.icon_button().as_mut().unwrap().color(ctx, status);
            });
        }
        true
    }
}