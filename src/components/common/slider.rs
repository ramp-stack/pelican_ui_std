use pelican_ui::events::{OnEvent, MouseState, Event, MouseEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Align, Component, Shape};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use crate::elements::{Text, ExpandableText, TextStyle, Circle, RoundedRectangle};
use crate::layout::{Column, Stack, Offset, Size, Padding, Bin};

/// ## Slider
///
/// A UI component that allows users to select a value along a continuous range. 
///
/// ![Slider Example](https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/src/examples/slider.png)
///
/// ### Example
/// ```rust
/// let slider = Slider::new(
///     ctx,V
///     50.0,
///     Some("Volume"),
///     Some("Adjust the sound level"),
///     |ctx: &mut Context, percentage: f32| {
///         println!("Slider released at %{percentage}");
///     }
/// );
/// ```

#[derive(Debug, Component)]
pub struct Slider(Column, Option<Text>, Option<ExpandableText>, SliderContent, #[skip] Option<f32>);
impl Slider {
    pub fn new(
        ctx: &mut Context, start: f32,
        label: Option<&str>, description: Option<&str>,
        on_release: impl FnMut(&mut Context, f32) + 'static,
    ) -> Self {
        let font_size = ctx.theme.fonts.size;
        Slider(Column::new(8.0, Offset::Start, Size::Fit, Padding::default()), 
            label.map(|l| Text::new(ctx, l, TextStyle::Heading, font_size.h5, Align::Left)),
            description.map(|t| ExpandableText::new(ctx, t, TextStyle::Primary, font_size.md, Align::Left, None)),
            SliderContent::new(ctx, on_release), Some(start)
        )
    }

    pub fn set_value(&mut self, i: f32) {
        let w = self.3.1.inner().shape().shape.size().0;
        self.3.2.adjust_scroll((i * w) / 100.0)
    }

    pub fn trigger_event(&mut self, ctx: &mut Context, p: f32) {
        (self.3.4)(ctx, p)
    }
}

impl OnEvent for Slider {
    fn on_event(&mut self, _ctx: &mut Context, event: &mut dyn Event) -> bool {
        if event.downcast_ref::<TickEvent>().is_some() {
            if let Some(val) = self.4.take() {
                self.set_value(val)
            }
        }
        true
    }
}

type SliderClosure = Box<dyn FnMut(&mut Context, f32)>;

#[derive(Component)]
pub struct SliderContent(Stack, Bin<Stack, RoundedRectangle>, SliderKnob, #[skip] f32, #[skip] SliderClosure, #[skip] bool);
impl SliderContent {
    pub fn new(ctx: &mut Context, on_release: impl FnMut(&mut Context, f32) + 'static) -> Self {
        let width = Size::custom(move |widths: Vec<(f32, f32)>|(widths[0].0.min(300.0), 300.0));
        let slider = Stack(Offset::Center, Offset::Center, width, Size::Static(12.0), Padding::default());
        let layout = Stack(Offset::Start, Offset::Center, Size::Fit, Size::Static(48.0), Padding::default());
        let color = ctx.theme.colors.brand.primary;
        SliderContent(layout, Bin(slider, RoundedRectangle::new(0.0, 6.0, color)), SliderKnob::new(ctx), 0.0, Box::new(on_release), false)
    }
    
    pub fn value(&mut self) -> &mut f32 {&mut self.3}
    
    pub fn percentage(&mut self) -> f32 {
        let w = self.1.inner().shape().shape.size().0;
        (*self.value() / w) * 100.0
    }
}

impl OnEvent for SliderContent {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent { state: MouseState::Pressed, position: Some((x, _)) }) = event.downcast_ref::<MouseEvent>() {
            self.2.adjust_scroll(*x);
            *self.value() = *x;
            self.5 = true;
        } else if let Some(MouseEvent { state: MouseState::Released, position: _}) = event.downcast_ref::<MouseEvent>() {
            if self.5 {
                self.5 = false;
                let p = self.percentage();
                (self.4)(ctx, p);
            }
        } else if let Some(MouseEvent { state: MouseState::Scroll(..), position: Some((x, _))}) = event.downcast_ref::<MouseEvent>() {
            *self.value() = *x;
            if self.5 { self.2.adjust_scroll(*x); }
        } else if let Some(MouseEvent { state: MouseState::Moved, position: Some((x, _)) }) = event.downcast_ref::<MouseEvent>() {
            *self.value() = *x;
            if self.5 { self.2.adjust_scroll(*x); }
        } else if event.downcast_ref::<TickEvent>().is_some() && self.5 {
            let p = self.percentage();
            (self.4)(ctx, p);
        }
        true
    }
}

impl std::fmt::Debug for SliderContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SliderContent")
    }
}

#[derive(Debug, Component)]
pub struct SliderKnob(Stack, Shape);
impl OnEvent for SliderKnob {}
impl SliderKnob {
    pub fn new(ctx: &mut Context) -> Self {
        let background = ctx.theme.colors.text.heading;
        SliderKnob(Stack::default(), Circle::new(18.0, background))
    }

    pub fn adjust_scroll(&mut self, i: f32) {self.0.0 = Offset::Static(i-9.0)}
}