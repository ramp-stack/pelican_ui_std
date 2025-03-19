use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align };
use crate::theme::fonts::{Text, TextSize};
use qrcode::{QrCode, Color};

pub struct QRCode(pub &'static str);

impl ComponentBuilder for QRCode {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let final_size = 294;
        let logo_size = 72;
        let logo_space = logo_size + 32;
        let code = QrCode::new(self.0).unwrap();
        let size = code.width() as u32;
        let module_size = final_size / size;
        let radius = (module_size as f32 / 2.25) as u32;
        let light_color = "ffffff";
        let dark_color = "000000";

        let logo_start = (final_size - logo_space) / 2;
        let logo_end = logo_start + logo_space;

        let mut rows: Vec<Box<dyn ComponentBuilder>> = vec![];

        for y in 0..size as usize {
            let mut row_cells: Vec<Box<dyn ComponentBuilder>> = vec![];
            for x in 0..size as usize {
                let px = x as u32 * module_size;
                let py = y as u32 * module_size;

                let color = if px >= logo_start
                    && px < logo_end
                    && py >= logo_start
                    && py < logo_end
                {
                    light_color
                } else if code[(x, y)] == Color::Dark {
                    dark_color
                } else {
                    light_color
                };

                row_cells.push(Circle(radius, color, None));
            }
            
            rows.push(Row(ZERO, 1, Align::Left,row_cells));
        }

        Stack(ZERO, Align::Center, vec![
            RoundedRectangle(280, 280, 8, light_color, None),
            Column(ZERO, 1, Align::Left, rows),
            Image::Rectangle(image, None, logo_size),
        ]).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
}
