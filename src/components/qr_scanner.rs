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
            match camera_access() {
                Ok(val) if val == "cameraEnabled".to_string() => {*self.2.message() = None; *self.2.background() = None;},
                Ok(val) if val == "waitForCamera".to_string() || val == "Error: unknown".to_string() => {
                    *self.2.message() = Some(Message::new(ctx, "camera", "Accessing device camera."));
                },
                _ => *self.2.message() = Some(Message::new(ctx, "settings", "Enable camera in settings."))
            }
            #[cfg(any(target_os = "ios", target_os = "macos"))] {
                let frame = match get_camera_frame_as_rgba_image() {
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

pub fn camera_access() -> Result<String, String> {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    let camera_access_status = unsafe { check_camera_access() };
    
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    if !camera_access_status.is_null() {
        let cstr = unsafe { std::ffi::CStr::from_ptr(camera_access_status) };
        let status = cstr.to_string_lossy().into_owned();
        return Ok(status)
    }

    Err("Failed to get camera access status".to_string())
}

use image::{RgbaImage, Rgba};

#[cfg(any(target_os = "ios", target_os = "macos"))]
extern "C" {
    fn start_camera_capture();
    fn check_camera_access() -> *const std::ffi::c_char;
    fn get_latest_frame() -> *mut std::ffi::c_void;
    fn get_latest_frame_stride() -> i32;
    fn get_initial_frame_size() -> i32;
    fn get_initial_frame_width() -> i32;
    fn get_initial_frame_height() -> i32;
}

pub fn capture() {
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    unsafe {
        start_camera_capture();
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub fn get_camera_frame_as_rgba_image() -> Option<RgbaImage> {
    unsafe {
        let ptr = get_latest_frame();
        let size = get_initial_frame_size();
        let stride = get_latest_frame_stride() as usize;
        let width = get_initial_frame_width() as u32;
        let height = get_initial_frame_height() as u32;

        if ptr.is_null() || size <= 0 || width == 0 || height == 0 {
            return None;
        }

        let slice = std::slice::from_raw_parts(ptr as *const u8, size as usize);
        let mut image = RgbaImage::new(width, height);

        let mut pixels = image.pixels_mut();

        for y in 0..height {
            let row_start = y as usize * stride;
            for x in 0..width {
                let src_index = row_start + x as usize * 4;
                if src_index + 3 >= slice.len() {
                    continue;
                }

                let r = slice[src_index + 2];
                let g = slice[src_index + 1]; 
                let b = slice[src_index];
                let a = slice[src_index + 3]; 

                let pixel = pixels.next().unwrap();
                *pixel = Rgba([r, g, b, a]);
            }
        }

        #[cfg(target_os = "ios")]
        return Some(image::imageops::rotate90(&image));
        #[cfg(not(target_os = "ios"))]
        return Some(image);
    }
}

