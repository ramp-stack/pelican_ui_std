use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::RoundedRectangle;
use crate::layout::{Column, Stack, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct QRCodeScanner(Stack, Option<Image>, QRGuide);

impl QRCodeScanner {
    pub fn new(ctx: &mut Context) -> Self {
        QRCodeScanner(Stack::center(), None, QRGuide::new(ctx))
    }
}

impl Events for QRCodeScanner {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            match Camera::access() {
                Ok(val) if val == "cameraEnabled".to_string() => {*self.2.message() = None; *self.2.background() = None;},
                Ok(val) if val == "waitForCamera".to_string() || val == "Error: unknown".to_string() => {
                    *self.2.message() = Some(Message::new(ctx, "camera", "Accessing device camera."));
                },
                _ => *self.2.message() = Some(Message::new(ctx, "settings", "Enable camera in settings."))
            }
            #[cfg(any(target_os = "ios", target_os = "macos"))] {
                let frame = match Camera::get() {
                    Some(frame) => frame,
                    None => return true,
                };
                let image = resources::Image::new(ctx, frame);
                self.1 = Some(Image{shape: ShapeType::Rectangle(0.0, (300.0, 300.0)), image, color: None});
            }
        }
        true
    }
}


#[derive(Debug, Component)]
struct QRGuide(Stack, Option<RoundedRectangle>, RoundedRectangle, Option<Message>);
impl Events for QRGuide {}

impl QRGuide {
    pub fn new(ctx: &mut Context) -> Self {
        let colors = ctx.get::<PelicanUI>().theme.colors;
        let (background, color) = (colors.background.secondary, colors.outline.secondary);
        QRGuide(
            Stack(Offset::Center, Offset::Center, Size::Static(308.0), Size::Static(308.0), Padding::default()), 
            Some(RoundedRectangle::new(0.0, 8.0, background)), 
            RoundedRectangle::new(4.0, 8.0, color), 
            Some(Message::new(ctx, "camera", "Accessing device camera."))
        )
    }

    pub fn message(&mut self) -> &mut Option<Message> {&mut self.3}
    pub fn background(&mut self) -> &mut Option<RoundedRectangle> {&mut self.1}
}

#[derive(Debug, Component)]
struct Message(Column, Image, BasicText);
impl Events for Message {}

impl Message {
    pub fn new(ctx: &mut Context, icon: &'static str, msg: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.shades.lighten, theme.fonts.size.sm);
        Message(Column::center(4.0), 
            Icon::new(ctx, icon, color, 48.0),
            Text::new(ctx, msg, TextStyle::Secondary, font_size)
        )
    }
}