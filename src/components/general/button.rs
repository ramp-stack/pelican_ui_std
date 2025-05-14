use rust_on_rails::prelude::*;
use crate::theme::colors::ButtonColorScheme;
use crate::PelicanUI;

#[allow(clippy::module_inception)]
mod button;
pub use button::{Button, ButtonWidth, QuickActions, QuickDeselectButton};

mod icon;
pub use icon::{IconButton, IconButtonRow};

/// Represents the style of a button.
///
/// The `ButtonStyle` enum defines different styles for buttons in the UI. Each style is
/// meant to be used in specific scenarios.
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonStyle {
    /// A standard button style that is most often used for main actions.
    Primary,
    /// A less prominent button style, usually used for secondary actions.
    Secondary,
    /// A transparent or minimal button style, often used for least important actions.
    Ghost
}

/// Represents the various states of a button.
///
/// The `ButtonState` enum defines different states that a button can be in during interaction.
/// These states allow the button's appearance and behavior to change dynamically in response to
/// user input (e.g., when the button is pressed, hovered, or disabled).
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum ButtonState {
    /// The normal, un-interacted state of the button.
    Default,
    /// The state of the button when it is  unclickable.
    Disabled,
    /// The state of the button when it has been selected or toggled on.
    Selected,
    /// The state of the button when it is toggled off.
    UnSelected,
    /// The state of the button when it is being clicked or pressed.
    Pressed,
    /// The state of the button when the mouse is hovering over it.
    Hover,
}

impl ButtonState {
    /// Handles the state transitions for the button based on mouse events.
    ///
    /// This method takes a mouse event and updates the button's state accordingly. 
    /// It handles transitions between states like hovering, pressing, and being selected.
    ///
    /// # Parameters:
    /// - **`_ctx`**: The current context (not used in this method, but can be extended if needed).
    /// - **`event`**: The mouse event that triggers the state change, containing event position and mouse state.
    ///
    /// # Returns:
    /// - **`Option<ButtonState>`**: The new state of the button, if it changes.
    ///
    /// # Example:
    /// ```rust
    /// button_state.handle(&mut ctx, event);
    /// ```
    pub fn handle(&mut self, _ctx: &mut Context, event: MouseEvent) -> Option<Self> {
        let state = match self {
            ButtonState::Default | ButtonState::UnSelected if event.position.is_some() => {
                match event.state {
                    MouseState::Pressed => {
                        #[cfg(target_os = "ios")]
                        crate::vibrate();
                        Some(ButtonState::Pressed)
                    },
                    MouseState::Moved => Some(if crate::config::IS_MOBILE {ButtonState::Default} else {ButtonState::Hover}),
                    _ => None
                }
            },
            ButtonState::Pressed => {
                match event.state {
                    MouseState::Released if event.position.is_some() => Some(if crate::config::IS_MOBILE {ButtonState::Default} else {ButtonState::Hover}),
                    MouseState::Moved if event.position.is_none() => Some(ButtonState::Default),
                    _ => None
                }
            },
            ButtonState::Hover => {
                match event.state {
                    MouseState::Pressed if event.position.is_some() => Some(ButtonState::Pressed),
                    MouseState::Moved if event.position.is_none() => Some(ButtonState::Default),
                    _ => None
                }
            }
            _ => None
        };
        if let Some(state) = state { *self = state; }
        state
    }

    /// Returns the color scheme for the button based on its state and style.
    ///
    /// This method determines the appropriate color scheme for the button depending on its current state
    /// (e.g., default, pressed, disabled) and style (primary, secondary, or ghost).
    ///
    /// # Parameters:
    /// - **`ctx`**: The current context, used to access the theme's colors.
    /// - **`style`**: The style of the button (Primary, Secondary, or Ghost).
    ///
    /// # Returns:
    /// - **`ButtonColorScheme`**: The color scheme for the button based on its style and state.
    fn color(&self, ctx: &mut Context, style: ButtonStyle) -> ButtonColorScheme {
        let schemes = &ctx.get::<PelicanUI>().theme.colors.button;
        match (style, self) {
            (ButtonStyle::Primary, ButtonState::Default) => schemes.primary_default,
            (ButtonStyle::Primary, ButtonState::Disabled) => schemes.primary_disabled,
            (ButtonStyle::Primary, ButtonState::Hover) => schemes.primary_hover,
            (ButtonStyle::Primary, ButtonState::Pressed) => schemes.primary_pressed,
            (ButtonStyle::Primary, ButtonState::Selected) => schemes.primary_selected,
            (ButtonStyle::Primary, ButtonState::UnSelected) => schemes.ghost_disabled,

            (ButtonStyle::Secondary, ButtonState::Default) => schemes.secondary_default,
            (ButtonStyle::Secondary, ButtonState::Disabled) => schemes.secondary_disabled,
            (ButtonStyle::Secondary, ButtonState::Hover) => schemes.secondary_hover,
            (ButtonStyle::Secondary, ButtonState::Pressed) => schemes.secondary_pressed,
            (ButtonStyle::Secondary, ButtonState::Selected) => schemes.secondary_selected,
            (ButtonStyle::Secondary, ButtonState::UnSelected) => schemes.ghost_disabled,

            (ButtonStyle::Ghost, ButtonState::Default) => schemes.ghost_default,
            (ButtonStyle::Ghost, ButtonState::Disabled) => schemes.ghost_disabled,
            (ButtonStyle::Ghost, ButtonState::Hover) => schemes.ghost_hover,
            (ButtonStyle::Ghost, ButtonState::Pressed) => schemes.ghost_pressed,
            (ButtonStyle::Ghost, ButtonState::Selected) => schemes.ghost_selected,
            (ButtonStyle::Ghost, ButtonState::UnSelected) => schemes.ghost_disabled,
        }
    }
}

/// Represents the size of a button.
///
/// The `ButtonSize` enum defines different sizes for buttons, allowing for customization of button
/// size based on context or UI requirements.
#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    /// A larger button size, often used for primary or more important actions.
    Large,
    /// A medium-sized button, typically used for secondary actions or less prominent buttons.
    Medium,
}

impl ButtonSize {
    fn content(&self, ctx: &mut Context) -> (f32, f32, f32) { // text size, icon size, spacing
        let font_size = ctx.get::<PelicanUI>().theme.fonts.size;
        match self {
            ButtonSize::Medium => (font_size.md, 16., 4.),
            ButtonSize::Large => (font_size.lg, 24., 12.)
        }
    }
    fn background(&self) -> (f32, f32) { // height, padding
        match self {
            ButtonSize::Medium => (32., 12.),
            ButtonSize::Large => (48., 24.)
        }
    }
}
