use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::{RoundedRectangle, Outline};
use crate::layout::Stack;

use super::{ButtonStyle, ButtonSize, ButtonState};

#[derive(Debug, Clone, Component)]
pub struct IconButton(Stack, RoundedRectangle, RoundedRectangle, Option<Icon>, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] fn(&mut Context, (u32, u32)) -> ());
impl IconButton {
    pub fn new(
        ctx: &mut Context,
        icon: Option<&'static str>,
        size: ButtonSize,
        style: ButtonStyle,
        state: ButtonState,
        on_click: fn(&mut Context, (u32, u32)) -> (),
    ) -> Self {
        let colors = state.color(ctx, style);
        let (size, icon_size, radius) = match (style, size) {
            (ButtonStyle::Secondary, ButtonSize::Large) => (48, 32, 12),
            (ButtonStyle::Secondary, ButtonSize::Medium) => (32, 20, 8),
            (ButtonStyle::Ghost, ButtonSize::Large) => (48, 48, 12),
            (ButtonStyle::Ghost, ButtonSize::Medium) => (32, 32, 8),
            _ => panic!("{:?} is not a valid style", style)
        };

        let icon = icon.map(|icon| Icon::new(ctx, icon, colors.label, icon_size));
        let background = RoundedRectangle::new(0, Some(size), Some(size), radius, colors.background);
        let outline = RoundedRectangle::new(1, Some(size), Some(size), radius, colors.outline);

        IconButton(Stack::center(), background, outline, icon, style, state, on_click)
    }
}

impl Events for IconButton {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        if let Some(state) = self.5.handle(ctx, event) {
            let colors = state.color(ctx, self.4);
            if let Some(icon) = &mut self.3 {icon.set_color(colors.label);}
            let RoundedRectangle(_, Shape(_, c)) = &mut self.1; *c = colors.background;
            let RoundedRectangle(_, Shape(_, c)) = &mut self.2; *c = colors.outline;
        }
        if let MouseEvent{state: MouseState::Pressed, position: Some(position)} = event {
            match self.5 {
                ButtonState::Default | ButtonState::Hover => (self.6)(ctx, position),
                _ => {}
            }
        }
        false
    }
}

impl IconButton {
    pub fn input(ctx: &mut Context, icon: &'static str, on_click: fn(&mut Context, (u32, u32)) -> ()) -> Self {
        IconButton::new(
            ctx,
            Some(icon),
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            on_click
        )
    }
}
