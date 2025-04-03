use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::shapes::RoundedRectangle;
use crate::elements::text::{Text, TextStyle};
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Stack, Offset, Size, Padding, Row};

use super::{ButtonState, ButtonStyle, ButtonSize};

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Debug, Component)]
pub struct Button(Stack, ButtonBackground, ButtonContent, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] fn(&mut Context) -> ());
impl Button {
    pub fn new(
        ctx: &mut Context,
        avatar: Option<AvatarContent>,
        icon_l: Option<&'static str>,
        label: Option<&'static str>,
        icon_r: Option<&'static str>,
        size: ButtonSize,
        width: ButtonWidth,
        style: ButtonStyle,
        state: ButtonState,
        offset: Offset,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        let (height, padding) = size.background();
        let colors = state.color(ctx, style);
        let content = ButtonContent::new(ctx, avatar, icon_l, label, icon_r, size, colors.label);

        let width = match width {
            ButtonWidth::Hug => Size::custom(move |widths: Vec<(u32, u32)>|
                (widths[1].0+(padding*2), widths[1].1+(padding*2))
            ),
            ButtonWidth::Expand => Size::custom(move |widths: Vec<(u32, u32)>|
                (widths[1].0+(padding*2), u32::MAX)
            ),
        };

        let background = ButtonBackground::new(colors.background, colors.outline, width, height);
        let layout = Stack(offset, Offset::Center, Size::Fit, Size::Fit, Padding::default());

        Button(layout, background, content, style, state, on_click)
    }
}

impl Events for Button {
    fn on_mouse(&mut self, ctx: &mut Context, event: MouseEvent) -> bool {
        if let Some(state) = self.4.handle(ctx, event) {
            let colors = state.color(ctx, self.3);
            self.1.set_color(colors.background, colors.outline);
            self.2.set_color(colors.label);
        }
        if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
            match self.4 {
                ButtonState::Default | ButtonState::Hover => (self.5)(ctx),
                _ => {}
            }
        }
        false
    }
}

#[derive(Debug, Component)]
struct ButtonContent(Row, Option<Avatar>, Option<Icon>, Option<BasicText>, Option<Icon>);
impl Events for ButtonContent {}

impl ButtonContent {
    fn new(
        ctx: &mut Context,
        avatar: Option<AvatarContent>,
        icon_l: Option<&'static str>,
        label: Option<&'static str>,
        icon_r: Option<&'static str>,
        size: ButtonSize,
        color: Color
    ) -> Self {
        let (text_size, icon_size, spacing) = size.content(ctx);
        ButtonContent(
            Row::center(spacing),
            avatar.map(|content| Avatar::new(ctx, content, None, false, icon_size)),
            icon_l.map(|icon| Icon::new(ctx, icon, color, icon_size)),
            label.map(|label| Text::new(ctx, label, TextStyle::Label(color), text_size)),
            icon_r.map(|icon| Icon::new(ctx, icon, color, icon_size)),
        )
    }

    fn set_color(&mut self, color: Color) {
        if let Some(icon) = &mut self.2 { *icon.color() = Some(color); }
        if let Some(text) = &mut self.3 { *text.color() = color; }
        if let Some(icon) = &mut self.4 { *icon.color() = Some(color); }
    }
}

#[derive(Debug, Component)]
struct ButtonBackground(Stack, RoundedRectangle, RoundedRectangle);
impl Events for ButtonBackground {}

impl ButtonBackground {
    pub fn new(bg: Color, oc: Color, width: Size, height: u32) -> Self {
        ButtonBackground(
            Stack(Offset::Center, Offset::Center, width, Size::Static(height), Padding::default()),
            RoundedRectangle::new(0, height/2, bg),
            RoundedRectangle::new(1, height/2, oc)
        )
    }

    fn set_color(&mut self, bg: Color, oc: Color) {
        *self.1.shape().color() = bg;
        *self.2.shape().color() = oc;
    }
}

impl Button {
    pub fn primary(
        ctx: &mut Context,
        label: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        Button::new(
            ctx,
            None,
            None,
            Some(label),
            None,
            ButtonSize::Large,
            ButtonWidth::Expand,
            ButtonStyle::Primary,
            ButtonState::Default,
            Offset::Center,
            on_click
        )
    }

    pub fn secondary(
        ctx: &mut Context,
        icon_l: Option<&'static str>,
        label: &'static str,
        icon_r: Option<&'static str>,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        Button::new(
            ctx,
            None,
            icon_l,
            Some(label),
            icon_r,
            ButtonSize::Medium,
            ButtonWidth::Hug,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Offset::Center,
            on_click
        )
    }

    pub fn ghost(
        ctx: &mut Context,
        label: &'static str,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        Button::new(
            ctx,
            None,
            None,
            Some(label),
            None,
            ButtonSize::Medium,
            ButtonWidth::Hug,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Offset::Center,
            on_click
        )
    }

    pub fn key_pad(
        ctx: &mut Context,
        label: Option<&'static str>,
        icon: Option<&'static str>,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        Button::new(
            ctx,
            None,
            icon,
            label,
            None,
            ButtonSize::Medium,
            ButtonWidth::Hug,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Offset::Center,
            on_click
        )
    }

    pub fn navigation(
        ctx: &mut Context,
        icon: &'static str,
        label: &'static str,
        selected: bool,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        Button::new(
            ctx,
            None,
            Some(icon),
            Some(label),
            None,
            ButtonSize::Large,
            ButtonWidth::Expand,
            ButtonStyle::Ghost,
            if selected {ButtonState::Selected} else {ButtonState::Default},
            Offset::Start,
            on_click
        )
    }

    pub fn photo(
        ctx: &mut Context,
        label: &'static str,
        photo: AvatarContent,
        selected: bool,
        on_click: fn(&mut Context) -> (),
    ) -> Self {
        Button::new(
            ctx,
            Some(photo),
            None,
            Some(label),
            None,
            ButtonSize::Large,
            ButtonWidth::Expand,
            ButtonStyle::Ghost,
            if selected {ButtonState::Selected} else {ButtonState::Default},
            Offset::Start,
            on_click
        )
    }
}
