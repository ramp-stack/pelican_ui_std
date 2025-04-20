use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;

use crate::ElementID;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text, TextStyle};
use crate::events::{RemoveContactEvent, SetActiveEvent, SetInactiveEvent};
use crate::layout::{Column, Offset, Padding, Row, Size, Stack, Wrap};

use super::{ButtonSize, ButtonState, ButtonStyle};

#[derive(Component)]
pub struct Button(
    Stack, 
    OutlinedRectangle, 
    ButtonContent, 
    #[skip] ButtonStyle, 
    #[skip] ButtonState, 
    #[skip] Option<ElementID>,
    #[skip] pub Box<dyn FnMut(&mut Context)>, 
);

impl Button {
    pub fn new(
        // App Context
        ctx: &mut Context,
        // Optional User Avatar 
        avatar: Option<AvatarContent>,
        // Optional Icon to Left of Label
        icon_l: Option<&'static str>,
        // Optional Label
        label: Option<&'static str>,
        // Optional Icon to Right of Label
        icon_r: Option<&'static str>,
        // Size of Button (Medium, Large)
        size: ButtonSize,
        // Width of Button (Expand, Hug)
        width: ButtonWidth,
        // Style of Button
        style: ButtonStyle,
        // State of Button
        state: ButtonState,
        // Alignment of Inner Content
        offset: Offset,
        // Optional Identifier
        id: Option<ElementID>,
        // Code to Run On Click
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        // Get height and padding of background based off the ButtonSize enum.
        let (height, padding) = size.background();
        // Get colors for label, background, outline based off the ButtonStyle enum.
        let colors = state.color(ctx, style);
        // Create the ButtonContent
        let content = ButtonContent::new(ctx, avatar, icon_l, label, icon_r, size, colors.label, padding);

        // Calculate button width
        let width = match width {
            ButtonWidth::Hug => Size::custom(move |widths: Vec<(f32, f32)>|
                (widths[1].0, widths[1].1)
            ),
            ButtonWidth::Expand => Size::custom(move |widths: Vec<(f32, f32)>|
                (widths[1].0, f32::MAX)
            ),
        };

        // Build background shape
        let background = OutlinedRectangle::new(colors.background, colors.outline, height/2.0, 1.0);
        // Create stack layout
        let layout = Stack(offset, Offset::Center, width, Size::Static(height), Padding::default());

        Button(layout, background, content, style, state, id, Box::new(on_click))
    }

    // Recolor button
    pub fn color(&mut self, ctx: &mut Context) {
        // Get colors based off ButtonStyle and ButtonState
        let colors = self.4.color(ctx, self.3);
        // Set color for label and icons
        self.2.set_color(colors.label);
        // Set color for outline
        *self.1.outline() = colors.outline;
        // Set color for background
        *self.1.background() = colors.background;
    }

    // Get button's ElementID
    pub fn id(&self) -> Option<ElementID> {self.5}
    // Get button's ButtonState
    pub fn status(&mut self) -> &mut ButtonState {&mut self.4}
}

