use rust_on_rails::prelude::*;
use crate::elements::images::Brand;
use crate::elements::shapes::{RoundedRectangle, Circle};
use crate::layout::{Row, Column, Bin, Stack, Size, Offset, Padding};
use crate::PelicanUI;

use qrcode::{QrCode, Color};

#[derive(Debug, Component)]
pub struct QRCode(Stack, Bin<Stack, RoundedRectangle>, QRModules, Image);
impl Events for QRCode {}

impl QRCode {
    pub fn new(ctx: &mut Context, data: &'static str) -> Self {
        let theme = &ctx.get::<PelicanUI>().theme;
        let (app_icon, color) = (theme.brand.app_icon.clone(), theme.colors.shades.white);
        let qr_size = 294;
        let logo_size = 72;

        QRCode (
            Stack::center(),
            Bin(
                Stack(Offset::Center, Offset::Center, Size::Static(qr_size+16), Size::Static(qr_size+16), Padding::default()),
                RoundedRectangle::new(0, 8, color),
            ),
            QRModules::new(ctx, data, qr_size, logo_size),
            Brand::new(app_icon, (logo_size, logo_size))
        )
    }
}

#[derive(Debug, Component)]
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

#[derive(Debug, Component)]
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
                && (px - (module_size / 6)) >= logo_start
                && (py - (module_size / 6)) >= logo_start
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

