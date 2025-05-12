use rust_on_rails::prelude::*;
use crate::ElementID;
use crate::components::avatar::{Avatar, AvatarContent};
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::elements::text::{Text, TextStyle};
use crate::events::RemoveContactEvent;
use crate::layout::{Offset, Padding, Row, Size, Stack, Wrap};

use super::{ButtonSize, ButtonState, ButtonStyle};

/// Defines the width behavior for the button.
#[derive(Debug, Clone, Copy)]
pub enum ButtonWidth {
    /// The button expands to fill the available space.
    Expand,
    
    /// The button's width adjusts to fit its content.
    Hug,
}

/// A clickable button component with customizable content, size, and styles. It supports various states
/// (e.g., default, hover, pressed) and handles click events to trigger actions.
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
    /// Creates a new `Button` component.
    ///
    /// # Parameters:
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `avatar`: An optional avatar image to display inside the button.
    /// - `icon_l`: An optional icon to display on the left side of the button.
    /// - `label`: An optional label for the button's text.
    /// - `icon_r`: An optional icon to display on the right side of the button.
    /// - `size`: Defines the size of the button, such as its height and padding.
    /// - `width`: Determines how the button's width behaves (e.g., hug content or expand).
    /// - `style`: Defines the button's overall style (colors, appearance).
    /// - `state`: Specifies the initial state of the button (e.g., default, hover).
    /// - `offset`: Specifies the button's position relative to its parent.
    /// - `on_click`: A closure to define the action to perform when the button is clicked.
    ///
    /// # Returns:
    /// A `Button` instance, which is ready to be used within the UI.
    ///
    /// # Example:
    /// ```
    /// let button = Button::new(
    ///     ctx, 
    ///     Some(avatar_data),
    ///     Some("left"),
    ///     Some("Click Me"),
    ///     Some("right"),
    ///     ButtonSize::Medium,
    ///     ButtonWidth::Hug,
    ///     ButtonStyle::Primary,
    ///     ButtonState::Default,
    ///     Offset::Start,
    ///     |ctx: &mut Context| { println!("Button clicked!") }
    /// );
    /// ```
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

    /// Updates the color of the button based on its current state and style.
    ///
    /// # Parameters:
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    ///
    /// This function updates the button's background, outline, and label colors.
    pub fn color(&mut self, ctx: &mut Context) {
        let colors = self.4.color(ctx, self.3);
        self.2.set_color(colors.label);
        *self.1.outline() = colors.outline;
        *self.1.background() = colors.background;
    }

    /// Returns a mutable reference to the current state of the button.
    ///
    /// # Returns:
    /// - A mutable reference to `ButtonState`, which can be used to modify the button's state.
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

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Button")
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

impl Button {
    /// Creates a primary style button. Typically used for the main actions in the UI.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `label`: The text displayed on the button.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Primary` style and default state.
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

    /// Creates a secondary style button with optional left and right icons.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `icon_l`: An optional icon to display on the left side of the button.
    /// - `label`: The text displayed on the button.
    /// - `icon_r`: An optional icon to display on the right side of the button.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Secondary` style and default state.
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

    /// Creates a ghost style button, typically used for non-intrusive actions.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `label`: The text displayed on the button.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Ghost` style and default state.
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

    /// Creates a disabled button, which cannot be interacted with.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `label`: The text displayed on the button.
    /// - `on_click`: A closure that defines the action when the button is clicked (this will not be triggered as the button is disabled).
    ///
    /// # Returns
    /// A `Button` with the `Primary` style and `Disabled` state.
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

    /// Creates a numeric keypad style button, typically used for numbers or symbols on a keypad.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `label`: The text displayed on the button.
    /// - `icon`: An optional icon displayed on the button.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Ghost` style and default state.
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

    /// Creates a navigation button for desktop-style navigators, with optional selection.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `icon`: The icon to display on the button.
    /// - `label`: The text displayed on the button.
    /// - `selected`: A flag that determines if the button should be in the selected state.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Ghost` style and either the `Selected` or `Default` state.
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

    /// Creates a profile photo button for desktop-style navigation with a photo.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `label`: The text displayed on the button.
    /// - `photo`: The photo or avatar content for the button.
    /// - `selected`: A flag that determines if the button should be in the pressed state.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Ghost` style and either the `Pressed` or `Default` state.
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

    /// Creates a close page button, typically used for closing dialogs or pages.
    ///
    /// # Parameters
    /// - `ctx`: The current context, used for accessing themes and UI elements.
    /// - `label`: The text displayed on the button.
    /// - `on_click`: A closure that defines the action when the button is clicked.
    ///
    /// # Returns
    /// A `Button` with the `Secondary` style and default state.
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
