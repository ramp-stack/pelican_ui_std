use rust_on_rails::prelude::*;
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::layout::{Stack, Offset, Size, Padding};

use super::{ButtonStyle, ButtonSize, ButtonState};

#[derive(Component)]
pub struct IconButton(
        Stack, OutlinedRectangle, Image,
        #[skip] ButtonStyle, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>
);

impl IconButton {
    pub fn new(
        ctx: &mut Context,
        icon: &'static str,
        size: ButtonSize,
        style: ButtonStyle,
        state: ButtonState,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let colors = state.color(ctx, style);
        let (size, icon_size, radius) = match (style, size) {
            (ButtonStyle::Secondary, ButtonSize::Large) => (52.0, 32.0, 12.0),
            (ButtonStyle::Secondary, ButtonSize::Medium) => (36.0, 20.0, 8.0),
            (ButtonStyle::Ghost, ButtonSize::Large) => (52.0, 48.0, 12.0),
            (ButtonStyle::Ghost, ButtonSize::Medium) => (36.0, 32.0, 8.0),
            _ => panic!("{:?} is not a valid style", style)
        };

        let icon = Icon::new(ctx, icon, colors.label, icon_size);
        let background = OutlinedRectangle::new(colors.background, colors.outline, radius, 1.0);


        IconButton(Stack(
            Offset::Center, Offset::Center, Size::Static(size), Size::Static(size), Padding::default()
        ), background, icon, style, state, Box::new(on_click))
    }
}

impl Events for IconButton {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let Some(state) = self.4.handle(ctx, *event) {
                let colors = state.color(ctx, self.3);
                *self.1.background() = colors.background;
                *self.1.outline() = colors.outline;
                self.2.color = Some(colors.label);
            }
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.4 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => (self.5)(ctx),
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

impl IconButton {
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
            on_click
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
            on_click
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
            on_click
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
            on_click
        )
    }

    pub fn tab_nav(
        ctx: &mut Context, 
        icon: &'static str, 
        selected: bool,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        let state = if selected {ButtonState::Default} else {ButtonState::UnSelected};
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            state,
            on_click
        )
    }
}

impl std::fmt::Debug for IconButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IconButton(...)")
    }
}