impl Events for Button {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            // Handle ButtonState on mouse event
            if let Some(state) = self.4.handle(ctx, *event) {
                // Recolor button for new state
                self.color(ctx);
            }
            // Run on_click when pressed
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.4 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => (self.6)(ctx),
                    _ => {}
                }
            }
        } else if let Some(SetActiveEvent(id)) = event.downcast_ref::<SetActiveEvent>() {
            // Check if received SetActiveEvent
            if self.5.is_some() && *id == self.5.unwrap() {
                // Set state to Default
                self.4 = ButtonState::Default;
                // Recolor button
                self.color(ctx);
            }
        } else if let Some(SetInactiveEvent(id)) = event.downcast_ref::<SetInactiveEvent>() {
            // Check if received SetInactiveEvent
            if self.5.is_some() && *id == self.5.unwrap() {
                // Set state to Disabled
                self.4 = ButtonState::Disabled;
                // Recolr button
                self.color(ctx);
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
struct ButtonContent(Row, Option<Avatar>, Option<Image>, Option<BasicText>, Option<Image>);
impl Events for ButtonContent {}

impl ButtonContent {
    fn new(
        // App Context
        ctx: &mut Context,
        // Optional User Avatar
        avatar: Option<AvatarContent>,
        // Optional Icon to Left of Label
        icon_l: Option<&'static str>,
        // Optional Label
        label: Option<&'static str>,
        // Optional Icon to Right of Label
        icon_r: Option<&'static str>,
        // Size of Button (Medium, Large)
        size: ButtonSize,
        // Color of Label and Icons
        color: Color,
        // Space Between Label and Icons
        padding: f32,
    ) -> Self {
        // Calculate sizes based off ButtonSize enum.
        let (text_size, icon_size, spacing) = size.content(ctx);
        ButtonContent(
            // Create row layout.
            Row(spacing, Offset::Center, Size::Fit, Padding(padding, 0.0, padding, 0.0)),
            // Create avatar if provided
            avatar.map(|content| Avatar::new(ctx, content, None, false, icon_size)),
            // Create left icon if provided
            icon_l.map(|icon| Icon::new(ctx, icon, color, icon_size)),
            // Create label if provided
            label.map(|label| Text::new(ctx, label, TextStyle::Label(color), text_size, TextAlign::Left)),
            // Create right icon if provided
            icon_r.map(|icon| Icon::new(ctx, icon, color, icon_size)),
        )
    }

    fn set_color(&mut self, color: Color) {
        // Set color for left icon
        if let Some(icon) = &mut self.2 { icon.color = Some(color); }
        // Set color for label
        if let Some(text) = &mut self.3 { text.color = color; }
        // Set color for right icon
        if let Some(icon) = &mut self.4 { icon.color = Some(color); }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    // Button width expands as wide as possible
    Expand,
    // Button width will hug it's content
    Hug,
}

impl Button {
    // Primary Button Preset
    pub fn primary (
        ctx: &mut Context,
        label: &'static str,
        element_id: Option<ElementID>,
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
            element_id,
            on_click,
        )
    }

    // Secondary Button Preset
    pub fn secondary(
        ctx: &mut Context,
        icon_l: Option<&'static str>,
        label: &'static str,
        icon_r: Option<&'static str>,
        element_id: Option<ElementID>,
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
            element_id,
            on_click,
        )
    }

    // Ghost Button Preset
    pub fn ghost(
        ctx: &mut Context,
        label: &'static str,
        element_id: Option<ElementID>,
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
            element_id,
            on_click,
        )
    }

    // Disabled Button Preset
    pub fn disabled(
        ctx: &mut Context,
        label: &'static str,
        element_id: Option<ElementID>,
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
            element_id,
            on_click,
        )
    }

    // Numeric Keypad Button Preset
    pub fn keypad(
        ctx: &mut Context,
        label: Option<&'static str>,
        icon: Option<&'static str>,
        element_id: Option<ElementID>,
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
            element_id,
            on_click,
        )
    }

    // Desktop Navigator Button Preset
    pub fn navigation(
        ctx: &mut Context,
        icon: &'static str,
        label: &'static str,
        selected: bool,
        element_id: ElementID,
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
            Some(element_id),
            on_click,
        )
    }

    // Desktop Navigator Profile Button Preset
    pub fn photo(
        ctx: &mut Context,
        label: &'static str,
        photo: AvatarContent,
        selected: bool,
        element_id: ElementID,
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
            Some(element_id),
            on_click,
        )
    }

    // Close Page Button Preset
    pub fn close(
        ctx: &mut Context,
        label: &'static str,
        element_id: Option<ElementID>,
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
            element_id,
            on_click,
        )
    }
}

#[derive(Debug, Component)]
pub struct QuickActions(Wrap, Vec<Button>);
impl Events for QuickActions {}

impl QuickActions {
    pub fn new(buttons: Vec<Button>) -> Self {
        // Wrap of custom buttons (secondary)
        QuickActions(Wrap(8.0, 8.0, Offset::Start, Offset::Center, Padding::default()), buttons)
    }
}

#[derive(Debug, Component)]
pub struct QuickDeselectButton(Stack, Button, #[skip] ElementID);
impl Events for QuickDeselectButton {}

impl QuickDeselectButton {
    pub fn new(ctx: &mut Context, name: &'static str, id: ElementID) -> Self {
        // Wrap of secondary contact buttons
        let button = Button::secondary(ctx, None, name, Some("close"), None, move |ctx: &mut Context| ctx.trigger_event(RemoveContactEvent(id)));
        QuickDeselectButton(Stack::default(), button, id)
    }

    pub fn id(&self) -> ElementID {self.2}
}
