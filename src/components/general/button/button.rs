use rust_on_rails::prelude::*;
use crate::ElementID;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text, TextStyle};
use crate::events::RemoveContactEvent;
use crate::layout::{Offset, Padding, Row, Size, Stack, Wrap};

use super::{ButtonSize, ButtonState, ButtonStyle};

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
            if let Some(_) = self.4.handle(ctx, *event) {
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

// Implement Debug for Button
impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button(...)")
    }
}


#[derive(Debug, Component)]
struct ButtonContent(Row, Option<Avatar>, Option<Image>, Option<Text>, Option<Image>);
impl OnEvent for ButtonContent {}

impl ButtonContent {
    fn new(
        ctx: &mut Context,
        avatar: Option<AvatarContent>,
        icon_l: Option<&'static str>,
        label: Option<&'static str>,
        icon_r: Option<&'static str>,
        size: ButtonSize,
        color: Color,
        padding: f32,
    ) -> Self {
        let (text_size, icon_size, spacing) = size.content(ctx);
        ButtonContent(
            Row(spacing, Offset::Center, Size::Fit, Padding(padding, 0.0, padding, 0.0)),
            avatar.map(|content| Avatar::new(ctx, content, None, false, icon_size)),
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

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

impl Button {
    // Primary Button Preset
    pub fn primary (
        ctx: &mut Context,
        label: &'static str,
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

    // Secondary Button Preset
    pub fn secondary(
        ctx: &mut Context,
        icon_l: Option<&'static str>,
        label: &'static str,
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

    // Ghost Button Preset
    pub fn ghost(
        ctx: &mut Context,
        label: &'static str,
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

    // Disabled Button Preset
    pub fn disabled(
        ctx: &mut Context,
        label: &'static str,
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

    // Numeric Keypad Button Preset
    pub fn keypad(
        ctx: &mut Context,
        label: Option<&'static str>,
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

    // Desktop Navigator Button Preset
    pub fn navigation(
        ctx: &mut Context,
        icon: &'static str,
        label: &'static str,
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

    // Desktop Navigator Profile Button Preset
    pub fn photo(
        ctx: &mut Context,
        label: &'static str,
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
            if selected {ButtonState::Pressed} else {ButtonState::Default},
            Offset::Start,
            on_click,
        )
    }

    // Close Page Button Preset
    pub fn close(
        ctx: &mut Context,
        label: &'static str,
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
        // Wrap of custom buttons (secondary)
        QuickActions(Wrap(8.0, 8.0, Offset::Start, Offset::Center, Padding::default()), buttons)
    }
}

#[derive(Debug, Component)]
pub struct QuickDeselectButton(Stack, Button, #[skip] ElementID);
impl OnEvent for QuickDeselectButton {}

impl QuickDeselectButton {
    pub fn new(ctx: &mut Context, name: &'static str, id: ElementID) -> Self {
        // Wrap of secondary contact buttons
        let button = Button::secondary(ctx, None, name, Some("close"), move |ctx: &mut Context| ctx.trigger_event(RemoveContactEvent(id)));
        QuickDeselectButton(Stack::default(), button, id)
    }

    pub fn id(&self) -> ElementID {self.2}
}
