use rust_on_rails::prelude::*;
use crate::elements::shapes::{RoundedRectangle, Circle};
use crate::layout::{Row, Column, Stack};
use crate::PelicanUI;

use qrcode::{QrCode, Color};

#[derive(Clone, Debug, Component)]
pub struct QRCode(Stack, Shape, QRModules, Image);
impl Events for QRCode {}

impl QRCode {
    pub fn new(ctx: &mut Context, data: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (app_icon, color) = (theme.brand.app_icon.clone(), theme.colors.shades.white);
        let qr_size = 294;
        let logo_size = 72;

        QRCode (
            Stack::center(),
            RoundedRectangle::new(qr_size+16, qr_size+16, 8, color),
            QRModules::new(ctx, data, qr_size, logo_size),
            Image(ShapeType::Rectangle(0, (logo_size, logo_size)), app_icon, None),
        )
    }
}

#[derive(Clone, Debug, Component)]
pub struct QRModules(Column, Vec<QRModuleRow>);
impl Events for QRModules {}

impl QRModules {
    pub fn new(ctx: &mut Context, code: &'static str, qr_size: u32, logo_size: u32) -> Self {
        let code = QrCode::new(code).unwrap();
        let module_count = code.width() as u32;
        let module_size = (qr_size as f32 / module_count as f32).ceil() as u32;

        let mut rows: Vec<QRModuleRow> = vec![];
        for y in 0..module_count {
            let py = y * module_size;
            rows.push(QRModuleRow::new(ctx, code.clone(), module_count, qr_size, logo_size, module_size, py, y));
        }

        QRModules(Column::center(0), rows)
    }
}

#[derive(Clone, Debug, Component)]
pub struct QRModuleRow(Row, Vec<Shape>);
impl Events for QRModuleRow{}

impl QRModuleRow {
    pub fn new(
        ctx: &mut Context, 
        code: QrCode, 
        module_count: u32, 
        qr_size: u32, 
        logo_size: u32,  
        module_size: u32,
        py: u32, 
        y: u32
    ) -> Self {
        let shades = &ctx.get::<PelicanUI>().theme.colors.shades;
        let logo_size = logo_size + 16;
        let logo_start = (qr_size - logo_size) / 2;
        let logo_end = logo_start + logo_size;

        let mut modules: Vec<Shape> = vec![];
        for x in 0..module_count {
            let px = x * module_size;
            let color = 
                if px > module_size 
                && py > module_size
                && (px + module_size) >= logo_start
                && (py + module_size) >= logo_start
                && px < logo_end
                && py < logo_end
            {
                shades.transparent
            } else if code[(x as usize, y as usize)] == Color::Dark {
                shades.black
            } else {
                shades.transparent
            };

            modules.push(Circle::new(module_size, color));
        }
        QRModuleRow(Row::center(0), modules)
    }
}

// pub struct QRCode(pub &'static str);

// impl ComponentBuilder for QRCode {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let qr_size = 294;
//         let logo_size = 72;
//         let logo_space = logo_size + 32;
//         let code = QrCode::new(self.0).unwrap();
//         let size = code.width() as u32;
//         let module_size = qr_size / size;
//         let radius = (module_size as f32 / 2.25) as u32;
//         let light_color = "ffffff";
//         let dark_color = "000000";

//         let logo_start = (qr_size - logo_space) / 2;
//         let logo_end = logo_start + logo_space;

//         let mut rows: Vec<Box<dyn ComponentBuilder>> = vec![];

//         for y in 0..size as usize {
//             let mut row_cells: Vec<Box<dyn ComponentBuilder>> = vec![];
//             for x in 0..size as usize {
//                 let px = x as u32 * module_size;
//                 let py = y as u32 * module_size;

//                 let color = if px >= logo_start
//                     && px < logo_end
//                     && py >= logo_start
//                     && py < logo_end
//                 {
//                     light_color
//                 } else if code[(x, y)] == Color::Dark {
//                     dark_color
//                 } else {
//                     light_color
//                 };

//                 row_cells.push(Circle(radius, color, None));
//             }
            
//             rows.push(Row(ZERO, 1, Align::Left,row_cells));
//         }

//         Stack(ZERO, Align::Center, vec![
//             RoundedRectangle(280, 280, 8, light_color, None),
//             Column(ZERO, 1, Align::Left, rows),
//             Image::Rectangle(image, None, logo_size),
//         ]).build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }
