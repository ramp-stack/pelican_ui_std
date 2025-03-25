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

#[derive(Clone, Debug)]
pub struct CircleIcon(Stack, pub _CircleIconContent, pub Option<Shape>, pub Option<_Flair>);

impl CircleIcon {
    pub fn new(ctx: &mut Context, content: CircleIconContent, flair: Option<(&'static str, CircleIconStyle)>, outline: bool, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        let flair_s = (size as f32 / 3.0).round() as u32;

        CircleIcon(
            Stack((Offset::End, Offset::End), (Size::Fit, Size::Fit)),
            _CircleIconContent::new(ctx, content, size),  // Adds primary content
            outline.then(|| Outline::circle(size, black)), // Adds an outline if enabled.
            flair.map(|(name, style)| _Flair::new(ctx, name, style, flair_s)) // Creates a flair if provided.
        )
    }
}

impl Component for CircleIcon {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {
        let mut children: Vec<&mut ComponentRef> = vec![];

        // Adds either an icon or an image.
        match &mut self.1 {
            _CircleIconContent::Icon(icon) => children.push(icon),
            _CircleIconContent::Image(image) => children.push(image)
        }

        // Adds optional outline and flair if they exist.
        if let Some(outline) = &mut self.2 { children.push(outline); }
        if let Some(flair) = &mut self.3 { children.push(flair); }
        
        children
    }

    fn children(&self) -> Vec<&ComponentRef> {
        let mut children: Vec<&ComponentRef> = vec![];

        // Adds either an icon or an image.
        match &self.1 {
            _CircleIconContent::Icon(icon) => children.push(icon),
            _CircleIconContent::Image(image) => children.push(image)
        }

        // Adds optional outline and flair if they exist.
        if let Some(outline) = &self.2 { children.push(outline); }
        if let Some(flair) = &self.3 { children.push(flair); }
        
        children
    }
    fn layout(&self) -> &dyn Layout {&self.0}
}

/// Defines available styles for a `CircleIcon`.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum CircleIconContent {
    Icon(&'static str, CircleIconStyle),
    Image(resources::Image)
}

/// Internal enumerator for handling icon content.
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct _CircleIconData(Stack, pub Shape, pub Icon);

impl _CircleIconData {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        let icon_size = (size as f32 * 0.75).round() as u32;
        let (background, icon_color) = style.get(ctx);
        _CircleIconData(
            Stack((Offset::Center, Offset::Center), (Size::Fit, Size::Fit)),
            Circle::new(size - 2, background), 
            Icon::new(ctx, name, icon_color, icon_size)
        )
    }
}

impl Component for _CircleIconData {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
    fn children(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
    fn layout(&self) -> &dyn Layout {&self.0}
}


#[derive(Clone, Debug)]
pub struct _Flair(Stack, _CircleIconData, Shape);

impl _Flair {
    pub fn new(ctx: &mut Context, name: &'static str, style: CircleIconStyle, size: u32) -> Self {
        let black = ctx.get::<PelicanUI>().theme.colors.shades.black;
        _Flair(
            Stack((Offset::Center, Offset::Center), (Size::Fit, Size::Fit)),
            _CircleIconData::new(ctx, name, style, size), 
            Outline::circle(size, black)
        )
    }
}

impl Component for _Flair {
    fn children_mut(&mut self) -> Vec<&mut ComponentRef> {vec![&mut self.1, &mut self.2]}
    fn children(&self) -> Vec<&ComponentRef> {vec![&self.1, &self.2]}
    fn layout(&self) -> &dyn Layout {&self.0}
}