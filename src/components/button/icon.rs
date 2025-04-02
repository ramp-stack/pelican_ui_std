use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::RoundedRectangle;
use crate::layout::Stack;

use super::{ButtonStyle, ButtonSize, ButtonState};

pub type Function = Box<dyn FnMut(&mut Context)>;

#[derive(Component)]
pub struct IconButton(
        Stack, RoundedRectangle, RoundedRectangle, Icon,
        #[skip] ButtonStyle, #[skip] ButtonState, #[skip] pub Function
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
        let background = RoundedRectangle::new(0, Some(size), Some(size), radius, colors.background);
        let outline = RoundedRectangle::new(1, Some(size), Some(size), radius, colors.outline);

        IconButton(Stack::center(), background, outline, icon, style, state, Box::new(on_click))
    }
}

impl Events for IconButton {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        if let Some(state) = self.5.handle(ctx, event) {
            let colors = state.color(ctx, self.4);
            *self.1.shape().color() = colors.background;
            *self.2.shape().color() = colors.outline;
            *self.3.color() = Some(colors.label);
        }
        if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
            match self.5 {
                ButtonState::Default | ButtonState::Hover | ButtonState::Selected => (self.6)(ctx),
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
}

impl std::fmt::Debug for IconButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IconButton(...)")
    }
}
