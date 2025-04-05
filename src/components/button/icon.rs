use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
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
            (ButtonStyle::Secondary, ButtonSize::Large) => (48, 32, 12),
            (ButtonStyle::Secondary, ButtonSize::Medium) => (32, 20, 8),
            (ButtonStyle::Ghost, ButtonSize::Large) => (48, 48, 12),
            (ButtonStyle::Ghost, ButtonSize::Medium) => (32, 32, 8),
            _ => panic!("{:?} is not a valid style", style)
        };

        let icon = Icon::new(ctx, icon, colors.label, icon_size);
        let background = OutlinedRectangle::new(colors.background, colors.outline, radius, 1);


        IconButton(Stack(
            Offset::Center, Offset::Center, Size::Static(size), Size::Static(size), Padding::default()
        ), background, icon, style, state, Box::new(on_click))
    }
}

impl Events for IconButton {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        if let Some(state) = self.4.handle(ctx, event) {
            let colors = state.color(ctx, self.3);
            *self.1.background() = colors.background;
            *self.1.outline() = colors.outline;
            *self.2.color() = Some(colors.label);
        }
        if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
            match self.4 {
                ButtonState::Default | ButtonState::Hover | ButtonState::Selected => (self.5)(ctx),
                _ => {}
            }
        }
        false
    }
}

impl IconButton {
    pub fn input(
        ctx: &mut Context, icon: &'static str, on_click: impl FnMut(&mut Context) + 'static
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

    pub fn keyboard(ctx: &mut Context, icon: &'static str, on_click: fn(&mut Context) -> ()) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            on_click
        )
    }
}

impl std::fmt::Debug for IconButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IconButton(...)")
    }
}
