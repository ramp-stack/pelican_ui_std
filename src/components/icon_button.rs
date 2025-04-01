use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::{RoundedRectangle, Outline};
use crate::components::button::{ButtonStyle, ButtonSize, ButtonState, ButtonColor};
use crate::layout::Stack;

#[derive(Debug, Clone, Component)]
pub struct IconButton(Stack, Shape, Shape, Option<Icon>, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] fn(&mut Context, (u32, u32)) -> ());
impl IconButton {
    pub fn new(
        ctx: &mut Context,
        icon: Option<&'static str>,
        size: ButtonSize,
        style: ButtonStyle,
        state: ButtonState,
        on_click: fn(&mut Context, (u32, u32)) -> (),
    ) -> Self {
        let colors = ButtonColor::get(ctx, style, state);
        let (size, icon_size, radius) = match (style, size) {
            (ButtonStyle::Secondary, ButtonSize::Large) => (48, 32, 12),
            (ButtonStyle::Secondary, ButtonSize::Medium) => (32, 20, 8),
            (ButtonStyle::Ghost, ButtonSize::Large) => (48, 48, 12),
            (ButtonStyle::Ghost, ButtonSize::Medium) => (32, 32, 8),
            _ => panic!("{:?} is not a valid style", style)
        };

        let icon = icon.map(|icon| Icon::new(ctx, icon, colors.label, icon_size));
        let background = RoundedRectangle::new(size, size, radius, colors.background);
        let outline = Outline::rounded_rectangle(size, size, radius, 1, colors.outline);

        IconButton(Stack::center(), background, outline, icon, style, state, on_click)
    }

    pub fn set_state(&mut self, ctx: &mut Context, state: ButtonState) {
        if self.5 != state {
            self.5 = state;
            let colors = ButtonColor::get(ctx, self.4, state);
            if let Some(icon) = &mut self.3 {icon.set_color(colors.label);}
            let Shape(_, c) = &mut self.1; *c = colors.background;
            let Shape(_, c) = &mut self.2; *c = colors.outline;
        }
    }
}

impl Events for IconButton {
    fn on_click(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        if let Some(position) = position {(self.6)(ctx, position);}
        false
    }
    fn on_move(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        // println!("move: {:?}", position);
        match (position.is_some(), self.5) {
            (true, ButtonState::Default) => self.set_state(ctx, ButtonState::Hover),
            (false, ButtonState::Hover) => self.set_state(ctx, ButtonState::Default),
            _ => {}
        };
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