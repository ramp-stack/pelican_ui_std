use rust_on_rails::prelude::*;
use rust_on_rails::prelude::Text as BasicText;
use crate::elements::images::Icon;
use crate::elements::text::{Text, TextStyle};
use crate::elements::shapes::Rectangle;
use crate::components::button::Button;
use crate::elements::shapes::RoundedRectangle;
use crate::layout::{Column, Bin, Stack, Row, Offset, Size, Padding};
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
            match camera_access() {
                Ok(val) if val == "Camera view enabled".to_string() => {*self.2.message() = None; *self.2.background() = None;},
                Ok(val) if val == "waitForCamera".to_string() => {
                    *self.2.message() = Some(Message::new(ctx, "camera", "Accessing device camera."));
                },
                _ => *self.2.message() = Some(Message::new(ctx, "settings", "Enable camera in settings."))
            }
            #[cfg(target_os = "ios")] {
                let frame = match get_camera_frame_as_rgba_image(300, 300) {
                    Some(frame) => frame,
                    None => return true,
                };
                let image = resources::Image::new(ctx, frame);
                self.1 = Some(Image{shape: ShapeType::Rectangle(0, (300, 300)), image, color: None});
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
            Stack(Offset::Center, Offset::Center, Size::Static(308), Size::Static(308), Padding::default()), 
            Some(RoundedRectangle::new(0, 8, background)), 
            RoundedRectangle::new(4, 8, color), 
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
        Message(Column::center(4), 
            Icon::new(ctx, icon, color, 48),
            Text::new(ctx, msg, TextStyle::Secondary, font_size)
        )
    }
}

pub fn camera_access() -> Result<String, String> {
    #[cfg(target_os = "ios")]
    let camera_access_status = unsafe { check_camera_access() };
    
    #[cfg(target_os = "ios")]
    if !camera_access_status.is_null() {
        let cstr = unsafe { std::ffi::CStr::from_ptr(camera_access_status) };
        let status = cstr.to_string_lossy().into_owned();
        return Ok(status)
    }

    Err("Failed to get camera access status".to_string())
}

use image::{RgbaImage, Rgba};

#[cfg(target_os = "ios")]
extern "C" {
    fn start_camera_capture();
    fn check_camera_access() -> *const std::ffi::c_char;
    fn get_latest_frame() -> *mut std::ffi::c_void;
    fn get_latest_frame_size() -> i32;
    fn get_latest_frame_stride() -> i32;
}

// #[cfg(target_os = "ios")]
pub fn capture() {
    // #[cfg(any(target_os = "ios", target_os = "macos"))]
    #[cfg(target_os = "ios")]
    unsafe {
        start_camera_capture();
    }
}

#[cfg(target_os = "ios")]
pub fn get_camera_frame_as_rgba_image(width: u32, height: u32) -> Option<RgbaImage> {
    unsafe {
        let ptr = get_latest_frame();
        let size = get_latest_frame_size();

        if ptr.is_null() || size <= 0 {
            return None;
        }

        let stride = unsafe { get_latest_frame_stride() } as usize;

        let slice = std::slice::from_raw_parts(ptr as *const u8, size as usize);
        let mut image = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let src_index = y as usize * stride + x as usize * 4;
                let dst_index = (y * width + x) as usize;

                if src_index + 3 >= slice.len() {
                    continue;
                }

                let b = slice[src_index];
                let g = slice[src_index + 1];
                let r = slice[src_index + 2];
                let a = slice[src_index + 3];

                image.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }

        Some(image::imageops::rotate90(&image))
    }
}
    
