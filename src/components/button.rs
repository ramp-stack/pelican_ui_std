use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::icon::Icon;
use crate::elements::shapes::{RoundedRectangle, Outline};
use crate::elements::text::{Text, TextStyle};
use crate::theme::colors::ButtonColorScheme;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Stack, Offset, Size, Row};
use crate::PelicanUI;

#[derive(Debug, Clone, Component)]
pub struct Button(Stack, ButtonBackground, ButtonContent, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] fn(&mut Context, (u32, u32)) -> ());
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
        on_click: fn(&mut Context, (u32, u32)) -> (),
    ) -> Self {

        let colors = ButtonColor::get(ctx, style, state);
        let content = ButtonContent::new(ctx, avatar, icon_l, label, icon_r, size, colors.label);
        let (height, padding) = size.background();

        let width = match width {
            ButtonWidth::Hug => Size::Fit,
            ButtonWidth::Expand => Size::Fill(content.size(ctx).min_width()+(padding*2), MaxSize::MAX)
        };

        let background = ButtonBackground::new(colors.background, colors.outline, width, height);

        Button(Stack::center(), background, content, style, state, on_click)
    }

    pub fn set_state(&mut self, ctx: &mut Context, state: ButtonState) {
        if self.4 != state {
            self.4 = state;
            let colors = ButtonColor::get(ctx, self.3, state);
            self.1.set_color(colors.background, colors.outline);
            self.2.set_color(colors.label);
        }
    }
}

impl Events for Button {
    fn on_click(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        if let Some(position) = position {(self.5)(ctx, position);}
        false
    }
    fn on_move(&mut self, ctx: &mut Context, position: Option<(u32, u32)>) -> bool {
        println!("move: {:?}", position);
        match (position.is_some(), self.4) {
            (true, ButtonState::Default) => self.set_state(ctx, ButtonState::Hover),
            (false, ButtonState::Hover) => self.set_state(ctx, ButtonState::Default),
            _ => {}
        };
        false
    }
}

#[derive(Clone, Debug, Component)]
pub struct ButtonContent(Row, Option<Avatar>, Option<Icon>, Option<BasicText>, Option<Icon>);
impl Events for ButtonContent {}

impl ButtonContent {
    fn new(ctx: &mut Context, avatar: Option<AvatarContent>, icon_l: Option<&'static str>, label: Option<&'static str>, icon_r: Option<&'static str>, size: ButtonSize, color: Color) -> Self {
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
        if let Some(icon) = &mut self.2 { icon.set_color(color); }
        if let Some(BasicText(_, c, _, _, _, _)) = &mut self.3 { *c = color; }
        if let Some(icon) = &mut self.4 { icon.set_color(color); }
    }
}

#[derive(Clone, Debug, Component)]
pub struct ButtonBackground(Stack, Shape, Shape);

impl ButtonBackground {
    pub fn new(bg: Color, oc: Color, width: Size, height: u32) -> Self {
        ButtonBackground(
            Stack(Offset::Center, Offset::Center, width, Size::Fit),
            RoundedRectangle::new(100, height, height/2, bg),
            Outline::rounded_rectangle(100, height, height/2, 1, oc)
        )
    }

    fn set_color(&mut self, bg: Color, oc: Color) {
        let Shape(_, c) = &mut self.1;
        *c = bg;
        let Shape(_, c) = &mut self.2;
        *c = oc;
    }
}

impl Events for ButtonBackground {
    fn on_resize(&mut self, _ctx: &mut Context, size: (u32, u32)) {
        if let Shape(ShapeType::RoundedRectangle(_, (w, _), _), _) = &mut self.1 {
            *w = size.0;
        }
        if let Shape(ShapeType::RoundedRectangle(_, (w, _), _), _) = &mut self.2 {
            *w = size.0;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    Large,
    Medium,
}

impl ButtonSize {
    fn content(&self, ctx: &mut Context) -> (u32, u32, u32) { // text size, icon size, spacing
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        match self {
            ButtonSize::Medium => (font_size.md, 16, 4),
            ButtonSize::Large => (font_size.lg, 24, 12)
        }
    }
    fn background(&self) -> (u32, u32) { // height, padding
        match self {
            ButtonSize::Medium => (32, 12),
            ButtonSize::Large => (48, 24)
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonState {
    Default,
    Disabled,
    Selected,
    Hover,
}

#[derive(Default, Clone)]
pub struct ButtonColor;
impl ButtonColor {
    pub fn get(ctx: &mut Context, style: ButtonStyle, state: ButtonState) -> ButtonColorScheme {
        let schemes = &ctx.get::<PelicanUI>().theme.colors.button;
        match (style, state) {
            (ButtonStyle::Primary, ButtonState::Default) => schemes.primary_default,
            (ButtonStyle::Primary, ButtonState::Disabled) => schemes.primary_disabled,
            (ButtonStyle::Primary, ButtonState::Hover) => schemes.primary_hover,
            (ButtonStyle::Primary, ButtonState::Selected) => schemes.primary_selected,

            (ButtonStyle::Secondary, ButtonState::Default) => schemes.secondary_default,
            (ButtonStyle::Secondary, ButtonState::Disabled) => schemes.secondary_disabled,
            (ButtonStyle::Secondary, ButtonState::Hover) => schemes.secondary_hover,
            (ButtonStyle::Secondary, ButtonState::Selected) => schemes.secondary_selected,

            (ButtonStyle::Ghost, ButtonState::Default) => schemes.ghost_default,
            (ButtonStyle::Ghost, ButtonState::Disabled) => schemes.ghost_disabled,
            (ButtonStyle::Ghost, ButtonState::Hover) => schemes.ghost_hover,
            (ButtonStyle::Ghost, ButtonState::Selected) => schemes.ghost_selected,
        }
    }
}


impl Button {
    pub fn primary(
        ctx: &mut Context,
        label: &'static str,
        on_click: fn(&mut Context, (u32, u32)) -> (),
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
            on_click
        )
    }

    pub fn secondary(
        ctx: &mut Context,
        icon_l: Option<&'static str>,
        label: &'static str,
        icon_r: Option<&'static str>,
        on_click: fn(&mut Context, (u32, u32)) -> (),
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
            on_click
        )
    }

    pub fn ghost(
        ctx: &mut Context,
        label: &'static str,
        on_click: fn(&mut Context, (u32, u32)) -> (),
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
            on_click
        )
    }

    pub fn key_pad(
        ctx: &mut Context,
        label: Option<&'static str>,
        icon: Option<&'static str>,
        on_click: fn(&mut Context, (u32, u32)) -> (),
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
            on_click
        )
    }

    pub fn navigation(
        ctx: &mut Context,
        icon: &'static str,
        label: &'static str,
        selected: bool,
        on_click: fn(&mut Context, (u32, u32)) -> (),
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
            on_click
        )
    }

    pub fn photo(
        ctx: &mut Context,
        label: &'static str,
        photo: AvatarContent,
        selected: bool,
        on_click: fn(&mut Context, (u32, u32)) -> (),
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
            on_click
        )
    }
}
