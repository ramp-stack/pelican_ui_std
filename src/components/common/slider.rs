use pelican_ui::events::{OnEvent, MouseState, Event, MouseEvent};
use pelican_ui::drawable::{Drawable, Component, Image, Color, Shape, ShapeType};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component, resources};

use crate::elements::images::Icon;
use crate::elements::shapes::{Outline, Circle, RoundedRectangle};
use crate::layout::{Stack, Offset, Size, Row, Padding, Bin};
use crate::utils::Callback;

#[derive(Debug, Component)]
pub struct Slider(Stack, Bin<Stack, RoundedRectangle>, SliderKnob, #[skip] f32);
impl Slider {
    pub fn new(ctx: &mut Context) -> Self {
        let slider = Stack(Offset::Center, Offset::Center, Size::fill(), Size::Static(48.0), Padding::default());
        let layout = Stack(Offset::Start, Offset::Center, Size::fill(), Size::Fit, Padding::default());
        let color = ctx.theme.colors.outline.primary;
        Slider(layout, Bin(slider, RoundedRectangle::new(0.0, 12.0, color)), SliderKnob::new(ctx), 0.0)
    }

    pub fn adjust_scroll(&mut self, i: f32) {
        let original = match self.0.0 {
            Offset::Static(x) => x,
            _ => 0.0
        };

        self.0.0 = Offset::Static(original+i);
    }
}

impl OnEvent for Slider {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(MouseEvent { state: MouseState::Scroll(x, _), position: Some(_) }) = event.downcast_ref::<MouseEvent>() {
            println!("Received scroll event {:?}", x);
            self.adjust_scroll(*x);
        }
        true
    }
}

#[derive(Debug, Component)]
pub struct SliderKnob(Stack, Shape, Shape);
impl OnEvent for SliderKnob {}
impl SliderKnob {
    pub fn new(ctx: &mut Context) -> Self {
        let background = ctx.theme.colors.background.primary;
        let outline = ctx.theme.colors.outline.primary;
        SliderKnob(Stack::default(), Circle::new(48.0, background), Outline::circle(48.0, outline))
    }
}