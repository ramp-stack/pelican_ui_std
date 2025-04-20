use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::events::{RemoveContactEvent, SetActiveEvent, SetInactiveEvent};
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text, TextStyle};
use crate::components::avatar::{Avatar, AvatarContent};
use crate::layout::{Stack, Offset, Size, Wrap, Padding, Row, Column};
use crate::ElementID;

use super::{ButtonState, ButtonStyle, ButtonSize};

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Component)]
pub struct Button(Stack, OutlinedRectangle, ButtonContent, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>, #[skip] Option<ElementID>);
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
        id: Option<ElementID>,
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

        Button(layout, background, content, style, state, Box::new(on_click), id)
    }

    pub fn color(&mut self, ctx: &mut Context) {
        let colors = self.4.color(ctx, self.3);
        self.2.set_color(colors.label);
        *self.1.outline() = colors.outline;
        *self.1.background() = colors.background;
    }

    pub fn id(&self) -> Option<ElementID> {self.6}
    pub fn status(&mut self) -> &mut ButtonState {&mut self.4}
}

impl Events for Button {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let Some(state) = self.4.handle(ctx, *event) {
                self.color(ctx);
            }
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.4 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => (self.5)(ctx),
                    _ => {}
                }
            }
        } else if let Some(SetActiveEvent(id)) = event.downcast_ref::<SetActiveEvent>() {
            if self.6.is_some() && *id == self.6.unwrap() {
                self.4 = ButtonState::Default;
                self.color(ctx);
            }
        } else if let Some(SetInactiveEvent(id)) = event.downcast_ref::<SetInactiveEvent>() {
            if self.6.is_some() && *id == self.6.unwrap() {
                self.4 = ButtonState::Disabled;
                self.color(ctx);
            }
        }
        false
    }
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button(...)")
    }
}


#[derive(Debug, Component)]
struct ButtonContent(Row, Option<Avatar>, Option<Image>, Option<BasicText>, Option<Image>);
impl Events for ButtonContent {}

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
            label.map(|label| Text::new(ctx, label, TextStyle::Label(color), text_size, TextAlign::Left)),
            icon_r.map(|icon| Icon::new(ctx, icon, color, icon_size)),
        )
    }

    fn set_color(&mut self, color: Color) {
        if let Some(icon) = &mut self.2 { icon.color = Some(color); }
        if let Some(text) = &mut self.3 { text.color = color; }
        if let Some(icon) = &mut self.4 { icon.color = Some(color); }
    }
}

impl Button {
    pub fn primary(
        ctx: &mut Context,
        label: &'static str,
        id: Option<ElementID>,
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
            id,
        )
    }

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
            None,
        )
    }

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
            None,
        )
    }

    pub fn disabled(
        ctx: &mut Context,
        label: &'static str,
        id: Option<ElementID>,
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
            id,
        )
    }

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
            None,
        )
    }

    pub fn navigation(
        ctx: &mut Context,
        icon: &'static str,
        label: &'static str,
        selected: bool,
        id: ElementID,
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
            Some(id),
        )
    }

    pub fn photo(
        ctx: &mut Context,
        label: &'static str,
        photo: AvatarContent,
        selected: bool,
        id: ElementID,
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
            Some(id),
        )
    }

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
            None
        )
    }
}

#[derive(Debug, Component)]
pub struct ButtonColumn(Column, Vec<Button>);
impl Events for ButtonColumn {}

impl ButtonColumn {
    pub fn new(buttons: Vec<Button>) -> Self {
        ButtonColumn(Column::center(8.0), buttons)
    }

    pub fn buttons(&mut self) -> &mut Vec<Button> {&mut self.1}
}

// #[derive(Debug, Component)]
// pub struct QuickActions(Stack, QuickActionsContent);
// impl Events for QuickActions {}

// impl QuickActions {
//     pub fn new(buttons: Vec<Button>) -> Self {
//         let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0, f32::MAX));
//         QuickActions(
//             Stack(Offset::Start, Offset::Start, width, Size::Fit, Padding::default()),
//             QuickActionsContent::new(buttons)
//         )
//     }
// }

#[derive(Debug, Component)]
pub struct QuickActions(Wrap, Vec<Button>);
impl Events for QuickActions {}

impl QuickActions {
    pub fn new(buttons: Vec<Button>) -> Self {
        QuickActions(Wrap(8.0, 8.0, Offset::Start, Offset::Center, Padding::default()), buttons)
    }
}

#[derive(Debug, Component)]
pub struct QuickDeselectButton(Stack, Button, #[skip] ElementID);
impl Events for QuickDeselectButton {}

impl QuickDeselectButton {
    pub fn new(ctx: &mut Context, name: &'static str, id: ElementID) -> Self {
        let button = Button::secondary(ctx, None, name, Some("close"), move |ctx: &mut Context| ctx.trigger_event(RemoveContactEvent(id)));
        QuickDeselectButton(Stack::default(), button, id)
    }

    pub fn id(&self) -> ElementID {self.2}
}
