use rust_on_rails::prelude::*;
use crate::Callback;
use crate::elements::images::Icon;
use crate::elements::shapes::OutlinedRectangle;
use crate::layout::{Offset, Padding, Size, Row, Stack};
use super::{ButtonSize, ButtonState, ButtonStyle};

/// The [`IconButton`] is a type of button that contains a singular icon instead of a label.
#[derive(Component)]
pub struct IconButton(Stack, OutlinedRectangle, Image, #[skip] ButtonStyle, #[skip] ButtonState, #[skip] pub Box<dyn FnMut(&mut Context)>);

impl IconButton {
    /// Creates a new [`IconButton`] with specified parameters.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `icon`: A string representing the [`Icon`]'s name.
    /// - `size`: The size of the button.
    /// - `style`: The style of the button. secondary or ghost. (primary is not supported).
    /// - `state`: The initial state of the button.
    /// - `on_click`: A closure that will be executed when the button is clicked.
    ///
    /// # Returns
    /// A new [`IconButton`] instance configured with the given parameters.
    pub fn new(
        ctx: &mut Context,
        icon: &'static str,
        size: ButtonSize,
        style: ButtonStyle,
        state: ButtonState,
        on_click: Box<dyn FnMut(&mut Context)>,
    ) -> Self {
        let colors = state.color(ctx, style);
        let (size, icon_size, radius) = match (style, size) {
            (ButtonStyle::Secondary, ButtonSize::Large) => (52.0, 32.0, 12.0),
            (ButtonStyle::Secondary, ButtonSize::Medium) => (36.0, 20.0, 8.0),
            (ButtonStyle::Ghost, ButtonSize::Large) => (52.0, 48.0, 12.0),
            (ButtonStyle::Ghost, ButtonSize::Medium) => (36.0, 32.0, 8.0),
            _ => panic!("{:?} is not a valid IconButton tyle", style)
        };

        let icon = Icon::new(ctx, icon, colors.label, icon_size);
        let background = OutlinedRectangle::new(colors.background, colors.outline, radius, 1.0);


        IconButton(
            Stack(Offset::Center, Offset::Center, Size::Static(size), Size::Static(size), Padding::default()),
            background, icon, style, state, on_click
        )
    }

    /// Updates the colors of the `IconButton` based on the button's state.
    pub fn color(&mut self, ctx: &mut Context, state: ButtonState) {
        let colors = state.color(ctx, self.3);
        *self.1.background() = colors.background;
        *self.1.outline() = colors.outline;
        self.2.color = Some(colors.label);
    }

    /// Returns a mutable reference to the [`ButtonState`] of the button.
    pub fn status(&mut self) -> &mut ButtonState {&mut self.4}
}

impl OnEvent for IconButton {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(event) = event.downcast_ref::<MouseEvent>() {
            if let Some(state) = self.4.handle(ctx, *event) {
                self.color(ctx, state);
            }
            if let MouseEvent{state: MouseState::Pressed, position: Some(_)} = event {
                match self.4 {
                    ButtonState::Default | ButtonState::Hover | ButtonState::Pressed => (self.5)(ctx),
                    _ => {}
                }
            }
            false
        } else {true}
    }
}

/// A row of secondary style [`IconButton`]s, spaced `24.0` apart.
#[derive(Debug, Component)]
pub struct IconButtonRow(Row, Vec<IconButton>);
impl OnEvent for IconButtonRow {}

