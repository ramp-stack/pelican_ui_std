use pelican_ui::events::{OnEvent, Event};
use pelican_ui::drawable::{Drawable, Component, Image};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::elements::shapes::{Rectangle};
use crate::elements::images::Brand;
use crate::events::{NavigatorSelect, NavigateEvent, NavigatorEvent};
use crate::layout::{Column, Stack, Bin, Row, Padding, Offset, Size};
use crate::components::button::{Button, ButtonState};
use crate::utils::{ElementID, AppPage};
use crate::pages::Error;

use std::fmt::Debug;
use super::{NavigationButton, NavigateInfo};

#[derive(Debug, Component)]
pub struct DesktopInterface(Row, Option<DesktopNavigator>, Bin<Stack, Rectangle>, Option<Box<dyn AppPage>>);

impl DesktopInterface {
    pub fn new(
        ctx: &mut Context, 
        start_page: Box<dyn AppPage>,
        navigation: Option<(usize, Vec<NavigateInfo>)>,
    ) -> Self {
        let color = ctx.theme.colors.outline.secondary;
        let navigator = navigation.map(|n| DesktopNavigator::new(ctx, n));

        DesktopInterface(
            Row::new(0.0, Offset::Start, Size::Fit, Padding::default()),
            navigator,
            Bin(
                Stack(Offset::default(), Offset::default(), Size::Static(1.0), Size::Fit, Padding::default()), 
                Rectangle::new(color)
            ),
            Some(start_page)
        )
    }
}

impl OnEvent for DesktopInterface {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(NavigateEvent(index)) = event.downcast_mut::<NavigateEvent>() {
            self.3 = match self.3.take().unwrap().navigate(ctx, *index) {
                Ok(p) => Some(p),
                Err(e) => Some(Box::new(Error::new(ctx, "404 Page Not Found", e)))
            };
        } else if let Some(NavigatorEvent(on_click)) = event.downcast_mut::<NavigatorEvent>() {
            self.3 = Some(on_click(ctx));
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct DesktopNavigator(Column, Image, ButtonColumn, Bin<Stack, Rectangle>, ButtonColumn);

impl DesktopNavigator {
    pub fn new(
        ctx: &mut Context,
        navigation: (usize, Vec<NavigateInfo>),
    ) -> Self {

        let mut top_col = Vec::new();
        let mut bot_col = Vec::new();

        for (i, (icon, name, avatar, on_click)) in navigation.1.into_iter().enumerate() {
            let id = ElementID::new();
            let closure = move |ctx: &mut Context| {
                ctx.trigger_event(NavigatorSelect(id));
                ctx.trigger_event(NavigatorEvent(on_click));
            };

            if let Some(avatar) = avatar {
                let profile = Button::photo(ctx, name, avatar, navigation.0 == i, closure);
                bot_col.push(NavigationButton::new(id, Some(profile), None))
            } else {
                let button = Button::navigation(ctx, icon, name, navigation.0 == i, closure);
                top_col.push(NavigationButton::new(id, Some(button), None))
            }
        }

        let theme = &ctx.theme;
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
