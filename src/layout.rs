use rust_on_rails::prelude::*;

// #[derive(Clone, Copy)]
// pub enum Align {
//     TopLeft,
//     TopCenter,
//     TopRight,
//     Left,
//     Center,
//     Right,
//     BottomLeft,
//     BottomCenter,
//     BottomRight,
// }

// impl Align {
//     pub fn align(&self, max_size: Vec2, min_size: Vec2) -> Vec2 {
//         match self {
//             Align::TopLeft => Vec2::new(0, 0),
//             Align::TopCenter => Vec2::new((max_size.x - min_size.x) / 2, 0),
//             Align::TopRight => Vec2::new(max_size.x - min_size.x, 0),
//             Align::Left => Vec2::new(0, (max_size.y - min_size.y) / 2),
//             Align::Center => Vec2::new((max_size.x - min_size.x) / 2, (max_size.y - min_size.y) / 2),
//             Align::Right => Vec2::new(max_size.x - min_size.x, (max_size.y - min_size.y) / 2),
//             Align::BottomLeft => Vec2::new(0, max_size.y - min_size.y),
//             Align::BottomCenter => Vec2::new((max_size.x - min_size.x) / 2, max_size.y - min_size.y),
//             Align::BottomRight => Vec2::new(max_size.x - min_size.x, max_size.y - min_size.y)
//         }
//     }
// }

// #[macro_export]
// macro_rules! Column {
//     ($x:expr, $i:expr, $a:expr, $( $child:expr ),* $(,)?) => {{
//         let children: Vec<Box<dyn ComponentBuilder>> = vec![
//             $( Box::new($child) as Box<dyn ComponentBuilder> ),*
//         ];
        
//         Column { 
//             children,
//             spacing: $i,
//             align: $a,
//             padding: $x,
//         }
//     }};
// }


// pub struct Column {
//     pub children: Vec<Box<dyn ComponentBuilder>>, 
//     pub spacing: u32,
//     pub align: Align,
//     pub padding: Vec2,
// }

// impl ComponentBuilder for Column {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let bound = Rect::new(self.padding.x, 0, max_size.x - (self.padding.x * 2), max_size.y);
//         let (mut max_width, mut current_y) = (0, 0);
    
//         for builder in &self.children {
//             let child = builder.build(ctx, bound);
//             let width = child.size(ctx).x;

//             if width > max_width { max_width = width; }
//         }

//         self.children.iter().map(|child| {
//             let mut bound = bound;
//             bound.y = current_y;

//             let mut child = child.build(ctx, bound);
//             let (height, width) = (child.size(ctx).y, child.size(ctx).x);

//             let y_offset = 0;

//             child.1.w -= y_offset;
//             child.1.x += y_offset;

//             current_y += height + self.spacing;

//             Box::new(child) as Box<dyn Drawable>
//         }).collect()
//     }

//     fn on_click(&mut self, ctx: &mut Context, max_size: Vec2, position: Vec2) {
//         let mut bound = Rect::new(0, 0, max_size.x, max_size.y);
//         for builder in &mut self.children {
//             let child = builder.build(ctx, bound);
//             let size = child.size(ctx);
//             if size.x > position.x && bound.y+size.y > position.y {
//                 println!("On Click Column");
//                 builder.on_click(
//                     ctx,
//                     Vec2::new(max_size.x, bound.y+self.spacing),
//                     Vec2::new(max_size.x, position.y-bound.y)
//                 );
//                 break;
//             }
//             if bound.y+size.y+self.spacing > position.y {
//                 break;
//             }
//             bound.h -= self.spacing + size.y;
//             bound.y += self.spacing + size.y;
//         }
//     }
// }

// #[macro_export]
// macro_rules! Row {
//     ($x:expr, $i:expr, $a:expr, $children:expr) => {{
//         let children: Vec<Box<dyn ComponentBuilder>> = $children
//             .into_iter()
//             .map(|child| Box::new(child) as Box<dyn ComponentBuilder>)
//             .collect();

//         Row { 
//             children,
//             spacing: $i,
//             align: $a,
//             padding: $x
//         }
//     }};
// }

// pub struct Row {
//     pub children: Vec<Box<dyn ComponentBuilder>>, 
//     pub spacing: u32,
//     pub align: Align,
//     pub padding: Vec2,
// }

// impl ComponentBuilder for Row {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let bound = Rect::new(self.padding.x, 0, max_size.x - (self.padding.x * 2), max_size.y);
//         let (mut max_height, mut current_x) = (0, 0);
    
//         for builder in &self.children {
//             let child = builder.build(ctx, bound);
//             let width = child.size(ctx).x;
//             let height = child.size(ctx).y;

//             if height > max_height { max_height = height; }
//         }

//         self.children.iter().map(|child| {
//             let mut bound = bound;
//             bound.x = current_x;

//             let mut child = child.build(ctx, bound);
//             let (height, width) = (child.size(ctx).y, child.size(ctx).x);

//             let y_offset = 0;

//             child.1.h -= y_offset;
//             child.1.y += y_offset;

//             current_x += width + self.spacing;

//             Box::new(child) as Box<dyn Drawable>
//         }).collect()
//     }
// }


// #[macro_export]
// macro_rules! Stack {
//     ($x:expr, $($child:expr),* $(,)?) => {{
//         let children: Vec<Box<dyn ComponentBuilder>> = vec![
//             $(Box::new($child) as Box<dyn ComponentBuilder>),*
//         ];
        
//         Stack($x, children)
//     }};
// }

// pub struct Stack(pub Align, pub Vec<Box<dyn ComponentBuilder>>);

// impl ComponentBuilder for Stack {
//     fn build_children(&self, ctx: &mut Context, max_size: Vec2) -> Vec<Box<dyn Drawable>> {
//         let bound = Rect::new(0, 0, max_size.x, max_size.y);

//         let (max_width, max_height, built) = self.1.iter()
//             .map(|builder| {
//                 let built = builder.build(ctx, bound);
//                 let size = built.size(ctx);
//                 (size, Box::new(built) as Box<dyn Drawable>)
//             })
//             .fold((0, 0, Vec::new()), |(max_x, max_y, mut built_vec), (size, built)| {
//                 built_vec.push(built);
//                 (max_x.max(size.x), max_y.max(size.y), built_vec)
//             });

//         self.1.iter().map(|builder| {
//             let mut child = builder.build(ctx, bound);
//             let (width, height) = (child.size(ctx).x, child.size(ctx).y);

//             let offsets = Vec2::new((max_width - width) / 2, (max_height - height) / 2); 

//             child.1.h -= offsets.y;
//             child.1.y += offsets.y;
//             child.1.w -= offsets.x;
//             child.1.x += offsets.x;

//             Box::new(child) as Box<dyn Drawable>
//         }).collect()
//     }

//     fn on_click(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
//     fn on_move(&mut self, _ctx: &mut Context, _max_size: Vec2, _position: Vec2) {}
// }

pub struct Container(pub (u32, u32), pub Vec<Box<dyn Component>>);

impl Component for Container {
    fn build(&self, ctx: &mut Context, max_size: (u32, u32)) -> Vec<((u32, u32), Box<dyn Component>)> {
        self.1.iter().flat_map(|c| c.build(ctx, self.0)).collect()
    }
}

#[macro_export]
macro_rules! container {
    ($x:expr, $($child:expr),* $(,)?) => {{
        Container($x, vec![ $(Box::new($child)),* ])
    }};
}