impl IconButtonRow {
    /// Creates a new [`IconButtonRow`] component with a list of icon buttons.
    ///
    /// This function initializes the [`IconButtonRow`] by creating a row layout and adding the provided list of
    /// icon buttons. Each button is created with a label and an associated callback function, which is triggered
    /// when the button is clicked. The buttons are arranged in a row with 24.0 units of space between them.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `buttons`: A vector of tuples, where each tuple contains a label (`&'static str`) and a callback function
    ///   (`Box<[`Callback`]>`) to be executed when the corresponding button is clicked.
    ///
    /// # Returns
    /// A new [`IconButtonRow`] component containing the list of icon buttons arranged in a centered row layout.
    ///
    /// # Example
    /// ```rust
    /// let icon_button_row = IconButtonRow::new(ctx, vec![
    ///     ("Button 1", Box::new(|ctx: &mut Context| { /* some action */ })),
    ///     ("Button 2", Box::new(|ctx: &mut Context| { /* some action */ }))
    /// ]);
    /// ```
    pub fn new(ctx: &mut Context, buttons: Vec<(&'static str, Callback)>) -> Self {
        let buttons = buttons.into_iter().map(|(i, on_click)| IconButton::secondary(ctx, i, on_click)).collect();
        IconButtonRow(Row::center(24.0), buttons)
    }
}

impl IconButton {
    /// Creates a new [`IconButton`] preset with a secondary style.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `icon`: The icon for the button, represented by it's name as a string.
    /// - `on_click`: A [`Callback`] that will be executed when the button is clicked.
    ///
    /// # Returns
    /// - A new `IconButton` with a secondary style, large size, and default state.
    pub fn secondary(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: Callback
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Large,
            ButtonStyle::Secondary,
            ButtonState::Default,
            on_click,
        )
    }

    /// Creates a new `IconButton` preset for input fields.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `icon`: The icon for the button, represented as a string (e.g., a file name or path).
    /// - `on_click`: A closure that will be executed when the button is clicked.
    ///
    /// # Returns
    /// - A new [`IconButton`] with style [`ButtonStyle::Secondary`], size [`ButtonSize::Medium`], and state [`ButtonState::Default`].
    pub fn input(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Secondary,
            ButtonState::Default,
            Box::new(on_click),
        )
    }

    /// Creates a new `IconButton` preset for use in a mobile keyboard.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `icon`: The icon for the button, represented as a string (e.g., a file name or path).
    /// - `on_click`: A closure that will be executed when the button is clicked.
    ///
    /// # Returns
    /// - A new [`IconButton`] with style [`ButtonStyle::Ghost`], size [`ButtonSize::Medium`], and state [`ButtonState::Default`].
    pub fn keyboard(
        ctx: &mut Context, 
        icon: &'static str,
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Box::new(on_click),
        )
    }
    
    /// Creates a new [`IconButton`] preset for header navigation.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `icon`: The icon for the button, represented as a string (e.g., a file name or path).
    /// - `on_click`: A closure that will be executed when the button is clicked.
    ///
    /// # Returns
    /// - A new [`IconButton`] with style [`ButtonStyle::Ghost`], size [`ButtonStyle::Medium`], and state [`ButtonState::Default`].
    pub fn navigation(
        ctx: &mut Context, 
        icon: &'static str, 
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Box::new(on_click),
        )
    }

    /// Creates a new [`IconButton`] preset for closing a page.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `on_click`: A closure that will be executed when the button is clicked.
    ///
    /// # Returns
    /// - A new [`IconButton`] with style [`ButtonStyle::Ghost`], size [`ButtonSize::Medium`], and state [`ButtonState::Default`]. The icon is set to "close".
    pub fn close(
        ctx: &mut Context, 
        on_click: impl FnMut(&mut Context) + 'static
    ) -> Self {
        IconButton::new(
            ctx,
            "close",
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            ButtonState::Default,
            Box::new(on_click),
        )
    }

    /// Creates a new [`IconButton`] preset for the app's mobile navigator.
    ///
    /// # Parameters
    /// - `ctx`: The [`Context`] for accessing the app's theme.
    /// - `icon`: The name of the icon to display.
    /// - `selected`: Indicates whether the icon is initially selected.
    /// - `on_click`: A closure that will be executed when the button is clicked.
    ///
    /// # Returns
    /// - A new [`IconButton`] with style [`ButtonStyle::Ghost`], size [`ButtonSize::Medium`].
    pub fn tab_nav(
        ctx: &mut Context, 
        icon: &'static str, 
        selected: bool,
        on_click: impl FnMut(&mut Context) + 'static,
    ) -> Self {
        let state = if selected {ButtonState::Selected} else {ButtonState::UnSelected};
        IconButton::new(
            ctx,
            icon,
            ButtonSize::Medium,
            ButtonStyle::Ghost,
            state,
            Box::new(on_click),
        )
    }
}

impl std::fmt::Debug for IconButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IconButton(...)")
    }
}
