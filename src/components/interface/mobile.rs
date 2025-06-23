use pelican_ui::events::{OnEvent, Event};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::events::{KeyboardActiveEvent, NavigatorSelect, NavigateEvent, NavigatorEvent};
use crate::layout::{Column, Row, Padding, Offset, Size, Opt, Stack, Bin};
use crate::components::button::{IconButton, ButtonState};
use crate::elements::shapes::Rectangle;
use crate::utils::{ElementID, AppPage};
use crate::pages::Error;

use std::fmt::Debug;
use super::{NavigationButton, NavigateInfo, MobileKeyboard, PageBuilder};

#[derive(Component)]
pub struct MobileInterface(Column, Bin<Stack, Rectangle>, Option<Box<dyn AppPage>>, Option<MobileKeyboard>, Option<Opt<MobileNavigator>>, Bin<Stack, Rectangle>,  #[skip] PageBuilder);

impl MobileInterface {
    pub fn new(
        ctx: &mut Context, 
        start_page: Box<dyn AppPage>,
        mut navigation: Option<(usize, Vec<NavigateInfo>)>
    ) -> Self {
        let background = ctx.theme.colors.background.primary;
        let pages = navigation.as_mut().map(|nav| nav.1.iter_mut().map(|n| n.3.take().unwrap()).collect::<Vec<_>>());
        let navigator = navigation.map(|n| Opt::new(MobileNavigator::new(ctx, n), true));
        let insets = (0.0, 0.0, 0.0, 0.0); // ctx.safe_area_insets();
        let inset = |h: f32| Bin(Stack(Offset::Center, Offset::Center, Size::fill(), Size::Static(h), Padding::default()), Rectangle::new(background));
        MobileInterface(
            Column::new(0.0, Offset::Center, Size::Fit, Padding::default()), 
            inset(insets.0),
            Some(start_page), 
            None, 
            navigator,
            inset(insets.1),
            pages
        )
    }

    pub fn page(&mut self) -> &mut Option<Box<dyn AppPage>> { &mut self.2 }
    pub fn navigator(&mut self) -> &mut Option<Opt<MobileNavigator>> { &mut self.4 }
}

impl OnEvent for MobileInterface {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigateEvent(index)) = event.downcast_mut::<NavigateEvent>() {
            self.2 = match self.2.take().unwrap().navigate(ctx, *index) {
                Ok(p) => Some(p),
                Err(e) => Some(Box::new(Error::new(ctx, "404 Page Not Found", e)))
            };

            if let Some(navigator) = &mut self.4 {navigator.display(self.2.as_ref().map(|s| s.has_nav()).unwrap_or(false));}
        } else if let Some(NavigatorEvent(index)) = event.downcast_mut::<NavigatorEvent>() {
            if let Some(nav) = self.6.as_mut() { self.2 = Some(nav[*index](ctx)); }
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

impl std::fmt::Debug for MobileInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mobile")
    }
}

#[derive(Debug, Component)]
pub struct MobileNavigator(Stack, Rectangle, MobileNavigatorContent);

impl MobileNavigator {
    pub fn new(
        ctx: &mut Context,
        navigation: (usize, Vec<NavigateInfo>)
    ) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
        let height = Size::custom(move |heights: Vec<(f32, f32)>|(heights[1].0, heights[1].1));
        let background = ctx.theme.colors.background.primary;

        MobileNavigator(
            Stack(Offset::Center, Offset::Start, width, height, Padding::default()), 
            Rectangle::new(background),
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
        for (i, (icon, _, _, _)) in navigation.1.into_iter().enumerate() {
            let id = ElementID::new();
            let closure = move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                ctx.trigger_event(NavigatorEvent(i));
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