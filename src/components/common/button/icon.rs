use pelican_ui::events::{MouseState, MouseEvent, OnEvent, Event};
use pelican_ui::drawable::{Drawable, Component, Image, Color};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::utils::Callback;
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::layout::{Offset, Padding, Size, Row, Stack, Opt};
use super::{ButtonSize, ButtonState, ButtonStyle};

#[derive(Debug, Component)]
pub struct IconButton(Stack, IconButtonContent, Option<Opt<Image>>);
impl IconButton {
    pub fn new(
        ctx: &mut Context,
        icon: &'static str,
        size: ButtonSize,
        style: ButtonStyle,
        state: ButtonState,
        on_click: Box<dyn FnMut(&mut Context)>,
        flair: Option<(&'static str, Color, bool)>, // icon name, color, is shown
    ) -> Self {
        let content = IconButtonContent::new(ctx, icon, size, style, state, on_click);
        let s = if size == ButtonSize::Large {52.0} else {36.0};
        let icon = flair.map(|(i, c, h)| Opt::new(Icon::new(ctx, i, c, s / 1.8), h));
        IconButton(Stack(Offset::End, Offset::Start, Size::Fit, Size::Fit, Padding::default()), content, icon)
    }

    pub fn color(&mut self, ctx: &mut Context, state: ButtonState) {
        let colors = state.color(ctx, self.1.3);
        *self.1.1.background() = colors.background;
        *self.1.1.outline() = colors.outline;
        self.1.2.color = Some(colors.label);
    }

    pub fn show_flair(&mut self, hide: bool) {if let Some(i) = self.2.as_mut() {i.display(hide);}}
    pub fn status(&mut self) -> &mut ButtonState {&mut self.1.4}
}

impl OnEvent for IconButton {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let Some(state) = self.1.4.handle(ctx, *event) {
                self.color(ctx, state);
            }
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.1.4 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => {
                        ctx.hardware.haptic();
                        (self.1.5)(ctx)
                    },
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

#[derive(Component)]
pub struct IconButtonContent(Stack, OutlinedRectangle, Image, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>);
impl OnEvent for IconButtonContent {}
impl IconButtonContent {
    pub fn new(
        ctx: &mut Context,
        icon: &'static str,
        size: ButtonSize,
        style: ButtonStyle,
        state: ButtonState,
        on_click: Box<dyn FnMut(&mut Context)>,
    ) -> Self {
        let colors = state.color(ctx, style);
        let (size, icon_size, radius) = match (style, size) {
            (ButtonStyle::Secondary, ButtonSize::Large) => (52.0, 32.0, 12.0),
            (ButtonStyle::Secondary, ButtonSize::Medium) => (36.0, 20.0, 8.0),
            (ButtonStyle::Ghost, ButtonSize::Large) => (52.0, 48.0, 12.0),
            (ButtonStyle::Ghost, ButtonSize::Medium) => (36.0, 32.0, 8.0),
            _ => panic!("{style:?} is not a valid IconButton style")
        };

        let icon = Icon::new(ctx, icon, colors.label, icon_size);
        let background = OutlinedRectangle::new(colors.background, colors.outline, radius, 1.0);


        IconButtonContent(
            Stack(Offset::Center, Offset::Center, Size::Static(size), Size::Static(size), Padding::default()),
            background, icon, style, state, on_click
        )
    }
}

impl std::fmt::Debug for IconButtonContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IconButtonContent(...)")
    }
}

#[derive(Debug, Component)]
pub struct IconButtonRow(Row, Vec<IconButton>);
impl OnEvent for IconButtonRow {}

impl IconButtonRow {
    pub fn new(ctx: &mut Context, buttons: Vec<(&'static str, Callback)>) -> Self {
        let buttons = buttons.into_iter().map(|(i, on_click)| IconButton::secondary(ctx, i, on_click)).collect();
        IconButtonRow(Row::center(24.0), buttons)
    }
}

impl IconButton {
    pub fn secondary(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: Callback
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Large,
            ButtonStyle::Secondary,
            ButtonState::Default,
            on_click,
            None,
        )
    }

    pub fn ghost(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: Callback
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Large,
            ButtonStyle::Ghost,
            ButtonState::Default,
            on_click,
            None,
        )
    }

    pub fn input(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(on_click),
            None,
        )
    }

    pub fn keyboard(
        ctx: &mut Context, 
        icon: &'static str,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Box::new(on_click),
            None,
        )
    }
    
    pub fn navigation(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Box::new(on_click),
            None
        )
    }

    pub fn close(
        ctx: &mut Context, 
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            "close",
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Box::new(on_click),
            None,
        )
    }

    pub fn tab_nav(
        ctx: &mut Context, 
        icon: &'static str, 
        selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let color = ctx.theme.colors.brand.primary;
        let state = if selected {ButtonState::Selected} else {ButtonState::UnSelected};
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            state,
            Box::new(on_click),
            Some(("notification", color, false))
        )
    }
}

