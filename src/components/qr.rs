use rust_on_rails::prelude::*;
use crate::{ Child, ConstrainedBox, Row, Column, COLORS, ZERO, Align };
use crate::theme::fonts::{Text, TextSize};
use qrcode::{QrCode, Color};


pub struct QRCode(pub &'static str);

impl ComponentBuilder for QRCode {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); 
        let image = ctx.load_image("images/app_icon.png").unwrap();
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

                row_cells.push(Box::new(Shape(ShapeType::Circle(radius), color, None)));
            }
            
            rows.push(Box::new(Row!(1, ZERO, Align::Left, true, row_cells)));
        }

        Stack!(ZERO, Align::Center,
            (Shape(ShapeType::Rectangle(280, 280), light_color, None), ZERO),
            (Column!(1, ZERO, Align::Left, true, rows), ZERO),
            (Image(ShapeType::Rectangle(logo_size, logo_size), image.clone()), ZERO)
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}



pub struct QRCodeScanner();

impl ComponentBuilder for QRCodeScanner {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let camera_disabled = true;

        let mut container: Vec<(Box<dyn ComponentBuilder>, Vec2)> = vec![];

        container.push((
            Box::new(Shape(ShapeType::Rectangle(236, 236), COLORS.background.secondary, None)), 
            ZERO
        ));

        container.push((
            Box::new(Shape(ShapeType::Rectangle(236, 236), COLORS.text.secondary, Some(1000))), 
            ZERO
        ));

        if camera_disabled {
            container.push((Box::new(Column!(4, ZERO, Align::Center, false,
                Row!(0, ZERO, Align::Center, false,
                    (Shape(ShapeType::Rectangle(32, 32), "ffffff", None), true)
                ), 
                Row!(0, ZERO, Align::Center, false,
                    (Text::secondary(ctx, "Enable camera in settings.", TextSize::sm()), true)
                )
            )), ZERO));
        }



        Stack!(ZERO, Align::Center, true, container).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}