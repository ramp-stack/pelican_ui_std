pub mod components;
pub mod theme;
pub mod layout;
pub mod elements;

use rust_on_rails::prelude::*;

use crate::theme::Theme;

use crate::layout::{ Stack};
use once_cell::sync::Lazy;
use crate::theme::colors::ColorResources;

pub const ZERO: Vec2 = Vec2{x:0,y:0};
static COLORS: Lazy<ColorResources> = Lazy::new(|| ColorResources::default());

// // fn icon(ctx: &mut Context) -> Handle {
// //     ctx.load_image("images/profile.png").unwrap()
// // }

// // pub fn pelican_startup(ctx: &mut Context) {
// //     ctx.include_assets(include_assets!("./resources")); 
// // }




// pub struct Padding(pub Vec2, pub &'static str);

// impl ComponentBuilder for Padding {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         Shape(ShapeType::Rectangle(self.0.x, self.0.y), self.1, None).build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct ConstrainedBox(pub u32, pub Box<dyn ComponentBuilder>);

// #[macro_export]
// macro_rules! ConstrainedBox {
//     ($x:expr, $i:expr) => {
//         ConstrainedBox($x, Box::new($i) as Box<dyn ComponentBuilder>)
//     };
// }

// impl ComponentBuilder for ConstrainedBox {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         self.1.build_children(ctx, Vec2::new(self.0, max_size.y))
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

// pub struct Expand(pub bool, pub u32, pub &'static str);

// impl ComponentBuilder for Expand {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         // println!("Max size: {}, {}", max_size.x, self.1);
//         match self.0 {
//             true => Shape(ShapeType::Rectangle(self.1, max_size.y), self.2, None).build_children(ctx, max_size),
//             false => Shape(ShapeType::Rectangle(max_size.x, self.1), self.2, None).build_children(ctx, max_size)
//         }
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

pub struct PelicanUI {
    pub theme: Theme,
}

impl PelicanUI {
    pub fn new(ctx: &mut Context) -> PelicanUI {
        ctx.include_assets(include_assets!("./resources"));
        PelicanUI { theme: Theme::new(ctx) }
    }
}

impl Plugin for PelicanUI {
    fn name() -> &'static str { "Pelican UI" }
}

pub mod prelude {
    // pub use crate::theme::colors::*;
    pub use crate::elements::icon::Icon;
    // pub use crate::theme::fonts::{Text, TextSize, TextStyle};
    // pub use crate::components::circle_icon::{IconStyle, CircleIcon, CircleIconData};
    // pub use crate::layout::*;
    // pub use crate::*;
    // pub use rust_on_rails::prelude::*;
    pub use crate::elements::shapes::{RoundedRectangle, Circle};
    pub use crate::elements::image::Image as ProfileImage;
    pub use crate::ZERO;
    pub use crate::layout::{Row, Column, Align};
    pub use crate::theme::Theme;
    pub use crate::PelicanUI;
}