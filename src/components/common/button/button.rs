use pelican_ui::events::{MouseState, MouseEvent, OnEvent, Event};
use pelican_ui::drawable::{Drawable, Component, Image, Color, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::components::avatar::{Avatar, AvatarContent};
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text, TextStyle};
use crate::layout::{Offset, Padding, Row, Size, Stack, Wrap};

use super::{ButtonSize, ButtonState, ButtonStyle};

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Component)]
pub struct Button(
    Stack, 
    OutlinedRectangle, 
    ButtonContent, 
    #[skip] ButtonStyle, 
    #[skip] ButtonState,
    #[skip] pub Box<dyn FnMut(&mut Context)>, 
);

impl Button {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ctx: &mut Context,
        avatar: Option<AvatarContent>,
        icon_l: Option<&'static str>,
        label: Option<&str>,
        icon_r: Option<&'static str>,
        size: ButtonSize,
        width: ButtonWidth,
        style: ButtonStyle,
        state: ButtonState,
        offset: Offset,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let (height, padding) = size.background();
        let colors = state.color(ctx, style);
        let content = ButtonContent::new(ctx, avatar, icon_l, label, icon_r, size, colors.label, padding);

        let width = match width {
            ButtonWidth::Hug => Size::custom(move |widths: Vec<(f32, f32)>|
                (widths[1].0, widths[1].1)
            ),
            ButtonWidth::Expand => Size::custom(move |widths: Vec<(f32, f32)>|
                (widths[1].0, f32::MAX)
            ),
        };

        let background = OutlinedRectangle::new(colors.background, colors.outline, height/2.0, 1.0);
        let layout = Stack(offset, Offset::Center, width, Size::Static(height), Padding::default());

        Button(layout, background, content, style, state, Box::new(on_click))
    }

    pub fn color(&mut self, ctx: &mut Context) {
        let colors = self.4.color(ctx, self.3);
        self.2.set_color(colors.label);
        *self.1.outline() = colors.outline;
        *self.1.background() = colors.background;
    }

    pub fn status(&mut self) -> &mut ButtonState {&mut self.4}
}

impl OnEvent for Button {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if self.4.handle(ctx, *event).is_some() {
                self.color(ctx);
            }

            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.4 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => (self.5)(ctx),
                    _ => {}
                }
            }
        }
        false
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button")
    }
}

#[derive(Debug, Component)]
struct ButtonContent(Row, Option<Avatar>, Option<Image>, Option<Text>, Option<Image>);
impl OnEvent for ButtonContent {}

impl ButtonContent {
    #[allow(clippy::too_many_arguments)]
    fn new(
        ctx: &mut Context,
        avatar: Option<AvatarContent>,
        icon_l: Option<&'static str>,
        label: Option<&str>,
        icon_r: Option<&'static str>,
        size: ButtonSize,
        color: Color,
        padding: f32,
    ) -> Self {
        let (text_size, icon_size, spacing) = size.content(ctx);
        ButtonContent(
            Row::new(spacing, Offset::Center, Size::Fit, Padding(padding, 0.0, padding, 0.0)),
            avatar.map(|content| Avatar::new(ctx, content, None, false, icon_size, None)),
            icon_l.map(|icon| Icon::new(ctx, icon, color, icon_size)),
            label.map(|label| Text::new(ctx, label, TextStyle::Label(color), text_size, Align::Left)),
            icon_r.map(|icon| Icon::new(ctx, icon, color, icon_size)),
        )
    }

    fn set_color(&mut self, color: Color) {
        if let Some(icon) = &mut self.2 { icon.color = Some(color); }
        if let Some(text) = &mut self.3 { text.text().set_color(color); }
        if let Some(icon) = &mut self.4 { icon.color = Some(color); }
    }
}

impl Button {
    pub fn primary (
        ctx: &mut Context,
        label: &str,
        on_click: impl FnMut(&mut Context) + 'static,
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
            on_click,
        )
    }

    pub fn secondary(
        ctx: &mut Context,
        icon_l: Option<&'static str>,
        label: &str,
        icon_r: Option<&'static str>,
        on_click: impl FnMut(&mut Context) + 'static,
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
            on_click,
        )
    }

    pub fn ghost(
        ctx: &mut Context,
        label: &str,
        on_click: impl FnMut(&mut Context) + 'static,
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
            on_click,
        )
    }

    pub fn disabled(
        ctx: &mut Context,
        label: &str,
        on_click: impl FnMut(&mut Context) + 'static,
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
            ButtonState::Disabled,
            Offset::Center,
            on_click,
        )
    }

    pub fn keypad(
        ctx: &mut Context,
        label: Option<&str>,
        icon: Option<&'static str>,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        Button::new(
            ctx,
            None,
            icon,
            label,
            None,
            ButtonSize::Large,
            ButtonWidth::Expand,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Offset::Center,
            on_click,
        )
    }

    pub fn navigation(
        ctx: &mut Context,
        icon: &'static str,
        label: &str,
        selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
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
            on_click,
        )
    }

    pub fn photo(
        ctx: &mut Context,
        label: &str,
        photo: AvatarContent,
        selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
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
            on_click,
        )
    }

    pub fn close(
        ctx: &mut Context,
        label: &str,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        Button::new(
            ctx,
            None,
            None,
            Some(label),
            None,
            ButtonSize::Large,
            ButtonWidth::Expand,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Offset::Center,
            on_click,
        )
    }
}

#[derive(Debug, Component)]
pub struct QuickActions(Wrap, Vec<Button>);

impl OnEvent for QuickActions {}

impl QuickActions {
    pub fn new(buttons: Vec<Button>) -> Self {
        QuickActions(Wrap(8.0, 8.0, Offset::Start, Offset::Center, Padding::default()), buttons)
    }
}
