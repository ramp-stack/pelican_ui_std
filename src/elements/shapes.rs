use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component, ShapeType, Shape, Color};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::layout::Stack;

/// # Outlined Rectangle
///
/// A rectangle component with a customizable background and outline.  
/// The rectangle supports rounded corners and adjustable stroke thickness.
///
/// Rectangles will always expand in all directions, so wrap them in a [`Bin`]
/// with a [`Stack`] layout to specify a size.
///
/// <img src="https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/src/examples/outlined_rectangle.png"
///      alt="Outlined Rectangle Example"
///      width="400">
///
/// ## Example
/// ```rust
/// let background = ctx.theme.colors.brand.primary;
/// let outline = ctx.theme.colors.outline.primary;
/// let rect = OutlinedRectangle::new(backround, outline, 8.0, 8.0);
/// let layout = Stack(Offset::Center, Offset::Center, Size::Static(100.0), Size::Static(100.0), Padding::default());
/// let shape = Bin(layout, rect);
/// ```
#[derive(Debug, Component)]
pub struct OutlinedRectangle(Stack, RoundedRectangle, RoundedRectangle);

impl OnEvent for OutlinedRectangle {}

impl OutlinedRectangle {
    pub fn new(bg: Color, oc: Color, radius: f32, stroke: f32) -> Self {
        OutlinedRectangle(
            Stack::default(),
            RoundedRectangle::new(0.0, radius, bg),
            RoundedRectangle::new(stroke, radius, oc)
        )
    }

    pub fn background(&mut self) -> &mut Color {&mut self.1.shape().color}
    pub fn outline(&mut self) -> &mut Color {&mut self.2.shape().color}
    pub fn size(&self) -> (f32, f32) {self.2.0.shape.size()}
}

/// # Roundend Rectangle
///
/// A rectangle component with a customizable color, radius and stroke thickness.  
/// Setting the stroke thickness to `0.0` creates a fully filled shape; any other value draws only the outline.
///
/// Rectangles will always expand in all directions, so wrap them in a [`Bin`]
/// with a [`Stack`] layout to specify a size.
///
/// <img src="https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/src/examples/rounded_rectangle.png"
///      alt="Rounded Rectangle Example"
///      width="400">
///
/// ## Example
/// ```rust
/// let background = ctx.theme.colors.brand.primary;
/// let rect = RoundedRectangle::new(0.0, 8.0, background);
/// let layout = Stack(Offset::Center, Offset::Center, Size::Static(100.0), Size::Static(100.0), Padding::default());
/// let shape = Bin(layout, rect);
/// ```
#[derive(Debug)]
pub struct RoundedRectangle(Shape);

impl RoundedRectangle {
    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }
    pub fn new(s: f32, r: f32, color: Color) -> Self {
        RoundedRectangle(Shape{shape: ShapeType::RoundedRectangle(s, (0.0, 0.0), r, 0.0), color})
    }
}

impl OnEvent for RoundedRectangle {}
impl Component for RoundedRectangle {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {vec![&mut self.0]}
    fn children(&self) -> Vec<&dyn Drawable> {vec![&self.0]}
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::fill()
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        if let ShapeType::RoundedRectangle(_, s, _, _) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}

/// # Rectangle
///
/// A rectangle component with a customizable color and stroke thickness.  
/// Setting the stroke thickness to `0.0` creates a fully filled shape; any other value draws only the outline.
///
/// Rectangles will always expand in all directions, so wrap them in a [`Bin`]
/// with a [`Stack`] layout to specify a size.
///
/// <img src="https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/src/examples/rectangle.png"
///      alt="Rectangle Example"
///      width="400">
///
/// ## Example
/// ```rust
/// let background = ctx.theme.colors.brand.primary;
/// let rect = Rectangle::new(background, 0.0);
/// let layout = Stack(Offset::Center, Offset::Center, Size::Static(100.0), Size::Static(100.0), Padding::default());
/// let shape = Bin(layout, rect);
/// ```
#[derive(Debug)]
pub struct Rectangle(Shape);

impl Rectangle {
    pub fn new(color: Color, stroke: f32) -> Self {
        Rectangle(Shape { shape: ShapeType::Rectangle(stroke, (0.0, 0.0), 0.0), color })
    }

    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }
}

impl OnEvent for Rectangle {}
impl Component for Rectangle {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> { vec![&mut self.0] }
    fn children(&self) -> Vec<&dyn Drawable> { vec![&self.0] }
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::fill()
    }
    fn build(&mut self, _ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        if let ShapeType::Rectangle(_, s, _) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}

/// # Outline
///
/// Creates an outlined shape with a specified size and color.
///
/// <img src="https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/src/examples/outline.png"
///      alt="Outline Example"
///      width="400">
///
/// ## Example
/// ```rust
/// let color = ctx.theme.colors.outline.primary;
/// let outline = Outline::circle(100.0, color);
/// ```
pub struct Outline;

impl Outline {
    /// Returns a circle shape with an outline.
    /// The stroke thickness is proportional to the size (`s * 0.06`).
    pub fn circle(s: f32, color: Color) -> Shape {
        Shape { shape: ShapeType::Ellipse(s * 0.06, (s, s), 0.0), color }
    }
}

/// # Circle
///
/// Creates a filled circle shape with a specified size and color.
///
/// <img src="https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/src/examples/circle.png"
///      alt="Circle Example"
///      width="400">  
///
/// ## Example
/// ```rust
/// let color = ctx.theme.colors.brand.primary;
/// let circle = Circle::new(100.0, color); 
/// ```
pub struct Circle;

impl Circle {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(s: f32, color: Color) -> Shape {
        Shape { shape: ShapeType::Ellipse(0.0, (s, s), 0.0), color }
    }
}
