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
        let qr_size = 315.0;
        let logo_size = 72.0;

        QRCode (
            Stack::center(),
            Bin(
                Stack(Offset::Center, Offset::Center, Size::Static(qr_size+16.0), Size::Static(qr_size+16.0), Padding::default()),
                RoundedRectangle::new(0.0, 8.0, color),
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
    pub fn new(ctx: &mut Context, code_str: &'static str, qr_size: f32, logo_size: f32) -> Self {
        let code = QrCode::new(code_str).unwrap();
        let module_count = code.width() as u32;
        let module_size = qr_size / module_count as f32;
    
        let mut rows: Vec<QRModuleRow> = vec![];
        for y in 0..module_count {
            rows.push(QRModuleRow::new(
                ctx,
                code.clone(),
                module_count,
                logo_size,
                module_size,
                y,
            ));
        }
    
        QRModules(Column::center(0.0), rows)
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
        logo_size: f32,  
        module_size: f32,
        y: u32
    ) -> Self {
        let shades = &ctx.get::<PelicanUI>().theme.colors.shades;
    
        let logo_modules = (logo_size / module_size) + 2.0;
        let logo_start = (module_count - logo_modules as u32) / 2;
        let logo_end = logo_start + logo_modules as u32;
    
        let mut modules: Vec<Shape> = vec![];
    
        for x in 0..module_count {
            let color = 
                if x >= logo_start && x < logo_end &&
                   y >= logo_start && y < logo_end
                {
                    shades.transparent
                } else if code[(x as usize, y as usize)] == Color::Dark {
                    shades.black
                } else {
                    shades.transparent
                };
            println!("circle size: {:?}", module_size);
            let circle = Circle::new(module_size, color);
    
            modules.push(circle);
        }
        QRModuleRow(Row::center(0.0), modules)
    }
    
}
