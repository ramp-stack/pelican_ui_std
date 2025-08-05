use pelican_ui::events::{OnEvent, MouseState, Event, MouseEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Align, Component, Image, Color, Shape, ShapeType};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component, resources};

use crate::elements::text::{Text, ExpandableText, TextStyle};
use crate::elements::images::Icon;
use crate::elements::shapes::{Outline, Circle, RoundedRectangle};
use crate::layout::{Column, Stack, Offset, Size, Row, Padding, Bin};
use crate::utils::Callback;

#[derive(Debug, Component)]
pub struct Slider(Column, Text, Option<ExpandableText>, SliderContent);
impl OnEvent for Slider {}
impl Slider {
    pub fn new(
        ctx: &mut Context,
        label: &str, description: Option<&str>,
        on_release: impl FnMut(&mut Context, f32) + 'static,
    ) -> Self {
        let font_size = ctx.theme.fonts.size;
        Slider(Column::new(8.0, Offset::Start, Size::Fit, Padding::default()), 
            Text::new(ctx, label, TextStyle::Heading, font_size.h5, Align::Left),
            description.map(|t| ExpandableText::new(ctx, t, TextStyle::Primary, font_size.md, Align::Left, None)),
            SliderContent::new(ctx, on_release)
        )
    }

    pub fn set_value(&mut self, i: f32) {
        println!("Setting value to {:?}", i);
        let w = self.3.1.inner().shape().shape.size().0;
        println!("Slider width {:?}", w);
        println!("Setting scroll to {:?}", (i * w) / 100.0);
        self.3.2.adjust_scroll((i * w) / 100.0)
    }
}

#[derive(Component)]
pub struct SliderContent(Stack, Bin<Stack, RoundedRectangle>, SliderKnob, #[skip] f32, #[skip] pub Box<dyn FnMut(&mut Context, f32)>, #[skip] bool);
impl SliderContent {
    pub fn new(ctx: &mut Context, on_release: impl FnMut(&mut Context, f32) + 'static) -> Self {
        let slider = Stack(Offset::Center, Offset::Center, Size::fill(), Size::Static(24.0), Padding::default());
        let layout = Stack(Offset::Start, Offset::Center, Size::fill(), Size::Fit, Padding::default());
        let color = ctx.theme.colors.outline.primary;
        SliderContent(layout, Bin(slider, RoundedRectangle::new(0.0, 12.0, color)), SliderKnob::new(ctx), 0.0, Box::new(on_release), false)
    }
    
    pub fn value(&mut self) -> &mut f32 {&mut self.3}
    pub fn percentage(&mut self) -> f32 {
        let w = self.1.inner().shape().shape.size().0;
        (*self.value() / w) * 100.0
    }
}

impl OnEvent for SliderContent {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent { state: MouseState::Pressed, position: Some((x, y)) }) = event.downcast_ref::<MouseEvent>() {
            self.2.adjust_scroll(*x);
            *self.value() = *x;
            self.5 = true;
        } else if let Some(MouseEvent { state: MouseState::Released, position: Some(_)}) = event.downcast_ref::<MouseEvent>() {
            let p = self.percentage().clone();
            (self.4)(ctx, p);
            self.5 = false;
        } else if let Some(MouseEvent { state: MouseState::Moved, position: Some((x, y)) }) = event.downcast_ref::<MouseEvent>() {
            *self.value() = *x;
            if self.5 { self.2.adjust_scroll(*x); }
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
pub struct SliderKnob(Stack, Shape, Shape);
impl OnEvent for SliderKnob {}
impl SliderKnob {
    pub fn new(ctx: &mut Context) -> Self {
        let background = ctx.theme.colors.brand.primary;
        let outline = ctx.theme.colors.brand.primary;
        // let layout = Stack(Offset::Static(start_val), Offset::Center, Size::Fit, Size::Fit, Padding::default());
        SliderKnob(Stack::default(), Circle::new(36.0, background), Outline::circle(36.0, outline))
    }

    pub fn adjust_scroll(&mut self, i: f32) {
        println!("ADJUST SCROLL {:?}", i);
        let original = match self.0.0 {
            Offset::Static(x) => x,
            _ => 0.0
        };

        let half = 36.0/2.0;
        self.0.0 = Offset::Static(i-(half));
    }
}