use rust_on_rails::prelude::*;
use crate::events::{KeyboardActiveEvent, NavigateEvent, NavigatorSelect};
use crate::layout::{Column, Row, Padding, Offset, Size, Opt};
use crate::components::avatar::AvatarContent;
use crate::components::button::{IconButton, ButtonState};
use crate::Callback;
use crate::AppPage;
use crate::ElementID;
use std::fmt::Debug;
use super::{NavigationButton, MobileKeyboard};

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
pub struct MobileInterface (Column, Box<dyn AppPage>, Option<Opt<MobileNavigator>>, Option<MobileKeyboard>);

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
        start_index: Option<usize>,
        navigation: Option<Vec<(&'static str, &'static str, Callback)>>,
        profile: Option<(&'static str, AvatarContent, Callback)>,
    ) -> Self {
        let navigator = navigation.zip(profile).zip(start_index).map(|((nav, p), i)| Opt::new(MobileNavigator::new(ctx, i, nav, p), true));
        println!("Navigator {:?}", navigator);
        #[cfg(target_os = "ios")] // move to rust_on_rails layer
        let insets = safe_area_insets();
        
        #[cfg(not(target_os = "ios"))]
        let insets = (0., 0., 0., 0.);
        
        MobileInterface(
            Column::new(0.0, Offset::Center, Size::Fit, Padding(0.0, insets.0, 0.0, insets.1)), 
            Box::new(start_page), 
            navigator, 
            None,
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
pub struct MobileNavigator(Row, Vec<NavigationButton>);

impl MobileNavigator {
    /// Creates a new `MobileNavigator` with the specified starting index, navigation options, and profile button.
    ///
    /// - `start_index`: The index of the currently selected navigation button.
    /// - `navigation`: A vector of tuples, where each tuple contains a navigation item (ID, label, and callback).
    /// - `profile`: A tuple containing the profile label, avatar content, and a callback to open the profile view.
    ///
    /// Example usage:
    /// ```rust
    /// let navigation = vec![
    ///     ("home", "Home", Box::new(|ctx| { println!("Navigating to Home!"); })),
    ///     ("settings", "Settings", Box::new(|ctx| { println!("Navigating to Settings!"); }))
    /// ];
    /// let profile = ("Profile", AvatarContent::new(), Box::new(|ctx| { println!("Opening profile..."); }));
    /// let mobile_nav = MobileNavigator::new(&mut ctx, 0, navigation, profile);
    /// ```
    pub fn new(
        ctx: &mut Context,
        start_index: usize,
        navigation: Vec<(&'static str, &'static str, Callback)>,
        mut profile: (&'static str, AvatarContent, Callback)
    ) -> Self {
        if navigation.is_empty() {
            panic!("MobileNavigator: Parameter 1 was empty. Navigator has no data.");
        }
        
        let profile_id = ElementID::new();

        let mut tabs: Vec<NavigationButton> = navigation.into_iter().enumerate().map(|(y, (i, _, mut c))| {
            let id = ElementID::new();
            let ib = IconButton::tab_nav(ctx, i, y == start_index, move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                (c)(ctx);
            });
            NavigationButton::new(id, None, Some(ib))
        }).collect();

        let ib = IconButton::tab_nav(ctx, "profile", false, move |ctx: &mut Context| {
            ctx.trigger_event(NavigatorSelect(profile_id));
            (profile.2)(ctx);
        });

        tabs.push(NavigationButton::new(profile_id, None, Some(ib)));

        MobileNavigator(
            Row::new(48.0, Offset::Center, Size::Fit, Padding(0.0, 8.0, 0.0, 8.0)),
            tabs
        )
    }
}


impl OnEvent for MobileNavigator {
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