use pelican_ui::events::OnEvent;
use pelican_ui::drawable::{ShapeType, Image, Color};
use pelican_ui::hardware::ImageOrientation;
use pelican_ui::{Context, resources};
use std::io::BufWriter;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use fast_image_resize::{IntoImageView, Resizer};
use fast_image_resize::images::Image as FirImage;
use image::GenericImageView;
use base64::{engine::general_purpose, Engine};

#[derive(Clone, Debug)]
pub struct Icon;
impl OnEvent for Icon {}
impl Icon {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(ctx: &mut Context, name: &'static str, color: Color, size: f32) -> Image {
        let icon = ctx.theme.icons.get(name);
        Image{shape: ShapeType::Rectangle(0.0, (size, size)), image: icon, color: Some(color)}
    }
}

#[derive(Clone, Debug)]
pub struct Brand;
impl OnEvent for Brand {}
impl Brand {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(image: resources::Image, size: (f32, f32)) -> Image {
        Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image, color: None}
    }
}

pub struct EncodedImage;

impl EncodedImage {
    pub fn encode(bytes: Vec<u8>, orientation: ImageOrientation) -> Option<String> {
        if let Ok(dynamic) = image::load_from_memory(&bytes) {
            let src_image = orientation.apply_to(image::DynamicImage::ImageRgba8(dynamic.to_rgba8()));
            let (w, h) = src_image.dimensions();
            let s = 256.0 / w.min(h) as f32;
            let (w, h) = ((w as f32 * s) as u32, (h as f32 * s) as u32);
            let mut dst_image = FirImage::new(w, h, src_image.pixel_type().unwrap());
            Resizer::new().resize(&src_image, &mut dst_image, None).unwrap();

            let mut result_buf = BufWriter::new(Vec::new());
            PngEncoder::new(&mut result_buf).write_image(dst_image.buffer(), w, h, src_image.color().into()).unwrap();
            let result_buf = result_buf.into_inner().unwrap(); // get the inner Vec<u8>
            return Some(general_purpose::STANDARD.encode(&result_buf))
        }
        None
    }

    pub fn decode(ctx: &mut Context, bytes: &String) -> resources::Image {
        let png_bytes = general_purpose::STANDARD.decode(bytes).unwrap();
        let image = image::load_from_memory(&png_bytes).unwrap();
        ctx.assets.add_image(image.into())  
    }
}
