use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{Drawable, Component, ShapeType, Shape, Color};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::layout::Stack;

/// Represents a rectangle with a background and an outline, both of which are rounded.
/// The `OutlinedRectangle` uses two `RoundedRectangle` components, one for the background
/// and another for the outline, combined in a `Stack` layout.
#[derive(Debug, Component)]
pub struct OutlinedRectangle(Stack, RoundedRectangle, RoundedRectangle);

impl OnEvent for OutlinedRectangle {}

impl OutlinedRectangle {
    /// Creates a new `OutlinedRectangle` with specified background color, outline color, corner radius, and outline stroke width.
    ///
    /// # Parameters:
    /// - `bg`: The background color of the rectangle.
    /// - `oc`: The outline color.
    /// - `radius`: The corner radius for the rectangle's rounded corners.
    /// - `stroke`: The stroke width for the rectangle's outline.
    ///
    /// # Returns:
    /// A new `OutlinedRectangle` component.
    pub fn new(bg: Color, oc: Color, radius: f32, stroke: f32) -> Self {
        OutlinedRectangle(
            Stack::default(),
            RoundedRectangle::new(0.0, radius, bg),
            RoundedRectangle::new(stroke, radius, oc)
        )
    }

    /// Returns a mutable reference to the background color of the `OutlinedRectangle`.
    pub fn background(&mut self) -> &mut Color {&mut self.1.shape().color}

    /// Returns a mutable reference to the outline color of the `OutlinedRectangle`.
    pub fn outline(&mut self) -> &mut Color {&mut self.2.shape().color}

    pub fn size(&self) -> (f32, f32) {self.2.0.shape.size()}
}

/// Represents a rounded rectangle with a stroke and corner radius.
#[derive(Debug)]
pub struct RoundedRectangle(Shape);

impl RoundedRectangle {
    /// Returns a mutable reference to the shape of the `RoundedRectangle`.
    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }

    /// Creates a new `RoundedRectangle` with a given stroke width, corner radius, and color.
    ///
    /// # Parameters:
    /// - `s`: The stroke width of the rectangle.
    /// - `r`: The corner radius.
    /// - `color`: The color of the rectangle.
    ///
    /// # Returns:
    /// A new `RoundedRectangle` component.
    pub fn new(s: f32, r: f32, color: Color) -> Self {
        RoundedRectangle(Shape{shape: ShapeType::RoundedRectangle(s, (0.0, 0.0), r), color})
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
        if let ShapeType::RoundedRectangle(_, s, _) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}

/// Represents a basic rectangle with no rounded corners or stroke.
#[derive(Debug)]
pub struct Rectangle(Shape);

impl Rectangle {
    /// Creates a new `Rectangle` with a specified color.
    ///
    /// # Parameters:
    /// - `color`: The color of the rectangle.
    ///
    /// # Returns:
    /// A new `Rectangle` component.
    pub fn new(color: Color) -> Self {
        Rectangle(Shape { shape: ShapeType::Rectangle(0.0, (0.0, 0.0)), color })
    }

    /// Returns a mutable reference to the shape of the `Rectangle`.
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
        if let ShapeType::Rectangle(_, s) = &mut self.0.shape {
            *s = size;
        }
        vec![Area { offset: (0.0, 0.0), size }]
    }
}


/// Utility struct for creating circular shapes (for example, to represent icons or buttons).
pub struct Outline;

impl Outline {
    /// Creates a circular outline for a given size and color.
    ///
    /// # Parameters:
    /// - `s`: The size of the circle.
    /// - `color`: The color of the circle.
    ///
    /// # Returns:
    /// A `Shape` representing the circular outline.
    pub fn circle(s: f32, color: Color) -> Shape {
        Shape { shape: ShapeType::Ellipse(s * 0.06, (s, s)), color }
    }
}

/// Represents a circle shape, typically used for icons or buttons.
pub struct Circle;

impl Circle {
    /// Creates a new circle shape with a specified size and color.
    ///
    /// # Parameters:
    /// - `s`: The size of the circle.
    /// - `color`: The color of the circle.
    ///
    /// # Returns:
    /// A `Shape` representing the circle.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(s: f32, color: Color) -> Shape {
        Shape { shape: ShapeType::Ellipse(0.0, (s, s)), color }
    }
}
