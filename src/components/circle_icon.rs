// use rust_on_rails::prelude::*;
// use crate::elements::icon::Icon;
// use crate::elements::shapes::Circle;
// use crate::elements::image::Image;
// use either::Either;
// use crate::PelicanUI;

// pub struct CircleIcon(pub Either<CircleIconData, resources::Image>, pub Option<CircleIconData>, pub bool);

// impl ComponentBuilder for CircleIcon {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let color = ctx.get::<PelicanUI>().theme.colors.shades.black;
//         let size = max_size.x.min(max_size.y);

//         vec![
//             match self.0 {
//                 Either::Left(circle_icon) => Box::new(circle_icon),
//                 Either::Right(profile) => Box::new(Image::circle(size, profile))
//             },
//             Box::new(match self.2 {
//                 true => Some(Shape(ShapeType::Ellipse((size as f32 * 0.06).round() as u32, (size, size)), color)),
//                 false => None
//             }),
//             Box::new(self.1.map(|circle_icon| {
//                 let s = (size as f32 * 0.3).round() as u32;
//                 vec![
//                     Box::new(Shape(ShapeType::Ellipse((s as f32 * 0.06).round() as u32, (s, s)), color)),
//                     Box::new(circle_icon)
//                 ]
//             }))
//         ]
//     }
// }

// pub struct CircleIconData(&'static str, CircleIconStyle);

// impl ComponentBuilder for CircleIconData {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let size = max_size.x.min(max_size.y);
//         let (background, icon_color) = self.1.get(ctx);
//         vec![
//             Box::new(Circle::new(size, background)),
//             Box::new(Icon(self.0, icon_color, size))
//         ]
//     }
// }

// pub enum CircleIconStyle {
//     Primary,
//     Secondary,
//     Brand,
//     Success,
//     Warning,
//     Danger
// }

// impl CircleIconStyle {
//     pub fn get(&self, ctx: &mut Context) -> (Color, Color) {
//         let colors = &ctx.get::<PelicanUI>().theme.colors;
//         match self {
//             CircleIconStyle::Primary => (colors.text.heading, colors.background.primary),
//             CircleIconStyle::Secondary => (colors.background.secondary, colors.text.secondary),
//             CircleIconStyle::Brand => (colors.brand.primary, colors.brand.secondary),
//             CircleIconStyle::Success => (colors.status.success, colors.text.heading),
//             CircleIconStyle::Warning => (colors.status.warning, colors.text.heading),
//             CircleIconStyle::Danger => (colors.status.danger, colors.text.heading),
//         }
//     }
// }
