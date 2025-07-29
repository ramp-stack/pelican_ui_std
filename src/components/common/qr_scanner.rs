use pelican_ui::events::{OnEvent, Event, TickEvent};
use pelican_ui::drawable::{Drawable, Component, ShapeType, Image, Align};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::hardware::{Camera, CameraError};
use pelican_ui::{Context, Component};

use crate::{
    RoundedRectangle,
    Padding,
    Size,
    Offset,
    Stack,
    TextStyle,
    Text,
    Icon,
    Column
};

use image::{DynamicImage, GrayImage, RgbaImage};
use std::sync::{Mutex, Arc};

use quircs::Quirc;

use crate::events::QRCodeScannedEvent;

#[derive(Debug, Component)]
pub struct QRCodeScanner(
    Stack, 
    Option<Image>, 
    QRGuide, 
    #[skip] Option<Camera>, 
    #[skip] Arc<Mutex<Option<String>>>, 
    #[skip] Arc<Mutex<bool>>
);

impl QRCodeScanner {
    pub fn new(ctx: &mut Context) -> Self {
        // Try to create custom camera for raw frame support
        let camera = match Camera::new_custom() {
            Ok(cam) => Some(cam),
            Err(e) => {
                println!("Failed to create custom camera: {:?}", e);
                None
            }
        };
        
        QRCodeScanner(
            Stack::center(), 
            None, 
            QRGuide::new(ctx), 
            camera, 
            Arc::new(Mutex::new(None)), 
            Arc::new(Mutex::new(false))
        )
    }

    fn find_code(&mut self, img: RgbaImage) {
        if *self.5.lock().unwrap() {return;}
        *self.5.lock().unwrap() = true;

        let result_clone = self.4.clone();
        let flag_clone = self.5.clone();

        std::thread::spawn(move || {
            let result = decode_image(img, Quirc::default());

            if let Some(r) = result {
                *result_clone.lock().unwrap() = Some(r);
            }

            *flag_clone.lock().unwrap() = false;
        });
    }
}

impl OnEvent for QRCodeScanner {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            if let Some(ref mut camera) = self.3 {
                match camera.get_frame() {
                    Some(raw_frame) => {
                        println!("Received frame: {}x{}", raw_frame.width(), raw_frame.height());
                        
                        self.find_code(raw_frame.clone());
                        
                        if let Some(data) = &*self.4.lock().unwrap() {
                            println!("QR Data: {}", data);
                            ctx.trigger_event(QRCodeScannedEvent(data.to_string()));
                        }
                        
                        *self.2.message() = None; 
                        *self.2.background() = None;
                        let image = ctx.assets.add_image(raw_frame);
                        self.1 = Some(Image{
                            shape: ShapeType::Rectangle(0.0, (300.0, 300.0)), 
                            image, 
                            color: None
                        });
                    },
                    None => {
                        // No raw frame available yet, show waiting message
                        let background = ctx.theme.colors.background.secondary;
                        *self.2.background() = Some(RoundedRectangle::new(0.0, 8.0, background));
                        *self.2.message() = Some(Message::new(ctx, "camera", "Waiting for raw camera frame."));
                    }
                }
            } else {
                // No camera available
                let background = ctx.theme.colors.background.secondary;
                *self.2.background() = Some(RoundedRectangle::new(0.0, 8.0, background));
                *self.2.message() = Some(Message::new(ctx, "settings", "Camera not available."));
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
        let background = ctx.theme.colors.background.secondary;
        let outline = ctx.theme.colors.outline.secondary;
        QRGuide(
            Stack(Offset::Center, Offset::Center, Size::Static(308.0), Size::Static(308.0), Padding::default()), 
            Some(RoundedRectangle::new(0.0, 8.0, background)), 
            RoundedRectangle::new(4.0, 8.0, outline), 
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
    pub fn new(ctx: &mut Context, icon: &'static str, msg: &str) -> Self {
        let theme = &ctx.theme;
        let (color, font_size) = (theme.colors.shades.lighten, theme.fonts.size.sm);
        Message(Column::center(4.0), 
            Icon::new(ctx, icon, color, 48.0),
            Text::new(ctx, msg, TextStyle::Secondary, font_size, Align::Left)
        )
    }
}

fn decode_image(img_rgba: RgbaImage, mut decoder: Quirc) -> Option<String> {
    let img_gray: GrayImage = DynamicImage::ImageRgba8(img_rgba).to_luma8();

    let codes = decoder.identify(
        img_gray.width() as usize,
        img_gray.height() as usize,
        &img_gray,
    );

    for code in codes {
        match code {
            Ok(c) => match c.decode() {
                Ok(decoded) => {
                    let code = std::str::from_utf8(&decoded.payload).unwrap_or("<invalid utf8>");
                    return Some(code.to_string());
                }
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }
    None
}