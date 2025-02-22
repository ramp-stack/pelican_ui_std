use rust_on_rails::prelude::*;
use crate::components::inputs::*;
use crate::components::display::*;
use crate::{
    ConstrainedBox,
    Row, 
    Expand,
    Column,
    Text, 
    COLORS
};
use crate::layout::Align;


pub struct Navigator(Vec<(&'static str, &'static str)>, u16, bool);

impl Navigator {
    pub fn new(tabs: Vec<(&'static str, &'static str)>, default_i: u16, is_desktop: bool) -> Self {
        Self(tabs, default_i, is_desktop)
    }
}

impl ComponentBuilder for Navigator {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let image = ctx.load_image("images/logomark.png").unwrap(); // Default logomark

        // Image(ShapeType::Rectangle(icon_size, 8), image),

        let buttons: Vec<Box<dyn ComponentBuilder>> = self.0.iter().enumerate().map(|(_index, (name, _))| {
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

pub struct TabNav();

impl ComponentBuilder for TabNav {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let image = ctx.load_image("images/profile.png").unwrap(); // Default logomark

        ConstrainedBox!(300, 
            Row!(0, Vec2::new(16, 32), Align::Center, false,
                (Shape(ShapeType::Rectangle(32, 32), "ffffff", None), false),
                (Expand(false, 1, COLORS.background.primary), true),
                (Shape(ShapeType::Rectangle(32, 32), "ffffff", None), false),
                (Expand(false, 1, COLORS.background.primary), true),
                (Image(ShapeType::Circle(32 / 2), image.clone()), false)
            )
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}

pub struct Header();

impl ComponentBuilder for Header {
    fn build_children(&self, ctx: &mut ComponentContext, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        let heading = ctx.load_font("fonts/outfit_bold.ttf").unwrap();

        ConstrainedBox!(300, 
            Row!(16, Vec2::new(16, 32), Align::Center, false,
                (Shape(ShapeType::Rectangle(32, 32), "ffffff", None), false),
                (Expand(false, 1, COLORS.background.primary), true),
                (Column!(10, Vec2::new(0, 0), Align::Center, false,
                    ProfilePictures(vec![""]),
                    Text::new("Ella Couch", COLORS.text.primary, 20, heading.clone())
                ), false),
                (Expand(false, 1, COLORS.background.primary), true),
                (Shape(ShapeType::Rectangle(32, 32), "ffffff", None), false)
            )
        ).build_children(ctx, max_size)
    }

    fn on_click(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
    fn on_move(&mut self, _ctx: &mut ComponentContext, _max_size: Vec2, _position: Vec2) {}
}