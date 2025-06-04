use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::events::{KeyboardActiveEvent, NavigatorSelect};
use crate::layout::{Column, Row, Padding, Offset, Size, Opt, Stack, Bin};
use crate::components::button::{IconButton, ButtonState};
use crate::elements::shapes::Rectangle;
use crate::AppPage;
use crate::utils::ElementID;
use std::fmt::Debug;
use super::{NavigationButton, NavigateInfo, MobileKeyboard};

#[derive(Debug, Component)]
pub struct MobileInterface (Column, Bin<Stack, Rectangle>, Box<dyn AppPage>, Option<MobileKeyboard>, Option<Opt<MobileNavigator>>, Bin<Stack, Rectangle>);

impl MobileInterface {
    pub fn new(
        ctx: &mut Context, 
        start_page: Box<dyn AppPage>,
        navigation: Option<(usize, Vec<NavigateInfo>)>
    ) -> Self {
        let background = ctx.theme.colors.background.primary;
        let navigator = navigation.map(|n| Opt::new(MobileNavigator::new(ctx, n), true));
        let insets = (0.0, 0.0, 0.0, 0.0); // ctx.safe_area_insets();
        
        MobileInterface(
            Column::new(0.0, Offset::Center, Size::Fit, Padding::default()), 
            Bin(Stack(Offset::Center, Offset::Center, Size::fill(), Size::Static(insets.0), Padding::default()), Rectangle::new(background)),
            start_page, None, navigator,
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