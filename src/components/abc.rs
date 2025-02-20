use rust_on_rails::prelude::*;
use crate::theme;
use crate::{Row, Column, Stack, Text, Padding, ConstrainedBox, COLORS};
use crate::layout::Align;
use crate::components::button::*;
use qrcode::{QrCode, Color};

#[derive(Clone, Copy)]
pub enum CircleIcon {
    Icon(&'static str, u32),
    Photo(&'static str, u32),
    Brand(&'static str, u32)
}

impl ComponentBuilder for CircleIcon {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); // Move this to theme startup
        let image = ctx.load_image("images/profile.png").unwrap(); // Get individual path images

        match self {
            CircleIcon::Photo(p, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), COLORS.background.primary, Some(100)), Vec2::new(0, 0)),
                    (Image(ShapeType::Circle(*s / 2), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            },
            CircleIcon::Icon(p, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), COLORS.background.secondary, None), Vec2::new(0, 0)),
                    (Image(ShapeType::Rectangle((*s as f32 * 0.75).round() as u32, (*s as f32 * 0.75).round() as u32), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            },
            CircleIcon::Brand(p, s) => {
                Stack!(Vec2::new(0, 0), Align::Center,
                    (Shape(ShapeType::Circle(*s / 2), COLORS.brand.primary, None), Vec2::new(0, 0)),
                    (Image(ShapeType::Rectangle((*s as f32 * 0.75).round() as u32, (*s as f32 * 0.75).round() as u32), image.clone()), Vec2::new(0, 0))
                ).build_children(ctx, max_size)
            }
        }
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Card {
    circle_icon: CircleIcon,
    title: &'static str,
    subtitle: &'static str,
    description: &'static str,
    button: Button
}

impl Card {
    pub fn room(n: &'static str, st: &'static str, d: &'static str) -> Self {
        Self {
            circle_icon: CircleIcon::Photo("profile", 64), // get user pfp
            title: n,
            subtitle: st,
            description: d,
            button: Button::Secondary("Join Room", Size::Medium, Width::Hug, None, Align::Center),
        }
    }
}

impl ComponentBuilder for Card {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        Column!(8, Vec2::new(24, 16), Align::Center, false,
            self.circle_icon,
            Text::new(self.title, COLORS.text.heading, 24, heading.clone()),
            Text::new(self.subtitle, COLORS.text.primary, 12, font.clone()),
            Padding(1, 6),
            Shape(ShapeType::Rectangle(230, 1), COLORS.outline.secondary, None),
            Padding(1, 6),
            Text::new(self.description, COLORS.text.primary, 14, font.clone()),
            self.button
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Navigator(Vec<(&'static str, &'static str)>, u16, bool);

impl Navigator {
    pub fn new(tabs: Vec<(&'static str, &'static str)>, default_i: u16, is_desktop: bool) -> Self {
        Self(tabs, default_i, is_desktop)
    }
}

impl ComponentBuilder for Navigator {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        let image = ctx.load_image("images/logomark.png").unwrap(); // Default logomark

        // Image(ShapeType::Rectangle(icon_size, 8), image),

        let buttons: Vec<Box<dyn ComponentBuilder>> = self.0.iter().enumerate().map(|(index, (name, _))| {
            // if index as u16 == self.1 { print!("selected") } else { print!{"not Selected"}}
            Box::new(Button::Ghost(*name, Size::Large, Width::Expand, None, Align::Left)) as Box<dyn ComponentBuilder>
        }).collect();

        ConstrainedBox!(300, 
            Column!(32, Vec2::new(16, 32), Align::Center, false,
                Image(ShapeType::Rectangle(150, 24), image),
                Column!(8, Vec2::new(0, 0), Align::Center, true, buttons),
                // Spacer
                Button::Ghost("My Profile", Size::Large, Width::Expand, None, Align::Left)
            )
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct QRCode(pub &'static str);

impl ComponentBuilder for QRCode {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        ctx.include_assets(include_assets!("./resources")); 
        let image = ctx.load_image("images/app_icon.png").unwrap();
        let final_size = 294;
        let logo_size = 72;
        let logo_space = logo_size + 24;
        let code = QrCode::new(self.0).unwrap();
        let size = code.width() as u32;
        let module_size = final_size / size;
        let radius = module_size / 2;
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
            
            rows.push(Box::new(Row!(0, Vec2::new(0, 0), Align::Center, true, row_cells)));
        }

        Stack!(Vec2::new(0, 0), Align::Center,
            (Shape(ShapeType::Rectangle(280, 280), light_color, None), Vec2::new(0, 0)),
            (Column!(0, Vec2::new(0, 0), Align::Center, true, rows), Vec2::new(0, 0)),
            (Image(ShapeType::Rectangle(logo_size, logo_size), image.clone()), Vec2::new(0, 0))
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct NumericKeypad();

impl ComponentBuilder for NumericKeypad {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();

        ConstrainedBox!(300, 
            Column!(16, Vec2::new(0, 0), Align::Center, false,
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost("1", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("2", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("3", Size::Large, Width::Expand, None, Align::Center), true)
                ),
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost("4", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("5", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("6", Size::Large, Width::Expand, None, Align::Center), true)
                ),
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost("7", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("8", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("9", Size::Large, Width::Expand, None, Align::Center), true)
                ),
                Row!(16, Vec2::new(0, 0), Align::Center, false,
                    (Button::Ghost(".", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("0", Size::Large, Width::Expand, None, Align::Center), true),
                    (Button::Ghost("<", Size::Large, Width::Expand, Some("back"), Align::Center), true)
                )
            )
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct ProfilePictures(pub Vec<&'static str>);

impl ComponentBuilder for ProfilePictures {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        
        ctx.include_assets(include_assets!("./resources")); 
        let image = ctx.load_image("images/profile.png").unwrap();

        let pfps: Vec<(Box<dyn ComponentBuilder>, Vec2)> = self.0
            .iter()
            .take(5)
            .enumerate()
            .map(|(index, _)| (
                Box::new(Stack!(Vec2::new(0, 0), Align::Center, 
                    (Image(ShapeType::Circle(32 / 2), image.clone()), Vec2::new(0, 0)),
                    (Shape(ShapeType::Circle(32 / 2), COLORS.background.primary, Some(500)), Vec2::new(0, 0))
                )) as Box<dyn ComponentBuilder>,
                Vec2::new(index as u32 * 20, 0)
            ))
            .collect();

        Stack!(Vec2::new(0, 0), Align::Left, true, pfps).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct MobileKeyboard();

impl ComponentBuilder for MobileKeyboard {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();
        let font = ctx.load_font("fonts/outfit_regular.ttf").unwrap();
        ctx.include_assets(include_assets!("./resources")); 
        let image = ctx.load_image("images/profile.png").unwrap();

        Stack!(Vec2::new(0, 0), Align::Top,
            (Shape(ShapeType::Rectangle(393, 300), COLORS.background.secondary, None), Vec2::new(0, 0)),
            (Column!(0, Vec2::new(0, 0), Align::Center, false,
                Row!(16, Vec2::new(12, 12), Align::Left, false,
                    (Image(ShapeType::Rectangle(36, 36), image.clone()), false),
                    (Image(ShapeType::Rectangle(36, 36), image.clone()), false),
                    (Image(ShapeType::Rectangle(36, 36), image.clone()), false),
                    (Image(ShapeType::Rectangle(36, 36), image.clone()), false),
                    (Image(ShapeType::Rectangle(36, 36), image.clone()), false),
                    (Shape(ShapeType::Rectangle(36, 36), COLORS.background.secondary, None), true), // Make expandable rectangles!
                    (Image(ShapeType::Rectangle(36, 36), image.clone()), false)
                ),
                Shape(ShapeType::Rectangle(393, 1), COLORS.outline.secondary, None) // Make expandable rectangles!
            ), Vec2::new(0, 0))
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}