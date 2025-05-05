use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::RoundedRectangle;
use crate::layout::{Column, Stack, Offset, Size, Padding};
use crate::PelicanUI;

#[derive(Debug, Component)]
pub struct QRCodeScanner(Stack, Option<Image>, QRGuide, #[skip] Camera);

impl QRCodeScanner {
    pub fn new(ctx: &mut Context) -> Self {
        QRCodeScanner(Stack::center(), None, QRGuide::new(ctx), Camera::new())
    }
}

impl OnEvent for QRCodeScanner {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref() {
            let frame = self.3.get_frame();
            match frame {
                Ok(f) => {
                    *self.2.message() = None; *self.2.background() = None;
                    let image = ctx.add_image(f);
                    self.1 = Some(Image{shape: ShapeType::Rectangle(0.0, (300.0, 300.0)), image, color: None});
                },
                Err(CameraViewError::AccessDenied) => {
                    let background = ctx.get::<PelicanUI>().theme.colors.background.secondary;
                    *self.2.background() = Some(RoundedRectangle::new(0.0, 8.0, background));
                    *self.2.message() = Some(Message::new(ctx, "settings", "Enable camera in settings."));
                },
                Err(CameraViewError::FailedToGetFrame) => {
                    let background = ctx.get::<PelicanUI>().theme.colors.background.secondary;
                    *self.2.background() = Some(RoundedRectangle::new(0.0, 8.0, background));
                    *self.2.message() = Some(Message::new(ctx, "camera", "Accessing device camera."));
                }
            }
        }
        true
    }
}


#[derive(Debug, Component)]
struct QRGuide(Stack, Option<RoundedRectangle>, RoundedRectangle, Option<Message>);
impl OnEvent for QRGuide {}

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
struct Message(Column, Image, Text);
impl OnEvent for Message {}

impl Message {
    pub fn new(ctx: &mut Context, icon: &'static str, msg: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (color, font_size) = (theme.colors.shades.lighten, theme.fonts.size.sm);
        Message(Column::center(4.0), 
            Icon::new(ctx, icon, color, 48.0),
            Text::new(ctx, msg, TextStyle::Secondary, font_size, Align::Left)
        )
    }
}