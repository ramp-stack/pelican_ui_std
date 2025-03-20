use rust_on_rails::prelude::*;

pub struct Image(Box<dyn ComponentBuilder>);

impl Image {
    pub fn circle(s: u32, i: resources::Image) -> Self {
        Self(Box::new(rust_on_rails::prelude::Image(ShapeType::Ellipse(0, (s, s)), i, None)))
    }

    // pub fn circle_outlined(s: u32, i: resources::Image, oc: Color) -> Self {
    //     let o = (s as f32 * 0.06).round() as u32;
    //     Self(Box::new(
    //         Stack!(Align::Center, 
    //             RImage(ShapeType::Ellipse(0, (s, s)), i, None),
    //             Shape(ShapeType::Ellipse(o, (s + o, s + o)), oc)
    //         )
    //     ))
    // }
}

impl ComponentBuilder for Image {
    fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
        self.0.build_children(ctx, max_size)
    }
}


// pub enum Image {
//     Circle(RailsImage, Option<Outline>, u32),
// }

// // Image::Circle(*img, outline, self.3),

// impl ComponentBuilder for Image {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let mut children: Vec<Box<dyn ComponentBuilder>> = vec![];

//         let (img, outline, size) = match self {
//             Image::Circle(i, o, s) => (i.clone(), *o, *s),
//         };

//         // children.push(Box::new(Image(ShapeType::Ellipse(0, (size, size)), ctx.add_image(img)))); // Image 

//         if let Some(o) = outline {
//             children.push(o.new(size)) // Outline
//         }

//         Stack(ZERO, Align::Center, children).build_children(ctx, max_size)
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }