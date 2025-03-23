use rust_on_rails::prelude::*;
use crate::elements::icon::Icon;
use crate::elements::shapes::{Circle, Outline};
use crate::layout::{Stack, Offset, Size};
use crate::PelicanUI;

// Rules:
// Exported structs and enums prefixed with name of the "top-layer" component.
// If a struct or enum isn’t exported, start its name with _.
// First item in a file should be top-layer component struct or enum
// 'User' should never touch the struct, only new functions

pub struct CircleIcon(pub _CircleIconContent, pub Option<Shape>, pub Option<_Flair>);

impl CircleIcon {
    pub fn new(ctx: &mut Context, content: CircleIconContent, flair: Option<(&'static str, CircleIconStyle)>, outline: bool, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        let flair_s = (size as f32 / 3.0).round() as u32;

        CircleIcon(
            _CircleIconContent::new(ctx, content, size),  // Adds primary content
            outline.then(|| Outline::circle(size, black)), // Adds an outline if enabled.
            flair.map(|(name, style)| _Flair::new(ctx, name, style, flair_s)) // Creates a flair if provided.
        )
    }
}

impl Component for CircleIcon {
    fn build(&mut self, _ctx: &mut Context, _max_size: (u32, u32)) -> Container {
        let mut children: Vec<&mut dyn Drawable> = vec![];

        // Adds either an icon or an image.
        match &mut self.0 {
            _CircleIconContent::Icon(icon) => children.push(icon),
            _CircleIconContent::Image(image) => children.push(image)
        }

        // Adds optional outline and flair if they exist.
        if let Some(outline) = &mut self.1 { children.push(outline); }
        if let Some(flair) = &mut self.2 { children.push(flair); }

        Container::new(Stack(Offset::BottomRight, Size::default()), children)
    }
}

/// Defines available styles for a `CircleIcon`.
pub enum CircleIconStyle {
    Primary,
    Secondary,
    Brand,
    Success,
    Warning,
    Danger
}

impl CircleIconStyle {
    /// Retrieves icon and background colors for the given style.
    pub fn get(&self, ctx: &mut Context) -> (Color, Color) {
        let colors = &ctx.get::<PelicanUI>().theme.colors;
        match self {
            CircleIconStyle::Primary => (colors.text.heading, colors.background.primary),
            CircleIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
            CircleIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
            CircleIconStyle::Success => (colors.status.success, colors.text.heading),
            CircleIconStyle::Warning => (colors.status.warning, colors.text.heading),
            CircleIconStyle::Danger => (colors.status.danger, colors.text.heading),
        }
    }
}

/// Defines content types.
pub enum CircleIconContent {
    Icon(&'static str, CircleIconStyle),
    Image(resources::Image)
}

/// Internal enumerator for handling icon content.
pub enum _CircleIconContent {
    Icon(_CircleIconData),
    Image(Image)
}

impl _CircleIconContent {
    pub fn new(ctx: &mut Context, input: CircleIconContent, s: u32) -> Self {
        match input {
            CircleIconContent::Image(image) => _CircleIconContent::Image(Image(ShapeType::Ellipse(0, (s, s)), image, None)),
            CircleIconContent::Icon(name, style) => _CircleIconContent::Icon(_CircleIconData::new(ctx, name, style, s))
        }
    }
}

/// Stores shape and icon data for a 'CircleIcon'.
pub struct _CircleIconData(pub Shape, pub Icon);

impl _CircleIconData {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        let icon_size = (size as f32 * 0.75).round() as u32;
        let (background, icon_color) = style.get(ctx);
        _CircleIconData(
            Circle::new(size - 2, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
}

impl Component for _CircleIconData {
    fn build(&mut self, _ctx: &mut Context, _max_size: (u32, u32)) -> Container {
        Container::new(Stack(Offset::Center, Size::default()), vec![&mut self.0, &mut self.1])
    }
}

/// Flair for a 'CircleIcon'.
//#[derive(Component(Stack(Offset::Center, Size::default())))]
pub struct _Flair(_CircleIconData, Shape);

impl _Flair {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        _Flair(_CircleIconData::new(ctx, name, style, size), Outline::circle(size, black))
    }
}

impl Component for _Flair {
    fn build(&mut self, _ctx: &mut Context, _max_size: (u32, u32)) -> Container {
        Container::new(Stack(Offset::Center, Size::default()), vec![&mut self.0, &mut self.1])
    }
}
